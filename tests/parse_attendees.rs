#![cfg(feature = "parser")]
use std::{convert::TryFrom, env, fs, path::PathBuf};

use icalendar::Attendee;
use icalendar::parser::{read_calendar, unfold};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("fixtures")
        .join(name)
}

fn read_attendees_from_fixture(name: &str) -> Vec<Attendee> {
    let path = fixture_path(name);
    let fixture = fs::read_to_string(path).unwrap();
    let binding = unfold(&fixture);
    let cal = read_calendar(&binding).unwrap();
    let comp = cal.components.first().expect("expected one component");

    comp.properties
        .iter()
        .filter(|p| p.name.as_str().eq_ignore_ascii_case("ATTENDEE"))
        .filter_map(|p| {
            // parser::Property -> crate::Property via From
            let crate_prop: icalendar::Property = p.clone().into();
            Attendee::try_from(&crate_prop).ok()
        })
        .collect()
}

#[test]
fn parse_minimal_attendees() {
    let attendees = read_attendees_from_fixture("event_minimal_attendees.ics");
    assert_eq!(attendees.len(), 1);
    assert_eq!(
        attendees[0].cal_address,
        "mailto:singleattendee@example.com"
    );
}

#[test]
fn parse_two_attendees() {
    let attendees = read_attendees_from_fixture("event_two_attendees.ics");
    // fixture contains three ATTENDEE lines
    assert_eq!(attendees.len(), 3);
    // ensure one attendee has CN="John Doe"
    assert!(
        attendees.iter().any(|a| a.cn.as_deref() == Some("John Doe")
            && a.cal_address == "mailto:johndoe@example.com")
    );
}

#[test]
fn parse_five_attendees() {
    let attendees = read_attendees_from_fixture("event_five_attendees.ics");
    assert_eq!(attendees.len(), 5);

    // check the complex attendee has expected fields
    let full = attendees
        .iter()
        .find(|a| a.cal_address == "mailto:fullattendee@example.com")
        .expect("full attendee not found");

    assert_eq!(full.cn.as_deref(), Some("Full Attendee"));
    assert!(full.member.iter().any(|m| m == "mailto:member@example.com"));
    assert_eq!(full.rsvp, Some(true));
}

#[test]
fn parse_mixed_complexity_attendees() {
    let attendees = read_attendees_from_fixture("event_attendees_mixed_complexity.ics");
    assert_eq!(attendees.len(), 3);
}
