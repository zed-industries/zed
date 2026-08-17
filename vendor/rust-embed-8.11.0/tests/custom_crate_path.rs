/// This test checks that the `crate_path` attribute can be used
/// to specify a custom path to the `rust_embed` crate.

mod custom {
  pub mod path {
    pub use rust_embed;
  }
}

// We introduce a 'rust_embed' module here to break compilation in case
// the `rust_embed` crate is not loaded correctly.
//
// To test this, try commenting out the attribute which specifies the
// the custom crate path -- you should find that the test fails to compile.
mod rust_embed {}

#[derive(custom::path::rust_embed::RustEmbed)]
#[crate_path = "custom::path::rust_embed"]
#[folder = "examples/public/"]
struct Asset;
