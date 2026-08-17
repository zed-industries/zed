use bon::{bon, builder, Builder};

#[builder(const)]
fn const_on_non_const_fn() {}

struct Sut;

#[bon]
impl Sut {
    #[builder(const)]
    fn const_on_non_const_fn() {}
}

#[derive(Builder)]
#[builder(const)]
struct IncompatibleDefault {
    #[builder(default)]
    x1: u32,
}

#[derive(Builder)]
#[builder(const)]
struct IncompatibleSkip {
    #[builder(skip)]
    x1: u32,
}

#[derive(Builder)]
#[builder(const)]
struct IncompatibleField {
    #[builder(field)]
    x1: u32,
}

#[derive(Builder)]
#[builder(const)]
struct IncompatibleInto {
    #[builder(into)]
    x1: u32,
}

#[derive(Builder)]
#[builder(const)]
struct IncompatibleFromIter1 {
    #[builder(with = FromIterator::from_iter)]
    x1: Vec<u32>,
}

#[derive(Builder)]
#[builder(const)]
struct IncompatibleFromIter2 {
    #[builder(with = <_>::from_iter)]
    x1: Vec<u32>,
}

#[derive(Builder)]
#[builder(const)]
struct IncompatibleDefaultExpression {
    #[builder(default = return 1)]
    x1: u32,
}

fn main() {}
