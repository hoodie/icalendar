#![doc = include_str!("../README.md")]
#![allow(deprecated)]
#![warn(
    missing_docs,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    missing_debug_implementations,
    clippy::indexing_slicing,
    clippy::dbg_macro,
    clippy::doc_markdown,
    clippy::redundant_closure_for_method_calls
)]

macro_rules! print_crlf {
    () => (print!("\r\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\r\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\r\n"), $($arg)*));
}

macro_rules! write_crlf {
    ($dst:expr) => (
        write!($dst, "\r\n")
    );
    ($dst:expr, $fmt:expr) => (
        write!($dst, concat!($fmt, "\r\n"))
    );
    ($dst:expr, $fmt:expr, $($arg:tt)*) => (
        write!($dst, concat!($fmt, "\r\n"), $($arg)*)
    );
}

#[cfg(all(test, feature = "parser"))]
#[macro_use]
mod assert;

//pub mod period;
mod calendar;
mod components;
#[cfg(feature = "parser")]
pub mod parser;
mod properties;
mod value_types;

pub use crate::{
    calendar::{Calendar, CalendarComponent},
    components::{
        alarm::{Alarm, Related, Trigger},
        date_time::{CalendarDateTime, DatePerhapsTime},
        Component, Event, EventLike, Other, Todo, Venue,
    },
    properties::{Class, EventStatus, Parameter, Property, TodoStatus},
    value_types::ValueType,
};

#[cfg(feature = "chrono-tz")]
pub use crate::components::date_time::ymd_hm_tzid;

// TODO Calendar TimeZone VTIMEZONE STANDARD DAYLIGHT (see thunderbird exports)
