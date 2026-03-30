use super::{Parameter, Property};

/// [RFC 5545, Section 3.2.3](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.3)
/// Calendar User Type (CUTYPE)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CUType {
    /// INDIVIDUAL  (RFC 5545, Section 3.2.3)
    Individual,
    /// GROUP       (RFC 5545, Section 3.2.3)
    Group,
    /// RESOURCE    (RFC 5545, Section 3.2.3)
    Resource,
    /// ROOM        (RFC 5545, Section 3.2.3)
    Room,
    /// UNKNOWN     (RFC 5545, Section 3.2.3)
    Unknown,
}

impl CUType {
    pub(crate) fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "INDIVIDUAL" => Some(CUType::Individual),
            "GROUP" => Some(CUType::Group),
            "RESOURCE" => Some(CUType::Resource),
            "ROOM" => Some(CUType::Room),
            "UNKNOWN" => Some(CUType::Unknown),
            _ => None,
        }
    }
}

impl From<CUType> for Parameter {
    fn from(cutype: CUType) -> Self {
        Parameter::new(
            "CUTYPE",
            match cutype {
                CUType::Individual => "INDIVIDUAL",
                CUType::Group => "GROUP",
                CUType::Resource => "RESOURCE",
                CUType::Room => "ROOM",
                CUType::Unknown => "UNKNOWN",
            },
        )
    }
}

/// [RFC 5545, Section 3.2.16](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.16)
/// Participation Role (ROLE)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    /// CHAIR           (RFC 5545, Section 3.2.16)
    Chair,
    /// REQ-PARTICIPANT (RFC 5545, Section 3.2.16)
    ReqParticipant,
    /// OPT-PARTICIPANT (RFC 5545, Section 3.2.16)
    OptParticipant,
    /// NON-PARTICIPANT (RFC 5545, Section 3.2.16)
    NonParticipant,
}

impl Role {
    pub(crate) fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "CHAIR" => Some(Role::Chair),
            "REQ-PARTICIPANT" => Some(Role::ReqParticipant),
            "OPT-PARTICIPANT" => Some(Role::OptParticipant),
            "NON-PARTICIPANT" => Some(Role::NonParticipant),
            _ => None,
        }
    }
}

impl From<Role> for Parameter {
    fn from(role: Role) -> Self {
        Parameter::new(
            "ROLE",
            match role {
                Role::Chair => "CHAIR",
                Role::ReqParticipant => "REQ-PARTICIPANT",
                Role::OptParticipant => "OPT-PARTICIPANT",
                Role::NonParticipant => "NON-PARTICIPANT",
            },
        )
    }
}

/// [RFC 5545, Section 3.2.12](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.12)
/// Participation Status (PARTSTAT)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PartStat {
    /// NEEDS-ACTION    (RFC 5545, Section 3.2.12)
    NeedsAction,
    /// ACCEPTED        (RFC 5545, Section 3.2.12)
    Accepted,
    /// DECLINED        (RFC 5545, Section 3.2.12)
    Declined,
    /// TENTATIVE       (RFC 5545, Section 3.2.12)
    Tentative,
    /// DELEGATED       (RFC 5545, Section 3.2.12)
    Delegated,
    /// COMPLETED       (RFC 5545, Section 3.2.12; VTODO only)
    Completed,
    /// IN-PROCESS      (RFC 5545, Section 3.2.12; VTODO only)
    InProcess,
}

impl PartStat {
    pub(crate) fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "NEEDS-ACTION" => Some(PartStat::NeedsAction),
            "ACCEPTED" => Some(PartStat::Accepted),
            "DECLINED" => Some(PartStat::Declined),
            "TENTATIVE" => Some(PartStat::Tentative),
            "DELEGATED" => Some(PartStat::Delegated),
            "COMPLETED" => Some(PartStat::Completed),
            "IN-PROCESS" => Some(PartStat::InProcess),
            _ => None,
        }
    }
}

impl From<PartStat> for Parameter {
    fn from(partstat: PartStat) -> Self {
        Parameter::new(
            "PARTSTAT",
            match partstat {
                PartStat::NeedsAction => "NEEDS-ACTION",
                PartStat::Accepted => "ACCEPTED",
                PartStat::Declined => "DECLINED",
                PartStat::Tentative => "TENTATIVE",
                PartStat::Delegated => "DELEGATED",
                PartStat::Completed => "COMPLETED",
                PartStat::InProcess => "IN-PROCESS",
            },
        )
    }
}

/// Encode a list of CAL-ADDRESS URIs as a comma-separated list of individually
/// quoted strings, as required by RFC 5545 §3.2.4, §3.2.5, and §3.2.11:
///
/// > The individual calendar address parameter values MUST each be
/// > specified in a quoted-string.
///
/// Example output: `"mailto:a@example.com","mailto:b@example.com"`
fn encode_cal_address_list(addrs: &[String]) -> String {
    addrs
        .iter()
        .map(|a| format!("\"{}\"", a))
        .collect::<Vec<_>>()
        .join(",")
}

/// Decode a comma-separated list of individually quoted CAL-ADDRESS URIs back
/// into plain address strings.  Handles both quoted (`"mailto:a@b"`) and
/// unquoted (`mailto:a@b`) entries so the decoder is tolerant of both forms.
fn decode_cal_address_list(s: &str) -> Vec<String> {
    // Split on `","` boundaries first (the RFC-correct form), then fall back
    // to a simple comma split for unquoted values produced by other clients.
    s.split(',')
        .map(|part| {
            let trimmed = part.trim();
            // Strip surrounding double-quotes if present.
            if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
                trimmed[1..trimmed.len() - 1].to_string()
            } else {
                trimmed.to_string()
            }
        })
        .filter(|s| !s.is_empty())
        .collect()
}

/// [RFC 5545, Section 3.8.4.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.1)
/// Attendee (ATTENDEE)
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Attendee {
    /// [RFC 5545, Section 3.2.2](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.2)
    /// Common Name
    pub cn: Option<String>,
    /// [RFC 5545, Section 3.2.3](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.3)
    /// Calendar User Type
    pub cutype: Option<CUType>,
    /// [RFC 5545, Section 3.2.4](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.4)
    /// Delegators (list of URIs)
    pub delegated_from: Vec<String>,
    /// [RFC 5545, Section 3.2.5](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.5)
    /// Delegatees (list of URIs)
    pub delegated_to: Vec<String>,
    /// [RFC 5545, Section 3.2.6](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.6)
    /// Directory Entry Reference (URI)
    pub dir: Option<String>,
    /// [RFC 5545, Section 3.2.10](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.10)
    /// Language
    pub language: Option<String>,
    /// [RFC 5545, Section 3.2.11](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.11)
    /// Member (list of URIs)
    pub member: Vec<String>,
    /// [RFC 5545, Section 3.2.12](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.12)
    /// Participation Status
    pub part_stat: Option<PartStat>,
    /// [RFC 5545, Section 3.2.16](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.16)
    /// Participation Role
    pub role: Option<Role>,
    /// [RFC 5545, Section 3.2.17](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.17)
    /// RSVP expectation
    pub rsvp: Option<bool>,
    /// [RFC 5545, Section 3.2.18](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.18)
    /// Sent By (URI)
    pub sent_by: Option<String>,
    /// [RFC 5545, Section 3.3.3](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.3)
    /// The attendee's CAL-ADDRESS (e.g., "mailto:user@example.com")
    pub cal_address: String,
}

impl Attendee {
    /// Create a new Attendee with just the CAL-ADDRESS (e.g., "mailto:user@example.com").
    pub fn new(cal_address: String) -> Self {
        Self {
            cal_address,
            ..Default::default()
        }
    }

    /// Set the CUTYPE.
    pub fn cutype(mut self, cutype: CUType) -> Self {
        self.cutype = Some(cutype);
        self
    }

    /// Add a MEMBER.
    pub fn member(mut self, member: String) -> Self {
        self.member.push(member);
        self
    }

    /// Set the ROLE.
    pub fn role(mut self, role: Role) -> Self {
        self.role = Some(role);
        self
    }

    /// Set the PARTSTAT.
    pub fn partstat(mut self, partstat: PartStat) -> Self {
        self.part_stat = Some(partstat);
        self
    }

    /// Set RSVP.
    pub fn rsvp(mut self, rsvp: bool) -> Self {
        self.rsvp = Some(rsvp);
        self
    }

    /// Add a DELEGATED-TO.
    pub fn delegated_to(mut self, delegated_to: String) -> Self {
        self.delegated_to.push(delegated_to);
        self
    }

    /// Add a DELEGATED-FROM.
    pub fn delegated_from(mut self, delegated_from: String) -> Self {
        self.delegated_from.push(delegated_from);
        self
    }

    /// Set SENT-BY.
    pub fn sentby(mut self, sentby: String) -> Self {
        self.sent_by = Some(sentby);
        self
    }

    /// Set CN.
    pub fn cn(mut self, cn: String) -> Self {
        self.cn = Some(cn);
        self
    }

    /// Set DIR.
    pub fn dir(mut self, dir: String) -> Self {
        self.dir = Some(dir);
        self
    }

    /// Set LANGUAGE.
    pub fn language(mut self, language: String) -> Self {
        self.language = Some(language);
        self
    }
}

impl From<Attendee> for Property {
    fn from(attendee: Attendee) -> Self {
        let mut prop = Property::new("ATTENDEE", attendee.cal_address);

        if let Some(cutype) = attendee.cutype {
            prop.append_parameter(cutype);
        }
        // RFC 5545 §3.2.11: each MEMBER address MUST be in a quoted-string.
        // We store the pre-quoted comma-separated list so that
        // `quote_if_contains_colon` in `to_line()` does not double-quote it.
        if !attendee.member.is_empty() {
            prop.add_parameter("MEMBER", &encode_cal_address_list(&attendee.member));
        }
        if let Some(role) = attendee.role {
            prop.append_parameter(role);
        }
        if let Some(partstat) = attendee.part_stat {
            prop.append_parameter(partstat);
        }
        if let Some(rsvp) = attendee.rsvp {
            prop.add_parameter("RSVP", if rsvp { "TRUE" } else { "FALSE" });
        }
        // RFC 5545 §3.2.5: each DELEGATED-TO address MUST be in a quoted-string.
        if !attendee.delegated_to.is_empty() {
            prop.add_parameter(
                "DELEGATED-TO",
                &encode_cal_address_list(&attendee.delegated_to),
            );
        }
        // RFC 5545 §3.2.4: each DELEGATED-FROM address MUST be in a quoted-string.
        if !attendee.delegated_from.is_empty() {
            prop.add_parameter(
                "DELEGATED-FROM",
                &encode_cal_address_list(&attendee.delegated_from),
            );
        }
        if let Some(sentby) = attendee.sent_by {
            prop.add_parameter("SENT-BY", &sentby);
        }
        if let Some(cn) = attendee.cn {
            prop.add_parameter("CN", &cn);
        }
        if let Some(dir) = attendee.dir {
            prop.add_parameter("DIR", &dir);
        }
        if let Some(language) = attendee.language {
            prop.add_parameter("LANGUAGE", &language);
        }

        prop.done()
    }
}

impl TryFrom<&Property> for Attendee {
    type Error = ();

    fn try_from(prop: &Property) -> Result<Self, Self::Error> {
        if prop.key() != "ATTENDEE" {
            return Err(());
        }
        let value = prop.value().to_string();

        let cutype = prop.get_param_as("CUTYPE", CUType::from_str);
        let member = prop
            .get_param_as("MEMBER", |s| Some(decode_cal_address_list(s)))
            .unwrap_or_default();
        let role = prop.get_param_as("ROLE", Role::from_str);
        let partstat = prop.get_param_as("PARTSTAT", PartStat::from_str);
        let rsvp = prop.get_param_as("RSVP", |s| match s.to_uppercase().as_str() {
            "TRUE" => Some(true),
            "FALSE" => Some(false),
            _ => None,
        });
        let delegated_to = prop
            .get_param_as("DELEGATED-TO", |s| Some(decode_cal_address_list(s)))
            .unwrap_or_default();
        let delegated_from = prop
            .get_param_as("DELEGATED-FROM", |s| Some(decode_cal_address_list(s)))
            .unwrap_or_default();
        let sentby = prop.get_param_as("SENT-BY", |s| Some(s.to_string()));
        let cn = prop.get_param_as("CN", |s| Some(s.to_string()));
        let dir = prop.get_param_as("DIR", |s| Some(s.to_string()));
        let language = prop.get_param_as("LANGUAGE", |s| Some(s.to_string()));

        Ok(Attendee {
            cal_address: value,
            cutype,
            member,
            role,
            part_stat: partstat,
            rsvp,
            delegated_to,
            delegated_from,
            sent_by: sentby,
            cn,
            dir,
            language,
        })
    }
}

#[cfg(test)]
mod test_attendee {
    use super::*;

    #[test]
    fn to_property_basic() {
        let attendee = Attendee::new("mailto:test@example.com".to_string());
        let prop: Property = attendee.into();

        assert_eq!(prop.key(), "ATTENDEE");
        assert_eq!(prop.value(), "mailto:test@example.com");
        assert!(prop.params().is_empty());
    }

    #[test]
    fn to_property_full() {
        let attendee = Attendee::new("mailto:test@example.com".to_string())
            .cutype(CUType::Individual)
            .role(Role::ReqParticipant)
            .partstat(PartStat::Accepted)
            .rsvp(true)
            .cn("Test User".to_string())
            .member("mailto:member1@example.com".to_string())
            .member("mailto:member2@example.com".to_string())
            .delegated_to("mailto:delegate@example.com".to_string())
            .sentby("mailto:sender@example.com".to_string())
            .dir("ldap://example.com/cn=Test%20User".to_string())
            .language("en".to_string());

        let prop: Property = attendee.into();

        assert_eq!(prop.key(), "ATTENDEE");
        assert_eq!(prop.value(), "mailto:test@example.com");

        assert_eq!(prop.params().get("CUTYPE").unwrap().value(), "INDIVIDUAL");
        assert_eq!(
            prop.params().get("ROLE").unwrap().value(),
            "REQ-PARTICIPANT"
        );
        assert_eq!(prop.params().get("PARTSTAT").unwrap().value(), "ACCEPTED");
        assert_eq!(prop.params().get("RSVP").unwrap().value(), "TRUE");
        assert_eq!(prop.params().get("CN").unwrap().value(), "Test User");
        // Multi-value URI params must be stored as individually quoted addresses.
        assert_eq!(
            prop.params().get("MEMBER").unwrap().value(),
            "\"mailto:member1@example.com\",\"mailto:member2@example.com\""
        );
        assert_eq!(
            prop.params().get("DELEGATED-TO").unwrap().value(),
            "\"mailto:delegate@example.com\""
        );
        assert_eq!(
            prop.params().get("SENT-BY").unwrap().value(),
            "mailto:sender@example.com"
        );
        assert_eq!(
            prop.params().get("DIR").unwrap().value(),
            "ldap://example.com/cn=Test%20User"
        );
        assert_eq!(prop.params().get("LANGUAGE").unwrap().value(), "en");
    }

    #[test]
    fn from_property_basic() {
        let prop = Property::new("ATTENDEE", "mailto:test@example.com").done();
        let attendee = Attendee::try_from(&prop).unwrap();

        assert_eq!(attendee.cal_address, "mailto:test@example.com");
        assert!(attendee.cutype.is_none());
        assert!(attendee.role.is_none());
        assert!(attendee.part_stat.is_none());
        assert!(attendee.rsvp.is_none());
        assert!(attendee.cn.is_none());
        assert!(attendee.member.is_empty());
        assert!(attendee.delegated_to.is_empty());
        assert!(attendee.sent_by.is_none());
        assert!(attendee.dir.is_none());
        assert!(attendee.language.is_none());
    }

    #[test]
    fn from_property_full() {
        // Simulate a Property whose parameter values are stored without their
        // surrounding quotes (the internal representation after parsing).
        let prop = Property::new("ATTENDEE", "mailto:test@example.com")
            .add_parameter("CUTYPE", "INDIVIDUAL")
            .add_parameter("ROLE", "REQ-PARTICIPANT")
            .add_parameter("PARTSTAT", "ACCEPTED")
            .add_parameter("RSVP", "TRUE")
            .add_parameter("CN", "Test User")
            .add_parameter(
                "MEMBER",
                // Quoted form as stored after encode_cal_address_list / as seen in real ICS.
                "\"mailto:member1@example.com\",\"mailto:member2@example.com\"",
            )
            .add_parameter("DELEGATED-TO", "\"mailto:delegate@example.com\"")
            .add_parameter("DELEGATED-FROM", "\"mailto:delegator@example.com\"")
            .add_parameter("SENT-BY", "mailto:sender@example.com")
            .add_parameter("DIR", "ldap://example.com/cn=Test%20User")
            .add_parameter("LANGUAGE", "en")
            .done();

        let attendee = Attendee::try_from(&prop).unwrap();

        assert_eq!(attendee.cal_address, "mailto:test@example.com");
        assert_eq!(attendee.cutype, Some(CUType::Individual));
        assert_eq!(attendee.role, Some(Role::ReqParticipant));
        assert_eq!(attendee.part_stat, Some(PartStat::Accepted));
        assert_eq!(attendee.rsvp, Some(true));
        assert_eq!(attendee.cn, Some("Test User".to_string()));
        assert_eq!(
            attendee.member,
            vec![
                "mailto:member1@example.com".to_string(),
                "mailto:member2@example.com".to_string()
            ]
        );
        assert_eq!(
            attendee.delegated_to,
            vec!["mailto:delegate@example.com".to_string()]
        );
        assert_eq!(
            attendee.delegated_from,
            vec!["mailto:delegator@example.com".to_string()]
        );
        assert_eq!(
            attendee.sent_by,
            Some("mailto:sender@example.com".to_string())
        );
        assert_eq!(
            attendee.dir,
            Some("ldap://example.com/cn=Test%20User".to_string())
        );
        assert_eq!(attendee.language, Some("en".to_string()));
    }

    #[test]
    fn try_from_invalid_property() {
        let prop = Property::new("NOT_ATTENDEE", "mailto:test@example.com").done();
        assert!(Attendee::try_from(&prop).is_err());
    }

    /// Unquoted multi-value params (produced by other clients) are still parsed correctly.
    #[test]
    fn from_property_unquoted_multi_value() {
        let prop = Property::new("ATTENDEE", "mailto:test@example.com")
            .add_parameter(
                "MEMBER",
                "mailto:member1@example.com,mailto:member2@example.com",
            )
            .done();

        let attendee = Attendee::try_from(&prop).unwrap();
        assert_eq!(
            attendee.member,
            vec![
                "mailto:member1@example.com".to_string(),
                "mailto:member2@example.com".to_string()
            ]
        );
    }

    #[test]
    fn roundtrip() {
        let original = Attendee::new("mailto:roundtrip@example.com".to_string())
            .cutype(CUType::Resource)
            .role(Role::OptParticipant)
            .partstat(PartStat::Declined)
            .rsvp(true)
            .cn("Roundtrip User".to_string())
            .member("mailto:rt_member@example.com".to_string())
            .delegated_to("mailto:rt_delegate@example.com".to_string())
            .sentby("mailto:rt_sender@example.com".to_string())
            .dir("ldap://rt.example.com".to_string())
            .language("de".to_string());

        let prop: Property = original.clone().into();
        let reconstructed = Attendee::try_from(&prop).unwrap();

        assert_eq!(original, reconstructed);
    }

    /// RFC 5545 §2: all parameter names and enumerated values are case-insensitive.
    #[test]
    fn from_str_case_insensitive() {
        assert_eq!(CUType::from_str("individual"), Some(CUType::Individual));
        assert_eq!(CUType::from_str("Group"), Some(CUType::Group));
        assert_eq!(Role::from_str("chair"), Some(Role::Chair));
        assert_eq!(
            Role::from_str("req-participant"),
            Some(Role::ReqParticipant)
        );
        assert_eq!(PartStat::from_str("accepted"), Some(PartStat::Accepted));
        assert_eq!(PartStat::from_str("In-Process"), Some(PartStat::InProcess));
    }
}
