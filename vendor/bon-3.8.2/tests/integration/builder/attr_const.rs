#[rustversion::since(1.61.0)]

mod msrv_1_61 {

    mod smoke {

        use crate::prelude::*;

        #[test]
        const fn test_struct() {
            #[derive(Builder)]
            #[builder(const)]
            struct Sut {
                #[builder(start_fn)]
                x1: u32,

                // There was a bug in `#[builder(field)]` `const` support, that
                // this is testing for: https://github.com/elastio/bon/issues/290
                #[builder(field = 11)]
                x2: u32,

                #[builder(finish_fn)]
                x3: u32,

                x4: u32,

                x5: Option<u32>,

                #[builder(default = x1 + 99)]
                x6: u32,

                #[builder(with = |a: u32, b: u32| a + b)]
                x7: u32,

                #[builder(with = |a: u32, b: u32| -> Result<_, ()> { Ok(a + b) })]
                x8: Option<u32>,

                #[builder(skip = 10)]
                x9: u32,
                //
                // This doesn't work because Rust complains about this in setters that
                // consume `self` and return a new instance of the builder:
                // ```
                // destructor of `builder::attr_const::test_struct::SutBuilder<S>`
                // cannot be evaluated at compile-time
                // ```
                // x7: Vec<String>,
                // x8: String,
            }

            const ACTUAL: Sut = Sut::builder(1).x4(2).x5(3).x7(4, 5).build(6);

            #[allow(clippy::assertions_on_constants)]
            {
                assert!(ACTUAL.x1 == 1);
                assert!(ACTUAL.x2 == 11);
                assert!(ACTUAL.x3 == 6);
                assert!(ACTUAL.x4 == 2);
                assert!(matches!(ACTUAL.x5, Some(3)));
                assert!(ACTUAL.x6 == 100);
                assert!(ACTUAL.x7 == 9);
                assert!(ACTUAL.x8.is_none());
                assert!(ACTUAL.x9 == 10);
            }
        }

        #[rustversion::since(1.83.0)]
        #[test]
        const fn test_struct_msrv_1_83() {
            #[derive(Builder)]
            #[builder(const)]
            struct Sut {
                #[builder(start_fn)]
                #[allow(dead_code)]
                x1: u32,

                #[builder(field = 11)]
                #[allow(dead_code)]
                x2: u32,
            }

            impl<S: sut_builder::State> SutBuilder<S> {
                const fn inc(&mut self) {
                    self.x1 += 1;
                    self.x2 += 1;
                }
            }

            let mut builder = Sut::builder(1);
            builder.inc();
            builder.inc();
            assert!(builder.x1 == 3);
            assert!(builder.x2 == 13);
        }

        #[test]
        const fn test_function() {
            type Output = (u32, u32, u32, u32, Option<u32>, u32, u32, Option<u32>);

            #[builder(const)]
            const fn sut(
                #[builder(start_fn)] x1: u32,

                #[builder(field = 10)] x2: u32,

                #[builder(finish_fn)] x3: u32,

                x4: u32,

                x5: Option<u32>,

                #[builder(default = x1 + 99)] x6: u32,

                #[builder(with = |a: u32, b: u32| a + b)] x7: u32,

                #[builder(with = |a: u32, b: u32| -> Result<_, ()> { Ok(a + b) })] x8: Option<u32>,
            ) -> Output {
                (x1, x2, x3, x4, x5, x6, x7, x8)
            }

            const ACTUAL: Output = sut(1).x4(2).x5(3).x7(4, 5).call(6);

            #[allow(clippy::assertions_on_constants)]
            {
                assert!(ACTUAL.0 == 1);
                assert!(ACTUAL.1 == 10);
                assert!(ACTUAL.2 == 6);
                assert!(ACTUAL.3 == 2);
                assert!(matches!(ACTUAL.4, Some(3)));
                assert!(ACTUAL.5 == 100);
                assert!(ACTUAL.6 == 9);
                assert!(ACTUAL.7.is_none());
            }
        }

        #[test]
        const fn test_method() {
            type Output = (u32, u32, u32, u32, Option<u32>, u32, u32, Option<u32>);

            struct Sut;

            #[bon]
            impl Sut {
                #[builder(const)]
                const fn sut(
                    #[builder(start_fn)] x1: u32,

                    #[builder(field = 10)] x2: u32,

                    #[builder(finish_fn)] x3: u32,

                    x4: u32,

                    x5: Option<u32>,

                    #[builder(default = x1 + 99)] x6: u32,

                    #[builder(with = |a: u32, b: u32| a + b)] x7: u32,

                    #[builder(with = |a: u32, b: u32| -> Result<_, ()> { Ok(a + b) })] x8: Option<
                        u32,
                    >,
                ) -> Output {
                    (x1, x2, x3, x4, x5, x6, x7, x8)
                }
            }

            const ACTUAL: Output = Sut::sut(1).x4(2).x5(3).x7(4, 5).call(6);

            #[allow(clippy::assertions_on_constants)]
            {
                assert!(ACTUAL.0 == 1);
                assert!(ACTUAL.1 == 10);
                assert!(ACTUAL.2 == 6);
                assert!(ACTUAL.3 == 2);
                assert!(matches!(ACTUAL.4, Some(3)));
                assert!(ACTUAL.5 == 100);
                assert!(ACTUAL.6 == 9);
                assert!(ACTUAL.7.is_none());
            }
        }
    }

    // Tests for the following bug: https://github.com/elastio/bon/issues/287
    mod visibility {
        use crate::prelude::*;

        #[test]
        const fn test_struct() {
            #[derive(Builder)]
            #[builder(const)]
            #[allow(unreachable_pub)]
            pub struct Sut {
                x1: u32,
            }

            const ACTUAL: Sut = Sut::builder().x1(1).build();

            #[allow(clippy::assertions_on_constants)]
            {
                assert!(ACTUAL.x1 == 1);
            }
        }

        #[test]
        const fn test_function() {
            #[builder(const)]
            pub(crate) const fn sut(#[builder(getter)] _x: u32) {}

            sut().x(1).call();
        }

        #[test]
        const fn test_method() {
            struct Sut;

            #[bon]
            impl Sut {
                #[builder(const)]
                pub(crate) const fn sut(#[builder(getter)] _x: u32) {}
            }

            Sut::sut().x(1).call();
        }
    }
}
