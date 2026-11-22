use icalendar::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = ZonedDateTime::now(); // using a local now so the date arithmetic is done in the local timezone
    let my_calendar = Calendar::new()
        .name("example calendar")
        .timezone("Europe/Berlin")
        .push(
            Event::new()
                .summary("test event")
                .description("here I have something really important to do")
                .starts(now.datetime()) // we can use a floading time here since we have a global TZ for the whole calendar
                .class(Class::Confidential)
                .ends((now + 1.day()).datetime())
                .append_property(
                    Property::new("TEST", "FOOBAR")
                        .add_parameter("IMPORTANCE", "very")
                        .add_parameter("DUE", "tomorrow"),
                ),
        )
        .push(
            Todo::new()
                .summary("groceries")
                .description("Buy some milk"),
        )
        .push(
            Event::new()
                .all_day(date(2016, 3, 15))
                .summary("My Birthday")
                .description("Hey, I'm gonna have a party\nBYOB: Bring your own beer.\nHendrik"),
        )
        .push(
            Event::new()
                // X-WR-TIMEZONE is a client hint; this time stays floating
                .starts(date(2023, 3, 15).at(18, 45, 0, 0))
                .summary("Birthday Party")
                .description("I'm gonna have a party\nBYOB: Bring your own beer.\nHendrik"),
        )
        .done();

    println!("{my_calendar}");
    Ok(())
}
