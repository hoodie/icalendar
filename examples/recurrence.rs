#![cfg(feature = "recurrence")]
use chrono::NaiveDate;
use icalendar::*;

// maximum number of events accepted
const RECURRENCE_LIMIT: u16 = u16::MAX;

fn main() {
    // Note: .all_day() sets DTSTART; .recurrence() derives the start time from it automatically.
    let my_event = Event::new()
        .all_day(NaiveDate::from_ymd_opt(2025, 3, 17).unwrap())
        .summary("weekly event")
        .description("this event happens every Monday for four weeks")
        .recurrence(
            RRule::default()
                .count(4)
                .freq(Frequency::Weekly)
                .by_weekday(vec![NWeekday::Every(Weekday::Mon)]),
        )
        .expect("DTSTART must be set and the rule must be valid")
        .done();

    let all_occurences = my_event
        .get_recurrence()
        .unwrap()
        .unwrap()
        .all(RECURRENCE_LIMIT)
        .dates;

    println!("{:#?}", all_occurences);
}
