/*!

Working with timers on the Web: `setTimeout` and `setInterval`.

These APIs come in two flavors:

1. a callback style (that more directly mimics the JavaScript APIs), and
2. a `Future`s and `Stream`s API.

## Timeouts

Timeouts fire once after a period of time (measured in milliseconds).

### Timeouts with a Callback Function

```no_run
use gloo_timers::callback::Timeout;

let timeout = Timeout::new(1_000, move || {
    // Do something after the one second timeout is up!
});

// Since we don't plan on cancelling the timeout, call `forget`.
timeout.forget();
```

### Timeouts as `Future`s

With the `futures` feature enabled, a `future` module containing futures-based
timers is exposed.

*/
#![cfg_attr(feature = "futures", doc = "```no_run")]
#![cfg_attr(not(feature = "futures"), doc = "```ignore")]
/*!
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;

// Spawn the `timeout` future on the local thread. If we just dropped it, then
// the timeout would be cancelled with `clearTimeout`.
spawn_local(async {
    TimeoutFuture::new(1_000).await;
    // Do something here after the one second timeout is up!
});
```

## Intervals

Intervals fire repeatedly every *n* milliseconds.

### Intervals with a Callback Function

TODO

### Intervals as `Stream`s

TODO

 */

#![deny(missing_docs, missing_debug_implementations)]

pub mod callback;

#[cfg(feature = "futures")]
pub mod future;
