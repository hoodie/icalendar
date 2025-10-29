#![cfg(feature = "rfc9073")]
use super::*;

/// PARTICIPANT (RFC9073)
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Participant {
    pub(super) inner: InnerComponent,
}

impl Participant {
    /// Creates a new Participant.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Participant with a UID.
    pub fn with_uid(uid: &str) -> Self {
        Self::new().uid(uid).done()
    }

    /// End of builder pattern.
    /// copies over everything
    pub fn done(&mut self) -> Self {
        Participant {
            inner: self.inner.done(),
        }
    }

    /// Sets the PARTICIPANT-TYPE property.
    pub fn participant_type(&mut self, value: &str) -> &mut Self {
        self.add_property("PARTICIPANT-TYPE", value)
    }

    /// Gets the PARTICIPANT-TYPE property.
    pub fn get_participant_type(&self) -> Option<&str> {
        self.property_value("PARTICIPANT-TYPE")
    }

    /// Sets the CALENDAR-ADDRESS property.
    pub fn calendar_address(&mut self, value: &str) -> &mut Self {
        self.add_property("CALENDAR-ADDRESS", value)
    }

    /// Gets the CALENDAR-ADDRESS property.
    pub fn get_calendar_address(&self) -> Option<&str> {
        self.property_value("CALENDAR-ADDRESS")
    }

    /// Sets the DESCRIPTION property.
    pub fn description(&mut self, value: &str) -> &mut Self {
        self.add_property("DESCRIPTION", value)
    }

    /// Gets the DESCRIPTION property.
    pub fn get_description(&self) -> Option<&str> {
        self.property_value("DESCRIPTION")
    }

    // TODO: Add more builder methods for other properties and nested components as needed.
}

/// LOCATION (RFC9073, VLOCATION)
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Location {
    pub(super) inner: InnerComponent,
}

impl Location {
    /// Creates a new Location.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Location with a UID.
    pub fn with_uid(uid: &str) -> Self {
        Self::new().uid(uid).done()
    }

    /// End of builder pattern.
    /// copies over everything
    pub fn done(&mut self) -> Self {
        Location {
            inner: self.inner.done(),
        }
    }

    /// Sets the LOCATION-TYPE property.
    pub fn location_type(&mut self, value: &str) -> &mut Self {
        self.add_property("LOCATION-TYPE", value)
    }

    /// Gets the LOCATION-TYPE property.
    pub fn get_location_type(&self) -> Option<&str> {
        self.property_value("LOCATION-TYPE")
    }

    /// Sets the NAME property.
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.add_property("NAME", value)
    }

    /// Gets the NAME property.
    pub fn get_name(&self) -> Option<&str> {
        self.property_value("NAME")
    }

    /// Sets the DESCRIPTION property.
    pub fn description(&mut self, value: &str) -> &mut Self {
        self.add_property("DESCRIPTION", value)
    }

    /// Gets the DESCRIPTION property.
    pub fn get_description(&self) -> Option<&str> {
        self.property_value("DESCRIPTION")
    }

    // TODO: Add more builder methods for other properties and nested components as needed.
}

/// RESOURCE (RFC9073, VRESOURCE)
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Resource {
    pub(super) inner: InnerComponent,
}

impl Resource {
    /// Creates a new Resource.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Resource with a UID.
    pub fn with_uid(uid: &str) -> Self {
        Self::new().uid(uid).done()
    }

    /// End of builder pattern.
    /// copies over everything
    pub fn done(&mut self) -> Self {
        Resource {
            inner: self.inner.done(),
        }
    }

    /// Sets the RESOURCE-TYPE property.
    pub fn resource_type(&mut self, value: &str) -> &mut Self {
        self.add_property("RESOURCE-TYPE", value)
    }

    /// Gets the RESOURCE-TYPE property.
    pub fn get_resource_type(&self) -> Option<&str> {
        self.property_value("RESOURCE-TYPE")
    }

    /// Sets the NAME property.
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.add_property("NAME", value)
    }

    /// Gets the NAME property.
    pub fn get_name(&self) -> Option<&str> {
        self.property_value("NAME")
    }

    /// Sets the DESCRIPTION property.
    pub fn description(&mut self, value: &str) -> &mut Self {
        self.add_property("DESCRIPTION", value)
    }

    /// Gets the DESCRIPTION property.
    pub fn get_description(&self) -> Option<&str> {
        self.property_value("DESCRIPTION")
    }

    // TODO: Add more builder methods for other properties and nested components as needed.
}

// TODO: Implement builder patterns, property setters/getters, and integration with calendar/event/todo components.
// TODO: Add tests using the Köln Concert fixture.
// TODO: Mark VVENUE as deprecated (in venue.rs) after migration.
