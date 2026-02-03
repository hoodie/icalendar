# Refactoring Plan: Cow + Yoke Unification

This document outlines a plan to simultaneously achieve two goals:

1. **Reduce allocations** in the builder types (property/parameter keys)
2. **Unify parser and builder types** using `yoke` for zero-copy parsing

These goals are synergistic — the foundational changes required for one enable the other.

---

## Background

### Current Architecture

The crate has two parallel type hierarchies:

- **Builder Types** (`src/`): Owned `String` fields, no lifetimes
- **Parser Types** (`src/parser/`): Zero-copy `Cow<'a, str>` fields, borrowed from input

This duplication exists because:
- Builders need owned data for mutation and long-lived storage
- Parsers want zero-copy for performance

### The Problem

1. **Allocation overhead**: Every property key (e.g., "SUMMARY", "DTSTART") allocates a `String`, even though these are static RFC 5545 names
2. **Code duplication**: Two sets of types that represent the same concepts
3. **Conversion overhead**: Parsing requires copying from parser types to builder types

### The Solution

Use `Cow<'a, str>` throughout, enabling:
- **Builders**: Use `Cow::Borrowed(&'static str)` for known keys — zero allocation
- **Parsers**: Use `Cow::Borrowed(&'input str)` for parsed data — zero copy
- **Unification**: Same types for both, with `yoke` to erase lifetimes from the public API

---

## Phase 1: Foundation (Cow + new_static)

**Goal**: Add lifetime parameter and `Cow` storage to `Property` and `Parameter`

**Estimated effort**: 4-8 hours

### 1.1 Update `Parameter`

```rust
// src/properties.rs

use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter<'a> {
    key: Cow<'a, str>,
    val: Cow<'a, str>,
}

impl<'a> Parameter<'a> {
    /// Creates a new Parameter (allocates key and val)
    pub fn new(key: &str, val: &str) -> Parameter<'static> {
        Parameter {
            key: Cow::Owned(key.to_owned()),
            val: Cow::Owned(val.to_owned()),
        }
    }

    /// Creates a Parameter with a static key (no key allocation)
    pub fn new_static(key: &'static str, val: &str) -> Parameter<'static> {
        Parameter {
            key: Cow::Borrowed(key),
            val: Cow::Owned(val.to_owned()),
        }
    }

    /// Creates a fully borrowed Parameter (for parsing)
    pub(crate) fn new_borrowed(key: &'a str, val: &'a str) -> Parameter<'a> {
        Parameter {
            key: Cow::Borrowed(key),
            val: Cow::Borrowed(val),
        }
    }

    /// Converts to a fully owned Parameter
    pub fn into_owned(self) -> Parameter<'static> {
        Parameter {
            key: Cow::Owned(self.key.into_owned()),
            val: Cow::Owned(self.val.into_owned()),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.val
    }
}

// Backward compatibility
impl From<(&str, &str)> for Parameter<'static> {
    fn from((key, val): (&str, &str)) -> Self {
        Parameter::new(key, val)
    }
}
```

### 1.2 Update `Property`

```rust
// src/properties.rs

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Property<'a> {
    pub(crate) key: Cow<'a, str>,
    pub(crate) val: Cow<'a, str>,
    pub(crate) params: Vec<Parameter<'a>>,  // Changed from BTreeMap
}

impl<'a> Property<'a> {
    /// Creates a new Property (allocates key and val)
    pub fn new(key: impl Into<String>, val: impl Into<String>) -> Property<'static> {
        Property {
            key: Cow::Owned(key.into()),
            val: Cow::Owned(val.into()),
            params: Vec::new(),
        }
    }

    /// Creates a Property with a static key (no key allocation)
    pub fn new_static(key: &'static str, val: impl Into<String>) -> Property<'static> {
        Property {
            key: Cow::Borrowed(key),
            val: Cow::Owned(val.into()),
            params: Vec::new(),
        }
    }

    /// Creates a fully borrowed Property (for parsing)
    pub(crate) fn new_borrowed(key: &'a str, val: &'a str) -> Property<'a> {
        Property {
            key: Cow::Borrowed(key),
            val: Cow::Borrowed(val),
            params: Vec::new(),
        }
    }

    /// Converts to a fully owned Property
    pub fn into_owned(self) -> Property<'static> {
        Property {
            key: Cow::Owned(self.key.into_owned()),
            val: Cow::Owned(self.val.into_owned()),
            params: self.params.into_iter().map(|p| p.into_owned()).collect(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.val
    }

    pub fn params(&self) -> &[Parameter<'a>] {
        &self.params
    }

    /// Find a parameter by key
    pub fn get_param(&self, key: &str) -> Option<&Parameter<'a>> {
        self.params.iter().find(|p| p.key() == key)
    }

    /// Appends a new parameter
    pub fn append_parameter<P: Into<Parameter<'a>>>(&mut self, param: P) -> &mut Self {
        self.params.push(param.into());
        self
    }
}
```

### 1.3 Type Aliases for Ergonomics

```rust
// src/lib.rs or src/properties.rs

/// Owned Property for builder use (no lifetime in public API)
pub type OwnedProperty = Property<'static>;

/// Owned Parameter for builder use
pub type OwnedParameter = Parameter<'static>;
```

### 1.4 Update Internal Builder Methods

Update all builder methods to use `new_static` for known property names:

```rust
// src/components.rs - Component trait

fn summary(&mut self, text: &str) -> &mut Self {
    self.append_property(Property::new_static("SUMMARY", text))
}

fn description(&mut self, text: &str) -> &mut Self {
    self.append_property(Property::new_static("DESCRIPTION", text))
}

fn uid(&mut self, uid: &str) -> &mut Self {
    self.append_property(Property::new_static("UID", uid))
}

// ... etc for all known properties
```

### 1.5 Migration Notes

- All existing public API signatures remain compatible
- `Property::new()` and `Parameter::new()` still work, returning `'static` lifetime
- Internal code migrates to `new_static()` for efficiency
- Parser code can use `new_borrowed()` for zero-copy

---

## Phase 2: Collection Changes

**Goal**: Replace `BTreeMap` with `Vec` for property storage

**Estimated effort**: 4-8 hours

### 2.1 Rationale

- `BTreeMap` doesn't implement `Yokeable` well (complex derive)
- `Vec` eliminates duplicate key storage (key only in Property, not also as map key)
- For typical component sizes (<20 properties), linear search is competitive

### 2.2 Update `InnerComponent`

```rust
// src/components.rs

pub(crate) struct InnerComponent<'a> {
    pub properties: Vec<Property<'a>>,
    pub multi_properties: Vec<Property<'a>>,
    pub components: Vec<Other<'a>>,
}

impl<'a> InnerComponent<'a> {
    pub fn new() -> Self {
        InnerComponent {
            properties: Vec::new(),
            multi_properties: Vec::new(),
            components: Vec::new(),
        }
    }

    /// Get a property by key
    pub fn get(&self, key: &str) -> Option<&Property<'a>> {
        self.properties.iter().find(|p| p.key() == key)
    }

    /// Get a mutable property by key
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Property<'a>> {
        self.properties.iter_mut().find(|p| p.key() == key)
    }

    /// Set a property (replace if exists, append if not)
    pub fn set(&mut self, property: Property<'a>) {
        if let Some(existing) = self.properties.iter_mut().find(|p| p.key() == property.key()) {
            *existing = property;
        } else {
            self.properties.push(property);
        }
    }

    /// Remove a property by key
    pub fn remove(&mut self, key: &str) {
        self.properties.retain(|p| p.key() != key);
    }

    /// Get property value by key
    pub fn property_value(&self, key: &str) -> Option<&str> {
        self.get(key).map(|p| p.value())
    }

    /// Convert to fully owned
    pub fn into_owned(self) -> InnerComponent<'static> {
        InnerComponent {
            properties: self.properties.into_iter().map(|p| p.into_owned()).collect(),
            multi_properties: self.multi_properties.into_iter().map(|p| p.into_owned()).collect(),
            components: self.components.into_iter().map(|c| c.into_owned()).collect(),
        }
    }
}
```

### 2.3 Update Component Types

Each component type gets the lifetime parameter:

```rust
// src/components/event.rs

pub struct Event<'a> {
    pub(super) inner: InnerComponent<'a>,
}

impl<'a> Event<'a> {
    pub fn new() -> Event<'static> {
        Event {
            inner: InnerComponent::new(),
        }
    }

    pub fn into_owned(self) -> Event<'static> {
        Event {
            inner: self.inner.into_owned(),
        }
    }
}

// Type alias for common case
pub type OwnedEvent = Event<'static>;
```

### 2.4 Update `CalendarComponent` Enum

```rust
// src/calendar/calendar_component.rs

#[non_exhaustive]
pub enum CalendarComponent<'a> {
    Todo(Todo<'a>),
    Event(Event<'a>),
    Venue(Venue<'a>),
    Other(Other<'a>),
}

impl<'a> CalendarComponent<'a> {
    pub fn into_owned(self) -> CalendarComponent<'static> {
        match self {
            CalendarComponent::Todo(t) => CalendarComponent::Todo(t.into_owned()),
            CalendarComponent::Event(e) => CalendarComponent::Event(e.into_owned()),
            CalendarComponent::Venue(v) => CalendarComponent::Venue(v.into_owned()),
            CalendarComponent::Other(o) => CalendarComponent::Other(o.into_owned()),
        }
    }
}

pub type OwnedCalendarComponent = CalendarComponent<'static>;
```

### 2.5 Update `Calendar`

```rust
// src/calendar.rs

pub struct Calendar<'a> {
    pub properties: Vec<Property<'a>>,
    pub components: Vec<CalendarComponent<'a>>,
}

impl<'a> Calendar<'a> {
    pub fn new() -> Calendar<'static> {
        Calendar {
            properties: vec![
                Property::new_static("VERSION", "2.0"),
                Property::new_static("PRODID", "ICALENDAR-RS"),
                Property::new_static("CALSCALE", "GREGORIAN"),
            ],
            components: Vec::new(),
        }
    }

    pub fn into_owned(self) -> Calendar<'static> {
        Calendar {
            properties: self.properties.into_iter().map(|p| p.into_owned()).collect(),
            components: self.components.into_iter().map(|c| c.into_owned()).collect(),
        }
    }
}

pub type OwnedCalendar = Calendar<'static>;
```

---

## Phase 3: Yoke Integration

**Goal**: Use `yoke` to unify parser and builder types with zero-copy parsing

**Estimated effort**: 1-2 days

### 3.1 Add Dependency

```toml
# Cargo.toml
[dependencies]
yoke = { version = "0.7", features = ["derive"] }
```

### 3.2 Derive `Yokeable`

```rust
use yoke::Yokeable;

#[derive(Clone, Debug, PartialEq, Eq, Yokeable)]
pub struct Parameter<'a> {
    key: Cow<'a, str>,
    val: Cow<'a, str>,
}

#[derive(Clone, Debug, PartialEq, Eq, Yokeable)]
pub struct Property<'a> {
    pub(crate) key: Cow<'a, str>,
    pub(crate) val: Cow<'a, str>,
    pub(crate) params: Vec<Parameter<'a>>,
}

// ... derive for all types with lifetime
```

### 3.3 Parser Returns Yoked Calendar

```rust
// src/parser/mod.rs

use yoke::Yoke;

/// A parsed calendar that borrows from its input string
pub type ParsedCalendar = Yoke<Calendar<'static>, String>;

/// Parse an iCalendar string into a zero-copy calendar
pub fn parse(input: String) -> Result<ParsedCalendar, ParseError> {
    Yoke::try_attach_to_cart(input, |input| {
        // Parse into Calendar<'a> where 'a borrows from input
        parse_calendar(input)
    })
}

/// Parse and immediately convert to owned (for compatibility)
pub fn parse_owned(input: &str) -> Result<Calendar<'static>, ParseError> {
    let calendar = parse_calendar(input)?;
    Ok(calendar.into_owned())
}
```

### 3.4 Remove Duplicate Parser Types

Once yoke is integrated, the separate parser types can be removed:

- Delete `src/parser/parsed_string.rs` (use `Cow` directly)
- Delete `src/parser/properties.rs` duplicate types
- Delete `src/parser/parameters.rs` duplicate types
- Delete `src/parser/components.rs` duplicate types

The parser now produces the same types as the builder, just borrowed.

### 3.5 Usage Examples

```rust
// Building (owned, no lifetime visible)
let event = Event::new()
    .summary("Meeting")
    .starts(Utc::now())
    .done();

let calendar = Calendar::from([event]);

// Parsing (zero-copy, yoked)
let input = std::fs::read_to_string("calendar.ics")?;
let parsed: ParsedCalendar = parse(input)?;

// Access parsed data (borrows from yoke)
for event in parsed.get().events() {
    println!("{}", event.get_summary().unwrap_or("(no summary)"));
}

// Convert to owned if needed for mutation
let mut owned: Calendar<'static> = parsed.get().clone().into_owned();
owned.push(Event::new().summary("New Event").done());
```

---

## Phase 4: Cleanup and Optimization

**Goal**: Polish and additional optimizations

**Estimated effort**: 4-8 hours

### 4.1 SmallVec for Properties (Optional)

```rust
use smallvec::SmallVec;

pub(crate) struct InnerComponent<'a> {
    // Inline storage for typical event sizes
    pub properties: SmallVec<[Property<'a>; 12]>,
    pub multi_properties: SmallVec<[Property<'a>; 4]>,
    pub components: SmallVec<[Other<'a>; 2]>,
}
```

### 4.2 Constant Property Names

Define constants for all RFC 5545 property names:

```rust
// src/property_names.rs

pub mod property {
    pub const SUMMARY: &str = "SUMMARY";
    pub const DESCRIPTION: &str = "DESCRIPTION";
    pub const DTSTART: &str = "DTSTART";
    pub const DTEND: &str = "DTEND";
    pub const DTSTAMP: &str = "DTSTAMP";
    pub const UID: &str = "UID";
    pub const STATUS: &str = "STATUS";
    pub const LOCATION: &str = "LOCATION";
    // ... all RFC 5545 properties
}

pub mod param {
    pub const VALUE: &str = "VALUE";
    pub const TZID: &str = "TZID";
    pub const LANGUAGE: &str = "LANGUAGE";
    // ... all RFC 5545 parameters
}
```

### 4.3 Update Documentation

- Document the lifetime parameter in public types
- Explain the `OwnedX` type aliases
- Add examples for zero-copy parsing
- Update migration guide

---

## API Compatibility Summary

| Current API | Phase 1-2 | Phase 3+ |
|-------------|-----------|----------|
| `Property::new(k, v)` | ✅ Returns `Property<'static>` | ✅ Same |
| `Event::new()` | ✅ Returns `Event<'static>` | ✅ Same |
| `Calendar::from([...])` | ✅ Returns `Calendar<'static>` | ✅ Same |
| `event.summary("x")` | ✅ Same signature | ✅ Same |
| `calendar.to_string()` | ✅ Same | ✅ Same |
| `Calendar::from_str(s)` | ✅ Returns `Calendar<'static>` | ✅ Same (owned) |
| — | — | 🆕 `parse(String) -> ParsedCalendar` |

**Key insight**: By using `'static` lifetime for builders and type aliases, the public API remains unchanged. Users only see lifetimes if they use the new zero-copy parsing APIs.

---

## Risks and Mitigations

### Risk: MSRV Compatibility

`yoke` requires a relatively recent Rust version.

**Mitigation**: Feature-gate yoke integration; keep non-yoke path available.

### Risk: Complexity Increase

Lifetimes add cognitive overhead.

**Mitigation**: 
- Type aliases hide lifetimes for common cases
- Good documentation with examples
- Builder API unchanged

### Risk: Breaking Changes

Some edge cases might have subtle breaks.

**Mitigation**:
- Extensive test coverage before starting
- Run full test suite after each phase
- Semver-major release when complete

### Risk: Performance Regression

Vec linear search might be slower than BTreeMap for large components.

**Mitigation**:
- Benchmark before and after
- Consider hybrid approach (Vec for small, switch to map at threshold)
- Most real-world components are small

---

## Success Metrics

1. **Allocation reduction**: Measure with `dhat` or similar
   - Target: 50%+ reduction in allocations for typical calendar building
   
2. **Parse performance**: Benchmark parsing large .ics files
   - Target: 2x+ speedup from zero-copy parsing
   
3. **API compatibility**: All existing tests pass without modification

4. **Code reduction**: Lines of code in parser module
   - Target: Remove ~50% of parser code (duplicate types)

---

## Implementation Order

1. **Phase 1.1-1.3**: Parameter and Property with Cow (foundation)
2. **Phase 1.4**: Update builder methods to use new_static
3. **Phase 1.5**: Verify all tests pass
4. **Phase 2.1-2.5**: InnerComponent and component types with lifetime
5. **Phase 2**: Verify all tests pass
6. **Phase 3.1-3.2**: Add yoke, derive Yokeable
7. **Phase 3.3-3.4**: Integrate yoke in parser
8. **Phase 3.5**: Remove duplicate parser types
9. **Phase 4**: Cleanup and optional optimizations

Each phase should be a separate PR for easier review.

---

## Future Considerations

After this refactoring, the path to **strongly-typed structs** (Option 1 in refactoring-options.md) becomes easier:

- Types already have the lifetime parameter
- `Yokeable` derive works on structs with `Cow` fields
- Could migrate field-by-field without breaking the unification

This refactoring is a stepping stone, not a dead end.