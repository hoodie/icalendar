use crate::calendar::CalendarComponent;

use super::{Component, Property, components::LikeComponent, read_calendar, unfold};
use core::{fmt, str::FromStr};

/// Helper-type for reserialization
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Calendar<'a> {
    pub properties: Vec<Property<'a>>,
    pub components: Vec<Component<'a>>,
}

impl Calendar<'_> {
    /// Prints to stdout
    pub fn print(&self) -> Result<(), fmt::Error> {
        print_crlf!("{}", self);
        Ok(())
    }
}

impl<'a> LikeComponent<'a> for Calendar<'a> {
    fn name(&self) -> &str {
        const CALENDAR_NAME: &str = "VCALENDAR";
        CALENDAR_NAME
    }

    fn properties(&self) -> &[Property<'a>] {
        &self.properties
    }

    fn components(&self) -> &[Component<'a>] {
        &self.components
    }
}

impl fmt::Display for Calendar<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_write(f)
    }
}

impl From<Calendar<'_>> for crate::Calendar {
    fn from(parsed: Calendar) -> Self {
        // Extract the calendar-level timezone before we move properties.
        let calendar_tz: Option<String> = parsed
            .properties
            .iter()
            .find(|p| p.name == "TIMEZONE-ID" || p.name == "X-WR-TIMEZONE")
            .map(|p| p.val.to_string());

        let mut cal = Self {
            components: parsed.components.into_iter().map(Into::into).collect(),
            properties: parsed.properties.into_iter().map(Into::into).collect(),
        };

        // Propagate the calendar timezone to every component.
        if calendar_tz.is_some() {
            for component in &mut cal.components {
                component.set_calendar_tz(calendar_tz.clone());
            }
        }

        cal
    }
}

impl From<crate::Calendar> for Calendar<'static> {
    fn from(value: crate::Calendar) -> Self {
        Calendar {
            components: value
                .components
                .into_iter()
                .map(|cc| match cc {
                    CalendarComponent::Event(e) => Component::from(e),
                    CalendarComponent::Todo(t) => Component::from(t),
                    CalendarComponent::Venue(v) => Component::from(v),
                    CalendarComponent::Other(o) => Component::from(o),
                })
                .collect(),
            properties: value.properties.into_iter().map(Into::into).collect(),
        }
    }
}

impl<C: crate::Component> From<C> for Component<'static> {
    fn from(root: C) -> Self {
        let properties = root
            .properties()
            .values()
            .chain(root.multi_properties().values().flatten())
            .cloned()
            .map(Into::into)
            .collect();

        let components = root.components().iter().cloned().map(Into::into).collect();

        Component {
            name: root.component_kind().into(),
            properties,
            components,
        }
    }
}

#[test]
fn test_calendar_from_parse_calendar() {
    // prove that we don't add additional version/calscale/prodid if those are already there

    let input = r#"
BEGIN:VCALENDAR
VERSION:3.0
PRODID:MANUAL
X-CALSCALE:HENDRIKIAN
END:VCALENDAR
"#;
    let parsed = read_calendar(input).unwrap();
    let calendar = crate::Calendar::from(parsed);
    let count_prop = |name: &str| calendar.properties.iter().filter(|p| p.key == name).count();

    assert_eq!(count_prop("VERSION"), 1);
    assert_eq!(count_prop("PRODID"), 1);
    assert_eq!(count_prop("CALSCALE"), 0);
}

impl<'a> From<Vec<Component<'a>>> for crate::Calendar {
    fn from(mut components: Vec<Component<'a>>) -> Self {
        let root_is_calendar = components
            .first()
            .map(|first_root| first_root.name == "VCALENDAR")
            .unwrap_or(false);

        let components: Vec<Component<'a>> = if root_is_calendar {
            components.swap_remove(0).components
        } else {
            components
        };
        components
            .into_iter()
            .map(|c: Component<'a>| {
                let elem: CalendarComponent = c.into();
                elem
            })
            .collect()
    }
}

impl FromStr for crate::Calendar {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let from_parsed = crate::Calendar::from(read_calendar(&unfold(s))?);
        Ok(from_parsed)
    }
}
