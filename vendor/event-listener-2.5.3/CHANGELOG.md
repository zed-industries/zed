# Version 2.5.3

- Fix fence on x86 and miri.

# Version 2.5.2

- Fix stacked borrows violation when `-Zmiri-tag-raw-pointers` is enabled. (#24)

# Version 2.5.1

- Replace spinlock with a mutex.

# Version 2.5.0

- Add `EventListener::discard()`.

# Version 2.4.0

- `Event::new()` is now a const fn.

# Version 2.3.3

- Fix a bug in `List::insert()` that was causing deadlocks.

# Version 2.3.2

- Optimization: use a simple spinlock and cache an `Entry` for less allocation.

# Version 2.3.1

- Optimization: don't initialize `Inner` when notifying `Event`.

# Version 2.3.0

- Implement `UnwindSafe`/`RefUnwindSafe` for `Event`/`EventListener`.

# Version 2.2.1

- Always keep the last waker in `EventListener::poll()`.

# Version 2.2.0

- Add `EventListener::same_event()`.

# Version 2.1.0

- Add `EventListener::listens_to()`.

# Version 2.0.1

- Replace `usize::MAX` with `std::usize::MAX`.

# Version 2.0.0

- Remove `Event::notify_one()` and `Event::notify_all()`.
- Add `Event::notify_relaxed()` and `Event::notify_additional_relaxed()`.
- Dropped notified `EventListener` now notifies one *or* one additional listener.

# Version 1.2.0

- Add `Event::notify_additional()`.

# Version 1.1.2

- Change a `Relaxed` load to `Acquire` load.

# Version 1.1.1

- Fix a bug in `EventListener::wait_timeout()`.

# Version 1.1.0

- Add `EventListener::notify()`.

# Version 1.0.1

- Reduce the complexity of `notify_all()` from O(n) to amortized O(1).
- Fix a bug where entries were notified in wrong order.
- Add tests.

# Version 1.0.0

- Initial version.
