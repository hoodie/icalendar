use nom::{
    Finish, IResult, Parser,
    branch::alt,
    bytes::complete::{is_not, tag, take_till1},
    character::complete::space0,
    combinator::{eof, opt},
    error::{ContextError, ParseError},
    multi::many0,
    sequence::{delimited, preceded, separated_pair, tuple},
};

#[cfg(test)]
use nom::error::ErrorKind;
use nom_language::error::{VerboseError, convert_error};

use super::{parsed_string::ParseString, utils::valid_key_sequence_cow};

/// Zero-copy version of [`crate::properties::Parameter`]
#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Parameter<'a> {
    pub key: ParseString<'a>,
    pub val: Option<ParseString<'a>>,
}

impl<'a> Parameter<'a> {
    pub fn new_ref(key: &'a str, val: Option<&'a str>) -> Parameter<'a> {
        Parameter {
            key: key.into(),
            val: val.map(Into::into),
        }
    }
}

impl<'i> TryFrom<&'i str> for Parameter<'i> {
    type Error = String;

    fn try_from(input: &'i str) -> Result<Self, Self::Error> {
        parameter(input)
            .finish()
            .map(|(_, x)| x)
            .map_err(|e: VerboseError<&str>| format!("error: {}", convert_error(input, e.clone())))
    }
}

impl From<Parameter<'_>> for crate::properties::Parameter {
    fn from(parameter: Parameter<'_>) -> crate::properties::Parameter {
        crate::properties::Parameter::new(
            parameter.key.as_ref(),
            parameter.val.as_ref().map(AsRef::as_ref).unwrap_or(""),
        )
    }
}

#[test]
fn test_parameter() {
    assert_parser!(
        parameter,
        ";KEY=VALUE",
        Parameter::new_ref("KEY", Some("VALUE"))
    );

    assert_parser!(
        parameter,
        "; KEY=VALUE",
        Parameter::new_ref("KEY", Some("VALUE"))
    );

    assert_parser!(
        parameter,
        "; KEY=VAL UE",
        Parameter::new_ref("KEY", Some("VAL UE"))
    );

    assert_parser!(parameter, "; KEY=", Parameter::new_ref("KEY", None));

    assert_parser!(
        parameter,
        ";KEY=VAL-UE",
        Parameter::new_ref("KEY", Some("VAL-UE"))
    );

    assert_parser!(
        parameter,
        ";KEY",
        Parameter {
            key: "KEY".into(),
            val: None,
        }
    );

    assert_parser!(
        parameter,
        ";email=rust@hoodie.de",
        Parameter::new_ref("email", Some("rust@hoodie.de"))
    );
}

#[test]
fn test_parameter_with_dash() {
    assert_parser!(
        parameter,
        ";X-HOODIE-KEY=VALUE",
        Parameter::new_ref("X-HOODIE-KEY", Some("VALUE"))
    );
}

#[test]
fn test_quirky_parameter() {
    assert_parser!(parameter, ";KEY=", Parameter::new_ref("KEY", None));
}

fn remove_empty_string(input: Option<&str>) -> Option<&str> {
    if let Some(input) = input {
        return if input.is_empty() { None } else { Some(input) };
    }
    None
}

fn remove_empty_string_parsed(input: Option<ParseString<'_>>) -> Option<ParseString<'_>> {
    if let Some(input) = input {
        return if input.as_ref().is_empty() {
            None
        } else {
            Some(input)
        };
    }
    None
}

fn parameter<'i, E>(input: &'i str) -> IResult<&'i str, Parameter<'i>, E>
where
    E: ParseError<&'i str> + ContextError<&'i str>,
{
    alt((pair_parameter, base_parameter)).parse(input)
}

fn pair_parameter<'i, E>(input: &'i str) -> IResult<&'i str, Parameter<'i>, E>
where
    E: ParseError<&'i str> + ContextError<&'i str>,
{
    preceded(
        tuple((tag(";"), space0)),
        separated_pair(
            valid_key_sequence_cow, //key
            tag("="),
            opt(alt((
                eof,
                delimited(tag("\""), is_not("\""), tag("\"")),
                take_till1(|x| x == ';' || x == ':'),
            )))
            .map(remove_empty_string),
        ),
    )
    .map(|(key, val)| Parameter {
        key,
        val: val.map(ParseString::from),
    })
    .parse(input)
}

fn base_parameter<'i, E>(input: &'i str) -> IResult<&'i str, Parameter<'i>, E>
where
    E: ParseError<&'i str> + ContextError<&'i str>,
{
    tuple((
        preceded(
            tuple((tag(";"), space0)),
            valid_key_sequence_cow, //key
        ),
        opt(preceded(
            tag("="),
            alt((
                eof,
                delimited(tag("\""), is_not("\""), tag("\"")),
                take_till1(|x| x == ';' || x == ':'),
            ))
            .map(ParseString::from),
        ))
        .map(remove_empty_string_parsed),
    ))
    .map(|(key, val)| Parameter { key, val })
    .parse(input)
}

// parameter list
#[test]
pub fn parse_parameter_list() {
    assert_parser!(
        parameters,
        ";KEY=VALUE",
        vec![Parameter::new_ref("KEY", Some("VALUE"))]
    );

    assert_parser!(
        parameters,
        ";KEY=VALUE;DATE=TODAY",
        vec![
            Parameter::new_ref("KEY", Some("VALUE")),
            Parameter::new_ref("DATE", Some("TODAY")),
        ]
    );

    assert_parser!(
        parameters,
        ";KEY=VALUE;DATE=20170218",
        vec![
            Parameter::new_ref("KEY", Some("VALUE")),
            Parameter::new_ref("DATE", Some("20170218")),
        ]
    );
    assert_parser!(
        parameters,
        ";TEXT=\"quoted text with \\;\"",
        vec![Parameter::new_ref("TEXT", Some("quoted text with \\;")),]
    );
}

pub fn parameters<'i, E>(input: &'i str) -> IResult<&'i str, Vec<Parameter<'i>>, E>
where
    E: ParseError<&'i str> + ContextError<&'i str>,
{
    many0(parameter).parse(input)
}
