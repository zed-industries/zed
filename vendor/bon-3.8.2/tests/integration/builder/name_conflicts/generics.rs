mod lifetimes {
    use crate::prelude::*;

    #[test]
    fn test_function() {
        #[builder]
        #[allow(
            single_use_lifetimes,
            clippy::needless_lifetimes,
            clippy::trivially_copy_pass_by_ref
        )]
        fn sut<'f1, 'f1_, 'f2>(
            _x1: &u32,
            _x2: &'f1 u32,
            _x3: &'f1_ u32,
            _x4: &'f2 u32,
            _x5: &u32,
        ) -> u32 {
            32
        }

        sut().x1(&32).x2(&32).x3(&32).x4(&32).x5(&32).call();
    }

    #[test]
    fn test_method() {
        #[derive(Default)]
        #[allow(dead_code)]
        struct Sut<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h>(
            &'a str,
            &'b str,
            &'c str,
            &'d str,
            &'e str,
            &'f str,
            &'g str,
            &'h str,
        );

        #[bon]
        impl<'i1, 'i1_, 'i2, 'f1, 'f1_, 'f2> Sut<'_, '_, 'i1, 'i1_, 'i2, 'f1, 'f1_, 'f2> {
            #[builder]
            #[allow(clippy::trivially_copy_pass_by_ref)]
            fn sut(_val: &u32, _val2: &u32) {}
        }

        Sut::sut().val(&32).val2(&32).call();
    }
}
mod impl_trait {
    use crate::prelude::*;

    #[test]
    fn test_function() {
        struct I1;
        type I2 = I1;

        impl I1 {
            fn get_val(&self) -> u32 {
                let _ = self;
                32
            }
        }

        {
            #[builder]
            fn sut(_arg1: impl Copy) -> u32 {
                I1.get_val()
            }

            sut().arg1(()).call();
        }

        {
            #[builder]
            fn sut(_arg1: impl Copy, _arg2: impl Sized) -> u32 {
                I2 {}.get_val()
            }

            sut().arg1(()).arg2(()).call();
        }
    }

    #[test]
    fn test_method() {
        struct I1;
        type I2 = I1;

        impl I1 {
            fn get_val(&self) -> u32 {
                let _ = self;
                32
            }
        }

        #[bon]
        impl I1 {
            #[builder]
            #[allow(clippy::use_self)]
            fn sut(_arg1: impl Copy) -> u32 {
                I1.get_val()
            }

            #[builder]
            fn with_self(&self, _arg1: impl Copy, _arg2: impl Sized) -> u32 {
                let _ = self;
                I2 {}.get_val()
            }
        }

        I1::sut().arg1(()).call();
        I1.with_self().arg1(()).arg2(()).call();
    }
}
