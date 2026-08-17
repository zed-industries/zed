# Instant

**This crate is no longer maintained. Please consider creating a fork or using `web-time` instead. Or reach out if
  you are interested  in taking over its maintenance.**

If you call `std::time::Instant::now()` on a WASM platform, it will panic. This crate provides a partial
replacement for `std::time::Instant` that works on WASM too. This defines the type `instant::Instant` which is:

* A struct emulating the behavior of **std::time::Instant** if you are targeting `wasm32-unknown-unknown` or `wasm32-unknown-asmjs`
**and** you enabled either the `stdweb` or the `wasm-bindgen` feature. This emulation is based on the javascript `performance.now()` function.
* A type alias for `std::time::Instant` otherwise.



Note that even if the **stdweb** or **wasm-bindgen** feature is enabled, this crate will continue to rely on `std::time::Instant`
as long as you are not targeting wasm32. This allows for portable code that will work on both native and WASM platforms.

This crate also exports the function `instant::now()` which returns a representation of the current time as an `f64`, expressed in milliseconds, in a platform-agnostic way. `instant::now()` will either:

* Call `performance.now()` when compiling for a WASM platform with the features **stdweb** or **wasm-bindgen** enabled, or using a custom javascript function.
* Return the time elapsed since the *Unix Epoch* on *native*, *non-WASM* platforms.

*Note*: The old feature, `now`, has been deprecated. `instant::now()` is always exported and the `now` feature flag no longer has any effect. It remains listed in `Cargo.toml` to avoid introducing breaking changes and may be removed in future versions.

## Examples
### Using `instant` for a native platform.
_Cargo.toml_:
```toml
[dependencies]
instant = "0.1"
```

_main.rs_:
```rust
fn main() {
    // Will be the same as `std::time::Instant`.
    let now = instant::Instant::now();
}
```

-----

### Using `instant` for a WASM platform.
This example shows the use of the `stdweb` feature. It would be similar with `wasm-bindgen`.

_Cargo.toml_:
```toml
[dependencies]
instant = { version = "0.1", features = [ "stdweb" ] }
```

_main.rs_:
```rust
fn main() {
    // Will emulate `std::time::Instant` based on `performance.now()`.
    let now = instant::Instant::now();
}
```

-----

### Using `instant` for a WASM platform where `performance.now()` is not available.
This example shows the use of the `inaccurate` feature.

_Cargo.toml_:
```toml
[dependencies]
instant = { version = "0.1", features = [ "wasm-bindgen", "inaccurate" ] }
```

_main.rs_:
```rust
fn main() {
    // Will emulate `std::time::Instant` based on `Date.now()`.
    let now = instant::Instant::now();
}
```


-----

### Using `instant` for any platform enabling a feature transitively.
_Cargo.toml_:
```toml
[features]
stdweb = [ "instant/stdweb" ]
wasm-bindgen = [ "instant/wasm-bindgen" ]

[dependencies]
instant = "0.1"
```

_lib.rs_:
```rust
fn my_function() {
    // Will select the proper implementation depending on the
    // feature selected by the user.
    let now = instant::Instant::now();
}
```

-----

### Using `instant::now()`
_Cargo.toml_:
```toml
[features]
stdweb = [ "instant/stdweb" ]
wasm-bindgen = [ "instant/wasm-bindgen" ]

[dependencies]
instant = "0.1"
```

_lib.rs_:
```rust
fn my_function() {
    // Will select the proper implementation depending on the
    // feature selected by the user.
    let now_instant = instant::Instant::now();
    let now_milliseconds = instant::now(); // In milliseconds.
}
```

### Using the feature `now` without `stdweb` or `wasm-bindgen`.
_Cargo.toml_:
```toml
[dependencies]
instant = "0.1"
```

_lib.rs_:
```rust
fn my_function() {
    // Will use the 'now' javascript implementation.
    let now_instant = instant::Instant::now();
    let now_milliseconds = instant::now(); // In milliseconds.
}
```

_javascript WASM bindings file_:
```js
function now() {
	return Date.now() / 1000.0;
}
```
