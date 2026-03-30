use icalendar::*;
use std::fs;

fn main() {
    let contents = fs::read_to_string("fixtures/icalendar-rb/event_five_attendees.ics").unwrap();

    let parsed_calendar: Calendar = contents.parse().unwrap();

    for component in &parsed_calendar.components {
        if let CalendarComponent::Event(event) = component {
            println!("Event: {}", event.get_summary().unwrap());
            let attendees = event.get_attendees();

            for attendee in attendees {
                println!("{:#?}", attendee);
            }
        }
    }
}
