use jiff::{Timestamp, civil::Date};
use uuid::Uuid;

use std::{collections::BTreeMap, fmt, mem};

use crate::{
    Attendee,
    properties::*,
    text::{join_text_list, split_text_list},
};
use date_time::{format_utc_date_time, naive_date_to_property, parse_utc_date_time};

pub mod alarm;
pub(crate) mod date_time;
pub(crate) mod duration;
mod event;
mod other;
mod todo;
mod venue;

use alarm::*;
#[cfg(test)]
use date_time::CalendarDateTime;
use date_time::DatePerhapsTime;
pub use event::*;
pub use other::*;
pub use todo::*;
pub use venue::*;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub(crate) struct InnerComponent {
    pub properties: BTreeMap<String, Property>,
    pub multi_properties: BTreeMap<String, Vec<Property>>,
    pub components: Vec<Other>,
}

impl From<Other> for InnerComponent {
    fn from(val: Other) -> Self {
        val.inner
    }
}

//impl<'a> Into<InnerComponent> for parser::Component<'a> {
//    fn into(self) -> InnerComponent {
//        unimplemented!()
//    }
//}

impl InnerComponent {
    /// End of builder pattern.
    /// moves all fields out of `self` and into the returned value
    pub fn done(&mut self) -> Self {
        InnerComponent {
            properties: mem::take(&mut self.properties),
            multi_properties: mem::take(&mut self.multi_properties),
            components: mem::take(&mut self.components),
        }
    }

    pub(crate) fn insert_multi(&mut self, property: impl Into<Property>) -> &mut Self {
        let property = property.into();
        let key = property.key().to_owned();

        self.multi_properties
            .entry(key)
            .and_modify(|v| v.push(property.to_owned()))
            .or_insert(vec![property.to_owned()]);
        self
    }

    #[cfg(test)]
    pub fn property_value(&self, key: &str) -> Option<&str> {
        Some(self.properties.get(key)?.value())
    }
}

/// Collects the items of a text list property (e.g. `CATEGORIES`) across all its occurrences.
fn get_text_list<C: Component + ?Sized>(component: &C, key: &str) -> Option<Vec<String>> {
    let single = component.properties().get(key);
    let multi = component.multi_properties().get(key);
    if single.is_none() && multi.is_none() {
        return None;
    }
    Some(
        single
            .into_iter()
            .chain(multi.into_iter().flatten())
            .flat_map(|p| split_text_list(p.value()))
            .collect(),
    )
}

/// Implemented by everything that goes into a `Calendar`
pub trait Component {
    /// Returns kind of component.
    ///
    ///
    /// Must be ALL CAPS
    /// These are used in the `BEGIN` and `END` line of the component.
    fn component_kind(&self) -> String;

    /// Allows access to the inner properties map.
    fn properties(&self) -> &BTreeMap<String, Property>;

    /// Allows access to the inner's child components.
    fn components(&self) -> &[Other];

    /// Read-only access to `multi_properties`
    fn multi_properties(&self) -> &BTreeMap<String, Vec<Property>>;

    /// Gets the value of a property.
    fn property_value(&self, key: &str) -> Option<&str> {
        Some(self.properties().get(key)?.value())
    }

    /// Writes [`Component`] using [`std::fmt`].
    fn fmt_write<W: fmt::Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        write_crlf!(out, "BEGIN:{}", self.component_kind())?;

        if !self.properties().contains_key("DTSTAMP") {
            let now = Timestamp::now();
            write_crlf!(out, "DTSTAMP:{}", format_utc_date_time(now))?;
        }

        for property in self.properties().values() {
            property.fmt_write(out)?;
        }

        if !self.properties().contains_key("UID") {
            write_crlf!(out, "UID:{}", Uuid::new_v4())?;
        }

        for property in self.multi_properties().values().flatten() {
            property.fmt_write(out)?;
        }

        for component in self.components() {
            component.fmt_write(out)?;
        }

        write_crlf!(out, "END:{}", self.component_kind())?;
        Ok(())
    }

    /// Serializes this component into [`rfc5545`](http://tools.ietf.org/html/rfc5545) again
    ///
    /// # Panic
    /// this can panic if [`std::fmt::write`] returns an Error
    /// use [`Component::try_into_string()`] if you don't like panicking
    fn to_string(&self) -> String {
        self.try_into_string().unwrap()
    }

    /// Serializes this component into [`rfc5545`](http://tools.ietf.org/html/rfc5545) again
    fn try_into_string(&self) -> Result<String, fmt::Error> {
        let mut out_string = String::new();
        self.fmt_write(&mut out_string)?;
        Ok(out_string)
    }

    /// Append a given [`Property`]
    fn append_property(&mut self, property: impl Into<Property>) -> &mut Self;

    /// Append a given [`Component`]
    fn append_component(&mut self, child: impl Into<Other>) -> &mut Self;

    /// Adds a [`Property`] of which there may be many
    fn append_multi_property(&mut self, property: impl Into<Property>) -> &mut Self;

    /// Construct and append a [`Property`]
    fn add_property(&mut self, key: impl Into<String>, val: impl Into<String>) -> &mut Self {
        self.append_property(Property::new(key, val))
    }

    /// Remove a property by its key
    fn remove_property(&mut self, key: &str) -> &mut Self;

    /// Remove a multi-property by its key
    fn remove_multi_property(&mut self, key: &str) -> &mut Self;

    #[deprecated]
    /// Construct and append a [`Property`]
    fn add_property_pre_alloc(&mut self, key: String, val: String) -> &mut Self {
        self.append_property(Property::new(key, val))
    }

    /// Construct and append a [`Property`]
    fn add_multi_property(&mut self, key: &str, val: &str) -> &mut Self {
        self.append_multi_property(Property::new(key, val))
    }

    /// Set the [`DTSTAMP`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.2) [`Property`]
    ///
    /// This must be a UTC date-time value.
    fn timestamp(&mut self, dt: Timestamp) -> &mut Self {
        self.add_property("DTSTAMP", format_utc_date_time(dt))
    }

    /// Remove the [`DTSTAMP`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.2) [`Property`]
    fn remove_timestamp(&mut self) -> &mut Self {
        self.remove_property("DTSTAMP")
    }

    /// Gets the [`DTSTAMP`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.2) property.
    fn get_timestamp(&self) -> Option<Timestamp> {
        parse_utc_date_time(self.property_value("DTSTAMP")?)
    }

    /// Gets the [`DTSTART`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.4) [`Property`]
    fn get_start(&self) -> Option<DatePerhapsTime> {
        DatePerhapsTime::from_property(self.properties().get("DTSTART")?)
    }

    /// Gets the [`RECURRENCE-ID`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.4) property.
    fn get_recurrence_id(&self) -> Option<DatePerhapsTime> {
        DatePerhapsTime::from_property(self.properties().get("RECURRENCE-ID")?)
    }

    /// Gets the [`DTEND`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.2) [`Property`]
    fn get_end(&self) -> Option<DatePerhapsTime> {
        DatePerhapsTime::from_property(self.properties().get("DTEND")?)
    }

    /// Defines the relative priority.
    ///
    /// Ranges from 0 to 10, larger values will be clamped to 9.
    /// 9 is the lowest priority.
    fn priority(&mut self, priority: u32) -> &mut Self {
        let priority = std::cmp::min(priority, 9);
        self.add_property("PRIORITY", priority.to_string())
    }

    /// Removes the relative priority.
    fn remove_priority(&mut self) -> &mut Self {
        self.remove_property("PRIORITY")
    }

    // /// Add the [`ATTACH`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.1) property
    // /// TODO: might have to move to Component
    // pub fn attach(&mut self, attachment: ) -> &mut Self {
    //     todo!()
    //     // self.append_multi_property(todo!())
    // }

    /// Gets the relative priority.
    ///
    /// Ranges from 0 to 10.
    fn get_priority(&self) -> Option<u32> {
        let priority = self.property_value("PRIORITY")?.parse().ok()?;
        if priority <= 10 { Some(priority) } else { None }
    }

    /// Prints to stdout
    fn print(&self) -> Result<(), fmt::Error> {
        let mut out = String::new();
        self.fmt_write(&mut out)?;
        print_crlf!("{}", out);
        Ok(())
    }

    /// Set the summary
    fn summary(&mut self, desc: &str) -> &mut Self {
        self.add_property("SUMMARY", desc)
    }

    /// Removes the summary
    fn remove_summary(&mut self) -> &mut Self {
        self.remove_property("SUMMARY")
    }

    /// Gets the summary
    fn get_summary(&self) -> Option<&str> {
        self.property_value("SUMMARY")
    }

    /// Set the description
    fn description(&mut self, desc: &str) -> &mut Self {
        self.add_property("DESCRIPTION", desc)
    }

    /// Removes the description
    fn remove_description(&mut self) -> &mut Self {
        self.remove_property("DESCRIPTION")
    }

    /// Gets the description
    fn get_description(&self) -> Option<&str> {
        self.property_value("DESCRIPTION")
    }

    /// Adds an attendee
    fn attendee(&mut self, attendee: Attendee) -> &mut Self {
        self.append_multi_property(attendee)
    }

    /// Returns all `ATTENDEE` properties parsed as `Attendee` structs.
    fn get_attendees(&self) -> Vec<Attendee> {
        self.multi_properties()
            .get("ATTENDEE")
            .map(|v| {
                v.iter()
                    .filter_map(|p| Attendee::try_from(p).ok())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    /// Set the UID
    fn uid(&mut self, uid: &str) -> &mut Self {
        self.add_property("UID", uid)
    }

    /// Gets the UID
    fn get_uid(&self) -> Option<&str> {
        self.property_value("UID")
    }

    /// Set the sequence
    fn sequence(&mut self, sequence: u32) -> &mut Self {
        self.add_property("SEQUENCE", sequence.to_string())
    }

    /// Removes the sequence
    fn remove_sequence(&mut self) -> &mut Self {
        self.remove_property("SEQUENCE")
    }

    /// Gets the sequence
    fn get_sequence(&self) -> Option<u32> {
        self.property_value("SEQUENCE").and_then(|s| s.parse().ok())
    }

    /// Set the visibility class
    fn class(&mut self, class: Class) -> &mut Self {
        self.append_property(class)
    }

    /// Removes the visibility class
    fn remove_class(&mut self) -> &mut Self {
        self.remove_property("CLASS")
    }

    /// Gets the visibility class
    fn get_class(&self) -> Option<Class> {
        Class::from_str(self.property_value("CLASS")?)
    }

    /// Sets the URL.
    fn url(&mut self, url: &str) -> &mut Self {
        self.add_property("URL", url)
    }

    /// Removes the URL.
    fn remove_url(&mut self) -> &mut Self {
        self.remove_property("URL")
    }

    /// Gets the URL.
    fn get_url(&self) -> Option<&str> {
        self.property_value("URL")
    }

    /// Set the [`LAST-MODIFIED`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.3) [`Property`]
    ///
    /// This must be a UTC date-time value.
    fn last_modified(&mut self, dt: Timestamp) -> &mut Self {
        self.add_property("LAST-MODIFIED", format_utc_date_time(dt))
    }

    /// Removes the [`LAST-MODIFIED`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.3) [`Property`]
    fn remove_last_modified(&mut self) -> &mut Self {
        self.remove_property("LAST-MODIFIED")
    }

    /// Gets the [`LAST-MODIFIED`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.3) property.
    fn get_last_modified(&self) -> Option<Timestamp> {
        parse_utc_date_time(self.property_value("LAST-MODIFIED")?)
    }

    /// Set the [`CREATED`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.1) [`Property`]
    ///
    /// This must be a UTC date-time value.
    fn created(&mut self, dt: Timestamp) -> &mut Self {
        self.add_property("CREATED", format_utc_date_time(dt))
    }

    /// Removes the [`CREATED`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.1) [`Property`]
    fn remove_created(&mut self) -> &mut Self {
        self.remove_property("CREATED")
    }

    /// Gets the [`CREATED`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.1) property.
    fn get_created(&self) -> Option<Timestamp> {
        parse_utc_date_time(self.property_value("CREATED")?)
    }
}

macro_rules! event_component_methods {
    ($component_type:ty, $end_property:literal) => {
        impl $component_type {
            /// Appends a `CATEGORIES` property. Each item is escaped.
            pub fn categories<I, S>(&mut self, categories: I) -> &mut Self
            where
                I: IntoIterator<Item = S>,
                S: AsRef<str>,
            {
                self.append_multi_property(Property::new("CATEGORIES", join_text_list(categories)))
            }

            /// Gets all categories, across every `CATEGORIES` property.
            pub fn get_categories(&self) -> Option<Vec<String>> {
                get_text_list(self, "CATEGORIES")
            }

            /// Appends a `RESOURCES` property. Each item is escaped.
            pub fn resources<I, S>(&mut self, resources: I) -> &mut Self
            where
                I: IntoIterator<Item = S>,
                S: AsRef<str>,
            {
                self.append_multi_property(Property::new("RESOURCES", join_text_list(resources)))
            }

            /// Gets all resources, across every `RESOURCES` property.
            pub fn get_resources(&self) -> Option<Vec<String>> {
                get_text_list(self, "RESOURCES")
            }

            /// Sets `DTSTART`.
            pub fn starts<T: Into<DatePerhapsTime>>(&mut self, date_time: T) -> &mut Self {
                self.append_property(date_time.into().to_property("DTSTART"))
            }

            /// Removes `DTSTART`.
            pub fn remove_starts(&mut self) -> &mut Self {
                self.remove_property("DTSTART")
            }

            /// Sets `DURATION`, replacing `DTEND` or `DUE`.
            ///
            /// The supplied span is not validated. RFC 5545 requires a positive
            /// duration, and date-only starts require days or weeks.
            pub fn lasts(&mut self, duration: jiff::Span) -> &mut Self {
                self.remove_property($end_property)
                    .append_property(duration)
            }

            /// Sets `RECURRENCE-ID`.
            pub fn recurrence_id<T: Into<DatePerhapsTime>>(&mut self, date_time: T) -> &mut Self {
                self.append_property(date_time.into().to_property("RECURRENCE-ID"))
            }

            /// Removes `RECURRENCE-ID`.
            pub fn remove_recurrence_id(&mut self) -> &mut Self {
                self.remove_property("RECURRENCE-ID")
            }

            /// Sets `LOCATION` with a venue UID from the expired ical-venue draft.
            pub fn venue(&mut self, location: &str, venue_uid: &str) -> &mut Self {
                self.append_property(
                    Property::new("LOCATION", location)
                        .append_parameter(Parameter::new("VVENUE", venue_uid))
                        .done(),
                )
            }

            /// Sets `LOCATION`.
            pub fn location(&mut self, location: &str) -> &mut Self {
                self.add_property("LOCATION", location)
            }

            /// Removes `LOCATION`.
            pub fn remove_location(&mut self) -> &mut Self {
                self.remove_property("LOCATION")
            }

            /// Gets `LOCATION`.
            pub fn get_location(&self) -> Option<&str> {
                self.property_value("LOCATION")
            }

            /// Appends an `RDATE` property.
            pub fn rdate<T: Into<DatePerhapsTime>>(&mut self, date_time: T) -> &mut Self {
                self.append_multi_property(date_time.into().to_property("RDATE"))
            }

            /// Appends an `EXDATE` property.
            pub fn exdate<T: Into<DatePerhapsTime>>(&mut self, date_time: T) -> &mut Self {
                self.append_multi_property(date_time.into().to_property("EXDATE"))
            }

            /// Appends an alarm.
            pub fn alarm<A: Into<Alarm>>(&mut self, alarm: A) -> &mut Self {
                self.append_component(alarm.into())
            }
        }
    };
}

macro_rules! component_impl {
    ($t:ty, $kind:expr) => {
        impl Component for $t {
            /// Tells you what kind of [`Component`] this is
            ///
            /// Might be [`VEVENT`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.1), [`VTODO`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.2), [`VALARM`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.6) etc
            fn component_kind(&self) -> String {
                $kind
            }

            /// Read-only access to `properties`
            fn properties(&self) -> &BTreeMap<String, Property> {
                &self.inner.properties
            }

            /// Read-only access to child `components`
            fn components(&self) -> &[Other] {
                &self.inner.components
            }

            /// Read-only access to `multi_properties`
            fn multi_properties(&self) -> &BTreeMap<String, Vec<Property>> {
                &self.inner.multi_properties
            }

            /// Adds a [`Property`]
            fn append_property(&mut self, property: impl Into<Property>) -> &mut Self {
                let property = property.into();
                self.inner
                    .properties
                    .insert(property.key().to_owned(), property);
                self
            }

            /// Appends a [`Component`]
            fn append_component(&mut self, child: impl Into<Other>) -> &mut Self {
                self.inner.components.push(child.into());
                self
            }

            /// Adds a [`Property`] of which there may be many
            fn append_multi_property(&mut self, property: impl Into<Property>) -> &mut Self {
                self.inner.insert_multi(property);
                self
            }

            /// Removes a [`Property`] by its key if it exists
            fn remove_property(&mut self, key: &str) -> &mut Self {
                self.inner.properties.remove(key);
                self
            }

            /// Removes a multi-property by its key if it exists
            fn remove_multi_property(&mut self, key: &str) -> &mut Self {
                self.inner.multi_properties.remove(key);
                self
            }
        }

        impl From<InnerComponent> for $t {
            fn from(inner: InnerComponent) -> $t {
                Self { inner }
            }
        }
        impl From<$t> for Other {
            fn from(val: $t) -> Self {
                (val.component_kind(), val.inner).into()
            }
        }

        impl TryInto<String> for $t {
            type Error = std::fmt::Error;

            fn try_into(self) -> Result<String, Self::Error> {
                self.try_into_string()
            }
        }
    };
}

component_impl! { Event, String::from("VEVENT") }
event_component_methods! { Event, "DTEND" }

component_impl! { Todo , String::from("VTODO")}
event_component_methods! { Todo, "DUE" }

component_impl! { Venue , String::from("VVENUE")}
component_impl! { Alarm, String::from("VALARM") }

#[cfg(test)]
mod tests {
    use jiff::{civil::date, tz::TimeZone};

    use super::*;

    #[test]
    fn text_list_getter_merges_all_occurrences() {
        let mut event = Event::new();
        event
            .add_property("CATEGORIES", r"Single\, one")
            .categories(["a", "b"])
            .categories(["c"]);
        assert_eq!(
            event.get_categories().unwrap(),
            ["Single, one", "a", "b", "c"]
        );

        assert_eq!(
            Event::new().add_property("CATEGORIES", "").get_categories(),
            Some(vec![])
        );
        assert_eq!(Event::new().get_categories(), None);
    }

    #[test]
    #[cfg(feature = "parser")]
    fn get_url() {
        let url = "http://hoodie.de/";
        let event = Event::new().url(url).done();

        let serialized = event.to_string();
        let reparsed =
            Other::from(crate::parser::Component::<'_>::try_from(serialized.as_str()).unwrap());

        assert_eq!(event.get_url(), Some(url));
        assert_eq!(reparsed.get_url(), Some(url));
    }

    #[test]
    fn get_properties_unset() {
        let event = Event::new();
        assert_eq!(event.get_priority(), None);
        assert_eq!(event.get_summary(), None);
        assert_eq!(event.get_description(), None);
        assert_eq!(event.get_location(), None);
        assert_eq!(event.get_uid(), None);
        assert_eq!(event.get_class(), None);
        assert_eq!(event.get_timestamp(), None);
        assert_eq!(event.get_last_modified(), None);
        assert_eq!(event.get_created(), None);
        assert_eq!(event.get_url(), None);
    }

    #[test]
    fn get_properties_set() {
        let event = Event::new()
            .priority(5)
            .summary("summary")
            .description("description")
            .location("location")
            .uid("uid")
            .class(Class::Private)
            .url("http://some.test/url")
            .done();
        assert_eq!(event.get_priority(), Some(5));
        assert_eq!(event.get_summary(), Some("summary"));
        assert_eq!(event.get_description(), Some("description"));
        assert_eq!(event.get_location(), Some("location"));
        assert_eq!(event.get_uid(), Some("uid"));
        assert_eq!(event.get_class(), Some(Class::Private));
        assert_eq!(event.get_url(), Some("http://some.test/url"));
    }

    #[test]
    fn get_properties_remove() {
        let mut event = Event::new()
            .priority(5)
            .summary("summary")
            .description("description")
            .location("location")
            .uid("uid")
            .class(Class::Private)
            .url("http://some.test/url")
            .done();
        assert_eq!(event.get_priority(), Some(5));
        assert_eq!(event.get_summary(), Some("summary"));
        assert_eq!(event.get_description(), Some("description"));
        assert_eq!(event.get_location(), Some("location"));
        assert_eq!(event.get_uid(), Some("uid"));
        assert_eq!(event.get_class(), Some(Class::Private));
        assert_eq!(event.get_url(), Some("http://some.test/url"));

        event
            .remove_priority()
            .remove_summary()
            .remove_description()
            .remove_location()
            .remove_class()
            .remove_url();
        assert_eq!(event.get_priority(), None);
        assert_eq!(event.get_summary(), None);
        assert_eq!(event.get_description(), None);
        assert_eq!(event.get_location(), None);
        assert_eq!(event.get_class(), None);
        assert_eq!(event.get_url(), None);
    }

    #[test]
    fn get_date_times_naive() {
        let naive_date_time = date(2001, 3, 13).at(14, 15, 16, 0);
        let event = Event::new()
            .starts(naive_date_time)
            .ends(naive_date_time)
            .done();
        assert_eq!(event.get_start(), Some(naive_date_time.into()));
        assert_eq!(event.get_end(), Some(naive_date_time.into()));
    }

    #[test]
    fn get_date_times_utc() {
        let utc_date_time = date(2001, 3, 13)
            .at(14, 15, 16, 0)
            .to_zoned_datetime(TimeZone::UTC)
            .unwrap()
            .timestamp();
        let event = Event::new()
            .timestamp(utc_date_time)
            .last_modified(utc_date_time)
            .created(utc_date_time)
            .starts(utc_date_time)
            .ends(utc_date_time)
            .done();
        assert_eq!(event.get_timestamp(), Some(utc_date_time));
        assert_eq!(event.get_last_modified(), Some(utc_date_time));
        assert_eq!(event.get_created(), Some(utc_date_time));
        assert_eq!(event.get_start(), Some(utc_date_time.into()));
        assert_eq!(event.get_end(), Some(utc_date_time.into()));
    }

    #[test]
    fn get_date_times_tzid() {
        let date_time = date(2001, 3, 13).at(14, 15, 16, 0);
        let date_time_tzid = CalendarDateTime::WithTimezone {
            date_time,
            tzid: "Pacific/Auckland".to_string(),
        };
        let event = Event::new()
            .starts(date_time_tzid.clone())
            .ends(date_time_tzid.clone())
            .done();
        assert_eq!(event.get_start(), Some(date_time_tzid.clone().into()));
        assert_eq!(event.get_end(), Some(date_time_tzid.into()));
    }

    #[test]
    fn get_dates_naive() {
        let naive_date = date(2001, 3, 13);
        let event = Event::new().starts(naive_date).ends(naive_date).done();
        assert_eq!(event.get_start(), Some(naive_date.into()));
        assert_eq!(event.get_end(), Some(naive_date.into()));
    }

    #[test]
    fn exdate_accepts_naive_date() {
        let naive_date = date(2001, 3, 13);
        let event = Event::new().exdate(naive_date).done();

        let exdates = event.multi_properties().get("EXDATE").unwrap();
        assert_eq!(exdates.len(), 1);
        // Must serialise with VALUE=DATE parameter and a date-only value
        let line = exdates.first().unwrap().to_line().unwrap();
        assert!(
            line.contains("VALUE=DATE"),
            "EXDATE should carry VALUE=DATE parameter, got: {line}"
        );
        assert_eq!(exdates.first().unwrap().value(), "20010313");
    }

    #[test]
    fn rdate_accepts_naive_date() {
        let naive_date = date(2001, 3, 13);
        let event = Event::new().rdate(naive_date).done();

        let rdates = event.multi_properties().get("RDATE").unwrap();
        assert_eq!(rdates.len(), 1);
        // Must serialise with VALUE=DATE parameter and a date-only value
        let line = rdates.first().unwrap().to_line().unwrap();
        assert!(
            line.contains("VALUE=DATE"),
            "RDATE should carry VALUE=DATE parameter, got: {line}"
        );
        assert_eq!(rdates.first().unwrap().value(), "20010313");
    }
}
