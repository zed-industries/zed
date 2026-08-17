// Members marked with `#[builder(start_fn)]` should be available as a private fields in the builder.
mod native_start_fn_fields {
    use crate::prelude::*;

    #[test]
    fn test_struct() {
        #[derive(Builder)]
        struct Sut {
            #[builder(start_fn)]
            #[allow(dead_code)]
            x1: u32,

            #[builder(start_fn)]
            #[allow(dead_code)]
            x2: bool,
        }

        let builder = Sut::builder(1, true);
        let x1: &u32 = &builder.x1;
        let x2: &bool = &builder.x2;

        assert_eq!(*x1, 1);
        assert!(*x2);
    }

    #[test]
    fn test_function() {
        #[builder]
        fn sut(#[builder(start_fn)] x1: u32, #[builder(start_fn)] x2: bool) {
            let _ = x1;
            let _ = x2;
        }

        let builder = sut(1, true);
        let x1: &u32 = &builder.x1;
        let x2: &bool = &builder.x2;

        assert_eq!(*x1, 1);
        assert!(*x2);
    }

    #[test]
    fn test_method() {
        struct Sut;

        #[bon]
        impl Sut {
            #[builder]
            fn method(#[builder(start_fn)] x1: u32, #[builder(start_fn)] x2: bool) {
                let _ = x1;
                let _ = x2;
            }
        }

        let builder = Sut::method(1, true);
        let x1: &u32 = &builder.x1;
        let x2: &bool = &builder.x2;

        assert_eq!(*x1, 1);
        assert!(*x2);
    }
}

mod native_receiver_field {
    use crate::prelude::*;

    // The `self_receiver` field in the builder should be available as a private field in the builder.
    #[test]
    fn native_receiver_field_smoke() {
        struct Sut {
            x1: u32,
        }

        #[bon]
        impl Sut {
            #[builder]
            fn method(self) {
                let _ = self;
            }

            #[builder]
            fn method_ref(&self) {
                let _ = self;
            }

            #[builder]
            fn method_mut(&mut self) {
                self.x1 += 1;
                let _ = self;
            }
        }

        {
            let builder = Sut { x1: 99 }.method();
            let receiver: Sut = builder.self_receiver;

            assert_eq!(receiver.x1, 99);
        }

        {
            let builder = Sut { x1: 99 }.method_ref();
            let receiver: &Sut = builder.self_receiver;

            assert_eq!(receiver.x1, 99);
        }

        {
            let mut sut = Sut { x1: 99 };
            let builder = sut.method_mut();
            let receiver: &mut Sut = builder.self_receiver;

            assert_eq!(receiver.x1, 99);
        }
    }

    #[test]
    fn name_conflict_resolution_for_receiver_field() {
        struct Sut {
            x1: u32,
        }

        #[bon]
        impl Sut {
            #[builder]
            fn method(
                &self,
                #[builder(start_fn)] self_receiver: u32,
                #[builder(start_fn)] self_receiver_: bool,
            ) {
                let _ = self_receiver;
                let _ = self_receiver_;
                let _ = self;
            }
        }

        let builder = Sut { x1: 99 }.method(2, true);
        let self_receiver: &u32 = &builder.self_receiver;
        let self_receiver_: &bool = &builder.self_receiver_;
        let self_receiver__: &Sut = builder.self_receiver__;

        assert_eq!(*self_receiver, 2);
        assert!(*self_receiver_);
        assert_eq!(self_receiver__.x1, 99);
    }
}
