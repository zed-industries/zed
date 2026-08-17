**Note**: This CHANGELOG is no longer maintained for newer 0.1.x releases.
See instead the github release tags and individual git commits.

-----

# 0.1.17 - 2017-10-31

* Add a `close` method on `sink::Wait`
* Undeprecate `stream::iter` as `stream::iter_result`
* Improve performance of wait-related methods
* Tweak buffered sinks with a 0 capacity to forward directly to the underlying
  sink.
* Add `FromIterator` implementation for `FuturesOrdered` and `FuturesUnordered`.

# 0.1.16 - 2017-09-15

* A `prelude` module has been added to glob import from and pick up a whole
  bunch of useful types
* `sync::mpsc::Sender::poll_ready` has been added as an API
* `sync::mpsc::Sender::try_send` has been added as an API

# 0.1.15 - 2017-08-24

* Improve performance of `BiLock` methods
* Implement `Clone` for `FutureResult`
* Forward `Stream` trait through `SinkMapErr`
* Add `stream::futures_ordered` next to `futures_unordered`
* Reimplement `Stream::buffered` on top of `stream::futures_ordered` (much more
  efficient at scale).
* Add a `with_notify` function for abstractions which previously required
  `UnparkEvent`.
* Add `get_ref`/`get_mut`/`into_inner` functions for stream take/skip methods
* Add a `Clone` implementation for `SharedItem` and `SharedError`
* Add a `mpsc::spawn` function to spawn a `Stream` into an `Executor`
* Add a `reunite` function for `BiLock` and the split stream/sink types to
  rejoin two halves and reclaim the original item.
* Add `stream::poll_fn` to behave similarly to `future::poll_fn`
* Add `Sink::with_flat_map` like `Iterator::flat_map`
* Bump the minimum Rust version to 1.13.0
* Expose `AtomicTask` in the public API for managing synchronization around task
  notifications.
* Unify the `Canceled` type of the `sync` and `unsync` modules.
* Deprecate the `boxed` methods. These methods have caused more confusion than
  they've solved historically, so it's recommended to use a local extension
  trait or a local helper instead of the trait-based methods.
* Deprecate the `Stream::merge` method as it's less ergonomic than `select`.
* Add `oneshot::Sender::is_canceled` to test if a oneshot is canceled off a
  task.
* Deprecates `UnboundedSender::send` in favor of a method named `unbounded_send`
  to avoid a conflict with `Sink::send`.
* Deprecate the `stream::iter` function in favor of an `stream::iter_ok` adaptor
  to avoid the need to deal with `Result` manually.
* Add an `inspect` function to the `Future` and `Stream` traits along the lines
  of `Iterator::inspect`

# 0.1.14 - 2017-05-30

This is a relatively large release of the `futures` crate, although much of it
is from reworking internals rather than new APIs. The banner feature of this
release is that the `futures::{task, executor}` modules are now available in
`no_std` contexts! A large refactoring of the task system was performed in
PR #436 to accommodate custom memory allocation schemes and otherwise remove
all dependencies on `std` for the task module. More details about this change
can be found on the PR itself.

Other API additions in this release are:

* A `FuturesUnordered::push` method was added and the `FuturesUnordered` type
  itself was completely rewritten to efficiently track a large number of
  futures.
* A `Task::will_notify_current` method was added with a slightly different
  implementation than `Task::is_current` but with stronger guarantees and
  documentation wording about its purpose.
* Many combinators now have `get_ref`, `get_mut`, and `into_inner` methods for
  accessing internal futures and state.
* A `Stream::concat2` method was added which should be considered the "fixed"
  version of `concat`, this one doesn't panic on empty streams.
* An `Executor` trait has been added to represent abstracting over the concept
  of spawning a new task. Crates which only need the ability to spawn a future
  can now be generic over `Executor` rather than requiring a
  `tokio_core::reactor::Handle`.

As with all 0.1.x releases this PR is intended to be 100% backwards compatible.
All code that previously compiled should continue to do so with these changes.
As with other changes, though, there are also some updates to be aware of:

* The `task::park` function has been renamed to `task::current`.
* The `Task::unpark` function has been renamed to `Task::notify`, and in general
  terminology around "unpark" has shifted to terminology around "notify"
* The `Unpark` trait has been deprecated in favor of the `Notify` trait
  mentioned above.
* The `UnparkEvent` structure has been deprecated. It currently should perform
  the same as it used to, but it's planned that in a future 0.1.x release the
  performance will regress for crates that have not transitioned away. The
  primary primitive to replace this is the addition of a `push` function on the
  `FuturesUnordered` type. If this does not help implement your use case though,
  please let us know!
* The `Task::is_current` method is now deprecated, and you likely want to use
  `Task::will_notify_current` instead, but let us know if this doesn't suffice!

# 0.1.13 - 2017-04-05

* Add forwarding sink/stream impls for `stream::FromErr` and `sink::SinkFromErr`
* Add `PartialEq` and `Eq` to `mpsc::SendError`
* Reimplement `Shared` with `spawn` instead of `UnparkEvent`

# 0.1.12 - 2017-04-03

* Add `Stream::from_err` and `Sink::from_err`
* Allow `SendError` to be `Clone` when possible

# 0.1.11 - 2017-03-13

The major highlight of this release is the addition of a new "default" method on
the `Sink` trait, `Sink::close`. This method is used to indicate to a sink that
no new values will ever need to get pushed into it. This can be used to
implement graceful shutdown of protocols and otherwise simply indicates to a
sink that it can start freeing up resources.

Currently this method is **not** a default method to preserve backwards
compatibility, but it's intended to become a default method in the 0.2 series of
the `futures` crate. It's highly recommended to audit implementations of `Sink`
to implement the `close` method as is fit.

Other changes in this release are:

* A new select combinator, `Future::select2` was added for a heterogeneous
  select.
* A `Shared::peek` method was added to check to see if it's done.
* `Sink::map_err` was implemented
* The `log` dependency was removed
* Implementations of the `Debug` trait are now generally available.
* The `stream::IterStream` type was renamed to `stream::Iter` (with a reexport
  for the old name).
* Add a `Sink::wait` method which returns an adapter to use an arbitrary `Sink`
  synchronously.
* A `Stream::concat` method was added to concatenate a sequence of lists.
* The `oneshot::Sender::complete` method was renamed to `send` and now returns a
  `Result` indicating successful transmission of a message or not. Note that the
  `complete` method still exists, it's just deprecated.

# 0.1.10 - 2017-01-30

* Add a new `unsync` module which mirrors `sync` to the extent that it can but
  is intended to not perform cross-thread synchronization (only usable within
  one thread).
* Tweak `Shared` to work when handles may not get poll'd again.

# 0.1.9 - 2017-01-18

* Fix `Send/Sync` of a few types
* Add `future::tail_fn` for more easily writing loops
* Export SharedItem/SharedError
* Remove an unused type parameter in `from_err`

# 0.1.8 - 2017-01-11

* Fix some race conditions in the `Shared` implementation
* Add `Stream::take_while`
* Fix an unwrap in `stream::futures_unordered`
* Generalize `Stream::for_each`
* Add `Stream::chain`
* Add `stream::repeat`
* Relax `&mut self` to `&self` in `UnboundedSender::send`

# 0.1.7 - 2016-12-18

* Add a `Future::shared` method for creating a future that can be shared
  amongst threads by cloning the future itself. All derivative futures
  will resolve to the same value once the original future has been
  resolved.
* Add a `FutureFrom` trait for future-based conversion
* Fix a wakeup bug in `Receiver::close`
* Add `future::poll_fn` for quickly adapting a `Poll`-based function to
  a future.
* Add an `Either` enum with two branches to easily create one future
  type based on two different futures created on two branches of control
  flow.
* Remove the `'static` bound on `Unpark`
* Optimize `send_all` and `forward` to send as many items as possible
  before calling `poll_complete`.
* Unify the return types of the `ok`, `err`, and `result` future to
  assist returning different varieties in different branches of a function.
* Add `CpuFuture::forget` to allow the computation to continue running
  after a drop.
* Add a `stream::futures_unordered` combinator to turn a list of futures
  into a stream representing their order of completion.

# 0.1.6 - 2016-11-22

* Fix `Clone` bound on the type parameter on `UnboundedSender`

# 0.1.5 - 2016-11-22

* Fix `#![no_std]` support

# 0.1.4 - 2016-11-22

This is quite a large release relative to the previous point releases! As
with all 0.1 releases, this release should be fully compatible with the 0.1.3
release. If any incompatibilities are discovered please file an issue!

The largest changes in 0.1.4 are the addition of a `Sink` trait coupled with a
reorganization of this crate. Note that all old locations for types/traits
still exist, they're just deprecated and tagged with `#[doc(hidden)]`.

The new `Sink` trait is used to represent types which can periodically over
time accept items, but may take some time to fully process the item before
another can be accepted. Essentially, a sink is the opposite of a stream. This
trait will then be used in the tokio-core crate to implement simple framing by
modeling I/O streams as both a stream and a sink of frames.

The organization of this crate is to now have three primary submodules,
`future`, `stream`, and `sink`. The traits as well as all combinator types are
defined in these submodules. The traits and types like `Async` and `Poll` are
then reexported at the top of the crate for convenient usage. It should be a
relatively rare occasion that the modules themselves are reached into.

Finally, the 0.1.4 release comes with a new module, `sync`, in the futures
crate.  This is intended to be the home of a suite of futures-aware
synchronization primitives. Currently this is inhabited with a `oneshot` module
(the old `oneshot` function), a `mpsc` module for a new multi-producer
single-consumer channel, and a `BiLock` type which represents sharing ownership
of one value between two consumers. This module may expand over time with more
types like a mutex, rwlock, spsc channel, etc.

Notable deprecations in the 0.1.4 release that will be deleted in an eventual
0.2 release:

* The `TaskRc` type is now deprecated in favor of `BiLock` or otherwise `Arc`
  sharing.
* All future combinators should be accessed through the `future` module, not
  the top-level of the crate.
* The `Oneshot` and `Complete` types are now replaced with the `sync::oneshot`
  module.
* Some old names like `collect` are deprecated in favor of more appropriately
  named versions like `join_all`
* The `finished` constructor is now `ok`.
* The `failed` constructor is now `err`.
* The `done` constructor is now `result`.

As always, please report bugs to https://github.com/rust-lang-nursery/futures-rs and
we always love feedback! If you've got situations we don't cover, combinators
you'd like to see, or slow code, please let us know!

Full changelog:

* Improve scalability of `buffer_unordered` combinator
* Fix a memory ordering bug in oneshot
* Add a new trait, `Sink`
* Reorganize the crate into three primary modules
* Add a new `sync` module for synchronization primitives
* Add a `BiLock` sync primitive for two-way sharing
* Deprecate `TaskRc`
* Rename `collect` to `join_all`
* Use a small vec in `Events` for improved clone performance
* Add `Stream::select` for selecting items from two streams like `merge` but
  requiring the same types.
* Add `stream::unfold` constructor
* Add a `sync::mpsc` module with a futures-aware multi-producer single-consumer
  queue. Both bounded (with backpressure) and unbounded (no backpressure)
  variants are provided.
* Renamed `failed`, `finished`, and `done` combinators to `err`, `ok`, and
  `result`.
* Add `Stream::forward` to send all items to a sink, like `Sink::send_all`
* Add `Stream::split` for streams which are both sinks and streams to have
  separate ownership of the stream/sink halves
* Improve `join_all` with concurrency

# 0.1.3 - 2016-10-24

* Rewrite `oneshot` for efficiency and removing allocations on send/recv
* Errors are passed through in `Stream::take` and `Stream::skip`
* Add a `select_ok` combinator to pick the first of a list that succeeds
* Remove the unnecessary `SelectAllNext` typedef
* Add `Stream::chunks` for receiving chunks of data
* Rewrite `stream::channel` for efficiency, correctness, and removing
  allocations
* Remove `Send + 'static` bounds on the `stream::Empty` type

# 0.1.2 - 2016-10-04

* Fixed a bug in drop of `FutureSender`
* Expose the channel `SendError` type
* Add `Future::into_stream` to convert to a single-element stream
* Add `Future::flatten_to_stream` to convert a future of a stream to a stream
* impl Debug for SendError
* Add stream::once for a one element stream
* Accept IntoIterator in stream::iter
* Add `Stream::catch_unwind`

# 0.1.1 - 2016-09-09

Initial release!
