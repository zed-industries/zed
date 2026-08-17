mod member_level {
    use crate::prelude::*;
    use core::fmt;

    #[test]
    fn test_struct() {
        #[derive(Debug, Builder)]
        #[allow(dead_code)]
        struct Sut<T> {
            #[builder(required)]
            regular: Option<u32>,

            #[builder(required)]
            generic: Option<T>,

            #[builder(required, into)]
            with_into: Option<u32>,

            #[builder(required, default = Some(99))]
            with_default: Option<u32>,

            #[builder(required, default = Some(10))]
            with_default_2: Option<u32>,
        }

        assert_debug_eq(
            Sut::builder()
                .regular(Some(1))
                .generic(Some(false))
                .with_into(2)
                .maybe_with_default_2(Some(Some(3)))
                .build(),
            expect![[r#"
                Sut {
                    regular: Some(
                        1,
                    ),
                    generic: Some(
                        false,
                    ),
                    with_into: Some(
                        2,
                    ),
                    with_default: Some(
                        99,
                    ),
                    with_default_2: Some(
                        3,
                    ),
                }"#]],
        );
    }

    #[test]
    fn test_function() {
        #[builder]
        fn sut<T: fmt::Debug>(
            #[builder(required)] regular: Option<u32>,
            #[builder(required)] generic: Option<T>,
            #[builder(required, into)] with_into: Option<u32>,
            #[builder(required, default = Some(99))] with_default: Option<u32>,
            #[builder(required, default = Some(10))] with_default_2: Option<u32>,
        ) -> impl fmt::Debug {
            (regular, generic, with_into, with_default, with_default_2)
        }

        assert_debug_eq(
            sut()
                .regular(Some(1))
                .generic(Some(false))
                .with_into(2)
                .maybe_with_default_2(Some(Some(3)))
                .call(),
            expect!["(Some(1), Some(false), Some(2), Some(99), Some(3))"],
        );
    }

    #[test]
    fn test_method() {
        struct Sut;

        #[bon]
        impl Sut {
            #[builder]
            fn sut<T: fmt::Debug>(
                #[builder(required)] regular: Option<u32>,
                #[builder(required)] generic: Option<T>,
                #[builder(required, into)] with_into: Option<u32>,
                #[builder(required, default = Some(99))] with_default: Option<u32>,
                #[builder(required, default = Some(10))] with_default_2: Option<u32>,
            ) -> impl fmt::Debug {
                (regular, generic, with_into, with_default, with_default_2)
            }

            #[builder]
            fn with_self<T: fmt::Debug>(
                &self,
                #[builder(required)] regular: Option<u32>,
                #[builder(required)] generic: Option<T>,
                #[builder(required, into)] with_into: Option<u32>,
                #[builder(required, default = Some(99))] with_default: Option<u32>,
                #[builder(required, default = Some(10))] with_default_2: Option<u32>,
            ) -> impl fmt::Debug {
                let _ = self;
                (regular, generic, with_into, with_default, with_default_2)
            }
        }

        assert_debug_eq(
            Sut::sut()
                .regular(Some(1))
                .generic(Some(false))
                .with_into(2)
                .maybe_with_default_2(Some(Some(3)))
                .call(),
            expect!["(Some(1), Some(false), Some(2), Some(99), Some(3))"],
        );

        assert_debug_eq(
            Sut.with_self()
                .regular(Some(1))
                .generic(Some(false))
                .with_into(2)
                .maybe_with_default_2(Some(Some(3)))
                .call(),
            expect!["(Some(1), Some(false), Some(2), Some(99), Some(3))"],
        );
    }
}

mod attr_on {
    use crate::prelude::*;
    use core::fmt;

    #[test]
    fn test_struct() {
        #[derive(Debug, Builder)]
        #[builder(on(_, required))]
        #[allow(dead_code)]
        struct Sut<T> {
            #[builder(start_fn)]
            start_fn: u32,

            regular: Option<u32>,
            generic: Option<T>,

            #[builder(into)]
            with_into: Option<u32>,

            #[builder(default = Some(99))]
            with_default: Option<u32>,

            #[builder(default = Some(10))]
            with_default_2: Option<u32>,
        }

        assert_debug_eq(
            Sut::builder(11)
                .regular(Some(1))
                .generic(Some(false))
                .with_into(2)
                .maybe_with_default_2(Some(Some(3)))
                .build(),
            expect![[r#"
                Sut {
                    start_fn: 11,
                    regular: Some(
                        1,
                    ),
                    generic: Some(
                        false,
                    ),
                    with_into: Some(
                        2,
                    ),
                    with_default: Some(
                        99,
                    ),
                    with_default_2: Some(
                        3,
                    ),
                }"#]],
        );
    }

    #[test]
    fn test_function() {
        #[builder(on(_, required))]
        fn sut<T: fmt::Debug>(
            #[builder(start_fn)] start_fn: u32,
            regular: Option<u32>,
            generic: Option<T>,
            #[builder(into)] with_into: Option<u32>,
            #[builder(default = Some(99))] with_default: Option<u32>,
            #[builder(default = Some(10))] with_default_2: Option<u32>,
        ) -> impl fmt::Debug {
            (
                start_fn,
                regular,
                generic,
                with_into,
                with_default,
                with_default_2,
            )
        }

        assert_debug_eq(
            sut(11)
                .regular(Some(1))
                .generic(Some(false))
                .with_into(2)
                .maybe_with_default_2(Some(Some(3)))
                .call(),
            expect!["(11, Some(1), Some(false), Some(2), Some(99), Some(3))"],
        );
    }

    #[test]
    fn test_method() {
        struct Sut;

        #[bon]
        impl Sut {
            #[builder(on(_, required))]
            fn sut<T: fmt::Debug>(
                #[builder(start_fn)] start_fn: u32,
                regular: Option<u32>,
                generic: Option<T>,
                #[builder(into)] with_into: Option<u32>,
                #[builder(default = Some(99))] with_default: Option<u32>,
                #[builder(default = Some(10))] with_default_2: Option<u32>,
            ) -> impl fmt::Debug {
                (
                    start_fn,
                    regular,
                    generic,
                    with_into,
                    with_default,
                    with_default_2,
                )
            }

            #[builder(on(_, required))]
            fn with_self<T: fmt::Debug>(
                &self,
                #[builder(start_fn)] start_fn: u32,
                regular: Option<u32>,
                generic: Option<T>,
                #[builder(into)] with_into: Option<u32>,
                #[builder(default = Some(99))] with_default: Option<u32>,
                #[builder(default = Some(10))] with_default_2: Option<u32>,
            ) -> impl fmt::Debug {
                let _ = self;
                (
                    start_fn,
                    regular,
                    generic,
                    with_into,
                    with_default,
                    with_default_2,
                )
            }
        }

        assert_debug_eq(
            Sut::sut(11)
                .regular(Some(1))
                .generic(Some(false))
                .with_into(2)
                .maybe_with_default_2(Some(Some(3)))
                .call(),
            expect!["(11, Some(1), Some(false), Some(2), Some(99), Some(3))"],
        );

        assert_debug_eq(
            Sut.with_self(11)
                .regular(Some(1))
                .generic(Some(false))
                .with_into(2)
                .maybe_with_default_2(Some(Some(3)))
                .call(),
            expect!["(11, Some(1), Some(false), Some(2), Some(99), Some(3))"],
        );
    }
}
