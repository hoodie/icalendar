use crate::Component;

#[cfg(feature = "rfc9073")]
use super::{Event, Location, Other, Participant, Resource, Todo, Venue};
#[cfg(not(feature = "rfc9073"))]
use super::{Event, Other, Todo, Venue};

use std::fmt;

/// Wrapper for [`Todo`], [`Event`] or [`Venue`]
#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CalendarComponent {
    Todo(Todo),
    Event(Event),
    Venue(Venue),
    #[cfg(feature = "rfc9073")]
    Participant(Participant),
    #[cfg(feature = "rfc9073")]
    Location(Location),
    #[cfg(feature = "rfc9073")]
    Resource(Resource),
    #[doc(hidden)]
    Other(Other),
}

impl CalendarComponent {
    /// Attempt to access the contained [`Event`], if it is one
    pub fn as_event(&self) -> Option<&Event> {
        match self {
            Self::Event(event) => Some(event),
            _ => None,
        }
    }

    /// Attempt to access the contained [`Todo`], if it is one
    pub fn as_todo(&self) -> Option<&Todo> {
        match self {
            Self::Todo(todo) => Some(todo),
            _ => None,
        }
    }

    /// Attempt to access the contained [`Participant`], if it is one
    #[cfg(feature = "rfc9073")]
    pub fn as_participant(&self) -> Option<&Participant> {
        match self {
            Self::Participant(ref participant) => Some(participant),
            _ => None,
        }
    }

    /// Attempt to access the contained [`Location`], if it is one
    #[cfg(feature = "rfc9073")]
    pub fn as_location(&self) -> Option<&Location> {
        match self {
            Self::Location(ref location) => Some(location),
            _ => None,
        }
    }

    /// Attempt to access the contained [`Resource`], if it is one
    #[cfg(feature = "rfc9073")]
    pub fn as_resource(&self) -> Option<&Resource> {
        match self {
            Self::Resource(ref resource) => Some(resource),
            _ => None,
        }
    }
}

impl From<Event> for CalendarComponent {
    fn from(val: Event) -> Self {
        CalendarComponent::Event(val)
    }
}

impl From<&Event> for CalendarComponent {
    fn from(val: &Event) -> Self {
        CalendarComponent::Event(val.to_owned())
    }
}

impl From<&mut Event> for CalendarComponent {
    fn from(val: &mut Event) -> Self {
        CalendarComponent::Event(val.to_owned())
    }
}

impl From<Todo> for CalendarComponent {
    fn from(val: Todo) -> Self {
        CalendarComponent::Todo(val)
    }
}

impl From<&Todo> for CalendarComponent {
    fn from(val: &Todo) -> Self {
        CalendarComponent::Todo(val.to_owned())
    }
}

impl From<&mut Todo> for CalendarComponent {
    fn from(val: &mut Todo) -> Self {
        CalendarComponent::Todo(val.to_owned())
    }
}

impl From<Venue> for CalendarComponent {
    fn from(val: Venue) -> Self {
        CalendarComponent::Venue(val)
    }
}

impl From<&Venue> for CalendarComponent {
    fn from(val: &Venue) -> Self {
        CalendarComponent::Venue(val.to_owned())
    }
}

impl From<&mut Venue> for CalendarComponent {
    fn from(val: &mut Venue) -> Self {
        CalendarComponent::Venue(val.to_owned())
    }
}

impl From<Other> for CalendarComponent {
    fn from(val: Other) -> Self {
        CalendarComponent::Other(val)
    }
}

impl From<&Other> for CalendarComponent {
    fn from(val: &Other) -> Self {
        CalendarComponent::Other(val.to_owned())
    }
}

impl From<&mut Other> for CalendarComponent {
    fn from(val: &mut Other) -> Self {
        CalendarComponent::Other(val.to_owned())
    }
}

#[cfg(feature = "rfc9073")]
impl From<Location> for CalendarComponent {
    fn from(val: Location) -> Self {
        CalendarComponent::Location(val)
    }
}

#[cfg(feature = "rfc9073")]
impl From<&Location> for CalendarComponent {
    fn from(val: &Location) -> Self {
        CalendarComponent::Location(val.to_owned())
    }
}

#[cfg(feature = "rfc9073")]
impl From<&mut Location> for CalendarComponent {
    fn from(val: &mut Location) -> Self {
        CalendarComponent::Location(val.to_owned())
    }
}

#[cfg(feature = "rfc9073")]
impl From<Resource> for CalendarComponent {
    fn from(val: Resource) -> Self {
        CalendarComponent::Resource(val)
    }
}

#[cfg(feature = "rfc9073")]
impl From<&Resource> for CalendarComponent {
    fn from(val: &Resource) -> Self {
        CalendarComponent::Resource(val.to_owned())
    }
}

#[cfg(feature = "rfc9073")]
impl From<&mut Resource> for CalendarComponent {
    fn from(val: &mut Resource) -> Self {
        CalendarComponent::Resource(val.to_owned())
    }
}

impl CalendarComponent {
    #[cfg(feature = "rfc9073")]
    pub(crate) fn fmt_write<W: fmt::Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        match *self {
            CalendarComponent::Todo(ref todo) => todo.fmt_write(out),
            CalendarComponent::Event(ref event) => event.fmt_write(out),
            CalendarComponent::Venue(ref venue) => venue.fmt_write(out),
            CalendarComponent::Participant(ref participant) => participant.fmt_write(out),
            CalendarComponent::Location(ref location) => location.fmt_write(out),
            CalendarComponent::Resource(ref resource) => resource.fmt_write(out),
            CalendarComponent::Other(ref other) => other.fmt_write(out),
        }
    }

    #[cfg(not(feature = "rfc9073"))]
    pub(crate) fn fmt_write<W: fmt::Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        match *self {
            CalendarComponent::Todo(ref todo) => todo.fmt_write(out),
            CalendarComponent::Event(ref event) => event.fmt_write(out),
            CalendarComponent::Venue(ref venue) => venue.fmt_write(out),
            CalendarComponent::Other(ref other) => other.fmt_write(out),
        }
    }
}
