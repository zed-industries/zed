#![deny(warnings)]

mod unsafe_fields_are_private {
    struct Sut;

    #[bon::bon]
    impl Sut {
        #[builder]
        fn sut(self, #[builder(start_fn)] _x1: u32, _x2: u32) {}
    }

    fn _test() {
        let sut = Sut.sut(99);

        // Previously, there was an attempt to generate names for private fields
        // with randomness to ensure users don't try to access them. This however,
        // conflicts with caching in some build systems. See the following issue
        // for details: https://github.com/elastio/bon/issues/218
        let SutSutBuilder {
            __unsafe_private_phantom: _,
            __unsafe_private_named: _,
            // These are public
            _x1: _,
            self_receiver: _,
        } = sut;
    }
}

mod self_receiver_and_start_fn_args_must_be_private {
    pub struct MustBePrivate;

    #[bon::bon]
    impl MustBePrivate {
        #[builder]
        pub fn method(self, #[builder(start_fn)] _x1: u32) {}
    }
}

fn main() {
    let builder = self_receiver_and_start_fn_args_must_be_private::MustBePrivate.method(99);
    let _ = builder.self_receiver;
    let _ = builder._x1;
}
