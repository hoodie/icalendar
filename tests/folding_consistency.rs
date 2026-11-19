#![cfg(feature = "parser")]
use icalendar::*;
use jiff::{Timestamp, ToSpan as _};
use pretty_assertions::assert_eq;

#[test]
fn test_folding_consistency() {
    let parsed_0 = Calendar::from([
        // add an event
        Event::new()
            .summary("test event")
            .description(include_str!("lorem.txt"))
            .timestamp(Timestamp::now())
            .uid("1234")
            .starts(Timestamp::now())
            .class(Class::Confidential)
            .ends(Timestamp::now() + 24.hours())
            .done(),
    ]);
    let serialized_1 = parsed_0.to_string();

    let parsed_1 = serialized_1.parse::<Calendar>().unwrap();
    assert_eq!(parsed_0, parsed_1);

    let serialized_2 = dbg!(parsed_1.to_string());

    let parsed_2 = serialized_2.parse::<Calendar>().unwrap();
    let serialized_3 = dbg!(parsed_2.to_string());

    let parsed_3 = serialized_3.parse::<Calendar>().unwrap();
    let serialized_4 = dbg!(parsed_3.to_string());

    assert_eq!(parsed_1, parsed_2);
    assert_eq!(parsed_2, parsed_3);

    assert_eq!(serialized_1, serialized_2);
    assert_eq!(serialized_2, serialized_3);
    assert_eq!(serialized_3, serialized_4);
}
