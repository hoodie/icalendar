#![cfg(feature = "recurrence")]
use chrono::*;
use icalendar::*;

// maximum number of events accepted
const RECURRENCE_LIMIT: u16 = u16::MAX;

fn main() {
    let dt_start = rrule::Tz::Europe__London
        .with_ymd_and_hms(2025, 3, 17, 0, 0, 0)
        .unwrap();

    let my_event = Event::new()
        .all_day(NaiveDate::from_ymd_opt(2025, 3, 17).unwrap())
        .summary("weekly event")
        .description("this event happens every Monday for four weeks")
        .recurrence(
            rrule::RRule::default()
                .count(4)
                .freq(rrule::Frequency::Weekly)
                .by_weekday(vec![rrule::NWeekday::Every(rrule::Weekday::Mon)])
                .build(dt_start)
                .unwrap(),
        )
        .done();

    let all_occurences = my_event
        .get_recurrence()
        .unwrap()
        .all(RECURRENCE_LIMIT)
        .dates;

    println!("{:#?}", all_occurences);
}
