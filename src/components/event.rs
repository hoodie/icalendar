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
        EventStatus::from_str(self.property_value("STATUS")?)
    }

    //pub fn repeats<R:Repeater+?Sized>(&mut self, repeat: R) -> &mut Self {
    //    unimplemented!()
    //}
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
}
