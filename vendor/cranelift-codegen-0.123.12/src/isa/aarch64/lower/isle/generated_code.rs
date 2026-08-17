// See https://github.com/rust-lang/rust/issues/47995: we cannot use `#![...]` attributes inside of
// the generated ISLE source below because we include!() it. We must include!() it because its path
// depends on an environment variable; and also because of this, we can't do the `#[path = "..."]
// mod generated_code;` trick either.
#![expect(
    missing_docs,
    dead_code,
    unreachable_patterns,
    unused_imports,
    unused_variables,
    irrefutable_let_patterns,
    clippy::clone_on_copy,
    reason = "generated code"
)]

include!(concat!(env!("ISLE_DIR"), "/isle_aarch64.rs"));
