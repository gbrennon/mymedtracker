use crate::{DomainError, Duration, Instant};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScheduleConfig {
    first_dose_at: Instant,
    interval: Duration,
    grace_period: Duration,
    dose_count: usize,
}

impl ScheduleConfig {
    pub fn new(
        first_dose_at: Instant,
        interval: Duration,
        grace_period: Duration,
        dose_count: usize,
    ) -> Result<Self, DomainError> {
        if interval.as_millis() == 0 {
            return Err(DomainError::InvalidInterval);
        }
        if grace_period >= interval {
            return Err(DomainError::GracePeriodMustBeShorterThanInterval);
        }
        if dose_count == 0 {
            return Err(DomainError::InvalidDoseCount);
        }
        Ok(Self {
            first_dose_at,
            interval,
            grace_period,
            dose_count,
        })
    }

    pub(crate) fn first_dose_at(self) -> Instant {
        self.first_dose_at
    }

    pub(crate) fn interval(self) -> Duration {
        self.interval
    }

    pub(crate) fn grace_period(self) -> Duration {
        self.grace_period
    }

    pub(crate) fn dose_count(self) -> usize {
        self.dose_count
    }
}
