use crate::{MedicationName, PatientId};

/// Identifies one medication schedule for one patient.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScheduleIdentity {
    patient: PatientId,
    medication: MedicationName,
}

impl ScheduleIdentity {
    /// Creates an identity from a patient and medication.
    pub fn new(patient: PatientId, medication: MedicationName) -> Self {
        Self {
            patient,
            medication,
        }
    }

    /// Returns the patient component of this identity.
    pub fn patient(&self) -> &PatientId {
        &self.patient
    }

    /// Returns the medication component of this identity.
    pub fn medication(&self) -> &MedicationName {
        &self.medication
    }
}
