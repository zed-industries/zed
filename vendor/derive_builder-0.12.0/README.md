![Build](https://github.com/colin-kiegel/rust-derive-builder/workflows/Build/badge.svg)
[![Rust version](https://img.shields.io/badge/rust-1.40+-blue.svg)]()
[![Documentation](https://docs.rs/derive_builder/badge.svg)](https://docs.rs/derive_builder)
[![Latest version](https://img.shields.io/crates/v/derive_builder.svg)](https://crates.io/crates/derive_builder)
[![All downloads](https://img.shields.io/crates/d/derive_builder.svg)](https://crates.io/crates/derive_builder)
[![Downloads of latest version](https://img.shields.io/crates/dv/derive_builder.svg)](https://crates.io/crates/derive_builder)

# Builder Pattern Derive

[Rust][rust] macro to automatically implement the **builder pattern** for arbitrary structs. A simple `#[derive(Builder)]` will generate a `FooBuilder` for your struct `Foo` with all setter-methods and a build method.

## How it Works

```rust
#[macro_use]
extern crate derive_builder;

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
struct Channel {
    token: i32,
    special_info: i32,
    // .. a whole bunch of other fields ..
}

fn main() {
    // builder pattern, go, go, go!...
    let ch = ChannelBuilder::default()
        .special_info(42u8)
        .token(19124)
        .build()
        .unwrap();
    println!("{:?}", ch);
}
```

Note that we did not write any definition or implementation of `ChannelBuilder`. Instead the `derive_builder` crate acts on `#[derive(Builder)]` and generates the necessary code at compile time.

This is the generated boilerplate code you didn't need to write. :-)

```rust,ignore
#[derive(Clone, Default)]
struct ChannelBuilder {
    token: Option<i32>,
    special_info: Option<i32>,
}

#[allow(dead_code)]
impl ChannelBuilder {
    pub fn token<VALUE: Into<i32>>(&mut self, value: VALUE) -> &mut Self {
        let mut new = self;
        new.token = Some(value.into());
        new
    }
    pub fn special_info<VALUE: Into<i32>>(&mut self, value: VALUE) -> &mut Self {
        let mut new = self;
        new.special_info = Some(value.into());
        new
    }
    fn build(
        &self,
    ) -> Result<Channel, ChannelBuilderError> {
        Ok(Channel {
            id: match self.id {
                Some(ref value) => Clone::clone(value),
                None => {
                    return Err(
                        Into::into(
                            ::derive_builder::UninitializedFieldError::from("id"),
                        ),
                    )
                }
            },
            token: match self.token {
                Some(ref value) => Clone::clone(value),
                None => {
                    return Err(
                        Into::into(
                            ::derive_builder::UninitializedFieldError::from("token"),
                        ),
                    )
                }
            },
            special_info: match self.special_info {
                Some(ref value) => Clone::clone(value),
                None => {
                    return Err(
                        Into::into(
                            ::derive_builder::UninitializedFieldError::from("special_info"),
                        ),
                    )
                }
            },
        })
    }
}
```

_Note: This is edited for readability. The generated code doesn't assume traits such as `Into` are in-scope, and uses full paths to access them._

## Get Started

It's as simple as three steps:

1. Add `derive_builder` to your `Cargo.toml` either manually or
   with [cargo-edit](https://github.com/killercup/cargo-edit):

- `cargo add derive_builder`

2. Add `use derive_builder::Builder;`
3. Annotate your struct with `#[derive(Builder)]`

## Usage and Features

- **Chaining**: The setter calls can be chained, because they consume and return `&mut self` by default.
- **Builder patterns**: You can opt into other builder patterns by preceding your struct (or field) with `#[builder(pattern = "owned")]` or `#[builder(pattern = "immutable")]`.
- **Extensible**: You can still define your own implementations for the builder struct and define additional methods. Just make sure to name them differently than the setter and build methods.
- **Documentation and attributes**: Setter methods can be documented by simply documenting the corresponding field. Similarly `#[cfg(...)]` and `#[allow(...)]` attributes are also applied to the setter methods.
- **Hidden fields**: You can skip setters via `#[builder(setter(skip))]` on each field individually.
- **Setter visibility**: You can opt into private setter by preceding your struct with `#[builder(private)]`.
- **Setter type conversions**: With `#[builder(setter(into))]`, setter methods will be generic over the input types – you can then supply every argument that implements the [`Into`][into] trait for the field type.
- **Setter strip option**: With `#[builder(setter(strip_option))]`, setter methods will take `T` as parameter'type for field of type `Option<T>`.
- **Collection setters**: Adding `#[builder(setter(each(name = "method_name")))]` to fields whose types implement `Default` and `Extend` will generate a setter which adds items to the builder collection for that field. It's possible for these setters to be generic over the `Into<T>` trait too, like so: `#[builder(setter(each(name = "foo", into)))]`.
- **Builder field visibility**: You can use `#[builder(field(private))]` or `..(public)`, to set field visibility of your builder.
- **Generic structs**: Are also supported, but you **must not** use a type parameter named `VALUE`, if you also activate setter type conversions.
- **Default values**: You can use `#[builder(default)]` to delegate to the `Default` implementation or any explicit value via ` = ".."`. This works both on the struct and field level.
- **Pre-build validation**: You can use `#[builder(build_fn(validate = "path::to::fn"))]` to add your own validation before the target struct is generated.
- **Build method suppression**: You can use `#[builder(build_fn(skip))]` to disable auto-implementation of the build method and provide your own.
- **Custom build method error types**: You can use `#[builder(build_fn(error = "path::to::Error"))]` to have your builder return an error type of your choosing. By default, the macro will emit an error type alongside the builder.
- **Builder derivations**: You can use `#[builder(derive(Trait1, Trait2, ...))]` to have the builder derive additonal traits. All builders derive `Default` and `Clone`, so you should not declare those in this attribute.
- **Pass-through attributes**: Use `#[builder_struct_attr(...)]`, `#[builder_impl_attr(...)]`, `#[builder_field_attr(...)]`, and `#[builder_setter_attr(...)]` to declare attributes that will be added to the relevant part of the generated builder.
- **no_std support**: Just add `#[builder(no_std)]` to your struct and add `extern crate alloc` to your crate.
- **Re-export support**: Use `#[builder(crate = "...")]` to set the root for `derive_builder`. This is useful if your crate is re-exporting `derive_builder::Builder` and needs the generated code to not directly reference the `derive_builder` crate.

For more information and examples please take a look at our [documentation][doc].

## Gotchas

- Renaming `derive_builder` in `Cargo.toml` is not supported.
- Tuple structs and unit structs are not supported as they have no field names. We do not intend to support them.
- When defining a generic struct, you cannot use `VALUE` as a generic parameter as this is what all setters are using.

## [Documentation][doc]

Detailed explaination of all features and tips for troubleshooting. You'll also find a discussion of different builder patterns.

[doc]: https://colin-kiegel.github.io/rust-derive-builder
[rust]: https://www.rust-lang.org/
[builder-pattern]: https://aturon.github.io/ownership/builders.html
[into]: https://doc.rust-lang.org/nightly/std/convert/trait.Into.html

## [Changelog](CHANGELOG.md)

Yes, we keep a changelog.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
