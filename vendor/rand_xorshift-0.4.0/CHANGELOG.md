# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2025-01-27
- Bump the MSRV to 1.63 (#58)
- Update to `rand_core` v0.9.0 (#58)
- Rename feature `serde1` to `serde` (#58)
- Document how zero seeds are handled
- Correctly document MSRV as 1.36
- Speed up `from_seed` implementation for 128-bit seeds
- Add examples for initializing the RNGs

## [0.3.0] - 2020-12-18
- Bump `rand_core` version to 0.6 (#17)
- Derive PartialEq+Eq for XorShiftRng (#6)
- Bump serde to 1.0.118 so that `serde1` feature can also be no-std (#12)

## [0.2.0] - 2019-06-12
- Bump minor crate version since rand_core bump is a breaking change
- Switch to Edition 2018

## [0.1.2] - 2019-06-06 - yanked
- Bump `rand_core` version
- Make XorShiftRng::from_rng portable by enforcing Endianness (#815)

## [0.1.1] - 2019-01-04
- Reorganise code and tests; tweak doc

## [0.1.0] - 2018-07-16
- Pulled out of the Rand crate
