use crate::{ApplicationError, IntakeEvent};
use medtrack_domain::{Instant, ReminderCandidate, Schedule, ScheduleIdentity};
use std::future::Future;
use std::pin::Pin;

pub trait PatientStore: Send + Sync {
    fn add_schedule(
        &self,
        schedule: Schedule,
    ) -> Pin<Box<dyn Future<Output = Result<(), ApplicationError>> + Send + '_>>;

    fn record_intake(
        &self,
        event: IntakeEvent,
    ) -> Pin<Box<dyn Future<Output = Result<(), ApplicationError>> + Send + '_>>;

    fn schedule(
        &self,
        identity: ScheduleIdentity,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Schedule>, ApplicationError>> + Send + '_>>;

    fn claim_overdue_reminders(
        &self,
        now: Instant,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ReminderCandidate>, ApplicationError>> + Send + '_>>;
}
