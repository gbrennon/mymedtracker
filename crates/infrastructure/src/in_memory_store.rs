use medtrack_application::{ApplicationError, IntakeEvent, PatientStore};
use medtrack_domain::{Instant, ReminderCandidate, Schedule, ScheduleIdentity};
use std::collections::HashMap;
use std::pin::Pin;
use tokio::sync::RwLock;

/// A concurrency-safe in-memory implementation of the patient store port.
pub struct InMemoryPatientStore {
    schedules: RwLock<HashMap<ScheduleIdentity, Schedule>>,
}

impl InMemoryPatientStore {
    /// Creates an empty store.
    pub fn new() -> Self {
        Self {
            schedules: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryPatientStore {
    fn default() -> Self {
        Self::new()
    }
}

impl PatientStore for InMemoryPatientStore {
    fn add_schedule(
        &self,
        schedule: Schedule,
    ) -> Pin<Box<dyn Future<Output = Result<(), ApplicationError>> + Send + '_>> {
        Box::pin(async move {
            let identity = schedule.identity().clone();
            self.schedules.write().await.insert(identity, schedule);
            Ok(())
        })
    }

    fn record_intake(
        &self,
        event: IntakeEvent,
    ) -> Pin<Box<dyn Future<Output = Result<(), ApplicationError>> + Send + '_>> {
        Box::pin(async move {
            let (patient, medication) = event.identity();
            let identity = ScheduleIdentity::new(patient.clone(), medication.clone());
            let mut schedules = self.schedules.write().await;
            let schedule = schedules
                .get_mut(&identity)
                .ok_or(ApplicationError::ScheduleNotFound)?;
            schedule.record_intake(event.at()).map_err(Into::into)
        })
    }

    fn schedule(
        &self,
        identity: ScheduleIdentity,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Schedule>, ApplicationError>> + Send + '_>> {
        Box::pin(async move {
            let schedule = self.schedules.read().await.get(&identity).cloned();
            Ok(schedule)
        })
    }

    fn claim_overdue_reminders(
        &self,
        now: Instant,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ReminderCandidate>, ApplicationError>> + Send + '_>>
    {
        Box::pin(async move {
            let mut schedules = self.schedules.write().await;
            Ok(schedules
                .values_mut()
                .flat_map(|schedule| schedule.claim_overdue_reminders(now))
                .collect())
        })
    }
}
