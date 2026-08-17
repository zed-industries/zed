#[derive(derive_more::PartialEq)]
struct Foo(#[partial_eq(unknown)] i32);

#[derive(derive_more::PartialEq)]
enum Enum {
    Bar { #[partial_eq(unknown)] i: i32 },
}

fn main() {}
