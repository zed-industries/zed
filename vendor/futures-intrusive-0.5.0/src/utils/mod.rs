//! Utilities which are used within the library

use core::task::{Context, Waker};

/// Updates a `Waker` which is stored inside a `Option` to the newest value
/// which is delivered via a `Context`.
pub fn update_waker_ref(waker_option: &mut Option<Waker>, cx: &Context) {
    if waker_option
        .as_ref()
        .map_or(true, |stored_waker| !stored_waker.will_wake(cx.waker()))
    {
        *waker_option = Some(cx.waker().clone());
    }
}
