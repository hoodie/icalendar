use icalendar::*;

fn main() {
    // Calendar::default will prefill the properties `VERSION`, `PRODID` and `CALSCALE`
    let new_calendar = Calendar::new();
    let default_calendar = Calendar::default();
    println!("{default_calendar}");

    assert_eq!(new_calendar, default_calendar);
}
