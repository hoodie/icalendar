use icalendar::*;

fn main() {
    let calendar = Calendar::new()
        .push(
            Event::new()
                .summary("Quarterly planning")
                .categories(["Work", "Planning"])
                // commas inside an item are escaped
                .categories(vec![String::from("Customers, VIP")])
                .resources(["Projector", "Room A, 2nd floor"]),
        )
        .done();

    let ical = calendar.to_string();
    println!("{ical}");

    let parsed: Calendar = ical.parse().unwrap();
    for event in parsed.components.iter().filter_map(|c| c.as_event()) {
        let summary = event.get_summary().unwrap_or_default();
        let categories = event.get_categories().unwrap_or_default();
        let resources = event.get_resources().unwrap_or_default();
        println!("{summary}:\n  categories: {categories:?}\n  resources:  {resources:?}\n");
    }
    println!("{parsed}");
}
