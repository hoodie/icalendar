use chrono::*;
use chrono_tz::Europe::Berlin;
use icalendar::*;

fn print_recurrences(event: &Event) {
    eprintln!(
        "All recurrences of {}: {:#?}",
        event.get_summary().unwrap_or_default(),
        event.get_recurrence().unwrap().all(1000).dates
    );
}

fn main() {
    use Month::*;
    // every wednesday until end of cycling season
    let road_cycling = Event::new()
        .starts(CalendarDateTime::from_ymd_hm_tzid(2026, 3, 18, 11, 55, Berlin).unwrap())
        .ends(CalendarDateTime::from_ymd_hm_tzid(2026, 3, 18, 13, 15, Berlin).unwrap())
        .summary("🚴‍♂️ Group ride with the colleagues")
        .description("every wednesday at noon, march through november")
        .recurrence(
            RRule::default()
                .freq(Frequency::Weekly)
                .by_weekday(vec![NWeekday::Every(Weekday::Wed)])
                .by_month(&[March, April, May, June, September, October, November]),
        )
        .unwrap()
        .done();

    // every last wednesday of the month
    let rust_meetup = Event::new()
        .starts(CalendarDateTime::from_ymd_hm_tzid(2026, 3, 25, 18, 00, Berlin).unwrap())
        .ends(CalendarDateTime::from_ymd_hm_tzid(2026, 3, 25, 22, 00, Berlin).unwrap())
        .summary("Dresden Rust Meetup")
        .description("Come and bring a topic")
        .recurrence(
            RRule::default()
                .freq(Frequency::Monthly)
                .interval(2)
                .by_weekday(vec![NWeekday::Nth(-1, Weekday::Wed)])
                .until(Tz::UTC.with_ymd_and_hms(2026, 12, 31, 23, 59, 59).unwrap()),
        )
        .unwrap()
        .done();

    // every two weeks on Friday
    let sprint_review = Event::new()
        .all_day(NaiveDate::from_ymd_opt(2026, 1, 9).unwrap())
        .summary("Sprint Review")
        .recurrence(
            RRule::default()
                .freq(Frequency::Weekly)
                .interval(2)
                .by_weekday(vec![NWeekday::Every(Weekday::Fri)])
                .count(12),
        )
        .expect("valid rule")
        .done();
    // every third thursday of the month
    let ttt = Event::new()
        .starts(CalendarDateTime::from_ymd_hm_tzid(2026, 3, 19, 17, 00, Berlin).unwrap())
        .ends(CalendarDateTime::from_ymd_hm_tzid(2026, 3, 19, 23, 00, Berlin).unwrap())
        .summary("Third Thirsty Thursday")
        .description("Casual get together for drinks in the office")
        .recurrence(
            RRule::default()
                .count(4)
                .freq(Frequency::Monthly)
                .by_weekday(vec![NWeekday::Nth(3, Weekday::Thu)]),
        )
        .unwrap()
        .done();

    print_recurrences(&road_cycling);
    print_recurrences(&rust_meetup);
    print_recurrences(&sprint_review);
    print_recurrences(&ttt);

    println!(
        "{}",
        Calendar::from([road_cycling, rust_meetup, sprint_review, ttt])
    );
}
