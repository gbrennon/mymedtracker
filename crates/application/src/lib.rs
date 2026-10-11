//! Application use cases and outbound ports for medication adherence workflows.
mod application_error;
mod intake_event;
mod port;
mod reminder_message;
mod use_case;

pub use application_error::ApplicationError;
pub use intake_event::IntakeEvent;
pub use port::{Clock, PatientStore, ReminderSink};
pub use reminder_message::ReminderMessage;
pub use use_case::{RegisterSchedule, SchedulerTick, SubmitIntake};
