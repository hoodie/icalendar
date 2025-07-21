use chrono::Utc;
use icalendar::*;
use pretty_assertions::assert_eq;

#[test]
fn uid_preserved_after_uncompleted() {
    // Create a Todo with a specific UID
    let uid = "test-todo-unique-identifier-123";

    let mut todo = Todo::new()
        .summary("Test Todo")
        .uid(uid)
        .completed(Utc::now())
        .percent_complete(100)
        .status(TodoStatus::Completed)
        .done();

    // Verify the initial UID
    assert_eq!(todo.get_uid(), Some(uid));

    // Mark the todo as uncompleted
    todo.mark_uncompleted();

    // Verify the UID is preserved after marking as uncompleted
    assert_eq!(todo.get_uid(), Some(uid));

    // Double check that other completion properties were removed
    assert_eq!(todo.get_completed(), None);
    assert_eq!(todo.get_percent_complete(), None);
    assert_eq!(todo.get_status(), None);
}

#[test]
fn uid_preserved_in_calendar() {
    // Create a calendar with a Todo
    let uid = "calendar-todo-uid-456";

    let mut calendar = Calendar::new();
    let todo = Todo::new()
        .summary("Calendar Test Todo")
        .uid(uid)
        .completed(Utc::now())
        .percent_complete(100)
        .status(TodoStatus::Completed)
        .done();

    // Add the todo to the calendar
    calendar.push(todo);

    // Extract and modify the todo
    if let Some(component) = calendar.components.iter_mut().next() {
        if let CalendarComponent::Todo(todo) = component {
            // Verify the initial UID
            assert_eq!(todo.get_uid(), Some(uid));

            // Mark as uncompleted
            todo.mark_uncompleted();

            // Verify the UID is still the same
            assert_eq!(todo.get_uid(), Some(uid));
        } else {
            panic!("Expected a Todo component");
        }
    }

    // Verify the UID is preserved in the calendar's string representation
    let calendar_str = calendar.to_string();
    assert!(calendar_str.contains(&format!("UID:{uid}")));
}
