//! # Parsing iCalendar document parser
//!
//! I would have loved to provide a zero-copy parser here however the *Internet Calendaring and Scheduling Core Object Specification (iCalendar)*
//! allows, nay demands special line folding:
//! >    Lines of text SHOULD NOT be longer than 75 octets, excluding the line break.  Long content lines SHOULD be split into a multiple line representations using a line "folding" technique.
//! > -- [rfc5545 3.1]
//!
//! For this reason parsing iCal is a bit indirect.
//! In this module you find the following functions to parser iCalendar document.
//! [`unfold()`] will unfold the iCal content and turn it into the nice machine-readable format it ought to be.
//! [`read_calendar_simple()`] returns a Vector of [`Component`]s
//! [`read_calendar()`] does the same thing but produces nicer parsing errors with line numbers (referencing the normalized content).
//!
//! You don't have to use `normalize()` on your document if your calendar does not obey the folding rules specified in [rfc5545 3.1].
//! If it unexpectedly does, the errors might be a tad confusing.
//!
//!
//! [rfc5545 3.1]: https://datatracker.ietf.org/doc/html/rfc5545#section-3.1
//!
//! A Calendar is always a tree of [`Component`]s.
//! It may contain multiple root elements so we have a `Vec<Component>` and each `Component` may more child `Component`s.
//! Each [`Component`] has properties, so a [`Vec`] of [`Property`] and those have of [`Parameter`]s.
//!
//!
#![allow(missing_docs)]
use nom::Finish;

mod calendar;
pub(crate) mod components;
mod parameters;
mod parsed_string;
mod properties;
#[cfg(test)]
mod tests;
mod utils;

pub use calendar::Calendar;
pub use components::Component;
use nom_language::error::{VerboseError, convert_error};
pub use parameters::Parameter;
pub use parsed_string::ParseString;
pub use properties::Property;

use components::*;

pub use utils::unfold;

/// Parse iCalendar file content into an array of [`Component`]s
///
/// This version produces very simple Errors for simplicity's sake.
pub fn read_calendar_simple(input: &str) -> Result<Vec<Component<'_>>, nom::error::Error<&str>> {
    components(input).finish().map(|(_, components)| components)
}

/// Parse iCalendar file content into an array of [`Component`]s
///
/// This version produces nice and readable errors with line numbers thanks the the awesomeness of [`nom`].
/// Line numbers are in regard to the normalized/unfolded version of the input, so better keep those around for reference.
///
pub fn read_calendar(input: &str) -> Result<Calendar<'_>, String> {
    components(input)
        .finish()
        .map(|(_, mut components)| {
            let root_is_calendar = components
                .first()
                .map(|first_root| first_root.name == "VCALENDAR")
                .unwrap_or(false);

            if root_is_calendar {
                let root = components.swap_remove(0);
                Calendar {
                    properties: root.properties,
                    components: root.components,
                }
            } else {
                Calendar {
                    components,
                    properties: Vec::new(),
                }
            }
        })
        .map_err(|e: VerboseError<&str>| format!("error: {}", convert_error(input, e.clone())))
}

#[test]
fn begin_crash() {
    assert!(read_calendar("BEGIN:").is_ok());
}

/// Parse iCalendar file content into an array of [`Component`]s
///
/// This version produces nice and readable errors with line numbers thanks the the awesomeness of [`nom`].
/// Line numbers are in regard to the normalized/unfolded version of the input, so better keep those around for reference.
///
pub fn read_components(input: &str) -> Result<Vec<Component<'_>>, String> {
    components(input)
        .finish()
        .map(|(_, components)| components)
        .map_err(|e: VerboseError<&str>| format!("error: {}", convert_error(input, e.clone())))
}
