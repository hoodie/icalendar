# Benchmarking Plan: Measuring the Cow + Yoke Refactoring Impact

This document defines how we'll measure whether the proposed refactoring actually improves performance. Without baseline measurements, any "optimization" is just speculation.

---

## Hypothesis

> Using `Cow<'a, str>` for property/parameter keys and `yoke` for zero-copy parsing will:
> 1. Reduce heap allocations by 40-60% for typical calendar building
> 2. Improve parsing throughput by 2x or more (zero-copy vs. copying)
> 3. Reduce peak memory usage for large calendars

---

## What We'll Measure

### 1. Allocation Metrics

| Metric | Why It Matters |
|--------|----------------|
| **Allocation count** | Number of heap allocations — fewer is better |
| **Bytes allocated** | Total memory requested from allocator |
| **Peak memory** | Maximum memory in use at any point |

### 2. Throughput Metrics

| Metric | Why It Matters |
|--------|----------------|
| **Events/second (building)** | How fast we can construct calendars |
| **Bytes/second (parsing)** | How fast we can parse .ics files |
| **Round-trip time** | Parse → serialize → parse cycle |

### 3. Workloads

We need representative workloads that exercise different code paths:

| Workload | Description | Exercises |
|----------|-------------|-----------|
| `build_single_event` | Create one event with 10 properties | Builder allocations |
| `build_calendar_100` | Create calendar with 100 events | Scaling behavior |
| `build_calendar_1000` | Create calendar with 1000 events | Large calendar overhead |
| `parse_small` | Parse `single_event.ics` | Parser baseline |
| `parse_medium` | Parse `timezone.ics` | Nested components |
| `parse_large` | Parse synthetic 1000-event file | Parser scaling |
| `roundtrip` | Parse → serialize → parse | Full cycle |
| `access_properties` | Parse then access every property | Read path after parsing |

---

## Tools

### Criterion (CPU Benchmarks)

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "icalendar_benchmarks"
harness = false
```

Criterion provides:
- Statistical rigor (multiple samples, outlier detection)
- Comparison between runs (before/after)
- HTML reports with graphs

### dhat (Allocation Profiling)

```toml
[dev-dependencies]
dhat = "0.3"

[features]
dhat-heap = []  # Enable for profiling runs
```

dhat provides:
- Exact allocation counts
- Allocation site tracking
- Peak memory usage
- Can be run in CI

### stats_alloc (Simple Allocation Counting)

```toml
[dev-dependencies]
stats_alloc = "0.1"
```

For quick allocation counts in benchmarks without full profiling overhead.

---

## Benchmark Implementation

### Directory Structure

```
benches/
├── icalendar_benchmarks.rs    # Main benchmark file (Criterion)
├── workloads/
│   ├── mod.rs                 # Workload module
│   ├── building.rs            # Calendar building workloads
│   └── parsing.rs             # Parsing workloads
└── fixtures/
    └── large_calendar.ics     # Synthetic large test file
```

### Example Benchmark Code

```rust
// benches/icalendar_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use icalendar::{Calendar, Event, Component, EventLike};
use chrono::Utc;

fn build_single_event() -> Event {
    Event::new()
        .summary("Team Meeting")
        .description("Weekly sync with the team to discuss progress")
        .location("Conference Room A")
        .uid("unique-id-12345")
        .starts(Utc::now())
        .ends(Utc::now() + chrono::Duration::hours(1))
        .class(icalendar::Class::Public)
        .priority(5)
        .done()
}

fn build_calendar_n(n: usize) -> Calendar {
    let mut calendar = Calendar::new();
    for i in 0..n {
        calendar.push(
            Event::new()
                .summary(&format!("Event {}", i))
                .description("A description for this event")
                .uid(&format!("uid-{}", i))
                .starts(Utc::now())
                .done()
        );
    }
    calendar
}

fn benchmark_building(c: &mut Criterion) {
    let mut group = c.benchmark_group("building");
    
    group.bench_function("single_event", |b| {
        b.iter(|| black_box(build_single_event()))
    });
    
    for size in [10, 100, 1000] {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_function(format!("calendar_{}", size), |b| {
            b.iter(|| black_box(build_calendar_n(size)))
        });
    }
    
    group.finish();
}

fn benchmark_parsing(c: &mut Criterion) {
    let small = include_str!("../fixtures/icalendar-rb/single_event.ics");
    let medium = include_str!("../fixtures/icalendar-rb/timezone.ics");
    // Generate or load large fixture
    let large = generate_large_calendar(1000);
    
    let mut group = c.benchmark_group("parsing");
    
    group.throughput(Throughput::Bytes(small.len() as u64));
    group.bench_function("small", |b| {
        b.iter(|| black_box(small.parse::<Calendar>().unwrap()))
    });
    
    group.throughput(Throughput::Bytes(medium.len() as u64));
    group.bench_function("medium", |b| {
        b.iter(|| black_box(medium.parse::<Calendar>().unwrap()))
    });
    
    group.throughput(Throughput::Bytes(large.len() as u64));
    group.bench_function("large", |b| {
        b.iter(|| black_box(large.parse::<Calendar>().unwrap()))
    });
    
    group.finish();
}

fn benchmark_roundtrip(c: &mut Criterion) {
    let input = include_str!("../fixtures/icalendar-rb/timezone.ics");
    
    c.bench_function("roundtrip", |b| {
        b.iter(|| {
            let parsed: Calendar = input.parse().unwrap();
            let serialized = parsed.to_string();
            let reparsed: Calendar = serialized.parse().unwrap();
            black_box(reparsed)
        })
    });
}

criterion_group!(
    benches,
    benchmark_building,
    benchmark_parsing,
    benchmark_roundtrip,
);
criterion_main!(benches);
```

### Allocation Counting in Benchmarks

```rust
// benches/allocation_benchmarks.rs

use stats_alloc::{StatsAlloc, Region, INSTRUMENTED_SYSTEM};
use std::alloc::System;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn count_allocations<F, R>(name: &str, f: F) -> R 
where
    F: FnOnce() -> R
{
    let reg = Region::new(&GLOBAL);
    let result = f();
    let stats = reg.change();
    
    println!("{}: {} allocations, {} bytes allocated, {} bytes freed",
        name,
        stats.allocations,
        stats.bytes_allocated,
        stats.bytes_deallocated,
    );
    
    result
}

fn main() {
    count_allocations("build_single_event", || {
        build_single_event()
    });
    
    count_allocations("build_calendar_100", || {
        build_calendar_n(100)
    });
    
    // etc.
}
```

---

## Baseline Process

### Step 1: Implement Benchmarks

Add the benchmark infrastructure to the project before making any changes.

### Step 2: Record Baseline

```bash
# Run CPU benchmarks
cargo bench --bench icalendar_benchmarks -- --save-baseline before

# Run allocation profiling
cargo run --example allocation_profile --features dhat-heap
```

Save results in `docs/benchmark-results/baseline.md` with:
- Commit hash
- Rust version
- Machine specs
- Benchmark numbers

### Step 3: After Each Phase

```bash
# Compare to baseline
cargo bench --bench icalendar_benchmarks -- --baseline before

# Run allocation profiling again
cargo run --example allocation_profile --features dhat-heap
```

Record in `docs/benchmark-results/phase-N.md`.

---

## Success Criteria

Based on our hypothesis, the refactoring is successful if:

| Metric | Target | Acceptable |
|--------|--------|------------|
| Allocation count (building) | -50% | -30% |
| Parsing throughput | +100% (2x) | +50% |
| Peak memory (large calendar) | -30% | -15% |
| Existing tests | 100% pass | 100% pass |
| API compatibility | No breaking changes | Documented deprecations only |

If we don't hit "acceptable" thresholds, we should reconsider whether the complexity is worth it.

---

## Reporting

After each phase, update `docs/benchmark-results/summary.md` with:

```markdown
## Phase N: [Description]

**Commit**: abc1234
**Date**: YYYY-MM-DD

### Allocation Count (build_calendar_100)
- Before: 1,234 allocations
- After: 567 allocations  
- Change: -54%

### Parsing Throughput (large)
- Before: 45 MB/s
- After: 112 MB/s
- Change: +149%

### Notes
- [Any observations, regressions, or unexpected results]
```

---

## Fixtures Needed

### Synthetic Large Calendar Generator

```rust
// benches/fixtures/generate.rs

fn generate_large_calendar(event_count: usize) -> String {
    let mut cal = Calendar::new();
    for i in 0..event_count {
        cal.push(
            Event::new()
                .summary(&format!("Event {} - A somewhat longer summary", i))
                .description("This is a description that contains some text to make the property value reasonably sized for benchmarking purposes.")
                .location(&format!("Room {}", i % 20))
                .uid(&format!("uid-{}-{}", i, uuid::Uuid::new_v4()))
                .class(icalendar::Class::Public)
                .done()
        );
    }
    cal.to_string()
}
```

Consider pre-generating and committing a `fixtures/large_calendar.ics` for reproducibility.

---

## CI Integration

Add benchmark regression detection to CI:

```yaml
# .github/workflows/bench.yml

name: Benchmarks
on:
  pull_request:
    paths:
      - 'src/**'
      - 'benches/**'

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Run benchmarks
        run: cargo bench --bench icalendar_benchmarks -- --noplot
      
      - name: Check for regressions
        # Could use criterion-compare-action or similar
```

---

## Next Steps

1. [ ] Add Criterion to dev-dependencies
2. [ ] Create `benches/icalendar_benchmarks.rs` with building benchmarks
3. [ ] Add parsing benchmarks
4. [ ] Generate large calendar fixture
5. [ ] Run baseline and save results
6. [ ] Create `docs/benchmark-results/` directory
7. [ ] Add to justfile: `just bench`, `just bench-save`, `just bench-compare`
8. [ ] Then proceed with Phase 1 of the refactoring