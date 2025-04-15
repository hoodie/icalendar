#![cfg(feature = "recurrence")]
use chrono::{DateTime, TimeZone};
use icalendar::{rrule::Tz, Calendar, CalendarComponent, EventLike};

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

#[test]
fn parse_recurrence() {
    // tests recurrence handling for all-day events (dates) and datetimes

    // all-day: calendar string excludes 2nd and 3rd as EXDATE, but includes 30th and 31st as RDATE
    let expected_datetimes_a = vec![
        Tz::UTC.with_ymd_and_hms(2024, 12, 30, 0, 0, 0).unwrap(),
        Tz::UTC.with_ymd_and_hms(2024, 12, 31, 0, 0, 0).unwrap(),
        Tz::UTC.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
        Tz::UTC.with_ymd_and_hms(2025, 1, 4, 0, 0, 0).unwrap(),
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
    assert_eq!(datetimes, expected_datetimes_a);

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
