#![cfg(feature = "recurrence")]
use chrono::NaiveDate;
use icalendar::*;

fn main() {
    // Note: .all_day() sets DTSTART; .recurrence() derives the start time from it automatically.
    let saint_patricks_day = Event::new()
        .all_day(NaiveDate::from_ymd_opt(2026, 3, 17).unwrap())
        .summary("🍀 Saint Patrick's Day")
        .description("wear something green")
        .recurrence(RRule::default().count(4).freq(Frequency::Yearly))
        .expect("DTSTART must be set and the rule must be valid")
        .done();

    eprintln!(
        "All Saint Patrick's days{:#?}",
        saint_patricks_day
            .get_recurrence()
            .expect("event should have a recurrence rule")
            .all(u16::MAX)
            .dates
    );
    println!("{}", Calendar::from(saint_patricks_day));
}
