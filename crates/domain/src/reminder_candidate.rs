use crate::{Instant, MedicationName, PatientId};

/// A domain-approved overdue dose awaiting delivery by an application sink.
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

    /// Returns the patient who owns the overdue dose.
    pub fn patient(&self) -> &PatientId {
        &self.patient
    }

    /// Returns the overdue medication name.
    pub fn medication(&self) -> &MedicationName {
        &self.medication
    }

    /// Returns the scheduled instant of the overdue dose.
    pub fn due_at(&self) -> Instant {
        self.due_at
    }
}
