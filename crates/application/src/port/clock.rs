use medtrack_domain::Instant;
use std::future::Future;
use std::pin::Pin;

pub trait Clock: Send + Sync {
    fn now(&self) -> Pin<Box<dyn Future<Output = Instant> + Send + '_>>;
}
