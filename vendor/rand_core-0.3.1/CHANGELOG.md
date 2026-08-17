# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.1] - 2019-01-25
- Compatibility shim around version 0.4

## [0.3.0] - 2018-09-24
- Add `SeedableRng::seed_from_u64` for convenient seeding. (#537)

## [0.2.1] - 2018-06-08
- References to a `CryptoRng` now also implement `CryptoRng`. (#470)

## [0.2.0] - 2018-05-21
- Enable the `std` feature by default. (#409)
- Remove `BlockRng{64}::inner` and `BlockRng::inner_mut`; instead making `core` public
- Add `BlockRng{64}::index` and `BlockRng{64}::generate_and_set`. (#374, #419)
- Change `BlockRngCore::Results` bound to also require `AsMut<[Self::Item]>`. (#419)
- Implement `std::io::Read` for RngCore. (#434)

## [0.1.0] - 2018-04-17
(Split out of the Rand crate, changes here are relative to rand 0.4.2)
- `RngCore` and `SeedableRng` are now part of `rand_core`. (#288)
- Add modules to help implementing RNGs `impl` and `le`. (#209, #228)
- Add `Error` and `ErrorKind`. (#225)
- Add `CryptoRng` marker trait. (#273)
- Add `BlockRngCore` trait. (#281)
- Add `BlockRng` and `BlockRng64` wrappers to help implementations. (#281, #325)
- Revise the `SeedableRng` trait. (#233)
- Remove default implementations for `RngCore::next_u64` and `RngCore::fill_bytes`. (#288)
- Add `RngCore::try_fill_bytes`. (#225)

## [0.0.1] - 2017-09-14 (yanked)
Experimental version as part of the rand crate refactor.
