use chrono_tz::Europe::Berlin;
use icalendar::*;

fn main() {
    let all_hands = Event::new()
        .uid("all_hands_2026@example.com")
        .summary("All-Hands Meeting")
        .description("Selected all-hands sessions, 09:00–10:00 Berlin time.")
        .starts(CalendarDateTime::from_ymd_hm_tzid(2026, 1, 5, 9, 0, Berlin).unwrap())
        .ends(CalendarDateTime::from_ymd_hm_tzid(2026, 1, 5, 10, 0, Berlin).unwrap())
        .rdate(CalendarDateTime::from_ymd_hm_tzid(2026, 1, 6, 9, 0, Berlin).unwrap())
        .rdate(CalendarDateTime::from_ymd_hm_tzid(2026, 1, 7, 9, 0, Berlin).unwrap())
        .rdate(CalendarDateTime::from_ymd_hm_tzid(2026, 1, 8, 9, 0, Berlin).unwrap())
        .rdate(CalendarDateTime::from_ymd_hm_tzid(2026, 12, 7, 9, 0, Berlin).unwrap())
        .exdate(CalendarDateTime::from_ymd_hm_tzid(2026, 12, 7, 9, 0, Berlin).unwrap())
        .done();

    let mut calendar = Calendar::new();
    calendar.push(all_hands);

    println!("{calendar}");
}
