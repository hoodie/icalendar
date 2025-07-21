use chrono::*;
#[cfg(feature = "chrono-tz")]
use chrono_tz::Europe::Berlin;
use icalendar::*;

fn main() {
    // lets create a calendar
    let my_calendar = Calendar::new()
        .name("example calendar")
        .push(
            // add an event
            Event::new()
                .summary("test event")
                .description("here I have something really important to do")
                .starts(Utc::now())
                .class(Class::Confidential)
                .ends(Utc::now() + Duration::days(1))
                .append_property(
                    Property::new("TEST", "FOOBAR")
                        .add_parameter("IMPORTANCE", "very")
                        .add_parameter("DUE", "tomorrow"),
                ),
        )
        .push(
            // add a todo
            Todo::new()
                .summary("groceries")
                .description("Buy some milk"),
        )
        .push(
            // add an all-day event
            Event::new()
                .all_day(NaiveDate::from_ymd_opt(2016, 3, 15).unwrap())
                .summary("My Birthday")
                .description("Hey, I'm gonna have a party\nBYOB: Bring your own beer.\nHendrik"),
        )
        .push(
            // local event with timezone
            Event::new()
                .starts({
                    #[cfg(feature = "chrono-tz")]
                    {
                        CalendarDateTime::from_ymd_hm_tzid(2023, 3, 15, 18, 45, Berlin).unwrap()
                    }
                    #[cfg(not(feature = "chrono-tz"))]
                    {
                        // probably not when you think
                        NaiveDate::from_ymd_opt(2016, 3, 15)
                            .unwrap()
                            .and_hms_opt(18, 45, 0)
                            .unwrap()
                    }
                })
                .summary("Birthday Party")
                .description("I'm gonna have a party\nBYOB: Bring your own beer.\nHendrik"),
        )
        .done();

    println!("{my_calendar}");
}
