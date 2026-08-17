#[derive(derive_more::PartialEq)]
enum Enum {
    #[partial_eq(unknown)]
    Bar { i: i32 },
}

fn main() {}
