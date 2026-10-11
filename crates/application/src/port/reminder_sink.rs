use crate::{ApplicationError, ReminderMessage};
use std::pin::Pin;

/// Delivers reminders to an external notification mechanism.
pub trait ReminderSink: Send + Sync {
    /// Delivers one reminder and reports delivery failure.
    fn send(
        &self,
        reminder: ReminderMessage,
    ) -> Pin<Box<dyn Future<Output = Result<(), ApplicationError>> + Send + '_>>;
}
