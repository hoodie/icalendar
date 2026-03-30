use icalendar::*;
use std::fs;

fn main() {
    let contents = fs::read_to_string("fixtures/event_five_attendees.ics").unwrap();

    let parsed_calendar: Calendar = contents.parse().unwrap();

    for event in parsed_calendar.events() {
        println!("Event: {}", event.get_summary().unwrap());
        let attendees = event.get_attendees();

        for attendee in attendees {
            println!("{:#?}", attendee);
        }
    }
}
