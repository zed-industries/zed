#[macro_use]
extern crate derive_builder;

#[derive(Debug, PartialEq, Default, Builder, Clone)]
pub struct Lorem {
    // `default` is incompatible with `field.build`
    #[builder(
        default = "1",
        field(build = "self.ipsum.map(|v| v + 42).unwrap_or(100)")
    )]
    ipsum: usize,

    // `default` is incompatible with `field.type`, even without `field.build`
    #[builder(default = "2", field(type = "usize"))]
    sit: usize,

    // Both errors can occur on the same field
    #[builder(default = "3", field(type = "usize", build = "self.ipsum + 42"))]
    amet: usize,
}

fn main() {}
