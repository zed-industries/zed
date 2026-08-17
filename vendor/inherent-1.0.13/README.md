\#\[inherent\]
==============

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/inherent-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/inherent)
[<img alt="crates.io" src="https://img.shields.io/crates/v/inherent.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/inherent)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-inherent-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/inherent)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/inherent/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/dtolnay/inherent/actions?query=branch%3Amaster)

This crate provides an attribute macro to make trait methods callable without
the trait in scope.

```toml
[dependencies]
inherent = "1.0"
```

## Example

```rust
mod types {
    use inherent::inherent;

    trait Trait {
        fn f(self);
    }

    pub struct Struct;

    #[inherent]
    impl Trait for Struct {
        pub fn f(self) {}
    }
}

fn main() {
    // types::Trait is not in scope, but method can be called.
    types::Struct.f();
}
```

Without the `inherent` macro on the trait impl, this would have failed with the
following error:

```console
error[E0599]: no method named `f` found for type `types::Struct` in the current scope
  --> src/main.rs:18:19
   |
8  |     pub struct Struct;
   |     ------------------ method `f` not found for this
...
18 |     types::Struct.f();
   |                   ^
   |
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `f`, perhaps you need to implement it:
           candidate #1: `types::Trait`
```

The `inherent` macro expands to inherent methods on the `Self` type of the trait
impl that forward to the trait methods. In the case above, the generated code
would be:

```rust
impl Struct {
    pub fn f(self) {
        <Self as Trait>::f(self)
    }
}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
