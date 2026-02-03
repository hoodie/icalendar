//! Allocation profiling for icalendar
//!
//! This benchmark measures heap allocations for various operations.
//! It can be run in two modes:
//!
//! 1. With dhat for detailed profiling:
//!    cargo run --release --bench allocation_profile --features dhat-heap
//!
//! 2. Without dhat for quick allocation counts:
//!    cargo run --release --bench allocation_profile
//!
//! The output shows allocation counts and bytes for each workload,
//! allowing comparison before and after refactoring.

use chrono::{Duration, Utc};
use icalendar::{Alarm, Calendar, Class, Component, Event, EventLike, EventStatus, Todo};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

// ============================================================================
// Workloads (same as in icalendar_benchmarks.rs for consistency)
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

/// Build an event with many properties
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
        .alarm(Alarm::display("Meeting starts in 15 minutes", -Duration::minutes(15)).done())
        .done()
}

/// Build a calendar with N simple events
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
// Simple allocation counter (when dhat is not available)
// ============================================================================

#[cfg(not(feature = "dhat-heap"))]
mod simple_counter {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicUsize, Ordering};

    pub static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);
    pub static ALLOC_BYTES: AtomicUsize = AtomicUsize::new(0);
    pub static DEALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);
    pub static DEALLOC_BYTES: AtomicUsize = AtomicUsize::new(0);

    pub struct CountingAlloc;

    unsafe impl GlobalAlloc for CountingAlloc {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
            ALLOC_BYTES.fetch_add(layout.size(), Ordering::Relaxed);
            unsafe { System.alloc(layout) }
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            DEALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
            DEALLOC_BYTES.fetch_add(layout.size(), Ordering::Relaxed);
            unsafe { System.dealloc(ptr, layout) }
        }

        unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
            DEALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
            DEALLOC_BYTES.fetch_add(layout.size(), Ordering::Relaxed);
            ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
            ALLOC_BYTES.fetch_add(new_size, Ordering::Relaxed);
            unsafe { System.realloc(ptr, layout, new_size) }
        }
    }

    pub fn reset() {
        ALLOC_COUNT.store(0, Ordering::Relaxed);
        ALLOC_BYTES.store(0, Ordering::Relaxed);
        DEALLOC_COUNT.store(0, Ordering::Relaxed);
        DEALLOC_BYTES.store(0, Ordering::Relaxed);
    }

    pub fn snapshot() -> (usize, usize, usize, usize) {
        (
            ALLOC_COUNT.load(Ordering::Relaxed),
            ALLOC_BYTES.load(Ordering::Relaxed),
            DEALLOC_COUNT.load(Ordering::Relaxed),
            DEALLOC_BYTES.load(Ordering::Relaxed),
        )
    }
}

#[cfg(not(feature = "dhat-heap"))]
#[global_allocator]
static ALLOC: simple_counter::CountingAlloc = simple_counter::CountingAlloc;

// ============================================================================
// Measurement helpers
// ============================================================================

struct AllocationStats {
    name: &'static str,
    alloc_count: usize,
    alloc_bytes: usize,
    #[allow(dead_code)]
    dealloc_count: usize,
    #[allow(dead_code)]
    dealloc_bytes: usize,
}

impl AllocationStats {
    fn print(&self) {
        println!(
            "{:40} {:>8} allocs, {:>10} bytes allocated",
            self.name, self.alloc_count, self.alloc_bytes,
        );
    }
}

#[cfg(not(feature = "dhat-heap"))]
fn measure<F, R>(name: &'static str, f: F) -> AllocationStats
where
    F: FnOnce() -> R,
{
    simple_counter::reset();
    let result = f();
    std::hint::black_box(result);
    let (alloc_count, alloc_bytes, dealloc_count, dealloc_bytes) = simple_counter::snapshot();
    AllocationStats {
        name,
        alloc_count,
        alloc_bytes,
        dealloc_count,
        dealloc_bytes,
    }
}

#[cfg(feature = "dhat-heap")]
fn measure<F, R>(name: &'static str, f: F) -> AllocationStats
where
    F: FnOnce() -> R,
{
    // With dhat, we can't easily get per-operation stats
    // dhat provides overall stats at the end
    // For now, just run the operation and note it was measured
    let result = f();
    std::hint::black_box(result);
    AllocationStats {
        name,
        alloc_count: 0, // dhat provides this globally
        alloc_bytes: 0,
        dealloc_count: 0,
        dealloc_bytes: 0,
    }
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    println!("=============================================================================");
    println!("Allocation Profile for icalendar");
    println!("=============================================================================");
    println!();

    #[cfg(feature = "dhat-heap")]
    println!("Running with dhat - detailed profiling enabled");
    #[cfg(feature = "dhat-heap")]
    println!("Full dhat report will be printed at program exit");
    #[cfg(feature = "dhat-heap")]
    println!();

    #[cfg(not(feature = "dhat-heap"))]
    {
        println!("Running with simple allocation counter");
        println!("For detailed profiling, run with: --features dhat-heap");
        println!();
    }

    let mut all_stats = Vec::new();

    // Building benchmarks
    println!("--- Building ---");

    all_stats.push(measure("build_single_event", || build_single_event()));

    all_stats.push(measure("build_complex_event", || build_complex_event(42)));

    all_stats.push(measure("build_calendar_10", || build_calendar_n(10)));

    all_stats.push(measure("build_calendar_100", || build_calendar_n(100)));

    all_stats.push(measure("build_complex_calendar_10", || {
        build_complex_calendar_n(10)
    }));

    all_stats.push(measure("build_complex_calendar_100", || {
        build_complex_calendar_n(100)
    }));

    all_stats.push(measure("build_mixed_calendar_100", || {
        build_mixed_calendar(100)
    }));

    #[cfg(not(feature = "dhat-heap"))]
    for stats in &all_stats {
        stats.print();
    }

    // Serialization benchmarks
    println!();
    println!("--- Serialization ---");

    let small_calendar = build_calendar_n(10);
    let medium_calendar = build_complex_calendar_n(100);

    all_stats.push(measure("serialize_small_calendar", || {
        small_calendar.to_string()
    }));

    all_stats.push(measure("serialize_medium_calendar", || {
        medium_calendar.to_string()
    }));

    #[cfg(not(feature = "dhat-heap"))]
    for stats in all_stats.iter().skip(all_stats.len() - 2) {
        stats.print();
    }

    // Parsing benchmarks (if parser feature is enabled)
    #[cfg(feature = "parser")]
    {
        use std::str::FromStr;

        println!();
        println!("--- Parsing ---");

        let single_event_ics = include_str!("../fixtures/icalendar-rb/single_event.ics");
        let timezone_ics = include_str!("../fixtures/icalendar-rb/timezone.ics");
        let large_ics = build_complex_calendar_n(100).to_string();

        all_stats.push(measure("parse_single_event", || {
            Calendar::from_str(single_event_ics).unwrap()
        }));

        all_stats.push(measure("parse_timezone", || {
            Calendar::from_str(timezone_ics).unwrap()
        }));

        all_stats.push(measure("parse_large_100", || {
            Calendar::from_str(&large_ics).unwrap()
        }));

        #[cfg(not(feature = "dhat-heap"))]
        for stats in all_stats.iter().skip(all_stats.len() - 3) {
            stats.print();
        }

        // Roundtrip benchmarks
        println!();
        println!("--- Roundtrip ---");

        all_stats.push(measure("roundtrip_single_event", || {
            let parsed = Calendar::from_str(single_event_ics).unwrap();
            let serialized = parsed.to_string();
            Calendar::from_str(&serialized).unwrap()
        }));

        all_stats.push(measure("roundtrip_large_100", || {
            let parsed = Calendar::from_str(&large_ics).unwrap();
            let serialized = parsed.to_string();
            Calendar::from_str(&serialized).unwrap()
        }));

        #[cfg(not(feature = "dhat-heap"))]
        for stats in all_stats.iter().skip(all_stats.len() - 2) {
            stats.print();
        }
    }

    println!();
    println!("=============================================================================");

    // Summary
    #[cfg(not(feature = "dhat-heap"))]
    {
        println!();
        println!("Summary (all operations combined):");
        let total_allocs: usize = all_stats.iter().map(|s| s.alloc_count).sum();
        let total_bytes: usize = all_stats.iter().map(|s| s.alloc_bytes).sum();
        println!("  Total allocations: {}", total_allocs);
        println!("  Total bytes: {}", total_bytes);
    }

    #[cfg(feature = "dhat-heap")]
    {
        println!();
        println!("dhat profiler will print detailed statistics below:");
    }
}