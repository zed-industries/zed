//! # proc-macro-error
//!
//! This crate aims to make error reporting in proc-macros simple and easy to use.
//! Migrate from `panic!`-based errors for as little effort as possible!
//!
//! (Also, you can explicitly [append a dummy token stream](dummy/index.html) to your errors).
//!
//! To achieve his, this crate serves as a tiny shim around `proc_macro::Diagnostic` and
//! `compile_error!`. It detects the best way of emitting available based on compiler's version.
//! When the underlying diagnostic type is finally stabilized, this crate will simply be
//! delegating to it requiring no changes in your code!
//!
//! So you can just use this crate and have *both* some of `proc_macro::Diagnostic` functionality
//! available on stable ahead of time *and* your error-reporting code future-proof.
//!
//! ## Cargo features
//!
//! This crate provides *enabled by default* `syn-error` feature that gates
//! `impl From<syn::Error> for Diagnostic` conversion. If you don't use `syn` and want
//! to cut off some of compilation time, you can disable it via
//!
//! ```toml
//! [dependencies]
//! proc-macro-error = { version = "1", default-features = false }
//! ```
//!
//! ***Please note that disabling this feature makes sense only if you don't depend on `syn`
//! directly or indirectly, and you very likely do.**
//!
//! ## Real world examples
//!
//! * [`structopt-derive`](https://github.com/TeXitoi/structopt/tree/master/structopt-derive)
//!   (abort-like usage)
//! * [`auto-impl`](https://github.com/auto-impl-rs/auto_impl/) (emit-like usage)
//!
//! ## Limitations
//!
//! - Warnings are emitted only on nightly, they are ignored on stable.
//! - "help" suggestions can't have their own span info on stable,
//!   (essentially inheriting the parent span).
//! - If a panic occurs somewhere in your macro no errors will be displayed. This is not a
//!   technical limitation but rather intentional design. `panic` is not for error reporting.
//!
//! ### `#[proc_macro_error]` attribute
//!
//! **This attribute MUST be present on the top level of your macro** (the function
//! annotated with any of `#[proc_macro]`, `#[proc_macro_derive]`, `#[proc_macro_attribute]`).
//!
//! This attribute performs the setup and cleanup necessary to make things work.
//!
//! In most cases you'll need the simple `#[proc_macro_error]` form without any
//! additional settings. Feel free to [skip the "Syntax" section](#macros).
//!
//! #### Syntax
//!
//! `#[proc_macro_error]` or `#[proc_macro_error(settings...)]`, where `settings...`
//! is a comma-separated list of:
//!
//! - `proc_macro_hack`:
//!
//!     In order to correctly cooperate with `#[proc_macro_hack]`, `#[proc_macro_error]`
//!     attribute must be placed *before* (above) it, like this:
//!
//!     ```no_run
//!     # use proc_macro2::TokenStream;
//!     # const IGNORE: &str = "
//!     #[proc_macro_error]
//!     #[proc_macro_hack]
//!     #[proc_macro]
//!     # ";
//!     fn my_macro(input: TokenStream) -> TokenStream {
//!         unimplemented!()
//!     }
//!     ```
//!
//!     If, for some reason, you can't place it like that you can use
//!     `#[proc_macro_error(proc_macro_hack)]` instead.
//!
//!     # Note
//!
//!     If `proc-macro-hack` was detected (by any means) `allow_not_macro`
//!     and `assert_unwind_safe` will be applied automatically.
//!
//! - `allow_not_macro`:
//!
//!     By default, the attribute checks that it's applied to a proc-macro.
//!     If none of `#[proc_macro]`, `#[proc_macro_derive]` nor `#[proc_macro_attribute]` are
//!     present it will panic. It's the intention - this crate is supposed to be used only with
//!     proc-macros.
//!
//!     This setting is made to bypass the check, useful in certain circumstances.
//!
//!     Pay attention: the function this attribute is applied to must return
//!     `proc_macro::TokenStream`.
//!
//!     This setting is implied if `proc-macro-hack` was detected.
//!
//! - `assert_unwind_safe`:
//!
//!     By default, your code must be [unwind safe]. If your code is not unwind safe,
//!     but you believe it's correct, you can use this setting to bypass the check.
//!     You would need this for code that uses `lazy_static` or `thread_local` with
//!     `Cell/RefCell` inside (and the like).
//!
//!     This setting is implied if `#[proc_macro_error]` is applied to a function
//!     marked as `#[proc_macro]`, `#[proc_macro_derive]` or `#[proc_macro_attribute]`.
//!
//!     This setting is also implied if `proc-macro-hack` was detected.
//!
//! ## Macros
//!
//! Most of the time you want to use the macros. Syntax is described in the next section below.
//!
//! You'll need to decide how you want to emit errors:
//!
//! * Emit the error and abort. Very much panic-like usage. Served by [`abort!`] and
//!   [`abort_call_site!`].
//! * Emit the error but do not abort right away, looking for other errors to report.
//!   Served by [`emit_error!`] and [`emit_call_site_error!`].
//!
//! You **can** mix these usages.
//!
//! `abort` and `emit_error` take a "source span" as the first argument. This source
//! will be used to highlight the place the error originates from. It must be one of:
//!
//! * *Something* that implements [`ToTokens`] (most types in `syn` and `proc-macro2` do).
//!   This source is the preferable one since it doesn't lose span information on multi-token
//!   spans, see [this issue](https://gitlab.com/CreepySkeleton/proc-macro-error/-/issues/6)
//!   for details.
//! * [`proc_macro::Span`]
//! * [`proc-macro2::Span`]
//!
//! The rest is your message in format-like style.
//!
//! See [the next section](#syntax-1) for detailed syntax.
//!
//! - [`abort!`]:
//!
//!     Very much panic-like usage - abort right away and show the error.
//!     Expands to [`!`] (never type).
//!
//! - [`abort_call_site!`]:
//!
//!     Shortcut for `abort!(Span::call_site(), ...)`. Expands to [`!`] (never type).
//!
//! - [`emit_error!`]:
//!
//!     [`proc_macro::Diagnostic`]-like usage - emit the error but keep going,
//!     looking for other errors to report.
//!     The compilation will fail nonetheless. Expands to [`()`] (unit type).
//!
//! - [`emit_call_site_error!`]:
//!
//!     Shortcut for `emit_error!(Span::call_site(), ...)`. Expands to [`()`] (unit type).
//!
//! - [`emit_warning!`]:
//!
//!     Like `emit_error!` but emit a warning instead of error. The compilation won't fail
//!     because of warnings.
//!     Expands to [`()`] (unit type).
//!
//!     **Beware**: warnings are nightly only, they are completely ignored on stable.
//!
//! - [`emit_call_site_warning!`]:
//!
//!     Shortcut for `emit_warning!(Span::call_site(), ...)`. Expands to [`()`] (unit type).
//!
//! - [`diagnostic`]:
//!
//!     Build an instance of `Diagnostic` in format-like style.
//!
//! #### Syntax
//!
//! All the macros have pretty much the same syntax:
//!
//! 1.  ```ignore
//!     abort!(single_expr)
//!     ```
//!     Shortcut for `Diagnostic::from(expr).abort()`.
//!
//! 2.  ```ignore
//!     abort!(span, message)
//!     ```
//!     The first argument is an expression the span info should be taken from.
//!
//!     The second argument is the error message, it must implement [`ToString`].
//!
//! 3.  ```ignore
//!     abort!(span, format_literal, format_args...)
//!     ```
//!
//!     This form is pretty much the same as 2, except `format!(format_literal, format_args...)`
//!     will be used to for the message instead of [`ToString`].
//!
//! That's it. `abort!`, `emit_warning`, `emit_error` share this exact syntax.
//!
//! `abort_call_site!`, `emit_call_site_warning`, `emit_call_site_error` lack 1 form
//! and do not take span in 2'th and 3'th forms. Those are essentially shortcuts for
//! `macro!(Span::call_site(), args...)`.
//!
//! `diagnostic!` requires a [`Level`] instance between `span` and second argument
//! (1'th form is the same).
//!
//! > **Important!**
//! >
//! > If you have some type from `proc_macro` or `syn` to point to, do not call `.span()`
//! > on it but rather use it directly:
//! > ```no_run
//! > # use proc_macro_error::abort;
//! > # let input = proc_macro2::TokenStream::new();
//! > let ty: syn::Type = syn::parse2(input).unwrap();
//! > abort!(ty, "BOOM");
//! > //     ^^ <-- avoid .span()
//! > ```
//! >
//! > `.span()` calls work too, but you may experience regressions in message quality.
//!
//! #### Note attachments
//!
//! 3.  Every macro can have "note" attachments (only 2 and 3 form).
//!   ```ignore
//!   let opt_help = if have_some_info { Some("did you mean `this`?") } else { None };
//!
//!   abort!(
//!       span, message; // <--- attachments start with `;` (semicolon)
//!
//!       help = "format {} {}", "arg1", "arg2"; // <--- every attachment ends with `;`,
//!                                              //      maybe except the last one
//!
//!       note = "to_string"; // <--- one arg uses `.to_string()` instead of `format!()`
//!
//!       yay = "I see what {} did here", "you"; // <--- "help =" and "hint =" are mapped
//!                                              // to Diagnostic::help,
//!                                              // anything else is Diagnostic::note
//!
//!       wow = note_span => "custom span"; // <--- attachments can have their own span
//!                                         //      it takes effect only on nightly though
//!
//!       hint =? opt_help; // <-- "optional" attachment, get displayed only if `Some`
//!                         //     must be single `Option` expression
//!
//!       note =? note_span => opt_help // <-- optional attachments can have custom spans too
//!   );
//!   ```
//!

//! ### Diagnostic type
//!
//! [`Diagnostic`] type is intentionally designed to be API compatible with [`proc_macro::Diagnostic`].
//! Not all API is implemented, only the part that can be reasonably implemented on stable.
//!
//!
//! [`abort!`]: macro.abort.html
//! [`abort_call_site!`]: macro.abort_call_site.html
//! [`emit_warning!`]: macro.emit_warning.html
//! [`emit_error!`]: macro.emit_error.html
//! [`emit_call_site_warning!`]: macro.emit_call_site_error.html
//! [`emit_call_site_error!`]: macro.emit_call_site_warning.html
//! [`diagnostic!`]: macro.diagnostic.html
//! [`Diagnostic`]: struct.Diagnostic.html
//!
//! [`proc_macro::Span`]: https://doc.rust-lang.org/proc_macro/struct.Span.html
//! [`proc_macro::Diagnostic`]: https://doc.rust-lang.org/proc_macro/struct.Diagnostic.html
//!
//! [unwind safe]: https://doc.rust-lang.org/std/panic/trait.UnwindSafe.html#what-is-unwind-safety
//! [`!`]: https://doc.rust-lang.org/std/primitive.never.html
//! [`()`]: https://doc.rust-lang.org/std/primitive.unit.html
//! [`ToString`]: https://doc.rust-lang.org/std/string/trait.ToString.html
//!
//! [`proc-macro2::Span`]: https://docs.rs/proc-macro2/1.0.10/proc_macro2/struct.Span.html
//! [`ToTokens`]: https://docs.rs/quote/1.0.3/quote/trait.ToTokens.html
//!

#![cfg_attr(not(use_fallback), feature(proc_macro_diagnostic))]
#![forbid(unsafe_code)]
#![allow(clippy::needless_doctest_main)]

extern crate proc_macro;

pub use crate::{
    diagnostic::{Diagnostic, DiagnosticExt, Level},
    dummy::{append_dummy, set_dummy},
};
pub use proc_macro_error_attr::proc_macro_error;

use proc_macro2::Span;
use quote::{quote, ToTokens};

use std::cell::Cell;
use std::panic::{catch_unwind, resume_unwind, UnwindSafe};

pub mod dummy;

mod diagnostic;
mod macros;
mod sealed;

#[cfg(use_fallback)]
#[path = "imp/fallback.rs"]
mod imp;

#[cfg(not(use_fallback))]
#[path = "imp/delegate.rs"]
mod imp;

#[derive(Debug, Clone, Copy)]
pub struct SpanRange {
    pub first: Span,
    pub last: Span,
}

impl SpanRange {
    /// Create a range with the `first` and `last` spans being the same.
    pub fn single_span(span: Span) -> Self {
        SpanRange {
            first: span,
            last: span,
        }
    }

    /// Create a `SpanRange` resolving at call site.
    pub fn call_site() -> Self {
        SpanRange::single_span(Span::call_site())
    }

    /// Construct span range from a `TokenStream`. This method always preserves all the
    /// range.
    ///
    /// ### Note
    ///
    /// If the stream is empty, the result is `SpanRange::call_site()`. If the stream
    /// consists of only one `TokenTree`, the result is `SpanRange::single_span(tt.span())`
    /// that doesn't lose anything.
    pub fn from_tokens(ts: &dyn ToTokens) -> Self {
        let mut spans = ts.to_token_stream().into_iter().map(|tt| tt.span());
        let first = spans.next().unwrap_or_else(|| Span::call_site());
        let last = spans.last().unwrap_or(first);

        SpanRange { first, last }
    }

    /// Join two span ranges. The resulting range will start at `self.first` and end at
    /// `other.last`.
    pub fn join_range(self, other: SpanRange) -> Self {
        SpanRange {
            first: self.first,
            last: other.last,
        }
    }

    /// Collapse the range into single span, preserving as much information as possible.
    pub fn collapse(self) -> Span {
        self.first.join(self.last).unwrap_or(self.first)
    }
}

/// This traits expands `Result<T, Into<Diagnostic>>` with some handy shortcuts.
pub trait ResultExt {
    type Ok;

    /// Behaves like `Result::unwrap`: if self is `Ok` yield the contained value,
    /// otherwise abort macro execution via `abort!`.
    fn unwrap_or_abort(self) -> Self::Ok;

    /// Behaves like `Result::expect`: if self is `Ok` yield the contained value,
    /// otherwise abort macro execution via `abort!`.
    /// If it aborts then resulting error message will be preceded with `message`.
    fn expect_or_abort(self, msg: &str) -> Self::Ok;
}

/// This traits expands `Option` with some handy shortcuts.
pub trait OptionExt {
    type Some;

    /// Behaves like `Option::expect`: if self is `Some` yield the contained value,
    /// otherwise abort macro execution via `abort_call_site!`.
    /// If it aborts the `message` will be used for [`compile_error!`][compl_err] invocation.
    ///
    /// [compl_err]: https://doc.rust-lang.org/std/macro.compile_error.html
    fn expect_or_abort(self, msg: &str) -> Self::Some;
}

/// Abort macro execution and display all the emitted errors, if any.
///
/// Does nothing if no errors were emitted (warnings do not count).
pub fn abort_if_dirty() {
    imp::abort_if_dirty();
}

impl<T, E: Into<Diagnostic>> ResultExt for Result<T, E> {
    type Ok = T;

    fn unwrap_or_abort(self) -> T {
        match self {
            Ok(res) => res,
            Err(e) => e.into().abort(),
        }
    }

    fn expect_or_abort(self, message: &str) -> T {
        match self {
            Ok(res) => res,
            Err(e) => {
                let mut e = e.into();
                e.msg = format!("{}: {}", message, e.msg);
                e.abort()
            }
        }
    }
}

impl<T> OptionExt for Option<T> {
    type Some = T;

    fn expect_or_abort(self, message: &str) -> T {
        match self {
            Some(res) => res,
            None => abort_call_site!(message),
        }
    }
}

/// This is the entry point for a proc-macro.
///
/// **NOT PUBLIC API, SUBJECT TO CHANGE WITHOUT ANY NOTICE**
#[doc(hidden)]
pub fn entry_point<F>(f: F, proc_macro_hack: bool) -> proc_macro::TokenStream
where
    F: FnOnce() -> proc_macro::TokenStream + UnwindSafe,
{
    ENTERED_ENTRY_POINT.with(|flag| flag.set(flag.get() + 1));
    let caught = catch_unwind(f);
    let dummy = dummy::cleanup();
    let err_storage = imp::cleanup();
    ENTERED_ENTRY_POINT.with(|flag| flag.set(flag.get() - 1));

    let gen_error = || {
        if proc_macro_hack {
            quote! {{
                macro_rules! proc_macro_call {
                    () => ( unimplemented!() )
                }

                #(#err_storage)*
                #dummy

                unimplemented!()
            }}
        } else {
            quote!( #(#err_storage)* #dummy )
        }
    };

    match caught {
        Ok(ts) => {
            if err_storage.is_empty() {
                ts
            } else {
                gen_error().into()
            }
        }

        Err(boxed) => match boxed.downcast::<AbortNow>() {
            Ok(_) => gen_error().into(),
            Err(boxed) => resume_unwind(boxed),
        },
    }
}

fn abort_now() -> ! {
    check_correctness();
    panic!(AbortNow)
}

thread_local! {
    static ENTERED_ENTRY_POINT: Cell<usize> = Cell::new(0);
}

struct AbortNow;

fn check_correctness() {
    if ENTERED_ENTRY_POINT.with(|flag| flag.get()) == 0 {
        panic!(
            "proc-macro-error API cannot be used outside of `entry_point` invocation, \
             perhaps you forgot to annotate your #[proc_macro] function with `#[proc_macro_error]"
        );
    }
}

/// **ALL THE STUFF INSIDE IS NOT PUBLIC API!!!**
#[doc(hidden)]
pub mod __export {
    // reexports for use in macros
    pub extern crate proc_macro;
    pub extern crate proc_macro2;

    use proc_macro2::Span;
    use quote::ToTokens;

    use crate::SpanRange;

    // inspired by
    // https://github.com/dtolnay/case-studies/blob/master/autoref-specialization/README.md#simple-application

    pub trait SpanAsSpanRange {
        #[allow(non_snake_case)]
        fn FIRST_ARG_MUST_EITHER_BE_Span_OR_IMPLEMENT_ToTokens_OR_BE_SpanRange(&self) -> SpanRange;
    }

    pub trait Span2AsSpanRange {
        #[allow(non_snake_case)]
        fn FIRST_ARG_MUST_EITHER_BE_Span_OR_IMPLEMENT_ToTokens_OR_BE_SpanRange(&self) -> SpanRange;
    }

    pub trait ToTokensAsSpanRange {
        #[allow(non_snake_case)]
        fn FIRST_ARG_MUST_EITHER_BE_Span_OR_IMPLEMENT_ToTokens_OR_BE_SpanRange(&self) -> SpanRange;
    }

    pub trait SpanRangeAsSpanRange {
        #[allow(non_snake_case)]
        fn FIRST_ARG_MUST_EITHER_BE_Span_OR_IMPLEMENT_ToTokens_OR_BE_SpanRange(&self) -> SpanRange;
    }

    impl<T: ToTokens> ToTokensAsSpanRange for &T {
        fn FIRST_ARG_MUST_EITHER_BE_Span_OR_IMPLEMENT_ToTokens_OR_BE_SpanRange(&self) -> SpanRange {
            let mut ts = self.to_token_stream().into_iter();
            let first = ts
                .next()
                .map(|tt| tt.span())
                .unwrap_or_else(Span::call_site);
            let last = ts.last().map(|tt| tt.span()).unwrap_or(first);
            SpanRange { first, last }
        }
    }

    impl Span2AsSpanRange for Span {
        fn FIRST_ARG_MUST_EITHER_BE_Span_OR_IMPLEMENT_ToTokens_OR_BE_SpanRange(&self) -> SpanRange {
            SpanRange {
                first: *self,
                last: *self,
            }
        }
    }

    impl SpanAsSpanRange for proc_macro::Span {
        fn FIRST_ARG_MUST_EITHER_BE_Span_OR_IMPLEMENT_ToTokens_OR_BE_SpanRange(&self) -> SpanRange {
            SpanRange {
                first: self.clone().into(),
                last: self.clone().into(),
            }
        }
    }

    impl SpanRangeAsSpanRange for SpanRange {
        fn FIRST_ARG_MUST_EITHER_BE_Span_OR_IMPLEMENT_ToTokens_OR_BE_SpanRange(&self) -> SpanRange {
            *self
        }
    }
}
