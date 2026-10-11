use medtrack_domain::{Instant, MedicationName, PatientId};

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

    pub fn patient(&self) -> &PatientId {
        &self.patient
    }

    pub fn medication(&self) -> &MedicationName {
        &self.medication
    }

    pub fn due_at(&self) -> Instant {
        self.due_at
    }

    pub fn sent_at(&self) -> Instant {
        self.sent_at
    }
}
