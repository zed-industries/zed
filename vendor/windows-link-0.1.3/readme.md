## Linking for Windows

The [windows-link](https://crates.io/crates/windows-link) crate provides the `link` macro that simplifies linking. The `link` macro is much the same as the one provided by [windows-targets](https://crates.io/crates/windows-targets) but uses `raw-dylib` and thus does not require import lib files.

* [Getting started](https://kennykerr.ca/rust-getting-started/)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-link]
version = "0.1"
```

Use the `link` macro to define the external functions you wish to call:

```rust
windows_link::link!("kernel32.dll" "system" fn SetLastError(code: u32));
windows_link::link!("kernel32.dll" "system" fn GetLastError() -> u32);

unsafe {
    SetLastError(1234);
    assert_eq!(GetLastError(), 1234);
}
```
