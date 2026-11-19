use icalendar::*;
use jiff::{Timestamp, ToSpan as _, civil::date};
use pretty_assertions::assert_eq;

const EXPECTED_CAL_CONTENT: &str = "\
BEGIN:VCALENDAR\r
VERSION:2.0\r
PRODID:ICALENDAR-RS\r
CALSCALE:GREGORIAN\r
BEGIN:VEVENT\r
CLASS:CONFIDENTIAL\r
DESCRIPTION:Description\r
DTEND:20140709T091011Z\r
DTSTAMP:20190307T181159\r
DTSTART:20140708T071011Z\r
LOCATION:Somewhere\r
PRIORITY:9\r
STATUS:TENTATIVE\r
SUMMARY:summary\r
UID:euid\r
END:VEVENT\r
BEGIN:VTODO\r
COMPLETED:20140709T091011Z\r
DTSTAMP:20190307T181159\r
DUE:20140708T091011\r
PERCENT-COMPLETE:95\r
SUMMARY:A Todo\r
UID:todouid\r
END:VTODO\r
END:VCALENDAR\r
";

#[test]
fn test_calendar_to_string() {
    let mut calendar = Calendar::new();
    let cest_date = date(2014, 7, 8)
        .at(9, 10, 11, 0)
        .in_tz("Etc/GMT-2")
        .unwrap();
    let utc_date = date(2014, 7, 9)
        .at(9, 10, 11, 0)
        .in_tz("UTC")
        .unwrap()
        .timestamp();
    let event = Event::new()
        .status(EventStatus::Tentative)
        .starts(cest_date.timestamp())
        .ends(utc_date)
        .priority(11) // converted to 9
        .summary("summary")
        .description("Description")
        .location("Somewhere")
        .uid("euid")
        .class(Class::Confidential)
        .add_property("DTSTAMP", "20190307T181159")
        .done();
    calendar.push(event);
    let todo = Todo::new()
        .percent_complete(95)
        .due(cest_date.datetime())
        .completed(utc_date)
        .summary("A Todo")
        .uid("todouid")
        .add_property("DTSTAMP", "20190307T181159")
        .done();
    calendar.push(todo);
    assert_eq!(calendar.to_string(), EXPECTED_CAL_CONTENT);
}

#[test]
fn test_build_calendar() {
    let event = Event::new()
        .summary("test event")
        .description("here I have something really important to do")
        .starts(Timestamp::now())
        .class(Class::Confidential)
        .ends(Timestamp::now() + 24.hours())
        .append_property(
            Property::new("TEST", "FOOBAR")
                .add_parameter("IMPORTANCE", "very")
                .add_parameter("DUE", "tomorrow")
                .done(),
        )
        .done();

    let bday = Event::new()
        .all_day(date(2023, 3, 15))
        .summary("My Birthday")
        .description(
            r#"Hey, I'm gonna have a party
BYOB: Bring your own beer.
Hendrik"#,
        )
        .done();

    let todo = Todo::new().summary("Buy some milk").done();

    let mut calendar = Calendar::new();
    calendar.push(event);
    calendar.push(todo);
    calendar.push(bday);
}
