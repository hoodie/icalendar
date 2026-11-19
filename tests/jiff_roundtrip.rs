use icalendar::{Alarm, Calendar, Component, Event, EventLike, Todo};
use jiff::{Timestamp, ToSpan as _, civil::date};

#[test]
fn jiff_wire_forms_round_trip() {
    let stamp: Timestamp = "2026-03-14T12:00:00Z".parse().unwrap();
    let event = Event::new()
        .uid("all-day@example.com")
        .timestamp(stamp)
        .starts(date(2026, 3, 15))
        .ends(date(2026, 3, 16))
        .alarm(
            Alarm::display("Reminder", -10.minutes())
                .duration_and_repeat(1.minute(), 4)
                .uid("alarm@example.com")
                .timestamp(stamp)
                .done(),
        )
        .done();
    let todo = Todo::new()
        .uid("floating@example.com")
        .timestamp(stamp)
        .starts(date(2026, 3, 15).at(18, 45, 0, 0))
        .due(date(2026, 3, 16).at(18, 45, 0, 0))
        .done();
    let calendar = Calendar::new()
        .ttl(1.day())
        .unwrap()
        .push(event)
        .push(todo)
        .done();
    let wire = calendar.to_string();
    assert!(wire.contains("REFRESH-INTERVAL;VALUE=DURATION:P1D\r\n"));
    assert!(wire.contains("DTSTART;VALUE=DATE:20260315\r\n"));
    assert!(wire.contains("DTEND;VALUE=DATE:20260316\r\n"));
    assert!(wire.contains("DTSTART:20260315T184500\r\n"));
    assert!(wire.contains("DUE:20260316T184500\r\n"));
    assert!(wire.contains("TRIGGER:-PT10M\r\n"));
    assert!(wire.contains("DURATION:PT1M\r\n"));

    #[cfg(feature = "parser")]
    {
        let parsed: Calendar = wire.parse().unwrap();
        assert_eq!(
            parsed.events().next().unwrap().get_start(),
            calendar.events().next().unwrap().get_start()
        );
        assert_eq!(
            parsed.todos().next().unwrap().get_due(),
            calendar.todos().next().unwrap().get_due()
        );
    }
}
