#![cfg(feature = "parser")]
use std::str::FromStr;

use chrono::*;
use icalendar::*;
// use pretty_assertions::assert_eq;

fn get_summary(calendar: &Calendar) -> &str {
    calendar.components[0]
        .as_event()
        .unwrap()
        .get_summary()
        .unwrap()
}
fn get_description(calendar: &Calendar) -> &str {
    calendar.components[0]
        .as_event()
        .unwrap()
        .get_description()
        .unwrap()
}

fn init_test_event() -> (&'static str, &'static str, Calendar) {
    let summary = ";IMPORTANCE=very;test event";
    let description = "this, contains: many escapeworthy->\n<-;characters";

    let event = Event::new()
        .summary(summary)
        .description(description)
        .starts(Utc::now())
        .class(Class::Confidential)
        .ends(Utc::now() + Duration::days(1))
        .append_property(
            Property::new("TEST", "FOOBAR")
                .add_parameter("IMPORTANCE", "very")
                .add_parameter("COMPLEX", r#"this is code; I think"#)
                .add_parameter("keyval", "color:red")
                .done(),
        )
        .uid("my.own.id")
        .done();

    let todo = Todo::new().summary("Buy some: milk").done();

    let mut built_calendar = Calendar::new();
    built_calendar.push(event);
    built_calendar.push(todo);
    (summary, description, built_calendar)
}

#[test]
#[ignore = "equality difficult"]
fn serializes_correctly() {
    let (_, _, built_calendar) = init_test_event();
    println!("built calendar:\n{built_calendar:#?}"); // inner representation of what we built

    let serialized = built_calendar.to_string();
    println!("serialized: {}", &serialized); // print what we built

    let from_parsed = Calendar::from_str(&serialized).unwrap();
    println!("parsed again:\n{from_parsed:#?}"); // inner representation of what we built and then parsed

    assert_eq!(built_calendar, from_parsed)
}

#[test]
fn escape_late() {
    let (summary, description, built_calendar) = init_test_event();
    println!("built calendar:\n{built_calendar:#?}"); // inner representation of what we built

    let serialized = built_calendar.to_string();
    println!("serialized: {}", &serialized); // print what we built

    let from_parsed = Calendar::from_str(&serialized).unwrap();
    println!("parsed again:\n{from_parsed:#?}"); // inner representation of what we built and then parsed

    // these should not be escaped
    assert_eq!(get_summary(&built_calendar), summary);
    assert_eq!(get_description(&built_calendar), description);
}

#[test]
fn unescape_text() {
    let (summary, description, built_calendar) = init_test_event();
    println!("built calendar:\n{built_calendar:#?}"); // inner representation of what we built

    let serialized = built_calendar.to_string();
    println!("serialized:\n {}", &serialized); // print what we built

    let from_parsed = Calendar::from_str(&serialized).unwrap();
    println!("parsed again:\n{from_parsed:#?}"); // inner representation of what we built and then parsed

    assert_eq!(get_summary(&from_parsed), summary);
    assert_eq!(get_description(&from_parsed), description);
}

#[test]
fn reparse_equivalence() {
    let (_summary, _description, built_calendar) = init_test_event();
    println!("built calendar:\n{built_calendar:#?}"); // inner representation of what we built

    let serialized = built_calendar.to_string();
    println!("serialized: {}", &serialized); // print what we built

    let from_parsed = Calendar::from_str(&serialized).unwrap();
    println!("parsed again:\n{from_parsed:#?}"); // inner representation of what we built and then parsed

    assert_eq!(get_summary(&built_calendar), get_summary(&from_parsed),);
}
