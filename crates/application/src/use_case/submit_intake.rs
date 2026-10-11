use crate::{ApplicationError, IntakeEvent, PatientStore};
use std::sync::Arc;

/// Submits an observed intake through the store's atomic mutation port.
pub struct SubmitIntake {
    store: Arc<dyn PatientStore>,
}

impl SubmitIntake {
    /// Creates an intake use case with an injected store.
    pub fn new(store: Arc<dyn PatientStore>) -> Self {
        Self { store }
    }

    /// Records an event or returns the domain or storage failure.
    pub async fn execute(&self, event: IntakeEvent) -> Result<(), ApplicationError> {
        self.store.record_intake(event).await
    }
}
