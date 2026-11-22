use icalendar::*;

fn main() {
    let mut calendar = Calendar::new();

    let now = ZonedDateTime::now();
    let soon = &now + 12.minutes();
    let tomorrow = &now + 1.day();

    let todo_test_audio = Todo::new()
        .summary("TODO with audio alarm -15min")
        .sequence(1)
        .starts(&now)
        .due(&soon)
        .status(TodoStatus::NeedsAction)
        .percent_complete(98)
        .alarm(Alarm::audio(-10.minutes()).duration_and_repeat(1.minute(), 4))
        .done();

    let event_test_display = Event::new()
        .summary("test event")
        .description("here I have something really important to do")
        .starts(&now + 5.minutes())
        .class(Class::Confidential)
        .ends(&now + 1.hour())
        .alarm(
            Alarm::display(
                "you should test your implementation",
                (&now + 1.minute()).timestamp(),
            )
            .duration_and_repeat(1.minute(), 4),
        )
        .done();

    let todo_test_display = Todo::new()
        .summary("TODO with display alarm now + 1 min")
        .sequence(3)
        .starts(&now)
        .due(&soon)
        .status(TodoStatus::NeedsAction)
        .alarm(
            Alarm::display(
                "you should test your implementation",
                (-10.minutes(), Related::End),
            )
            .duration_and_repeat(1.minute(), 4),
        )
        .done();

    let todo_taxes = Todo::new()
        .summary("Submit Income Taxes")
        .sequence(4)
        .starts(&now)
        .due(&tomorrow)
        .status(TodoStatus::NeedsAction)
        .alarm(Alarm::audio((&now + 1.minute()).timestamp()).duration_and_repeat(1.minute(), 4))
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
