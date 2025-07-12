use chrono::*;
use icalendar::*;
use pretty_assertions::assert_eq;

/// Taken from https://datatracker.ietf.org/doc/html/rfc5545
/// some properties have been removed or reordered
const EXPECTED_CAL_CONTENT: &str = "\
BEGIN:VCALENDAR\r
VERSION:2.0\r
PRODID:ICALENDAR-RS\r
CALSCALE:GREGORIAN\r
BEGIN:VTODO\r
DTSTAMP:19980130T134500Z\r
DTSTART:19980130T134500Z\r
DUE:19980415T000000\r
SEQUENCE:2\r
STATUS:NEEDS-ACTION\r
SUMMARY:Submit Income Taxes\r
UID:uid4@example.com\r
BEGIN:VALARM\r
ACTION:AUDIO\r
DTSTAMP:19980130T134500Z\r
DURATION:PT3600S\r
REPEAT:4\r
TRIGGER;VALUE=DATE-TIME:19980403T120000Z\r
UID:OverwriteForConsistency\r
END:VALARM\r
END:VTODO\r
END:VCALENDAR\r
";

#[test]
fn test_alarm_to_string() {
    let mut calendar = Calendar::new();
    let todo = Todo::new()
        .uid("uid4@example.com")
        .add_property("DTSTAMP", "19980130T134500Z")
        .sequence(2)
        .starts(Utc.with_ymd_and_hms(1998, 1, 30, 13, 45, 0).unwrap())
        .due(
            NaiveDate::from_ymd_opt(1998, 4, 15)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .status(TodoStatus::NeedsAction)
        .summary("Submit Income Taxes")
        .append_component(
            Alarm::audio(Utc.with_ymd_and_hms(1998, 4, 3, 12, 0, 0).unwrap())
                .duration_and_repeat(chrono::Duration::hours(1), 4)
                .uid("OverwriteForConsistency")
                .add_property("DTSTAMP", "19980130T134500Z")
                .done(),
        )
        .done();
    calendar.push(todo);
    assert_eq!(calendar.to_string(), EXPECTED_CAL_CONTENT);

    #[cfg(feature = "parser")]
    {
        use std::str::FromStr;

        let reparse = Calendar::from_str(&calendar.to_string()).unwrap();
        println!("{reparse:?}");
    }
}
