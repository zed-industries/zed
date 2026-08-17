#![deny(single_use_lifetimes)]

use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Enum<'id> {
    #[snafu(display("`{to}` is foo, yo"))]
    Foo { to: &'id u32 },
    #[snafu(display("bar `{from}` frobnicated `{to}`"))]
    Bar { from: &'id String, to: &'id i8 },
}

#[derive(Debug, Snafu)]
pub struct Struct<'id>(Enum<'id>);

#[test]
fn it_compiles() {}
