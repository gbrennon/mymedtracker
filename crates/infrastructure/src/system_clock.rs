use medtrack_application::Clock;
use medtrack_domain::Instant;
use std::pin::Pin;
use std::time::{SystemTime, UNIX_EPOCH};

/// Reads the host system clock and converts it to a domain instant.
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> Pin<Box<dyn Future<Output = Instant> + Send + '_>> {
        Box::pin(async move {
            let elapsed = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default();
            let milliseconds = i64::try_from(elapsed.as_millis()).unwrap_or(i64::MAX);
            Instant::from_millis(milliseconds)
        })
    }
}
