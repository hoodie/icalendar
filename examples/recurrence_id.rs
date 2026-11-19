use icalendar::*;
use jiff::civil::date;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let standup = Event::new()
        .uid("weekly_standup_2026@example.com")
        .summary("Weekly Team Standup")
        .description("Every Wednesday 10:00–11:00, Berlin time.")
        .starts(date(2026, 1, 7).at(10, 0, 0, 0).in_tz("Europe/Berlin")?)
        .ends(date(2026, 1, 7).at(11, 0, 0, 0).in_tz("Europe/Berlin")?)
        .add_property("RRULE", "FREQ=WEEKLY;BYDAY=WE")
        .done();

    let standup_rescheduled = Event::new()
        .uid("weekly_standup_2026@example.com") // ← same UID as the master
        .summary("Weekly Team Standup (rescheduled — public holiday)")
        .description("Moved from Wed Apr 1 to Thu Apr 2 because Apr 1 is a public holiday.")
        // The original slot this component replaces — NOT the new time:
        .recurrence_id(date(2026, 4, 1).at(10, 0, 0, 0).in_tz("Europe/Berlin")?)
        // The new time:
        .starts(date(2026, 4, 2).at(10, 0, 0, 0).in_tz("Europe/Berlin")?)
        .ends(date(2026, 4, 2).at(11, 0, 0, 0).in_tz("Europe/Berlin")?)
        .done();

    let mut calendar = Calendar::new();
    calendar.push(standup);
    calendar.push(standup_rescheduled);

    println!("{calendar}");
    Ok(())
}
