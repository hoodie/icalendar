use std::str::FromStr;

use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, TimeZone as _, Utc};

use crate::{Property, ValueType};

const NAIVE_DATE_TIME_FORMAT: &str = "%Y%m%dT%H%M%S";
const UTC_DATE_TIME_FORMAT: &str = "%Y%m%dT%H%M%SZ";
const NAIVE_DATE_FORMAT: &str = "%Y%m%d";

// #[deprecated(note = "use `CalendarDateTime::from_str` if you can")]
pub(crate) fn parse_utc_date_time(s: &str) -> Option<DateTime<Utc>> {
    Utc.datetime_from_str(s, UTC_DATE_TIME_FORMAT).ok()
}

pub(crate) fn parse_naive_date_time(s: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(s, NAIVE_DATE_TIME_FORMAT).ok()
}

pub(crate) fn format_utc_date_time(utc_dt: DateTime<Utc>) -> String {
    utc_dt.format(UTC_DATE_TIME_FORMAT).to_string()
}

pub(crate) fn parse_duration(s: &str) -> Option<Duration> {
    iso8601::duration(s)
        .ok()
        .and_then(|iso| Duration::from_std(iso.into()).ok())
}

pub(crate) fn naive_date_to_property(date: NaiveDate, key: &str) -> Property {
    Property::new(key, date.format(NAIVE_DATE_FORMAT).to_string())
        .append_parameter(ValueType::Date)
        .done()
}

/// Representation of various forms of `DATE-TIME` per
/// [RFC 5545, Section 3.3.5](https://tools.ietf.org/html/rfc5545#section-3.3.5)
///
/// Conversions from [chrono] types are provided in form of [From] implementations, see
/// documentation of individual variants.
///
/// In addition to readily implemented `FORM #1` and `FORM #2`, the RFC also specifies
/// `FORM #3: DATE WITH LOCAL TIME AND TIME ZONE REFERENCE`. This variant is not yet implemented.
/// Adding it will require adding support for `VTIMEZONE` and referencing it using `TZID`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CalendarDateTime {
    /// `FORM #1: DATE WITH LOCAL TIME`: floating, follows current time-zone of the attendee.
    ///
    /// Conversion from [`chrono::NaiveDateTime`] results in this variant.
    ///
    /// ## Note
    /// finding this in a calendar is a red flag, datetimes should end in `'Z'` for `UTC` or have a `TZID` property
    Floating(NaiveDateTime),

    /// `FORM #2: DATE WITH UTC TIME`: rendered with Z suffix character.
    ///
    /// Conversion from [`chrono::DateTime<Utc>`](DateTime) results in this variant. Use
    /// `date_time.with_timezone(&Utc)` to convert `date_time` from arbitrary time zone to UTC.
    Utc(DateTime<Utc>),

    /// `FORM #3: DATE WITH LOCAL TIME AND TIME ZONE REFERENCE`: refers to a time zone definition.
    WithTimezone {
        /// The date and time in the given time zone.
        date_time: NaiveDateTime,
        /// The ID of the time zone definition in a VTIMEZONE calendar component.
        tzid: String,
    },
}

impl CalendarDateTime {
    /// this is not actually now, just a fixed date for testing
    #[cfg(test)]
    pub(crate) fn now() -> Self {
        NaiveDate::from_ymd_opt(2015, 10, 26)
            .unwrap()
            .and_hms_opt(1, 22, 00)
            .unwrap()
            .into()
    }

    pub(crate) fn from_property(property: &Property) -> Option<Self> {
        let value = property.value();
        if let Some(tzid) = property.params().get("TZID") {
            Some(Self::WithTimezone {
                date_time: NaiveDateTime::parse_from_str(value, NAIVE_DATE_TIME_FORMAT).ok()?,
                tzid: tzid.value().to_owned(),
            })
        } else if let Ok(naive_date_time) =
            NaiveDateTime::parse_from_str(value, NAIVE_DATE_TIME_FORMAT)
        {
            Some(naive_date_time.into())
        } else {
            Self::from_str(value).ok()
        }
    }

    pub(crate) fn to_property(&self, key: &str) -> Property {
        match self {
            CalendarDateTime::Floating(naive_dt) => {
                Property::new(key, naive_dt.format(NAIVE_DATE_TIME_FORMAT).to_string())
            }
            CalendarDateTime::Utc(utc_dt) => Property::new(key, format_utc_date_time(*utc_dt)),
            CalendarDateTime::WithTimezone { date_time, tzid } => {
                Property::new(key, date_time.format(NAIVE_DATE_TIME_FORMAT).to_string())
                    .add_parameter("TZID", tzid)
                    .done()
            }
        }
    }

    pub(crate) fn from_utc_string(s: &str) -> Option<Self> {
        parse_utc_date_time(s).map(CalendarDateTime::Utc)
    }

    pub(crate) fn from_naive_string(s: &str) -> Option<Self> {
        parse_naive_date_time(s).map(CalendarDateTime::Floating)
    }

    /// attempts to convert the into UTC
    #[cfg(feature = "chrono-tz")]
    pub fn try_into_utc(&self) -> Option<DateTime<Utc>> {
        match self {
            CalendarDateTime::Floating(_) => None, // we shouldn't guess here
            CalendarDateTime::Utc(inner) => Some(*inner),
            CalendarDateTime::WithTimezone { date_time, tzid } => tzid
                .parse::<chrono_tz::Tz>()
                .ok()
                .and_then(|tz| tz.from_local_datetime(date_time).single())
                .map(|tz| tz.with_timezone(&Utc)),
        }
    }

    /// TODO: make public or delete
    #[cfg(feature = "chrono-tz")]
    #[allow(dead_code)]
    pub(crate) fn with_timezone(dt: NaiveDateTime, tz_id: chrono_tz::Tz) -> Self {
        Self::WithTimezone {
            date_time: dt,
            tzid: tz_id.name().to_owned(),
        }
    }

    /// will return [`None`] if date is not valid
    #[cfg(feature = "chrono-tz")]
    pub fn from_ymd_hm_tzid(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        tz_id: chrono_tz::Tz,
    ) -> Option<Self> {
        NaiveDate::from_ymd_opt(year, month, day)
            .and_then(|date| date.and_hms_opt(hour, min, 0))
            .zip(Some(tz_id))
            .map(|(dt, tz)| Self::with_timezone(dt, tz))
    }

    /// Create a new instance with the given timezone
    #[cfg(feature = "chrono-tz")]
    pub fn from_date_time<TZ: chrono::TimeZone<Offset = O>, O: chrono_tz::OffsetName>(
        dt: DateTime<TZ>,
    ) -> Self {
        Self::WithTimezone {
            date_time: dt.naive_local(),
            tzid: dt.offset().tz_id().to_owned(),
        }
    }
}

/// will return [`None`] if date is not valid
#[cfg(feature = "chrono-tz")]
pub fn ymd_hm_tzid(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
    tz_id: chrono_tz::Tz,
) -> Option<CalendarDateTime> {
    CalendarDateTime::from_ymd_hm_tzid(year, month, day, hour, min, tz_id)
}

/// Converts from time zone-aware UTC date-time to [`CalendarDateTime::Utc`].
impl From<DateTime<Utc>> for CalendarDateTime {
    fn from(dt: DateTime<Utc>) -> Self {
        Self::Utc(dt)
    }
}

// impl<TZ: chrono::TimeZone<Offset = O>, O: chrono_tz::OffsetName> From<DateTime<TZ>>
//     for CalendarDateTime
// {
//     fn from(date_time: DateTime<TZ>) -> Self {
//         CalendarDateTime::WithTimezone {
//             date_time: date_time.naive_local(),
//             tzid: date_time.offset().tz_id().to_owned(),
//         }
//     }
// }

/// Converts from time zone-less date-time to [`CalendarDateTime::Floating`].
impl From<NaiveDateTime> for CalendarDateTime {
    fn from(dt: NaiveDateTime) -> Self {
        Self::Floating(dt)
    }
}

#[cfg(feature = "chrono-tz")]
impl From<(NaiveDateTime, chrono_tz::Tz)> for CalendarDateTime {
    fn from((date_time, tzid): (NaiveDateTime, chrono_tz::Tz)) -> Self {
        Self::WithTimezone {
            date_time,
            tzid: tzid.name().into(),
        }
    }
}

#[cfg(feature = "chrono-tz")]
impl TryFrom<(NaiveDateTime, &str)> for CalendarDateTime {
    type Error = String;

    fn try_from((dt, maybe_tz): (NaiveDateTime, &str)) -> Result<Self, Self::Error> {
        let tzid: chrono_tz::Tz = maybe_tz
            .parse()
            .map_err(|e: chrono_tz::ParseError| e.to_string())?;
        Ok(CalendarDateTime::from((dt, tzid)))
    }
}

impl FromStr for CalendarDateTime {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CalendarDateTime::from_utc_string(s)
            .or_else(|| CalendarDateTime::from_naive_string(s))
            .ok_or(())
    }
}

/// Either a `DATE-TIME` or a `DATE`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DatePerhapsTime {
    /// A `DATE-TIME` property.
    DateTime(CalendarDateTime),
    /// A `DATE` property.
    Date(NaiveDate),
}

impl DatePerhapsTime {
    /// Attempts to convert the given property into a `DatePerhapsTime`.
    pub fn from_property(property: &Property) -> Option<Self> {
        let value = property.value();

        // Check if VALUE=DATE parameter is explicitly set
        if property.value_type() == Some(ValueType::Date) {
            return Some(
                NaiveDate::parse_from_str(value, NAIVE_DATE_FORMAT)
                    .ok()?
                    .into(),
            );
        }

        // Try parsing as DATE-TIME first (the default for DTSTART)
        if let Some(dt) = CalendarDateTime::from_property(property) {
            return Some(dt.into());
        }

        // Fall back to parsing as DATE for leniency (handles DATE without VALUE=DATE parameter)
        #[cfg(not(feature = "strict-dates"))]
        {
            NaiveDate::parse_from_str(value, NAIVE_DATE_FORMAT)
                .ok()
                .map(Into::into)
        }

        // In strict mode, do not accept DATE without VALUE=DATE parameter
        #[cfg(feature = "strict-dates")]
        None
    }

    /// Converts this `DatePerhapsTime` into a `Property`.
    pub fn to_property(&self, key: &str) -> Property {
        match self {
            Self::DateTime(date_time) => date_time.to_property(key),
            Self::Date(date) => naive_date_to_property(*date, key),
        }
    }

    /// Discards time, assumes UTC, and returns an owned instance of a pure date
    pub fn date_naive(&self) -> NaiveDate {
        use crate::DatePerhapsTime::*;
        match self {
            Date(date) => date.to_owned(),
            DateTime(CalendarDateTime::Floating(date_time)) => date_time.date(),
            DateTime(CalendarDateTime::Utc(date_time)) => date_time.date_naive(),
            DateTime(CalendarDateTime::WithTimezone { date_time, tzid: _ }) => date_time.date(),
        }
    }
}

/// TODO: make public or delete
#[cfg(feature = "chrono-tz")]
#[allow(dead_code)]
pub fn with_timezone<T: chrono::TimeZone + chrono_tz::OffsetName>(
    dt: DateTime<T>,
) -> DatePerhapsTime {
    CalendarDateTime::WithTimezone {
        date_time: dt.naive_local(),
        tzid: dt.timezone().tz_id().to_owned(),
    }
    .into()
}

impl From<DatePerhapsTime> for NaiveDate {
    fn from(dt: DatePerhapsTime) -> Self {
        match dt {
            DatePerhapsTime::Date(date) => date,
            DatePerhapsTime::DateTime(CalendarDateTime::Floating(date_time)) => date_time.date(),
            DatePerhapsTime::DateTime(CalendarDateTime::Utc(date_time)) => date_time.date_naive(),
            DatePerhapsTime::DateTime(CalendarDateTime::WithTimezone { date_time, tzid: _ }) => {
                date_time.date()
            }
        }
    }
}

impl From<CalendarDateTime> for DatePerhapsTime {
    fn from(dt: CalendarDateTime) -> Self {
        Self::DateTime(dt)
    }
}

impl From<DateTime<Utc>> for DatePerhapsTime {
    fn from(dt: DateTime<Utc>) -> Self {
        Self::DateTime(CalendarDateTime::Utc(dt))
    }
}

// CANT HAVE NICE THINGS until specializations are stable
// OR: breaking change and make this the default
// impl<TZ: chrono::TimeZone<Offset = O>, O: chrono_tz::OffsetName> From<DateTime<TZ>>
//     for DatePerhapsTime
// {
//     fn from(date_time: DateTime<TZ>) -> Self {
//         Self::DateTime(CalendarDateTime::from(date_time))
//     }
// }

#[allow(deprecated)]
impl From<chrono::Date<Utc>> for DatePerhapsTime {
    fn from(dt: chrono::Date<Utc>) -> Self {
        Self::Date(dt.naive_utc())
    }
}

impl From<NaiveDateTime> for DatePerhapsTime {
    fn from(dt: NaiveDateTime) -> Self {
        Self::DateTime(dt.into())
    }
}

#[cfg(feature = "chrono-tz")]
impl TryFrom<(NaiveDateTime, &str)> for DatePerhapsTime {
    type Error = String;

    fn try_from(value: (NaiveDateTime, &str)) -> Result<Self, Self::Error> {
        Ok(Self::DateTime(value.try_into()?))
    }
}
#[cfg(feature = "chrono-tz")]
impl From<(NaiveDateTime, chrono_tz::Tz)> for DatePerhapsTime {
    fn from(both: (NaiveDateTime, chrono_tz::Tz)) -> Self {
        Self::DateTime(both.into())
    }
}

impl From<NaiveDate> for DatePerhapsTime {
    fn from(date: NaiveDate) -> Self {
        Self::Date(date)
    }
}

#[cfg(feature = "time")]
impl From<time::Date> for DatePerhapsTime {
    fn from(date: time::Date) -> Self {
        let (y, o) = date.to_ordinal_date();
        Self::Date(NaiveDate::from_yo_opt(y, o as u32).expect("bug: invalid date"))
    }
}

#[cfg(feature = "time")]
impl From<time::OffsetDateTime> for DatePerhapsTime {
    fn from(datetime: time::OffsetDateTime) -> Self {
        // NOTE: A `time::UtcOffset` doesn't carry information about the timezone other than the offset
        datetime.to_utc().into()
    }
}

#[cfg(feature = "time")]
impl From<time::UtcDateTime> for DatePerhapsTime {
    fn from(datetime: time::UtcDateTime) -> Self {
        Self::DateTime(CalendarDateTime::Utc(
            DateTime::from_timestamp(datetime.unix_timestamp(), datetime.nanosecond())
                .expect("bug: invalid time"),
        ))
    }
}

#[cfg(feature = "time")]
impl From<time::PrimitiveDateTime> for DatePerhapsTime {
    fn from(datetime: time::PrimitiveDateTime) -> Self {
        let utc = datetime.assume_utc();
        Self::DateTime(CalendarDateTime::Floating(
            NaiveDateTime::from_timestamp_opt(utc.unix_timestamp(), utc.nanosecond())
                .expect("bug: invalid date"),
        ))
    }
}

#[cfg(feature = "parser")]
impl TryFrom<&crate::parser::Property<'_>> for DatePerhapsTime {
    type Error = &'static str;

    fn try_from(value: &crate::parser::Property) -> Result<Self, Self::Error> {
        let val = value.val.as_ref();

        // UTC is here first because lots of fields MUST be UTC, so it should,
        // in practice, be more common that others.
        if let Ok(utc_dt) = Utc.datetime_from_str(val, "%Y%m%dT%H%M%SZ") {
            return Ok(Self::DateTime(CalendarDateTime::Utc(utc_dt)));
        };

        if let Ok(naive_date) = NaiveDate::parse_from_str(val, "%Y%m%d") {
            return Ok(Self::Date(naive_date));
        };

        if let Ok(naive_dt) = NaiveDateTime::parse_from_str(val, "%Y%m%dT%H%M%S") {
            if let Some(tz_param) = value.params.iter().find(|p| p.key == "TZID") {
                if let Some(tzid) = &tz_param.val {
                    return Ok(Self::DateTime(CalendarDateTime::WithTimezone {
                        date_time: naive_dt,
                        tzid: tzid.as_ref().to_string(),
                    }));
                } else {
                    return Err("Found empty TZID param.");
                }
            } else {
                return Ok(Self::DateTime(CalendarDateTime::Floating(naive_dt)));
            };
        };

        Err("Value does not look like a known DATE-TIME")
    }
}

#[cfg(all(test, feature = "parser"))]
mod try_from_tests {
    use super::*;

    #[test]
    fn try_from_utc_dt() {
        let prop = crate::parser::Property {
            name: "TRIGGER".into(),
            val: "20220716T141500Z".into(),
            params: vec![crate::parser::Parameter {
                key: "VALUE".into(),
                val: Some("DATE-TIME".into()),
            }],
        };

        let result = DatePerhapsTime::try_from(&prop);
        let expected = Utc.ymd(2022, 7, 16).and_hms(14, 15, 0);

        assert_eq!(
            result,
            Ok(DatePerhapsTime::DateTime(CalendarDateTime::Utc(expected)))
        );
    }

    #[test]
    fn try_from_naive_date() {
        let prop = crate::parser::Property {
            name: "TRIGGER".into(),
            val: "19970714".into(),
            params: vec![crate::parser::Parameter {
                key: "VALUE".into(),
                val: Some("DATE-TIME".into()),
            }],
        };

        let result = DatePerhapsTime::try_from(&prop);
        let expected = NaiveDate::from_ymd(1997, 7, 14);

        assert_eq!(result, Ok(DatePerhapsTime::Date(expected)));
    }

    #[test]
    fn try_from_dt_with_tz() {
        let prop = crate::parser::Property {
            name: "TRIGGER".into(),
            val: "20220716T141500".into(),
            params: vec![
                crate::parser::Parameter {
                    key: "VALUE".into(),
                    val: Some("DATE-TIME".into()),
                },
                crate::parser::Parameter {
                    key: "TZID".into(),
                    val: Some("MY-TZ-ID".into()),
                },
            ],
        };

        let result = DatePerhapsTime::try_from(&prop);
        let expected = NaiveDate::from_ymd(2022, 7, 16).and_hms(14, 15, 0);

        assert_eq!(
            result,
            Ok(DatePerhapsTime::DateTime(CalendarDateTime::WithTimezone {
                date_time: expected,
                tzid: "MY-TZ-ID".into(),
            }))
        );
    }

    #[test]
    fn try_from_dt_with_empty_tz() {
        let prop = crate::parser::Property {
            name: "TRIGGER".into(),
            val: "20220716T141500".into(),
            params: vec![
                crate::parser::Parameter {
                    key: "VALUE".into(),
                    val: Some("DATE-TIME".into()),
                },
                crate::parser::Parameter {
                    key: "TZID".into(),
                    val: None,
                },
            ],
        };

        let result = DatePerhapsTime::try_from(&prop);

        assert_eq!(result, Err("Found empty TZID param."));
    }

    #[test]
    fn try_from_floating_dt() {
        let prop = crate::parser::Property {
            name: "TRIGGER".into(),
            val: "20220716T141500".into(),
            params: vec![crate::parser::Parameter {
                key: "VALUE".into(),
                val: Some("DATE-TIME".into()),
            }],
        };

        let result = DatePerhapsTime::try_from(&prop);
        let expected = NaiveDate::from_ymd(2022, 7, 16).and_hms(14, 15, 0);

        assert_eq!(
            result,
            Ok(DatePerhapsTime::DateTime(CalendarDateTime::Floating(
                expected
            )))
        );
    }

    #[test]
    fn try_from_non_dt_prop() {
        let prop = crate::parser::Property {
            name: "TZNAME".into(),
            val: "CET".into(),
            params: vec![],
        };

        let result = DatePerhapsTime::try_from(&prop);

        assert_eq!(result, Err("Value does not look like a known DATE-TIME"));
    }
}

#[cfg(test)]
mod strict_dates_tests {
    use super::*;

    #[test]
    #[cfg(not(feature = "strict-dates"))]
    fn lenient_mode_accepts_date_without_value_param() {
        let dt = DatePerhapsTime::from_property(&Property::new("DTSTART", "20000101"));

        assert!(
            dt.is_some(),
            "In lenient mode, DATE without VALUE=DATE parameter should parse successfully"
        );

        assert!(
            matches!(dt, Some(DatePerhapsTime::Date(_))),
            "Expected DatePerhapsTime::Date variant"
        );
    }

    #[test]
    #[cfg(feature = "strict-dates")]
    fn strict_mode_rejects_date_without_value_param() {
        assert!(
            DatePerhapsTime::from_property(&Property::new("DTSTART", "20000101")).is_none(),
            "In strict mode, DATE without VALUE=DATE parameter should fail to parse"
        );
    }

    #[test]
    fn both_modes_accept_date_with_value_param() {
        let dt = DatePerhapsTime::from_property(
            Property::new("DTSTART", "20000101").append_parameter(("VALUE", "DATE")),
        );

        assert!(
            dt.is_some(),
            "Both modes should parse DATE with explicit VALUE=DATE parameter"
        );

        assert!(
            matches!(dt, Some(DatePerhapsTime::Date(_))),
            "Expected DatePerhapsTime::Date variant"
        );
    }

    #[test]
    fn both_modes_accept_datetime() {
        let dt = DatePerhapsTime::from_property(&Property::new("DTSTART", "20000101T120000Z"));

        assert!(dt.is_some(), "Both modes should parse DATE-TIME values");

        assert!(
            matches!(dt, Some(DatePerhapsTime::DateTime(_))),
            "Expected DatePerhapsTime::DateTime variant"
        );
    }
}
