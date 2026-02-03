# Refactoring Options: Reducing Allocations in icalendar-rs

This document explores architectural changes to reduce heap allocations in the icalendar crate while maintaining API compatibility.

## Problem Statement

The current implementation uses `BTreeMap<String, Property>` for storing component properties. This leads to excessive allocations:

1. **String keys**: Every property insertion allocates a `String` for the map key
2. **Duplicate storage**: The property key is stored both as the map key AND inside the `Property` struct
3. **Map overhead**: BTreeMap nodes have allocation overhead
4. **No compile-time guarantees**: Property uniqueness is enforced at runtime, not compile-time

---

## Option 1: Everything as Structs

**Status**: Proposed  
**Impact**: High (major refactor)  
**API Compatibility**: Preservable via same method signatures

### Concept

RFC 5545 defines exactly which properties can appear in each component and their cardinality (0..1 vs 0..n). Encode this directly in Rust's type system using struct fields instead of maps.

### Proposed Event Structure

```rust
/// VEVENT with all properties as struct fields
pub struct Event {
    // === Required (exactly one) ===
    pub uid: String,
    pub dtstamp: DateTime<Utc>,

    // === Optional (at most one) - enforced by Option<T> ===
    pub dtstart: Option<DateTimeProperty>,
    pub dtend: Option<DateTimeProperty>,
    pub duration: Option<Duration>,
    pub summary: Option<TextProperty>,
    pub description: Option<TextProperty>,
    pub location: Option<TextProperty>,
    pub status: Option<EventStatus>,
    pub class: Option<Class>,
    pub priority: Option<u8>,  // 0-9
    pub sequence: Option<u32>,
    pub transp: Option<TimeTransparency>,
    pub url: Option<Url>,
    pub geo: Option<Geo>,
    pub created: Option<DateTime<Utc>>,
    pub last_modified: Option<DateTime<Utc>>,
    pub organizer: Option<Organizer>,
    pub recurrence_id: Option<RecurrenceId>,
    pub rrule: Option<RecurrenceRule>,

    // === Multi-occurrence (0..n) - enforced by Vec<T> ===
    pub attendees: Vec<Attendee>,
    pub categories: Vec<String>,
    pub comments: Vec<TextProperty>,
    pub contacts: Vec<TextProperty>,
    pub exdates: Vec<DatePerhapsTime>,
    pub rdates: Vec<RDate>,
    pub related_to: Vec<RelatedTo>,
    pub resources: Vec<String>,
    pub request_status: Vec<RequestStatus>,

    // === Sub-components ===
    pub alarms: Vec<Alarm>,

    // === Extension properties (the only dynamic part) ===
    pub x_properties: Vec<Property>,
    pub iana_properties: Vec<Property>,
}
```

### Property Values with Parameters

Properties can have parameters (e.g., `DTSTART;TZID=America/New_York:...`). Encode these in typed wrappers:

```rust
/// Text property with common parameters
pub struct TextProperty {
    pub value: String,
    pub language: Option<String>,      // LANGUAGE param
    pub altrep: Option<String>,        // ALTREP param
}

/// Date-time property with timezone info
pub struct DateTimeProperty {
    pub value: DatePerhapsTime,
    // TZID is already encoded in CalendarDateTime::WithTimezone
    // VALUE=DATE is encoded by using DatePerhapsTime::Date
}

/// Attendee with all its specific parameters
pub struct Attendee {
    pub address: String,               // CAL-ADDRESS (mailto:...)
    pub cn: Option<String>,            // CN (common name)
    pub cutype: Option<CalendarUserType>,
    pub role: Option<Role>,
    pub partstat: Option<ParticipationStatus>,
    pub rsvp: Option<bool>,
    pub sent_by: Option<String>,
    pub dir: Option<String>,
    pub member: Vec<String>,
    pub delegated_to: Vec<String>,
    pub delegated_from: Vec<String>,
}

/// Organizer with its parameters
pub struct Organizer {
    pub address: String,
    pub cn: Option<String>,
    pub sent_by: Option<String>,
    pub dir: Option<String>,
}

/// Geographic position
pub struct Geo {
    pub latitude: f64,
    pub longitude: f64,
}
```

### Calendar Structure

```rust
pub struct Calendar {
    // === Required ===
    pub version: CalendarVersion,  // Always 2.0 for iCalendar
    pub prodid: String,

    // === Optional (at most one) ===
    pub calscale: Option<Calscale>,
    pub method: Option<Method>,
    
    // RFC 7986 extensions
    pub name: Option<TextProperty>,
    pub description: Option<TextProperty>,
    pub refresh_interval: Option<Duration>,
    pub source: Option<String>,
    pub color: Option<String>,

    // === Components (0..n each) ===
    pub events: Vec<Event>,
    pub todos: Vec<Todo>,
    pub journals: Vec<Journal>,
    pub free_busy: Vec<FreeBusy>,
    pub timezones: Vec<VTimezone>,

    // === Extensions ===
    pub x_properties: Vec<Property>,
}
```

### Enums for Well-Known Values

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventStatus {
    Tentative,
    Confirmed,
    Cancelled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeTransparency {
    Opaque,      // Blocks time
    Transparent, // Doesn't block time
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frequency {
    Secondly,
    Minutely,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Chair,
    ReqParticipant,
    OptParticipant,
    NonParticipant,
}
```

### API Compatibility Layer

The existing builder API can be preserved:

```rust
impl Event {
    pub fn new() -> Self {
        Self::default()
    }

    // Same signature as before
    pub fn summary(&mut self, text: &str) -> &mut Self {
        self.summary = Some(TextProperty {
            value: text.to_string(),
            language: None,
            altrep: None,
        });
        self
    }

    pub fn get_summary(&self) -> Option<&str> {
        self.summary.as_ref().map(|p| p.value.as_str())
    }

    pub fn remove_summary(&mut self) -> &mut Self {
        self.summary = None;
        self
    }

    pub fn starts<T: Into<DatePerhapsTime>>(&mut self, dt: T) -> &mut Self {
        self.dtstart = Some(DateTimeProperty { value: dt.into() });
        self
    }

    pub fn get_start(&self) -> Option<DatePerhapsTime> {
        self.dtstart.as_ref().map(|p| p.value.clone())
    }

    // For arbitrary properties - still available
    pub fn add_x_property(&mut self, name: &str, value: &str) -> &mut Self {
        self.x_properties.push(Property::new(name, value));
        self
    }
}
```

### Component Trait Adaptation

```rust
pub trait Component {
    fn component_kind(&self) -> &'static str;  // No allocation!
    
    fn uid(&self) -> &str;
    fn dtstamp(&self) -> DateTime<Utc>;
    
    // Serialization
    fn fmt_write<W: fmt::Write>(&self, out: &mut W) -> fmt::Result;
    
    // For backward compat - iterates over all properties
    fn properties(&self) -> PropertyIter<'_>;
}

impl Component for Event {
    fn component_kind(&self) -> &'static str { "VEVENT" }
    
    fn uid(&self) -> &str { &self.uid }
    fn dtstamp(&self) -> DateTime<Utc> { self.dtstamp }
    
    fn properties(&self) -> PropertyIter<'_> {
        // Yields Property views from struct fields
        PropertyIter::new(self)
    }
}
```

### Benefits

| Aspect | Current (BTreeMap) | Proposed (Structs) |
|--------|-------------------|-------------------|
| **Property access** | O(log n) + string compare | O(1) field access |
| **Uniqueness** | Runtime (map semantics) | Compile-time (`Option<T>`) |
| **Memory** | String keys + tree nodes | Just the values |
| **Type safety** | `&str` values | Proper types (`DateTime`, enums) |
| **Allocations** | Per-property key + value | Only value data |
| **API discoverability** | Check docs for property names | IDE autocomplete on fields |

### Migration Path

1. **Phase 1**: Define the new structured types alongside existing ones
2. **Phase 2**: Implement `From` conversions between old and new
3. **Phase 3**: Update builder methods to populate struct fields directly
4. **Phase 4**: Update serialization to emit from struct fields
5. **Phase 5**: Deprecate internal `InnerComponent` / keep only for `Other`

The `Other` component type would remain dynamic (using the old `Property` storage) for unknown/unparsed components.

### Drawbacks

- Significant implementation effort
- Need to enumerate all RFC 5545 properties
- Adding new properties requires code changes (not just string constants)
- More complex serialization logic (iterate over struct fields)

---

## Option 2: [Reserved]

*To be added*

---

## Option 3: [Reserved]

*To be added*