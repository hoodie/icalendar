//! Error types for recurrence rule handling.

/// Errors that can occur when setting or parsing recurrence rules.
///
/// Returned by [`EventLike::recurrence`](crate::EventLike::recurrence) and
/// [`EventLike::try_recurrence`](crate::EventLike::try_recurrence).
#[cfg(feature = "recurrence")]
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum RecurrenceError {
    /// `DTSTART` was not set on the component before calling
    /// [`recurrence()`](crate::EventLike::recurrence). Call `.starts()` or
    /// `.all_day()` first.
    #[error("DTSTART must be set before calling recurrence()")]
    MissingDtStart,

    /// The `TZID` parameter on `DTSTART` could not be resolved to a known
    /// timezone.
    #[error("unrecognised timezone in DTSTART: {0}")]
    InvalidTimezone(String),

    /// The local datetime in `DTSTART` is ambiguous or invalid for the given
    /// timezone (e.g. a time that falls in a DST gap).
    #[error("the local datetime in DTSTART is ambiguous or invalid for its timezone")]
    AmbiguousDateTime,

    /// The `DTSTART` property value could not be parsed.
    #[error("could not parse DTSTART property value")]
    InvalidDtStart,

    /// The recurrence rule itself failed rrule's own validation or parsing.
    #[error("recurrence rule error: {0}")]
    Rule(#[from] rrule::RRuleError),
}
