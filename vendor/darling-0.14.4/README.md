Darling
=======

[![Build Status](https://github.com/TedDriggs/darling/workflows/CI/badge.svg)](https://github.com/TedDriggs/darling/actions)
[![Latest Version](https://img.shields.io/crates/v/darling.svg)](https://crates.io/crates/darling)
[![Rustc Version 1.31+](https://img.shields.io/badge/rustc-1.31+-lightgray.svg)](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html)

`darling` is a crate for proc macro authors, which enables parsing attributes into structs. It is heavily inspired by `serde` both in its internals and in its API.

# Benefits
* Easy and declarative parsing of macro input - make your proc-macros highly controllable with minimal time investment.
* Great validation and errors, no work required. When users of your proc-macro make a mistake, `darling` makes sure they get error markers at the right place in their source, and provides "did you mean" suggestions for misspelled fields.

# Usage
`darling` provides a set of traits which can be derived or manually implemented.

1. `FromMeta` is used to extract values from a meta-item in an attribute. Implementations are likely reusable for many libraries, much like `FromStr` or `serde::Deserialize`. Trait implementations are provided for primitives, some std types, and some `syn` types.
2. `FromDeriveInput` is implemented or derived by each proc-macro crate which depends on `darling`. This is the root for input parsing; it gets access to the identity, generics, and visibility of the target type, and can specify which attribute names should be parsed or forwarded from the input AST.
3. `FromField` is implemented or derived by each proc-macro crate which depends on `darling`. Structs deriving this trait will get access to the identity (if it exists), type, and visibility of the field.
4. `FromVariant` is implemented or derived by each proc-macro crate which depends on `darling`. Structs deriving this trait will get access to the identity and contents of the variant, which can be transformed the same as any other `darling` input.
5. `FromAttributes` is a lower-level version of the more-specific `FromDeriveInput`, `FromField`, and `FromVariant` traits. Structs deriving this trait get a meta-item extractor and error collection which works for any syntax element, including traits, trait items, and functions. This is useful for non-derive proc macros.

## Additional Modules
* `darling::ast` provides generic types for representing the AST.
* `darling::usage` provides traits and functions for determining where type parameters and lifetimes are used in a struct or enum.
* `darling::util` provides helper types with special `FromMeta` implementations, such as `IdentList`.

# Example

```rust,ignore
#[macro_use]
extern crate darling;
extern crate syn;

#[derive(Default, FromMeta)]
#[darling(default)]
pub struct Lorem {
    #[darling(rename = "sit")]
    ipsum: bool,
    dolor: Option<String>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(my_crate), forward_attrs(allow, doc, cfg))]
pub struct MyTraitOpts {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    lorem: Lorem,
}
```

The above code will then be able to parse this input:

```rust,ignore
/// A doc comment which will be available in `MyTraitOpts::attrs`.
#[derive(MyTrait)]
#[my_crate(lorem(dolor = "Hello", sit))]
pub struct ConsumingType;
```

# Attribute Macros
Non-derive attribute macros are supported.
To parse arguments for attribute macros, derive `FromMeta` on the argument receiver type, then pass `&syn::AttributeArgs` to the `from_list` method.
This will produce a normal `darling::Result<T>` that can be used the same as a result from parsing a `DeriveInput`.

## Macro Code
```rust,ignore
use darling::FromMeta;
use syn::{AttributeArgs, ItemFn};
use proc_macro::TokenStream;

#[derive(Debug, FromMeta)]
pub struct MacroArgs {
    #[darling(default)]
    timeout_ms: Option<u16>,
    path: String,
}

#[proc_macro_attribute]
fn your_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let _input = parse_macro_input!(input as ItemFn);

    let _args = match MacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(e.write_errors()); }
    };

    // do things with `args`
    unimplemented!()
}
```

## Consuming Code
```rust,ignore
use your_crate::your_attr;

#[your_attr(path = "hello", timeout_ms = 15)]
fn do_stuff() {
    println!("Hello");
}
```

# Features
Darling's features are built to work well for real-world projects.

* **Defaults**: Supports struct- and field-level defaults, using the same path syntax as `serde`. 
  Additionally, `Option<T>` and `darling::util::Flag` fields are innately optional; you don't need to declare `#[darling(default)]` for those.
* **Field Renaming**: Fields can have different names in usage vs. the backing code.
* **Auto-populated fields**: Structs deriving `FromDeriveInput` and `FromField` can declare properties named `ident`, `vis`, `ty`, `attrs`, and `generics` to automatically get copies of the matching values from the input AST. `FromDeriveInput` additionally exposes `data` to get access to the body of the deriving type, and `FromVariant` exposes `fields`.
* **Mapping function**: Use `#[darling(map="path")]` or `#[darling(and_then="path")]` to specify a function that runs on the result of parsing a meta-item field. This can change the return type, which enables you to parse to an intermediate form and convert that to the type you need in your struct.
* **Skip fields**: Use `#[darling(skip)]` to mark a field that shouldn't be read from attribute meta-items.
* **Multiple-occurrence fields**: Use `#[darling(multiple)]` on a `Vec` field to allow that field to appear multiple times in the meta-item. Each occurrence will be pushed into the `Vec`.
* **Span access**: Use `darling::util::SpannedValue` in a struct to get access to that meta item's source code span. This can be used to emit warnings that point at a specific field from your proc macro. In addition, you can use `darling::Error::write_errors` to automatically get precise error location details in most cases.
* **"Did you mean" suggestions**: Compile errors from derived darling trait impls include suggestions for misspelled fields.

## Shape Validation
Some proc-macros only work on structs, while others need enums whose variants are either unit or newtype variants.
Darling makes this sort of validation extremely simple.
On the receiver that derives `FromDeriveInput`, add `#[darling(supports(...))]` and then list the shapes that your macro should accept.

|Name|Description|
|---|---|
|`any`|Accept anything|
|`struct_any`|Accept any struct|
|`struct_named`|Accept structs with named fields, e.g. `struct Example { field: String }`|
|`struct_newtype`|Accept newtype structs, e.g. `struct Example(String)`|
|`struct_tuple`|Accept tuple structs, e.g. `struct Example(String, String)`|
|`struct_unit`|Accept unit structs, e.g. `struct Example;`|
|`enum_any`|Accept any enum|
|`enum_named`|Accept enum variants with named fields|
|`enum_newtype`|Accept newtype enum variants|
|`enum_tuple`|Accept tuple enum variants|
|`enum_unit`|Accept unit enum variants|

Each one is additive, so listing `#[darling(supports(struct_any, enum_newtype))]` would accept all structs and any enum where every variant is a newtype variant.

This can also be used when deriving `FromVariant`, without the `enum_` prefix.