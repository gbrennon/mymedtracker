use crate::{ApplicationError, PatientStore};
use medtrack_domain::Schedule;
use std::sync::Arc;

/// Registers a schedule through the storage port.
pub struct RegisterSchedule {
    store: Arc<dyn PatientStore>,
}

impl RegisterSchedule {
    /// Creates a registration use case with an injected store.
    pub fn new(store: Arc<dyn PatientStore>) -> Self {
        Self { store }
    }

    /// Persists the supplied schedule.
    pub async fn execute(&self, schedule: Schedule) -> Result<(), ApplicationError> {
        self.store.add_schedule(schedule).await
    }
}
