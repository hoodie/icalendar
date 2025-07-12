use chrono::*;
use icalendar::*;

fn main() {
    let mut calendar = Calendar::new();

    let now = Utc::now();
    let soon = Utc::now() + Duration::minutes(12);
    let tomorrow = Utc::now() + Duration::days(1);

    let todo_test_audio = Todo::new()
        .summary("TODO with audio alarm -15min")
        .sequence(1)
        .starts(now)
        .due(soon)
        .status(TodoStatus::NeedsAction)
        .percent_complete(98)
        .alarm(
            Alarm::audio(-Duration::minutes(10))
                .duration_and_repeat(chrono::Duration::minutes(1), 4),
        )
        .done();

    let event_test_display = Event::new()
        .summary("test event")
        .description("here I have something really important to do")
        .starts(Utc::now() + Duration::minutes(5))
        .class(Class::Confidential)
        .ends(Utc::now() + Duration::hours(1))
        .alarm(
            Alarm::display(
                "you should test your implementation",
                Utc::now() + Duration::minutes(1),
            )
            .duration_and_repeat(chrono::Duration::minutes(1), 4),
        )
        .done();

    let todo_test_display = Todo::new()
        .summary("TODO with display alarm now + 1 min")
        .sequence(3)
        .starts(now)
        .due(soon)
        .status(TodoStatus::NeedsAction)
        .alarm(
            Alarm::display(
                "you should test your implementation",
                (-Duration::minutes(10), Related::End),
            )
            .duration_and_repeat(chrono::Duration::minutes(1), 4),
        )
        .done();

    let todo_taxes = Todo::new()
        .summary("Submit Income Taxes")
        .sequence(4)
        .starts(now)
        .due(tomorrow)
        .status(TodoStatus::NeedsAction)
        .alarm(
            Alarm::audio(now + Duration::minutes(1))
                .duration_and_repeat(chrono::Duration::minutes(1), 4),
        )
        .done();

    calendar.push(event_test_display);
    calendar.push(todo_test_audio);
    calendar.push(todo_test_display);
    calendar.push(todo_taxes);

    println!("{calendar}");

    #[cfg(feature = "parser")]
    {
        use std::str::FromStr;

        let source = calendar.to_string();
        let reparse = Calendar::from_str(&source).unwrap();
        println!("{reparse:#?}");
    }
}
