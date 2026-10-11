use crate::{Instant, PatientId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DomainError {
    EmptyMedicationName,
    EmptyPatientId,
    InvalidDoseCount,
    InvalidInterval,
    GracePeriodMustBeShorterThanInterval,
    ScheduleOverflow,
    IntakeBeforeScheduleStart { at: Instant },
    IntakeOutsideDoseWindow { at: Instant },
    DuplicateDoseIntake { patient: PatientId, at: Instant },
    DoseAlreadyTaken,
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::EmptyMedicationName => "medication name cannot be empty",
            Self::EmptyPatientId => "patient id cannot be empty",
            Self::InvalidDoseCount => "dose count must be positive",
            Self::InvalidInterval => "dose interval must be positive",
            Self::GracePeriodMustBeShorterThanInterval => {
                "grace period must be shorter than dose interval"
            }
            Self::ScheduleOverflow => "schedule time overflowed",
            Self::IntakeBeforeScheduleStart { .. } => "intake is before schedule start",
            Self::IntakeOutsideDoseWindow { .. } => "intake is outside every dose window",
            Self::DuplicateDoseIntake { .. } => "dose intake was already recorded",
            Self::DoseAlreadyTaken => "dose was already taken",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for DomainError {}
