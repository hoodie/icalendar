use std::{
    collections::BTreeMap,
    fmt::{self, Write},
    mem,
    str::FromStr,
};

use crate::value_types::ValueType;

#[derive(Clone, Debug, PartialEq, Eq)]
/// key-value pairs inside of `Property`s
pub struct Parameter {
    key: String,
    val: String,
}

impl Parameter {
    /// Creates a new `Parameter`
    pub fn new(key: &str, val: &str) -> Self {
        Parameter {
            key: key.to_owned(),
            val: val.to_owned(),
        }
    }

    /// Returns a reference to the key field.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns a reference to the value field.
    pub fn value(&self) -> &str {
        &self.val
    }
}

impl From<(&str, &str)> for Parameter {
    fn from((key, val): (&str, &str)) -> Self {
        Parameter::new(key, val)
    }
}

pub type EntryParameters = BTreeMap<String, Parameter>;

#[derive(Clone, Debug, PartialEq, Eq)]
/// key-value pairs inside of `Component`s
pub struct Property {
    pub(crate) key: String,
    pub(crate) val: String,
    pub(crate) params: EntryParameters,
}

impl From<(&str, &str)> for Property {
    fn from((key, val): (&str, &str)) -> Self {
        Property::new(key, val)
    }
}

impl From<&mut Property> for Property {
    fn from(val: &mut Property) -> Self {
        val.to_owned()
    }
}

impl Property {
    /// Guess what this does :D
    pub fn new(key: impl Into<String>, val: impl Into<String>) -> Self {
        Property {
            key: key.into(),
            val: val.into(),
            params: Default::default(),
        }
    }

    #[deprecated]
    /// if you already have `String`s I'll gladly take
    pub fn new_pre_alloc(key: String, val: String) -> Self {
        Property {
            key,
            val,
            params: Default::default(),
        }
    }

    /// Returns a reference to the key field.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns a reference to the value field.
    pub fn value(&self) -> &str {
        &self.val
    }

    /// Returns a reference to the parameters.
    pub fn params(&self) -> &EntryParameters {
        &self.params
    }

    /// Produces a `Vec` of `Property` from an array of other types.
    pub fn from_array<P: Into<Property>, const N: usize>(array: [P; N]) -> Vec<Property> {
        array.into_iter().map(Into::into).collect::<Vec<_>>()
    }

    /// Returns the `VALUE` parameter, if any is specified.
    pub fn value_type(&self) -> Option<ValueType> {
        self.params
            .get("VALUE")
            .and_then(|p| ValueType::from_str(&p.val).ok())
            .or_else(|| ValueType::by_name(self.key()))
    }

    // /// Returns the value as a certain type
    // pub fn get_value<T>(&self) -> Result<T, E>
    // where
    //     T: std::str::FromStr,
    //     E: std::error::Error,
    //     <T as std::str::FromStr::Err>: E
    // {
    //     T::from_str(&self.val).ok()
    // }

    /// Returns the value as a certain type
    pub fn get_value_as<F, T>(&self, converter: F) -> Option<T>
    where
        F: Fn(&str) -> Option<T>,
    {
        converter(&self.val)
    }

    /// Returns the value of a parameter as a certain type
    pub fn get_param_as<F, T>(&self, key: &str, converter: F) -> Option<T>
    where
        F: Fn(&str) -> Option<T>,
    {
        self.params.get(key).and_then(|param| converter(&param.val))
    }

    /// Appends a new parameter.
    pub fn append_parameter<I: Into<Parameter>>(&mut self, into_parameter: I) -> &mut Self {
        let parameter = into_parameter.into();
        self.params.insert(parameter.key.clone(), parameter);
        self
    }

    /// Creates and appends a parameter.
    pub fn add_parameter(&mut self, key: &str, val: &str) -> &mut Self {
        self.append_parameter(Parameter::new(key, val));
        self
    }

    /// End of Builder Pattern.
    pub fn done(&mut self) -> Self {
        Property {
            key: mem::take(&mut self.key),
            val: mem::take(&mut self.val),
            params: mem::take(&mut self.params),
        }
    }

    /// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.11>
    fn escape_text(input: &str) -> String {
        input
            .replace('\\', r#"\\"#)
            .replace(',', r#"\,"#)
            .replace(';', r#"\;"#)
            .replace('\n', r#"\n"#)
    }

    fn quote_if_contains_colon(input: &str) -> String {
        if input.contains([':', ';']) {
            let mut quoted = String::with_capacity(input.len() + 2);
            quoted.push('"');
            quoted.push_str(input);
            quoted.push('"');
            quoted
        } else {
            input.to_string()
        }
    }

    /// Writes this Property to `out`
    pub(crate) fn fmt_write<W: Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        // A nice starting capacity for the majority of content lines
        let mut line = String::with_capacity(150);

        write!(line, "{}", self.key)?;
        for Parameter { key, val } in self.params.values() {
            write!(line, ";{}={}", key, Self::quote_if_contains_colon(val))?;
        }
        let value_type = self.value_type();
        match value_type {
            Some(ValueType::Text) => write!(line, ":{}", Self::escape_text(&self.val))?,
            _ => write!(line, ":{}", self.val)?,
        }
        write_crlf!(out, "{}", fold_line(&line))?;
        Ok(())
    }
}

impl TryInto<String> for Property {
    type Error = fmt::Error;

    fn try_into(self) -> Result<String, Self::Error> {
        let mut out_string = String::new();
        self.fmt_write(&mut out_string)?;
        Ok(out_string)
    }
}

/// This property defines the access classification for a calendar component.
/// [RFC 5545, Section 3.8.1.3](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.3)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Class {
    /// [`Public`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.3)
    Public,
    /// [`Private`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.3)
    Private,
    /// [`Confidential`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.3)
    Confidential,
}

impl Class {
    pub(crate) fn from_str(s: &str) -> Option<Self> {
        match s {
            "PUBLIC" => Some(Self::Public),
            "PRIVATE" => Some(Self::Private),
            "CONFIDENTIAL" => Some(Self::Confidential),
            _ => None,
        }
    }
}

impl From<Class> for Property {
    fn from(val: Class) -> Self {
        Property {
            key: String::from("CLASS"),
            val: String::from(match val {
                Class::Public => "PUBLIC",
                Class::Private => "PRIVATE",
                Class::Confidential => "CONFIDENTIAL",
            }),
            params: Default::default(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// Encodes the status of an `Event`
/// [RFC 5545, Section 3.8.1.11](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.11)
pub enum EventStatus {
    /// Indicates event is tentative.
    Tentative,
    /// Indicates event is definite.
    Confirmed,
    /// Indicates event was cancelled.
    Cancelled,
    //Custom(&str)
}

impl EventStatus {
    pub(crate) fn from_str(s: &str) -> Option<Self> {
        match s {
            "TENTATIVE" => Some(Self::Tentative),
            "CONFIRMED" => Some(Self::Confirmed),
            "CANCELLED" => Some(Self::Cancelled),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// Encodes the status of a `Todo`
/// [RFC 5545, Section 3.8.1.11](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.11)
pub enum TodoStatus {
    /// Indicates to-do needs action.
    NeedsAction,
    /// Indicates to-do is completed.
    Completed,
    /// Indicates to-do is in process.
    InProcess,
    /// Indicates to-do was cancelled.
    Cancelled,
    //Custom(&str)
}

impl TodoStatus {
    pub(crate) fn from_str(s: &str) -> Option<Self> {
        match s {
            "NEEDS-ACTION" => Some(Self::NeedsAction),
            "COMPLETED" => Some(Self::Completed),
            "IN-PROCESS" => Some(Self::InProcess),
            "CANCELLED" => Some(Self::Cancelled),
            _ => None,
        }
    }
}

//pub enum JournalStatus{
//    Draft,
//    Final,
//    Cancelled,
//    Custom(&str)
//}

impl From<EventStatus> for Property {
    fn from(val: EventStatus) -> Self {
        Property {
            key: String::from("STATUS"),
            val: String::from(match val {
                EventStatus::Tentative => "TENTATIVE",
                EventStatus::Confirmed => "CONFIRMED",
                EventStatus::Cancelled => "CANCELLED",
            }),
            params: Default::default(),
        }
    }
}

// TODO: why do we add this?
impl From<ValueType> for Parameter {
    fn from(val: ValueType) -> Self {
        Parameter {
            key: String::from("VALUE"),
            val: String::from(match val {
                ValueType::Binary => "BINARY",
                ValueType::Boolean => "BOOLEAN",
                ValueType::CalAddress => "CAL-ADDRESS",
                ValueType::Date => "DATE",
                ValueType::DateTime => "DATE-TIME",
                ValueType::Duration => "DURATION",
                ValueType::Float => "FLOAT",
                ValueType::Integer => "INTEGER",
                ValueType::Period => "PERIOD",
                ValueType::Recur => "RECUR",
                ValueType::Text => "TEXT",
                ValueType::Time => "TIME",
                ValueType::Uri => "URI",
                ValueType::UtcOffset => "UTC-OFFSET",
            }),
        }
    }
}

impl From<TodoStatus> for Property {
    fn from(val: TodoStatus) -> Self {
        Property::new(
            "STATUS",
            match val {
                TodoStatus::NeedsAction => "NEEDS-ACTION",
                TodoStatus::Completed => "COMPLETED",
                TodoStatus::InProcess => "IN-PROCESS",
                TodoStatus::Cancelled => "CANCELLED",
                //TodoStatus::Custom(s)   => "CU",
            },
        )
    }
}

impl From<chrono::Duration> for Property {
    fn from(duration: chrono::Duration) -> Self {
        Property::new("DURATION", duration.to_string())
    }
}
//pub enum AttendeeRole {
//    /// CHAIR           (RFC 5545, Section 3.2.16)
//    Chair,
//
//    /// REQ-PARTICIPANT (RFC 5545, Section 3.2.16)
//    ReqParticipant,
//
//    /// OPT-PARTICIPANT (RFC 5545, Section 3.2.16)
//    OptParticipant,
//
//    /// NON-PARTICIPANT (RFC 5545, Section 3.2.16)
//    NonParticipant
//}
//
//pub struct Attendee {
//    cn: String,
//    role: AttendeeRole,
//    delegated_from: String,
//    part_stat: String,
//    sent_by: String,
//    dir: String,
//}
//
//impl Into<Property> for Attendee {
//}

// Fold a content line as described in RFC 5545, Section 3.1
#[allow(clippy::indexing_slicing)]
pub(crate) fn fold_line(line: &str) -> String {
    const LIMIT: usize = 75;
    let len = line.len();
    let mut ret = String::with_capacity(len + (len / LIMIT * 3));
    let mut bytes_remaining = len;

    let mut pos = 0;
    let mut next_pos = LIMIT;

    while bytes_remaining > LIMIT {
        let pos_is_whitespace = |line: &str, next_pos| {
            line.chars()
                .nth(next_pos)
                .map(char::is_whitespace)
                .unwrap_or(false)
        };
        if pos_is_whitespace(line, next_pos) {
            next_pos -= 1;
        }

        while !line.is_char_boundary(next_pos) {
            next_pos -= 1;
            if pos_is_whitespace(line, next_pos) {
                next_pos -= 1;
            }
        }
        ret.push_str(&line[pos..next_pos]);
        ret.push_str("\r\n ");

        bytes_remaining -= next_pos - pos;
        pos = next_pos;
        next_pos += LIMIT - 1;
    }

    ret.push_str(&line[len - bytes_remaining..]);
    ret
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn fold_line_short() {
        let line = "This is a short line";
        assert_eq!(line, fold_line(line));
    }

    #[test]
    fn fold_line_folds_on_char_boundary() {
        let line = "Content lines shouldn't be folded in the middle \
             of a UTF-8 character. 老虎.";

        let expected = "Content lines shouldn't be folded in the middle \
             of a UTF-8 character. 老\r\n 虎.";
        assert_eq!(expected, fold_line(line));
    }

    #[test]
    fn escape_special_characters_in_text() {
        let line = "\n\\;,:";

        let expected = r"\n\\\;\,:";
        assert_eq!(expected, Property::escape_text(line));
    }

    #[test]
    fn escape_special_characters_in_serialized_property() {
        let line = "\n\\;,:";
        let expected = r"\n\\\;\,:";

        let prop = Property::new("DESCRIPTION", line)
            .append_parameter(("VALUE", "TEXT"))
            .done();
        let expected = format!("DESCRIPTION;VALUE=TEXT:{expected}\r\n");

        let mut buf = String::new();
        prop.fmt_write(&mut buf).unwrap();
        assert_eq!(expected, buf);
    }

    #[cfg(feature = "parser")]
    #[test]
    fn preserve_spaces() {
        use crate::parser::unfold;
        let lines = [
            r#"01234567890123456789012345678901234567890123456789012345HERE_COMES_A_SPACE( )"#,
            r#"01234567890123456789012345678901234567890123456789012345HERE_COMES_A_SPACE( )<-----78901234567890123456789012345678901234567890123HERE_COMES_A_SPACE( )<---"#,
        ];
        for line in lines {
            let folded = fold_line(line);
            let unfolded = unfold(&folded);

            assert_eq!(line, unfolded);
        }
    }

    #[test]
    fn serialize_property() {
        let prop = Property::new("SUMMARY", "This is a summary");
        let expected = "SUMMARY:This is a summary\r\n";
        let mut buf = String::new();
        prop.fmt_write(&mut buf).unwrap();
        assert_eq!(expected, buf);
    }
}
