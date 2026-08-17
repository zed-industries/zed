#[macro_use]
extern crate derive_builder;

// This is a stand-in for any bad crate directive
mod empty {}

#[derive(Builder)]
// It would be nice if the "failed to resolve" errors would identify `"empty"` as the error span,
// but doing so would require rewriting a lot of the code generation to use the crate_root span
// for the full path of the thing being imported, and that doesn't seem worth the code churn.
#[builder(crate = "empty")]
struct BadCrate {
    lorem: String,
}

fn main() {}
