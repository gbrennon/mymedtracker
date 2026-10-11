#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// The derived lifecycle state of a scheduled dose.
pub enum DoseStatus {
    /// The dose is awaiting its intake window.
    Scheduled,
    /// The dose has a recorded intake.
    Taken,
    /// The dose passed its grace period without intake.
    Overdue,
}
