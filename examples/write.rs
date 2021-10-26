use chrono::*;
use icalendar::*;

fn main() {
    let event = Event::new()
        .summary("test event")
        .description("here I have something really important to do")
        .starts(Utc::now())
        .class(Class::Confidential)
        //.repeats_every(15.days())
        //.repeats(Every::second().wednesday())
        .ends(Utc::now() + Duration::days(1))
        //.all_day()
        .append_property(
            Property::new("TEST", "FOOBAR")
                .parameter("IMPORTANCE", "very")
                .parameter("DUE", "tomorrow"),
        )
        .uid("my.own.id");

    let bday = Event::new()
        .start_date(Utc.ymd(2016, 3, 15))
        .end_date(Utc.ymd(2016, 3, 15))
        .summary("My Birthday")
        .description(
            r#"Hey, I'm gonna have a party
BYOB: Bring your own beer.
Hendrik"#,
        );

    let bday2 = Event::new().all_day(Utc.ymd(2016, 3, 15));

    let todo = Todo::new().summary("Buy some milk");

    let mut calendar = Calendar::new();
    calendar.push(event);
    calendar.push(todo);
    calendar.push(bday);
    calendar.push(bday2);

    calendar.print().unwrap();
}
