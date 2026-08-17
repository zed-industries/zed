#![cfg_attr(nightly_diagnostics, feature(proc_macro_diagnostic, proc_macro_span))]

//! Diagnostic emulation on stable and nightly.
//!
//! # Usage
//!
//! 1. Depend on the library in your proc-macro.
//!
//! ```toml
//! [dependencies]
//! proc_macro2_diagnostics = "0.10"
//! ```
//!
//! 2. Import [`SpanDiagnosticExt`] and use its methods on a
//!    [`proc_macro2::Span`] to create [`Diagnostic`]s:
//!
//! ```rust
//! use syn::spanned::Spanned;
//! use proc_macro2::TokenStream;
//! use proc_macro2_diagnostics::{SpanDiagnosticExt, Diagnostic};
//!
//! fn my_macro(input: TokenStream) -> Result<TokenStream, Diagnostic> {
//!     Err(input.span().error("there's a problem here..."))
//! }
//! ```
//!
//! 3. If there's an error, emit the diagnostic as tokens:
//!
//! ```rust
//! extern crate proc_macro;
//!
//! # use proc_macro2::TokenStream;
//! # use proc_macro2_diagnostics::{SpanDiagnosticExt, Diagnostic};
//! # use syn::spanned::Spanned;
//! # fn my_macro(input: TokenStream) -> Result<TokenStream, Diagnostic> {
//! #     Err(input.span().error("there's a problem here..."))
//! # }
//! # /*
//! #[proc_macro]
//! # */
//! pub fn real_macro(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
//!     match my_macro(tokens.into()) {
//!         Ok(tokens) => tokens.into(),
//!         Err(diag) => diag.emit_as_expr_tokens().into()
//!     }
//! }
//! ```
//!
//! This does the right thing on nightly _or_ stable.
//!
//! # Caveats
//!
//! On stable, due to limitations, any top-level, non-error diagnostics are
//! emitted as errors. This will abort compilation. To avoid this, you may want
//! to `cfg`-gate emitting non-error diagnostics to nightly.
//!
//! # Colors
//!
//! By default, error messages are colored on stable. To disable, disable
//! default features:
//!
//! ```toml
//! [dependencies]
//! proc_macro2_diagnostics = { version = "0.10", default-features = false }
//! ```
//!
//! The compiler always colors diagnostics on nightly.

extern crate proc_macro;

mod ext;
mod diagnostic;
mod line;

pub use diagnostic::{Diagnostic, Level};
pub use ext::SpanDiagnosticExt;

// We stole this from proc_macro2. Checks whether nightly proc_macro things
// _actually_ work by checking if calls to proc_macro::Span panic.
#[cfg(nightly_diagnostics)]
fn nightly_works() -> bool {
    use std::panic::{self, PanicInfo};
    use std::sync::atomic::*;
    use std::sync::Once;

    static WORKS: AtomicUsize = AtomicUsize::new(0);
    static INIT: Once = Once::new();

    match WORKS.load(Ordering::SeqCst) {
        1 => return false,
        2 => return true,
        _ => {}
    }

    // Swap in a null panic hook to avoid printing "thread panicked" to stderr,
    // then use catch_unwind to determine whether the compiler's proc_macro is
    // working. When proc-macro2 is used from outside of a procedural macro all
    // of the proc_macro crate's APIs currently panic.
    //
    // The Once is to prevent the possibility of this ordering:
    //
    //     thread 1 calls take_hook, gets the user's original hook
    //     thread 1 calls set_hook with the null hook
    //     thread 2 calls take_hook, thinks null hook is the original hook
    //     thread 2 calls set_hook with the null hook
    //     thread 1 calls set_hook with the actual original hook
    //     thread 2 calls set_hook with what it thinks is the original hook
    //
    // in which the user's hook has been lost.
    //
    // There is still a race condition where a panic in a different thread can
    // happen during the interval that the user's original panic hook is
    // unregistered such that their hook is incorrectly not called. This is
    // sufficiently unlikely and less bad than printing panic messages to stderr
    // on correct use of this crate. Maybe there is a libstd feature request
    // here. For now, if a user needs to guarantee that this failure mode does
    // not occur, they need to call e.g. `proc_macro2::Span::call_site()` from
    // the main thread before launching any other threads.
    INIT.call_once(|| {
        type PanicHook = dyn Fn(&PanicInfo) + Sync + Send + 'static;

        let null_hook: Box<PanicHook> = Box::new(|_panic_info| { /* ignore */ });
        let sanity_check = &*null_hook as *const PanicHook;
        let original_hook = panic::take_hook();
        panic::set_hook(null_hook);

        let works = panic::catch_unwind(|| proc_macro::Span::call_site()).is_ok();
        WORKS.store(works as usize + 1, Ordering::SeqCst);

        let hopefully_null_hook = panic::take_hook();
        panic::set_hook(original_hook);
        if sanity_check != &*hopefully_null_hook {
            panic!("observed race condition in proc_macro2::nightly_works");
        }
    });

    nightly_works()
}
