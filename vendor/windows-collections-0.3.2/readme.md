## Windows collection types

The [windows-collections](https://crates.io/crates/windows-collections) crate provides stock collection support for Windows APIs.

* [Getting started](https://kennykerr.ca/rust-getting-started/)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-collections]
version = "0.3"
```

Use the Windows collection types as needed:

```rust
use windows_collections::*;

let numbers = IIterable::<i32>::from(vec![1, 2, 3]);

for value in numbers {
    println!("{value}");
}
```

Naturally, the Windows collection types work with other Windows crates:

```rust
use windows_collections::*;
use windows_strings::*;

let greetings =
    IVectorView::<HSTRING>::from(vec![HSTRING::from("hello"), HSTRING::from("world")]);

for value in greetings {
    println!("{value}");
}

let map = std::collections::BTreeMap::from([("one".into(), 1), ("two".into(), 2)]);
let map = IMapView::<HSTRING, i32>::from(map);

assert_eq!(map.Lookup(h!("one")).unwrap(), 1);
assert_eq!(map.Lookup(h!("two")).unwrap(), 2);
```
