#![cfg(feature = "rfc9073")]
//! Tests for RFC9073 multi-instance property support (STRUCTURED-DATA with ORDER parameter)

use icalendar::{Location, Participant, Resource};

#[test]
fn participant_multi_instance_properties() {
    let mut participant = Participant::new();
    participant.structured_data("https://example.com/1", &[("VALUE", "URI"), ("ORDER", "1")]);
    participant.structured_data("https://example.com/2", &[("VALUE", "URI"), ("ORDER", "2")]);

    let all_sd = participant.get_all_structured_data();
    assert_eq!(all_sd.len(), 2);
    assert_eq!(all_sd[0].value(), "https://example.com/1");
    assert_eq!(all_sd[1].value(), "https://example.com/2");
    assert_eq!(all_sd[0].params().get("ORDER").unwrap().value(), "1");
    assert_eq!(all_sd[1].params().get("ORDER").unwrap().value(), "2");
}

#[test]
fn location_multi_instance_properties() {
    let mut location = Location::new();
    location.structured_data("https://loc.example/1", &[("VALUE", "URI"), ("ORDER", "1")]);
    location.structured_data("https://loc.example/2", &[("VALUE", "URI"), ("ORDER", "2")]);

    let all_sd = location.get_all_structured_data();
    assert_eq!(all_sd.len(), 2);
    assert_eq!(all_sd[0].value(), "https://loc.example/1");
    assert_eq!(all_sd[1].value(), "https://loc.example/2");
    assert_eq!(all_sd[0].params().get("ORDER").unwrap().value(), "1");
    assert_eq!(all_sd[1].params().get("ORDER").unwrap().value(), "2");
}

#[test]
fn resource_multi_instance_properties() {
    let mut resource = Resource::new();
    resource.structured_data("https://res.example/1", &[("VALUE", "URI"), ("ORDER", "1")]);
    resource.structured_data("https://res.example/2", &[("VALUE", "URI"), ("ORDER", "2")]);

    let all_sd = resource.get_all_structured_data();
    assert_eq!(all_sd.len(), 2);
    assert_eq!(all_sd[0].value(), "https://res.example/1");
    assert_eq!(all_sd[1].value(), "https://res.example/2");
    assert_eq!(all_sd[0].params().get("ORDER").unwrap().value(), "1");
    assert_eq!(all_sd[1].params().get("ORDER").unwrap().value(), "2");
}
