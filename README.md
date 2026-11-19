<div align="center">

# iCalendar in Rust

[![build](https://img.shields.io/github/actions/workflow/status/hoodie/icalendar/ci.yml?branch=main)](https://github.com/hoodie/icalendar/actions?query=workflow%3A"Continuous+Integration")
[![Crates.io](https://img.shields.io/crates/d/icalendar)](https://crates.io/crates/icalendar)
[![contributors](https://img.shields.io/github/contributors/hoodie/icalendar)](https://github.com/hoodie/icalendar/graphs/contributors)
![maintenance](https://img.shields.io/maintenance/yes/2027)

[![version](https://img.shields.io/crates/v/icalendar)](https://crates.io/crates/icalendar/)
[![documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/icalendar/)
[![license](https://img.shields.io/crates/l/icalendar.svg?style=flat)](https://crates.io/crates/icalendar/)

A builder and parser for [`rfc5545`](http://tools.ietf.org/html/rfc5545) iCalendar.

</div>

You want to help make this more mature? Please talk to me, Pull Requests and suggestions are very welcome.

## Usage & Documentation

Please see the [documentation](https://docs.rs/icalendar/) for current examples.

Below are a few examples of how to use this library. See the `examples` directory as well as the documentation for many more.

### Building a new Calendar

Use the builder-pattern to assemble the full calendar or event by event.
Display printing produces the rfc5545 format.

```rust
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use icalendar::{Calendar, Class, Component, Event, EventLike, Property, Todo};
use jiff::{Timestamp, ToSpan as _, civil::date};

let now = Timestamp::now();

// let's create a calendar
let my_calendar = Calendar::new()
    .name("example calendar")
    .push(
        // add an event
        Event::new()
            .summary("test event")
            .description("here I have something really important to do")
            .starts(now)
            .class(Class::Confidential)
            .ends(now + 24.hours())
            .append_property(
                Property::new("TEST", "FOOBAR")
                    .add_parameter("IMPORTANCE", "very")
                    .add_parameter("DUE", "tomorrow")
            )
    )
    .push(
        // add a todo
        Todo::new()
            .summary("groceries")
            .description("Buy some milk")
    )
    .push(
        // add an all-day event
        Event::new()
            .all_day(date(2016, 3, 15))
            .summary("My Birthday")
            .description("Hey, I'm gonna have a party\nBYOB: Bring your own beer.\nHendrik")
    )
    .push(
        // event with utc timezone
        Event::new()
            .starts(date(2024, 10, 24).at(20, 10, 0, 0).in_tz("UTC")?.timestamp())
            .summary("Birthday Party")
            .description("I'm gonna have a party\nBYOB: Bring your own beer.\nHendrik")
    )
    .done();

println!("{}", my_calendar);
Ok(())
# }
```

### Removing Properties from Components

You can remove properties from components when you need to update their state:

```rust
use icalendar::{Calendar, Component, Event, EventStatus, Todo, TodoStatus};
use jiff::Timestamp;

// Create a completed todo
let mut todo = Todo::new();
todo
    .summary("Buy milk")
    .completed(Timestamp::now())
    .percent_complete(100)
    .status(TodoStatus::Completed);

// Later, mark it as uncompleted by removing completion properties
todo.mark_uncompleted();
// This removes COMPLETED, PERCENT-COMPLETE, and STATUS properties

// For events, you can remove specific properties
let mut event = Event::new();
event
    .summary("Team Meeting")
    .status(EventStatus::Cancelled);

// Remove the cancelled status
event.remove_status();

// You can also remove any property directly
event.remove_property("LOCATION");
```

### Parsing a Calendar

There is a feature called `"parser"` which allows you to read calendars again like this:

```rust
use std::fs::read_to_string;

use icalendar::{Calendar, CalendarComponent, Component};

let contents = read_to_string("fixtures/icalendar-rb/event.ics").unwrap();

let parsed_calendar: Calendar = contents.parse().unwrap();

for event in parsed_calendar.events() {
    println!("Event: {}", event.get_summary().unwrap())
}

```

## Structure

A [`Calendar`] represents a full calendar, which contains multiple [`Component`]s. These may be either [`Event`]s, [`Todo`]s, or [`Venue`]s. Components in turn have [`Property`]s, which may have [`Parameter`]s.

## Recurrence properties

Calendars can still parse and emit `RRULE`, `RDATE`, `EXDATE`, and `RECURRENCE-ID` properties. Use `add_property("RRULE", "FREQ=DAILY;COUNT=4")` for a raw rule, and `rdate()`, `exdate()`, and `recurrence_id()` for the other values. Typed rule construction and occurrence expansion are not available; use an external engine if you need expansion.

## Migrating to Jiff

Add `jiff = "1"` to your application: this crate does not re-export Jiff. Pass `civil::Date` for `VALUE=DATE`, `civil::DateTime` for a floating local time, `Timestamp` for UTC, or a named `ZonedDateTime` for a local time with `TZID`. For example, `date(2026, 3, 15).at(18, 45, 0, 0).in_tz("Europe/Berlin")?` can be passed to `starts()`. Parsed custom `TZID` values are retained even if they cannot be resolved locally.

Use `CalendarDateTime::to_zoned_datetime()` for a known `TZID`, or `DatePerhapsTime::to_zoned_datetime(&zone)` for floating and date-only values. `Calendar::calendar_events()` and `calendar_todos()` provide resolved accessors using `X-WR-TIMEZONE` when those values need a calendar zone. Resolution defaults to Jiff's `Compatible` gap/fold policy; pass `Disambiguation::Reject` to a `_with` variant to reject ambiguity. A floating value remains floating on the wire even when the calendar has `X-WR-TIMEZONE`.

The migration table, breaking changes, and current limitations are in the [Unreleased changelog](CHANGELOG.md#unreleased). In particular, `all_day()` does not yet produce an exclusive end date, and resolved event `end()` does not yet calculate `DTSTART + DURATION`.

Browser WASM applications using Jiff's clock or system timezone should enable `jiff/js` in their own dependency. This crate does not enable it by default. Named-zone lookup also depends on available timezone data; check the target runtime, especially on minimal containers, Windows and WASM.

## Contributing

Any help in form of descriptive and friendly [issues](https://github.com/hoodie/icalendar/issues) or comprehensive pull requests are welcome!

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in icalendar by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

### Conventions

The Changelog of this library is generated from its commit log, there any commit message must conform with <https://www.conventionalcommits.org/en/v1.0.0/>. For simplicity you could make your commits with [convco](https://crates.io/crates/convco).

### Recognition

Thanks goes to these wonderful people:

 <a href="https://github.com/hoodie/icalendar/graphs/contributors">
   <img src="https://contrib.rocks/image?repo=hoodie/icalendar" />
 </a>

### License

<sup>
`icalendar` (this crate) is licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
