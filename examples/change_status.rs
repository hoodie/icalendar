use std::io;

use chrono::{Duration, Utc};
use icalendar::*;

fn buy_milk() -> Todo {
    Todo::new()
        .summary("Buy milk")
        .description("Get 2% milk from the store")
        .completed(Utc::now())
        .percent_complete(100)
        .status(TodoStatus::Completed)
        .done()
}

fn team_meeting() -> Event {
    Event::new()
        .summary("Team Meeting")
        .description("Weekly team status update")
        .starts(Utc::now())
        .ends(Utc::now() + Duration::hours(1))
        .status(EventStatus::Cancelled)
        .done()
}

fn summarize_todo(todo: &Todo, title: &str) {
    println!("\n=== TODO {title} ===");
    println!("Summary: {}", todo.get_summary().unwrap());
    println!("Status: {:?}", todo.get_status());
    println!("Percent Complete: {:?}", todo.get_percent_complete());
    println!("Completed Date: {:?}", todo.get_completed());
}

fn summarize_event(event: &Event, title: &str) {
    println!("\n=== EVENT {title} ===");
    println!("Summary: {}", event.get_summary().unwrap());
    println!("Status: {:?}", event.get_status());
}

fn main() -> io::Result<()> {
    let mut calendar = Calendar::new()
        .name("Property Removal Example")
        .push(buy_milk())
        .push(team_meeting())
        .done();

    // Print the initial calendar
    println!("=== INITIAL CALENDAR ===");
    println!("{calendar}");

    // Now let's modify the components by removing properties

    // Get the todo and mark it as uncompleted
    if let Some(todo) = calendar.todos_mut().next() {
        summarize_todo(todo, "BEFORE CHANGE");

        todo.mark_uncompleted();

        summarize_todo(todo, "AFTER CHANGE");
    }

    // Get the event and remove its cancelled status
    if let Some(event) = calendar.events_mut().next() {
        summarize_event(event, "BEFORE CHANGE");

        // Remove the cancelled status
        event.remove_status();

        summarize_event(event, "AFTER CHANGE");
    }

    // Print the final calendar with properties removed
    println!("\n=== FINAL CALENDAR ===");
    println!("{calendar}");

    Ok(())
}
