#[macro_use]
extern crate derive_builder;

#[derive(Debug, PartialEq, Default, Builder, Clone)]
pub struct Lorem {
    ok: String,

    #[builder_field_attr(no_such_attribute)]
    broken: String,
}

fn main() {}
