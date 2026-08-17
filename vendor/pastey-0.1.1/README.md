Macros for all your token pasting needs
=======================================

[<img alt="github" src="https://img.shields.io/badge/github-as1100k/pastey-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/as1100k/pastey)
[<img alt="crates.io" src="https://img.shields.io/crates/v/pastey.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/pastey)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-pastey-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/pastey)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/as1100k/pastey/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/as1100k/pastey/actions?query=branch%master)

**_`pastey` is the fork of `paste` and is aimed to be a drop-in replacement with additional features for
`paste` crate_**

<br/>

<details>
<summary>Migrating from <code>paste</code> crate</summary>

Migrating from `paste` crate to `pastey` is super simple, just change the following in your `Cargo.toml`

```diff
[dependencies]
- paste = "1"
+ pastey = "*" # Or any specific version of pastey
```

Or even better way:

```diff
[dependencies]
- paste = "1"
+ paste = { package = "pastey", version = "*" }
```

</details>

<br>

## Quick Start

Add `pastey` as your dependency in `Cargo.toml`

```toml
[dependencies]
# TODO: Replace with latest version available on crates.io
pastey = "*"
```

This approach works with any Rust compiler 1.54+.

## Pasting identifiers

Within the `paste!` macro, identifiers inside `[<`...`>]` are pasted together to
form a single identifier.

```rust
use pastey::paste;

paste! {
    // Defines a const called `QRST`.
    const [<Q R S T>]: &str = "success!";
}

fn main() {
    assert_eq!(
        paste! { [<Q R S T>].len() },
        8,
    );
}
```

<br>

## More elaborate example

The next example shows a macro that generates accessor methods for some struct
fields. It demonstrates how you might find it useful to bundle a paste
invocation inside of a macro\_rules macro.

```rust
use pastey::paste;

macro_rules! make_a_struct_and_getters {
    ($name:ident { $($field:ident),* }) => {
        // Define a struct. This expands to:
        //
        //     pub struct S {
        //         a: String,
        //         b: String,
        //         c: String,
        //     }
        pub struct $name {
            $(
                $field: String,
            )*
        }

        // Build an impl block with getters. This expands to:
        //
        //     impl S {
        //         pub fn get_a(&self) -> &str { &self.a }
        //         pub fn get_b(&self) -> &str { &self.b }
        //         pub fn get_c(&self) -> &str { &self.c }
        //     }
        paste! {
            impl $name {
                $(
                    pub fn [<get_ $field>](&self) -> &str {
                        &self.$field
                    }
                )*
            }
        }
    }
}

make_a_struct_and_getters!(S { a, b, c });

fn call_some_getters(s: &S) -> bool {
    s.get_a() == s.get_b() && s.get_c().is_empty()
}
```

<br>

## Case conversion

The `pastey` crate supports the following case modfiers:

| Modifier                           | Description                           |
|------------------------------------|---------------------------------------|
| `$var:lower`                       | Lower Case                            |
| `$var:upper`                       | Upper Case                            |
| `$var:snake`                       | [Snake Case]                          |
| `$var:camel` or `$var:upper_camel` | Upper Camel Case                      |
| `$var:lower_camel`                 | Lower Camel Case [#4]                 |
| `$var:camel_edge`                  | Covers Edge cases of Camel Case. [#3] |

_**NOTE: The pastey crate is going to be a drop in replacement to paste crate,
and will not change the behaviour of existing modifier like `lower`, `upper`,
`snake` and `camel`. For modifying the behaviour new modifiers will be created,
like `camel_edge`**_

You can also use multiple of these modifers like `$var:snake:upper` would give you
`SCREAMING_SNAKE_CASE`.

Example

```rust
use pastey::paste;

paste! {
    const [<LIB env!("CARGO_PKG_NAME"):snake:upper>]: &str = "libpastey";

    let _ = LIBPASTEY;
}
```

The precise Unicode conversions are as defined by [`str::to_lowercase`] and
[`str::to_uppercase`].

[#3]: https://github.com/AS1100K/pastey/issues/3
[#4]: https://github.com/AS1100K/pastey/issues/4
[`str::to_lowercase`]: https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase
[`str::to_uppercase`]: https://doc.rust-lang.org/std/primitive.str.html#method.to_uppercase

<br>

## Raw Identifier Generation

`pastey` now supports raw identifiers using a special raw mode. By prefixing a token with `#`
inside the paste syntax, it treats that token as a raw identifier.

```rust
use pastey::paste;

macro_rules! define_struct_and_impl {
    ($name:ident $(- $name_tail:ident)*) => {
        paste!{
            struct [< # $name:camel $( $name_tail)* >]; // '#' signals a raw identifier

            impl [< # $name:camel $( $name_tail)* >] {
                fn [< # $name:snake $( _ $name_tail:snake)* >]() {}
            }

        }
    }
}

define_struct_and_impl!(loop);
define_struct_and_impl!(loop - xyz);

fn test_fn() {
    let _ = Loop::r#loop();
    let _ = Loopxyz::loop_xyz();
}
```

<br>

## Pasting documentation strings

Within the `paste!` macro, arguments to a #\[doc ...\] attribute are implicitly
concatenated together to form a coherent documentation string.

```rust
use pastey::paste;

macro_rules! method_new {
    ($ret:ident) => {
        paste! {
            #[doc = "Create a new `" $ret "` object."]
            pub fn new() -> $ret { todo!() }
        }
    };
}

pub struct Pastey {}

method_new!(Pastey);  // expands to #[doc = "Create a new `Paste` object"]
```

<br>

#### Credits

<sup>
This crate is the fork of <a href="https://github.com/dtolnay/paste"><code>paste</code></a> and I appreciate the efforts of
@dtolnay and other contributors.
</sup>

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
