#![cfg(feature = "recurrence")]
use chrono::NaiveDate;
use icalendar::*;

fn main() {
    // Build a calendar with a default timezone (no chrono-tz feature needed).
    let event = Event::new()
        .all_day(NaiveDate::from_ymd_opt(2026, 4, 1).unwrap())
        .summary("All-day standup")
        .recurrence(RRule::default().count(3).freq(Frequency::Daily))
        .expect("DTSTART must be set and the rule must be valid")
        .done();

    let mut calendar = Calendar::new();
    calendar.timezone("Europe/Berlin");
    calendar.push(event);

    // calendar_events() pairs each event with the calendar-level timezone so
    // that DATE-only DTSTART values are anchored to midnight in that timezone.
    for cal_event in calendar.calendar_events() {
        println!(
            "Event: {}",
            cal_event.get_summary().unwrap_or("(no summary)")
        );
        println!(
            "Calendar timezone: {}",
            cal_event.calendar_tz().unwrap_or("(none)")
        );

        let occurrences = cal_event
            .get_recurrence()
            .expect("event should have a recurrence rule")
            .all(10)
            .dates;

        println!("Occurrences ({} total):", occurrences.len());
        for dt in &occurrences {
            println!(
                "  local: {}  UTC: {}",
                dt.naive_local().date(),
                dt.naive_utc(),
            );
        }
    }
}
