# v1.0.4 (2020-7-31)

* `SpanRange` facility is now public.
* Docs have been improved.
* Introduced the `syn-error` feature so you can opt-out from the `syn` dependency.

# v1.0.3 (2020-6-26)

* Corrected a few typos.
* Fixed the `emit_call_site_warning` macro.

# v1.0.2 (2020-4-9)

* An obsolete note was removed from documentation.

# v1.0.1 (2020-4-9)

* `proc-macro-hack` is now well tested and supported. Not sure about `proc-macro-nested`,
  please fill a request if you need it.
* Fixed `emit_call_site_error`.
* Documentation improvements.

# v1.0.0 (2020-3-25)

I believe the API can be considered stable because it's been a few months without
breaking changes, and I also don't think this crate will receive much further evolution.
It's perfect, admit it.

Hence, meet the new, stable release!

### Improvements

* Supported nested `#[proc_macro_error]` attributes. Well, you aren't supposed to do that,
  but I caught myself doing it by accident on one occasion and the behavior was... surprising.
  Better to handle this smooth.

# v0.4.12 (2020-3-23)

* Error message on macros' misuse is now a bit more understandable.

# v0.4.11 (2020-3-02)

* `build.rs` no longer fails when `rustc` date could not be determined,
  (thanks to [`Fabian Möller`](https://gitlab.com/CreepySkeleton/proc-macro-error/issues/8)
  for noticing and to [`Igor Gnatenko`](https://gitlab.com/CreepySkeleton/proc-macro-error/-/merge_requests/25)
  for fixing).

# v0.4.10 (2020-2-29)

* `proc-macro-error` doesn't depend on syn\[full\] anymore, the compilation
  is \~30secs faster.

# v0.4.9 (2020-2-13)

* New function: `append_dummy`.

# v0.4.8 (2020-2-01)

* Support for children messages

# v0.4.7 (2020-1-31)

* Now any type that implements `quote::ToTokens` can be used instead of spans.
  This allows for high quality error messages.

# v0.4.6 (2020-1-31)

* `From<syn::Error>` implementation doesn't lose span info anymore, see
  [#6](https://gitlab.com/CreepySkeleton/proc-macro-error/issues/6).

# v0.4.5 (2020-1-20)
Just a small intermediate release.

* Fix some bugs.
* Populate license files into subfolders.

# v0.4.4 (2019-11-13)
* Fix `abort_if_dirty` + warnings bug
* Allow trailing commas in macros

# v0.4.2 (2019-11-7)
* FINALLY fixed `__pme__suggestions not found` bug

# v0.4.1 (2019-11-7) YANKED
* Fixed `__pme__suggestions not found` bug
* Documentation improvements, links checked

# v0.4.0 (2019-11-6) YANKED

## New features
* "help" messages that can have their own span on nightly, they
    inherit parent span on stable.
    ```rust
    let cond_help = if condition { Some("some help message") else { None } };
    abort!(
        span, // parent span
        "something's wrong, {} wrongs in total", 10; // main message
        help = "here's a help for you, {}", "take it"; // unconditional help message
        help =? cond_help; // conditional help message, must be Option
        note = note_span => "don't forget the note, {}", "would you?" // notes can have their own span but it's effective only on nightly
    )
    ```
* Warnings via `emit_warning` and `emit_warning_call_site`. Nightly only, they're ignored on stable.
* Now `proc-macro-error` delegates to `proc_macro::Diagnostic` on nightly.

## Breaking changes
* `MacroError` is now replaced by `Diagnostic`. Its API resembles `proc_macro::Diagnostic`.
* `Diagnostic` does not implement `From<&str/String>` so `Result<T, &str/String>::abort_or_exit()`
    won't work anymore (nobody used it anyway).
* `macro_error!` macro is replaced with `diagnostic!`.

## Improvements
* Now `proc-macro-error` renders notes exactly just like rustc does.
* We don't parse a body of a function annotated with `#[proc_macro_error]` anymore,
  only looking at the signature. This should somewhat decrease expansion time for large functions.

# v0.3.3 (2019-10-16)
* Now you can use any word instead of "help", undocumented.

# v0.3.2 (2019-10-16)
* Introduced support for "help" messages, undocumented.

# v0.3.0 (2019-10-8)

## The crate has been completely rewritten from scratch!

## Changes (most are breaking):
* Renamed macros:
  * `span_error` => `abort`
  * `call_site_error` => `abort_call_site`
* `filter_macro_errors` was replaced by `#[proc_macro_error]` attribute.
* `set_dummy` now takes `TokenStream` instead of `Option<TokenStream>`
* Support for multiple errors via `emit_error` and `emit_call_site_error`
* New `macro_error` macro for building errors in format=like style.
* `MacroError` API had been reconsidered. It also now implements `quote::ToTokens`.

# v0.2.6 (2019-09-02)
* Introduce support for dummy implementations via `dummy::set_dummy`
* `multi::*` is now deprecated, will be completely rewritten in v0.3

# v0.2.0 (2019-08-15)

## Breaking changes
* `trigger_error` replaced with `MacroError::trigger` and `filter_macro_error_panics`
  is hidden from docs.
  This is not quite a breaking change since users weren't supposed to use these functions directly anyway.
* All dependencies are updated to `v1.*`.

## New features
* Ability to stack multiple errors via `multi::MultiMacroErrors` and emit them at once.

## Improvements
* Now `MacroError` implements `std::fmt::Display` instead of `std::string::ToString`.
* `MacroError::span` inherent method.
* `From<MacroError> for proc_macro/proc_macro2::TokenStream` implementations.
* `AsRef/AsMut<String> for MacroError` implementations.

# v0.1.x (2019-07-XX)

## New features
* An easy way to report errors inside within a proc-macro via `span_error`,
  `call_site_error` and `filter_macro_errors`.
