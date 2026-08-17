mod types {
    use inherent::inherent;

    trait Trait {
        fn f();
    }

    pub struct Struct;

    #[inherent]
    impl Trait for Struct {
        fn f() {}
    }
}

fn main() {
    types::Struct::f();
}
