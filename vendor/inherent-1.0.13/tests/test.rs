mod types {
    use inherent::inherent;

    trait Trait {
        fn f<T: ?Sized>(self);
        // A default method
        fn g(&self) {}

        #[rustversion::since(1.75)]
        async fn a(&self);
    }

    pub struct Struct;

    #[inherent]
    impl Trait for Struct {
        pub fn f<T: ?Sized>(self) {}
        pub fn g(&self);

        #[rustversion::since(1.75)]
        pub async fn a(&self) {}
    }
}

#[test]
fn test() {
    // types::Trait is not in scope.
    let s = types::Struct;
    s.g();
    s.f::<str>();
}

#[rustversion::since(1.75)]
#[test]
fn test_async() {
    fn assert_future<T: std::future::Future<Output = ()>>(_: T) {}

    let s = types::Struct;
    assert_future(s.a());
}
