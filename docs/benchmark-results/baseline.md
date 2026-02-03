# Benchmark Results: Baseline (Before Refactoring)

**Date**: 2026-01-11  
**Commit**: `85c2425`  
**Rust Version**: `rustc 1.92.0 (ded5c06cf 2025-12-08)`  
**Machine**: macOS, Apple M4 Pro (arm64)

---

## Summary

This is the baseline measurement before implementing the Cow + Yoke refactoring.
These numbers will be compared against future measurements to validate that the
refactoring actually improves performance.

| Metric | Baseline | Target | Notes |
|--------|----------|--------|-------|
| Allocations (build_calendar_100) | 1,813 | ~900 (-50%) | Key metric for builder optimization |
| Allocations (parse_large_100) | 36,845 | ~18,000 (-50%) | Key metric for parser optimization |
| Parsing throughput (large_500) | 58 MB/s | ~116 MB/s (2x) | Zero-copy parsing goal |
| Build time (complex_calendar_100) | 158 µs | ~100 µs (-35%) | Less allocation overhead |

---

## Allocation Profile

```
Workload                                      Allocs      Bytes Allocated
---------------------------------------------------------------------------
build_single_event                                34            1,406
build_complex_event                               60            5,571
build_calendar_10                                190           15,309
build_calendar_100                             1,813          148,233
build_complex_calendar_10                        620           60,048
build_complex_calendar_100                     6,023          582,992
build_mixed_calendar_100                       1,411          143,843
serialize_small_calendar                         286           14,009
serialize_medium_calendar                      8,144          759,768
parse_single_event                               362           35,843
parse_timezone                                   524           49,941
parse_large_100                               36,845        3,789,913
roundtrip_single_event                           798           80,366
roundtrip_large_100                           82,034        8,371,622
---------------------------------------------------------------------------
TOTAL                                        139,144       14,058,864
```

### Key Observations

- **build_single_event**: 34 allocations for a single event with ~10 properties
  - That's ~3.4 allocations per property (key + value + overhead)
  - Target: ~1 allocation per property (just the value)

- **build_calendar_100**: 1,813 allocations
  - ~18 allocations per event
  - Most are for property keys that could be static

- **parse_large_100**: 36,845 allocations
  - Current parser copies all strings from input
  - With zero-copy, this should drop dramatically

---

## Criterion Benchmarks

### Building

| Benchmark | Time | Throughput |
|-----------|------|------------|
| single_event | 856.30 ns | — |
| complex_event | 1.475 µs | — |
| calendar/10 | 4.60 µs | 2.17 Melem/s |
| calendar/100 | 45.26 µs | 2.21 Melem/s |
| calendar/500 | 245.06 µs | 2.04 Melem/s |
| calendar/1000 | 483.33 µs | 2.07 Melem/s |
| complex_calendar/10 | 15.16 µs | 660 Kelem/s |
| complex_calendar/100 | 158.28 µs | 632 Kelem/s |
| complex_calendar/500 | 831.89 µs | 601 Kelem/s |
| mixed_calendar_100 | 35.47 µs | 2.82 Melem/s |

### Serialization

| Benchmark | Time |
|-----------|------|
| serialize_small | 7.42 µs |
| serialize_medium | 305.54 µs |
| serialize_large | 667.72 µs |

### Parsing

| Benchmark | Time | Throughput |
|-----------|------|------------|
| parse_single_event | 10.97 µs | 71.87 MB/s |
| parse_two_events | 17.43 µs | 52.73 MB/s |
| parse_timezone | 15.01 µs | 48.54 MB/s |
| parse_recurrence | — | — |
| parse_large_500 | 5.81 ms | 58.06 MB/s |

### Roundtrip

| Benchmark | Time |
|-----------|------|
| roundtrip_single_event | 24.43 µs |
| roundtrip_timezone | 39.35 µs |
| roundtrip_built_100 | 2.54 ms |

### Property Access

| Benchmark | Time |
|-----------|------|
| iterate_events | 1.17 µs |
| access_all_properties | 37.59 µs |

---

## Analysis

### Where Allocations Come From (Building)

For `build_single_event` with ~10 properties, we see 34 allocations:

1. **Property keys**: Each `Property::new("SUMMARY", ...)` allocates a String for "SUMMARY"
2. **Property values**: Each value is allocated (unavoidable for dynamic content)
3. **BTreeMap nodes**: The property storage uses BTreeMap which allocates nodes
4. **Parameter keys/values**: Similar to properties

With `Cow<'static, str>` for keys + `Vec` instead of `BTreeMap`:
- Property keys for known names: 0 allocations (borrowed static)
- BTreeMap overhead: eliminated
- Expected: ~10-15 allocations (just values)

### Where Allocations Come From (Parsing)

For `parse_large_100`, we see 36,845 allocations:

1. **String copies**: Parser converts borrowed slices to owned Strings
2. **Property/Parameter construction**: Same as building
3. **Component construction**: Creating Event/Todo structs

With zero-copy parsing via `yoke`:
- String copies: 0 (borrow from input)
- Construction: minimal (just struct assembly)
- Expected: ~500-1000 allocations (just structural overhead)

---

## Baseline Saved

The Criterion baseline has been saved as `before`. After implementing changes, run:

```bash
just bench-compare before
```

To see the comparison.

---

## Next Steps

1. [x] Establish baseline measurements (this document)
2. [ ] Implement Phase 1: Cow + new_static for Property/Parameter
3. [ ] Measure Phase 1 results
4. [ ] Implement Phase 2: Vec instead of BTreeMap
5. [ ] Measure Phase 2 results
6. [ ] Implement Phase 3: Yoke integration
7. [ ] Measure final results
8. [ ] Document conclusions