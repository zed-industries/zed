mod smoke {
    use crate::prelude::*;
    use core::fmt;

    #[test]
    fn test_struct() {
        #[derive(Debug, Builder)]
        #[allow(dead_code)]
        struct Sut {
            #[builder(overwritable)]
            a: u32,

            #[builder(overwritable)]
            b: Option<u32>,

            #[builder(overwritable, default)]
            c: u32,
        }

        assert_debug_eq(
            Sut::builder().a(1).a(2).b(3).b(4).c(5).c(6).build(),
            expect!["Sut { a: 2, b: Some(4), c: 6 }"],
        );
    }

    #[test]
    fn test_function() {
        #[builder]
        fn sut(
            #[builder(overwritable)] a: u32,
            #[builder(overwritable)] b: Option<u32>,
            #[builder(overwritable, default)] c: u32,
        ) -> impl fmt::Debug {
            (a, b, c)
        }

        assert_debug_eq(
            sut().a(1).a(2).b(3).b(4).c(5).c(6).call(),
            expect!["(2, Some(4), 6)"],
        );
    }

    #[test]
    fn test_method() {
        struct Sut;

        #[bon]
        impl Sut {
            #[builder]
            fn sut(
                #[builder(overwritable)] a: u32,
                #[builder(overwritable)] b: Option<u32>,
                #[builder(overwritable, default)] c: u32,
            ) -> impl fmt::Debug {
                (a, b, c)
            }

            #[builder]
            fn with_self(
                &self,
                #[builder(overwritable)] a: u32,
                #[builder(overwritable)] b: Option<u32>,
                #[builder(overwritable, default)] c: u32,
            ) -> impl fmt::Debug {
                let _ = self;
                (a, b, c)
            }
        }

        assert_debug_eq(
            Sut::sut().a(1).a(2).b(3).b(4).c(5).c(6).call(),
            expect!["(2, Some(4), 6)"],
        );

        assert_debug_eq(
            Sut.with_self().a(1).a(2).b(3).b(4).c(5).c(6).call(),
            expect!["(2, Some(4), 6)"],
        );
    }
}

mod on {
    use crate::prelude::*;

    #[test]
    fn test_struct() {
        #[derive(Debug, Builder)]
        #[allow(dead_code)]
        #[builder(on(u32, overwritable))]
        struct Sut {
            a: u32,
            b: Option<u32>,

            #[builder(default)]
            c: u32,
        }

        assert_debug_eq(
            Sut::builder().a(1).a(2).b(3).b(4).c(5).c(6).build(),
            expect!["Sut { a: 2, b: Some(4), c: 6 }"],
        );
    }
}
