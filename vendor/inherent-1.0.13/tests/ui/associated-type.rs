use inherent::inherent;

pub trait Trait {
    type Assoc;
    fn f() -> Self::Assoc;
}

pub struct Struct;

#[inherent]
impl Trait for Struct {
    type Assoc = ();

    // TODO: https://github.com/dtolnay/inherent/issues/15
    fn f() -> Self::Assoc {}
}

fn main() {}
