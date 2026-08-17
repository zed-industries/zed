#[derive(derive_more::TryFrom)]
#[try_from(repr)]
#[repr(a + b)]
enum Enum {
    Variant
}

fn main() {}
