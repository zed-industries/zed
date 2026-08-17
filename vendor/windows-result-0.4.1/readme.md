## Windows error handling

The [windows-result](https://crates.io/crates/windows-result) crate provides efficient Windows error handling and propagation with support for Win32, COM, and WinRT APIs.

* [Getting started](https://kennykerr.ca/rust-getting-started/)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-result]
version = "0.4"
```

Use the `HRESULT`, `Error`, and specialized `Result` types as needed:

```rust
use windows_result::*;

const S_OK: HRESULT = HRESULT(0);
const ERROR_CANCELLED: u32 = 1223;
const E_CANCELLED: HRESULT = HRESULT::from_win32(ERROR_CANCELLED);

fn main() -> Result<()> {
    S_OK.ok()?;
    let e = Error::new(E_CANCELLED, "test message");
    assert_eq!(e.code(), E_CANCELLED);
    assert_eq!(e.message(), "test message");
    Ok(())
}
```
