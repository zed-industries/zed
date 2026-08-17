# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# Unreleased

### Added

### Removed

### Changed

### Fixed

# [0.11.1] - 2024-10-19

### Added

- `SendSink::sender`
- `SendFut`, `SendSink`, `RecvFut`, `RecvStream`, `WeakSender`, `Iter`, `TryIter`, and `IntoIter` now implement `Debug`
- Docs now show required features

### Removed

### Changed

- `WeakSender` is now `Clone`
- `spin` feature no longer uses `std::thread::sleep` for locking except on Unix-like operating systems and Windows
- Flume is now in [casual maintenance mode](https://casuallymaintained.tech/).

### Fixed

# [0.11.0] - 2023-08-16

### Added

- `WeakSender`, a sender that doesn't keep the channel open
- `Sender/Receiver::sender_count/receiver_count`, a way to query the number of senders and receivers attached to a channel
- `Sender/Receiver::same_channel`, a way to determine whether senders and receivers are attached to the same channel

### Changed

- Relaxed some API features
- Make all remaining spinlocks opt-in

### Fixed

- Fixed a rare race condition in the async implementation

# [0.10.14] - 2022-07-21

### Fixed

- Fixed unbounded memory usage in `RecvFut::poll_inner`

# [0.10.13] - 2022-06-10

### Added

- `SendSink::sender`, to get the sender of a `SendSink`

# [0.10.12] - 2022-03-10

### Changed

- Updated `nanorand` to 0.7

# [0.10.11] - 2022-02-14

### Fixed

- Out-of-order bug when using channels asynchronously

# [0.10.10] - 2022-01-11

### Added

- `From<SendError>` and `From<RecvError>` impls for other error types
- Marked futures as `#[must_use]`

### Changes

- Switched to scheduler-driven locking by default, with a `spin` feature to reenable the old behaviour
- Minor doc improvements

# [0.10.9] - 2021-08-25

### Changed

- Switched from `spinning_top` to `spin`

# [0.10.8] - 2021-08-06

### Changed

- Updated `nanorand` to `0.6`

# [0.10.7] - 2021-06-10

### Fixed

- Removed accidental nightly-only syntax

# [0.10.6] - 2021-06-10

### Added

- `fn into_inner(self) -> T` for send errors, allowing for easy access to the unsent message

# [0.10.5] - 2021-04-26

### Added

- `is_disconnected`, `is_empty`, `is_full`, `len`, and `capacity` on future types

# [0.10.4] - 2021-04-12

### Fixed

- Shutdown-related race condition with async recv that caused spurious errors

# [0.10.3] - 2021-04-09

### Fixed

- Compilation error when enabling `select` without `eventual_fairness`

# [0.10.2] - 2021-02-07

### Fixed

- Incorrect pointer comparison in `Selector` causing missing receives

# [0.10.1] - 2020-12-30

### Removed

- Removed `T: Unpin` requirement from async traits using `pin_project`

# [0.10.0] - 2020-12-09

### Changed

- Renamed `SendFuture` to `SendFut` to be consistent with `RecvFut`
- Improved async-related documentation

### Fixed

- Updated `nanorand` to address security advisory
