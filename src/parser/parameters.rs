use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    character::complete::space0,
    combinator::{eof, map, opt},
    error::{convert_error, ContextError, ParseError, VerboseError},
    multi::many0,
    sequence::{preceded, separated_pair, tuple},
    Finish, IResult,
};

#[cfg(test)]
use nom::error::ErrorKind;

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

impl<'a> TryFrom<&'a str> for Parameter<'a> {
    type Error = String;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        parameter(input)
            .finish()
            .map(|(_, x)| x)
            .map_err(|e: VerboseError<&str>| format!("error: {}", convert_error(input, e.clone())))
    }
}

impl<'a> From<Parameter<'a>> for crate::properties::Parameter {
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

fn parameter<'a, E: ParseError<&'a str> + ContextError<&'a str> + nom_supreme::tag::TagError<&'a str, &'a str>>(
    input: &'a str,
) -> IResult<&'a str, Parameter<'a>, E> {
    alt((pair_parameter, base_parameter))(input)
}

fn pair_parameter<'a, E: ParseError<&'a str> + ContextError<&'a str> + nom_supreme::tag::TagError<&'a str, &'a str>>(
    input: &'a str,
) -> IResult<&'a str, Parameter<'a>, E> {
    map(
        preceded(
            tuple((tag(";"), space0)),
            separated_pair(
                valid_key_sequence_cow, //key
                tag("="),
                map(
                    opt(alt((eof, take_till1(|x| x == ';' || x == ':')))),
                    remove_empty_string,
                ),
            ),
        ),
        |(key, val)| Parameter {
            key,
            val: val.map(ParseString::from),
        },
    )(input)
}

fn base_parameter<'a, E: ParseError<&'a str> + ContextError<&'a str> + nom_supreme::tag::TagError<&'a str, &'a str>>(
    input: &'a str,
) -> IResult<&'a str, Parameter<'a>, E> {
    map(
        tuple((
            preceded(
                tuple((tag(";"), space0)),
                valid_key_sequence_cow, //key
            ),
            map(
                opt(preceded(
                    tag("="),
                    map(
                        alt((eof, take_till1(|x| x == ';' || x == ':'))),
                        ParseString::from,
                    ),
                )),
                remove_empty_string_parsed,
            ),
        )),
        |(key, val)| Parameter { key, val },
    )(input)
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
}

pub fn parameters<'a, E: ParseError<&'a str> + ContextError<&'a str> + nom_supreme::tag::TagError<&'a str, &'a str>>(
    input: &'a str,
) -> IResult<&'a str, Vec<Parameter>, E> {
    many0(parameter)(input)
}
