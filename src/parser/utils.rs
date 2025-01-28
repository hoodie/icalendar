use nom::{
    bytes::complete::{tag_no_case, take_while},
    character::complete::line_ending,
    combinator::complete,
    error::{ContextError, ParseError},
    multi::many0,
    sequence::{delimited, preceded},
    Err, IResult, Parser,
};
#[cfg(test)]
use pretty_assertions::assert_eq;

use super::parsed_string::ParseString;

// TODO: how do I express <<alpha_or_dash, but not "END">>
pub fn property_key<'i, E>(input: &'i str) -> IResult<&'i str, &'i str, E>
where
    E: ParseError<&'i str> + ContextError<&'i str>,
{
    if input.get(0..=2) == Some("END") || input.get(0..=4) == Some("BEGIN") {
        IResult::Err(Err::Error(nom::error::make_error(
            input,
            nom::error::ErrorKind::Satisfy,
        )))
    } else {
        valid_key_sequence(input)
    }
}

pub fn valid_key_sequence<'i, E>(input: &'i str) -> IResult<&'i str, &'i str, E>
where
    E: ParseError<&'i str> + ContextError<&'i str>,
{
    take_while(|c: char| {
        c == '.' || c == ',' || c == '/' || c == '_' || c == '-' || c.is_alphanumeric()
    })
    .parse(input)
}

pub fn valid_key_sequence_cow<'i, E>(input: &'i str) -> IResult<&'i str, ParseString<'i>, E>
where
    E: ParseError<&'i str> + ContextError<&'i str>,
{
    take_while(|c: char| {
        c == '.' || c == ',' || c == '/' || c == '_' || c == '-' || c.is_alphanumeric()
    })
    .map(ParseString::from)
    .parse(input)
}

pub fn line<'i, O, E, P>(prefix: &'i str, parser: P) -> impl Parser<&'i str, Output = O, Error = E>
where
    E: ParseError<&'i str>,
    P: Parser<&'i str, Output = O, Error = E>,
{
    line_separated(complete(preceded(tag_no_case(prefix), parser)))
}

pub fn line_separated<'i, O, E, P>(parser: P) -> impl Parser<&'i str, Output = O, Error = E>
where
    E: ParseError<&'i str>,
    P: Parser<&'i str, Output = O, Error = E>,
{
    delimited(many0(line_ending), parser, many0(line_ending))
}

/// Normalize content lines.
///
/// This simplifies line endings and unfolds breaks to simplify parsing.
/// iCal specifies that content may be folded and to fit into a certain
/// length, which must be undone before parsing.
///
/// This is a copying operation.
///
/// # Example
///
/// ```
/// # use icalendar::parser::unfold;
/// #[rustfmt::skip]
/// let line = "this gets w\r
///  rapped i\r
///  n a w\r
///  eird\r
///   way";
///
/// assert_eq!(unfold(line), "this gets wrapped in a weird way")
/// ```
pub fn unfold(input: &str) -> String {
    input
        .split("\r\n ")
        .flat_map(|l| l.split("\n "))
        .flat_map(|l| l.split("\r\n	"))
        .flat_map(|l| l.split("\n	"))
        .collect()
}

#[test]
fn test_unfold1() {
    let input = "1 hello world\r\n2 hello \r\n   world\r\n3 hello \r\n world\r\n4 hello world";
    let expected = "1 hello world\r\n2 hello   world\r\n3 hello world\r\n4 hello world";
    assert_eq!(unfold(input), expected);
}

#[test]
fn test_unfold1_tabs() {
    let input = "1 hello world\r\n2 hello \r\n		world\r\n3 hello \r\n	world\r\n4 hello world";
    let expected = "1 hello world\r\n2 hello 	world\r\n3 hello world\r\n4 hello world";
    assert_eq!(unfold(input), expected);
}

/// this is actually also allowed by the spec
#[test]
fn test_unfold2() {
    let input1 = "1 hello world\n2 hello \n   world\n3 hello world\n4 hello world";
    let input2 = "1 hello world\r\n2 hello \r\n   world\r\n3 hello \r\n world\r\n4 hello world";

    let expected = vec![
        "1 hello world",
        "2 hello   world",
        "3 hello world",
        "4 hello world",
    ];

    assert_eq!(unfold(input1).lines().collect::<Vec<_>>(), expected);
    assert_eq!(unfold(input2).lines().collect::<Vec<_>>(), expected);
}
