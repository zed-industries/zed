//! Wrapper environment for generated code from optimization rules in ISLE.

// See https://github.com/rust-lang/rust/issues/47995: we cannot use `#![...]` attributes inside of
// the generated ISLE source below because we include!() it. We must include!() it because its path
// depends on an environment variable; and also because of this, we can't do the `#[path = "..."]
// mod generated_code;` trick either.
#![expect(
    dead_code,
    unreachable_patterns,
    unused_imports,
    unused_variables,
    irrefutable_let_patterns,
    non_camel_case_types,
    clippy::clone_on_copy,
    reason = "generated code"
)]

include!(concat!(env!("ISLE_DIR"), "/isle_opt.rs"));
