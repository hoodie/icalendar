# Benchmark Results: [Phase Name]

**Date**: YYYY-MM-DD  
**Commit**: `abc1234`  
**Rust Version**: `rustc 1.XX.0`  
**Machine**: [OS, CPU, RAM]

---

## Summary

| Metric | Before | After | Change |
|--------|--------|-------|--------|
| Allocations (build_calendar_100) | — | — | — |
| Bytes allocated (build_calendar_100) | — | — | — |
| Parsing throughput (large) | — | — | — |
| Build throughput (complex_calendar_100) | — | — | — |

---

## Allocation Profile

```
Workload                                    Allocs      Bytes Allocated
---------------------------------------------------------------------------
build_single_event                          —           —
build_complex_event                         —           —
build_calendar_10                           —           —
build_calendar_100                          —           —
build_complex_calendar_10                   —           —
build_complex_calendar_100                  —           —
build_mixed_calendar_100                    —           —
serialize_small_calendar                    —           —
serialize_medium_calendar                   —           —
parse_single_event                          —           —
parse_timezone                              —           —
parse_large_100                             —           —
roundtrip_single_event                      —           —
roundtrip_large_100                         —           —
---------------------------------------------------------------------------
TOTAL                                       —           —
```

---

## Criterion Benchmarks

### Building

| Benchmark | Time | Throughput |
|-----------|------|------------|
| single_event | — | — |
| complex_event | — | — |
| calendar/10 | — | — events/s |
| calendar/100 | — | — events/s |
| calendar/500 | — | — events/s |
| calendar/1000 | — | — events/s |
| complex_calendar/10 | — | — events/s |
| complex_calendar/100 | — | — events/s |
| complex_calendar/500 | — | — events/s |
| mixed_calendar_100 | — | — events/s |

### Serialization

| Benchmark | Time | Throughput |
|-----------|------|------------|
| serialize_small | — | — |
| serialize_medium | — | — |
| serialize_large | — | — |

### Parsing

| Benchmark | Time | Throughput |
|-----------|------|------------|
| parse_single_event | — | — MB/s |
| parse_two_events | — | — MB/s |
| parse_timezone | — | — MB/s |
| parse_recurrence | — | — MB/s |
| parse_large_500 | — | — MB/s |

### Roundtrip

| Benchmark | Time |
|-----------|------|
| roundtrip_single_event | — |
| roundtrip_timezone | — |
| roundtrip_built_100 | — |

### Property Access

| Benchmark | Time |
|-----------|------|
| iterate_events | — |
| access_all_properties | — |

---

## Observations

- [Notable findings]
- [Regressions, if any]
- [Unexpected results]

---

## Next Steps

- [ ] [Action items based on results]