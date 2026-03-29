use chrono_tz::Europe::Berlin;
use icalendar::*;

/// TODO: MAKE THIS INTO A TEST
/// there does not necessarily have to be an rrule for rdates to produce recurrences!!!
fn main() {
    let mut all_hands = Event::new()
        .uid("all_hands_2026@example.com")
        .summary("All-Hands Meeting")
        .description("Monthly all-hands. First Monday of each month, 09:00–10:00 Berlin time.")
        .starts(CalendarDateTime::from_ymd_hm_tzid(2026, 1, 5, 9, 0, Berlin).unwrap())
        .ends(CalendarDateTime::from_ymd_hm_tzid(2026, 1, 5, 10, 0, Berlin).unwrap())
        .rdate(CalendarDateTime::from_ymd_hm_tzid(2026, 1, 6, 9, 0, Berlin).unwrap())
        .rdate(CalendarDateTime::from_ymd_hm_tzid(2026, 1, 7, 9, 0, Berlin).unwrap())
        .rdate(CalendarDateTime::from_ymd_hm_tzid(2026, 1, 8, 9, 0, Berlin).unwrap())
        // .recurrence(RRule::new(Frequency::Monthly).by_weekday(vec![NWeekday::Nth(1, Weekday::Mon)])).unwrap()
        .done();

    // cancel the December instance
    let december_instance = CalendarDateTime::from_ymd_hm_tzid(2026, 12, 7, 9, 0, Berlin).unwrap();
    all_hands.exdate(december_instance);

    // add an emergency session on a Wednesday
    // 2026-06-17 is a Wednesday, outside the first-Monday RRULE pattern.
    let emergency_session = CalendarDateTime::from_ymd_hm_tzid(2026, 6, 17, 9, 0, Berlin).unwrap();
    all_hands.rdate(emergency_session);

    let mut calendar = Calendar::new();
    calendar.push(all_hands);

    let recurrences = calendar
        .events()
        .next()
        .unwrap()
        .to_owned()
        .get_recurrence()
        .unwrap()
        .all(100)
        .dates;

    eprintln!("recurrences: {recurrences:#?}");

    // A conforming calendar client will:
    //   • Generate the first-Monday occurrences Jan–Nov from the RRULE.
    //   • Add the extra Wed Jun 17 session from RDATE.
    //   • Suppress the Dec 7 occurrence because of EXDATE.
    println!("{calendar}");
}
