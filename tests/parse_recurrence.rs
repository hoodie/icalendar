#![cfg(all(feature = "recurrence", feature = "parser"))]
use chrono::{DateTime, TimeZone};
use icalendar::{Calendar, CalendarComponent, EventLike, Tz};

/// An event whose DTSTART uses `TZID=Europe/Berlin` but whose RDATE and EXDATE
/// use `TZID=America/New_York`.
///
/// This is the critical regression test for the parameter-stripping bug: when
/// `TZID` is dropped from `RDATE`/`EXDATE` before passing to the rrule parser,
/// those bare datetime strings get interpreted in the DTSTART timezone
/// (`Europe/Berlin`) instead of their own timezone (`America/New_York`).
///
/// - `DTSTART;TZID=Europe/Berlin:20250101T160000` — Jan 1, 16:00 Berlin = 15:00 UTC
/// - `RRULE:FREQ=DAILY;COUNT=4` — generates Jan 1, 2, 3, 4 at 16:00 Berlin
/// - `RDATE;TZID=America/New_York:20241231T100000` — Dec 31, 10:00 NYC = 15:00 UTC
/// - `EXDATE;TZID=America/New_York:20250102T100000` — Jan 2, 10:00 NYC = 15:00 UTC
///
/// Without the fix, both the RDATE and EXDATE lose their `TZID=America/New_York`
/// and are interpreted as 10:00 Berlin (= 09:00 UTC). That makes the RDATE a
/// different instant (doesn't match any occurrence) and the EXDATE also a
/// different instant (fails to exclude Jan 2), producing wrong results.
///
/// With the fix the RDATE correctly adds Dec 31 at 15:00 UTC and the EXDATE
/// correctly removes Jan 2 at 15:00 UTC, giving: Dec 31, Jan 1, Jan 3, Jan 4.
const TZID_RDATE_EXDATE_STR: &str = r#"BEGIN:VCALENDAR
VERSION:2.0
PRODID:-//test//test//EN
BEGIN:VEVENT
UID:tzid-rdate-exdate-test
DTSTART;TZID=Europe/Berlin:20250101T160000
RRULE:FREQ=DAILY;COUNT=4
RDATE;TZID=America/New_York:20241231T100000
EXDATE;TZID=America/New_York:20250102T100000
END:VEVENT
END:VCALENDAR
"#;

fn naive_dates(dates: &[DateTime<Tz>]) -> Vec<String> {
    dates
        .iter()
        .map(|dt| dt.naive_local().format("%Y-%m-%dT%H:%M:%S").to_string())
        .collect()
}

const TEST_CALENDAR_STR: &str = r#"
BEGIN:VCALENDAR
VERSION:2.0
PRODID:bsprodidfortestabc123
BEGIN:VEVENT
DTSTAMP:20050118T211523Z
UID:bsuidfortestabc123
DTSTART;VALUE=DATE:20250101
DTEND;VALUE=DATE:20250101
RDATE;VALUE=DATE:20241231,20241230
SUMMARY:Test Recurrence
EXDATE;VALUE=DATE:20250102,20250103
RRULE:FREQ=DAILY;COUNT=4
END:VEVENT
BEGIN:VEVENT
DTSTAMP:20050118T211523Z
UID:bsuidfortestabc123
DTSTART:20250101T090000Z
DTEND:20250101T110000Z
RDATE:20241231T100000Z
SUMMARY:Test Recurrence
EXDATE:20250102T090000Z,20250103T100000Z
RRULE:FREQ=DAILY;COUNT=4
END:VEVENT
END:VCALENDAR
"#;

/// Verifies that `TZID` parameters on `RDATE` and `EXDATE` are preserved when
/// the properties are serialized for the rrule parser.
///
/// DTSTART is in `Europe/Berlin`, but RDATE and EXDATE are in `America/New_York`.
/// The two timezones are chosen so that 10:00 NYC == 16:00 Berlin (both = 15:00 UTC
/// in winter), meaning the RDATE and EXDATE instants align with the RRULE occurrences
/// only when the correct `TZID` is used. If `TZID` is stripped, 10:00 is interpreted
/// as 10:00 Berlin (= 09:00 UTC) — a completely different instant — so the RDATE
/// would not contribute an occurrence and the EXDATE would not remove one.
///
/// Note: rrule preserves each date in the timezone it was given, so RDATE occurrences
/// come back as `America/New_York` and RRULE occurrences as `Europe/Berlin`. We
/// therefore compare UTC instants rather than timezone-tagged values.
#[test]
fn rdate_and_exdate_tzid_params_are_preserved() {
    let calendar: Calendar = TZID_RDATE_EXDATE_STR
        .parse()
        .expect("failed to parse calendar");

    let event = match calendar.components.first() {
        Some(CalendarComponent::Event(e)) => e,
        _ => panic!("first component should be an event"),
    };

    let rrule_set = event
        .get_recurrence()
        .expect("event should have a valid recurrence rule");

    let dates: Vec<DateTime<Tz>> = rrule_set.all(10).dates;

    // Dec 31 (RDATE) + Jan 1, 3, 4 (RRULE minus EXDATE on Jan 2) = 4 occurrences.
    assert_eq!(dates.len(), 4, "expected 4 occurrences, got: {dates:?}");

    // Compare as UTC instants: DTSTART occurrences are 16:00 Berlin = 15:00 UTC;
    // RDATE/EXDATE are 10:00 NYC = 15:00 UTC. All four land on the same UTC hour.
    // Without the TZID fix, RDATE/EXDATE would be 10:00 Berlin = 09:00 UTC — a
    // different instant — so Dec 31 would be absent and Jan 2 would be present.
    let utc_timestamps: Vec<String> = dates
        .iter()
        .map(|dt| {
            dt.with_timezone(&Tz::UTC)
                .format("%Y-%m-%dT%H:%M:%SZ")
                .to_string()
        })
        .collect();

    assert_eq!(
        utc_timestamps,
        vec![
            "2024-12-31T15:00:00Z",
            "2025-01-01T15:00:00Z",
            "2025-01-03T15:00:00Z",
            "2025-01-04T15:00:00Z",
        ]
    );
}

#[test]
fn parse_recurrence() {
    // tests recurrence handling for all-day events (dates) and datetimes

    // all-day: calendar string excludes 2nd and 3rd as EXDATE, but includes 30th and 31st as RDATE
    // Bare DATE-only DTSTART has no TZID, so rrule treats it as local/floating — compare naive dates only.
    let expected_naive_dates_a = vec![
        "2024-12-30T00:00:00",
        "2024-12-31T00:00:00",
        "2025-01-01T00:00:00",
        "2025-01-04T00:00:00",
    ];

    // time-based: calendar string excludes 2nd as EXDATE and does NOT exclude 3rd, and includes 31st as RDATE
    let expected_datetimes_b = vec![
        Tz::UTC.with_ymd_and_hms(2024, 12, 31, 10, 0, 0).unwrap(),
        Tz::UTC.with_ymd_and_hms(2025, 1, 1, 9, 0, 0).unwrap(),
        Tz::UTC.with_ymd_and_hms(2025, 1, 3, 9, 0, 0).unwrap(),
        Tz::UTC.with_ymd_and_hms(2025, 1, 4, 9, 0, 0).unwrap(),
    ];

    let calendar: Calendar = TEST_CALENDAR_STR.parse().expect("failed to parse calendar");
    assert_eq!(calendar.components.len(), 2);

    let event = match calendar.components.first() {
        Some(CalendarComponent::Event(event)) => event,
        _ => panic!("calendar component should be an event"),
    };
    let rrules = event
        .get_recurrence()
        .expect("event should have recurrence rules");
    let datetimes: Vec<DateTime<Tz>> = rrules.all(10).dates;
    assert_eq!(naive_dates(&datetimes), expected_naive_dates_a);

    let event = match calendar.components.get(1) {
        Some(CalendarComponent::Event(event)) => event,
        _ => panic!("calendar component should be an event"),
    };
    let rrules = event
        .get_recurrence()
        .expect("event should have recurrence rules");
    let datetimes: Vec<DateTime<Tz>> = rrules.all(10).dates;
    assert_eq!(datetimes, expected_datetimes_b);
}
