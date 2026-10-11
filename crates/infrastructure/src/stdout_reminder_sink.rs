use medtrack_application::{ApplicationError, ReminderMessage, ReminderSink};
use std::pin::Pin;

/// Writes reminder messages to standard output.
pub struct StdoutReminderSink;

impl ReminderSink for StdoutReminderSink {
    fn send(
        &self,
        reminder: ReminderMessage,
    ) -> Pin<Box<dyn Future<Output = Result<(), ApplicationError>> + Send + '_>> {
        Box::pin(async move {
            println!(
                "REMINDER patient={} medication={} due_at={} sent_at={}",
                reminder.patient(),
                reminder.medication(),
                reminder.due_at().as_millis(),
                reminder.sent_at().as_millis()
            );
            Ok(())
        })
    }
}
