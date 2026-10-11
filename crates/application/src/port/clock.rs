use medtrack_domain::Instant;
use std::pin::Pin;

/// Supplies the current instant to time-dependent application workflows.
pub trait Clock: Send + Sync {
    /// Returns the current instant without imposing a concrete time source.
    fn now(&self) -> Pin<Box<dyn Future<Output = Instant> + Send + '_>>;
}
