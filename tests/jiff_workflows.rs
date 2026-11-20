use icalendar::{Calendar, CalendarDateTime, DatePerhapsTime, Event, EventLike, Todo};
use jiff::{
    civil::date,
    tz::{Disambiguation, TimeZone},
};

#[test]
fn date_values_resolve_in_supplied_zone() {
    let zone = TimeZone::get("Europe/Berlin").unwrap();
    let value = DatePerhapsTime::from(date(2026, 12, 31));
    assert_eq!(
        value.to_zoned_datetime(&zone).unwrap().date(),
        date(2026, 12, 31)
    );
    let utc = DatePerhapsTime::from(
        date(2026, 12, 31)
            .at(12, 0, 0, 0)
            .in_tz("UTC")
            .unwrap()
            .timestamp(),
    );
    assert_eq!(
        utc.to_zoned_datetime(&zone)
            .unwrap()
            .time_zone()
            .iana_name(),
        Some("UTC")
    );
}

#[test]
fn calendar_views_resolve_floating_values_and_reject_gaps() {
    let local = date(2026, 3, 8).at(2, 30, 0, 0);
    let value = DatePerhapsTime::from(local);
    let zone = TimeZone::get("America/New_York").unwrap();
    assert!(value.to_zoned_datetime(&zone).is_some());
    assert!(
        value
            .to_zoned_datetime_with(&zone, Disambiguation::Reject)
            .is_none()
    );
    assert!(
        CalendarDateTime::Floating(local)
            .to_zoned_datetime()
            .is_none()
    );

    let event = Event::new()
        .starts(local)
        .ends(date(2026, 3, 8).at(3, 30, 0, 0))
        .done();
    let todo = Todo::new()
        .starts(local)
        .due(date(2026, 3, 8).at(4, 30, 0, 0))
        .done();
    let mut calendar = Calendar::from([
        icalendar::CalendarComponent::Event(event),
        icalendar::CalendarComponent::Todo(todo),
    ]);
    assert!(calendar.calendar_events().next().unwrap().start().is_none());
    assert!(calendar.calendar_todos().next().unwrap().due().is_none());
    calendar.timezone("America/New_York");
    let event = calendar.calendar_events().next().unwrap();
    assert!(event.start().is_some());
    assert!(event.start_with(Disambiguation::Reject).is_none());
    assert_eq!(event.end().unwrap().time().hour(), 3);
    let todo = calendar.calendar_todos().next().unwrap();
    assert!(todo.start().is_some());
    assert_eq!(todo.due().unwrap().time().hour(), 4);
}

#[test]
fn unknown_tzids_are_preserved_without_resolution() {
    let event = Event::new()
        .starts(CalendarDateTime::WithTimezone {
            date_time: date(2026, 1, 1).at(9, 0, 0, 0),
            tzid: "Custom/Zone".into(),
        })
        .done();
    let calendar = Calendar::from(event);
    assert_eq!(calendar.calendar_events().next().unwrap().start(), None);
    assert!(calendar.to_string().contains("TZID=Custom/Zone"));
}
