use crate::{Instant, PatientId};

/// Errors raised when a medication schedule violates a domain invariant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DomainError {
    /// The medication name contains no non-whitespace characters.
    EmptyMedicationName,
    /// The patient identifier contains no non-whitespace characters.
    EmptyPatientId,
    /// A schedule must contain at least one dose.
    InvalidDoseCount,
    /// A schedule interval must be positive.
    InvalidInterval,
    /// A grace period must not overlap the next dose interval.
    GracePeriodMustBeShorterThanInterval,
    /// Schedule generation exceeded the supported instant range.
    ScheduleOverflow,
    /// An intake occurred before the first scheduled dose.
    IntakeBeforeScheduleStart {
        /// The rejected intake instant.
        at: Instant,
    },
    /// An intake did not fall inside any dose window.
    IntakeOutsideDoseWindow {
        /// The rejected intake instant.
        at: Instant,
    },
    /// An intake targeted a dose that had already been recorded.
    DuplicateDoseIntake {
        /// The patient whose dose was duplicated.
        patient: PatientId,
        /// The duplicate intake instant.
        at: Instant,
    },
    /// A direct dose transition attempted to record a second intake.
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
