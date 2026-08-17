## Windows registry

The [windows-registry](https://crates.io/crates/windows-registry) crate provides simple, safe, and efficient access to the Windows registry.

* [Getting started](https://kennykerr.ca/rust-getting-started/)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-registry]
version = "0.5"
```

Read and write registry keys and values as needed:

```rust,no_run
use windows_registry::*;

fn main() -> Result<()> {
    let key = CURRENT_USER.create("software\\windows-rs")?;

    key.set_u32("number", 123)?;
    key.set_string("name", "Rust")?;

    println!("{}", key.get_u32("number")?);
    println!("{}", key.get_string("name")?);

    Ok(())
}
```

Use the `options()` method for even more control:

```rust,no_run
use windows_registry::*;

fn main() -> Result<()> {
    let tx = Transaction::new()?;

    let key = CURRENT_USER
        .options()
        .read()
        .write()
        .create()
        .transaction(&tx)
        .open("software\\windows-rs")?;

    key.set_u32("name", 123)?;

    tx.commit()?;

    Ok(())
}
```