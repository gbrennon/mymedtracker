use crate::{ApplicationError, ReminderMessage};
use std::future::Future;
use std::pin::Pin;

pub trait ReminderSink: Send + Sync {
    fn send(
        &self,
        reminder: ReminderMessage,
    ) -> Pin<Box<dyn Future<Output = Result<(), ApplicationError>> + Send + '_>>;
}
