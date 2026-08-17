## Windows async types

The [windows-future](https://crates.io/crates/windows-future) crate provides stock async support for Windows APIs.

* [Getting started](https://kennykerr.ca/rust-getting-started/)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-future]
version = "0.2"
```

Use the Windows async types as needed:

```rust
use windows_future::*;
use windows_result::*;

fn main() -> Result<()> {
    // This result will be available immediately.
    let ready = IAsyncOperation::ready(Ok(123));
    assert_eq!(ready.get()?, 123);

    let ready = IAsyncOperation::spawn(|| {
        // Some lengthy operation goes here...
        Ok(456)
    });

    assert_eq!(ready.get()?, 456);

    Ok(())
}
```
