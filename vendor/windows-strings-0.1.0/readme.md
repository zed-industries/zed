## Windows string types

The [windows-strings](https://crates.io/crates/windows-strings) crate provides common Windows string types used by various Windows APIs.

* [Getting started](https://kennykerr.ca/rust-getting-started/)
* [Samples](https://github.com/microsoft/windows-rs/tree/0.58.0/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-strings]
version = "0.1"
```

Use the Windows string types as needed:

```rust
use windows_strings::*;

const A: PCSTR = s!("ansi");
const W: PCWSTR = w!("wide");

fn main() -> Result<()> {
    let b = BSTR::from("bstr");
    let h = HSTRING::from("hstring");

    assert_eq!(b, "bstr");
    assert_eq!(h, "hstring");

    assert_eq!(unsafe { A.to_string()? }, "ansi");
    assert_eq!(unsafe { W.to_string()? }, "wide");

    Ok(())
}
```
