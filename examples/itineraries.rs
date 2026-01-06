//! RFC9073 Itineraries Example
//!
//! This example demonstrates how RFC9073 Event Publishing Extensions can be
//! used for travel itineraries, as described in [RFC9073 Section 3.1.2](https://www.rfc-editor.org/rfc/rfc9073.html#section-3.1.2).
//!
//! The RFC states:
//! > These additions also provide opportunities for the travel industry.
//! > When booking a flight, the "PARTICIPANT" component can be used to
//! > provide references to businesses at the airports and to rental car
//! > businesses at the destination.
//! >
//! > The embedded location information can guide the traveler around the
//! > airport itself or to their final destination. The contact
//! > information can provide detailed information about the booking agent,
//! > airlines, car hire companies, and hotel.
//!
//! This example creates a complete travel itinerary with:
//! - Flight booking event with airline and airport information
//! - Car rental pickup event
//! - Hotel check-in event
//! - Business meeting at destination

use icalendar::{Calendar, Component, Event, EventLike, Location, Participant, Resource};

fn main() {
    // ==========================================================================
    // Flight Event
    // ==========================================================================
    //
    // A flight booking event demonstrating PARTICIPANT for airline/booking agent
    // and VLOCATION for departure and arrival airports.

    // Airline as a contact participant
    let airline = Participant::new()
        .uid("airline-skyways@example.com")
        .participant_type("CONTACT")
        .calendar_address("mailto:reservations@skyways-air.example.com")
        .description("SkyWays Airlines - Flight SW1234")
        .structured_data(
            "https://skyways-air.example.com/vcard/contact.vcf",
            &[("VALUE", "URI"), ("FMTTYPE", "text/vcard")],
        )
        .done();

    // Booking agent contact
    let booking_agent = Participant::new()
        .uid("booking-agent@example.com")
        .participant_type("BOOKING-CONTACT")
        .calendar_address("mailto:support@travelagency.example.com")
        .description("TravelEasy Agency - Your booking reference: TE-2025-78432")
        .structured_data("tel:+1-800-555-0199", &[("VALUE", "URI")])
        .done();

    // Emergency contact for the traveler
    let travel_emergency = Participant::new()
        .uid("emergency-travel@example.com")
        .participant_type("EMERGENCY-CONTACT")
        .calendar_address("mailto:emergency@travelagency.example.com")
        .description("24/7 Travel Emergency Assistance")
        .done();

    // Departure airport location
    let departure_airport = Location::new()
        .uid("airport-jfk@example.com")
        .name("John F. Kennedy International Airport")
        .location_type("terminal") // RFC4589 location type
        .description("Terminal 4, Gate B22. Check-in opens 3 hours before departure.")
        .structured_data("geo:40.6413,-73.7781", &[("VALUE", "URI")])
        .done();

    // Arrival airport location
    let arrival_airport = Location::new()
        .uid("airport-lax@example.com")
        .name("Los Angeles International Airport")
        .location_type("terminal")
        .description("Arriving at Tom Bradley International Terminal. Baggage claim carousel 5.")
        .structured_data("geo:33.9416,-118.4085", &[("VALUE", "URI")])
        .done();

    // Airport lounge as a sponsor/advertiser (as mentioned in RFC for local establishments)
    let airport_lounge = Participant::new()
        .uid("lounge-jfk@example.com")
        .participant_type("SPONSOR")
        .description(
            "SkyClub Lounge - Complimentary access with your booking. Terminal 4, Level 3.",
        )
        .structured_data("https://skyclub.example.com/jfk/", &[("VALUE", "URI")])
        .done();

    let flight_event = Event::new()
        .uid("flight-sw1234-2025@example.com")
        .summary("Flight SW1234: New York (JFK) → Los Angeles (LAX)")
        .description(
            "SkyWays Airlines Flight SW1234\n\
             Confirmation: SW-ABC123\n\
             Seat: 14A (Window)\n\
             Meal: Vegetarian pre-ordered",
        )
        .location("JFK Terminal 4 → LAX Tom Bradley Terminal")
        .append_component(airline)
        .append_component(booking_agent)
        .append_component(travel_emergency)
        .append_component(airport_lounge)
        .append_component(departure_airport)
        .append_component(arrival_airport)
        .done();

    // ==========================================================================
    // Car Rental Pickup Event
    // ==========================================================================
    //
    // Demonstrates PARTICIPANT for car rental company and VLOCATION for pickup.

    let car_rental_company = Participant::new()
        .uid("rental-speedycar@example.com")
        .participant_type("CONTACT")
        .calendar_address("mailto:lax@speedycar.example.com")
        .description("SpeedyCar Rentals - Reservation #SC-98765")
        .structured_data(
            "https://speedycar.example.com/vcard/lax.vcf",
            &[("VALUE", "URI")],
        )
        .structured_data("tel:+1-310-555-0123", &[("VALUE", "URI"), ("ORDER", "2")])
        .done();

    let rental_pickup_location = Location::new()
        .uid("rental-pickup-lax@example.com")
        .name("SpeedyCar LAX Rental Center")
        .location_type("parking")
        .description(
            "Take the free shuttle from Terminal arrivals (blue line). \
             Counter is open 24/7. Have your driver's license and credit card ready.",
        )
        .structured_data("geo:33.9461,-118.3920", &[("VALUE", "URI")])
        .done();

    // Vehicle as a resource
    let rental_vehicle = Resource::new()
        .uid("vehicle-compact@example.com")
        .name("Compact SUV or Similar")
        .resource_type("VEHICLE")
        .description("Toyota RAV4 or equivalent. Automatic, A/C, GPS included.")
        .done();

    let car_rental_event = Event::new()
        .uid("car-rental-2025@example.com")
        .summary("Car Rental Pickup - SpeedyCar LAX")
        .description(
            "SpeedyCar Rentals\n\
             Reservation: SC-98765\n\
             Vehicle: Compact SUV\n\
             Return: Same location, full tank required",
        )
        .location("SpeedyCar LAX Rental Center")
        .append_component(car_rental_company)
        .append_component(rental_pickup_location)
        .append_component(rental_vehicle)
        .done();

    // ==========================================================================
    // Hotel Check-in Event
    // ==========================================================================
    //
    // Demonstrates hotel accommodation with contact and location details.

    let hotel_contact = Participant::new()
        .uid("hotel-grandview@example.com")
        .participant_type("CONTACT")
        .calendar_address("mailto:reservations@grandviewhotel.example.com")
        .description("Grand View Hotel - Confirmation #GV-2025-1234")
        .structured_data(
            "https://grandviewhotel.example.com/vcard/main.vcf",
            &[("VALUE", "URI")],
        )
        .done();

    let hotel_concierge = Participant::new()
        .uid("concierge-grandview@example.com")
        .participant_type("CONTACT")
        .calendar_address("mailto:concierge@grandviewhotel.example.com")
        .description(
            "Hotel Concierge - Available for restaurant reservations and local recommendations",
        )
        .done();

    let hotel_location = Location::new()
        .uid("hotel-location@example.com")
        .name("Grand View Hotel Downtown LA")
        .location_type("hotel")
        .description(
            "5-star hotel in downtown Los Angeles. \
             Valet parking available ($45/night). \
             Check-in: 3:00 PM, Check-out: 11:00 AM",
        )
        .structured_data("geo:34.0522,-118.2437", &[("VALUE", "URI")])
        .structured_data(
            "https://grandviewhotel.example.com/map/downtown-la.html",
            &[("VALUE", "URI"), ("ORDER", "2")],
        )
        .done();

    // Hotel amenities as resources
    let conference_room = Resource::new()
        .uid("hotel-conf-room@example.com")
        .name("Executive Boardroom - 10th Floor")
        .resource_type("ROOM")
        .description("Reserved for your meeting. Capacity: 12 people. A/V equipment included.")
        .done();

    let video_conference = Resource::new()
        .uid("hotel-video-conf@example.com")
        .name("Video Conferencing System")
        .resource_type("REMOTE-CONFERENCE-VIDEO")
        .description("Zoom Rooms enabled. Meeting ID will be provided at check-in.")
        .done();

    let hotel_event = Event::new()
        .uid("hotel-checkin-2025@example.com")
        .summary("Hotel Check-in - Grand View Hotel Downtown LA")
        .description(
            "Grand View Hotel\n\
             Confirmation: GV-2025-1234\n\
             Room Type: Executive Suite\n\
             Nights: 3\n\
             Special Requests: High floor, non-smoking",
        )
        .location("Grand View Hotel, 500 S Grand Ave, Los Angeles, CA 90071")
        .append_component(hotel_contact)
        .append_component(hotel_concierge)
        .append_component(hotel_location)
        .append_component(conference_room)
        .append_component(video_conference)
        .done();

    // ==========================================================================
    // Business Meeting at Destination
    // ==========================================================================
    //
    // The final destination meeting with client contacts and meeting location.

    let client_contact = Participant::new()
        .uid("client-techcorp@example.com")
        .participant_type("CONTACT")
        .calendar_address("mailto:john.smith@techcorp.example.com")
        .description("John Smith - VP of Engineering, TechCorp Inc.")
        .structured_data(
            "https://techcorp.example.com/team/john-smith.vcf",
            &[("VALUE", "URI"), ("FMTTYPE", "text/vcard")],
        )
        .done();

    let meeting_planner = Participant::new()
        .uid("planner-techcorp@example.com")
        .participant_type("PLANNER-CONTACT")
        .calendar_address("mailto:events@techcorp.example.com")
        .description("TechCorp Events Team - Contact for any schedule changes")
        .done();

    let meeting_location = Location::new()
        .uid("techcorp-hq@example.com")
        .name("TechCorp Headquarters")
        .location_type("office")
        .description(
            "Main conference room on 15th floor. \
             Visitor parking in structure B. \
             Security will provide visitor badge at reception.",
        )
        .structured_data("geo:34.0195,-118.4912", &[("VALUE", "URI")])
        .done();

    let meeting_parking = Location::new()
        .uid("techcorp-parking@example.com")
        .name("TechCorp Visitor Parking - Structure B")
        .location_type("parking")
        .description("Enter from Olympic Blvd. Take ticket and validate at reception.")
        .done();

    let presentation_equipment = Resource::new()
        .uid("techcorp-projector@example.com")
        .name("Conference Room A/V System")
        .resource_type("PROJECTOR")
        .description("4K display with HDMI and USB-C connectivity. Adapter available if needed.")
        .done();

    let meeting_event = Event::new()
        .uid("meeting-techcorp-2025@example.com")
        .summary("Quarterly Review Meeting - TechCorp HQ")
        .description(
            "Q3 Partnership Review\n\
             Agenda:\n\
             - Project status update\n\
             - Q4 roadmap discussion\n\
             - Contract renewal terms\n\n\
             Attendees: John Smith (TechCorp), Sarah Johnson (TechCorp), You",
        )
        .location("TechCorp HQ, 2000 Olympic Blvd, Santa Monica, CA 90404")
        .append_component(client_contact)
        .append_component(meeting_planner)
        .append_component(meeting_location)
        .append_component(meeting_parking)
        .append_component(presentation_equipment)
        .done();

    // ==========================================================================
    // Building the Complete Itinerary Calendar
    // ==========================================================================

    let itinerary = Calendar::new()
        .name("Business Trip - Los Angeles, October 2025")
        .push(flight_event)
        .push(car_rental_event)
        .push(hotel_event)
        .push(meeting_event)
        .done();

    // Print the complete itinerary
    println!("RFC9073 Travel Itinerary Example");
    println!("=================================\n");
    println!("This example demonstrates how RFC9073 components can be used");
    println!("for travel industry applications, as described in RFC9073 Section 3.1.2.\n");
    println!("{}", itinerary);

    // ==========================================================================
    // Summary of RFC9073 Features Used
    // ==========================================================================

    println!("\n\nRFC9073 Features Demonstrated:");
    println!("===============================\n");

    println!("PARTICIPANT component types used:");
    println!("  - CONTACT: Airlines, rental companies, hotels, clients");
    println!("  - BOOKING-CONTACT: Travel agency for reservation management");
    println!("  - EMERGENCY-CONTACT: 24/7 travel assistance");
    println!("  - SPONSOR: Airport lounge (local establishment advertising)");
    println!("  - PLANNER-CONTACT: Meeting coordinator");

    println!("\nVLOCATION component types used (RFC4589):");
    println!("  - terminal: Airport terminals");
    println!("  - parking: Car rental, hotel/office parking");
    println!("  - hotel: Accommodation");
    println!("  - office: Business meeting location");

    println!("\nVRESOURCE component types used:");
    println!("  - VEHICLE: Rental car");
    println!("  - ROOM: Hotel conference room");
    println!("  - REMOTE-CONFERENCE-VIDEO: Video conferencing system");
    println!("  - PROJECTOR: Presentation equipment");

    println!("\nSTRUCTURED-DATA property uses:");
    println!("  - geo: URIs for precise locations");
    println!("  - tel: URIs for phone contacts");
    println!("  - https: URIs for vCards and web resources");
    println!("  - ORDER parameter for multiple instances");
}
