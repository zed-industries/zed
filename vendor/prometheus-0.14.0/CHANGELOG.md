# Changelog

## 0.14.0

- API change: Use `AsRef<str>` for owned label values (#537)

- Improvement: Hashing improvements (#532)

- Dependency upgrade: Update `hyper` to 1.6 (#524)

- Dependency upgrade: Update `procfs` to 0.17 (#543)

- Dependency upgrade: Update `protobuf` to 3.7.2 for RUSTSEC-2024-0437 (#541)

- Dependency upgrade: Update `thiserror` to 2.0 (#534)

- Internal change: Fix LSP and Clippy warnings (#540)

- Internal change: Bump MSRV to 1.81 (#539)

- Documentation: Fix `register_histogram_vec_with_registry` docstring (#528)

- Documentation: Fix typos in static-metric docstrings (#479)

- Documentation: Add missing `protobuf` feature to README list (#531)

## 0.13.4

- Improvement: Add PullingGauge (#405)

- Improvement: Let cargo know which example requires which features (#511)

- Bug fix: Prevent `clippy::ignored_unit_patterns` in macro expansions (#497)

- Internal change: Add CI job for minimum toolchain (MSRV) (#467)

- Internal change: Update CI to `actions/checkout@v4` (#499)

- Internal change: Update dependencies

## 0.13.3

- Bug fix: Prevent ProcessCollector underflow with CPU time counter (#465)

- Internal change: Update dependencies

## 0.13.2

- Bug fix: Fix compilation on 32-bit targets (#446)

## 0.13.1

- Improvement: ProcessCollector use IntGauge to provide better performance (#430)

- Bug fix: Fix re-export of TEXT_FORMAT to not require protobuf (#416)

- Bug fix: Fix doc for encode (#433)

- Bug fix: Fix broken doc links (#426)

- Bug fix: Fix crates.io badge (#436)

- Internal change: Derive default instead of obvious manual impl (#437)

- Internal change: Remove needless borrow (#427)

- Internal change: Update dependencies

## 0.13.0

- Bug fix: Avoid panics from `Instant::elapsed` (#406)

- Improvement: Allow trailing comma on macros (#390)

- Improvement: Add macros for custom registry (#396)

- Improvement: Export thread count from `process_collector` (#401)

- Improvement: Add convenience TextEncoder functions to encode directly to string (#402)

- Internal change: Clean up the use of macro_use and extern crate (#398)

- Internal change: Update dependencies

## 0.12.0

 - Improvement: Fix format string in panic!() calls (#391)

 - Improvement: Replace regex with memchr (#385)

 - Improvement: Update reqwest requirement from ^0.10 to ^0.11 (#379)

## 0.11.0

- Improvement: Switch to more efficient `fd_count()` for `process_open_fds` (#357).

- API change: Change Integer Counter type from AtomicI64 to AtomicU64 (#365).

- Internal change: Update dependencies.

## 0.10.0

- Improvement: Use different generic parameters for name and help at metric construction (#324).

- Bug fix: Error instead of panic when constructing histogram with unequal label key and label value length (#326).

- Bug fix: Return `Error::AlreadyReg` on duplicate collector registration (#333).

- Bug fix: Make Histogram::observe atomic across collects (#314).

- Internal change: Replace spin with parking_lot (#318).

- Internal change: Optimize metric formatting (#327).

- Internal change: Update parking_lot and procfs dependencies (#337).

## 0.9.0

- Add: Implement `encode` function for summary type metrics. 

## 0.8.0

- Add: Reset Counters (#261)

- Add: Discard Histogram timers (#257)

- Add: `observe_closure_duration` for better observing closure duration for local histogram (#296)

- Fix a bug that global labels are not handled correctly (#269)

- Improve linear bucket accuracy by avoiding accumulating error (#276)

- Internal change: Use [thiserror](https://docs.rs/thiserror) to generate the error structure (#285)

- Internal change: Switch from procinfo to [procfs](https://docs.rs/procfs) (#290)

- Internal change: Update to newer dependencies

## 0.7.0

- Provide immutable interface for local counters.

## 0.6.1

- Fix compile error when ProtoBuf feature is not enabled (#240).

## 0.6.0

- Add: Expose the default registry (#231).

- Add: Support common namespace prefix and common labels (#233).

## 0.5.0

- Change: Added TLS and BasicAuthentication support to `push` client.

## 0.4.2

- Change: Update to use protobuf 2.0.

## 0.4.1

- Change: `(Local)(Int)Counter.inc_by` only panics in debug build if the given value is < 0 (#168).

## 0.4.0

- Add: Provides `IntCounter`, `IntCounterVec`, `IntGauge`, `IntGaugeVec`, `LocalIntCounter`, `LocalIntCounterVec` for better performance when metric values are all integers (#158).

- Change: When the given value is < 0, `(Local)Counter.inc_by` no longer return errors, instead it will panic (#156).

- Improve performance (#161).
