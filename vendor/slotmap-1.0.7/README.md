# slotmap

A Rust library providing three containers with persistent unique keys to access
stored values, `SlotMap`, `HopSlotMap` and `DenseSlotMap`. Upon insertion a key
is returned that can be used to later access or remove the values. Insertion,
deletion and access all take O(1) time with low overhead. Great for storing
collections of objects that need stable, safe references but have no clear
ownership otherwise, such as game entities or graph nodes. Two secondary maps,
`SecondaryMap` and `SparseSecondaryMap` are also provided that allow you to map
further objects to the keys created by one of the slot maps. Please refer to
[**the documentation**](https://docs.rs/slotmap) for more information.

The minimum required stable Rust version for `slotmap` is 1.49. To start using
`slotmap` add the following to your `Cargo.toml`:

```toml
[dependencies]
slotmap = "1.0"
```

# Example

A short example:

```rust
use slotmap::{SlotMap, SecondaryMap};

let mut sm = SlotMap::new();
let foo = sm.insert("foo");  // Key generated on insert.
let bar = sm.insert("bar");
assert_eq!(sm[foo], "foo");
assert_eq!(sm[bar], "bar");

sm.remove(bar);
let reuse = sm.insert("reuse");  // Space from bar reused.
assert_eq!(sm.contains_key(bar), false);  // After deletion a key stays invalid.

let mut sec = SecondaryMap::new();
sec.insert(foo, "noun");  // We provide the key for secondary maps.
sec.insert(reuse, "verb");

for (key, val) in sm {
    println!("{} is a {}", val, sec[key]);
}
```

# License

`slotmap` is released under the Zlib license, a permissive license. It is
OSI and FSF approved and GPL compatible.
