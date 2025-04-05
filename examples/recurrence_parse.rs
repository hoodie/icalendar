#![cfg(feature = "recurrence")]
use chrono::*;
use icalendar::*;
use std::fs;

// maximum number of events accepted
const RECURRENCE_LIMIT: u16 = u16::MAX;

fn main() {
    let contents = fs::read_to_string("fixtures/icalendar-rb/recurrence.ics").unwrap();

    let parsed_calendar: Calendar = contents.parse().unwrap();

    for component in &parsed_calendar.components {
        if let CalendarComponent::Event(event) = component {
            println!("Event: {}", event.get_summary().unwrap());
            if let Some(rrules) = event.get_recurrence() {
                let datetimes: Vec<DateTime<rrule::Tz>> = rrules
                    .all(RECURRENCE_LIMIT)
                    .dates;

                println!("Repeating on the following dates (showing first 10): ");
                println!("{:#?}", &datetimes[..10]);
            }
        }
    }
}
