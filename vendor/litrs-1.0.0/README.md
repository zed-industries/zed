# `litrs`: parsing and inspecting Rust literals

[<img alt="CI status of main" src="https://img.shields.io/github/actions/workflow/status/LukasKalbertodt/litrs/ci.yml?branch=main&label=CI&logo=github&logoColor=white&style=for-the-badge" height="23">](https://github.com/LukasKalbertodt/litrs/actions/workflows/ci.yml)
[<img alt="Crates.io Version" src="https://img.shields.io/crates/v/litrs?logo=rust&style=for-the-badge" height="23">](https://crates.io/crates/litrs)
[<img alt="docs.rs" src="https://img.shields.io/crates/v/litrs?color=blue&label=docs&style=for-the-badge" height="23">](https://docs.rs/litrs)

`litrs` offers functionality to parse Rust literals, i.e. tokens in the Rust programming language that represent fixed values.
For example: `27`, `"crab"`, `bool`.
This is particularly useful for proc macros, but can also be used outside of a proc-macro context.

**Why this library?**
Unfortunately, the `proc_macro` API shipped with the compiler offers no easy way to inspect literals.
There are mainly two libraries for this purpose:
[`syn`](https://github.com/dtolnay/syn) and [`literalext`](https://github.com/mystor/literalext).
The latter is deprecated.
And `syn` is oftentimes overkill for the task at hand, especially when developing function-like proc-macros (e.g. `foo!(..)`).
This crate is a lightweight alternative.
Also, when it comes to literals, `litrs` offers a bit more flexibility and a few more features compared to `syn`.

## Example

### In proc macro

```rust
use std::convert::TryFrom;
use proc_macro::TokenStream;
use litrs::Literal;

#[proc_macro]
pub fn foo(input: TokenStream) -> TokenStream {
    // Please do proper error handling in your real code!
    let first_token = input.into_iter().next().expect("no input");

    // `try_from` will return an error if the token is not a literal.
    match Literal::try_from(first_token) {
        // Convenient methods to produce decent errors via `compile_error!`.
        Err(e) => return e.to_compile_error(),

        // You can now inspect your literal!
        Ok(Literal::Integer(i)) => {
            println!("Got an integer specified in base {:?}", i.base());

            let value = i.value::<u64>().expect("integer literal too large");
            println!("Is your integer even? {}", value % 2 == 0);
        }
        Ok(other) => {
            println!("Got a non-integer literal");
        }
    }

    TokenStream::new() // dummy output
}
```

If you are expecting a specific kind of literal, you can also use this, which will return an error if the token is not a float literal.

```rust
FloatLit::try_from(first_token)
```

### Parsing from a `&str`

Outside of a proc macro context you might want to parse a string directly.

```rust
use litrs::{FloatLit, Literal};

let lit = Literal::parse("'🦀'").expect("failed to parse literal");
let float_lit = FloatLit::parse("2.7e3").expect("failed to parse as float literal");
```

See [**the documentation**](https://docs.rs/litrs) or the `examples/` directory for more examples and information.


<br />

---

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
