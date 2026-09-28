#![cfg(feature = "parser")]

use icalendar::{Attendee, Calendar, Component, Event};

fn assert_attendee_roundtrip(attendee: Attendee) {
    let event = Event::new().attendee(attendee.clone()).done();
    let calendar = Calendar::from([event]);

    let parsed: Calendar = calendar.to_string().parse().unwrap();
    assert_eq!(
        parsed.events().next().unwrap().get_attendees(),
        vec![attendee.clone()]
    );

    let reparsed: Calendar = parsed.to_string().parse().unwrap();
    assert_eq!(
        reparsed.events().next().unwrap().get_attendees(),
        vec![attendee]
    );
}

#[test]
fn roundtrip_multiple_members() {
    let attendee = Attendee::new("mailto:attendee@example.com".into())
        .member("mailto:group1@example.com".into())
        .member("mailto:group2@example.com".into())
        .member("mailto:group3@example.com".into());

    assert_attendee_roundtrip(attendee);
}

#[test]
fn roundtrip_multiple_delegatees() {
    let attendee = Attendee::new("mailto:attendee@example.com".into())
        .delegated_to("mailto:delegate1@example.com".into())
        .delegated_to("mailto:delegate2@example.com".into())
        .rsvp(true);

    assert_attendee_roundtrip(attendee);
}

#[test]
fn roundtrip_multiple_delegators() {
    let attendee = Attendee::new("mailto:attendee@example.com".into())
        .delegated_from("mailto:delegator1@example.com".into())
        .delegated_from("mailto:delegator2@example.com".into())
        .delegated_to("mailto:delegate@example.com".into());

    assert_attendee_roundtrip(attendee);
}

#[test]
fn roundtrip_single_member_with_quoted_name() {
    let attendee = Attendee::new("mailto:attendee@example.com".into())
        .cn("Example: attendee".into())
        .member("mailto:group@example.com".into())
        .rsvp(false);

    assert_attendee_roundtrip(attendee);
}
