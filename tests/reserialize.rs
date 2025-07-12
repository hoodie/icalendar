#![cfg(feature = "parser")]
use icalendar::parser::unfold;
use pretty_assertions::assert_eq;

const SAMPLE: &str = "\
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
PRIORITY:10\r
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
BEGIN:VALARM\r
TRIGGER;RELATED=END:-P2D\r
ACTION:EMAIL\r
ATTENDEE:\"mailto:john_doe@example.com\"\r
SUMMARY:*** REMINDER: SEND AGENDA FOR WEEKLY STAFF MEETING ***\r
DESCRIPTION:A draft agenda needs to be sent out to the attendees to the wee\r
 kly managers meeting (MGR-LIST). Attached is a pointer the document templa\r
 te for the agenda file.\r
ATTACH;FMTTYPE=application/msword:http://example.com/templates/agenda.doc\r
END:VALARM\r
END:VCALENDAR\r
";

#[test]
fn reserialization() {
    let unfolded = unfold(SAMPLE);
    // print_with_lines(&unfolded);

    let parsed = icalendar::parser::read_calendar(&unfolded).unwrap();
    let reserialized = parsed.to_string();
    println!("{reserialized}");
    assert_eq!(SAMPLE, reserialized);
}
