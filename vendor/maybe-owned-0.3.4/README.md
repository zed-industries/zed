
# maybe-owned [![Build Status](https://travis-ci.org/rustonaut/maybe-owned.svg?branch=master)](https://travis-ci.org/rustonaut/maybe-owned) [![Crates.io](https://img.shields.io/crates/v/maybe-owned.svg)](https://crates.io/crates/maybe-owned) [![maybe-owned](https://docs.rs/maybe-owned/badge.svg)](https://docs.rs/maybe-owned) [![maintenance](https://img.shields.io/badge/maintenance-passively--maintained-blue.svg)](https://img.shields.io/badge/maintenance-passively--maintained-blue.svg) [![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT) [![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**provides a `MaybeOwned<'a,T>` type different to std's `Cow` it implements `From<T>` and `From<&'a T>` and does not require `ToOwned`**

---

This crate provides a `MaybeOwned<'a,T>` enum. Different to `std::borrow::Cow` it
implements `From<T>` and `From<&'a T>` and does not require a `ToOwned` implementation.
While this can be nice for API's mainly consuming T's not implementing `ToOwned` or implementing
`ToOwned` through `Clone` it also means it's borrowed version of `String` is
`&String` and not `&str` making it less performant for cases like `String` or `Vec`.


Documentation can be [viewed on docs.rs](https://docs.rs/maybe-owned).


## Example

Take a look at the [examples dir](./examples) and the documentation
for more complete examples.

The main benefit of `MaybeOwned` over `Cow` is for API design,
allowing API consumer to pass in both `T` and `&'a T`:

```rust

//... in a trait implementation
    fn register<D>(&mut self, key: SomeId, data: D)
        where D: Into<MaybeOwned<'a, Data>>
    {
        self.map.entry(key).or_insert_with(||data.into());
    }
//...

//... in usage
    // use owned data
    registry.register(id1, data_owned);
    // use a reference to the data
    registry.register(id2, &data_ref);
    // it ok to use the same reference again
    registry.register(id3, &data_ref);
//...
```




## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.


## Change Log

- `v0.3.4`:
  - Added `make_owned()` as a `to_mut()` replacement,
    but also available for `MaybeOwnedMut` and more
    clear in it's functionality.
  - Added a `as_mut()` method to `MaybeOwned` which
    return a `Option<&mut T>`
  - Added missing BorrowMut implementation
    for `MaybeOwnedMut`


- `v0.3.3`:
  - added MaybeOwnedMut

- `v0.3.2`:
  - added transitive `std::ops` implementations

- `v0.3.1`:
  - added `serde` support

