use std::fmt;

#[macro_export]
/// helper for testing parsers
macro_rules! xassert_parser {
    ($call:expr, $expected:expr) => {
        let result = $call.unwrap().1;
        assert_eq!(result, $expected);
    };
}

pub fn print_result<T: fmt::Debug>(input: &str, rest: &str, result: &T) {
    println!("INPUT: {input:?}\nLEFT:  {rest:?}\nRESULT: {result:#?}");
}

macro_rules! assert_parser {
    ($parser:ident, $line:expr) => {
        assert_parser!($line, $parser);
    };

    ($parser:ident, $line:expr, $expectation:expr) => {{
        match $parser::<(_, ErrorKind)>(&$line) {
            Ok((rest, parsed)) => {
                crate::assert::print_result($line, &rest, &parsed);
                pretty_assertions::assert_eq!(
                    parsed,
                    $expectation,
                    "{:?} not parsed as expected",
                    $line
                );
                assert!(rest.is_empty(), "not parsed completely");
            }
            Err(error) => {
                assert!(false, "{}", error);
            }
        }
    }};

    ($parser:ident, $line:expr, $expectation:expr, print) => {{
        let (rest, parsed) = $parser::<(_, ErrorKind)>(&$line).unwrap();
        crate::assert::print_result($line, &rest, &parsed);
        pretty_assertions::assert_eq!(parsed, $expectation, "{:?} not parsed as expected", $line);
        assert!(rest.is_empty(), "not parsed completely");

        let serialized = parsed.to_string();
        pretty_assertions::assert_eq!($line, serialized);
    }};
}
