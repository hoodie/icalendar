use std::str::FromStr;

use crate::Parameter;

/// see 8.3.4. [Value Data Types Registry](https://tools.ietf.org/html/rfc5545#section-8.3.4)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ValueType {
    /// [`Binary`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.1)
    Binary,
    /// [`Boolean`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.2)
    Boolean,
    /// [`CalAddress`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.3)
    CalAddress,
    /// [`Date`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.4)
    Date,
    /// [`DateTime`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.5)
    DateTime,
    /// [`Duration`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.6)
    Duration,
    /// [`Float`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.7)
    Float,
    /// [`Integer`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.8)
    Integer,
    /// [`Period`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.9)
    Period,
    /// [`Recur`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.10)
    Recur,
    /// [`Text`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.11)
    Text,
    /// [`Time`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.12)
    Time,
    /// [`Uri`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.13)
    Uri,
    /// [`UtcOffset`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.14)
    UtcOffset,
}

impl ValueType {
    pub(crate) fn by_name(name: &str) -> Option<Self> {
        if name.chars().any(char::is_lowercase) {
            // eprintln!("property_name must be uppercase");
            return None;
        }
        use ValueType::*;
        match name {
            // 3.7.0 calendar properties
            "CALSCALE" => Some(Text), // 3.7.1
            "METHOD" => Some(Text),   // 3.7.2
            "PRODID" => Some(Text),   // 3.7.3
            "VERSION" => Some(Text),  //3.7.4

            // 3.8.0 component properties
            "ATTACH" => Some(Uri),               // or BINARY // 3.8.1.1
            "CATEGORIES" => Some(Text),          // 3.8.1.2
            "CLASS" => Some(Text),               // 3.8.1.3
            "COMMENT" => Some(Text),             // 3.8.1.4
            "DESCRIPTION" => Some(Text),         // 3.8.1.5
            "GEO" => Some(Float),                // 3.8.1.6
            "LOCATION" => Some(Text),            // 3.8.1.7
            "PERCENT-COMPLETE" => Some(Integer), // 3.8.1.8
            "PRIORITY" => Some(Integer),         // 3.8.1.9
            "RESOURCES" => Some(Text),           // 3.8.1.10
            "STATUS" => Some(Text),              // 3.8.1.11
            "SUMMARY" => Some(Text),             // 3.8.8.1.12
            "COMPLETED" => Some(DateTime),       // 3.8.2.1
            "DTEND" => Some(DateTime),           // or DATE // 3.8.2.2
            "DUE" => Some(DateTime),             // or DATE // 3.8.2.3
            "DTSTART" => Some(DateTime),         // or DATE 3.8.2.4
            "DURATION" => Some(Duration),        // 3.8.2.5
            "FREEBUSY" => Some(Period),          // 3.8.2.6
            "TRANSP" => Some(Text),              // 3.8.2.7
            "TZID" => Some(Text),                // 3.8.3.1
            "TZNAME" => Some(Text),              // 3.8.3.2
            "TZOFFSETFROM" => Some(UtcOffset),   //
            "TZOFFSETTO" => Some(UtcOffset),     //
            "TZURL" => Some(Uri),                //
            "ATTENDEE" => Some(CalAddress),      // 3.8.4.1
            "CONTACT" => Some(Text),             // 3.8.4.2
            "ORGANIZER" => Some(CalAddress),     // 3.8.4.3
            "RECURRENCE-ID" => Some(DateTime),   // 3.8.4.4
            "RELATED-TO" => Some(Text),          //
            "URL" => Some(Uri),                  //
            "UID" => Some(Text),                 //
            "EXDATE" => Some(DateTime),          // or DATE-TIME //
            "RDATE" => Some(DateTime),           // or PERIOD // or DATE-TIME // or DATE //
            "RRULE" => Some(Recur),              //
            "ACTION" => Some(Text),              //
            "REPEAT" => Some(Integer),           //
            "TRIGGER" => Some(Duration),         // or DATE-TIME (must be UTC) // 3.8.6.3
            "CREATED" => Some(DateTime),         // 3.8.7.1
            "DTSTAMP" => Some(DateTime),         //  3.8.7.2
            "LAST-MODIFIED" => Some(DateTime),   // 3.8.7.3
            "SEQUENCE" => Some(Integer),         // 3.8.7.4
            // 3.8.8.1
            // "An IANA-registered property name" => Any parameter can be specified on this property.
            n if n.starts_with("X-") => Some(Text), // any type 3.8.8.2
            "REQUEST-STATUS" => Some(Text),         // 3.8.8.3
            _ => None,
        }
    }
}

impl TryFrom<Parameter> for ValueType {
    type Error = ();

    fn try_from(par: Parameter) -> Result<Self, Self::Error> {
        if par.key() != "VALUE" {
            Err(())
        } else {
            FromStr::from_str(par.value())
        }
    }
}

impl FromStr for ValueType {
    type Err = ();

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        match val {
            "BINARY" => Ok(Self::Binary),
            "BOOLEAN" => Ok(Self::Boolean),
            "CAL-ADDRESS" => Ok(Self::CalAddress),
            "DATE" => Ok(Self::Date),
            "DATE-TIME" => Ok(Self::DateTime),
            "DURATION" => Ok(Self::Duration),
            "FLOAT" => Ok(Self::Float),
            "INTEGER" => Ok(Self::Integer),
            "PERIOD" => Ok(Self::Period),
            "RECUR" => Ok(Self::Recur),
            "TEXT" => Ok(Self::Text),
            "TIME" => Ok(Self::Time),
            "URI" => Ok(Self::Uri),
            "UTC-OFFSET" => Ok(Self::UtcOffset),
            _ => Err(()),
        }
    }
}
