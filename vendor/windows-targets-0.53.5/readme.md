## Import libs for Windows

The [windows-targets](https://crates.io/crates/windows-targets) crate includes import libs, supports semantic versioning, and optional support for raw-dylib. 

Note: As of Rust 1.71, the [windows-link](https://crates.io/crates/windows-link) crate should be preferred.

* [Getting started](https://kennykerr.ca/rust-getting-started/)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)
* [Understanding the `windows-targets` crate](https://kennykerr.ca/rust-getting-started/understanding-windows-targets.html)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-targets]
version = "0.53"
```

Use the `link` macro to define the external functions you wish to call:

```rust
windows_targets::link!("kernel32.dll" "system" fn SetLastError(code: u32));
windows_targets::link!("kernel32.dll" "system" fn GetLastError() -> u32);

unsafe {
    SetLastError(1234);
    assert_eq!(GetLastError(), 1234);
}
```
