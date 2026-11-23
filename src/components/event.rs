use super::*;
/// VEVENT [(RFC 5545, Section 3.6.1 )](https://tools.ietf.org/html/rfc5545#section-3.6.1)
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Event {
    pub(super) inner: InnerComponent,
}

impl Event {
    /// Creates a new Event.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Event with a UID.
    pub fn with_uid(uid: &str) -> Self {
        Self::new().uid(uid).done()
    }

    /// Set a date-only start and its exclusive next-day end.
    ///
    /// # Side effects
    /// Removes `DURATION` and replaces `DTSTART` and `DTEND`. If the next day
    /// cannot be represented, removes `DTEND` instead.
    pub fn all_day(&mut self, date: Date) -> &mut Self {
        self.remove_property("DURATION");
        self.append_property(naive_date_to_property(date, "DTSTART"));
        if let Ok(end_date) = date.checked_add(jiff::Span::new().days(1)) {
            self.append_property(naive_date_to_property(end_date, "DTEND"));
        } else {
            // A date-only DTSTART without DTEND or DURATION defaults to one day.
            self.remove_property("DTEND");
        }
        self
    }

    /// Set the non-inclusive `DTEND` of the event.
    ///
    /// # Side effects
    /// Removes `DURATION`.
    pub fn ends<T: Into<DatePerhapsTime>>(&mut self, date_time: T) -> &mut Self {
        self.remove_property("DURATION")
            .append_property(date_time.into().to_property("DTEND"))
    }

    /// Remove `DTEND`.
    pub fn remove_ends(&mut self) -> &mut Self {
        self.remove_property("DTEND")
    }

    /// End of builder pattern.
    /// copies over everything
    pub fn done(&mut self) -> Self {
        Event {
            inner: self.inner.done(),
        }
    }

    /// Defines the overall status or confirmation
    pub fn status(&mut self, status: EventStatus) -> &mut Self {
        self.append_property(status)
    }

    /// Gets the overall status or confirmation.
    pub fn get_status(&self) -> Option<EventStatus> {
        self.property_value("STATUS")?.parse().ok()
    }

    //pub fn repeats<R:Repeater+?Sized>(&mut self, repeat: R) -> &mut Self {
    //    unimplemented!()
    //}

    /// Remove the status property from an event
    pub fn remove_status(&mut self) -> &mut Self {
        self.remove_property("STATUS")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_properties_unset() {
        let event = Event::new();
        assert_eq!(event.get_status(), None);
    }

    #[test]
    fn get_properties_set() {
        let event = Event::new().status(EventStatus::Tentative).done();
        assert_eq!(event.get_status(), Some(EventStatus::Tentative));
    }

    #[test]
    fn test_remove_status() {
        let mut event = Event::new().status(EventStatus::Cancelled).done();

        assert_eq!(event.get_status(), Some(EventStatus::Cancelled));

        event.remove_status();

        assert_eq!(event.get_status(), None);
    }
}
