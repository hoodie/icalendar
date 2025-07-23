use chrono::Duration;
use std::{collections::HashMap, fmt::Debug, str::FromStr};

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
/// so the Trigger can either be an absolute [`CalendarDateTime`] or a [`Duration`] plus a
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
///  # use chrono::*;
///  # use icalendar::*;
///
///  // alarm will occur one minute from now
///  let event_with_absolute_audio_alarm = Event::new()
///      .alarm(
///          Alarm::audio(Utc::now() + Duration::minutes(1))
///          .duration_and_repeat(Duration::minutes(1), 4)
///      )
///      .done();
///
///  // alarm will occur one minute before the start
///  let event_with_relative_display_alarm = Event::new()
///      .alarm(
///          Alarm::display("ALARM! ALARM!", -Duration::minutes(1))
///          .duration_and_repeat(Duration::minutes(1), 4)
///      )
///      .done();
///
///  // alarm will occur one minute before the end
///  let event_with_relative_display_alarm_end = Event::new()
///      .alarm(
///          Alarm::display("ALARM! ALARM!", (-Duration::minutes(1), Related::End))
///          .duration_and_repeat(Duration::minutes(1), 4)
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

    /// Sets duration the
    /// [`REPEAT`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.2) and
    /// [`DURATION`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.5) property,
    /// which must not occur independent from one another
    pub fn duration_and_repeat<R: Copy + Clone + Into<Repeat>>(
        mut self,
        duration: Duration,
        repeat_count: R,
    ) -> Self {
        // self.add_property("ACTION", action.as_str());
        self.append_property(duration);

        let repeat: Repeat = repeat_count.into();
        self.append_property(repeat);
        self
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
    let alarm = Alarm::audio((Duration::minutes(15), Related::Start))
        .duration_and_repeat(Duration::minutes(5), 3)
        .done();
    assert_eq!(alarm.get_action(), Some(Action::Audio));
    assert_eq!(
        alarm.get_trigger(),
        Some(Trigger::Duration(
            Duration::minutes(15),
            Related::Start.into()
        ))
    );
    assert_eq!(
        alarm.get_trigger().unwrap().as_duration(),
        Some(&Duration::minutes(15))
    );
    assert_eq!(alarm.get_trigger().unwrap().related(), Some(Related::Start));
    assert_eq!(alarm.get_repeat(), 3);
    alarm.print().unwrap();
}

#[test]
fn test_display() {
    let now = CalendarDateTime::now();

    let alarm = Alarm::display("test alarm with display", now.clone());
    assert_eq!(alarm.get_action(), Some(Action::Display));
    assert_eq!(alarm.get_trigger().unwrap().as_date_time().unwrap(), &now);
    assert_eq!(alarm.get_description(), Some("test alarm with display"));
}

#[test]
#[ignore]
fn test_email() {
    let now = CalendarDateTime::now();

    let alarm = Alarm::email("test alarm with email", now.clone(), "important email");
    assert_eq!(alarm.get_action(), Some(Action::Email));
    assert_eq!(alarm.get_trigger().unwrap().as_date_time().unwrap(), &now);
    assert_eq!(alarm.get_description(), Some("test alarm with email"));
    assert_eq!(alarm.get_summary(), Some("important email"));
    todo!("add attendee handling");
    // assert_eq!(alarm.get_attendees(), Vec(todo!()));
}

pub mod properties {

    use crate::components::date_time::parse_duration;

    use super::*;

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
                params: HashMap::new(),
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
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum Trigger {
        /// Duration in relation to either Start or End of the event
        Duration(/*std::time::*/Duration, Option<Related>),
        /// Absolute `DateTime` of the Trigger
        DateTime(CalendarDateTime),
    }

    impl Trigger {
        /// ```
        /// # use icalendar::*;
        /// # use chrono::*;
        /// assert_eq!(
        ///     Trigger::after_start(Duration::hours(1)),
        ///     Trigger::Duration(Duration::hours(1), Some(Related::Start))
        /// )
        /// ```
        /// please don't supply negative durations, you'll just confuse everybody
        pub fn after_start(duration: Duration) -> Trigger {
            Trigger::Duration(duration, Some(Related::Start))
        }

        /// ```
        /// # use icalendar::*;
        /// # use chrono::*;
        /// assert_eq!(
        ///     Trigger::after_end(Duration::hours(1)),
        ///     Trigger::Duration(Duration::hours(1), Some(Related::End))
        /// )
        /// ```
        /// please don't supply negative durations, you'll just confuse everybody
        pub fn after_end(duration: Duration) -> Trigger {
            Trigger::Duration(duration, Some(Related::End))
        }

        /// ```
        /// # use icalendar::*;
        /// # use chrono::*;
        /// assert_eq!(
        ///     Trigger::before_start(Duration::hours(1)),
        ///     Trigger::Duration(-Duration::hours(1), Some(Related::Start))
        /// )
        /// ```
        /// please don't supply negative durations, you'll just confuse everybody
        pub fn before_start(duration: Duration) -> Trigger {
            Trigger::Duration(-duration, Some(Related::Start))
        }

        /// ```
        /// # use icalendar::*;
        /// # use chrono::*;
        /// assert_eq!(
        ///     Trigger::before_end(Duration::hours(1)),
        ///     Trigger::Duration(-Duration::hours(1), Some(Related::End))
        /// )
        /// ```
        /// please don't supply negative durations, you'll just confuse everybody
        pub fn before_end(duration: Duration) -> Trigger {
            Trigger::Duration(-duration, Some(Related::End))
        }

        /// Returns the containing [`Related`] if the [`Trigger`] contains one
        pub fn related(&self) -> Option<Related> {
            match self {
                Trigger::Duration(_, related) => *related,
                Trigger::DateTime(_) => None,
            }
        }

        /// Returns the containing [`Duration`] if the [`Trigger`] contains one
        pub fn as_duration(&self) -> Option<&Duration> {
            match self {
                Trigger::Duration(duration, _) => Some(duration),
                Trigger::DateTime(_) => None,
            }
        }

        /// Returns the containing [`CalendarDateTime`] if the [`Trigger`] contains one
        pub fn as_date_time(&self) -> Option<&CalendarDateTime> {
            match self {
                Trigger::Duration(..) => None,
                Trigger::DateTime(dt) => Some(dt),
            }
        }
    }

    impl From<Duration> for Trigger {
        fn from(duration: Duration) -> Self {
            Trigger::Duration(duration.into(), None)
        }
    }

    impl From<jiff::Span> for Trigger {
        fn from(duration: jiff::Span) -> Self {
            Trigger::Duration(duration, None)
        }
    }

    impl<T> From<T> for Trigger
    where
        CalendarDateTime: From<T>,
    {
        fn from(dt: T) -> Self {
            Trigger::DateTime(CalendarDateTime::from(dt))
        }
    }

    impl From<(Duration, Related)> for Trigger {
        fn from((duration, related): (Duration, Related)) -> Self {
            Trigger::Duration(duration, Some(related))
        }
    }

    impl TryFrom<&Property> for Trigger {
        type Error = ();
        fn try_from(prop: &Property) -> Result<Self, Self::Error> {
            match (prop.key(), prop.params().get("VALUE").map(Parameter::value)) {
                ("TRIGGER", Some("DATE-TIME")) => {
                    // date-time needs to be qualified "VALUE=DATE-TIME"
                    CalendarDateTime::from_str(prop.value()).map(Trigger::from)
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
                Trigger::Duration(duration, Some(related)) => {
                    Property::new("TRIGGER", duration.to_string())
                        .append_parameter(related)
                        .done()
                }

                Trigger::Duration(duration, None) => Property::new("TRIGGER", duration.to_string()),

                Trigger::DateTime(dt) => dt
                    .to_property("TRIGGER")
                    .add_parameter("VALUE", "DATE-TIME")
                    .done(),
            }
        }
    }

    #[test]
    fn test_trigger() {
        let prop: Property = Trigger::from(Duration::weeks(14)).into();
        let mut out = String::new();
        prop.fmt_write(&mut out).unwrap();
    }

    /// adding triggers to Alarm component serializes them and so we need to re-parse to read them
    /// I know this sucks, but let's do the refactoring of the internal representation elsewhere
    #[test]
    fn test_trigger_abs_from_str() {
        let now: CalendarDateTime = NaiveDate::from_ymd_opt(2022, 11, 17)
            .unwrap()
            .and_hms_opt(21, 32, 45)
            .unwrap()
            .into();

        let alarm_with_abs_trigger = Alarm::default()
            .append_property(Trigger::from(now.clone()))
            .done();

        alarm_with_abs_trigger.print().unwrap();

        pretty_assertions::assert_eq!(
            alarm_with_abs_trigger.get_trigger(),
            Some(Trigger::DateTime(now))
        );
    }

    #[test]
    fn test_trigger_abs_from_str_naive() {
        let now: CalendarDateTime = NaiveDate::from_ymd_opt(2022, 11, 17)
            .unwrap()
            .and_hms_opt(21, 32, 45)
            .unwrap()
            .into();

        let alarm_with_abs_trigger = Alarm::default()
            .append_property(Trigger::from(now.clone()))
            .done();

        alarm_with_abs_trigger.print().unwrap();

        pretty_assertions::assert_eq!(
            alarm_with_abs_trigger.get_trigger(),
            Some(Trigger::DateTime(now))
        );
    }

    #[test]
    fn test_trigger_dur_from_str() {
        let dur = Duration::minutes(15);

        let alarm_with_rel_trigger = Alarm::default().append_property(Trigger::from(dur)).done();
        alarm_with_rel_trigger.print().unwrap();

        pretty_assertions::assert_eq!(
            alarm_with_rel_trigger.get_trigger(),
            Some(Trigger::Duration(dur, None))
        );
    }

    #[test]
    fn test_trigger_dur_from_str_start() {
        let dur = Duration::minutes(15);
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
