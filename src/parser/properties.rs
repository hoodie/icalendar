use std::{
    fmt::{self, Write},
    str::FromStr,
};

use crate::{parser::utils::valid_key_sequence_cow, properties::fold_line, value_types::ValueType};

use super::{
    parameters::{parameters, Parameter},
    parsed_string::ParseString,
    utils::property_key,
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::{line_ending, multispace0},
    combinator::{cut, opt},
    error::{context, ContextError, ParseError},
    sequence::{preceded, separated_pair, tuple},
    Finish, IResult, Parser,
};

#[cfg(test)]
use nom::error::ErrorKind;
use nom_language::error::{convert_error, VerboseError};

/// [RFC-5545](https://datatracker.ietf.org/doc/html/rfc5545) states that the following
/// "MAY occur more than once" in a VEVENT, VTODO, VJOURNAL, and VFREEBUSY.
/// Note: A VJOURNAL can also contain multiple DECRIPTION but this is not covered here.
const MULTIS: [&str; 13] = [
    "ATTACH",
    "ATTENDEE",
    "CATEGORIES",
    "COMMENT",
    "CONTACT",
    "EXDATE",
    "FREEBUSY",
    "IANA-PROP",
    "RDATE",
    "RELATED",
    "RESOURCES",
    "RSTATUS",
    "X-PROP",
];

/// Zero-copy version of [`crate::properties::Property`]
#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Property<'a> {
    pub name: ParseString<'a>,
    pub val: ParseString<'a>,
    pub params: Vec<Parameter<'a>>,
}

impl<'a> Property<'a> {
    pub fn new_ref(key: &'a str, val: &'a str) -> Property<'a> {
        Property {
            name: key.into(),
            val: val.into(),
            params: vec![],
        }
    }
}

impl Property<'_> {
    pub(crate) fn fmt_write<W: Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        // A nice starting capacity for the majority of content lines
        let mut line = String::with_capacity(150);

        write!(line, "{}", self.name.as_str())?;
        for Parameter { key, val } in &self.params {
            if let Some(val) = val {
                write!(line, ";{}={}", key.as_str(), val.as_str())?;
            } else {
                write!(line, ";{}", key.as_str())?;
            }
        }
        write!(line, ":{}", self.val.as_str())?;
        write_crlf!(out, "{}", fold_line(&line))?;
        Ok(())
    }

    pub(crate) fn is_multi_property(&self) -> bool {
        MULTIS.contains(&self.name.as_str())
    }
}

impl fmt::Display for Property<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_write(f)
    }
}

impl<'a> TryFrom<&'a str> for Property<'a> {
    type Error = String;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        property(input)
            .finish()
            .map(|(_, x)| x)
            .map_err(|e: VerboseError<&str>| format!("error: {}", convert_error(input, e.clone())))
    }
}

impl From<Property<'_>> for crate::Property {
    fn from(parsed: Property<'_>) -> Self {
        Self {
            key: parsed.name.as_ref().to_owned(),
            val: parsed.val.as_ref().to_owned(),
            params: parsed
                .params
                .into_iter()
                .map(|p| (p.key.as_ref().to_owned(), p.into()))
                .collect(),
        }
    }
}

impl FromStr for crate::Property {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Property::try_from(s)?.into())
    }
}

#[test]
fn properties() {
    assert_parser!(
        property,
        "KEY:VALUE\n",
        Property {
            name: "KEY".into(),
            val: "VALUE".into(),
            params: vec![]
        }
    );

    assert_parser!(
        property,
        "KEY1;foo=bar:VALUE\n",
        Property {
            name: "KEY1".into(),
            val: "VALUE".into(),
            params: vec![Parameter::new_ref("foo", Some("bar"))]
        }
    );

    assert_parser!(
        property,
        "KEY2;foo=bar:VALUE space separated\n",
        Property {
            name: "KEY2".into(),
            val: "VALUE space separated".into(),
            params: vec![Parameter::new_ref("foo", Some("bar"))]
        }
    );

    assert_parser!(
        property,
        "KEY2;foo=bar:important:VALUE\n",
        Property {
            name: "KEY2".into(),
            val: "important:VALUE".into(),
            params: vec![Parameter::new_ref("foo", Some("bar"))]
        }
    );
}

#[test]
fn unescape_text() {
    assert_eq!(
        "DESCRIPTION:https\\://example.com\n"
            .parse::<crate::Property>()
            .map(|p| p.val)
            .unwrap(),
        "https://example.com"
    );

    assert_parser!(
        property,
        "DESCRIPTION:https\\://example.com\n",
        Property {
            name: "DESCRIPTION".into(),
            val: "https://example.com".into(),
            params: Default::default()
        }
    );
}

#[test]
fn property_escape_url_as_url() {
    assert_eq!(
        "URL:https://example.com\n"
            .parse::<crate::Property>()
            .map(|p| p.val)
            .unwrap(),
        "https://example.com"
    );

    assert_parser!(
        property,
        "URL:https://example.com\n",
        Property {
            name: "URL".into(),
            val: "https://example.com".into(),
            params: Default::default()
        }
    );
}

#[test]
fn property_escape_description_as_text() {
    assert_parser!(
        property,
        "DESCRIPTION;foo=bar:VALUE\\n newline separated\n",
        Property {
            name: "DESCRIPTION".into(),
            val: "VALUE\n newline separated".into(),
            params: vec![Parameter::new_ref("foo", Some("bar"))]
        }
    );
}

#[test]
fn property_escape_by_value_param() {
    assert_parser!(
        property,
        "FOO;VALUE=TEXT:VALUE\\n newline separated\n",
        Property {
            name: "FOO".into(),
            val: "VALUE\n newline separated".into(),
            params: vec![Parameter::new_ref("VALUE", Some("TEXT"))]
        }
    );

    assert_parser!(
        property,
        "FOO_AS_TEXT;VALUE=TEXT:https\\://example.com\n",
        Property {
            name: "FOO_AS_TEXT".into(),
            val: "https://example.com".into(),
            params: vec![Parameter::new_ref("VALUE", Some("TEXT"))]
        }
    );
    assert_parser!(
        property,
        "FOO_AS_URL;VALUE=URL:https\\://example.com\n",
        Property {
            name: "FOO_AS_URL".into(),
            val: "https\\://example.com".into(),
            params: vec![Parameter::new_ref("VALUE", Some("URL"))]
        }
    );
}

#[test]
fn property_with_dash() {
    assert_parser!(
        property,
        "X-HOODIE-KEY:VALUE\n",
        Property {
            name: "X-HOODIE-KEY".into(),
            val: "VALUE".into(),
            params: vec![]
        }
    );
}

#[test]
fn parse_properties_from_rfc() {
    assert_parser!(
        property,
        "home.tel;type=fax,voice,msg:+49 3581 123456\n",
        Property {
            name: "home.tel".into(),
            val: "+49 3581 123456".into(),
            params: vec![Parameter::new_ref("type", Some("fax,voice,msg"),)]
        }
    );

    assert_parser!(
        property,
        "email;internet:mb@goerlitz.de\n",
        Property {
            name: "email".into(),
            val: "mb@goerlitz.de".into(),
            params: vec![Parameter::new_ref("internet", None,)]
        }
    );
}

#[test]
fn parse_property_with_breaks() {
    let sample_0 =
        "DESCRIPTION:Hey, I'm gonna have a party\\n BYOB: Bring your own beer.\\n Hendrik\\n\n";

    let expectation = Property {
        name: "DESCRIPTION".into(),
        val: "Hey, I'm gonna have a party\n BYOB: Bring your own beer.\n Hendrik\n".into(),
        params: vec![],
    };

    assert_parser!(property, sample_0, expectation);
}

#[test]
fn parse_invalid_property() {
    let sample_0 = "END;RELTYPE=:c605e4e8-8ea3-4315-b139-19394ab3ced6\n";
    use nom::error::ErrorKind::*;
    use nom_language::error::VerboseErrorKind::*;
    pretty_assertions::assert_eq!(
        property::<VerboseError<&str>>(sample_0),
        Err(nom::Err::Failure(VerboseError {
            errors: vec![
                (
                    "END;RELTYPE=:c605e4e8-8ea3-4315-b139-19394ab3ced6\n",
                    Nom(Satisfy)
                ),
                (
                    "END;RELTYPE=:c605e4e8-8ea3-4315-b139-19394ab3ced6\n",
                    Context("property cannot be END or BEGIN")
                ),
                (
                    "END;RELTYPE=:c605e4e8-8ea3-4315-b139-19394ab3ced6\n",
                    Context("property")
                )
            ]
        }))
    );
}

#[test]
fn parse_property_with_colon() {
    let sample_0 = "RELATED-TO;RELTYPE=:c605e4e8-8ea3-4315-b139-19394ab3ced6\n";

    let expectation = Property {
        name: "RELATED-TO".into(),
        val: "c605e4e8-8ea3-4315-b139-19394ab3ced6".into(),
        params: vec![Parameter {
            key: "RELTYPE".into(),
            val: None,
        }],
    };

    assert_parser!(property, sample_0, expectation);
}

#[test]
fn parse_property_with_no_value() {
    let sample_0 = "X-NO-VALUE";

    let expectation = Property {
        name: "X-NO-VALUE".into(),
        val: "".into(),
        params: vec![],
    };

    assert_parser!(property, sample_0, expectation);
}

fn determin_value_type(name: &ParseString, params: &[Parameter]) -> Option<ValueType> {
    params
        .iter()
        .find(|param| param.key == "VALUE")
        .and_then(|Parameter { val, .. }| val.as_ref())
        .and_then(|typ| ValueType::from_str(typ.as_str()).ok())
        .or_else(|| ValueType::by_name(name.as_str()))
}

pub fn property<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Property<'a>, E> {
    context(
        "property",
        cut(tuple((
            alt((
                separated_pair(
                    tuple((
                        // preceded(multispace0, alpha_or_dash), // key
                        cut(context(
                            // this must be interpreted as component by `component()`
                            // if you get here at all then the parser is in a wrong state
                            "property cannot be END or BEGIN",
                            preceded(multispace0, property_key).map(ParseString::from),
                        )), // key
                        parameters, // params
                    )),
                    context("property separator", tag(":")), // separator
                    context(
                        "property value",
                        alt((
                            take_until("\r\n"),
                            take_until("\n"),
                            // this is for single line prop parsing, just so I can leave off the '\n'
                            take_while(|_| true),
                        ))
                        .map(ParseString::from),
                    ),
                ),
                context(
                    "no-value property",
                    valid_key_sequence_cow.map(|key| ((key, vec![]), ParseString::from(""))), // key and nothing else
                ),
            )),
            opt(line_ending),
        ))
        .map(|(((name, params), val), _)| {
            let val = if let Some(value_type) = determin_value_type(&name, &params) {
                val.unescape_by_value_type(value_type)
            } else {
                val
            };

            Property { name, val, params }
        })),
    )
    .parse(input)
}
