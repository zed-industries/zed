/// Severity levels that determine the style of the component.
/// Usually, it affects the background. Most of the time,
/// it also follows with an icon corresponding the severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Info,
    Success,
    Warning,
    Error,
}
