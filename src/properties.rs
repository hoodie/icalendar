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
    pub(crate) fn to_line(&self) -> Result<String, fmt::Error> {
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
        Ok(line)
    }

    /// Writes this Property to `out`
    pub(crate) fn fmt_write<W: Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        let line = self.to_line()?;
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

/// [RFC 5545, Section 3.2.3](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.3)
/// Calendar User Type (CUTYPE)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CUType {
    /// INDIVIDUAL  (RFC 5545, Section 3.2.3)
    Individual,
    /// GROUP       (RFC 5545, Section 3.2.3)
    Group,
    /// RESOURCE    (RFC 5545, Section 3.2.3)
    Resource,
    /// ROOM        (RFC 5545, Section 3.2.3)
    Room,
    /// UNKNOWN     (RFC 5545, Section 3.2.3)
    Unknown,
}

impl CUType {
    pub(crate) fn from_str(s: &str) -> Option<Self> {
        match s {
            "INDIVIDUAL" => Some(CUType::Individual),
            "GROUP" => Some(CUType::Group),
            "RESOURCE" => Some(CUType::Resource),
            "ROOM" => Some(CUType::Room),
            "UNKNOWN" => Some(CUType::Unknown),
            _ => None,
        }
    }
}

impl From<CUType> for Parameter {
    fn from(cutype: CUType) -> Self {
        Parameter::new(
            "CUTYPE",
            match cutype {
                CUType::Individual => "INDIVIDUAL",
                CUType::Group => "GROUP",
                CUType::Resource => "RESOURCE",
                CUType::Room => "ROOM",
                CUType::Unknown => "UNKNOWN",
            },
        )
    }
}

/// [RFC 5545, Section 3.2.16](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.16)
/// Participation Role (ROLE)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    /// CHAIR           (RFC 5545, Section 3.2.16)
    Chair,
    /// REQ-PARTICIPANT (RFC 5545, Section 3.2.16)
    ReqParticipant,
    /// OPT-PARTICIPANT (RFC 5545, Section 3.2.16)
    OptParticipant,
    /// NON-PARTICIPANT (RFC 5545, Section 3.2.16)
    NonParticipant,
}

impl Role {
    pub(crate) fn from_str(s: &str) -> Option<Self> {
        match s {
            "CHAIR" => Some(Role::Chair),
            "REQ-PARTICIPANT" => Some(Role::ReqParticipant),
            "OPT-PARTICIPANT" => Some(Role::OptParticipant),
            "NON-PARTICIPANT" => Some(Role::NonParticipant),
            _ => None,
        }
    }
}

impl From<Role> for Parameter {
    fn from(role: Role) -> Self {
        Parameter::new(
            "ROLE",
            match role {
                Role::Chair => "CHAIR",
                Role::ReqParticipant => "REQ-PARTICIPANT",
                Role::OptParticipant => "OPT-PARTICIPANT",
                Role::NonParticipant => "NON-PARTICIPANT",
            },
        )
    }
}

/// [RFC 5545, Section 3.2.12](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.12)
/// Participation Status (PARTSTAT)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PartStat {
    /// NEEDS-ACTION    (RFC 5545, Section 3.2.12)
    NeedsAction,
    /// ACCEPTED        (RFC 5545, Section 3.2.12)
    Accepted,
    /// DECLINED        (RFC 5545, Section 3.2.12)
    Declined,
    /// TENTATIVE       (RFC 5545, Section 3.2.12)
    Tentative,
    /// DELEGATED       (RFC 5545, Section 3.2.12)
    Delegated,
    /// COMPLETED       (RFC 5545, Section 3.2.12)
    Completed,
    /// TENTATIVE       (RFC 5545, Section 3.2.12)
    InProcess,
}

impl PartStat {
    pub(crate) fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "NEEDS-ACTION" => Some(PartStat::NeedsAction),
            "ACCEPTED" => Some(PartStat::Accepted),
            "DECLINED" => Some(PartStat::Declined),
            "TENTATIVE" => Some(PartStat::Tentative),
            "DELEGATED" => Some(PartStat::Delegated),
            "COMPLETED" => Some(PartStat::Completed),
            "IN-PROCESS" => Some(PartStat::InProcess),
            _ => None,
        }
    }
}

impl From<PartStat> for Parameter {
    fn from(partstat: PartStat) -> Self {
        Parameter::new(
            "PARTSTAT",
            match partstat {
                PartStat::NeedsAction => "NEEDS-ACTION",
                PartStat::Accepted => "ACCEPTED",
                PartStat::Declined => "DECLINED",
                PartStat::Tentative => "TENTATIVE",
                PartStat::Delegated => "DELEGATED",
                PartStat::Completed => "COMPLETED",
                PartStat::InProcess => "IN-PROCESS",
            },
        )
    }
}

/// [RFC 5545, Section 3.8.4.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.1)
/// Attendee (ATTENDEE)
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Attendee {
    /// [RFC 5545, Section 3.2.2](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.2)
    /// Common Name
    pub cn: Option<String>,
    /// [RFC 5545, Section 3.2.3](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.3)
    /// Calendar User Type
    pub cutype: Option<CUType>,
    /// [RFC 5545, Section 3.2.4](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.4)
    /// Delegators (list of URIs)
    pub delegated_from: Vec<String>,
    /// [RFC 5545, Section 3.2.5](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.5)
    /// Delegatees (list of URIs)
    pub delegated_to: Vec<String>,
    /// [RFC 5545, Section 3.2.6](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.6)
    /// Directory Entry Reference (URI)
    pub dir: Option<String>,
    /// [RFC 5545, Section 3.2.10](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.10)
    /// Language
    pub language: Option<String>,
    /// [RFC 5545, Section 3.2.11](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.11)
    /// Member (list of URIs)
    pub member: Vec<String>,
    /// [RFC 5545, Section 3.2.12](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.12)
    /// Participation Status
    pub part_stat: Option<PartStat>,
    /// [RFC 5545, Section 3.2.16](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.16)
    /// Participation Role
    pub role: Option<Role>,
    /// [RFC 5545, Section 3.2.17](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.17)
    /// RSVP expectation
    pub rsvp: Option<bool>,
    /// [RFC 5545, Section 3.2.18](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.18)
    /// Sent By (URI)
    pub sent_by: Option<String>,
    /// [RFC 5545, Section 3.3.3](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.3)
    /// The attendee's CAL-ADDRESS (e.g., "mailto:user@example.com")
    pub cal_address: String,
}

impl Attendee {
    /// Create a new Attendee with just the CAL-ADDRESS (e.g., "mailto:user@example.com").
    pub fn new(cal_address: String) -> Self {
        Self {
            cal_address,
            ..Default::default()
        }
    }

    /// Set the CUTYPE.
    pub fn cutype(mut self, cutype: CUType) -> Self {
        self.cutype = Some(cutype);
        self
    }

    /// Add a MEMBER.
    pub fn member(mut self, member: String) -> Self {
        self.member.push(member);
        self
    }

    /// Set the ROLE.
    pub fn role(mut self, role: Role) -> Self {
        self.role = Some(role);
        self
    }

    /// Set the PARTSTAT.
    pub fn partstat(mut self, partstat: PartStat) -> Self {
        self.part_stat = Some(partstat);
        self
    }

    /// Set RSVP.
    pub fn rsvp(mut self, rsvp: bool) -> Self {
        self.rsvp = Some(rsvp);
        self
    }

    /// Add a DELEGATED-TO.
    pub fn delegated_to(mut self, delegated_to: String) -> Self {
        self.delegated_to.push(delegated_to);
        self
    }

    /// Add a DELEGATED-FROM.
    pub fn delegated_from(mut self, delegated_from: String) -> Self {
        self.delegated_from.push(delegated_from);
        self
    }

    /// Set SENT-BY.
    pub fn sentby(mut self, sentby: String) -> Self {
        self.sent_by = Some(sentby);
        self
    }

    /// Set CN.
    pub fn cn(mut self, cn: String) -> Self {
        self.cn = Some(cn);
        self
    }

    /// Set DIR.
    pub fn dir(mut self, dir: String) -> Self {
        self.dir = Some(dir);
        self
    }

    /// Set LANGUAGE.
    pub fn language(mut self, language: String) -> Self {
        self.language = Some(language);
        self
    }
}

impl From<Attendee> for Property {
    fn from(attendee: Attendee) -> Self {
        let mut prop = Property::new("ATTENDEE", attendee.cal_address);

        if let Some(cutype) = attendee.cutype {
            prop.append_parameter(cutype);
        }
        if !attendee.member.is_empty() {
            prop.add_parameter("MEMBER", &attendee.member.join(","));
        }
        if let Some(role) = attendee.role {
            prop.append_parameter(role);
        }
        if let Some(partstat) = attendee.part_stat {
            prop.append_parameter(partstat);
        }
        if let Some(rsvp) = attendee.rsvp {
            prop.add_parameter("RSVP", if rsvp { "TRUE" } else { "FALSE" });
        }
        if !attendee.delegated_to.is_empty() {
            prop.add_parameter("DELEGATED-TO", &attendee.delegated_to.join(","));
        }
        if !attendee.delegated_from.is_empty() {
            prop.add_parameter("DELEGATED-FROM", &attendee.delegated_from.join(","));
        }
        if let Some(sentby) = attendee.sent_by {
            prop.add_parameter("SENT-BY", &sentby);
        }
        if let Some(cn) = attendee.cn {
            prop.add_parameter("CN", &cn);
        }
        if let Some(dir) = attendee.dir {
            prop.add_parameter("DIR", &dir);
        }
        if let Some(language) = attendee.language {
            prop.add_parameter("LANGUAGE", &language);
        }

        prop.done()
    }
}

impl TryFrom<&Property> for Attendee {
    type Error = ();

    fn try_from(prop: &Property) -> Result<Self, Self::Error> {
        if prop.key() != "ATTENDEE" {
            return Err(());
        }
        let value = prop.value().to_string();

        let cutype = prop.get_param_as("CUTYPE", CUType::from_str);
        let member = prop
            .get_param_as("MEMBER", |s| {
                Some(s.split(',').map(|s| s.trim().to_string()).collect())
            })
            .unwrap_or_default();
        let role = prop.get_param_as("ROLE", Role::from_str);
        let partstat = prop.get_param_as("PARTSTAT", PartStat::from_str);
        let rsvp = prop.get_param_as("RSVP", |s| match s.to_uppercase().as_str() {
            "TRUE" => Some(true),
            "FALSE" => Some(false),
            _ => None,
        });
        let delegated_to = prop
            .get_param_as("DELEGATED-TO", |s| {
                Some(s.split(',').map(|s| s.trim().to_string()).collect())
            })
            .unwrap_or_default();
        let delegated_from = prop
            .get_param_as("DELEGATED-FROM", |s| {
                Some(s.split(',').map(|s| s.trim().to_string()).collect())
            })
            .unwrap_or_default();
        let sentby = prop.get_param_as("SENT-BY", |s| Some(s.to_string()));
        let cn = prop.get_param_as("CN", |s| Some(s.to_string()));
        let dir = prop.get_param_as("DIR", |s| Some(s.to_string()));
        let language = prop.get_param_as("LANGUAGE", |s| Some(s.to_string()));

        Ok(Attendee {
            cal_address: value,
            cutype,
            member,
            role,
            part_stat: partstat,
            rsvp,
            delegated_to,
            delegated_from,
            sent_by: sentby,
            cn,
            dir,
            language,
        })
    }
}

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

    #[test]
    fn attendee_to_property_basic() {
        let attendee = Attendee::new("mailto:test@example.com".to_string());
        let prop: Property = attendee.into();

        assert_eq!(prop.key(), "ATTENDEE");
        assert_eq!(prop.value(), "mailto:test@example.com");
        assert!(prop.params().is_empty());
    }

    #[test]
    fn attendee_to_property_full() {
        let attendee = Attendee::new("mailto:test@example.com".to_string())
            .cutype(CUType::Individual)
            .role(Role::ReqParticipant)
            .partstat(PartStat::Accepted)
            .rsvp(true)
            .cn("Test User".to_string())
            .member("mailto:member1@example.com".to_string())
            .member("mailto:member2@example.com".to_string())
            .delegated_to("mailto:delegate@example.com".to_string())
            .sentby("mailto:sender@example.com".to_string())
            .dir("ldap://example.com/cn=Test%20User".to_string())
            .language("en".to_string());

        let prop: Property = attendee.into();

        assert_eq!(prop.key(), "ATTENDEE");
        assert_eq!(prop.value(), "mailto:test@example.com");

        // Check parameters
        assert_eq!(prop.params().get("CUTYPE").unwrap().value(), "INDIVIDUAL");
        assert_eq!(prop.params().get("ROLE").unwrap().value(), "REQ-PARTICIPANT");
        assert_eq!(prop.params().get("PARTSTAT").unwrap().value(), "ACCEPTED");
        assert_eq!(prop.params().get("RSVP").unwrap().value(), "TRUE");
        assert_eq!(prop.params().get("CN").unwrap().value(), "Test User");
        assert_eq!(
            prop.params().get("MEMBER").unwrap().value(),
            "mailto:member1@example.com,mailto:member2@example.com"
        );
        assert_eq!(
            prop.params().get("DELEGATED-TO").unwrap().value(),
            "mailto:delegate@example.com"
        );
        assert_eq!(prop.params().get("SENT-BY").unwrap().value(), "mailto:sender@example.com");
        assert_eq!(
            prop.params().get("DIR").unwrap().value(),
            "ldap://example.com/cn=Test%20User"
        );
        assert_eq!(prop.params().get("LANGUAGE").unwrap().value(), "en");
    }

    #[test]
    fn property_to_attendee_basic() {
        let prop = Property::new("ATTENDEE", "mailto:test@example.com").done();
        let attendee = Attendee::try_from(&prop).unwrap();

        assert_eq!(attendee.cal_address, "mailto:test@example.com");
        assert!(attendee.cutype.is_none());
        assert!(attendee.role.is_none());
        assert!(attendee.part_stat.is_none());
        assert!(attendee.rsvp.is_none());
        assert!(attendee.cn.is_none());
        assert!(attendee.member.is_empty());
        assert!(attendee.delegated_to.is_empty());
        assert!(attendee.sent_by.is_none());
        assert!(attendee.dir.is_none());
        assert!(attendee.language.is_none());
    }

    #[test]
    fn property_to_attendee_full() {
        let prop = Property::new("ATTENDEE", "mailto:test@example.com")
            .add_parameter("CUTYPE", "INDIVIDUAL")
            .add_parameter("ROLE", "REQ-PARTICIPANT")
            .add_parameter("PARTSTAT", "ACCEPTED")
            .add_parameter("RSVP", "TRUE")
            .add_parameter("CN", "Test User")
            .add_parameter("MEMBER", "mailto:member1@example.com,mailto:member2@example.com")
            .add_parameter("DELEGATED-TO", "mailto:delegate@example.com")
            .add_parameter("DELEGATED-FROM", "mailto:delegator@example.com")
            .add_parameter("SENT-BY", "mailto:sender@example.com")
            .add_parameter("DIR", "ldap://example.com/cn=Test%20User")
            .add_parameter("LANGUAGE", "en")
            .done();

        let attendee = Attendee::try_from(&prop).unwrap();

        assert_eq!(attendee.cal_address, "mailto:test@example.com");
        assert_eq!(attendee.cutype, Some(CUType::Individual));
        assert_eq!(attendee.role, Some(Role::ReqParticipant));
        assert_eq!(attendee.part_stat, Some(PartStat::Accepted));
        assert_eq!(attendee.rsvp, Some(true));
        assert_eq!(attendee.cn, Some("Test User".to_string()));
        assert_eq!(
            attendee.member,
            vec!["mailto:member1@example.com".to_string(), "mailto:member2@example.com".to_string()]
        );
        assert_eq!(
            attendee.delegated_to,
            vec!["mailto:delegate@example.com".to_string()]
        );
        assert_eq!(
            attendee.delegated_from,
            vec!["mailto:delegator@example.com".to_string()]
        );
        assert_eq!(attendee.sent_by, Some("mailto:sender@example.com".to_string()));
        assert_eq!(attendee.dir, Some("ldap://example.com/cn=Test%20User".to_string()));
        assert_eq!(attendee.language, Some("en".to_string()));
    }

    #[test]
    fn attendee_try_from_invalid_property() {
        let prop = Property::new("NOT_ATTENDEE", "mailto:test@example.com").done();
        assert!(Attendee::try_from(&prop).is_err());
    }

    #[test]
    fn attendee_roundtrip() {
        let original = Attendee::new("mailto:roundtrip@example.com".to_string())
            .cutype(CUType::Resource)
            .role(Role::OptParticipant)
            .partstat(PartStat::Declined)
            .rsvp(true)
            .cn("Roundtrip User".to_string())
            .member("mailto:rt_member@example.com".to_string())
            .delegated_to("mailto:rt_delegate@example.com".to_string())
            .sentby("mailto:rt_sender@example.com".to_string())
            .dir("ldap://rt.example.com".to_string())
            .language("de".to_string());

        let prop: Property = original.clone().into();
        let reconstructed = Attendee::try_from(&prop).unwrap();

        assert_eq!(original, reconstructed);
    }
}