use chrono::*;

use super::*;

/// VTODO  [(RFC 5545, Section 3.6.2 )](https://tools.ietf.org/html/rfc5545#section-3.6.2)
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Todo {
    pub(super) inner: InnerComponent,
}

impl Todo {
    /// Creates a new Todo.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Event with a UID.
    pub fn with_uid(uid: &str) -> Self {
        Self::new().uid(uid).done()
    }

    /// End of builder pattern.
    /// copies over everything
    pub fn done(&mut self) -> Self {
        Todo {
            inner: self.inner.done(),
        }
    }

    /// Set the [`PERCENT-COMPLETE`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.8) property
    ///
    /// Ranges between 0 - 100
    pub fn percent_complete(&mut self, percent: u8) -> &mut Self {
        self.add_property("PERCENT-COMPLETE", percent.to_string())
    }

    /// Gets the [`PERCENT-COMPLETE`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.8) property.
    ///
    /// Ranges between 0 - 100.
    pub fn get_percent_complete(&self) -> Option<u8> {
        self.property_value("PERCENT-COMPLETE")?.parse().ok()
    }

    /// Set the [`DUE`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.3) property
    ///
    /// See [`CalendarDateTime`] for info how are different [`chrono`] types converted automatically.
    pub fn due<T: Into<DatePerhapsTime>>(&mut self, dt: T) -> &mut Self {
        let calendar_dt: DatePerhapsTime = dt.into();
        self.append_property(calendar_dt.to_property("DUE"))
    }

    /// Gets the [`DUE`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.3) property
    pub fn get_due(&self) -> Option<DatePerhapsTime> {
        DatePerhapsTime::from_property(self.properties().get("DUE")?)
    }

    /// Set the [`COMPLETED`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.1) property
    ///
    /// Per [RFC 5545, Section 3.8.2.1](https://tools.ietf.org/html/rfc5545#section-3.8.2.1), this
    /// must be a date-time in UTC format.
    pub fn completed(&mut self, dt: DateTime<Utc>) -> &mut Self {
        self.add_property("COMPLETED", format_utc_date_time(dt))
    }

    /// Gets the [`COMPLETED`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.1) property
    ///
    /// Per [RFC 5545, Section 3.8.2.1](https://tools.ietf.org/html/rfc5545#section-3.8.2.1), this
    /// must be a date-time in UTC format.
    pub fn get_completed(&self) -> Option<DateTime<Utc>> {
        let completed = self.property_value("COMPLETED")?;
        parse_utc_date_time(completed)
    }

    /// Defines the overall status or confirmation
    pub fn status(&mut self, status: TodoStatus) -> &mut Self {
        self.append_property(status)
    }

    /// Gets the overall status.
    pub fn get_status(&self) -> Option<TodoStatus> {
        TodoStatus::from_str(self.property_value("STATUS")?)
    }

    //pub fn repeats<R:Repeater+?Sized>(&mut self, repeat: R) -> &mut Self {
    //    unimplemented!()
    //}

    /// Mark a todo as uncompleted by removing completion-related properties.
    ///
    /// This removes the following properties:
    /// - COMPLETED: The timestamp when the task was completed
    /// - PERCENT-COMPLETE: The percentage of the task that has been completed
    /// - STATUS: The task status, if it was set to "COMPLETED"
    ///
    /// # Example
    /// ```
    /// use icalendar::{Todo, Component, TodoStatus};
    /// use chrono::Utc;
    ///
    /// let mut todo = Todo::new()
    ///     .completed(Utc::now())
    ///     .percent_complete(100)
    ///     .status(TodoStatus::Completed)
    ///     .done();
    ///
    /// // Later, when marking the task as uncompleted:
    /// todo.mark_uncompleted();
    /// ```
    pub fn mark_uncompleted(&mut self) -> &mut Self {
        self.remove_property("COMPLETED")
            .remove_property("PERCENT-COMPLETE");

        // Only remove STATUS if it's set to COMPLETED
        if let Some(TodoStatus::Completed) = self.get_status() {
            self.remove_property("STATUS");
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_properties_unset() {
        let todo = Todo::new();
        assert_eq!(todo.get_percent_complete(), None);
        assert_eq!(todo.get_status(), None);
        assert_eq!(todo.get_completed(), None);
        assert_eq!(todo.get_due(), None);
    }

    #[test]
    fn get_properties_set() {
        let completed = Utc.with_ymd_and_hms(2001, 3, 13, 14, 15, 16).unwrap();
        let todo = Todo::new()
            .percent_complete(42)
            .status(TodoStatus::NeedsAction)
            .completed(completed)
            .done();
        assert_eq!(todo.get_percent_complete(), Some(42));
        assert_eq!(todo.get_status(), Some(TodoStatus::NeedsAction));
        assert_eq!(todo.get_completed(), Some(completed))
    }

    #[test]
    fn get_date_times_naive() {
        let naive_date_time = NaiveDate::from_ymd_opt(2001, 3, 13)
            .unwrap()
            .and_hms_opt(14, 15, 16)
            .unwrap();
        let todo = Todo::new().due(naive_date_time).done();
        assert_eq!(todo.get_due(), Some(naive_date_time.into()));
    }

    #[test]
    fn get_date_times_utc() {
        let utc_date_time = Utc.with_ymd_and_hms(2001, 3, 13, 14, 15, 16).unwrap();
        let todo = Todo::new()
            .due(utc_date_time)
            .completed(utc_date_time)
            .done();
        assert_eq!(todo.get_due(), Some(utc_date_time.into()));
        assert_eq!(todo.get_completed(), Some(utc_date_time));
    }

    #[test]
    fn get_dates_naive() {
        let naive_date = NaiveDate::from_ymd_opt(2001, 3, 13).unwrap();
        let todo = Todo::new().due(naive_date).done();
        assert_eq!(todo.get_due(), Some(naive_date.into()));
    }

    #[test]
    fn test_mark_uncompleted() {
        let completed = Utc.with_ymd_and_hms(2001, 3, 13, 14, 15, 16).unwrap();

        // Create a completed todo
        let mut todo = Todo::new()
            .percent_complete(100)
            .status(TodoStatus::Completed)
            .completed(completed)
            .done();

        // Verify completed state
        assert_eq!(todo.get_percent_complete(), Some(100));
        assert_eq!(todo.get_status(), Some(TodoStatus::Completed));
        assert_eq!(todo.get_completed(), Some(completed));

        // Mark as uncompleted
        todo.mark_uncompleted();

        // Verify uncompleted state
        assert_eq!(todo.get_percent_complete(), None);
        assert_eq!(todo.get_status(), None);
        assert_eq!(todo.get_completed(), None);
    }
}
