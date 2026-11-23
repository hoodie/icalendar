use icalendar::{Calendar, Component, Event, Todo};

#[test]
fn calendar_views_are_available_without_recurrence_expansion() {
    let mut calendar = Calendar::new();
    calendar.timezone("Europe/Berlin");
    calendar.push(Event::new().summary("Meeting").done());
    calendar.push(Todo::new().summary("Prepare").done());

    let event = calendar.calendar_events().next().unwrap();
    assert_eq!(event.calendar_tz(), Some("Europe/Berlin"));
    assert_eq!(event.get_summary(), Some("Meeting"));

    let todo = calendar.calendar_todos().next().unwrap();
    assert_eq!(todo.calendar_tz(), Some("Europe/Berlin"));
    assert_eq!(todo.get_summary(), Some("Prepare"));
}

#[cfg(feature = "parser")]
#[test]
fn raw_recurrence_properties_round_trip() {
    let source = concat!(
        "BEGIN:VCALENDAR\r\n",
        "VERSION:2.0\r\n",
        "PRODID:-//test//EN\r\n",
        "BEGIN:VEVENT\r\n",
        "UID:recurring@example.com\r\n",
        "DTSTAMP:20260101T000000Z\r\n",
        "DTSTART;VALUE=DATE:20260105\r\n",
        "DTEND;VALUE=DATE:20260106\r\n",
        "RRULE:FREQ=DAILY;COUNT=3\r\n",
        "RDATE;VALUE=DATE:20260110\r\n",
        "EXDATE;VALUE=DATE:20260106\r\n",
        "END:VEVENT\r\n",
        "BEGIN:VEVENT\r\n",
        "UID:recurring@example.com\r\n",
        "DTSTAMP:20260101T000000Z\r\n",
        "DTSTART;VALUE=DATE:20260107\r\n",
        "DTEND;VALUE=DATE:20260108\r\n",
        "RECURRENCE-ID;VALUE=DATE:20260106\r\n",
        "END:VEVENT\r\n",
        "END:VCALENDAR\r\n",
    );

    let calendar: Calendar = source.parse().unwrap();
    let serialized = calendar.to_string();
    let reparsed: Calendar = serialized.parse().unwrap();
    let mut events = reparsed.events();
    let master = events.next().unwrap();
    assert_eq!(master.property_value("RRULE"), Some("FREQ=DAILY;COUNT=3"));
    assert!(serialized.contains("RDATE;VALUE=DATE:20260110"));
    assert!(serialized.contains("EXDATE;VALUE=DATE:20260106"));
    assert_eq!(
        events
            .next()
            .unwrap()
            .get_recurrence_id()
            .unwrap()
            .date()
            .to_string(),
        "2026-01-06"
    );
}
