use chrono::{DateTime, FixedOffset, NaiveDateTime, Offset};

/// Overrides the "local timezone offset" (in minutes) for the current thread.
///
/// Mermaid (notably Gantt) follows JavaScript local-time semantics in several places, which can
/// make fixtures produce different snapshot outputs on CI runners in different timezones.
///
/// This helper provides a minimally invasive mechanism: during the closure, treat "local time" as
/// a fixed `FixedOffset` for deterministic, reproducible snapshots. `None` uses the system
/// `chrono::Local` timezone.
pub fn with_fixed_local_offset_minutes<R>(offset_minutes: Option<i32>, f: impl FnOnce() -> R) -> R {
    crate::runtime::with_fixed_local_offset_minutes(offset_minutes, f)
}

/// Interprets a local `NaiveDateTime` as an absolute instant in the active local timezone.
///
/// When `with_fixed_local_offset_minutes(Some(x))` is active, the fixed offset is used. Otherwise,
/// the system local timezone is used.
pub fn datetime_from_naive_local(naive: NaiveDateTime) -> DateTime<FixedOffset> {
    crate::runtime::datetime_from_naive_local(naive)
}

/// Maps an absolute instant to the active local timezone (as a `FixedOffset`).
///
/// When `with_fixed_local_offset_minutes(Some(x))` is active, the fixed offset is used. Otherwise,
/// the system local timezone is used.
pub fn datetime_to_local_fixed(dt: DateTime<FixedOffset>) -> DateTime<FixedOffset> {
    crate::runtime::datetime_to_local_fixed(dt)
}

/// Returns the `NaiveDateTime` for an absolute instant under the active local-time semantics.
pub fn datetime_to_naive_local(dt: DateTime<FixedOffset>) -> NaiveDateTime {
    crate::runtime::datetime_to_naive_local(dt)
}

/// Returns the UTC fixed offset without fallible construction.
pub fn utc_fixed_offset() -> FixedOffset {
    chrono::Utc.fix()
}
