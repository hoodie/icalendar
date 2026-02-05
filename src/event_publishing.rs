#![allow(clippy::upper_case_acronyms)]
//! Event Publishing Extensions to iCalendar (RFC9073)
//!
//! This module provides enums for open registries defined by
//! [RFC9073: Event Publishing Extensions to iCalendar](https://www.rfc-editor.org/rfc/rfc9073.html)
//! and related specifications (e.g., [RFC4589](https://www.rfc-editor.org/rfc/rfc4589.html)).
//!
//! These enums represent the value types for the PARTICIPANT-TYPE, RESOURCE-TYPE, and LOCATION-TYPE
//! properties. Each is an open registry: new values may be registered in the future, so an `Other(String)`
//! variant is provided for forward compatibility.
//!
//! ## References
//! - [RFC9073: Event Publishing Extensions to iCalendar](https://www.rfc-editor.org/rfc/rfc9073.html)
//! - [RFC4589: Location Types Registry](https://www.rfc-editor.org/rfc/rfc4589.html)

use std::fmt;
use std::str::FromStr;

/// PARTICIPANT-TYPE property values as defined in [RFC9073 Section 6.2](https://www.rfc-editor.org/rfc/rfc9073.html#section-6.2)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParticipantType {
    Active,
    Inactive,
    Sponsor,
    Contact,
    BookingContact,
    EmergencyContact,
    PublicityContact,
    PlannerContact,
    Performer,
    Speaker,
    Other(String),
}

impl fmt::Display for ParticipantType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParticipantType::*;
        match self {
            Active => write!(f, "ACTIVE"),
            Inactive => write!(f, "INACTIVE"),
            Sponsor => write!(f, "SPONSOR"),
            Contact => write!(f, "CONTACT"),
            BookingContact => write!(f, "BOOKING-CONTACT"),
            EmergencyContact => write!(f, "EMERGENCY-CONTACT"),
            PublicityContact => write!(f, "PUBLICITY-CONTACT"),
            PlannerContact => write!(f, "PLANNER-CONTACT"),
            Performer => write!(f, "PERFORMER"),
            Speaker => write!(f, "SPEAKER"),
            Other(s) => write!(f, "{s}"),
        }
    }
}

impl FromStr for ParticipantType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "ACTIVE" => Ok(Self::Active),
            "INACTIVE" => Ok(Self::Inactive),
            "SPONSOR" => Ok(Self::Sponsor),
            "CONTACT" => Ok(Self::Contact),
            "BOOKING-CONTACT" => Ok(Self::BookingContact),
            "EMERGENCY-CONTACT" => Ok(Self::EmergencyContact),
            "PUBLICITY-CONTACT" => Ok(Self::PublicityContact),
            "PLANNER-CONTACT" => Ok(Self::PlannerContact),
            "PERFORMER" => Ok(Self::Performer),
            "SPEAKER" => Ok(Self::Speaker),
            other => Ok(Self::Other(other.to_string())),
        }
    }
}

/// RESOURCE-TYPE property values as defined in [RFC9073 Section 6.3](https://www.rfc-editor.org/rfc/rfc9073.html#section-6.3)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Projector,
    Room,
    RemoteConferenceAudio,
    RemoteConferenceVideo,
    Other(String),
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ResourceType::*;
        match self {
            Projector => write!(f, "PROJECTOR"),
            Room => write!(f, "ROOM"),
            RemoteConferenceAudio => write!(f, "REMOTE-CONFERENCE-AUDIO"),
            RemoteConferenceVideo => write!(f, "REMOTE-CONFERENCE-VIDEO"),
            Other(s) => write!(f, "{s}"),
        }
    }
}

impl FromStr for ResourceType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "PROJECTOR" => Ok(Self::Projector),
            "ROOM" => Ok(Self::Room),
            "REMOTE-CONFERENCE-AUDIO" => Ok(Self::RemoteConferenceAudio),
            "REMOTE-CONFERENCE-VIDEO" => Ok(Self::RemoteConferenceVideo),
            other => Ok(Self::Other(other.to_string())),
        }
    }
}

/// LOCATION-TYPE property values as defined in [RFC4589 Section 3.1](https://www.rfc-editor.org/rfc/rfc4589.html#section-3.1)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LocationType {
    Room,
    Auditorium,
    Building,
    Campus,
    Vehicle,
    Conference,
    Park,
    Museum,
    Stadium,
    Arena,
    Other(String),
}

impl fmt::Display for LocationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LocationType::*;
        match self {
            Room => write!(f, "ROOM"),
            Auditorium => write!(f, "AUDITORIUM"),
            Building => write!(f, "BUILDING"),
            Campus => write!(f, "CAMPUS"),
            Vehicle => write!(f, "VEHICLE"),
            Conference => write!(f, "CONFERENCE"),
            Park => write!(f, "PARK"),
            Museum => write!(f, "MUSEUM"),
            Stadium => write!(f, "STADIUM"),
            Arena => write!(f, "ARENA"),
            Other(s) => write!(f, "{s}"),
        }
    }
}

impl FromStr for LocationType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "ROOM" => Ok(Self::Room),
            "AUDITORIUM" => Ok(Self::Auditorium),
            "BUILDING" => Ok(Self::Building),
            "CAMPUS" => Ok(Self::Campus),
            "VEHICLE" => Ok(Self::Vehicle),
            "CONFERENCE" => Ok(Self::Conference),
            "PARK" => Ok(Self::Park),
            "MUSEUM" => Ok(Self::Museum),
            "STADIUM" => Ok(Self::Stadium),
            "ARENA" => Ok(Self::Arena),
            other => Ok(Self::Other(other.to_string())),
        }
    }
}
