use medtrack_domain::DomainError;

#[derive(Debug)]
pub enum ApplicationError {
    Domain(DomainError),
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
