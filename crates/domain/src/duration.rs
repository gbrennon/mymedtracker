#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Duration(u64);

impl Duration {
    pub fn from_millis(milliseconds: u64) -> Self {
        Self(milliseconds)
    }

    pub fn as_millis(self) -> u64 {
        self.0
    }
}
