use icalendar::{Calendar, CalendarDateTime, DatePerhapsTime, Event, Todo};
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

mod test_event_end {
    use icalendar::{Component, Event};
    use jiff::civil::date;

    #[test]
    fn all_day_ends_at_the_exclusive_next_date() {
        let event = Event::new().all_day(date(2026, 12, 31)).done();

        assert!(event.to_string().contains("DTEND;VALUE=DATE:20270101\r\n"));
    }
}

mod test_event_duration {
    use icalendar::{Component, Event, Todo};
    use jiff::{ToSpan as _, civil::date};

    #[test]
    fn duration_replaces_an_explicit_end() {
        let event = Event::new()
            .starts(date(2026, 3, 8).at(1, 30, 0, 0))
            .ends(date(2026, 3, 8).at(3, 30, 0, 0))
            .lasts(1.day())
            .done();

        assert_eq!(event.property_value("DURATION"), Some("P1D"));
        assert_eq!(event.get_end(), None);
    }

    #[test]
    fn all_day_duration_replaces_the_exclusive_end() {
        let event = Event::new()
            .all_day(date(2026, 12, 31))
            .lasts(3.days())
            .done();

        assert_eq!(event.property_value("DURATION"), Some("P3D"));
        assert_eq!(event.get_end(), None);
    }

    #[test]
    fn explicit_end_properties_replace_duration() {
        let event = Event::new()
            .starts(date(2026, 3, 8).at(1, 30, 0, 0))
            .lasts(1.day())
            .ends(date(2026, 3, 8).at(3, 30, 0, 0))
            .done();
        let todo = Todo::new()
            .starts(date(2026, 3, 8).at(1, 30, 0, 0))
            .lasts(1.day())
            .due(date(2026, 3, 8).at(3, 30, 0, 0))
            .done();

        assert_eq!(event.property_value("DURATION"), None);
        assert!(event.get_end().is_some());
        assert_eq!(todo.property_value("DURATION"), None);
        assert!(todo.get_due().is_some());
    }

    #[test]
    fn nominal_day_duration_keeps_wall_clock_time_across_dst() {
        let event = Event::new()
            .starts(date(2026, 3, 7).at(12, 0, 0, 0))
            .lasts(1.day())
            .done();
        let mut calendar = icalendar::Calendar::from(event);
        calendar.timezone("America/New_York");

        let resolved_end = calendar.calendar_events().next().unwrap().end().unwrap();

        assert_eq!(resolved_end.date(), date(2026, 3, 8));
        assert_eq!(resolved_end.time().hour(), 12);
    }

    #[test]
    fn exact_day_duration_keeps_elapsed_time_across_dst() {
        let event = Event::new()
            .starts(date(2026, 3, 7).at(12, 0, 0, 0))
            .lasts(24.hours())
            .done();
        let mut calendar = icalendar::Calendar::from(event);
        calendar.timezone("America/New_York");

        let resolved_end = calendar.calendar_events().next().unwrap().end().unwrap();

        assert_eq!(resolved_end.date(), date(2026, 3, 8));
        assert_eq!(resolved_end.time().hour(), 13);
    }

    #[test]
    fn todo_duration_replaces_due_and_resolves_with_timezone() {
        let todo = Todo::new()
            .starts(date(2026, 3, 7).at(12, 0, 0, 0))
            .due(date(2026, 3, 9).at(12, 0, 0, 0))
            .lasts(1.day())
            .done();
        assert_eq!(todo.get_due(), None);
        assert_eq!(todo.property_value("DURATION"), Some("P1D"));

        let mut calendar = icalendar::Calendar::from(todo);
        calendar.timezone("America/New_York");
        let resolved_due = calendar.calendar_todos().next().unwrap().due().unwrap();

        assert_eq!(resolved_due.date(), date(2026, 3, 8));
        assert_eq!(resolved_due.time().hour(), 12);
    }
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
