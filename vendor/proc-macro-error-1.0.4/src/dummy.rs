//! Facility to emit dummy implementations (or whatever) in case
//! an error happen.
//!
//! `compile_error!` does not abort a compilation right away. This means
//! `rustc` doesn't just show you the error and abort, it carries on the
//! compilation process looking for other errors to report.
//!
//! Let's consider an example:
//!
//! ```rust,ignore
//! use proc_macro::TokenStream;
//! use proc_macro_error::*;
//!
//! trait MyTrait {
//!     fn do_thing();
//! }
//!
//! // this proc macro is supposed to generate MyTrait impl
//! #[proc_macro_derive(MyTrait)]
//! #[proc_macro_error]
//! fn example(input: TokenStream) -> TokenStream {
//!     // somewhere deep inside
//!     abort!(span, "something's wrong");
//!
//!     // this implementation will be generated if no error happened
//!     quote! {
//!         impl MyTrait for #name {
//!             fn do_thing() {/* whatever */}
//!         }
//!     }
//! }
//!
//! // ================
//! // in main.rs
//!
//! // this derive triggers an error
//! #[derive(MyTrait)] // first BOOM!
//! struct Foo;
//!
//! fn main() {
//!     Foo::do_thing(); // second BOOM!
//! }
//! ```
//!
//! The problem is: the generated token stream contains only `compile_error!`
//! invocation, the impl was not generated. That means user will see two compilation
//! errors:
//!
//! ```text
//! error: something's wrong
//!  --> $DIR/probe.rs:9:10
//!   |
//! 9 |#[proc_macro_derive(MyTrait)]
//!   |                    ^^^^^^^
//!
//! error[E0599]: no function or associated item named `do_thing` found for type `Foo` in the current scope
//!  --> src\main.rs:3:10
//!   |
//! 1 | struct Foo;
//!   | ----------- function or associated item `do_thing` not found for this
//! 2 | fn main() {
//! 3 |     Foo::do_thing(); // second BOOM!
//!   |          ^^^^^^^^ function or associated item not found in `Foo`
//! ```
//!
//! But the second error is meaningless! We definitely need to fix this.
//!
//! Most used approach in cases like this is "dummy implementation" -
//! omit `impl MyTrait for #name` and fill functions bodies with `unimplemented!()`.
//!
//! This is how you do it:
//!
//! ```rust,ignore
//! use proc_macro::TokenStream;
//! use proc_macro_error::*;
//!
//!  trait MyTrait {
//!      fn do_thing();
//!  }
//!
//!  // this proc macro is supposed to generate MyTrait impl
//!  #[proc_macro_derive(MyTrait)]
//!  #[proc_macro_error]
//!  fn example(input: TokenStream) -> TokenStream {
//!      // first of all - we set a dummy impl which will be appended to
//!      // `compile_error!` invocations in case a trigger does happen
//!      set_dummy(quote! {
//!          impl MyTrait for #name {
//!              fn do_thing() { unimplemented!() }
//!          }
//!      });
//!
//!      // somewhere deep inside
//!      abort!(span, "something's wrong");
//!
//!      // this implementation will be generated if no error happened
//!      quote! {
//!          impl MyTrait for #name {
//!              fn do_thing() {/* whatever */}
//!          }
//!      }
//!  }
//!
//!  // ================
//!  // in main.rs
//!
//!  // this derive triggers an error
//!  #[derive(MyTrait)] // first BOOM!
//!  struct Foo;
//!
//!  fn main() {
//!      Foo::do_thing(); // no more errors!
//!  }
//! ```

use proc_macro2::TokenStream;
use std::cell::RefCell;

use crate::check_correctness;

thread_local! {
    static DUMMY_IMPL: RefCell<Option<TokenStream>> = RefCell::new(None);
}

/// Sets dummy token stream which will be appended to `compile_error!(msg);...`
/// invocations in case you'll emit any errors.
///
/// See [guide](../index.html#guide).
pub fn set_dummy(dummy: TokenStream) -> Option<TokenStream> {
    check_correctness();
    DUMMY_IMPL.with(|old_dummy| old_dummy.replace(Some(dummy)))
}

/// Same as [`set_dummy`] but, instead of resetting, appends tokens to the
/// existing dummy (if any). Behaves as `set_dummy` if no dummy is present.
pub fn append_dummy(dummy: TokenStream) {
    check_correctness();
    DUMMY_IMPL.with(|old_dummy| {
        let mut cell = old_dummy.borrow_mut();
        if let Some(ts) = cell.as_mut() {
            ts.extend(dummy);
        } else {
            *cell = Some(dummy);
        }
    });
}

pub(crate) fn cleanup() -> Option<TokenStream> {
    DUMMY_IMPL.with(|old_dummy| old_dummy.replace(None))
}
