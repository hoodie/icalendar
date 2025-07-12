use std::io;

use icalendar::*;

fn main() -> io::Result<()> {
    let mut event = Event::new()
        .summary("Property Test")
        .add_property("CUSTOM-PROP", "Some value")
        .add_property("ANOTHER-PROP", "Another value")
        .done();

    println!(
        "Custom property value: {:?}",
        event.property_value("CUSTOM-PROP")
    );

    event.remove_property("CUSTOM-PROP");

    println!(
        "After removal - custom property value: {:?}",
        event.property_value("CUSTOM-PROP")
    );
    println!(
        "After removal - other property value: {:?}",
        event.property_value("ANOTHER-PROP")
    );

    Ok(())
}
