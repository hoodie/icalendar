//! RFC9073 Example
//!
//! This example demonstrates the Event Publishing Extensions to iCalendar
//! as defined in [RFC9073](https://www.rfc-editor.org/rfc/rfc9073.html).
//!
//! RFC9073 introduces three new components:
//! - PARTICIPANT: Information about event participants (performers, sponsors, contacts)
//! - VLOCATION: Detailed location information (venues, parking)
//! - VRESOURCE: Resources used in an event (equipment, rooms)

use icalendar::{
    Calendar, CalendarComponent, Component, Event, EventLike, Location, Participant, Resource,
};

fn main() {
    // Create a performer participant
    let performer = Participant::new()
        .uid("performer-001@example.com")
        .participant_type("PERFORMER")
        .calendar_address("mailto:jane.doe@example.com")
        .description("Jane Doe, lead vocalist and guitarist")
        // STRUCTURED-DATA can occur multiple times with ORDER parameter
        // to indicate preference (lower ORDER = higher preference)
        .structured_data(
            "https://example.com/vcard/janedoe.vcf",
            &[("VALUE", "URI"), ("ORDER", "1")],
        )
        .structured_data(
            "https://backup.example.com/vcard/janedoe.vcf",
            &[("VALUE", "URI"), ("ORDER", "2")],
        )
        .done();

    // Create a sponsor participant
    let sponsor = Participant::new()
        .uid("sponsor-001@example.com")
        .participant_type("SPONSOR")
        .description("Acme Corp - Event Sponsor")
        .structured_data(
            "https://acme.example.com/vcard/company.vcf",
            &[("VALUE", "URI")],
        )
        .done();

    // Create an emergency contact participant
    let emergency_contact = Participant::new()
        .uid("emergency-001@example.com")
        .participant_type("EMERGENCY-CONTACT")
        .calendar_address("mailto:security@venue.example.com")
        .description("Venue Security Team - Available 24/7")
        .done();

    // ==========================================================================
    // VLOCATION Component (RFC9073 Section 7.2)
    // ==========================================================================
    //
    // The VLOCATION component provides detailed location information for an
    // event, including venue details, parking locations, or other related places.
    // Location types are defined in RFC4589.

    // Create the main venue location
    let venue = Location::new()
        .uid("venue-001@example.com")
        .name("Grand Concert Hall")
        .location_type("auditorium") // Location types from RFC4589
        .description("Main performance venue with 2000 seats")
        .structured_data(
            "https://venue.example.com/vcard/grand-hall.vcf",
            &[("VALUE", "URI")],
        )
        .done();

    // Create a parking location
    let parking = Location::new()
        .uid("parking-001@example.com")
        .name("Concert Hall Parking Garage")
        .location_type("parking")
        .description("Underground parking with 500 spaces. Enter from Main Street.")
        .done();

    // ==========================================================================
    // VRESOURCE Component (RFC9073 Section 7.3)
    // ==========================================================================
    //
    // The VRESOURCE component provides information about resources used in
    // an event, such as projectors, rooms, or remote conferencing equipment.

    // Create a projector resource
    let projector = Resource::new()
        .uid("resource-001@example.com")
        .name("4K Laser Projector")
        .resource_type("PROJECTOR")
        .description("High-definition laser projector for visual displays")
        .done();

    // Create a remote conferencing resource
    let live_stream = Resource::new()
        .uid("resource-002@example.com")
        .name("Live Stream Setup")
        .resource_type("REMOTE-CONFERENCE-VIDEO")
        .description("Professional live streaming equipment for remote viewers")
        .structured_data(
            "https://stream.example.com/event/config.json",
            &[
                ("VALUE", "URI"),
                ("FMTTYPE", "application/json"),
                ("SCHEMA", "https://schema.org/VideoObject"),
            ],
        )
        .done();

    // ==========================================================================
    // Building the Event with RFC9073 Components
    // ==========================================================================

    let event = Event::new()
        .uid("concert-2025@example.com")
        .summary("Summer Music Festival - Opening Night")
        .description("Join us for an unforgettable evening of live music!")
        .location("Grand Concert Hall, 123 Main Street")
        // Add participants
        .append_component(performer)
        .append_component(sponsor)
        .append_component(emergency_contact)
        // Add locations
        .append_component(venue)
        .append_component(parking)
        // Add resources
        .append_component(projector)
        .append_component(live_stream)
        .done();

    // ==========================================================================
    // Building the Calendar
    // ==========================================================================

    let calendar = Calendar::new()
        .name("Summer Music Festival 2025")
        .push(event)
        .done();

    // Print the calendar in iCalendar format
    println!("Generated iCalendar with RFC9073 components:");
    println!("=============================================\n");
    println!("{}", calendar);

    // ==========================================================================
    // Accessing RFC9073 Component Properties
    // ==========================================================================

    println!("\n\nAccessing component properties:");
    println!("================================\n");

    // Create a participant and access its properties
    let artist = Participant::new()
        .uid("artist@example.com")
        .participant_type("PERFORMER")
        .description("Featured Artist")
        .structured_data("https://artist.example.com/card.vcf", &[("VALUE", "URI")])
        .structured_data(
            "https://backup.artist.example.com/card.vcf",
            &[("VALUE", "URI"), ("ORDER", "2")],
        )
        .done();

    println!("Participant UID: {:?}", artist.get_uid());
    println!("Participant Type: {:?}", artist.get_participant_type());
    println!("Description: {:?}", artist.get_description());
    println!("First STRUCTURED-DATA: {:?}", artist.get_structured_data());
    println!(
        "All STRUCTURED-DATA count: {}",
        artist.get_all_structured_data().len()
    );

    // Demonstrate Location properties
    let loc = Location::new()
        .uid("loc@example.com")
        .name("Test Venue")
        .location_type("arena")
        .done();

    println!("\nLocation UID: {:?}", loc.get_uid());
    println!("Location Name: {:?}", loc.get_name());
    println!("Location Type: {:?}", loc.get_location_type());

    // Demonstrate Resource properties
    let res = Resource::new()
        .uid("res@example.com")
        .name("Microphone")
        .resource_type("AUDIO-EQUIPMENT")
        .done();

    println!("\nResource UID: {:?}", res.get_uid());
    println!("Resource Name: {:?}", res.get_name());
    println!("Resource Type: {:?}", res.get_resource_type());

    // ==========================================================================
    // Roundtrip Test: Serialize -> Parse -> Verify Components
    // ==========================================================================
    //
    // Demonstrate that a calendar with RFC9073 components can be serialized,
    // parsed back, and all component data is preserved correctly.
    // Note: DTSTAMP changes on each serialization (current time), so we
    // compare the parsed component properties instead of raw strings.

    println!("\n\nRoundtrip parsing test:");
    println!("=======================\n");

    // Create a simple calendar with RFC9073 components
    let original_calendar = Calendar::new()
        .name("Roundtrip Test")
        .push(
            Event::new()
                .uid("roundtrip-test@example.com")
                .summary("Roundtrip Test Event")
                .append_component(
                    Participant::new()
                        .uid("participant@example.com")
                        .participant_type("PERFORMER")
                        .description("Test Performer")
                        .done(),
                )
                .append_component(
                    Location::new()
                        .uid("location@example.com")
                        .name("Test Venue")
                        .location_type("auditorium")
                        .done(),
                )
                .append_component(
                    Resource::new()
                        .uid("resource@example.com")
                        .name("Test Projector")
                        .resource_type("PROJECTOR")
                        .done(),
                )
                .done(),
        )
        .done();

    // Serialize to string
    let serialized = original_calendar.to_string();
    println!("Original calendar serialized ({} bytes)", serialized.len());

    // Parse back
    let parsed_calendar: Calendar = serialized.parse().expect("Failed to parse calendar");
    println!("Calendar parsed successfully");

    // Verify the parsed components match
    let mut all_match = true;
    let mut found_participant = false;
    let mut found_location = false;
    let mut found_resource = false;

    // Check calendar name
    if parsed_calendar.get_name() != Some("Roundtrip Test") {
        println!("✗ Calendar name mismatch");
        all_match = false;
    }

    // Find and verify components
    for component in &parsed_calendar.components {
        if let CalendarComponent::Event(event) = component {
            if event.get_uid() != Some("roundtrip-test@example.com") {
                println!("✗ Event UID mismatch");
                all_match = false;
            }
            if event.get_summary() != Some("Roundtrip Test Event") {
                println!("✗ Event summary mismatch");
                all_match = false;
            }

            // Check nested RFC9073 components using component_kind()
            for child in event.components() {
                match child.component_kind().as_str() {
                    "PARTICIPANT" => {
                        found_participant = true;
                        if child.property_value("UID") != Some("participant@example.com")
                            || child.property_value("PARTICIPANT-TYPE") != Some("PERFORMER")
                            || child.property_value("DESCRIPTION") != Some("Test Performer")
                        {
                            println!("✗ Participant properties mismatch");
                            all_match = false;
                        }
                    }
                    "VLOCATION" => {
                        found_location = true;
                        if child.property_value("UID") != Some("location@example.com")
                            || child.property_value("NAME") != Some("Test Venue")
                            || child.property_value("LOCATION-TYPE") != Some("auditorium")
                        {
                            println!("✗ Location properties mismatch");
                            all_match = false;
                        }
                    }
                    "VRESOURCE" => {
                        found_resource = true;
                        if child.property_value("UID") != Some("resource@example.com")
                            || child.property_value("NAME") != Some("Test Projector")
                            || child.property_value("RESOURCE-TYPE") != Some("PROJECTOR")
                        {
                            println!("✗ Resource properties mismatch");
                            all_match = false;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    if !found_participant {
        println!("✗ PARTICIPANT component not found");
        all_match = false;
    }
    if !found_location {
        println!("✗ VLOCATION component not found");
        all_match = false;
    }
    if !found_resource {
        println!("✗ VRESOURCE component not found");
        all_match = false;
    }

    if all_match {
        println!("\n✓ Roundtrip successful! All RFC9073 components parsed and verified.");
    } else {
        println!("\n✗ Roundtrip failed - see mismatches above.");
    }
}
