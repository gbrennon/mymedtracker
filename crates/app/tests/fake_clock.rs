use medtrack_application::Clock;
use medtrack_domain::Instant;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

/// A manually advanced clock for deterministic scheduler tests.
pub struct FakeClock {
    now: Arc<Mutex<Instant>>,
}

impl FakeClock {
    /// Creates a fake clock at the supplied instant.
    pub fn new(now: Instant) -> Self {
        Self {
            now: Arc::new(Mutex::new(now)),
        }
    }

    /// Advances the clock without waiting on wall time.
    pub fn advance_to(&self, now: Instant) {
        *self.now.lock().expect("clock lock") = now;
    }
}

impl Clock for FakeClock {
    fn now(&self) -> Pin<Box<dyn Future<Output = Instant> + Send + '_>> {
        let now = self.now.clone();
        Box::pin(async move { *now.lock().expect("clock lock") })
    }
}
