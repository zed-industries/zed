use bon::{bon, builder, Builder};

#[derive(Builder)]
#[builder(crate = self::bon)]
struct Relative1 {}

#[derive(Builder)]
#[builder(crate = super::bon)]
struct Relative2 {}

#[derive(Builder)]
#[builder(crate = bon)]
struct Relative3 {}

#[builder(crate = self::bon)]
fn relative_1() {}

#[builder(crate = super::bon)]
fn relative_2() {}

#[builder(crate = bon)]
fn relative_3() {}

struct CrateAttrInMethod;

#[bon]
impl CrateAttrInMethod {
    #[builder(crate = ::bon)]
    fn method() {}
}

struct Relative;

#[bon(crate = self::bon)]
impl Relative {
    #[builder]
    fn method1() {}
}

#[bon(crate = super::bon)]
impl Relative {
    #[builder]
    fn method2() {}
}

#[bon(crate = bon)]
impl Relative {
    #[builder]
    fn method3() {}
}

fn main() {}
