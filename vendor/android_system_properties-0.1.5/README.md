# android_system_properties

A thin rust wrapper for Android system properties.

This crate is similar to the `android-properties` crate with the exception that
the necessary Android libc symbols are loaded dynamically instead of linked
statically. In practice this means that the same binary will work with old and
new versions of Android, even though the API for reading system properties changed
around Android L.

## Example

```rust
use android_system_properties::AndroidSystemProperties;

let properties = AndroidSystemProperties::new();

if let Some(value) = properties.get("persist.sys.timezone") {
   println!("{}", value);
}
```

## Listing and setting properties

For the sake of simplicity this crate currently only contains what's needed by wgpu.
The implementations for listing and setting properties can be added back if anyone needs
them (let me know by filing an issue).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
