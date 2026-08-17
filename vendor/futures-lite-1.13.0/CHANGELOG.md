# Version 1.13.0

- Unbind Debug implementations of BufReader and BufWriter. (#49)
- Add the once_future() combinator. (#59)
- Add a combinator for temporarily using an AsyncRead/AsyncWrite as Read/Write. (#62)
- Implement more methods for stream::BlockOn. (#68)

# Version 1.12.0

- Implement `BufRead` for `BlockOn`

# Version 1.11.3

- Update `pin-project-lite`.

# Version 1.11.2

- Improve docs for `ready!`.

# Version 1.11.1

- Fix some typos.

# Version 1.11.0

- Add the new `prelude` module.
- Deprecate trait re-exports in the root module.

# Version 1.10.1

- Fix compilation errors with Rust 1.42.0 and 1.45.2

# Version 1.10.0

- Add `io::split()`.

# Version 1.9.0

- Add `FutureExt::poll()`.
- Add `StreamExt::poll_next()`.
- Add `AsyncBufReadExt::fill_buf()`.
- Add `AsyncBufReadExt::consume()`.

# Version 1.8.0

- Add `BoxedReader` and `BoxedWriter`.

# Version 1.7.0

- Implement `AsyncRead` for `Bytes`.
- Add `StreamExt::then()`.

# Version 1.6.0

- Add `FutureExt::catch_unwind()`.

# Version 1.5.0

- Add `stream::race()` and `StreamExt::race()`.

# Version 1.4.0

- Add `alloc` Cargo feature.

# Version 1.3.0

- Add `future::or()`.
- Add `FutureExt::race()`.
- Disable `waker-fn` dependency on `#![no_std]` targets.

# Version 1.2.0

- Fix compilation errors on `#![no_std]` systems.
- Add `StreamExt::try_next()`.
- Add `StreamExt::partition()`.
- Add `StreamExt::for_each()`.
- Add `StreamExt::try_for_each()`.
- Add `StreamExt::zip()`.
- Add `StreamExt::unzip()`.
- Add `StreamExt::nth()`.
- Add `StreamExt::last()`.
- Add `StreamExt::find()`.
- Add `StreamExt::find_map()`.
- Add `StreamExt::position()`.
- Add `StreamExt::all()`.
- Add `StreamExt::any()`.
- Add `StreamExt::scan()`.
- Add `StreamExt::flat_map()`.
- Add `StreamExt::flatten()`.
- Add `StreamExt::skip()`.
- Add `StreamExt::skip_while()`.

# Version 1.1.0

- Add `StreamExt::take()`.
- Add `StreamExt::take_while()`.
- Add `StreamExt::step_by()`.
- Add `StreamExt::fuse()`.
- Add `StreamExt::chain()`.
- Add `StreamExt::cloned()`.
- Add `StreamExt::copied()`.
- Add `StreamExt::cycle()`.
- Add `StreamExt::enumeraate()`.
- Add `StreamExt::inspect()`.
- Parametrize `FutureExt::boxed()` and `FutureExt::boxed_local()` over a lifetime.
- Parametrize `StreamExt::boxed()` and `StreamExt::boxed_local()` over a lifetime.

# Version 1.0.0

- Add `StreamExt::map()`.
- Add `StreamExt::count()`.
- Add `StreamExt::filter()`.
- Add `StreamExt::filter_map()`.
- Rename `future::join()` to `future::zip()`.
- Rename `future::try_join()` to `future::try_zip()`.

# Version 0.1.11

- Update `parking` to v2.0.0

# Version 0.1.10

- Add `AssertAsync`.

# Version 0.1.9

- Add `FutureExt::or()`.
- Put `#[must_use]` on all futures and streams.

# Version 0.1.8

- Fix lints about unsafe code.

# Version 0.1.7

- Add blocking APIs (`block_on()` and `BlockOn`).

# Version 0.1.6

- Add `boxed()`, `boxed_local()`, `Boxed`, and `BoxedLocal`.

# Version 0.1.5

- Add `fold()` and `try_fold()`.

# Version 0.1.4

- Add `future::race()`.
- Fix a bug in `BufReader`.

# Version 0.1.3

- Add `future::join()`, `future::try_join()`, and `AsyncWriteExt::close()`.

# Version 0.1.2

- Lots of new APIs.

# Version 0.1.1

- Initial version
