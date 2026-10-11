use medtrack_domain::{Instant, MedicationName, PatientId};

/// An externally observed medication intake event.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntakeEvent {
    patient: PatientId,
    medication: MedicationName,
    at: Instant,
}

impl IntakeEvent {
    /// Creates an intake event for a patient, medication, and observed instant.
    pub fn new(patient: PatientId, medication: MedicationName, at: Instant) -> Self {
        Self {
            patient,
            medication,
            at,
        }
    }

    /// Returns the patient and medication used to locate the schedule.
    pub fn identity(&self) -> (&PatientId, &MedicationName) {
        (&self.patient, &self.medication)
    }

    /// Returns the observed intake instant.
    pub fn at(&self) -> Instant {
        self.at
    }
}
