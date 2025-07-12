use chrono::*;
use icalendar::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let groceries = Todo::new()
        .summary("buy groceries")
        .description("* soy-milk\n* oak-meal\n* vegan chocolate\n* kale\n* bacon\nabcdefghijklmnopqrstuvwxyz")
        .starts(Local::now().naive_local())
        .ends(Local::now().naive_local() + Duration::hours(24))
        .priority(12)
        .percent_complete(28)
        .status(TodoStatus::InProcess)
        .completed(Utc::now())
        .due(Local::now().with_timezone(&Utc))
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
