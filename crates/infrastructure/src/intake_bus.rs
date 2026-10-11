use medtrack_application::{ApplicationError, IntakeEvent, SubmitIntake};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

#[derive(Clone)]
pub struct IntakeBus {
    sender: mpsc::Sender<IntakeEvent>,
}

impl IntakeBus {
    pub fn start(
        submit_intake: Arc<SubmitIntake>,
        capacity: usize,
    ) -> (Self, JoinHandle<Result<(), ApplicationError>>) {
        let (sender, mut receiver) = mpsc::channel(capacity);
        let task = tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                submit_intake.execute(event).await?;
            }
            Ok(())
        });
        (Self { sender }, task)
    }

    pub async fn send(
        &self,
        event: IntakeEvent,
    ) -> Result<(), mpsc::error::SendError<IntakeEvent>> {
        self.sender.send(event).await
    }
}
