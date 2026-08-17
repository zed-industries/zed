# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# Unreleased

### Added

### Changed

### Fixed

# [0.9.8] - 2023-04-03

### Fixed

- Unsoundness in `Once::try_call_once` caused by an `Err(_)` result

# [0.9.7] - 2023-03-27

### Fixed

- Relaxed accidentally restricted `Send`/`Sync` bounds for `Mutex` guards

# [0.9.6] - 2023-03-13

### Fixed

- Relaxed accidentally restricted `Send`/`Sync` bounds for `RwLock` guards

# [0.9.5] - 2023-02-07

### Added

- `FairMutex`, a new mutex implementation that reduces writer starvation.
- A MSRV policy: Rust 1.38 is currently required

### Changed

- The crate's CI now has full MIRI integration, further improving the confidence you can have in the implementation.

### Fixed

- Ensured that the crate's abstractions comply with stacked borrows rules.
- Unsoundness in the `RwLock` that could be triggered via a reader overflow
- Relaxed various `Send`/`Sync` bound requirements to make the crate more flexible

# [0.9.4] - 2022-07-14

### Fixed

- Fixed unsoundness in `RwLock` on reader overflow
- Relaxed `Send`/`Sync` bounds for `SpinMutex` and `TicketMutex` (doesn't affect `Mutex` itself)

# [0.9.3] - 2022-04-17

### Added

- Implemented `Default` for `Once`
- `Once::try_call_once`

### Fixed

- Fixed bug that caused `Once::call_once` to incorrectly fail

# [0.9.2] - 2021-07-09

### Changed

- Improved `Once` performance by reducing the memory footprint of internal state to one byte

### Fixed

- Improved performance of `Once` by relaxing ordering guarantees and removing redundant checks

# [0.9.1] - 2021-06-21

### Added

- Default type parameter on `Once` for better ergonomics

# [0.9.0] - 2021-03-18

### Changed

- Placed all major API features behind feature flags

### Fixed

- A compilation bug with the `lock_api` feature

# [0.8.0] - 2021-03-15

### Added

- `Once::get_unchecked`
- `RelaxStrategy` trait with type parameter on all locks to support switching between relax strategies

### Changed

- `lock_api1` feature is now named `lock_api`

# [0.7.1] - 2021-01-12

### Fixed

- Prevented `Once` leaking the inner value upon drop

# [0.7.0] - 2020-10-18

### Added

- `Once::initialized`
- `Once::get_mut`
- `Once::try_into_inner`
- `Once::poll`
- `RwLock`, `Mutex` and `Once` now implement `From<T>`
- `Lazy` type for lazy initialization
- `TicketMutex`, an alternative mutex implementation
- `std` feature flag to enable thread yielding instead of spinning
- `Mutex::is_locked`/`SpinMutex::is_locked`/`TicketMutex::is_locked`
- `Barrier`

### Changed

- `Once::wait` now spins even if initialization has not yet started
- `Guard::leak` is now an associated function instead of a method
- Improved the performance of `SpinMutex` by relaxing unnecessarily conservative
  ordering requirements

# [0.6.0] - 2020-10-08

### Added

- More dynamic `Send`/`Sync` bounds for lock guards
- `lock_api` compatibility
- `Guard::leak` methods
- `RwLock::reader_count` and `RwLock::writer_count`
- `Display` implementation for guard types

### Changed

- Made `Debug` impls of lock guards just show the inner type like `std`
