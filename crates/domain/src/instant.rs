use crate::Duration;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Instant(i64);

impl Instant {
    pub fn from_millis(milliseconds: i64) -> Self {
        Self(milliseconds)
    }

    pub fn as_millis(self) -> i64 {
        self.0
    }

    pub fn checked_add(self, duration: Duration) -> Option<Self> {
        i64::try_from(duration.as_millis())
            .ok()
            .and_then(|value| self.0.checked_add(value))
            .map(Self)
    }
}
