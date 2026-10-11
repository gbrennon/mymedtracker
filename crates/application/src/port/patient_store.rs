use crate::{ApplicationError, IntakeEvent};
use medtrack_domain::{Instant, ReminderCandidate, Schedule, ScheduleIdentity};
use std::pin::Pin;

/// Stores schedules and provides atomic aggregate operations.
pub trait PatientStore: Send + Sync {
    /// Persists or replaces a schedule under its identity.
    fn add_schedule(
        &self,
        schedule: Schedule,
    ) -> Pin<Box<dyn Future<Output = Result<(), ApplicationError>> + Send + '_>>;

    /// Records an intake atomically with schedule lookup and mutation.
    fn record_intake(
        &self,
        event: IntakeEvent,
    ) -> Pin<Box<dyn Future<Output = Result<(), ApplicationError>> + Send + '_>>;

    /// Returns a cloned schedule for read-only reporting.
    fn schedule(
        &self,
        identity: ScheduleIdentity,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Schedule>, ApplicationError>> + Send + '_>>;

    /// Claims all overdue reminders atomically and prevents later duplicate claims.
    fn claim_overdue_reminders(
        &self,
        now: Instant,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ReminderCandidate>, ApplicationError>> + Send + '_>>;
}
