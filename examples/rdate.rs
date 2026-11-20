use icalendar::*;
use jiff::civil::date;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let all_hands = Event::new()
        .uid("all_hands_2026@example.com")
        .summary("All-Hands Meeting")
        .description("Selected all-hands sessions, 09:00–10:00 Berlin time.")
        .starts(date(2026, 1, 5).at(9, 0, 0, 0).in_tz("Europe/Berlin")?)
        .ends(date(2026, 1, 5).at(10, 0, 0, 0).in_tz("Europe/Berlin")?)
        .rdate(date(2026, 1, 6).at(9, 0, 0, 0).in_tz("Europe/Berlin")?)
        .rdate(date(2026, 1, 7).at(9, 0, 0, 0).in_tz("Europe/Berlin")?)
        .rdate(date(2026, 1, 8).at(9, 0, 0, 0).in_tz("Europe/Berlin")?)
        .rdate(date(2026, 12, 7).at(9, 0, 0, 0).in_tz("Europe/Berlin")?)
        .exdate(date(2026, 12, 7).at(9, 0, 0, 0).in_tz("Europe/Berlin")?)
        .done();

    let mut calendar = Calendar::new();
    calendar.push(all_hands);

    println!("{calendar}");
    Ok(())
}
