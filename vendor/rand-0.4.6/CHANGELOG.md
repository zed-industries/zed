# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [0.4.5] - 2019-01-25
### Platforms
- Fuchsia: Replaced fuchsia-zircon with fuchsia-cprng

## [0.4.4] - 2019-01-06
### Added
- SGX support

## [0.4.3] - 2018-08-16
### Fixed
- Use correct syscall number for PowerPC (#589)

## [0.4.2] - 2018-01-05
### Changed
- Use winapi on Windows
- Update for Fuchsia OS
- Remove dev-dependency on `log`

## [0.4.1] - 2017-12-17
### Added
- `no_std` support

## [0.4.0-pre.0] - 2017-12-11
### Added
- `JitterRng` added as a high-quality alternative entropy source using the
  system timer
- new `seq` module with `sample_iter`, `sample_slice`, etc.
- WASM support via dummy implementations (fail at run-time)
- Additional benchmarks, covering generators and new seq code

### Changed
- `thread_rng` uses `JitterRng` if seeding from system time fails
  (slower but more secure than previous method)

### Deprecated
  - `sample` function deprecated (replaced by `sample_iter`)

## [0.3.18] - 2017-11-06
### Changed
- `thread_rng` is seeded from the system time if `OsRng` fails
- `weak_rng` now uses `thread_rng` internally


## [0.3.17] - 2017-10-07
### Changed
 - Fuchsia: Magenta was renamed Zircon

## [0.3.16] - 2017-07-27
### Added
- Implement Debug for mote non-public types
- implement `Rand` for (i|u)i128
- Support for Fuchsia

### Changed
- Add inline attribute to SampleRange::construct_range.
  This improves the benchmark for sample in 11% and for shuffle in 16%.
- Use `RtlGenRandom` instead of `CryptGenRandom`


## [0.3.15] - 2016-11-26
### Added
- Add `Rng` trait method `choose_mut`
- Redox support

### Changed
- Use `arc4rand` for `OsRng` on FreeBSD.
- Use `arc4random(3)` for `OsRng` on OpenBSD.

### Fixed
- Fix filling buffers 4 GiB or larger with `OsRng::fill_bytes` on Windows


## [0.3.14] - 2016-02-13
### Fixed
- Inline definitions from winapi/advapi32, wich decreases build times


## [0.3.13] - 2016-01-09
### Fixed
- Compatible with Rust 1.7.0-nightly (needed some extra type annotations)


## [0.3.12] - 2015-11-09
### Changed
- Replaced the methods in `next_f32` and `next_f64` with the technique described
  Saito & Matsumoto at MCQMC'08. The new method should exhibit a slightly more
  uniform distribution.
- Depend on libc 0.2

### Fixed
- Fix iterator protocol issue in `rand::sample`


## [0.3.11] - 2015-08-31
### Added
- Implement `Rand` for arrays with n <= 32


## [0.3.10] - 2015-08-17
### Added
- Support for NaCl platforms

### Changed
- Allow `Rng` to be `?Sized`, impl for `&mut R` and `Box<R>` where `R: ?Sized + Rng`


## [0.3.9] - 2015-06-18
### Changed
- Use `winapi` for Windows API things

### Fixed
- Fixed test on stable/nightly
- Fix `getrandom` syscall number for aarch64-unknown-linux-gnu


## [0.3.8] - 2015-04-23
### Changed
- `log` is a dev dependency

### Fixed
- Fix race condition of atomics in `is_getrandom_available`


## [0.3.7] - 2015-04-03
### Fixed
- Derive Copy/Clone changes


## [0.3.6] - 2015-04-02
### Changed
- Move to stable Rust!


## [0.3.5] - 2015-04-01
### Fixed
- Compatible with Rust master


## [0.3.4] - 2015-03-31
### Added
- Implement Clone for `Weighted`

### Fixed
- Compatible with Rust master


## [0.3.3] - 2015-03-26
### Fixed
- Fix compile on Windows


## [0.3.2] - 2015-03-26


## [0.3.1] - 2015-03-26
### Fixed
- Fix compile on Windows


## [0.3.0] - 2015-03-25
### Changed
- Update to use log version 0.3.x


## [0.2.1] - 2015-03-22
### Fixed
- Compatible with Rust master
- Fixed iOS compilation


## [0.2.0] - 2015-03-06
### Fixed
- Compatible with Rust master (move from `old_io` to `std::io`)


## [0.1.4] - 2015-03-04
### Fixed
- Compatible with Rust master (use wrapping ops)


## [0.1.3] - 2015-02-20
### Fixed
- Compatible with Rust master

### Removed
- Removed Copy inplementaions from RNGs


## [0.1.2] - 2015-02-03
### Added
- Imported functionality from `std::rand`, including:
  - `StdRng`, `SeedableRng`, `TreadRng`, `weak_rng()`
  - `ReaderRng`: A wrapper around any Reader to treat it as an RNG.
- Imported documentation from `std::rand`
- Imported tests from `std::rand`


## [0.1.1] - 2015-02-03
### Added
- Migrate to a cargo-compatible directory structure.

### Fixed
- Do not use entropy during `gen_weighted_bool(1)`


## [Rust 0.12.0] - 2014-10-09
### Added
- Impl Rand for tuples of arity 11 and 12
- Include ChaCha pseudorandom generator
- Add `next_f64` and `next_f32` to Rng
- Implement Clone for PRNGs

### Changed
- Rename `TaskRng` to `ThreadRng` and `task_rng` to `thread_rng` (since a
  runtime is removed from Rust).

### Fixed
- Improved performance of ISAAC and ISAAC64 by 30% and 12 % respectively, by
  informing the optimiser that indexing is never out-of-bounds.

### Removed
- Removed the Deprecated `choose_option`


## [Rust 0.11.0] - 2014-07-02
### Added
- document when to use `OSRng` in cryptographic context, and explain why we use `/dev/urandom` instead of `/dev/random`
- `Rng::gen_iter()` which will return an infinite stream of random values
- `Rng::gen_ascii_chars()` which will return an infinite stream of random ascii characters

### Changed
- Now only depends on libcore!   2adf5363f88ffe06f6d2ea5c338d1b186d47f4a1
- Remove `Rng.choose()`, rename `Rng.choose_option()` to `.choose()`
- Rename OSRng to OsRng
- The WeightedChoice structure is no longer built with a `Vec<Weighted<T>>`,
  but rather a `&mut [Weighted<T>]`. This means that the WeightedChoice
  structure now has a lifetime associated with it.
- The `sample` method on `Rng` has been moved to a top-level function in the
  `rand` module due to its dependence on `Vec`.

### Removed
- `Rng::gen_vec()` was removed. Previous behavior can be regained with
  `rng.gen_iter().take(n).collect()`
- `Rng::gen_ascii_str()` was removed. Previous behavior can be regained with
  `rng.gen_ascii_chars().take(n).collect()`
- {IsaacRng, Isaac64Rng, XorShiftRng}::new() have all been removed. These all
  relied on being able to use an OSRng for seeding, but this is no longer
  available in librand (where these types are defined). To retain the same
  functionality, these types now implement the `Rand` trait so they can be
  generated with a random seed from another random number generator. This allows
  the stdlib to use an OSRng to create seeded instances of these RNGs.
- Rand implementations for `Box<T>` and `@T` were removed. These seemed to be
  pretty rare in the codebase, and it allows for librand to not depend on
  liballoc.  Additionally, other pointer types like Rc<T> and Arc<T> were not
  supported.
- Remove a slew of old deprecated functions


## [Rust 0.10] - 2014-04-03
### Changed
- replace `Rng.shuffle's` functionality with `.shuffle_mut`
- bubble up IO errors when creating an OSRng

### Fixed
- Use `fill()` instead of `read()`
- Rewrite OsRng in Rust for windows

## [0.10-pre] - 2014-03-02
### Added
- Seperate `rand` out of the standard library

