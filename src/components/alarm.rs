use crate::components::duration::{format_duration, parse_duration, span_from_signed};
#[cfg(test)]
use jiff::ToSpan as _;
use jiff::{SignedDuration, Span, Timestamp};
use std::{fmt, str::FromStr};

pub use self::properties::{Related, Trigger};

use self::properties::*;
use super::*;

/// VALARM [(RFC 5545, Section 3.6.6 )](https://tools.ietf.org/html/rfc5545#section-3.6.6)
///
/// Alarms can be added to [`Event`]s and [`Todo`]s.
/// An alarm always has a **Trigger** and an **Action**.
/// The Trigger describes when the alarm should happen
/// and the Action describes what should happen.
///
/// ## Triggers
///
/// An alarm can either be triggered at an absolute time or in relation to its event,
/// so the Trigger can either be an absolute UTC [`Timestamp`] or a [`Span`] plus a
/// [relation tag](`Related`), which defines wether the duration is related to the events start or
/// end.
///
/// ## Action
///
/// When it is time for the Alarm to occur we have to define what is actually supposed to happen.
/// The RFC5545 know three different actions, two of which are currently implemented.
///
/// 1. Display
/// 2. Audio
/// 3. Email (not yet implemented)
///
/// ```rust
///  # use jiff::{Timestamp, ToSpan as _};
///  # use icalendar::*;
///
///  // alarm will occur one minute from now
///  let event_with_absolute_audio_alarm = Event::new()
///      .alarm(
///          Alarm::audio(Timestamp::now() + 1.minute())
///          .duration_and_repeat(1.minute(), 4)
///      )
///      .done();
///
///  // alarm will occur one minute before the start
///  let event_with_relative_display_alarm = Event::new()
///      .alarm(
///          Alarm::display("ALARM! ALARM!", -1.minute())
///          .duration_and_repeat(1.minute(), 4)
///      )
///      .done();
///
///  // alarm will occur one minute before the end
///  let event_with_relative_display_alarm_end = Event::new()
///      .alarm(
///          Alarm::display("ALARM! ALARM!", (-1.minute(), Related::End))
///          .duration_and_repeat(1.minute(), 4)
///      )
///      .done();
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Alarm {
    pub(crate) inner: InnerComponent,
}

impl Alarm {
    /// You are not supposed to create an empty Alarm by yourself since
    /// this would allow leaving out certain fields that are required,
    /// hench creating incompliant Alarms.
    pub(self) fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    /// Creates a new Audio-
    /// [Alarm Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.6)
    ///
    /// ## Definition
    /// 'action' and 'trigger' are both REQUIRED,
    ///  but MUST NOT occur more than once.
    ///
    /// `action / trigger /`
    ///
    ///  'duration' and 'repeat' are both OPTIONAL,
    ///  and MUST NOT occur more than once each;
    ///  but if one occurs, so MUST the other.
    ///
    /// `duration / repeat /`
    ///
    ///  The following is OPTIONAL,
    ///  but MUST NOT occur more than once.
    ///
    /// `attach /`
    ///
    ///  The following is OPTIONAL,
    ///  and MAY occur more than once.
    ///
    /// `x-prop / iana-prop`
    ///
    pub fn audio<T: Into<Trigger>>(trigger: T) -> Self {
        let trigger: Trigger = trigger.into();
        Alarm::default()
            .append_property(Action::Audio)
            .append_property(trigger)
            .done()
    }

    /// Creates a new Display-
    /// [Alarm Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.6)
    ///
    /// ## Definition
    ///  The following are REQUIRED,
    ///  but MUST NOT occur more than once.
    ///
    /// `action / description / trigger /`
    ///
    ///  'duration' and 'repeat' are both OPTIONAL,
    ///  and MUST NOT occur more than once each;
    ///  but if one occurs, so MUST the other.
    ///
    /// `duration / repeat /`
    ///
    ///  The following is OPTIONAL,
    ///  and MAY occur more than once.
    ///
    /// `x-prop / iana-prop`
    // TODO: make descript `impl ToString`
    pub fn display(description: &str, trigger: impl Into<Trigger>) -> Self {
        let trigger: Trigger = trigger.into();
        Alarm::default()
            .append_property(Action::Display)
            .append_property(trigger)
            .add_property("DESCRIPTION", description)
            .done()
    }

    /// Creates a new Email-
    ///
    /// [Alarm Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.6)
    ///
    /// TODO: this requires [Attendee support](https://www.rfc-editor.org/rfc/rfc5545#section-3.8.4.1)
    ///
    /// ## Definition
    ///
    ///  The following are all REQUIRED,
    ///  but MUST NOT occur more than once.
    ///
    /// `action / description / trigger / summary /`
    ///
    ///  The following is REQUIRED,
    ///  and MAY occur more than once.
    ///
    /// `attendee /`
    ///
    ///  'duration' and 'repeat' are both OPTIONAL,
    ///  and MUST NOT occur more than once each;
    ///  but if one occurs, so MUST the other.
    ///
    /// `duration / repeat /`
    ///
    ///  The following are OPTIONAL,
    ///  and MAY occur more than once.
    ///
    /// `attach / x-prop / iana-prop`
    #[allow(dead_code)]
    fn email(description: &str, trigger: impl Into<Trigger>, summary: &str) -> Self {
        let trigger: Trigger = trigger.into();
        Alarm::default()
            .append_property(Action::Email)
            .append_property(trigger)
            .add_property("DESCRIPTION", description)
            .add_property("SUMMARY", summary)
            .done()
    }

    /// Sets the
    /// [`REPEAT`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.2) and
    /// [`DURATION`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.5) property,
    /// which must not occur independent from one another.
    /// This infallible builder does not validate the interval or repeat count;
    /// use [`Alarm::try_duration_and_repeat`] for checked input.
    pub fn duration_and_repeat<R: Into<Repeat>>(mut self, duration: Span, repeat_count: R) -> Self {
        self.append_property(duration);
        self.append_property(repeat_count.into());
        self
    }

    /// Set `DURATION` and `REPEAT` if both values are valid.
    pub fn try_duration_and_repeat<R: Into<Repeat>, D: TryInto<Span>>(
        mut self,
        duration: D,
        repeat_count: R,
    ) -> Result<Self, String>
    where
        D::Error: fmt::Display,
    {
        let duration = duration.try_into().map_err(|error| error.to_string())?;
        format_duration(duration)?;
        if !duration.is_positive() {
            return Err("alarm repeat duration must be positive".into());
        }
        let repeat: Repeat = repeat_count.into();
        if repeat.is_zero() {
            return Err("alarm repeat count must be positive".into());
        }
        self.append_property(duration);
        self.append_property(repeat);
        Ok(self)
    }

    /// Returns the get action of this [`Alarm`].
    #[cfg(test)]
    pub(self) fn get_action(&self) -> Option<Action> {
        self.property_value("ACTION")
            .and_then(|p| Action::from_str(p).ok())
    }

    /// Returns the trigger of this [`Alarm`].
    #[cfg(test)]
    pub(self) fn get_trigger(&self) -> Option<Trigger> {
        self.inner
            .properties
            .get("TRIGGER")
            .and_then(|prop| Trigger::try_from(prop).ok())
    }

    /// Returns the description of this [`Alarm`].
    #[cfg(test)]
    pub(self) fn get_description(&self) -> Option<&str> {
        self.inner.property_value("DESCRIPTION")
    }

    /// Returns the repeat count of this [`Alarm`].
    #[cfg(test)]
    pub(self) fn get_repeat(&self) -> usize {
        self.inner
            .property_value("REPEAT")
            .and_then(|repeat| repeat.parse().ok())
            .unwrap_or(0)
    }

    /// End of builder pattern.
    /// copies over everything
    pub fn done(&mut self) -> Self {
        Alarm {
            inner: self.inner.done(),
        }
    }

    //pub fn repeats<R:Repeater+?Sized>(&mut self, repeat: R) -> &mut Self {
    //    unimplemented!()
    //}
}

#[test]
fn test_audio() {
    let alarm = Alarm::audio((15.minutes(), Related::Start))
        .duration_and_repeat(5.minutes(), 3)
        .done();
    assert_eq!(alarm.get_action(), Some(Action::Audio));
    assert_eq!(
        alarm.get_trigger(),
        Some(Trigger::Duration(15.minutes(), Related::Start.into()))
    );
    assert_eq!(
        alarm
            .get_trigger()
            .unwrap()
            .as_duration()
            .map(|span| span.fieldwise()),
        Some(15.minutes().fieldwise())
    );
    assert_eq!(alarm.get_trigger().unwrap().related(), Some(Related::Start));
    assert_eq!(alarm.get_repeat(), 3);
    alarm.print().unwrap();
}

#[test]
fn test_display() {
    let now = CalendarDateTime::now().try_into_utc().unwrap();

    let alarm = Alarm::display("test alarm with display", now);
    assert_eq!(alarm.get_action(), Some(Action::Display));
    assert_eq!(alarm.get_trigger().unwrap().as_date_time().unwrap(), &now);
    assert_eq!(alarm.get_description(), Some("test alarm with display"));
}

#[test]
#[ignore]
fn test_email() {
    let now = CalendarDateTime::now().try_into_utc().unwrap();

    let alarm = Alarm::email("test alarm with email", now, "important email");
    assert_eq!(alarm.get_action(), Some(Action::Email));
    assert_eq!(alarm.get_trigger().unwrap().as_date_time().unwrap(), &now);
    assert_eq!(alarm.get_description(), Some("test alarm with email"));
    assert_eq!(alarm.get_summary(), Some("important email"));
    todo!("add attendee handling");
    // assert_eq!(alarm.get_attendees(), Vec(todo!()));
}

pub mod properties {

    use super::*;
    #[cfg(test)]
    use jiff::civil::date;

    /// [rfc5545#section-3.8.6.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.1)
    #[derive(Debug, PartialEq, Eq)]
    pub(crate) enum Action {
        /// [rfc5545#section-3.8.6.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.1)
        Audio,
        /// [rfc5545#section-3.8.6.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.1)
        Email,
        /// [rfc5545#section-3.8.6.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.1)
        Display,
        // /// [rfc5545#section-3.8.6.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.1)
        // IanaToken(String),
        // /// [rfc5545#section-3.8.6.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.1)
        // XName{vendor: String, name: String},
        /// what ever else
        Other(String),
    }

    impl FromStr for Action {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "AUDIO" => Action::Audio,
                "EMAIL" => Action::Email,
                "DISPLAY" => Action::Display,
                other => Action::Other(other.into()),
            })
        }
    }

    impl fmt::Display for Action {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Action::Audio => write!(f, "AUDIO"),
                Action::Email => write!(f, "EMAIL"),
                Action::Display => write!(f, "DISPLAY"),
                Action::Other(other) => write!(f, "{other}"),
            }
        }
    }

    impl From<Action> for Property {
        fn from(action: Action) -> Self {
            Property {
                key: String::from("ACTION"),
                val: action.to_string(),
                params: Default::default(),
            }
        }
    }

    /// [rfc5545#section-3.8.6.2](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.2)
    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
    pub struct Repeat(
        u32, // technically a signed integer according to spec
    );

    impl From<u32> for Repeat {
        fn from(count: u32) -> Self {
            Repeat(count)
        }
    }

    impl Repeat {
        pub(super) fn is_zero(self) -> bool {
            self.0 == 0
        }
    }

    impl From<Repeat> for Property {
        fn from(r: Repeat) -> Self {
            Property::new("REPEAT", r.0.to_string())
        }
    }

    impl FromStr for Repeat {
        type Err = ();

        fn from_str(_: &str) -> Result<Self, Self::Err> {
            todo!()
        }
    }

    /// Alarm Trigger Relationship [RFC 5545, Section 3.2.14](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.14)
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum Related {
        /// the parameter value START will set the alarm to trigger off the start of the calendar component
        Start,
        /// the parameter value END will set the alarm to trigger off the end of the calendar component
        End,
    }

    impl From<Related> for Parameter {
        fn from(related: Related) -> Self {
            match related {
                Related::Start => Parameter::new("RELATED", "START"),
                Related::End => Parameter::new("RELATED", "END"),
            }
        }
    }

    impl FromStr for Related {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "START" => Ok(Related::Start),
                "END" => Ok(Related::End),
                _ => Err(()),
            }
        }
    }

    #[test]
    fn test_repeat_default() {
        assert_eq!(
            Property::from(Repeat::default()).try_into(),
            Ok(String::from("REPEAT:0\r\n"))
        )
    }

    /// Describes when an alarm is supposed to occure.
    ///
    /// [RFC 5545, Section 3.8.6.3](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.3),
    /// see also [Alarm Trigger Relationship](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.14)
    /// This property specifies when an alarm will trigger.
    #[derive(Clone, Copy, Debug)]
    pub enum Trigger {
        /// Duration in relation to either Start or End of the event
        Duration(Span, Option<Related>),
        /// Absolute UTC instant of the trigger
        DateTime(Timestamp),
    }

    impl PartialEq for Trigger {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Duration(left, lrel), Self::Duration(right, rrel)) => {
                    left.fieldwise() == right.fieldwise() && lrel == rrel
                }
                (Self::DateTime(left), Self::DateTime(right)) => left == right,
                _ => false,
            }
        }
    }

    impl Eq for Trigger {}

    impl Trigger {
        /// ```
        /// # use icalendar::*;
        /// # use jiff::ToSpan as _;
        /// assert_eq!(
        ///     Trigger::after_start(1.hour()),
        ///     Trigger::Duration(1.hour(), Some(Related::Start))
        /// )
        /// ```
        /// please don't supply negative durations, you'll just confuse everybody
        pub fn after_start(duration: Span) -> Trigger {
            Trigger::Duration(duration, Some(Related::Start))
        }

        /// ```
        /// # use icalendar::*;
        /// # use jiff::ToSpan as _;
        /// assert_eq!(
        ///     Trigger::after_end(1.hour()),
        ///     Trigger::Duration(1.hour(), Some(Related::End))
        /// )
        /// ```
        /// please don't supply negative durations, you'll just confuse everybody
        pub fn after_end(duration: Span) -> Trigger {
            Trigger::Duration(duration, Some(Related::End))
        }

        /// ```
        /// # use icalendar::*;
        /// # use jiff::ToSpan as _;
        /// assert_eq!(
        ///     Trigger::before_start(1.hour()),
        ///     Trigger::Duration(-1.hour(), Some(Related::Start))
        /// )
        /// ```
        /// please don't supply negative durations, you'll just confuse everybody
        pub fn before_start(duration: Span) -> Trigger {
            Trigger::Duration(-duration, Some(Related::Start))
        }

        /// ```
        /// # use icalendar::*;
        /// # use jiff::ToSpan as _;
        /// assert_eq!(
        ///     Trigger::before_end(1.hour()),
        ///     Trigger::Duration(-1.hour(), Some(Related::End))
        /// )
        /// ```
        /// please don't supply negative durations, you'll just confuse everybody
        pub fn before_end(duration: Span) -> Trigger {
            Trigger::Duration(-duration, Some(Related::End))
        }

        /// Returns the containing [`Related`] if the [`Trigger`] contains one
        pub fn related(&self) -> Option<Related> {
            match self {
                Trigger::Duration(_, related) => *related,
                Trigger::DateTime(_) => None,
            }
        }

        /// Returns the containing [`Span`] if the [`Trigger`] contains one
        pub fn as_duration(&self) -> Option<&Span> {
            match self {
                Trigger::Duration(duration, _) => Some(duration),
                Trigger::DateTime(_) => None,
            }
        }

        /// Returns the absolute instant, if this trigger contains one
        pub fn as_date_time(&self) -> Option<&Timestamp> {
            match self {
                Trigger::Duration(..) => None,
                Trigger::DateTime(dt) => Some(dt),
            }
        }
    }

    impl From<Span> for Trigger {
        fn from(duration: Span) -> Self {
            Trigger::Duration(duration, None)
        }
    }

    impl TryFrom<SignedDuration> for Trigger {
        type Error = String;

        fn try_from(duration: SignedDuration) -> Result<Self, Self::Error> {
            Ok(Trigger::Duration(span_from_signed(duration)?, None))
        }
    }

    impl From<Timestamp> for Trigger {
        fn from(dt: Timestamp) -> Self {
            Trigger::DateTime(dt)
        }
    }

    impl From<&Timestamp> for Trigger {
        fn from(dt: &Timestamp) -> Self {
            Trigger::DateTime(*dt)
        }
    }

    impl From<(Span, Related)> for Trigger {
        fn from((duration, related): (Span, Related)) -> Self {
            Trigger::Duration(duration, Some(related))
        }
    }

    impl TryFrom<(SignedDuration, Related)> for Trigger {
        type Error = String;

        fn try_from((duration, related): (SignedDuration, Related)) -> Result<Self, Self::Error> {
            Ok(Trigger::Duration(
                span_from_signed(duration)?,
                Some(related),
            ))
        }
    }

    impl TryFrom<&Property> for Trigger {
        type Error = ();
        fn try_from(prop: &Property) -> Result<Self, Self::Error> {
            match (prop.key(), prop.params().get("VALUE").map(Parameter::value)) {
                ("TRIGGER", Some("DATE-TIME")) => {
                    // date-time needs to be qualified "VALUE=DATE-TIME"
                    parse_utc_date_time(prop.value())
                        .map(Trigger::DateTime)
                        .ok_or(())
                }
                ("TRIGGER", Some("DURATION") | None) => {
                    // duration is the assumed default or "VALUE=DURATION"
                    let param_related = prop.get_param_as("RELATED", |s| Related::from_str(s).ok());

                    // TODO: improve error handling here
                    // TODO: yes I found icalendar-duration, let's find a way to integrate this if possible
                    let parsed_duration = prop.get_value_as(parse_duration);

                    if let Some(duration) = parsed_duration {
                        Ok(Trigger::Duration(duration, param_related))
                    } else {
                        Err(())
                    }
                }
                _ => Err(()),
            }
        }
    }

    impl From<Trigger> for Property {
        fn from(trigger: Trigger) -> Self {
            match trigger {
                Trigger::Duration(duration, Some(related)) => Property::new(
                    "TRIGGER",
                    format_duration(duration).unwrap_or_else(|_| duration.to_string()),
                )
                .append_parameter(related)
                .done(),

                Trigger::Duration(duration, None) => Property::new(
                    "TRIGGER",
                    format_duration(duration).unwrap_or_else(|_| duration.to_string()),
                ),

                Trigger::DateTime(dt) => Property::new("TRIGGER", format_utc_date_time(dt))
                    .add_parameter("VALUE", "DATE-TIME")
                    .done(),
            }
        }
    }

    #[test]
    fn unchecked_alarm_durations_do_not_panic() {
        let alarm = Alarm::audio(1.month()).duration_and_repeat(1.month(), 0);
        assert_eq!(alarm.property_value("TRIGGER"), Some("P1M"));
        assert_eq!(alarm.property_value("DURATION"), Some("P1M"));
        assert_eq!(alarm.property_value("REPEAT"), Some("0"));
    }

    #[test]
    fn reject_invalid_repeats_and_triggers() {
        assert!(
            Alarm::audio(-1.minute())
                .try_duration_and_repeat(0.seconds(), 1)
                .is_err()
        );
        assert!(
            Alarm::audio(-1.minute())
                .try_duration_and_repeat(1.month(), 1)
                .is_err()
        );
        assert!(
            Alarm::audio(-1.minute())
                .try_duration_and_repeat(1.minute(), 0)
                .is_err()
        );
        assert!(Trigger::try_from(&Property::new("TRIGGER", "P1W2D")).is_err());
        assert!(Trigger::try_from(&Property::new("TRIGGER", "1 hour")).is_err());
        assert!(
            Trigger::try_from(
                &Property::new("TRIGGER", "20260315T184500")
                    .add_parameter("VALUE", "DATE-TIME")
                    .done()
            )
            .is_err()
        );
    }

    #[test]
    fn test_trigger() {
        let prop: Property = Trigger::from(14.weeks()).into();
        let mut out = String::new();
        prop.fmt_write(&mut out).unwrap();
    }

    /// adding triggers to Alarm component serializes them and so we need to re-parse to read them
    /// I know this sucks, but let's do the refactoring of the internal representation elsewhere
    #[test]
    fn test_trigger_abs_from_str() {
        let now: Timestamp = date(2022, 11, 17)
            .at(21, 32, 45, 0)
            .to_zoned_datetime(jiff::tz::TimeZone::UTC)
            .unwrap()
            .timestamp();

        let property: Property = Trigger::from(now).into();
        assert_eq!(property.value(), "20221117T213245Z");
        assert_eq!(
            property.params().get("VALUE").map(Parameter::value),
            Some("DATE-TIME")
        );

        let alarm_with_abs_trigger = Alarm::default().append_property(Trigger::from(now)).done();

        alarm_with_abs_trigger.print().unwrap();

        pretty_assertions::assert_eq!(
            alarm_with_abs_trigger.get_trigger(),
            Some(Trigger::DateTime(now))
        );
    }

    #[test]
    fn test_trigger_abs_from_str_naive() {
        let now: Timestamp = date(2022, 11, 17)
            .at(21, 32, 45, 0)
            .to_zoned_datetime(jiff::tz::TimeZone::UTC)
            .unwrap()
            .timestamp();

        let alarm_with_abs_trigger = Alarm::default().append_property(Trigger::from(&now)).done();

        alarm_with_abs_trigger.print().unwrap();

        pretty_assertions::assert_eq!(
            alarm_with_abs_trigger.get_trigger(),
            Some(Trigger::DateTime(now))
        );
    }

    #[test]
    fn test_trigger_dur_from_str() {
        let dur = 15.minutes();

        let alarm_with_rel_trigger = Alarm::default().append_property(Trigger::from(dur)).done();
        alarm_with_rel_trigger.print().unwrap();

        pretty_assertions::assert_eq!(
            alarm_with_rel_trigger.get_trigger(),
            Some(Trigger::Duration(dur, None))
        );
    }

    #[test]
    fn test_trigger_dur_from_str_start() {
        let dur = 15.minutes();
        let alarm_with_rel_start_trigger = Alarm::default()
            .append_property(Trigger::from((dur, Related::Start)))
            .done();

        alarm_with_rel_start_trigger.print().unwrap();
        pretty_assertions::assert_eq!(
            alarm_with_rel_start_trigger.get_trigger(),
            Some(Trigger::Duration(dur, Some(Related::Start)))
        );
    }
}
