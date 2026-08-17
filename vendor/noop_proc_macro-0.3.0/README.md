# NoOp proc macro

NoOp mocks for `proc_macros` that you might want to make optional.

``` rust
#[cfg(feature = "serde")]
pub(crate) use serde_derive::{Serialize, Deserialize};

#[cfg(not(feature = "serde")]
pub(crate) use noop_proc_macro::{Serialize, Deserialize};
```

## Supported `proc_macros`

- [Serde](https://serde.rs)
- [rust-hawktracer](https://github.com/AlexEne/rust_hawktracer)
- [wasm_bindgen](https://github.com/rustwasm/wasm-bindgen)
