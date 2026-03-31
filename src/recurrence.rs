#![allow(unused_qualifications)]
use crate::{
    Property,
    components::date_time::{CalendarDateTime, DatePerhapsTime},
};
use chrono::{DateTime, TimeZone as _};

pub use rrule::{self, Frequency, NWeekday, RRule, RRuleSet, Tz, Weekday};

use rrule::Unvalidated;

/// A not-yet-validated recurrence rule. Alias for [`RRule<Unvalidated>`].
///
/// Use this type when storing or returning an [`RRule`] that has not yet been
/// bound to a start date, for example in helper functions or struct fields.
/// At the call site of [`EventLike::recurrence`] the type is always inferred,
/// so you only need to name it explicitly when the compiler asks you to.
pub type UnvalidatedRRule = RRule<Unvalidated>;

/// Converts a `DTSTART` [`Property`] into a [`chrono::DateTime<rrule::Tz>`] suitable
/// for use with the `rrule` crate.
///
/// Extracted from [`EventLike::recurrence`] for testability.
pub(crate) fn dt_start_to_rrule_datetime(
    property: &Property,
) -> Result<DateTime<rrule::Tz>, RecurrenceError> {
    match DatePerhapsTime::from_property(property) {
        Some(DatePerhapsTime::DateTime(CalendarDateTime::Utc(utc))) => {
            Ok(rrule::Tz::UTC.from_utc_datetime(&utc.naive_utc()))
        }
        Some(DatePerhapsTime::DateTime(CalendarDateTime::WithTimezone { date_time, tzid })) => {
            let tz: rrule::Tz = tzid
                .parse::<chrono_tz::Tz>()
                .map_err(|_| RecurrenceError::InvalidTimezone(tzid.clone()))?
                .into();
            tz.from_local_datetime(&date_time)
                .single()
                .ok_or(RecurrenceError::AmbiguousDateTime)
        }
        Some(DatePerhapsTime::DateTime(CalendarDateTime::Floating(naive))) => Ok(rrule::Tz::LOCAL
            .from_local_datetime(&naive)
            .single()
            .ok_or(RecurrenceError::AmbiguousDateTime)?),

        Some(DatePerhapsTime::Date(naive_date)) => Ok(rrule::Tz::LOCAL
            .from_local_datetime(&naive_date.and_hms_opt(0, 0, 0).unwrap())
            .single()
            .ok_or(RecurrenceError::AmbiguousDateTime)?),
        None => Err(RecurrenceError::InvalidDtStart),
    }
}

/// Errors that can occur when setting or parsing recurrence rules.
///
/// Returned by [`EventLike::recurrence`](crate::EventLike::recurrence) and
/// [`EventLike::try_recurrence`](crate::EventLike::try_recurrence).
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

#[cfg(all(test, feature = "parser"))]
mod test_recurrence_tzid {
    use crate::{
        Calendar, Event, EventLike, Frequency, NWeekday, RRule, Tz, UnvalidatedRRule, Weekday,
        components::date_time::CalendarDateTime,
    };
    use chrono::{NaiveDate, TimeZone, Utc};

    /// Builds an unbuilt weekly `RRule` for UTC tests.
    fn weekly_utc_rrule() -> UnvalidatedRRule {
        rrule::RRule::default().count(4).freq(Frequency::Weekly)
    }

    /// A `DTSTART;TZID=...` event should produce occurrences in the named timezone,
    /// not in UTC or the local machine timezone.
    #[test]
    fn tzid_dtstart_preserves_timezone() {
        let rrule = RRule::default()
            .count(3)
            .freq(Frequency::Weekly)
            .by_weekday(vec![NWeekday::Every(Weekday::Mon)]);

        let dt_start_ical = CalendarDateTime::WithTimezone {
            date_time: NaiveDate::from_ymd_opt(2025, 6, 2)
                .unwrap()
                .and_hms_opt(10, 0, 0)
                .unwrap(),
            tzid: "Europe/Berlin".to_string(),
        };

        let event = Event::new()
            .starts(dt_start_ical)
            .recurrence(rrule)
            .unwrap()
            .done();

        let rrule_set_out = event
            .get_recurrence()
            .expect("event should have a recurrence rule");
        let dates = rrule_set_out.all(10).dates;

        assert_eq!(dates.len(), 3);
        // All occurrences must be in Europe/Berlin, not UTC
        for dt in &dates {
            assert_eq!(dt.timezone(), Tz::Europe__Berlin);
            assert_eq!(dt.format("%H:%M").to_string(), "10:00");
        }
    }

    /// Serializing an event with `DTSTART;TZID=...` and parsing it back must yield the
    /// same occurrences as the original (round-trip correctness).
    #[test]
    fn tzid_dtstart_round_trips_through_serialization() {
        let rrule = RRule::default()
            .count(3)
            .freq(Frequency::Weekly)
            .by_weekday(vec![NWeekday::Every(Weekday::Mon)]);

        let dt_start_ical = CalendarDateTime::WithTimezone {
            date_time: NaiveDate::from_ymd_opt(2025, 6, 2)
                .unwrap()
                .and_hms_opt(10, 0, 0)
                .unwrap(),
            tzid: "Europe/Berlin".to_string(),
        };

        let event = Event::new()
            .starts(dt_start_ical)
            .recurrence(rrule)
            .unwrap()
            .done();

        let original_dates = event
            .get_recurrence()
            .expect("event should have a recurrence rule")
            .all(10)
            .dates;

        // Serialize → parse back
        let mut calendar = Calendar::new();
        calendar.push(event);
        let serialized = calendar.to_string();
        let reparsed: Calendar = serialized.parse().unwrap();

        let reparsed_event = reparsed.events().next().unwrap();

        let reparsed_dates = reparsed_event
            .get_recurrence()
            .expect("reparsed event should have a recurrence rule")
            .all(10)
            .dates;

        assert_eq!(original_dates, reparsed_dates);
        assert_eq!(reparsed_dates.len(), 3);
    }

    /// A UTC `DTSTART` (no TZID parameter) must still work correctly after the refactor.
    #[test]
    fn utc_dtstart_still_works() {
        let utc_dt = Utc.with_ymd_and_hms(2025, 3, 17, 9, 0, 0).unwrap();

        let event = Event::new()
            .starts(CalendarDateTime::Utc(utc_dt))
            .recurrence(weekly_utc_rrule())
            .unwrap()
            .done();

        let dates = event
            .get_recurrence()
            .expect("event should have a recurrence rule")
            .all(10)
            .dates;
        assert_eq!(dates.len(), 4);
        for dt in &dates {
            assert_eq!(dt.timezone(), Tz::UTC);
        }
    }

    /// A floating `DTSTART` must survive a serialize → parse round-trip:
    /// the `Tz::LOCAL` tag and wall-clock times must be preserved.
    #[test]
    fn floating_dtstart_round_trips_through_serialization() {
        let naive_dt = NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(9, 0, 0)
            .unwrap();

        let event = Event::new()
            .starts(CalendarDateTime::Floating(naive_dt))
            .recurrence(RRule::default().count(3).freq(Frequency::Daily))
            .unwrap()
            .done();

        let original_rrule_set = event
            .get_recurrence()
            // .expect("event should have a recurrence rule")
            .expect("recurrence rule should be valid");

        assert_eq!(original_rrule_set.get_dt_start().timezone(), Tz::LOCAL);

        let mut calendar = Calendar::new();
        calendar.push(event);
        let reparsed: Calendar = calendar.to_string().parse().unwrap();

        let reparsed_event = reparsed.events().next().unwrap();

        let reparsed_rrule_set = reparsed_event
            .get_recurrence()
            .expect("reparsed recurrence rule should be valid");

        assert_eq!(reparsed_rrule_set.get_dt_start().timezone(), Tz::LOCAL);
        assert_eq!(
            original_rrule_set.get_dt_start().naive_local(),
            reparsed_rrule_set.get_dt_start().naive_local(),
            "floating DTSTART wall-clock time must survive round-trip"
        );
        assert_eq!(reparsed_rrule_set.all(10).dates.len(), 3);
    }

    /// An all-day (DATE) `DTSTART` must survive a serialize → parse round-trip:
    /// the `Tz::LOCAL` tag and midnight wall-clock time must be preserved.
    #[test]
    fn all_day_dtstart_round_trips_through_serialization() {
        let naive_date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();

        let event = Event::new()
            .all_day(naive_date)
            .recurrence(RRule::default().count(3).freq(Frequency::Daily))
            .unwrap()
            .done();

        let original_rrule_set = event
            .get_recurrence()
            // .expect("event should have a recurrence rule")
            .expect("recurrence rule should be valid");

        assert_eq!(original_rrule_set.get_dt_start().timezone(), Tz::LOCAL);

        let mut calendar = Calendar::new();
        calendar.push(event);
        let reparsed: Calendar = calendar.to_string().parse().unwrap();

        let reparsed_event = reparsed.events().next().unwrap();

        let reparsed_rrule_set = reparsed_event
            .get_recurrence()
            // .expect("reparsed event should have a recurrence rule")
            .expect("reparsed recurrence rule should be valid");

        assert_eq!(reparsed_rrule_set.get_dt_start().timezone(), Tz::LOCAL);
        assert_eq!(
            original_rrule_set.get_dt_start().naive_local(),
            reparsed_rrule_set.get_dt_start().naive_local(),
            "all-day DTSTART midnight must survive round-trip"
        );
        assert_eq!(reparsed_rrule_set.all(10).dates.len(), 3);
    }
}

#[cfg(test)]
mod test_recurrence_errors {
    use crate::{
        Component, Event, EventLike as _, Frequency, RRule, RecurrenceError,
        components::date_time::CalendarDateTime,
    };
    use chrono::{NaiveDate, TimeZone, Utc};

    /// An event with only DTSTART (no RRULE, no RDATE) should still return Ok,
    /// yielding the start date itself as the sole occurrence.
    #[test]
    fn no_rrule_returns_single_occurrence() {
        let dt = Utc.with_ymd_and_hms(2025, 1, 1, 9, 0, 0).unwrap();
        let event = Event::new().starts(dt).done();

        let rruleset = event
            .get_recurrence()
            .expect("expected Ok even without RRULE");
        let dates = rruleset.all(10).dates;
        assert_eq!(dates.len(), 1);
        assert_eq!(dates.first().unwrap().timestamp(), dt.timestamp());
    }

    /// An event with a valid RRULE should return Some(_) / Some(Ok(_)).
    #[test]
    fn valid_rrule_returns_some() {
        let event = Event::new()
            .starts(Utc.with_ymd_and_hms(2025, 1, 1, 9, 0, 0).unwrap())
            .recurrence(RRule::default().count(3).freq(Frequency::Daily))
            .unwrap()
            .done();

        assert!(event.get_recurrence().is_ok());
    }

    /// An event with a syntactically invalid RRULE value should return None / Some(Err(_)).
    #[test]
    fn invalid_rrule_returns_none_and_some_err() {
        let event = Event::new()
            .starts(Utc.with_ymd_and_hms(2025, 1, 1, 9, 0, 0).unwrap())
            .add_property("RRULE", "THIS IS NOT VALID")
            .done();

        assert!(event.get_recurrence().is_err());
        assert!(matches!(
            event.get_recurrence(),
            Err(RecurrenceError::Rule(_))
        ));
    }

    /// Calling `recurrence()` before setting DTSTART should return `MissingDtStart`.
    #[test]
    fn missing_dtstart_returns_error() {
        let mut event = Event::new();
        let result = event.recurrence(RRule::default().freq(Frequency::Daily));
        assert!(matches!(result, Err(RecurrenceError::MissingDtStart)));
    }

    /// An unrecognised TZID in DTSTART should return `InvalidTimezone`.
    #[test]
    fn invalid_timezone_returns_error() {
        let dt_start = CalendarDateTime::WithTimezone {
            date_time: NaiveDate::from_ymd_opt(2025, 1, 1)
                .unwrap()
                .and_hms_opt(9, 0, 0)
                .unwrap(),
            tzid: "Not/ATimezone".to_string(),
        };

        let mut event = Event::new();
        event.starts(dt_start);
        let result = event.recurrence(RRule::default().freq(Frequency::Daily));

        assert!(matches!(
            result,
            Err(RecurrenceError::InvalidTimezone(tz)) if tz == "Not/ATimezone"
        ));
    }
}

#[cfg(test)]
mod test_rdates {

    use std::vec;

    use chrono::TimeZone as _;
    use chrono_tz::Europe::Berlin;

    use crate::{
        Calendar, Component, Event, EventLike as _, components::date_time::CalendarDateTime,
    };

    #[test]
    fn use_rdates_for_recurrence() {
        let mut all_hands = Event::new()
            .uid("all_hands_2026@example.com")
            .summary("All-Hands Meeting")
            .description("Monthly all-hands. First Monday of each month, 09:00–10:00 Berlin time.")
            .starts(CalendarDateTime::from_ymd_hm_tzid(2026, 1, 5, 9, 0, Berlin).unwrap())
            .ends(CalendarDateTime::from_ymd_hm_tzid(2026, 1, 5, 10, 0, Berlin).unwrap())
            .rdate(CalendarDateTime::from_ymd_hm_tzid(2026, 1, 6, 9, 0, Berlin).unwrap())
            .rdate(CalendarDateTime::from_ymd_hm_tzid(2026, 1, 7, 9, 0, Berlin).unwrap())
            .rdate(CalendarDateTime::from_ymd_hm_tzid(2026, 1, 8, 9, 0, Berlin).unwrap())
            // .recurrence(RRule::new(Frequency::Monthly).by_weekday(vec![NWeekday::Nth(1, Weekday::Mon)])).unwrap()
            .done();

        // cancel the December instance
        let december_instance =
            CalendarDateTime::from_ymd_hm_tzid(2026, 12, 7, 9, 0, Berlin).unwrap();
        all_hands.exdate(december_instance);

        let mut calendar = Calendar::new();
        calendar.push(all_hands);

        let recurrences = calendar
            .events()
            .next()
            .unwrap()
            .to_owned()
            .get_recurrence()
            .unwrap()
            .all(100)
            .dates;

        let expected = vec![
            Berlin.ymd(2026, 1, 5).and_hms(9, 0, 0),
            Berlin.ymd(2026, 1, 6).and_hms(9, 0, 0),
            Berlin.ymd(2026, 1, 7).and_hms(9, 0, 0),
            Berlin.ymd(2026, 1, 8).and_hms(9, 0, 0),
            // December instance should be excluded by EXDATE
            // Berlin.ymd(2026, 12, 7).and_hms(9, 0, 0)
            // Emergency session should be included by RDATE
        ]
        .into_iter()
        .collect::<Vec<_>>();

        assert_eq!(
            recurrences
                .into_iter()
                .map(|dt| dt.with_timezone(&chrono_tz::Europe::Berlin))
                .collect::<Vec<_>>(),
            expected
        );
    }
}

#[cfg(test)]
mod test_recurrence_properties {
    use crate::{
        Component, Event, EventLike as _, Frequency, RRule, Tz,
        components::date_time::CalendarDateTime,
    };
    use chrono::{NaiveDate, TimeZone as _, Utc};

    use super::dt_start_to_rrule_datetime;

    /// Calling `recurrence()` without any RDATEs or EXDATEs must not write
    /// blank `RDATE` or `EXDATE` multi-properties onto the component.
    #[test]
    fn no_spurious_rdate_or_exdate_properties() {
        let event = Event::new()
            .starts(Utc.with_ymd_and_hms(2025, 1, 1, 9, 0, 0).unwrap())
            .recurrence(RRule::default().count(3).freq(Frequency::Daily))
            .unwrap()
            .done();

        let multi = event.multi_properties();
        assert!(
            !multi.contains_key("RDATE"),
            "RDATE should not be present when no RDATEs were supplied"
        );
        assert!(
            !multi.contains_key("EXDATE"),
            "EXDATE should not be present when no EXDATEs were supplied"
        );
    }

    /// The absence of blank RDATE/EXDATE properties must also hold in the
    /// serialized ICS output — no blank-value lines should appear.
    #[test]
    fn serialized_output_contains_no_blank_rdate_or_exdate() {
        let event = Event::new()
            .starts(Utc.with_ymd_and_hms(2025, 1, 1, 9, 0, 0).unwrap())
            .recurrence(RRule::default().count(3).freq(Frequency::Daily))
            .unwrap()
            .done();

        let ics = event.to_string();
        for line in ics.lines() {
            let key = line.split(':').next().unwrap_or("");
            assert!(
                key != "RDATE",
                "unexpected blank RDATE line in serialized output"
            );
            assert!(
                key != "EXDATE",
                "unexpected blank EXDATE line in serialized output"
            );
        }
    }

    /// The DTSTART-to-rrule-datetime conversion for a floating datetime must produce
    /// a `Tz::LOCAL`-tagged instant, matching what rrule's string parser returns for
    /// the same `DTSTART:YYYYMMDDTHHmmss` value.
    #[test]
    fn floating_dtstart_produces_correct_occurrences() {
        let naive_dt = NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(9, 0, 0)
            .unwrap();

        // Build a Property the same way `starts(CalendarDateTime::Floating(...))` would.
        let prop = CalendarDateTime::Floating(naive_dt).to_property("DTSTART");

        let dt_start =
            dt_start_to_rrule_datetime(&prop).expect("floating DTSTART should be convertible");

        assert_eq!(
            dt_start.timezone(),
            Tz::LOCAL,
            "floating DTSTART must be interpreted as Tz::LOCAL (not Tz::UTC) to match rrule's string parser"
        );
        assert_eq!(dt_start.naive_local().time(), naive_dt.time());
    }

    /// The DTSTART-to-rrule-datetime conversion for an all-day date must produce
    /// a `Tz::LOCAL`-tagged midnight instant, matching what rrule's string parser
    /// returns for the same `DTSTART:YYYYMMDD` value.
    #[test]
    fn all_day_dtstart_produces_correct_occurrences() {
        let naive_date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();

        // Build a Property the same way `all_day(naive_date)` would.
        use crate::components::date_time::naive_date_to_property;
        let prop = naive_date_to_property(naive_date, "DTSTART");

        let dt_start =
            dt_start_to_rrule_datetime(&prop).expect("all-day DTSTART should be convertible");

        assert_eq!(
            dt_start.timezone(),
            Tz::LOCAL,
            "all-day DTSTART must be interpreted as Tz::LOCAL (not Tz::UTC) to match rrule's string parser"
        );
        assert_eq!(
            dt_start.naive_local().time(),
            chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap()
        );
    }
}
