use chrono::Duration;
use std::{fmt, mem, ops::Deref};

#[cfg(feature = "recurrence")]
use crate::components::build_recurrence_set;
use crate::{Parameter, Property, components::*};

/// IANA timezone name newtype, used as the argument to [`Calendar::timezone`].
///
/// Private. Just here so `impl Into<TimezoneId>` works for plain strings
/// and (with `chrono-tz`) `chrono_tz::Tz` values.
struct TimezoneId(String);

impl From<&str> for TimezoneId {
    fn from(s: &str) -> Self {
        TimezoneId(s.to_owned())
    }
}

impl From<String> for TimezoneId {
    fn from(s: String) -> Self {
        TimezoneId(s)
    }
}

#[cfg(feature = "chrono-tz")]
impl From<chrono_tz::Tz> for TimezoneId {
    fn from(tz: chrono_tz::Tz) -> Self {
        TimezoneId(tz.name().to_owned())
    }
}

mod calendar_component;

pub use calendar_component::CalendarComponent;

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
/// ### `Calendar` is a container for `CalendarComponent`
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
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "crate::parser::Calendar"))]
#[cfg_attr(feature = "serde", serde(from = "crate::parser::Calendar"))]
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
                ("VERSION", "2.0"),
                ("PRODID", "ICALENDAR-RS"),
                ("CALSCALE", "GREGORIAN"),
            ]),
            components: Default::default(),
        }
    }
}

impl<U> Extend<U> for Calendar
where
    U: Into<CalendarComponent>,
{
    /// Extends this `Calendar` with the elements of an iterator.
    fn extend<T>(&mut self, other: T)
    where
        T: IntoIterator<Item = U>,
    {
        Calendar::extend(self, other);
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

    /// Extends this `Calendar` with the elements of an iterator.
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

    /// Set the [`NAME`](https://datatracker.ietf.org/doc/html/rfc7986#section-5.1) and `X-WR-CALNAME` properties.
    ///
    /// `NAME` is from [RFC 7986](https://datatracker.ietf.org/doc/html/rfc7986).
    /// `X-WR-CALNAME` is the Apple iCal extension; most clients understand it.
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

    /// Set the `X-WR-TIMEZONE` property (the calendar's default timezone).
    ///
    /// Accepts a plain IANA string or, with the `chrono-tz` feature, a `chrono_tz::Tz`
    /// value (name validated at compile time).
    ///
    /// ```
    /// # use icalendar::Calendar;
    /// let cal = Calendar::new().timezone("Europe/Berlin").done();
    /// assert_eq!(cal.get_timezone(), Some("Europe/Berlin"));
    /// ```
    #[allow(private_bounds)]
    pub fn timezone(&mut self, timezone: impl Into<TimezoneId>) -> &mut Self {
        let id = timezone.into();
        self.append_property(Property::new("X-WR-TIMEZONE", &id.0));
        self
    }

    /// Returns the `X-WR-TIMEZONE` value, or `None` if unset.
    ///
    /// Older versions of this crate wrote `TIMEZONE-ID` instead. That property is no
    /// longer read here - use [`property_value("TIMEZONE-ID")`](Calendar::property_value)
    /// if you need to handle those old calendars.
    pub fn get_timezone(&self) -> Option<&str> {
        self.property_value("X-WR-TIMEZONE")
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

    /// Gets the value of the `REFRESH-INTERVAL` or `X-PUBLISHED-TTL` property.
    pub fn get_ttl(&self) -> Option<Duration> {
        self.property_value("REFRESH-INTERVAL")
            .and_then(|refresh_interval| iso8601::duration(refresh_interval).ok())
            .or_else(|| {
                self.property_value("X-PUBLISHED-TTL")
                    .and_then(|published_ttl| iso8601::duration(published_ttl).ok())
            })
            .map(std::time::Duration::from)
            .map(Duration::from_std)
            .transpose()
            .ok()
            .flatten()
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

    /// Returns an iterator over all `Event` components.
    ///
    /// For timezone-aware recurrence on all-day events, use
    /// [`calendar_events()`](Calendar::calendar_events) instead.
    ///
    // TODO: next semver-major, change return type to `impl Iterator<Item = CalendarEvent<'_>>`
    // and drop calendar_events().
    pub fn events(&self) -> impl Iterator<Item = &Event> {
        self.components
            .iter()
            .filter_map(|component| match component {
                CalendarComponent::Event(event) => Some(event),
                _ => None,
            })
    }

    /// Returns an iterator over all mutable `Event` components.
    pub fn events_mut(&mut self) -> impl Iterator<Item = &mut Event> {
        self.components
            .iter_mut()
            .filter_map(|component| match component {
                CalendarComponent::Event(event) => Some(event),
                _ => None,
            })
    }

    /// Returns an iterator over all `Todo` components.
    ///
    /// For timezone-aware recurrence on all-day todos, use
    /// [`calendar_todos()`](Calendar::calendar_todos) instead.
    ///
    // TODO: next semver-major, change return type to `impl Iterator<Item = CalendarTodo<'_>>`
    // and drop calendar_todos().
    pub fn todos(&self) -> impl Iterator<Item = &Todo> {
        self.components
            .iter()
            .filter_map(|component| match component {
                CalendarComponent::Todo(todo) => Some(todo),
                _ => None,
            })
    }

    /// Returns an iterator over all mutable `Todo` components.
    pub fn todos_mut(&mut self) -> impl Iterator<Item = &mut Todo> {
        self.components
            .iter_mut()
            .filter_map(|component| match component {
                CalendarComponent::Todo(todo) => Some(todo),
                _ => None,
            })
    }

    /// Like [`events()`](Calendar::events) but each item carries the calendar's timezone.
    ///
    /// Needed for timezone-aware recurrence on all-day events (`recurrence` feature).
    pub fn calendar_events(&self) -> impl Iterator<Item = CalendarEvent<'_>> {
        let tz = self.get_timezone();
        self.events().map(move |event| CalendarEvent {
            event,
            calendar_tz: tz,
        })
    }

    /// Like [`todos()`](Calendar::todos) but each item carries the calendar's timezone.
    ///
    /// Needed for timezone-aware recurrence on all-day todos (`recurrence` feature).
    pub fn calendar_todos(&self) -> impl Iterator<Item = CalendarTodo<'_>> {
        let tz = self.get_timezone();
        self.todos().map(move |todo| CalendarTodo {
            todo,
            calendar_tz: tz,
        })
    }
}

/// Borrowed view of an [`Event`] paired with its calendar's timezone.
///
/// Obtained from [`Calendar::calendar_events`]. The timezone is needed to anchor
/// DATE-only `DTSTART` values when expanding recurrences.
///
/// ## Serialisation
///
/// Doesn't implement `Serialize`/`Deserialize` - it's a view, not owned data.
/// Serialise the inner event via [`.event()`](CalendarEvent::event).
#[derive(Debug, Clone, Copy)]
pub struct CalendarEvent<'a> {
    event: &'a Event,
    calendar_tz: Option<&'a str>,
}

impl<'a> CalendarEvent<'a> {
    /// The underlying event.
    pub fn event(&self) -> &'a Event {
        self.event
    }

    /// The calendar's timezone, if one was set.
    pub fn calendar_tz(&self) -> Option<&str> {
        self.calendar_tz
    }

    /// Like [`EventLike::get_recurrence`] but anchors DATE-only values to the calendar timezone.
    #[cfg(feature = "recurrence")]
    pub fn get_recurrence(&self) -> Result<rrule::RRuleSet, crate::RecurrenceError> {
        build_recurrence_set(self.event, self.calendar_tz)
    }
}

impl<'a> Deref for CalendarEvent<'a> {
    type Target = Event;
    fn deref(&self) -> &Event {
        self.event
    }
}

/// Borrowed view of a [`Todo`] paired with its calendar's timezone.
///
/// Obtained from [`Calendar::calendar_todos`]. The timezone is needed to anchor
/// DATE-only `DTSTART` values when expanding recurrences.
///
/// ## Serialisation
///
/// Doesn't implement `Serialize`/`Deserialize` - it's a view, not owned data.
/// Serialise the inner todo via [`.todo()`](CalendarTodo::todo).
#[derive(Debug, Clone, Copy)]
pub struct CalendarTodo<'a> {
    todo: &'a Todo,
    calendar_tz: Option<&'a str>,
}

impl<'a> CalendarTodo<'a> {
    /// The underlying todo.
    pub fn todo(&self) -> &'a Todo {
        self.todo
    }

    /// The calendar's timezone, if one was set.
    pub fn calendar_tz(&self) -> Option<&str> {
        self.calendar_tz
    }

    /// Like [`EventLike::get_recurrence`] but anchors DATE-only values to the calendar timezone.
    #[cfg(feature = "recurrence")]
    pub fn get_recurrence(&self) -> Result<rrule::RRuleSet, crate::RecurrenceError> {
        build_recurrence_set(self.todo, self.calendar_tz)
    }
}

impl<'a> Deref for CalendarTodo<'a> {
    type Target = Todo;
    fn deref(&self) -> &Todo {
        self.todo
    }
}

impl fmt::Display for Calendar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_write(f)
    }
}

impl TryInto<String> for &Calendar {
    type Error = fmt::Error;
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
            ..Default::default()
        }
    }
}
#[test]
fn from_adds_default_properties() {
    let todo = Todo::default();
    let cal = Calendar::from([todo]);
    assert!(cal.property_value("VERSION").is_some());
    assert!(cal.property_value("CALSCALE").is_some());
    assert!(cal.property_value("PRODID").is_some());

    assert!(
        cal.property_value("VERSION")
            .and(cal.property_value("PRODID"))
            .and(cal.property_value("CALSCALE"))
            .is_some()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

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
            .done();
        assert_eq!(calendar.get_name(), Some("name"));
        assert_eq!(calendar.get_description(), Some("description"));
        assert_eq!(calendar.get_timezone(), None);
    }

    #[test]
    fn timezone_accepts_str() {
        let calendar = Calendar::new().timezone("Europe/Berlin").done();
        assert_eq!(calendar.get_timezone(), Some("Europe/Berlin"));
    }

    #[test]
    #[cfg(feature = "chrono-tz")]
    fn timezone_accepts_chrono_tz() {
        let calendar = Calendar::new().timezone(chrono_tz::Europe::Berlin).done();
        assert_eq!(calendar.get_timezone(), Some("Europe/Berlin"));
    }

    #[test]
    fn timezone_writes_only_xwr_timezone() {
        let calendar = Calendar::new().timezone("Europe/Berlin").done();
        let has_timezone_id = calendar.properties.iter().any(|p| p.key() == "TIMEZONE-ID");
        let xwr_count = calendar
            .properties
            .iter()
            .filter(|p| p.key() == "X-WR-TIMEZONE")
            .count();
        assert!(!has_timezone_id, "TIMEZONE-ID must not be written");
        assert_eq!(xwr_count, 1, "exactly one X-WR-TIMEZONE property expected");
        assert_eq!(calendar.get_timezone(), Some("Europe/Berlin"));
    }

    #[test]
    fn get_timezone_ignores_timezone_id() {
        // Simulate a calendar serialised by an older version of this crate that
        // wrote TIMEZONE-ID but not X-WR-TIMEZONE.
        let calendar = Calendar::new()
            .append_property(Property::new("TIMEZONE-ID", "Europe/Berlin"))
            .done();
        assert_eq!(
            calendar.get_timezone(),
            None,
            "get_timezone() must not read TIMEZONE-ID"
        );
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

        // this would not pass because icalendar adds certain properties like CALSCALE or PRODID
        // pretty_assertions::assert_eq!(parsed, original)
    }
}
