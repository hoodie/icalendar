//! RFC9073 Event Publishing Extensions Components
//!
//! This module provides the PARTICIPANT, VLOCATION, and VRESOURCE components
//! as defined in [RFC9073: Event Publishing Extensions to iCalendar](https://www.rfc-editor.org/rfc/rfc9073.html).

use crate::{properties::Property, CalendarComponent};

use super::*;

/// PARTICIPANT component as defined in [RFC9073 Section 7.1](https://www.rfc-editor.org/rfc/rfc9073.html#section-7.1)
///
/// The PARTICIPANT component provides information about a participant in an event,
/// such as performers, sponsors, or contacts.
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

    /// Adds a STRUCTURED-DATA property (multi-instance allowed).
    ///
    /// [RFC9073 Section 6.6: STRUCTURED-DATA](https://www.rfc-editor.org/rfc/rfc9073.html#section-6.6)
    ///
    /// This property MAY occur multiple times with ORDER parameter to indicate preference.
    pub fn structured_data(&mut self, value: &str, params: &[(&str, &str)]) -> &mut Self {
        let mut prop = Property::new("STRUCTURED-DATA", value);
        for (k, v) in params {
            prop.add_parameter(k, v);
        }
        self.inner.insert_multi(prop);
        self
    }

    /// Gets the first STRUCTURED-DATA property value.
    pub fn get_structured_data(&self) -> Option<&str> {
        self.inner
            .multi_properties
            .get("STRUCTURED-DATA")
            .and_then(|v| v.first())
            .map(Property::value)
    }

    /// Gets all STRUCTURED-DATA properties.
    ///
    /// Returns a slice of all STRUCTURED-DATA properties, which may have different
    /// ORDER parameters to indicate preference ordering.
    pub fn get_all_structured_data(&self) -> &[Property] {
        self.inner
            .multi_properties
            .get("STRUCTURED-DATA")
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }
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

/// VLOCATION component as defined in [RFC9073 Section 7.2](https://www.rfc-editor.org/rfc/rfc9073.html#section-7.2)
///
/// The VLOCATION component provides detailed location information for an event,
/// including venue details, parking locations, or other related places.
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
    ///
    /// [RFC9073 Section 6.1: LOCATION-TYPE](https://www.rfc-editor.org/rfc/rfc9073.html#section-6.1)
    ///
    /// Values are defined in [RFC4589](https://www.rfc-editor.org/rfc/rfc4589.html).
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

    /// Adds a STRUCTURED-DATA property (multi-instance allowed).
    ///
    /// [RFC9073 Section 6.6: STRUCTURED-DATA](https://www.rfc-editor.org/rfc/rfc9073.html#section-6.6)
    ///
    /// This property MAY occur multiple times with ORDER parameter to indicate preference.
    pub fn structured_data(&mut self, value: &str, params: &[(&str, &str)]) -> &mut Self {
        let mut prop = Property::new("STRUCTURED-DATA", value);
        for (k, v) in params {
            prop.add_parameter(k, v);
        }
        self.inner.insert_multi(prop);
        self
    }

    /// Gets the first STRUCTURED-DATA property value.
    pub fn get_structured_data(&self) -> Option<&str> {
        self.inner
            .multi_properties
            .get("STRUCTURED-DATA")
            .and_then(|v| v.first())
            .map(Property::value)
    }

    /// Gets all STRUCTURED-DATA properties.
    ///
    /// Returns a slice of all STRUCTURED-DATA properties, which may have different
    /// ORDER parameters to indicate preference ordering.
    pub fn get_all_structured_data(&self) -> &[Property] {
        self.inner
            .multi_properties
            .get("STRUCTURED-DATA")
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }
}

/// VRESOURCE component as defined in [RFC9073 Section 7.3](https://www.rfc-editor.org/rfc/rfc9073.html#section-7.3)
///
/// The VRESOURCE component provides information about resources used in an event,
/// such as projectors, rooms, or remote conferencing equipment.
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

    /// Adds a STRUCTURED-DATA property (multi-instance allowed).
    ///
    /// [RFC9073 Section 6.6: STRUCTURED-DATA](https://www.rfc-editor.org/rfc/rfc9073.html#section-6.6)
    ///
    /// This property MAY occur multiple times with ORDER parameter to indicate preference.
    pub fn structured_data(&mut self, value: &str, params: &[(&str, &str)]) -> &mut Self {
        let mut prop = Property::new("STRUCTURED-DATA", value);
        for (k, v) in params {
            prop.add_parameter(k, v);
        }
        self.inner.insert_multi(prop);
        self
    }

    /// Gets the first STRUCTURED-DATA property value.
    pub fn get_structured_data(&self) -> Option<&str> {
        self.inner
            .multi_properties
            .get("STRUCTURED-DATA")
            .and_then(|v| v.first())
            .map(Property::value)
    }

    /// Gets all STRUCTURED-DATA properties.
    ///
    /// Returns a slice of all STRUCTURED-DATA properties, which may have different
    /// ORDER parameters to indicate preference ordering.
    pub fn get_all_structured_data(&self) -> &[Property] {
        self.inner
            .multi_properties
            .get("STRUCTURED-DATA")
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }
}
