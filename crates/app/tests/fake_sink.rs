use medtrack_application::{ApplicationError, ReminderMessage, ReminderSink};
use std::pin::Pin;
use std::sync::{Arc, Mutex};

/// Collects reminders in memory for deterministic assertions.
pub struct FakeReminderSink {
    reminders: Arc<Mutex<Vec<ReminderMessage>>>,
}

impl FakeReminderSink {
    /// Creates an empty fake sink.
    pub fn new() -> Self {
        Self {
            reminders: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Returns a snapshot of all delivered reminders.
    pub fn reminders(&self) -> Vec<ReminderMessage> {
        self.reminders.lock().expect("sink lock").clone()
    }
}

impl ReminderSink for FakeReminderSink {
    fn send(
        &self,
        reminder: ReminderMessage,
    ) -> Pin<Box<dyn Future<Output = Result<(), ApplicationError>> + Send + '_>> {
        let reminders = self.reminders.clone();
        Box::pin(async move {
            reminders.lock().expect("sink lock").push(reminder);
            Ok(())
        })
    }
}
