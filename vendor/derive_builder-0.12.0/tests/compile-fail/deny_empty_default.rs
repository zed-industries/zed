#[macro_use]
extern crate derive_builder;

// deny `#[builder(default = "")]`, because we don't want to define a meaning (yet)! :-)
#[allow(dead_code)]
#[derive(Builder)]
struct Lorem {
    #[builder(default = "")]
    ipsum: String,
}

fn main() {}
