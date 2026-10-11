//! Pure domain entities, value objects, and medication-adherence invariants.
mod dose;
mod dose_status;
mod duration;
mod errors;
mod instant;
mod medication_name;
mod patient_id;
mod reminder_candidate;
mod schedule;
mod schedule_config;
mod schedule_identity;

pub use dose::Dose;
pub use dose_status::DoseStatus;
pub use duration::Duration;
pub use errors::DomainError;
pub use instant::Instant;
pub use medication_name::MedicationName;
pub use patient_id::PatientId;
pub use reminder_candidate::ReminderCandidate;
pub use schedule::Schedule;
pub use schedule_config::ScheduleConfig;
pub use schedule_identity::ScheduleIdentity;
