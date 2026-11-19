use icalendar::*;
use jiff::{Timestamp, ToSpan as _, ZonedDateTime};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = ZonedDateTime::now();
    let groceries = Todo::new()
        .summary("buy groceries")
        .description("* soy-milk\n* oak-meal\n* vegan chocolate\n* kale\n* bacon\nabcdefghijklmnopqrstuvwxyz")
        .starts(&now)
        .priority(0)
        .percent_complete(28)
        .status(TodoStatus::InProcess)
        .completed(Timestamp::now())
        .due(now + 1.day())
        .done();

    let calendar = dbg!(Calendar::from([groceries]));
    println!("{calendar}");

    #[cfg(feature = "parser")]
    {
        let parsed_calendar = dbg!(calendar.to_string().parse::<Calendar>()?);
        parsed_calendar.print()?;
    }
    Ok(())
}
