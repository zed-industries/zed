//! [![github]](https://github.com/dtolnay/inherent)&ensp;[![crates-io]](https://crates.io/crates/inherent)&ensp;[![docs-rs]](https://docs.rs/inherent)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! ##### An attribute macro to make trait methods callable without the trait in scope.
//!
//! # Example
//!
//! ```rust
//! mod types {
//!     use inherent::inherent;
//!
//!     trait Trait {
//!         fn f(self);
//!     }
//!
//!     pub struct Struct;
//!
//!     #[inherent]
//!     impl Trait for Struct {
//!         pub fn f(self) {}
//!     }
//! }
//!
//! fn main() {
//!     // types::Trait is not in scope, but method can be called.
//!     types::Struct.f();
//! }
//! ```
//!
//! Without the `inherent` macro on the trait impl, this would have failed with the
//! following error:
//!
//! ```console
//! error[E0599]: no method named `f` found for type `types::Struct` in the current scope
//!   --> src/main.rs:18:19
//!    |
//! 8  |     pub struct Struct;
//!    |     ------------------ method `f` not found for this
//! ...
//! 18 |     types::Struct.f();
//!    |                   ^
//!    |
//!    = help: items from traits can only be used if the trait is implemented and in scope
//!    = note: the following trait defines an item `f`, perhaps you need to implement it:
//!            candidate #1: `types::Trait`
//! ```
//!
//! The `inherent` macro expands to inherent methods on the `Self` type of the trait
//! impl that forward to the trait methods. In the case above, the generated code
//! would be:
//!
//! ```rust
//! # trait Trait {
//! #     fn f(self);
//! # }
//! #
//! # pub struct Struct;
//! #
//! # impl Trait for Struct {
//! #     fn f(self) {}
//! # }
//! #
//! impl Struct {
//!     pub fn f(self) {
//!         <Self as Trait>::f(self)
//!     }
//! }
//! ```

#![doc(html_root_url = "https://docs.rs/inherent/1.0.13")]
#![allow(
    clippy::default_trait_access,
    clippy::module_name_repetitions,
    clippy::needless_doctest_main,
    clippy::needless_pass_by_value,
    clippy::uninlined_format_args
)]

extern crate proc_macro;

mod expand;
mod parse;
mod verbatim;

use proc_macro::TokenStream;
use syn::parse::Nothing;
use syn::parse_macro_input;

use crate::parse::TraitImpl;

#[proc_macro_attribute]
pub fn inherent(args: TokenStream, input: TokenStream) -> TokenStream {
    parse_macro_input!(args as Nothing);
    let input = parse_macro_input!(input as TraitImpl);
    expand::inherent(input).into()
}
