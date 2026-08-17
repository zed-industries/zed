# Borsh in Rust &emsp; [![Latest Version]][crates.io] [![borsh: rustc 1.67+]][Rust 1.67] [![License Apache-2.0 badge]][License Apache-2.0] [![License MIT badge]][License MIT]

[Borsh]: https://borsh.io
[Latest Version]: https://img.shields.io/crates/v/borsh.svg
[crates.io]: https://crates.io/crates/borsh
[borsh: rustc 1.67+]: https://img.shields.io/badge/rustc-1.67+-lightgray.svg
[Rust 1.67]: https://blog.rust-lang.org/2023/01/26/Rust-1.67.0.html
[License Apache-2.0 badge]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[License Apache-2.0]: https://opensource.org/licenses/Apache-2.0
[License MIT badge]: https://img.shields.io/badge/license-MIT-blue.svg
[License MIT]: https://opensource.org/licenses/MIT

**borsh-rs** is Rust implementation of the [Borsh] binary serialization format.

Borsh stands for _Binary Object Representation Serializer for Hashing_. It is meant to be used in
security-critical projects as it prioritizes [consistency, safety, speed][Borsh], and comes with a
strict [specification](https://github.com/near/borsh#specification).

## Example

```rust
use borsh::{BorshSerialize, BorshDeserialize, from_slice, to_vec};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
struct A {
    x: u64,
    y: String,
}

#[test]
fn test_simple_struct() {
    let a = A {
        x: 3301,
        y: "liber primus".to_string(),
    };
    let encoded_a = to_vec(&a).unwrap();
    let decoded_a = from_slice::<A>(&encoded_a).unwrap();
    assert_eq!(a, decoded_a);
}
```

## Docs shortcuts

Following pages are highlighted here just to give reader a chance at learning that
they exist.  

- [Derive Macro `BorshSerialize`](./borsh/docs/rustdoc_include/borsh_serialize.md)
- [Derive Macro `BorshDeserialize`](./borsh/docs/rustdoc_include/borsh_deserialize.md)
- [Derive Macro `BorshSchema`](./borsh/docs/rustdoc_include/borsh_schema.md)

## Advanced examples 

Some of the less trivial examples are present in [examples](./borsh/examples) folder:

- [implementing `BorshSerialize`/`BorshDeserialize` for third-party `serde_json::Value`](./borsh/examples/serde_json_value.rs)

## Testing

Integration tests should generally be preferred to unit ones. Root module of integration tests of `borsh` crate is [linked](./borsh/tests/tests.rs) here.
 
## Releasing

The versions of all public crates in this repository are collectively managed by a single version in the [workspace manifest](https://github.com/near/borsh-rs/blob/master/Cargo.toml).

So, to publish a new version of all the crates, you can do so by simply bumping that to the next "patch" version and submit a PR.

We have CI Infrastructure put in place to automate the process of publishing all crates once a version change has merged into master.

However, before you release, make sure the [CHANGELOG](CHANGELOG.md) is up to date and that the `[Unreleased]` section is present but empty.

## License

This repository is distributed under the terms of both the MIT license and the Apache License (Version 2.0).
See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for details.
