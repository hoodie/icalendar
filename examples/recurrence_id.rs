use chrono_tz::Europe::Berlin;
use icalendar::*;

fn main() {
    let standup = Event::new()
        .uid("weekly_standup_2026@example.com")
        .summary("Weekly Team Standup")
        .description("Every Wednesday 10:00–11:00, Berlin time.")
        .starts(CalendarDateTime::from_ymd_hm_tzid(2026, 1, 7, 10, 0, Berlin).unwrap())
        .ends(CalendarDateTime::from_ymd_hm_tzid(2026, 1, 7, 11, 0, Berlin).unwrap())
        .recurrence(RRule::new(Frequency::Weekly).by_weekday(vec![NWeekday::Every(Weekday::Wed)]))
        .unwrap()
        .done();

    let standup_rescheduled = Event::new()
        .uid("weekly_standup_2026@example.com") // ← same UID as the master
        .summary("Weekly Team Standup (rescheduled — public holiday)")
        .description("Moved from Wed Apr 1 to Thu Apr 2 because Apr 1 is a public holiday.")
        // The original slot this component replaces — NOT the new time:
        .recurrence_id(CalendarDateTime::from_ymd_hm_tzid(2026, 4, 1, 10, 0, Berlin).unwrap())
        // The new time:
        .starts(CalendarDateTime::from_ymd_hm_tzid(2026, 4, 2, 10, 0, Berlin).unwrap())
        .ends(CalendarDateTime::from_ymd_hm_tzid(2026, 4, 2, 11, 0, Berlin).unwrap())
        .done();

    let mut calendar = Calendar::new();
    calendar.push(standup);
    calendar.push(standup_rescheduled);

    println!("{calendar}");
}
