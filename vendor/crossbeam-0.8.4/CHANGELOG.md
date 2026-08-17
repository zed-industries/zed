# Version 0.8.4

- Remove dependency on `cfg-if`. (#1072)

# Version 0.8.3

- Bump the minimum supported Rust version to 1.61. (#1037)

# Version 0.8.2

- Bump the minimum supported Rust version to 1.38. (#877)

# Version 0.8.1

- Support targets that do not have atomic CAS on stable Rust (#698)

# Version 0.8.0

- Bump the minimum supported Rust version to 1.36.
- Bump `crossbeam-channel` to `0.5`.
- Bump `crossbeam-deque` to `0.8`.
- Bump `crossbeam-epoch` to `0.9`.
- Bump `crossbeam-queue` to `0.3`.
- Bump `crossbeam-utils` to `0.8`.

# Version 0.7.3

- Fix breakage with nightly feature due to rust-lang/rust#65214.
- Bump `crossbeam-channel` to `0.4`.
- Bump `crossbeam-epoch` to `0.8`.
- Bump `crossbeam-queue` to `0.2`.
- Bump `crossbeam-utils` to `0.7`.

# Version 0.7.2

- Bump `crossbeam-channel` to `0.3.9`.
- Bump `crossbeam-epoch` to `0.7.2`.
- Bump `crossbeam-utils` to `0.6.6`.

# Version 0.7.1

- Bump `crossbeam-utils` to `0.6.5`.

# Version 0.7.0

- Remove `ArcCell`, `MsQueue`, and `TreiberStack`.
- Change the interface of `ShardedLock` to match `RwLock`.
- Add `SegQueue::len()`.
- Rename `SegQueue::try_pop()` to `SegQueue::pop()`.
- Change the return type of `SegQueue::pop()` to `Result`.
- Introduce `ArrayQueue`.
- Update dependencies.

# Version 0.6.0

- Update dependencies.

# Version 0.5.0

- Update `crossbeam-channel` to 0.3.
- Update `crossbeam-utils` to 0.6.
- Add `AtomicCell`, `SharedLock`, and `WaitGroup`.

# Version 0.4.1

- Fix a double-free bug in `MsQueue` and `SegQueue`.

# Version 0.4

- Switch to the new implementation of epoch-based reclamation in
  [`crossbeam-epoch`](https://github.com/crossbeam-rs/crossbeam-epoch), fixing numerous bugs in the
  old implementation.  Its API is changed in a backward-incompatible way.
- Switch to the new implementation of `CachePadded` and scoped thread in
  [`crossbeam-utils`](https://github.com/crossbeam-rs/crossbeam-utils).  The scoped thread API is
  changed in a backward-incompatible way.
- Switch to the new implementation of Chase-Lev deque in
  [`crossbeam-deque`](https://github.com/crossbeam-rs/crossbeam-deque).  Its API is changed in a
  backward-incompatible way.
- Export channel implemented in
  [`crossbeam-channel`](https://github.com/crossbeam-rs/crossbeam-channel).
- Remove `AtomicOption`.
- Implement `Default` and `From` traits.

# Version 0.3

- Introduced `ScopedThreadBuilder` with the ability to name threads and set stack size
- `Worker` methods in the Chase-Lev deque don't require mutable access anymore
- Fixed a bug when unblocking `pop()` in `MsQueue`
- Implemented `Drop` for `MsQueue`, `SegQueue`, and `TreiberStack`
- Implemented `Default` for `TreiberStack`
- Added `is_empty` to `SegQueue`
- Renamed `mem::epoch` to `epoch`
- Other bug fixes

# Version 0.2

- Changed existing non-blocking `pop` methods to `try_pop`
- Added blocking `pop` support to Michael-Scott queue
- Added Chase-Lev work-stealing deque

# Version 0.1

- Added [epoch-based memory management](http://aturon.github.io/blog/2015/08/27/epoch/)
- Added Michael-Scott queue
- Added Segmented array queue
