use medtrack_application::Clock;
use medtrack_domain::Instant;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

pub struct FakeClock {
    now: Arc<Mutex<Instant>>,
}

impl FakeClock {
    pub fn new(now: Instant) -> Self {
        Self {
            now: Arc::new(Mutex::new(now)),
        }
    }

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
