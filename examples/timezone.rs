#![cfg(feature = "chrono-tz")]
use chrono::*;
use chrono_tz::Europe::Berlin;
use icalendar::*;

fn main() {
    // lets make sure everybody arrives at the expected time
    let my_calendar = Calendar::new()
        .push(
            Event::new()
                .starts(CalendarDateTime::from_ymd_hm_tzid(2023, 3, 15, 18, 45, Berlin).unwrap())
                .description("I'm gonna have a party\nBYOB: Bring your own beer.\nHendrik")
                .done(),
        )
        .push(
            Event::new()
                .starts(
                    CalendarDateTime::from_ymd_hm_tzid(2023, 3, 15, 18, 45, Berlin)
                        .and_then(|cdt| cdt.try_into_utc())
                        .unwrap(),
                )
                .description("I'm gonna have a party\nBYOB: Bring your own beer.\nHendrik")
                .done(),
        )
        .push(
            Event::new()
                .starts(CalendarDateTime::from_date_time(
                    chrono_tz::Europe::Berlin
                        .with_ymd_and_hms(2023, 3, 15, 18, 45, 0)
                        .single()
                        .unwrap(),
                ))
                .summary("My Birthday")
                .description("I'm gonna have a party\nBYOB: Bring your own beer.\nHendrik")
                .done(),
        )
        .done();

    println!("{my_calendar}");
}
