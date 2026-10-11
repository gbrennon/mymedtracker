use crate::{DomainError, DoseStatus, Duration, Instant};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dose {
    scheduled_at: Instant,
    taken_at: Option<Instant>,
    reminder_sent: bool,
}

impl Dose {
    pub(crate) fn new(scheduled_at: Instant) -> Self {
        Self {
            scheduled_at,
            taken_at: None,
            reminder_sent: false,
        }
    }

    pub fn scheduled_at(&self) -> Instant {
        self.scheduled_at
    }

    pub fn taken_at(&self) -> Option<Instant> {
        self.taken_at
    }

    pub fn status_at(&self, now: Instant, grace_period: Duration) -> DoseStatus {
        if self.taken_at.is_some() {
            return DoseStatus::Taken;
        }
        if self.is_overdue_at(now, grace_period) {
            return DoseStatus::Overdue;
        }
        DoseStatus::Scheduled
    }

    pub fn mark_taken(&mut self, at: Instant) -> Result<(), DomainError> {
        if self.taken_at.is_some() {
            return Err(DomainError::DoseAlreadyTaken);
        }
        if at < self.scheduled_at {
            return Err(DomainError::IntakeBeforeScheduleStart { at });
        }
        self.taken_at = Some(at);
        Ok(())
    }

    pub(crate) fn is_taken(&self) -> bool {
        self.taken_at.is_some()
    }

    pub(crate) fn is_within_window(&self, at: Instant, grace_period: Duration) -> bool {
        self.scheduled_at <= at
            && self
                .scheduled_at
                .checked_add(grace_period)
                .is_some_and(|deadline| at <= deadline)
    }

    pub(crate) fn is_overdue_at(&self, now: Instant, grace_period: Duration) -> bool {
        self.scheduled_at
            .checked_add(grace_period)
            .is_some_and(|deadline| now > deadline)
            && !self.is_taken()
    }

    pub(crate) fn claim_reminder(&mut self, now: Instant, grace_period: Duration) -> bool {
        if self.reminder_sent || !self.is_overdue_at(now, grace_period) {
            return false;
        }
        self.reminder_sent = true;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newly_created_dose_is_scheduled() {
        let dose = Dose::new(Instant::from_millis(1_000));

        let status = dose.status_at(Instant::from_millis(1_500), Duration::from_millis(1_000));

        assert_eq!(status, DoseStatus::Scheduled);
    }

    #[test]
    fn taken_dose_rejects_second_intake() {
        let mut dose = Dose::new(Instant::from_millis(1_000));
        dose.mark_taken(Instant::from_millis(1_100))
            .expect("first intake");

        let result = dose.mark_taken(Instant::from_millis(1_200));

        assert_eq!(result, Err(DomainError::DoseAlreadyTaken));
    }

    #[test]
    fn overdue_status_requires_grace_period_to_pass() {
        let dose = Dose::new(Instant::from_millis(1_000));

        let status = dose.status_at(Instant::from_millis(2_001), Duration::from_millis(1_000));

        assert_eq!(status, DoseStatus::Overdue);
    }
}
