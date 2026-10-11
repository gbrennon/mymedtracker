use crate::Duration;

/// An instant measured as signed milliseconds from the Unix epoch.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Instant(i64);

impl Instant {
    /// Creates an instant from signed milliseconds.
    pub fn from_millis(milliseconds: i64) -> Self {
        Self(milliseconds)
    }

    /// Returns the instant in milliseconds.
    pub fn as_millis(self) -> i64 {
        self.0
    }

    /// Adds a duration, returning `None` when the signed range would overflow.
    pub fn checked_add(self, duration: Duration) -> Option<Self> {
        i64::try_from(duration.as_millis())
            .ok()
            .and_then(|value| self.0.checked_add(value))
            .map(Self)
    }
}
