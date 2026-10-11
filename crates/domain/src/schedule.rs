use crate::{DomainError, Dose, Instant, ReminderCandidate, ScheduleConfig, ScheduleIdentity};

/// A medication schedule aggregate that owns dose transitions and adherence queries.
#[derive(Clone, Debug, PartialEq)]
pub struct Schedule {
    identity: ScheduleIdentity,
    grace_period: crate::Duration,
    doses: Vec<Dose>,
}

impl Schedule {
    /// Builds a schedule after validating its identity and finite timing configuration.
    pub fn new(
        identity: ScheduleIdentity,
        configuration: ScheduleConfig,
    ) -> Result<Self, DomainError> {
        let doses = Self::build_doses(
            configuration.first_dose_at(),
            configuration.interval(),
            configuration.dose_count(),
        )?;
        Ok(Self {
            identity,
            grace_period: configuration.grace_period(),
            doses,
        })
    }

    /// Returns the patient and medication identity for this schedule.
    pub fn identity(&self) -> &ScheduleIdentity {
        &self.identity
    }

    /// Records an intake in the matching dose window.
    pub fn record_intake(&mut self, at: Instant) -> Result<(), DomainError> {
        if at < self.first_dose_at() {
            return Err(DomainError::IntakeBeforeScheduleStart { at });
        }
        for dose in &mut self.doses {
            if dose.is_within_window(at, self.grace_period) {
                if dose.is_taken() {
                    return Err(DomainError::DuplicateDoseIntake {
                        patient: self.identity.patient().clone(),
                        at,
                    });
                }
                return dose.mark_taken(at);
            }
        }
        Err(DomainError::IntakeOutsideDoseWindow { at })
    }

    /// Calculates taken doses divided by doses scheduled by the supplied instant.
    pub fn adherence_rate(&self, now: Instant) -> f64 {
        let elapsed = self
            .doses
            .iter()
            .filter(|dose| dose.scheduled_at() <= now)
            .collect::<Vec<_>>();
        if elapsed.is_empty() {
            return 0.0;
        }
        let taken = elapsed.iter().filter(|dose| dose.is_taken()).count();
        taken as f64 / elapsed.len() as f64
    }

    /// Atomically marks every currently overdue unclaimed dose and returns candidates.
    pub fn claim_overdue_reminders(&mut self, now: Instant) -> Vec<ReminderCandidate> {
        self.doses
            .iter_mut()
            .filter_map(|dose| {
                dose.claim_reminder(now, self.grace_period).then(|| {
                    ReminderCandidate::new(
                        self.identity.patient().clone(),
                        self.identity.medication().clone(),
                        dose.scheduled_at(),
                    )
                })
            })
            .collect()
    }

    fn build_doses(
        first_dose_at: Instant,
        interval: crate::Duration,
        dose_count: usize,
    ) -> Result<Vec<Dose>, DomainError> {
        let mut doses = Vec::with_capacity(dose_count);
        let mut scheduled_at = first_dose_at;
        for _ in 0..dose_count {
            doses.push(Dose::new(scheduled_at));
            scheduled_at = scheduled_at
                .checked_add(interval)
                .ok_or(DomainError::ScheduleOverflow)?;
        }
        Ok(doses)
    }

    fn first_dose_at(&self) -> Instant {
        self.doses[0].scheduled_at()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Duration, MedicationName, PatientId};

    fn schedule() -> Schedule {
        let identity = ScheduleIdentity::new(
            PatientId::try_new("patient-1").expect("patient"),
            MedicationName::try_new("medication-a").expect("medication"),
        );
        let configuration = ScheduleConfig::new(
            Instant::from_millis(1_000),
            Duration::from_millis(1_000),
            Duration::from_millis(100),
            3,
        )
        .expect("configuration");
        Schedule::new(identity, configuration).expect("schedule")
    }

    #[test]
    fn intake_counts_against_the_matching_dose() {
        let mut schedule = schedule();
        schedule
            .record_intake(Instant::from_millis(1_050))
            .expect("intake");

        assert_eq!(schedule.adherence_rate(Instant::from_millis(1_500)), 1.0);
    }

    #[test]
    fn adherence_uses_only_doses_due_by_now() {
        let mut schedule = schedule();
        schedule
            .record_intake(Instant::from_millis(1_050))
            .expect("intake");

        let adherence = schedule.adherence_rate(Instant::from_millis(2_500));

        assert_eq!(adherence, 0.5);
    }

    #[test]
    fn overdue_claim_is_idempotent() {
        let mut schedule = schedule();
        let now = Instant::from_millis(1_101);

        let first_claim = schedule.claim_overdue_reminders(now);
        let second_claim = schedule.claim_overdue_reminders(now);

        assert_eq!(first_claim.len(), 1);
        assert!(second_claim.is_empty());
        assert_eq!(first_claim[0].due_at(), Instant::from_millis(1_000));
    }

    #[test]
    fn intake_before_schedule_start_is_rejected() {
        let mut schedule = schedule();

        let result = schedule.record_intake(Instant::from_millis(999));

        assert_eq!(
            result,
            Err(DomainError::IntakeBeforeScheduleStart {
                at: Instant::from_millis(999)
            })
        );
    }
}
