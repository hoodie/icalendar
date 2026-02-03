//! Criterion benchmarks for icalendar
//!
//! Run with: cargo bench --bench icalendar_benchmarks
//! Compare to baseline: cargo bench --bench icalendar_benchmarks -- --baseline before
//! Save baseline: cargo bench --bench icalendar_benchmarks -- --save-baseline before

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use chrono::{Duration, Utc};
use icalendar::{Alarm, Calendar, Class, Component, Event, EventLike, EventStatus, Todo};

// ============================================================================
// Building Workloads
// ============================================================================

/// Build a single event with typical properties
fn build_single_event() -> Event {
    Event::new()
        .summary("Team Meeting")
        .description("Weekly sync with the team to discuss progress and blockers")
        .location("Conference Room A")
        .uid("unique-id-12345")
        .starts(Utc::now())
        .ends(Utc::now() + Duration::hours(1))
        .class(Class::Public)
        .priority(5)
        .status(EventStatus::Confirmed)
        .done()
}

/// Build an event with many properties to stress the property storage
fn build_complex_event(i: usize) -> Event {
    let now = Utc::now();
    Event::new()
        .summary(&format!("Event {} - Important Meeting", i))
        .description(
            "This is a longer description that contains more text to simulate \
             real-world calendar entries. It might include agenda items, notes, \
             or other relevant information for attendees.",
        )
        .location(&format!("Building {} - Room {}", i % 5, i % 20))
        .uid(&format!("uid-{}-abcdef123456", i))
        .starts(now)
        .ends(now + Duration::hours(1))
        .class(Class::Public)
        .priority((i % 9) as u32 + 1)
        .status(EventStatus::Confirmed)
        .url("https://example.com/meeting")
        .add_property("CATEGORIES", "MEETING,WORK")
        .add_property("ORGANIZER", "mailto:organizer@example.com")
        .alarm(
            Alarm::display("Meeting starts in 15 minutes", -Duration::minutes(15)).done(),
        )
        .done()
}

/// Build a calendar with N events
fn build_calendar_n(n: usize) -> Calendar {
    let mut calendar = Calendar::new();
    for i in 0..n {
        calendar.push(
            Event::new()
                .summary(&format!("Event {}", i))
                .description("A description for this event")
                .uid(&format!("uid-{}", i))
                .starts(Utc::now())
                .done(),
        );
    }
    calendar
}

/// Build a calendar with N complex events
fn build_complex_calendar_n(n: usize) -> Calendar {
    let mut calendar = Calendar::new()
        .name("Work Calendar")
        .description("Calendar for work events")
        .done();
    for i in 0..n {
        calendar.push(build_complex_event(i));
    }
    calendar
}

/// Build a calendar with mixed component types
fn build_mixed_calendar(n: usize) -> Calendar {
    let mut calendar = Calendar::new();
    for i in 0..n {
        if i % 3 == 0 {
            calendar.push(
                Todo::new()
                    .summary(&format!("Task {}", i))
                    .description("A todo item")
                    .uid(&format!("todo-uid-{}", i))
                    .done(),
            );
        } else {
            calendar.push(
                Event::new()
                    .summary(&format!("Event {}", i))
                    .uid(&format!("event-uid-{}", i))
                    .starts(Utc::now())
                    .done(),
            );
        }
    }
    calendar
}

// ============================================================================
// Parsing Workloads
// ============================================================================

const SINGLE_EVENT_ICS: &str = include_str!("../fixtures/icalendar-rb/single_event.ics");
const TIMEZONE_ICS: &str = include_str!("../fixtures/icalendar-rb/timezone.ics");
const TWO_EVENTS_ICS: &str = include_str!("../fixtures/icalendar-rb/two_events.ics");
const RECURRENCE_ICS: &str = include_str!("../fixtures/icalendar-rb/recurrence.ics");

/// Generate a large calendar string for benchmarking
fn generate_large_calendar_string(event_count: usize) -> String {
    build_complex_calendar_n(event_count).to_string()
}

// ============================================================================
// Benchmark Groups
// ============================================================================

fn benchmark_building(c: &mut Criterion) {
    let mut group = c.benchmark_group("building");

    // Single event
    group.bench_function("single_event", |b| {
        b.iter(|| black_box(build_single_event()))
    });

    // Complex single event
    group.bench_function("complex_event", |b| {
        b.iter(|| black_box(build_complex_event(42)))
    });

    // Calendars of various sizes
    for size in [10, 100, 500, 1000] {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("calendar", size), &size, |b, &size| {
            b.iter(|| black_box(build_calendar_n(size)))
        });
    }

    // Complex calendars
    for size in [10, 100, 500] {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(
            BenchmarkId::new("complex_calendar", size),
            &size,
            |b, &size| b.iter(|| black_box(build_complex_calendar_n(size))),
        );
    }

    // Mixed calendar (events + todos)
    group.throughput(Throughput::Elements(100));
    group.bench_function("mixed_calendar_100", |b| {
        b.iter(|| black_box(build_mixed_calendar(100)))
    });

    group.finish();
}

fn benchmark_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization");

    // Pre-build calendars for serialization benchmarks
    let small_calendar = build_calendar_n(10);
    let medium_calendar = build_complex_calendar_n(100);
    let large_calendar = build_calendar_n(1000);

    group.bench_function("serialize_small", |b| {
        b.iter(|| black_box(small_calendar.to_string()))
    });

    group.bench_function("serialize_medium", |b| {
        b.iter(|| black_box(medium_calendar.to_string()))
    });

    group.bench_function("serialize_large", |b| {
        b.iter(|| black_box(large_calendar.to_string()))
    });

    group.finish();
}

#[cfg(feature = "parser")]
fn benchmark_parsing(c: &mut Criterion) {
    use std::str::FromStr;

    let mut group = c.benchmark_group("parsing");

    // Small fixtures
    group.throughput(Throughput::Bytes(SINGLE_EVENT_ICS.len() as u64));
    group.bench_function("parse_single_event", |b| {
        b.iter(|| black_box(Calendar::from_str(SINGLE_EVENT_ICS).unwrap()))
    });

    group.throughput(Throughput::Bytes(TWO_EVENTS_ICS.len() as u64));
    group.bench_function("parse_two_events", |b| {
        b.iter(|| black_box(Calendar::from_str(TWO_EVENTS_ICS).unwrap()))
    });

    // Medium fixtures
    group.throughput(Throughput::Bytes(TIMEZONE_ICS.len() as u64));
    group.bench_function("parse_timezone", |b| {
        b.iter(|| black_box(Calendar::from_str(TIMEZONE_ICS).unwrap()))
    });

    group.throughput(Throughput::Bytes(RECURRENCE_ICS.len() as u64));
    group.bench_function("parse_recurrence", |b| {
        b.iter(|| black_box(Calendar::from_str(RECURRENCE_ICS).unwrap()))
    });

    // Large synthetic fixture
    let large_ics = generate_large_calendar_string(500);
    group.throughput(Throughput::Bytes(large_ics.len() as u64));
    group.bench_function("parse_large_500", |b| {
        b.iter(|| black_box(Calendar::from_str(&large_ics).unwrap()))
    });

    group.finish();
}

#[cfg(feature = "parser")]
fn benchmark_roundtrip(c: &mut Criterion) {
    use std::str::FromStr;

    let mut group = c.benchmark_group("roundtrip");

    // Roundtrip small
    group.bench_function("roundtrip_single_event", |b| {
        b.iter(|| {
            let parsed = Calendar::from_str(SINGLE_EVENT_ICS).unwrap();
            let serialized = parsed.to_string();
            let reparsed = Calendar::from_str(&serialized).unwrap();
            black_box(reparsed)
        })
    });

    // Roundtrip medium
    group.bench_function("roundtrip_timezone", |b| {
        b.iter(|| {
            let parsed = Calendar::from_str(TIMEZONE_ICS).unwrap();
            let serialized = parsed.to_string();
            let reparsed = Calendar::from_str(&serialized).unwrap();
            black_box(reparsed)
        })
    });

    // Roundtrip built calendar
    let built = build_complex_calendar_n(100);
    let built_string = built.to_string();
    group.bench_function("roundtrip_built_100", |b| {
        b.iter(|| {
            let parsed = Calendar::from_str(&built_string).unwrap();
            let serialized = parsed.to_string();
            let reparsed = Calendar::from_str(&serialized).unwrap();
            black_box(reparsed)
        })
    });

    group.finish();
}

#[cfg(feature = "parser")]
fn benchmark_property_access(c: &mut Criterion) {
    use std::str::FromStr;

    let mut group = c.benchmark_group("property_access");

    // Parse once, then benchmark property access
    let large_ics = generate_large_calendar_string(100);
    let calendar = Calendar::from_str(&large_ics).unwrap();

    group.bench_function("iterate_events", |b| {
        b.iter(|| {
            let mut count = 0;
            for event in calendar.events() {
                black_box(event.get_summary());
                count += 1;
            }
            black_box(count)
        })
    });

    group.bench_function("access_all_properties", |b| {
        b.iter(|| {
            for event in calendar.events() {
                black_box(event.get_summary());
                black_box(event.get_description());
                black_box(event.get_location());
                black_box(event.get_uid());
                black_box(event.get_start());
                black_box(event.get_end());
                black_box(event.get_priority());
                black_box(event.get_status());
            }
        })
    });

    group.finish();
}

// ============================================================================
// Criterion Configuration
// ============================================================================

#[cfg(feature = "parser")]
criterion_group!(
    benches,
    benchmark_building,
    benchmark_serialization,
    benchmark_parsing,
    benchmark_roundtrip,
    benchmark_property_access,
);

#[cfg(not(feature = "parser"))]
criterion_group!(benches, benchmark_building, benchmark_serialization,);

criterion_main!(benches);