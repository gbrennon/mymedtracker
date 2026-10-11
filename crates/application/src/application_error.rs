use medtrack_domain::DomainError;

/// Failures returned by application use cases and their outbound ports.
#[derive(Debug)]
pub enum ApplicationError {
    /// A domain invariant rejected the requested operation.
    Domain(DomainError),
    /// No schedule exists for the requested patient and medication.
    ScheduleNotFound,
}

impl From<DomainError> for ApplicationError {
    fn from(error: DomainError) -> Self {
        Self::Domain(error)
    }
}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Domain(error) => error.fmt(formatter),
            Self::ScheduleNotFound => formatter.write_str("schedule was not found"),
        }
    }
}

impl std::error::Error for ApplicationError {}
