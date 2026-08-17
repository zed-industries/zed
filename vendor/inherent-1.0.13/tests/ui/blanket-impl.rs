use inherent::inherent;

trait A {
    fn a(&self);
}

#[inherent]
impl<T> A for T {
    fn a(&self) {}
}

fn main() {}
