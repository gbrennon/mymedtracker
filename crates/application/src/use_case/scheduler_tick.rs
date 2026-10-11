use crate::{ApplicationError, Clock, PatientStore, ReminderMessage, ReminderSink};
use std::sync::Arc;

pub struct SchedulerTick {
    store: Arc<dyn PatientStore>,
    clock: Arc<dyn Clock>,
    sink: Arc<dyn ReminderSink>,
}

impl SchedulerTick {
    pub fn new(
        store: Arc<dyn PatientStore>,
        clock: Arc<dyn Clock>,
        sink: Arc<dyn ReminderSink>,
    ) -> Self {
        Self { store, clock, sink }
    }

    pub async fn execute(&self) -> Result<usize, ApplicationError> {
        let now = self.clock.now().await;
        let candidates = self.store.claim_overdue_reminders(now).await?;
        let mut sent = 0;
        for candidate in candidates {
            let reminder = ReminderMessage::new(
                candidate.patient().clone(),
                candidate.medication().clone(),
                candidate.due_at(),
                now,
            );
            self.sink.send(reminder).await?;
            sent += 1;
        }
        Ok(sent)
    }
}
