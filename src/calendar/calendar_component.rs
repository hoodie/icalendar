use crate::Component;

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
    #[doc(hidden)]
    Other(Other),
}

impl CalendarComponent {
    /// Attempt to access the containted [`Event`], if it is one
    pub fn as_event(&self) -> Option<&Event> {
        match self {
            Self::Event(event) => Some(event),
            _ => None,
        }
    }
    /// Attempt to access the containted [`Todo`], if it is one
    pub fn as_todo(&self) -> Option<&Todo> {
        match self {
            Self::Todo(todo) => Some(todo),
            _ => None,
        }
    }
}

impl From<Event> for CalendarComponent {
    fn from(val: Event) -> Self {
        CalendarComponent::Event(val)
    }
}

impl From<Todo> for CalendarComponent {
    fn from(val: Todo) -> Self {
        CalendarComponent::Todo(val)
    }
}

impl From<Venue> for CalendarComponent {
    fn from(val: Venue) -> Self {
        CalendarComponent::Venue(val)
    }
}

impl From<Other> for CalendarComponent {
    fn from(val: Other) -> Self {
        CalendarComponent::Other(val)
    }
}

impl CalendarComponent {
    pub(crate) fn fmt_write<W: fmt::Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        match *self {
            CalendarComponent::Todo(ref todo) => todo.fmt_write(out),
            CalendarComponent::Event(ref event) => event.fmt_write(out),
            CalendarComponent::Venue(ref venue) => venue.fmt_write(out),
            CalendarComponent::Other(ref other) => other.fmt_write(out),
        }
    }
}
