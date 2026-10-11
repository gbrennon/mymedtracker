use medtrack_domain::{Instant, MedicationName, PatientId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntakeEvent {
    patient: PatientId,
    medication: MedicationName,
    at: Instant,
}

impl IntakeEvent {
    pub fn new(patient: PatientId, medication: MedicationName, at: Instant) -> Self {
        Self {
            patient,
            medication,
            at,
        }
    }

    pub fn identity(&self) -> (&PatientId, &MedicationName) {
        (&self.patient, &self.medication)
    }

    pub fn at(&self) -> Instant {
        self.at
    }
}
