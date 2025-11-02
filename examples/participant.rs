use icalendar::*;

fn main() {
    // Construct a PARTICIPANT component according to RFC9073
    let mut participant = Participant::new();
    participant
        .uid("keith.jarrett@example.com")
        .participant_type("PERFORMER")
        .calendar_address("mailto:keith.jarrett@example.com")
        .description("Keith Jarrett, American jazz and classical pianist, performing solo improvisations.");
        // .structured_data("https://www.keithjarrett.org/vcard/keithjarrett.vcf") // Uncomment if/when implemented
        // .styled_description("Keith Jarrett, American jazz and classical pianist, performing solo improvisations."); // Uncomment if/when implemented

    let participant = participant.done();

    // Print the PARTICIPANT component as iCalendar text
    let mut output = String::new();
    participant.fmt_write(&mut output).unwrap();
    println!("{}", output);
}
