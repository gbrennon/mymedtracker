use crate::{ApplicationError, PatientStore};
use medtrack_domain::Schedule;
use std::sync::Arc;

pub struct RegisterSchedule {
    store: Arc<dyn PatientStore>,
}

impl RegisterSchedule {
    pub fn new(store: Arc<dyn PatientStore>) -> Self {
        Self { store }
    }

    pub async fn execute(&self, schedule: Schedule) -> Result<(), ApplicationError> {
        self.store.add_schedule(schedule).await
    }
}
