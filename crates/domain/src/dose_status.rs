#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DoseStatus {
    Scheduled,
    Taken,
    Overdue,
}
