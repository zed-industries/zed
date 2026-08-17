# Change Log

## Unreleased

## 0.14.3 - 2025-03-08

- Add `WeakLoopHandle`. (#216)

## 0.14.2 -- 2024-12-03

#### Bugfixes

- Simplify the way timeouts are calculated for `Poll`. (#214)

## 0.14.1 -- 2024-09-05

#### Additions

- Use the `tracing` crate for logging instead of the `log` crate. (#195)
- Make the `Generic` type `UnwindSafe`, even if its error is not. (#204)

#### Bugfixes

- Optimizer timers in such a way that cancelling and restarting a timer doesn't
  allocate on the heap. (#184)
- Make it so the number of channel elements are bounded per iteration of the
  event loop. (#200)

## 0.14.0 -- 2024-06-01

#### Breaking Changes

- Remove `nix` from the public API. This replaces `Signal` with a home-grown
  `Signal` enum and `siginfo` with a structure that provides most of its fields. (#182)
- Replace `thiserror` usage with manual implementations. This may cause the API to be slightly different in some cases, but should mostly be identical. (#186)

#### Additions

- Improve the MSRV policy. Now, changes to the MSRV are no longer considered breaking changes, improving stability. (#189)

#### Bugfixes

- Bump `nix` to v0.29. (#188)

## 0.13.0 -- 2024-02-25

#### Breaking changes

- Bump `nix` to `v0.28`. As `nix` is exposed in the public API, this is a
  breaking change. (#176)

#### Bugfixes

- Fix a panic that would occur when a task is scheduled in an event callback.
  (#172)

## 0.12.4 -- 2024-01-15

#### Additions

- `calloop` is now supported for Windows. (#168)
- Add the `signals` feature to `docs.rs`. (#166)

#### Bugfixes

- Fix a borrow error that can occur while using the executor. (#165)

## 0.12.3 -- 2023-10-10

#### Additions

- `Token` and `RegistrationToken` are now invalidated when the event source they represent is removed from the event loop.
- Implement `AsRawFd` and `AsFd` for `EventLoop<'l, Data>`

#### Bugfixes

- Fix an issue, where id-reuse could execute a PostAction on a newly registered event source

## 0.12.2 -- 2023-09-25

#### Bugfixes

- Fix an issue where the `Generic` event source would try to unregister its contents from the event loop
  after a failed registration.
- Fix an issue where the `EventLoop` would panic when processing a `PostAction::Remove` from an event source
  with subsources.

## 0.12.1 -- 2023-09-19

#### Bugfixes

- Fix `EventSource::before_handle_events()` being erroneously give an iterator over synthetic events instead of real events

## 0.12.0 -- 2023-09-11

#### Breaking changes

- Bump MSRV to 1.63
- Make signals an optional feature under the `signals` features.
- Replace the `nix` crate with standard library I/O errors and the `rustix` crate.
- `pre_run` and `post_run` on `EventSource` have been replaced with `before_sleep` and `before_handle_events`, respectively.
  These are now opt-in through the `NEEDS_EXTRA_LIFECYCLE_EVENTS` associated constant, and occur at slightly different times to
  the methods they are replacing. This allows greater compatibility with Wayland based event sources.

## 0.11.0 -- 2023-06-05

#### Bugfixes

- Fixed a crash due to double borrow when handling pre/post run hooks
- Fixes a panic that can occur when large `Duration`s are passed to `Timer::from_duration`.
- Replace the `sys` module with the `polling` crate.

#### Additions

- With the `block_on` feature enabled, the `EventLoop` method now has a `block_on` method that runs a future to completion on the event loop.

#### Breaking changes

- Bump MSRV to 1.56
- **Breaking:** The `TransientSource` is now an opaque type. It provides API methods for removing or
  replacing the wrapped source. This mitigates a potential leak of registration data if the
  TransientSource is replaced by direct assignment in a parent source.
- **Breaking:** `Timer::current_deadline` returns `Option<Instant>`, so that it can return `None`
  in the event of an overflow.
- **Breaking:** Use `AsFd` instead of `AsRawFd`/`RawFd`.

## 0.10.2 -- 2022-11-08

#### Bugfixes

- The return value of `LoopHandle::insert_idle` no longer borrows the `LoopHandle`.

## 0.10.1 -- 2022-06-20

#### Additions

- The `Channel` now has proxy methods for `Receiver::recv` and `Receiver::try_recv`
- Enable support for `target_os = "android"`

## 0.10.0 -- 2022-05-06

- **Breaking:** Calloop's internal storage is now backed by a `slotmap`. As a result the
  `RegistrationToken` is now `Copy+Clone`, and the low-level registration API of `Poll` is
  altered in a breaking way. MSRV is bumped to 1.49.
- **Breaking:** `generic::Fd` adapter is removed, as since that rust version `RawFd` implements
  `AsRawFd`, allowing it to be used directly in `Generic`.
- **Breaking:** The `EventSource` trait has a new associated type `Error`. This determines the type
  of the error variant returned by `EventSource::process_events()`. It must be convertible into
  `Box<dyn std::error::Error + Sync + Send>`.
- **Breaking:** All library-provided event sources now have their own error types for the
  associated `Error` type on the `EventSource` trait.
- **Breaking:** Many API functions now use Calloop's own error type (`calloop::Error`) instead of
  `std::io::Error` as the error variants of their returned results.
- **Breaking:** The `Timer` event source has been completely reworked and is now directly driven by
  calloop polling mechanism instead of a background thread. Timer multiplexing is now handled by
  creating multiple `Timer`s, and self-repeating timers is handled by the return value of the
  associated event callback.
- **Breaking:** The minimum supported Rust version is now 1.53.0
- Introduce `EventLoop::try_new_high_precision()` for sub-millisecond accuracy in the event loop
- The `PingSource` event source now uses an `eventfd` instead of a pipe on Linux.

## 0.9.2 -- 2021-12-27

#### Additions

- Introduce the methods `pre_run()` and `post_run()` to `EventSource`, allowing event sources
  to do preparations before entering a run/dispatch session, and cleanup afterwards. They have default
  implementations doing nothing.

## 0.9.1 -- 2021-08-10

- Update `nix` dependency to 0.22

## 0.9.0 -- 2021-06-29

#### Breaking changes

- MSRV is now 1.41
- The `futures` module now has a proper error type for `Scheduler::schedule()`
- The return type of `EventSource::process_events()` is now `io::Result<PostAction>` allowing
  the sources to directly request the event loop to reregister/disable/destroy them.
- The `Token` creation mechanism is now driven by a `TokenFactory`, that
  dynamically generates new unique token for sub-sources. Following for this
  if you create a new event source that is not built by composing the ones
  provided by calloop, you need to check if the `Token` provided to
  `process_events` is the same as the one you created when (re)registering
  your source. If you delegate `process_events` to a sub-source, you no longer
  need to check the `sub_id` before, instead the source you are delegating to
  is responsible to to this check.

#### Bugfixes

- Cancelling a timeout no longer prevents later timeouts from firing.

## 0.8.0 -- 2021-05-30

#### Breaking changes

- The `Dispatcher` type no longer has the closure type within its type parameters,
  but instead now has an explicit lifetime parameter, as well as the source type `S`
  and the event loop `Data` type. This allows the type to be explicitly named and
  stored into an other struct.

#### Additions

- `Token` now has a method `with_sub_id()` that returns a copy of the token
  but with the given `sub_id`.

## 0.7.2 -- 2021-02-09

#### Changes

- `EventLoop::run()` now accepts `Into<Option<Duration>>`, like `EventLoop::dispatch()`

#### Bugfixes

- The `Ping` event source now automatically disables itself when its sending end is
  dropped, preventing to always be considered readable (which caused a busy-loop).
  This also fixes a similar behavior of `Executor` and `Channel`, which use `Ping`
  internally.

## 0.7.0 -- 2020-10-13

#### Breaking Changes

- The return type for `LoopHandle::insert_source` was renamed as
  `RegistrationToken` and can be used in `{enable,disable,update,remove,kill}`
  just like before.
- Allow non-`'static` event sources and callbacks, so they can hold references
  to other values.
  - `LoopHandle::with_source` was removed. To achieve the same behaviour, use a
    `Dispatcher` and register it via the `LoopHandle::register_dispatcher`. The
    `EventSource` will be available using `Dispatcher::as_source_{ref,mut}`.
  - `LoopHandle::remove` doesn't return the event source any more. To achieve
    the same behaviour, use a `Dispatcher` and register it via the
    `LoopHandle::register_dispatcher`. After removing the `EventSource` with
    `LoopHandle::remove`, you will be able to call
    `Dispatcher::into_source_inner` to get ownership of the `EventSource`.
  - `LoopHandle::register_dispatcher` can be used in place of
    `LoopHandle::insert_source` when the source needs to be accessed after
    its insertion in the loop.
- `Interest` is changed into a struct to allow empty interest queries

#### Additions

- Introduce a futures executor as a new event source, behind the `executor` cargo
  feature.
- Introduce the `LoopHandle::adapt_io` method for creating `Async<F>` adapters to
  adapt IO objects for async use, powered by the event loop.

## 0.6.5 -- 2020-10-07

#### Fixes

- Channel now signals readinnes after the event has actually been sent, fixing a race
  condition where the event loop would try to read the message before it has been
  written.

## 0.6.4 -- 2020-08-30

#### Fixes

- Fix double borrow during dispatch when some event source is getting removed

## 0.6.3 -- 2020-08-27

#### Aditions

- Add support for `openbsd`, `netbsd`, and `dragonfly`.
- `InsertError<E>` now implements `std::error::Error`.

#### Changes

- Allow non-`'static` dispatch `Data`. `Data` is passed as an argument to the
  `callback`s while dispatching. This change allows defining `Data` types which
  can hold references to other values.
- `dispatch` now will retry on `EINTR`.

## 0.6.2 -- 2020-04-23

- Update the README and keywords for crates.io

## 0.6.1 -- 2020-04-22

- Introduce `LoopHandle::kill` to allow dropping a source from within its callback

## 0.6.0 -- 2020-04-22

- Drop the `mio` dependency
- **Breaking Change**: Significantly rework the `calloop` API, notably:
  - Event sources are now owned by the `EventLoop`
  - Users can now again set the polling mode (Level/Edge/OneShot)
- Introduce the `Ping` event source

## 0.5.2 -- 2020-04-14

- `channel::Channel` is now `Send`, allowing you to create a channel in one thread and sending
  its receiving end to an other thread for event loop insertion.

## 0.5.1 -- 2020-03-14

- Update `mio` to `0.7`

## 0.5.0 -- 2020-02-07

- Update to 2018 edition
- Update `nix` dependency to `0.17`
- **Breaking** Update `mio` dependency to `0.7.0-alpha.1`. The API of `calloop` for custom
  event sources significantly changed, and the channel and timer event sources are now
  implemented in `calloop` rather than pulled from `mio-extras`.

## 0.4.4 -- 2019-06-13

- Update `nix` dependency to `0.14`

## 0.4.3 -- 2019-02-17

- Update `mio` dependency
- Update `nix` dependency

## 0.4.2 -- 2018-11-15

- Implement `Debug` for `InsertError`.

## 0.4.1 -- 2018-11-14

- Disable the `sources::signal` module on FreeBSD so that the library can be built on this
  platform.

## 0.4.0 -- 2018-11-04

- **Breaking** Use `mio-extras` instead of `mio-more` which is not maintained.
- **Breaking** Reexport `mio` rather than selectively re-exporting some of its types.
- Add methods to `Generic` to retrive the inner `Rc` and construct it from an `Rc`
- **Breaking** `LoopHandle::insert_source` now allows to retrieve the source on error.

## 0.3.2 -- 2018-09-25

- Fix the contents of `EventedRawFd` which was erroneously not public.

## 0.3.1 -- 2018-09-25

- introduce `EventedRawFd` as a special case for the `Generic` event source, for when
  you really have to manipulate raw fds
- Don't panic when the removal of an event source trigger the removal of an other one

## 0.3.0 -- 2018-09-10

- Fixed a bug where inserting an event source from within a callback caused a panic.
- **[breaking]** Erase the `Data` type parameter from `Source` and `Idle`, for
  improved ergonomics.

## 0.2.2 -- 2018-09-10

- Introduce an `EventLoop::run` method, as well as the `LoopSignal` handle allowing to
  wakeup or stop the event loop from anywhere.

## 0.2.1 -- 2018-09-01

- Use `FnOnce` for insertion in idle callbacks.

## 0.2.0 -- 2018-08-30

- **[breaking]** Add a `&mut shared_data` argument to `EventLoop::dispatch(..)` to share data
  between callbacks.

## 0.1.1 -- 2018-08-29

- `Generic` event source for wrapping arbitrary `Evented` types
- timer event sources
- UNIX signal event sources
- channel event sources

## 0.1.0 -- 2018-08-24

Initial release
