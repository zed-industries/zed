# Version 2.0.2

- Update docs to mention `smol-macros`. (#319)

# Version 2.0.1

- Add a mention to the documentation of `smol::spawn` that tasks spawned with
  this function don't have their destructors called when the program ends. (#312)

# Version 2.0.0

- **Breaking:** Bump subcrates to their newest major versions. (#277, #280, #281, #282, #283)
- Run the `async-process` driver on the executor. (#284)

# Version 1.3.0

- Remove the dependency on the `once_cell` crate to restore the MSRV. (#241)

# Version 1.2.5

- Bump version for docs.rs to pick up latest dependencies.

# Version 1.2.4

- Update dependencies.

# Version 1.2.3

- Bump version for docs.rs to pick up latest dependencies.

# Version 1.2.2

- Bump version for docs.rs to pick up latest dependencies.

# Version 1.2.1

- Temporarily downgrade `async-executor`.

# Version 1.2.0

- Update all dependencies.

# Version 1.1.0

- Update `async-executor`.

# Version 1.0.1

- Update dependencies.

# Version 1.0.0

- Stabilize.

# Version 0.4.2

- Update dependencies.

# Version 0.4.1

- Bring back `SMOL_THREADS`.

# Version 0.4.0

- Add `process`, `fs`, `net`, `lock`, `channel` modules.
- Update all dependencies
- Remove `smol::run()`.

# Version 0.3.3

- Add `block_on()`.
- Use `SMOL_THREADS` environment variable.

# Version 0.3.2

- Reexport `FutureExt`.

# Version 0.3.1

- Fix some typos in docs.

# Version 0.3.0

- Reexport `futures-lite`, `blocking`, `async-executor`.
- Re-introduce `smol::run()`.

# Version 0.2.0

- Split `smol` into `async-io`, `blocking`, and `multitask`.
- Big breaking change - there is now only one type `Task`.

# Version 0.1.18

- Support Rust 1.39.0

# Version 0.1.17

- Support more platforms by changing `AtomicU64` to `AtomicUsize`.
- Remove `IoEvent` and simplify reactor notification.

# Version 0.1.16

- Add `Async::readable()` and `Async::writable()`.

# Version 0.1.15

- Fix wakeups lost inside the executor.
- Fix a fairness issue in the executor.

# Version 0.1.14

- Clear the flag after every call to `react()`.

# Version 0.1.13

- Fix deadlocks caused by lost wakeups.
- Refactor the executor.

# Version 0.1.12

- Fix a bug in `Async::<UdpSocket>::recv()`.

# Version 0.1.11

- Update `wepoll-binding`.
- Reduce dependencies.
- Replace `nix` with `libc`.
- Set minimum required `tokio` version to 0.2.

# Version 0.1.10

- Fix incorrectly reported error kind when connecting fails.

# Version 0.1.9

- Switch to oneshot-style notifications on all platforms.
- Fix a bug that caused 100% CPU usage on Windows.
- Deprecate `Async::with()` and `Async::with_mut()`.
- Add `Async::read_with()`, `Async::read_with_mut()`,
  `Async::write_with()`, and `Async::write_with_mut()`.
- Fix a bug where eventfd was not closed.

# Version 0.1.8

- Revert the use of `blocking` crate.

# Version 0.1.7

- Update `blocking` to `0.4.2`.
- Make `Task::blocking()` work without `run()`.

# Version 0.1.6

- Fix a deadlock by always re-registering `IoEvent`.

# Version 0.1.5

- Use `blocking` crate for blocking I/O.
- Fix a re-registration bug when in oneshot mode.
- Use eventfd on Linux.
- More tests.
- Fix timeout rounding error in epoll/wepoll.

# Version 0.1.4

- Fix a bug in UDS async connect

# Version 0.1.3

- Fix the writability check in async connect
- More comments and documentation
- Better security advice on certificates

# Version 0.1.2

- Improved internal docs, fixed typos, and more comments

# Version 0.1.1

- Upgrade dependencies

# Version 0.1.0

- Initial release
