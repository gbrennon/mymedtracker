use medtrack_domain::{Instant, MedicationName, PatientId};

/// A reminder message ready for delivery by an outbound sink.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReminderMessage {
    patient: PatientId,
    medication: MedicationName,
    due_at: Instant,
    sent_at: Instant,
}

impl ReminderMessage {
    pub(crate) fn new(
        patient: PatientId,
        medication: MedicationName,
        due_at: Instant,
        sent_at: Instant,
    ) -> Self {
        Self {
            patient,
            medication,
            due_at,
            sent_at,
        }
    }

    /// Returns the patient who should receive this reminder.
    pub fn patient(&self) -> &PatientId {
        &self.patient
    }

    /// Returns the medication associated with this reminder.
    pub fn medication(&self) -> &MedicationName {
        &self.medication
    }

    /// Returns the originally scheduled dose instant.
    pub fn due_at(&self) -> Instant {
        self.due_at
    }

    /// Returns the clock instant when the application emitted the reminder.
    pub fn sent_at(&self) -> Instant {
        self.sent_at
    }
}
