# Changelog

## v0.4.2 (2024-10-25)

* Updated to `wasm-bindgen` 0.2.95 and `web-sys` 0.3.72.
* Used more `web-sys` types directly for the crate's internals.

## v0.4.1 (2024-09-28)

* Fixed "closure invoked recursively or after being dropped" when dropping `IntoStream` and `IntoAsyncRead`. ([#24](https://github.com/MattiasBuelens/wasm-streams/issues/24), [#25](https://github.com/MattiasBuelens/wasm-streams/pull/25))

## v0.4.0 (2023-10-31)

* Added `ReadableStream::from(async_iterable)` and `try_from(async_iterable)`. ([#23](https://github.com/MattiasBuelens/wasm-streams/pull/23))
* Stop calling `byobRequest.respond(0)` on cancel ([#16](https://github.com/MattiasBuelens/wasm-streams/pull/16))
* ⚠ **Breaking change:** The system modules (`readable::sys`, `writable::sys` and `transform::sys`) now re-export directly from [the `web-sys` crate](https://docs.rs/web-sys/latest/web_sys/). This should make it easier to use `from_raw()`, `as_raw()` and `into_raw()`. ([#22](https://github.com/MattiasBuelens/wasm-streams/pull/22))

## v0.3.0 (2022-10-16)

* Added support for web workers, by removing usage of [JavaScript snippets](https://rustwasm.github.io/docs/wasm-bindgen/reference/js-snippets.html). ([#13](https://github.com/MattiasBuelens/wasm-streams/issues/13), [#14](https://github.com/MattiasBuelens/wasm-streams/pull/14))
* ⚠ **Breaking change:** This removes a workaround for [Chromium bug #1187774](https://crbug.com/1187774) that was previously needed for `ReadableStream::from_async_read`. This bug was fixed upstream in March 2021 with Chrome 91. ([#14](https://github.com/MattiasBuelens/wasm-streams/pull/14))
* Updated documentation of `ReadableStream(Default|BYOB)Reader::release_lock()` around the expected behavior when there are pending read requests.
  See the corresponding [Streams specification change](https://github.com/whatwg/streams/commit/d5f92d9f17306d31ba6b27424d23d58e89bf64a5) for details.
  ([#15](https://github.com/MattiasBuelens/wasm-streams/pull/15)) 

## v0.2.3 (2022-05-18)

* Replaced `futures` dependency with `futures-util` to improve compilation times ([#11](https://github.com/MattiasBuelens/wasm-streams/pull/11), [#12](https://github.com/MattiasBuelens/wasm-streams/pull/12))

## v0.2.2 (2021-12-09)

* Added `WritableStream::into_async_write()` to turn a `WritableStream` accepting `Uint8Array`s 
  into an `AsyncWrite` ([#9](https://github.com/MattiasBuelens/wasm-streams/issues/9),
  [#10](https://github.com/MattiasBuelens/wasm-streams/pull/10))
* Added `IntoSink::abort()` ([#10](https://github.com/MattiasBuelens/wasm-streams/pull/10))

## v0.2.1 (2021-09-23)

* `ReadableStream::into_stream()` and `ReadableStream::into_async_read()` now automatically 
  cancel the stream when dropped ([#7](https://github.com/MattiasBuelens/wasm-streams/issues/7), [#8](https://github.com/MattiasBuelens/wasm-streams/pull/8))
* Added `IntoStream::cancel()` and `IntoAsyncRead::cancel()` ([#8](https://github.com/MattiasBuelens/wasm-streams/pull/8))

## v0.2.0 (2021-06-22)

* Add support for readable byte streams ([#6](https://github.com/MattiasBuelens/wasm-streams/pull/6))
    * Add `ReadableStream::(try_)get_byob_reader` to acquire
      a [BYOB reader](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader).
    * Add `ReadableStream::from_async_read` to turn
      an [`AsyncRead`](https://docs.rs/futures/0.3.15/futures/io/trait.AsyncRead.html)
      into a readable byte stream.
    * Add `ReadableStream::(try_)into_async_read` to turn a readable byte stream into
      an [`AsyncRead`](https://docs.rs/futures/0.3.15/futures/io/trait.AsyncRead.html).
* Improve error handling and drop behavior of `ReadableStream::from_stream()`

## v0.1.2 (2020-10-31)

* Include license files in repository ([#5](https://github.com/MattiasBuelens/wasm-streams/issues/5))

## v0.1.1 (2020-08-08)

* Specify TypeScript type for raw streams ([#1](https://github.com/MattiasBuelens/wasm-streams/pull/1))

## v0.1.0 (2020-06-15)

First release! 🎉
