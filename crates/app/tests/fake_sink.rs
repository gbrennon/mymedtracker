use medtrack_application::{ApplicationError, ReminderMessage, ReminderSink};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

pub struct FakeReminderSink {
    reminders: Arc<Mutex<Vec<ReminderMessage>>>,
}

impl FakeReminderSink {
    pub fn new() -> Self {
        Self {
            reminders: Arc::new(Mutex::new(Vec::new())),
        }
    }

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
