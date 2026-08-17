# `wasm-bindgen-futures`

[API Documentation][docs]

This crate bridges the gap between Rust `Future`s and JavaScript `Promise`s.

As of this version the implementation lives in [`js-sys`] under the `futures`
feature. Depending on this crate automatically activates that feature, which
gives `js_sys::Promise` a first-class `IntoFuture` implementation — meaning
you can `.await` any `Promise` directly:

```rust
use js_sys::Promise;
use wasm_bindgen::prelude::*;

async fn example(promise: Promise) -> Result<JsValue, JsValue> {
    promise.await
}
```

All public items from the previous API are re-exported unchanged for backwards
compatibility:

- [`JsFuture`] — convert a `Promise` into a named `Future` type
- [`spawn_local`] — run a `Future<Output = ()>` on the JS microtask queue
- [`future_to_promise`] — convert a `Future` into a JS `Promise`
- [`future_to_promise_typed`] — typed variant of `future_to_promise`

Under the feature flag `futures-core-03-stream` there is support for
`AsyncIterator` to `Stream` conversion via `JsStream`.

See the [API documentation][docs] for more info.

[docs]: https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/
[`js-sys`]: https://docs.rs/js-sys
[`JsFuture`]: https://docs.rs/js-sys/*/js_sys/futures/struct.JsFuture.html
[`spawn_local`]: https://docs.rs/js-sys/*/js_sys/futures/fn.spawn_local.html
[`future_to_promise`]: https://docs.rs/js-sys/*/js_sys/futures/fn.future_to_promise.html
[`future_to_promise_typed`]: https://docs.rs/js-sys/*/js_sys/futures/fn.future_to_promise_typed.html
