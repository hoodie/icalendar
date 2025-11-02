#![cfg(feature = "rfc9073")]
use crate::{CalendarComponent, properties::{Parameter, Property}};

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

    /// Helper to add a property with parameters.
    // TODO: is this really necessary? remove if possible
    fn add_property_with_params(
        &mut self,
        key: &str,
        value: &str,
        params: &[(&str, &str)],
    ) -> &mut Self {
        let mut prop = Property::new(key, value);
        for (k, v) in params {
            prop.add_parameter(k, v);
        }
        self.inner.properties.insert(key.to_string(), prop);
        self
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
    ///
    /// [RFC9073 Section 6.2: PARTICIPANT-TYPE](https://www.rfc-editor.org/rfc/rfc9073.html#section-6.2)
    pub fn participant_type(&mut self, value: &str) -> &mut Self {
        self.add_property("PARTICIPANT-TYPE", value)
    }

    /// Gets the PARTICIPANT-TYPE property.
    pub fn get_participant_type(&self) -> Option<&str> {
        self.property_value("PARTICIPANT-TYPE")
    }

    /// Sets the CALENDAR-ADDRESS property.
    ///
    /// [RFC9073 Section 6.4: CALENDAR-ADDRESS](https://www.rfc-editor.org/rfc/rfc9073.html#section-6.4)
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

    /// Sets the STRUCTURED-DATA property.
    ///
    /// [RFC9073 Section 6.6: STRUCTURED-DATA](https://www.rfc-editor.org/rfc/rfc9073.html#section-6.6)
    /// Accepts value and optional parameters as Vec of (key, value) pairs.
    pub fn structured_data(&mut self, value: &str, params: &[(&str, &str)]) -> &mut Self {
        self.add_property_with_params("STRUCTURED-DATA", value, params)
    }

    /// Gets the STRUCTURED-DATA property value (first occurrence).
    pub fn get_structured_data(&self) -> Option<&str> {
        self.property_value("STRUCTURED-DATA")
    }



    // TODO: Add more builder methods for other properties and nested components as needed.
}

impl From<Participant> for CalendarComponent {
    fn from(val: Participant) -> Self {
        CalendarComponent::Participant(val)
    }
}

impl From<&Participant> for CalendarComponent {
    fn from(val: &Participant) -> Self {
        CalendarComponent::Participant(val.to_owned())
    }
}

impl From<&mut Participant> for CalendarComponent {
    fn from(val: &mut Participant) -> Self {
        CalendarComponent::Participant(val.to_owned())
    }
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

    /// Helper to add a property with parameters.
    fn add_property_with_params(
        &mut self,
        key: &str,
        value: &str,
        params: &[(&str, &str)],
    ) -> &mut Self {
        let mut prop = Property::new(key, value);
        for (k, v) in params {
            prop.add_parameter(k, v);
        }
        self.inner.properties.insert(key.to_string(), prop);
        self
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
    ///
    /// [RFC9073 Section 6.1: LOCATION-TYPE](https://www.rfc-editor.org/rfc/rfc9073.html#section-6.1)
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

    /// Sets the STRUCTURED-DATA property.
    ///
    /// [RFC9073 Section 6.6: STRUCTURED-DATA](https://www.rfc-editor.org/rfc/rfc9073.html#section-6.6)
    /// Accepts value and optional parameters as Vec of (key, value) pairs.
    pub fn structured_data(&mut self, value: &str, params: &[(&str, &str)]) -> &mut Self {
        self.add_property_with_params("STRUCTURED-DATA", value, params)
    }

    /// Gets the STRUCTURED-DATA property value (first occurrence).
    pub fn get_structured_data(&self) -> Option<&str> {
        self.property_value("STRUCTURED-DATA")
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

    /// Helper to add a property with parameters.
    fn add_property_with_params(
        &mut self,
        key: &str,
        value: &str,
        params: &[(&str, &str)],
    ) -> &mut Self {
        let mut prop = Property::new(key, value);
        for (k, v) in params {
            prop.add_parameter(k, v);
        }
        self.inner.properties.insert(key.to_string(), prop);
        self
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
    ///
    /// [RFC9073 Section 6.3: RESOURCE-TYPE](https://www.rfc-editor.org/rfc/rfc9073.html#section-6.3)
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

    /// Sets the STRUCTURED-DATA property.
    ///
    /// [RFC9073 Section 6.6: STRUCTURED-DATA](https://www.rfc-editor.org/rfc/rfc9073.html#section-6.6)
    /// Accepts value and optional parameters as Vec of (key, value) pairs.
    pub fn structured_data(&mut self, value: &str, params: &[(&str, &str)]) -> &mut Self {
        self.add_property_with_params("STRUCTURED-DATA", value, params)
    }

    /// Gets the STRUCTURED-DATA property value (first occurrence).
    pub fn get_structured_data(&self) -> Option<&str> {
        self.property_value("STRUCTURED-DATA")
    }



    // TODO: Add more builder methods for other properties and nested components as needed.
}

// TODO: Implement builder patterns, property setters/getters, and integration with calendar/event/todo components.
// TODO: Add tests using the Köln Concert fixture.
// TODO: Mark VVENUE as deprecated (in venue.rs) after migration.
