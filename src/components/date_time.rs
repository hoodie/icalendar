use std::str::FromStr;

use jiff::{
    Timestamp, ZonedDateTime, civil,
    tz::{Disambiguation, TimeZone},
};

use crate::{Property, ValueType};

fn digits(value: &str) -> bool {
    value.bytes().all(|byte| byte.is_ascii_digit())
}

pub(crate) fn parse_date(value: &str) -> Option<civil::Date> {
    if value.len() != 8 || !digits(value) {
        return None;
    }
    let date = civil::Date::strptime("%Y%m%d", value).ok()?;
    (date.year() > 0 && format_date(date) == value).then_some(date)
}

fn format_date(date: civil::Date) -> String {
    assert!(
        date.year() > 0,
        "iCalendar dates require a four-digit positive year"
    );
    date.strftime("%Y%m%d").to_string()
}

pub(crate) fn parse_naive_date_time(value: &str) -> Option<civil::DateTime> {
    if value.len() != 15
        || value.as_bytes().get(8) != Some(&b'T')
        || !digits(value.get(..8)?)
        || !digits(value.get(9..)?)
        || value.get(13..15)?.parse::<u8>().ok()? > 59
    {
        return None;
    }
    let date_time = civil::DateTime::strptime("%Y%m%dT%H%M%S", value).ok()?;
    (date_time.year() > 0 && format_naive_date_time(date_time) == value).then_some(date_time)
}

fn format_naive_date_time(value: civil::DateTime) -> String {
    assert!(
        value.year() > 0,
        "iCalendar dates require a four-digit positive year"
    );
    value.strftime("%Y%m%dT%H%M%S").to_string()
}

pub(crate) fn parse_utc_date_time(value: &str) -> Option<Timestamp> {
    let naive = parse_naive_date_time(value.strip_suffix('Z')?)?;
    Some(naive.to_zoned_datetime(TimeZone::UTC).ok()?.timestamp())
}

pub(crate) fn format_utc_date_time(value: Timestamp) -> String {
    format!(
        "{}Z",
        format_naive_date_time(value.to_zoned_datetime(TimeZone::UTC).datetime())
    )
}

pub(crate) fn naive_date_to_property(date: civil::Date, key: &str) -> Property {
    Property::new(key, format_date(date))
        .append_parameter(ValueType::Date)
        .done()
}

/// Representation of the `DATE-TIME` forms in
/// [RFC 5545, Section 3.3.5](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.5)
///
/// Conversions from Jiff types are provided through [`From`] implementations
///
/// All three RFC forms are supported. Parsing preserves `TZID` values as written,
/// but does not resolve them against `VTIMEZONE` definitions in the calendar
/// `try_into_utc` resolves only time zones known to Jiff
///
/// Subseconds are dropped on serialization
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CalendarDateTime {
    /// `FORM #1: DATE WITH LOCAL TIME`: floating, follows the attendee's current time zone
    ///
    /// Conversion from [`civil::DateTime`] results in this variant
    /// No `Z` suffix or `TZID`, so the instant depends on the attendee's time zone
    Floating(civil::DateTime),

    /// `FORM #2: DATE WITH UTC TIME`: rendered with a `Z` suffix
    ///
    /// Conversion from [`Timestamp`] results in this variant
    Utc(Timestamp),

    /// `FORM #3: DATE WITH LOCAL TIME AND TIME ZONE REFERENCE`: refers to a time zone by `TZID`
    ///
    /// Conversion from [`ZonedDateTime`] with an IANA zone results in this variant,
    /// except for the later occurrence in a DST fold, which uses [`Utc`](Self::Utc)
    WithTimezone {
        /// The date and time in the given time zone
        date_time: civil::DateTime,
        /// The ID of the referenced time zone, possibly a `VTIMEZONE` definition
        tzid: String,
    },
}

impl CalendarDateTime {
    /// Fixed date for testing, not the current time
    #[cfg(test)]
    pub(crate) fn now() -> Self {
        civil::date(2015, 10, 26)
            .at(1, 22, 0, 0)
            .to_zoned_datetime(TimeZone::UTC)
            .unwrap()
            .timestamp()
            .into()
    }

    pub(crate) fn from_property(property: &Property) -> Option<Self> {
        let value = property.value();
        if let Some(tzid) = property.params().get("TZID") {
            if tzid.value().is_empty() {
                return None;
            }
            return Some(Self::WithTimezone {
                date_time: parse_naive_date_time(value)?,
                tzid: tzid.value().to_owned(),
            });
        }
        value.parse().ok()
    }

    pub(crate) fn to_property(&self, key: &str) -> Property {
        match self {
            Self::Floating(date_time) => Property::new(key, format_naive_date_time(*date_time)),
            Self::Utc(timestamp) => Property::new(key, format_utc_date_time(*timestamp)),
            Self::WithTimezone { date_time, tzid } => {
                Property::new(key, format_naive_date_time(*date_time))
                    .add_parameter("TZID", tzid)
                    .done()
            }
        }
    }

    pub(crate) fn from_utc_string(value: &str) -> Option<Self> {
        parse_utc_date_time(value).map(Self::Utc)
    }

    pub(crate) fn from_naive_string(value: &str) -> Option<Self> {
        parse_naive_date_time(value).map(Self::Floating)
    }

    /// Resolve a TZID with Jiff's compatible gap and fold policy
    /// Floating times and unknown zones return `None`
    pub fn try_into_utc(&self) -> Option<Timestamp> {
        self.try_into_utc_with(Disambiguation::Compatible)
    }

    /// Resolve a TZID with the given ambiguity policy
    pub fn try_into_utc_with(&self, disambiguation: Disambiguation) -> Option<Timestamp> {
        match self {
            Self::Floating(_) => None,
            Self::Utc(value) => Some(*value),
            Self::WithTimezone { date_time, tzid } => TimeZone::get(tzid)
                .ok()?
                .to_ambiguous_zoned_datetime(*date_time)
                .disambiguate(disambiguation)
                .ok()
                .map(|value| value.timestamp()),
        }
    }
}

impl From<Timestamp> for CalendarDateTime {
    fn from(value: Timestamp) -> Self {
        Self::Utc(value)
    }
}

impl From<&Timestamp> for CalendarDateTime {
    fn from(value: &Timestamp) -> Self {
        Self::Utc(*value)
    }
}

impl From<civil::DateTime> for CalendarDateTime {
    fn from(value: civil::DateTime) -> Self {
        Self::Floating(value)
    }
}

impl From<&civil::DateTime> for CalendarDateTime {
    fn from(value: &civil::DateTime) -> Self {
        Self::Floating(*value)
    }
}

impl From<&ZonedDateTime> for CalendarDateTime {
    fn from(value: &ZonedDateTime) -> Self {
        let Some(tzid) = value.time_zone().iana_name() else {
            return Self::Utc(value.timestamp());
        };
        let local = value.datetime();
        let first = value
            .time_zone()
            .to_ambiguous_zoned_datetime(local)
            .compatible();
        if first.is_ok_and(|first| first.timestamp() != value.timestamp()) {
            return Self::Utc(value.timestamp());
        }
        Self::WithTimezone {
            date_time: local,
            tzid: tzid.to_owned(),
        }
    }
}

impl From<ZonedDateTime> for CalendarDateTime {
    fn from(value: ZonedDateTime) -> Self {
        Self::from(&value)
    }
}

impl FromStr for CalendarDateTime {
    type Err = ();
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::from_utc_string(value)
            .or_else(|| Self::from_naive_string(value))
            .ok_or(())
    }
}

/// iCalendar [`DATE`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.4)
/// or [`DATE-TIME`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.5)
///
/// Jiff inputs, owned or borrowed:
///
/// | Jiff input | Variant | Serialized form |
/// | --- | --- | --- |
/// | [`civil::Date`] | [`Date`](Self::Date) | `VALUE=DATE` |
/// | [`civil::DateTime`] | [`DateTime`](Self::DateTime) with [`Floating`](CalendarDateTime::Floating) | floating local `DATE-TIME` |
/// | [`Timestamp`] | [`DateTime`](Self::DateTime) with [`Utc`](CalendarDateTime::Utc) | UTC `DATE-TIME` ending in `Z` |
/// | [`ZonedDateTime`] with an IANA zone | [`DateTime`](Self::DateTime) with [`WithTimezone`](CalendarDateTime::WithTimezone) | local `DATE-TIME` with `TZID` |
/// | [`ZonedDateTime`] without an IANA zone | [`DateTime`](Self::DateTime) with [`Utc`](CalendarDateTime::Utc) | UTC `DATE-TIME` ending in `Z` |
///
/// A floating time, such as `DTSTART:20261102T090000`, has neither a `Z` suffix nor a `TZID`
///
/// A `VTIMEZONE` elsewhere doesn't change that.
/// Use UTC or [`TZID`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.19) for a fixed instant ([RFC 5545 §3.3.5](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.5)).
///
/// The later occurrence in a DST fold falls back to UTC.
/// `TZID` alone can't preserve which occurrence it was.
/// `DATE` isn't midnight. Parsed TZIDs stay unresolved, including custom `VTIMEZONE` IDs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DatePerhapsTime {
    /// floating, UTC, or `TZID` local time
    DateTime(CalendarDateTime),
    /// date-only, with `VALUE=DATE`
    Date(civil::Date),
}

impl DatePerhapsTime {
    /// Parse a date or datetime without resolving its TZID
    pub fn from_property(property: &Property) -> Option<Self> {
        let value = property.value();
        if property.value_type() == Some(ValueType::Date) {
            return (property.params().get("TZID").is_none())
                .then(|| parse_date(value))
                .flatten()
                .map(Self::Date);
        }
        if let Some(date_time) = CalendarDateTime::from_property(property) {
            return Some(Self::DateTime(date_time));
        }
        #[cfg(not(feature = "strict-dates"))]
        {
            if property.params().get("VALUE").is_some() || property.params().get("TZID").is_some() {
                return None;
            }
            parse_date(value).map(Self::Date)
        }
        #[cfg(feature = "strict-dates")]
        {
            None
        }
    }

    /// Convert to an iCalendar property
    pub fn to_property(&self, key: &str) -> Property {
        match self {
            Self::DateTime(value) => value.to_property(key),
            Self::Date(value) => naive_date_to_property(*value, key),
        }
    }

    /// Local date, or UTC date for a UTC instant
    pub fn date_naive(&self) -> civil::Date {
        match self {
            Self::Date(date) => *date,
            Self::DateTime(CalendarDateTime::Floating(value)) => value.date(),
            Self::DateTime(CalendarDateTime::Utc(value)) => {
                value.to_zoned_datetime(TimeZone::UTC).date()
            }
            Self::DateTime(CalendarDateTime::WithTimezone { date_time, .. }) => date_time.date(),
        }
    }
}

impl From<DatePerhapsTime> for civil::Date {
    fn from(value: DatePerhapsTime) -> Self {
        value.date_naive()
    }
}

impl From<CalendarDateTime> for DatePerhapsTime {
    fn from(value: CalendarDateTime) -> Self {
        Self::DateTime(value)
    }
}

impl From<civil::Date> for DatePerhapsTime {
    fn from(value: civil::Date) -> Self {
        Self::Date(value)
    }
}

impl From<&civil::Date> for DatePerhapsTime {
    fn from(value: &civil::Date) -> Self {
        Self::Date(*value)
    }
}

impl From<civil::DateTime> for DatePerhapsTime {
    fn from(value: civil::DateTime) -> Self {
        Self::DateTime(value.into())
    }
}

impl From<&civil::DateTime> for DatePerhapsTime {
    fn from(value: &civil::DateTime) -> Self {
        Self::DateTime(value.into())
    }
}

impl From<Timestamp> for DatePerhapsTime {
    fn from(value: Timestamp) -> Self {
        Self::DateTime(value.into())
    }
}

impl From<&Timestamp> for DatePerhapsTime {
    fn from(value: &Timestamp) -> Self {
        Self::DateTime(value.into())
    }
}

impl From<ZonedDateTime> for DatePerhapsTime {
    fn from(value: ZonedDateTime) -> Self {
        Self::DateTime(value.into())
    }
}

impl From<&ZonedDateTime> for DatePerhapsTime {
    fn from(value: &ZonedDateTime) -> Self {
        Self::DateTime(value.into())
    }
}

#[cfg(feature = "parser")]
impl TryFrom<&crate::parser::Property<'_>> for DatePerhapsTime {
    type Error = &'static str;

    fn try_from(value: &crate::parser::Property) -> Result<Self, Self::Error> {
        let raw = value.val.as_ref();
        let value_type = value
            .params
            .iter()
            .find(|param| param.key == "VALUE")
            .and_then(|param| param.val.as_ref())
            .map(crate::parser::ParseString::as_str);
        let tz_param = value.params.iter().find(|param| param.key == "TZID");
        if !matches!(value_type, None | Some("DATE") | Some("DATE-TIME")) {
            return Err("Unknown DATE-TIME value type");
        }
        if value_type == Some("DATE") {
            if tz_param.is_some() {
                return Err("DATE cannot have TZID");
            }
            return parse_date(raw).map(Self::Date).ok_or("Invalid DATE");
        }
        if let Some(timestamp) = parse_utc_date_time(raw) {
            if tz_param.is_some() {
                return Err("UTC cannot have TZID");
            }
            return Ok(Self::DateTime(CalendarDateTime::Utc(timestamp)));
        }
        if let Some(date) = parse_date(raw) {
            if cfg!(feature = "strict-dates") || value_type.is_some() || tz_param.is_some() {
                return Err("DATE requires VALUE=DATE");
            }
            return Ok(Self::Date(date));
        }
        if let Some(date_time) = parse_naive_date_time(raw) {
            if let Some(tz) = tz_param {
                let tzid = tz
                    .val
                    .as_ref()
                    .map(crate::parser::ParseString::as_str)
                    .filter(|tzid| !tzid.is_empty())
                    .ok_or("Found empty TZID param.")?;
                return Ok(Self::DateTime(CalendarDateTime::WithTimezone {
                    date_time,
                    tzid: tzid.to_string(),
                }));
            }
            return Ok(Self::DateTime(CalendarDateTime::Floating(date_time)));
        }
        Err("Value does not look like a known DATE-TIME")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    #[test]
    fn wire_forms() {
        let local = date(2026, 3, 15).at(18, 45, 0, 0);
        assert_eq!(
            CalendarDateTime::from(local).to_property("DTSTART").value(),
            "20260315T184500"
        );
        assert_eq!(
            DatePerhapsTime::from(local.date())
                .to_property("DTSTART")
                .value(),
            "20260315"
        );
        assert_eq!(
            format_utc_date_time(parse_utc_date_time("20260315T184500Z").unwrap()),
            "20260315T184500Z"
        );
        for invalid in [
            "2026-03-15",
            "20260315T184500+01:00",
            "20260315T1845Z",
            "20260315T184560Z",
            "20260315T184500.1Z",
        ] {
            assert!(parse_date(invalid).is_none(), "{invalid}");
            assert!(parse_utc_date_time(invalid).is_none(), "{invalid}");
            assert!(parse_naive_date_time(invalid).is_none(), "{invalid}");
        }
    }

    #[test]
    fn named_and_unknown_zones() {
        let local = date(2026, 3, 15).at(18, 45, 0, 0);
        let zoned = local.in_tz("Europe/Berlin").unwrap();
        assert_eq!(
            CalendarDateTime::from(&zoned)
                .to_property("DTSTART")
                .params()
                .get("TZID")
                .unwrap()
                .value(),
            "Europe/Berlin"
        );
        let custom = Property::new("DTSTART", "20260315T184500")
            .add_parameter("TZID", "Custom/Zone")
            .done();
        assert_eq!(
            CalendarDateTime::from_property(&custom)
                .unwrap()
                .to_property("DTSTART"),
            custom
        );
        assert_eq!(
            CalendarDateTime::from_property(&custom)
                .unwrap()
                .try_into_utc(),
            None
        );
    }

    #[test]
    fn fixed_offsets_are_serialized_in_utc() {
        let timestamp = date(2026, 3, 15)
            .at(18, 45, 0, 0)
            .in_tz("UTC")
            .unwrap()
            .timestamp();
        let fixed = timestamp.to_zoned_datetime(TimeZone::fixed(jiff::tz::offset(2)));
        let converted = CalendarDateTime::from(&fixed);
        assert_eq!(converted, CalendarDateTime::Utc(timestamp));
        assert_eq!(converted.to_property("DTSTART").value(), "20260315T184500Z");
    }

    #[test]
    fn gaps_use_compatible_resolution_by_default() {
        let local = date(2026, 3, 8).at(2, 30, 0, 0);
        let value = CalendarDateTime::WithTimezone {
            date_time: local,
            tzid: "America/New_York".into(),
        };
        assert!(value.try_into_utc().is_some());
        assert!(value.try_into_utc_with(Disambiguation::Reject).is_none());
    }

    #[test]
    fn subsecond_inputs_are_truncated_on_output() {
        let local = date(2026, 3, 15).at(18, 45, 0, 123_456_789);
        assert_eq!(
            CalendarDateTime::from(local).to_property("DTSTART").value(),
            "20260315T184500"
        );
    }

    #[test]
    fn fold_preserves_instant() {
        let zone = TimeZone::get("America/New_York").unwrap();
        let local = date(2026, 11, 1).at(1, 30, 0, 0);
        let later = zone.to_ambiguous_zoned_datetime(local).later().unwrap();
        assert_eq!(
            CalendarDateTime::from(&later),
            CalendarDateTime::Utc(later.timestamp())
        );
        let earlier = zone.to_ambiguous_zoned_datetime(local).earlier().unwrap();
        assert!(matches!(
            CalendarDateTime::from(&earlier),
            CalendarDateTime::WithTimezone { .. }
        ));
        let named = CalendarDateTime::WithTimezone {
            date_time: local,
            tzid: "America/New_York".into(),
        };
        assert!(named.try_into_utc_with(Disambiguation::Reject).is_none());
    }

    #[test]
    #[cfg(feature = "parser")]
    fn parsed_properties_keep_custom_tzid_and_date_policy() {
        let mut parsed = crate::parser::Property {
            name: "DTSTART".into(),
            val: "20260315T184500".into(),
            params: vec![crate::parser::Parameter {
                key: "TZID".into(),
                val: Some("Custom/Zone".into()),
            }],
        };
        assert_eq!(
            DatePerhapsTime::try_from(&parsed).unwrap(),
            DatePerhapsTime::DateTime(CalendarDateTime::WithTimezone {
                date_time: date(2026, 3, 15).at(18, 45, 0, 0),
                tzid: "Custom/Zone".into()
            })
        );
        parsed.val = "20260315".into();
        assert!(DatePerhapsTime::try_from(&parsed).is_err());
        parsed.params.clear();
        assert_eq!(
            DatePerhapsTime::try_from(&parsed).is_ok(),
            !cfg!(feature = "strict-dates")
        );
        parsed.params.push(crate::parser::Parameter {
            key: "VALUE".into(),
            val: Some("DATE".into()),
        });
        assert_eq!(
            DatePerhapsTime::try_from(&parsed).unwrap(),
            DatePerhapsTime::Date(date(2026, 3, 15))
        );
    }

    #[test]
    #[cfg(feature = "parser")]
    fn parsed_datetime_forms_reject_empty_tzid() {
        let mut parsed = crate::parser::Property {
            name: "DTSTART".into(),
            val: "20260315T184500Z".into(),
            params: vec![],
        };
        assert!(matches!(
            DatePerhapsTime::try_from(&parsed),
            Ok(DatePerhapsTime::DateTime(CalendarDateTime::Utc(_)))
        ));
        parsed.val = "20260315T184500".into();
        assert!(matches!(
            DatePerhapsTime::try_from(&parsed),
            Ok(DatePerhapsTime::DateTime(CalendarDateTime::Floating(_)))
        ));
        parsed.params.push(crate::parser::Parameter {
            key: "TZID".into(),
            val: None,
        });
        assert_eq!(
            DatePerhapsTime::try_from(&parsed),
            Err("Found empty TZID param.")
        );
        parsed.val = "20260315T184500Z".into();
        assert_eq!(
            DatePerhapsTime::try_from(&parsed),
            Err("UTC cannot have TZID")
        );
    }

    #[test]
    fn strict_date_feature() {
        let mut raw = Property::new("DTSTART", "20000101");
        assert_eq!(
            DatePerhapsTime::from_property(&raw).is_some(),
            !cfg!(feature = "strict-dates")
        );
        let explicit = raw.append_parameter(("VALUE", "DATE"));
        assert!(DatePerhapsTime::from_property(explicit).is_some());
    }
}
