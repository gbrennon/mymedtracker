use crate::{MedicationName, PatientId};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScheduleIdentity {
    patient: PatientId,
    medication: MedicationName,
}

impl ScheduleIdentity {
    pub fn new(patient: PatientId, medication: MedicationName) -> Self {
        Self {
            patient,
            medication,
        }
    }

    pub fn patient(&self) -> &PatientId {
        &self.patient
    }

    pub fn medication(&self) -> &MedicationName {
        &self.medication
    }
}
