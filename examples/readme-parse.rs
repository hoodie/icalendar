fn main() {
    use std::fs::read_to_string;

    use icalendar::{Calendar, Component};

    let contents = read_to_string("fixtures/icalendar-rb/event.ics").unwrap();

    let parsed_calendar: Calendar = contents.parse().unwrap();

    for event in parsed_calendar.events() {
        println!("Event: {}", event.get_summary().unwrap())
    }
}
