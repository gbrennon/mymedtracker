use crate::DomainError;

/// A validated medication name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MedicationName(String);

impl MedicationName {
    /// Creates a medication name and rejects blank input.
    pub fn try_new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(DomainError::EmptyMedicationName);
        }
        Ok(Self(value))
    }

    /// Returns the underlying medication name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for MedicationName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}
