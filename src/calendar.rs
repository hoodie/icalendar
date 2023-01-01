use chrono::Duration;
use std::{fmt, iter::FromIterator, mem, ops::Deref};

use crate::{components::*, Parameter, Property};

mod calendar_component;

pub use calendar_component::CalendarComponent;

const VERSION_PROPERTY_NAME: &str = "VERSION";
const PRODID_PROPERTY_NAME: &str = "PRODID";
const CALSCALE_PROPERTY_NAME: &str = "CALSCALE";

/// Represents a calendar
///
///
/// ### create calendar from an array of calendar events
/// You can create a [`Calendar`] in a few different ways.
/// ```
/// # use icalendar::*;
/// let todo1 = Todo::new();
/// let todo2 = Todo::new();
///
/// let calendar = Calendar::from([todo1, todo2])
///     .name("things that need to get done")
///     .print();
/// ```
///
/// ### push events into a calendar
/// ```
/// # use icalendar::*;
/// let todo = Todo::new();
/// let event = Event::new();
///
/// let mut calendar = Calendar::new();
/// calendar.push(todo);
/// calendar.push(event);
/// calendar.print();
/// ```
///
/// ## Container semantics
///
/// ### collect into a calendar from an `iterator` of calendar events
/// ```
/// # use icalendar::*;
/// let todo1 = Todo::new();
/// let todo2 = Todo::new();
///
/// let cal_from_iterator = vec![todo1, todo2]
///     .into_iter()
///     .collect::<Calendar>();
/// ```
///
/// ### `Calendar` is a container for `CalendarElement`
/// ```
/// # use icalendar::*;
/// let todo1 = Todo::new();
/// let todo2 = Todo::new();
///
/// let calendar = Calendar::from([todo1, todo2]);
/// for element in calendar.iter() {
/// // ...
/// }
/// ```
///
///
#[derive(Debug, PartialEq, Eq)]
pub struct Calendar {
    /// Top-level calendar properties
    pub properties: Vec<Property>,
    /// Events, Todos and Venues defined in the calendar
    pub components: Vec<CalendarComponent>,
}

impl Default for Calendar {
    fn default() -> Self {
        Self {
            properties: Property::from_array([
                (VERSION_PROPERTY_NAME, "2.0"),
                (PRODID_PROPERTY_NAME, "ICALENDAR-RS"),
                (CALSCALE_PROPERTY_NAME, "GREGORIAN"),
            ]),
            components: Default::default(),
        }
    }
}

impl Calendar {
    /// Creates a new Calendar.
    pub fn new() -> Self {
        Default::default()
    }

    /// Produces a calendar without any default properties.
    ///
    /// [`Calendar::new()`] and [`Calendar::default()`] will prefill the properties `VERSION`, `PRODID` and `CALSCALE`, this method does not.
    /// ```
    /// assert_eq!(icalendar::Calendar::empty().properties.len(), 0);
    /// ```
    pub fn empty() -> Self {
        Self {
            properties: Default::default(),
            components: Default::default(),
        }
    }

    #[deprecated(note = "Use .push() instead")]
    #[doc(hidden)]
    pub fn add<T: Into<CalendarComponent>>(&mut self, component: T) -> &mut Self {
        self.push(component)
    }

    /// Moves all the elements of other into Self, leaving other empty.
    pub fn append(&mut self, other: &mut Calendar) {
        self.components.append(&mut other.components);
    }

    /// Append a given `Property` to the `Calendar`
    pub fn append_property(&mut self, property: impl Into<Property>) -> &mut Self {
        self.properties.push(property.into());
        self
    }

    /// Gets the value of a property.
    pub fn property_value(&self, key: &str) -> Option<&str> {
        Some(
            self.properties
                .iter()
                .find(|property| property.key() == key)?
                .value(),
        )
    }

    /// Extends this `Calendar` with the contends of another.
    pub fn extend<T, U>(&mut self, other: T)
    where
        T: IntoIterator<Item = U>,
        U: Into<CalendarComponent>,
    {
        self.components.extend(other.into_iter().map(Into::into));
    }

    /// Appends an element to the back of the `Calendar`.
    pub fn push<T: Into<CalendarComponent>>(&mut self, component: T) -> &mut Self {
        self.components.push(component.into());
        self
    }

    /// Set the `NAME` and `X-WR-CALNAME` `Property`s
    // TODO: where is `NAME` specified? it's not in rfc5545 or rfc2445
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.append_property(Property::new("NAME", name));
        self.append_property(Property::new("X-WR-CALNAME", name));
        self
    }

    /// Gets the value of the `NAME` or `X-WR-CALNAME` property.
    pub fn get_name(&self) -> Option<&str> {
        self.property_value("NAME")
            .or_else(|| self.property_value("X-WR-CALNAME"))
    }

    /// Set the [`DESCRIPTION`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.5) and `X-WR-CALDESC` `Property`s
    pub fn description(&mut self, description: &str) -> &mut Self {
        self.append_property(Property::new("DESCRIPTION", description));
        self.append_property(Property::new("X-WR-CALDESC", description));
        self
    }

    /// Gets the value of the `DESCRIPTION` or `X-WR-CALDESC` property.
    pub fn get_description(&self) -> Option<&str> {
        self.property_value("DESCRIPTION")
            .or_else(|| self.property_value("X-WR-CALDESC"))
    }

    /// Set the `TIMEZONE-ID` and `X-WR-TIMEZONE` `Property`s
    // TODO: where is `TIMEZONE-ID` specified? it's not in rfc5545 or rfc2445
    pub fn timezone(&mut self, timezone: &str) -> &mut Self {
        self.append_property(Property::new("TIMEZONE-ID", timezone));
        self.append_property(Property::new("X-WR-TIMEZONE", timezone));
        self
    }

    /// Gets the value of the `TIMEZONE_ID` or `X-WR-TIMEZONE` property.
    pub fn get_timezone(&self) -> Option<&str> {
        self.property_value("TIMEZONE_ID")
            .or_else(|| self.property_value("X-WR-TIMEZONE"))
    }

    /// Set the `REFRESH-INTERVAL` and `X-PUBLISHED-TTL` `Property`s
    pub fn ttl(&mut self, duration: &Duration) -> &mut Self {
        let duration_string = duration.to_string();
        self.append_property(
            Property::new("REFRESH-INTERVAL", &duration_string)
                .append_parameter(Parameter::new("VALUE", "DURATION"))
                .done(),
        );
        self.append_property(Property::new("X-PUBLISHED-TTL", &duration_string));
        self
    }

    /// End of builder pattern.
    /// copies over everything
    pub fn done(&mut self) -> Self {
        Calendar {
            properties: mem::take(&mut self.properties),
            components: mem::take(&mut self.components),
        }
    }

    /// Writes `Calendar` into a `Writer` using `std::fmt`.
    fn fmt_write<W: fmt::Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        write_crlf!(out, "BEGIN:VCALENDAR")?;
        for property in &self.properties {
            property.fmt_write(out)?;
        }

        for component in &self.components {
            component.fmt_write(out)?;
        }
        write_crlf!(out, "END:VCALENDAR")?;
        Ok(())
    }

    /// Prints to stdout
    pub fn print(&self) -> Result<(), fmt::Error> {
        print_crlf!("{}", self);
        Ok(())
    }
}

impl fmt::Display for Calendar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_write(f)
    }
}

impl TryInto<String> for &Calendar {
    type Error = std::fmt::Error;
    fn try_into(self) -> Result<String, Self::Error> {
        let mut out_string = String::new();
        self.fmt_write(&mut out_string)?;
        Ok(out_string)
    }
}

impl Deref for Calendar {
    type Target = [CalendarComponent];

    fn deref(&self) -> &[CalendarComponent] {
        self.components.deref()
    }
}

impl AsRef<[CalendarComponent]> for Calendar {
    fn as_ref(&self) -> &[CalendarComponent] {
        self.components.deref()
    }
}

impl<T: Into<CalendarComponent>, const N: usize> From<[T; N]> for Calendar {
    fn from(elements: [T; N]) -> Self {
        elements.into_iter().collect()
    }
}

impl<C: Into<CalendarComponent>> From<C> for Calendar {
    fn from(element: C) -> Self {
        Calendar {
            components: vec![element.into()],
            ..Default::default()
        }
    }
}

impl<C: Into<CalendarComponent>> FromIterator<C> for Calendar {
    fn from_iter<T: IntoIterator<Item = C>>(iter: T) -> Self {
        Calendar {
            components: iter.into_iter().map(Into::into).collect(),
            properties: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Check that a default [`Calendar`] has certain properties set.
    #[test]
    fn calender_default_properties() {
      let calendar = Calendar::default();
      assert!(calendar.property_value(VERSION_PROPERTY_NAME).is_some());
      assert!(calendar.property_value(PRODID_PROPERTY_NAME).is_some());
      assert!(calendar.property_value(CALSCALE_PROPERTY_NAME).is_some());
    }

    /// Check that a [`Calendar`] created from a single
    /// [`CalendarComponent`] has default properties set.
    #[test]
    fn calender_from_component_properties() {
      let calendar = Calendar::from(Todo::new());
      assert!(calendar.property_value(VERSION_PROPERTY_NAME).is_some());
      assert!(calendar.property_value(PRODID_PROPERTY_NAME).is_some());
      assert!(calendar.property_value(CALSCALE_PROPERTY_NAME).is_some());
    }

    /// Check that a [`Calendar`] created from a list of
    /// [`CalendarComponent`] objects has default properties set.
    #[test]
    fn calender_from_components_properties() {
      let todos = [Todo::new(); 0];
      let calendar = Calendar::from(todos);
      assert!(calendar.property_value(VERSION_PROPERTY_NAME).is_some());
      assert!(calendar.property_value(PRODID_PROPERTY_NAME).is_some());
      assert!(calendar.property_value(CALSCALE_PROPERTY_NAME).is_some());
    }

    #[test]
    fn calendar_extend_components() {
        let mut calendar = Calendar::new();
        let components = vec![
            CalendarComponent::Event(Event::new()),
            CalendarComponent::Event(Event::new()),
        ];
        calendar.extend(components);
        assert_eq!(calendar.components.len(), 2);
    }

    #[test]
    fn calendar_extend_events() {
        let mut calendar = Calendar::new();
        let events = vec![Event::new(), Event::new()];
        calendar.extend(events);
        assert_eq!(calendar.components.len(), 2);
    }

    #[test]
    fn get_properties_unset() {
        let calendar = Calendar::new();
        assert_eq!(calendar.get_name(), None);
        assert_eq!(calendar.get_description(), None);
        assert_eq!(calendar.get_timezone(), None);
    }

    #[test]
    fn get_properties_set() {
        let calendar = Calendar::new()
            .name("name")
            .description("description")
            .timezone("timezone")
            .done();
        assert_eq!(calendar.get_name(), Some("name"));
        assert_eq!(calendar.get_description(), Some("description"));
        assert_eq!(calendar.get_timezone(), Some("timezone"));
    }

    #[test]
    fn get_properties_alternate() {
        let calendar = Calendar::new()
            .append_property(Property::new("X-WR-CALNAME", "name"))
            .append_property(Property::new("X-WR-CALDESC", "description"))
            .append_property(Property::new("X-WR-TIMEZONE", "timezone"))
            .done();
        assert_eq!(calendar.get_name(), Some("name"));
        assert_eq!(calendar.get_description(), Some("description"));
        assert_eq!(calendar.get_timezone(), Some("timezone"));
    }

    #[test]
    #[cfg(feature = "parser")]
    fn emit_parse_icalendar() {
        use std::str::FromStr;

        let mut original = Calendar::new();
        original.append_property(Property::new("FOOBAR", "foobar"));

        let emitted = original.to_string();
        let parsed = Calendar::from_str(&emitted).unwrap();

        pretty_assertions::assert_eq!(parsed.property_value("FOOBAR"), Some("foobar"));

        // this would not pass because icalendar-rs adds certain properties like CALSCALE or PRODID
        // pretty_assertions::assert_eq!(parsed, original)
    }
}
