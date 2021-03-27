use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    character::complete::{alpha0, space0},
    combinator::eof,
    error::{ContextError, ParseError},
    multi::many0,
    IResult,
};
#[cfg(test)]
use pretty_assertions::assert_eq;

#[cfg(test)]
use nom::error::ErrorKind;

/// Zero-copy version of `properties::Parameter`
#[derive(PartialEq, Debug, Clone)]
pub struct Parameter<'a> {
    pub key: &'a str,
    pub val: &'a str,
}

impl<'a> From<Parameter<'a>> for crate::properties::Parameter {
    fn from(parameter: Parameter<'_>) -> crate::properties::Parameter {
        crate::properties::Parameter::new(parameter.key, parameter.val)
    }
}

#[test]
fn test_parameter() {
    assert_parser!(
        parameter::<(_, ErrorKind)>(";KEY=VALUE"),
        Parameter {
            key: "KEY",
            val: "VALUE"
        }
    );
    assert_parser!(
        parameter::<(_, ErrorKind)>("; KEY=VALUE"),
        Parameter {
            key: "KEY",
            val: "VALUE"
        }
    );
    assert_parser!(
        parameter::<(_, ErrorKind)>("; KEY=VAL UE"),
        Parameter {
            key: "KEY",
            val: "VAL UE"
        }
    );
    assert_parser!(
        parameter::<(_, ErrorKind)>("; KEY="),
        Parameter {
            key: "KEY",
            val: ""
        }
    );
    assert_parser!(
        parameter::<(_, ErrorKind)>(";KEY=VAL-UE"),
        Parameter {
            key: "KEY",
            val: "VAL-UE"
        }
    );
}

#[test]
#[rustfmt::skip]
fn test_parameter_error() {
    assert!(parameter::<(_, ErrorKind)>(";KEY").is_err());
}

fn parameter<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Parameter, E> {
    let (i, _) = tag(";")(i)?;
    let (i, _) = space0(i)?;
    let (i, key) = alpha0(i)?;
    let (i, _) = tag("=")(i)?;
    let (i, val) = alt((eof, take_till1(|x| x == ';' || x == ':')))(i)?;
    Ok((i, Parameter { key, val }))
}

// parameter list
#[test]
pub fn parse_parameter_list() {
    assert_parser!(
        parameters::<(_, ErrorKind)>(";KEY=VALUE"),
        vec![Parameter {
            key: "KEY",
            val: "VALUE"
        }]
    );

    assert_parser!(
        parameters::<(_, ErrorKind)>(";KEY=VALUE;DATE=TODAY"),
        vec![
            Parameter {
                key: "KEY",
                val: "VALUE"
            },
            Parameter {
                key: "DATE",
                val: "TODAY"
            }
        ]
    );

    assert_parser!(
        parameters::<(_, ErrorKind)>(";KEY=VALUE;DATE=20170218"),
        vec![
            Parameter {
                key: "KEY",
                val: "VALUE"
            },
            Parameter {
                key: "DATE",
                val: "20170218"
            }
        ]
    );
}

pub fn parameters<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Vec<Parameter>, E> {
    many0(parameter)(input)
}
