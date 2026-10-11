/// A non-negative duration measured in milliseconds.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Duration(u64);

impl Duration {
    /// Creates a duration from milliseconds.
    pub fn from_millis(milliseconds: u64) -> Self {
        Self(milliseconds)
    }

    /// Returns the duration in milliseconds.
    pub fn as_millis(self) -> u64 {
        self.0
    }
}
