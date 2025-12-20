//! Component Preview Example
//!
//! Run with: `cargo run -p component_preview --example component_preview --features="preview"`
//!
//! To use this in other projects, add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! component_preview = { path = "../component_preview", features = ["preview"] }
//!
//! [[example]]
//! name = "component_preview"
//! path = "examples/component_preview.rs"
//! ```

fn main() {
    component_preview::run_component_preview();
}
