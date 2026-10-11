use crate::{Instant, MedicationName, PatientId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReminderCandidate {
    patient: PatientId,
    medication: MedicationName,
    due_at: Instant,
}

impl ReminderCandidate {
    pub(crate) fn new(patient: PatientId, medication: MedicationName, due_at: Instant) -> Self {
        Self {
            patient,
            medication,
            due_at,
        }
    }

    pub fn patient(&self) -> &PatientId {
        &self.patient
    }

    pub fn medication(&self) -> &MedicationName {
        &self.medication
    }

    pub fn due_at(&self) -> Instant {
        self.due_at
    }
}
