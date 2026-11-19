use icalendar::*;
use jiff::civil::date;

#[test]
fn try_into_string() -> Result<(), Box<dyn std::error::Error>> {
    let bday = Event::new()
        .starts(date(2016, 3, 15))
        .ends(date(2016, 3, 15))
        .summary("My Birthday")
        .description(
            r#"Hey, I'm gonna have a party
BYOB: Bring your own beer.
Hendrik"#,
        )
        .done();

    let mut calendar = Calendar::new();
    calendar.push(bday);

    let s1: String = (&calendar).try_into()?;
    let s2: String = calendar.to_string();

    println!("{:?}", (s1, s2));

    Ok(())
}
