//! # `console_error_panic_hook`
//!
//! [![](https://docs.rs/console_error_panic_hook/badge.svg)](https://docs.rs/console_error_panic_hook/)
//! [![](https://img.shields.io/crates/v/console_error_panic_hook.svg)](https://crates.io/crates/console_error_panic_hook)
//! [![](https://img.shields.io/crates/d/console_error_panic_hook.png)](https://crates.io/crates/console_error_panic_hook)
//! [![Build Status](https://travis-ci.org/rustwasm/console_error_panic_hook.svg?branch=master)](https://travis-ci.org/rustwasm/console_error_panic_hook)
//!
//! This crate lets you debug panics on `wasm32-unknown-unknown` by providing a
//! panic hook that forwards panic messages to
//! [`console.error`](https://developer.mozilla.org/en-US/docs/Web/API/Console/error).
//!
//! When an error is reported with `console.error`, browser devtools and node.js
//! will typically capture a stack trace and display it with the logged error
//! message.
//!
//! Without `console_error_panic_hook` you just get something like *RuntimeError: Unreachable executed*
//!
//! Browser:
//! ![Console without panic hook](without_panic_hook.png)
//!
//! Node:
//! ![Node console without panic hook](without_panic_hook_node.png)
//!
//! With this panic hook installed you will see the panic message
//!
//! Browser:
//! ![Console with panic hook set up](with_panic_hook.png)
//!
//! Node:
//! ![Node console with panic hook set up](with_panic_hook_node.png)
//!
//! ## Usage
//!
//! There are two ways to install this panic hook.
//!
//! First, you can set the hook yourself by calling `std::panic::set_hook` in
//! some initialization function:
//!
//! ```
//! extern crate console_error_panic_hook;
//! use std::panic;
//!
//! fn my_init_function() {
//!     panic::set_hook(Box::new(console_error_panic_hook::hook));
//!
//!     // ...
//! }
//! ```
//!
//! Alternatively, use `set_once` on some common code path to ensure that
//! `set_hook` is called, but only the one time. Under the hood, this uses
//! `std::sync::Once`.
//!
//! ```
//! extern crate console_error_panic_hook;
//!
//! struct MyBigThing;
//!
//! impl MyBigThing {
//!     pub fn new() -> MyBigThing {
//!         console_error_panic_hook::set_once();
//!
//!         MyBigThing
//!     }
//! }
//! ```
//!
//! ## Error.stackTraceLimit
//!
//! Many browsers only capture the top 10 frames of a stack trace. In rust programs this is less likely to be enough. To see more frames, you can set the non-standard value `Error.stackTraceLimit`. For more information see the [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Microsoft_Extensions/Error.stackTraceLimit) or [v8 docs](https://v8.dev/docs/stack-trace-api).
//!

#[macro_use]
extern crate cfg_if;

use std::panic;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        extern crate wasm_bindgen;
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        extern {
            #[wasm_bindgen(js_namespace = console)]
            fn error(msg: String);

            type Error;

            #[wasm_bindgen(constructor)]
            fn new() -> Error;

            #[wasm_bindgen(structural, method, getter)]
            fn stack(error: &Error) -> String;
        }

        fn hook_impl(info: &panic::PanicInfo) {
            let mut msg = info.to_string();

            // Add the error stack to our message.
            //
            // This ensures that even if the `console` implementation doesn't
            // include stacks for `console.error`, the stack is still available
            // for the user. Additionally, Firefox's console tries to clean up
            // stack traces, and ruins Rust symbols in the process
            // (https://bugzilla.mozilla.org/show_bug.cgi?id=1519569) but since
            // it only touches the logged message's associated stack, and not
            // the message's contents, by including the stack in the message
            // contents we make sure it is available to the user.
            msg.push_str("\n\nStack:\n\n");
            let e = Error::new();
            let stack = e.stack();
            msg.push_str(&stack);

            // Safari's devtools, on the other hand, _do_ mess with logged
            // messages' contents, so we attempt to break their heuristics for
            // doing that by appending some whitespace.
            // https://github.com/rustwasm/console_error_panic_hook/issues/7
            msg.push_str("\n\n");

            // Finally, log the panic with `console.error`!
            error(msg);
        }
    } else {
        use std::io::{self, Write};

        fn hook_impl(info: &panic::PanicInfo) {
            let _ = writeln!(io::stderr(), "{}", info);
        }
    }
}

/// A panic hook for use with
/// [`std::panic::set_hook`](https://doc.rust-lang.org/nightly/std/panic/fn.set_hook.html)
/// that logs panics into
/// [`console.error`](https://developer.mozilla.org/en-US/docs/Web/API/Console/error).
///
/// On non-wasm targets, prints the panic to `stderr`.
pub fn hook(info: &panic::PanicInfo) {
    hook_impl(info);
}

/// Set the `console.error` panic hook the first time this is called. Subsequent
/// invocations do nothing.
#[inline]
pub fn set_once() {
    use std::sync::Once;
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(|| {
        panic::set_hook(Box::new(hook));
    });
}
