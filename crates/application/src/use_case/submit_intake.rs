use crate::{ApplicationError, IntakeEvent, PatientStore};
use std::sync::Arc;

pub struct SubmitIntake {
    store: Arc<dyn PatientStore>,
}

impl SubmitIntake {
    pub fn new(store: Arc<dyn PatientStore>) -> Self {
        Self { store }
    }

    pub async fn execute(&self, event: IntakeEvent) -> Result<(), ApplicationError> {
        self.store.record_intake(event).await
    }
}
