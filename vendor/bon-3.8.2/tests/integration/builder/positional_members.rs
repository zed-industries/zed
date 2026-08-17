use crate::prelude::*;

struct IntoStrRef<'a>(&'a str);

impl<'a> From<IntoStrRef<'a>> for &'a str {
    fn from(val: IntoStrRef<'a>) -> Self {
        val.0
    }
}

struct IntoChar(char);

impl From<IntoChar> for char {
    fn from(val: IntoChar) -> Self {
        val.0
    }
}

mod smoke {
    use super::*;
    use core::fmt;

    #[test]
    fn test_struct() {
        #[derive(Debug, Builder)]
        #[allow(dead_code)]
        struct Sut {
            #[builder(start_fn)]
            starter_1: bool,

            #[builder(start_fn, into)]
            starter_2: char,

            #[builder(start_fn, into)]
            starter_3: Option<&'static str>,

            #[builder(finish_fn)]
            finisher_1: &'static str,

            #[builder(finish_fn, into)]
            finisher_2: &'static str,

            named: u32,
        }

        assert_debug_eq(
            Sut::builder(true, IntoChar('c'), None)
                .named(99)
                .build("1", IntoStrRef("2")),
            expect![[r#"
            Sut {
                starter_1: true,
                starter_2: 'c',
                starter_3: None,
                finisher_1: "1",
                finisher_2: "2",
                named: 99,
            }"#]],
        );

        let _actual = Sut::builder(true, 'c', "str");
    }

    #[test]
    fn test_function() {
        #[builder]
        fn sut(
            #[builder(start_fn)] starter_1: bool,
            #[builder(start_fn, into)] starter_2: char,
            #[builder(start_fn, into)] starter_3: Option<&'static str>,
            #[builder(finish_fn)] finisher_1: &'static str,
            #[builder(finish_fn, into)] finisher_2: &'static str,
            named: u32,
        ) -> impl fmt::Debug {
            (
                starter_1, starter_2, starter_3, named, finisher_1, finisher_2,
            )
        }

        assert_debug_eq(
            sut(true, IntoChar('c'), None)
                .named(99)
                .call("1", IntoStrRef("2")),
            expect![[r#"(true, 'c', None, 99, "1", "2")"#]],
        );

        let _actual = sut(true, 'c', "str");
    }

    #[test]
    fn test_method() {
        struct Sut;

        #[bon]
        impl Sut {
            #[builder]
            fn sut(
                #[builder(start_fn)] starter_1: bool,
                #[builder(start_fn, into)] starter_2: char,
                #[builder(start_fn, into)] starter_3: Option<&'static str>,
                #[builder(finish_fn)] finisher_1: &'static str,
                #[builder(finish_fn, into)] finisher_2: &'static str,
                named: u32,
            ) -> impl fmt::Debug {
                (
                    starter_1, starter_2, starter_3, named, finisher_1, finisher_2,
                )
            }

            #[builder]
            fn with_self(
                &self,
                #[builder(start_fn)] starter_1: bool,
                #[builder(finish_fn)] finisher_1: &'static str,
                named: u32,
            ) -> impl fmt::Debug {
                let _ = self;
                (starter_1, named, finisher_1)
            }
        }

        assert_debug_eq(
            Sut::sut(true, IntoChar('c'), None)
                .named(99)
                .call("1", IntoStrRef("2")),
            expect![[r#"(true, 'c', None, 99, "1", "2")"#]],
        );

        let _actual = Sut::sut(true, 'c', "str");

        assert_debug_eq(
            Sut.with_self(true).named(99).call("1"),
            expect![[r#"(true, 99, "1")"#]],
        );
    }
}

mod attr_on {
    use super::*;
    use core::fmt;

    #[test]
    fn test_struct() {
        #[derive(Debug, Builder)]
        #[builder(on(&str, into))]
        #[allow(dead_code)]
        struct Sut {
            #[builder(start_fn)]
            starter_1: bool,

            #[builder(start_fn, into)]
            starter_3: Option<&'static str>,

            #[builder(finish_fn)]
            finisher_1: &'static str,

            named: u32,
        }

        assert_debug_eq(
            Sut::builder(true, "Roseluck")
                .named(99)
                .build(IntoStrRef("Daisy")),
            expect![[r#"
                Sut {
                    starter_1: true,
                    starter_3: Some(
                        "Roseluck",
                    ),
                    finisher_1: "Daisy",
                    named: 99,
                }"#]],
        );
    }

    #[test]
    fn test_function() {
        #[builder(on(&str, into))]
        fn sut(
            #[builder(start_fn)] starter_1: bool,

            #[builder(start_fn, into)] starter_3: Option<&'static str>,

            #[builder(finish_fn)] finisher_1: &'static str,

            named: u32,
        ) -> impl fmt::Debug {
            (starter_1, starter_3, finisher_1, named)
        }

        assert_debug_eq(
            sut(true, "Roseluck").named(99).call(IntoStrRef("Daisy")),
            expect![[r#"(true, Some("Roseluck"), "Daisy", 99)"#]],
        );
    }

    #[test]
    fn test_method() {
        struct Sut;

        #[bon]
        impl Sut {
            #[builder(on(&str, into))]
            fn sut(
                #[builder(start_fn)] starter_1: bool,

                #[builder(start_fn, into)] starter_3: Option<&'static str>,

                #[builder(finish_fn)] finisher_1: &'static str,

                named: u32,
            ) -> impl fmt::Debug {
                (starter_1, starter_3, finisher_1, named)
            }

            #[builder(on(&str, into))]
            fn with_self(
                &self,
                #[builder(start_fn)] starter_1: bool,
                #[builder(finish_fn)] finisher_1: &'static str,
                named: u32,
            ) -> impl fmt::Debug {
                let _ = self;
                (starter_1, finisher_1, named)
            }
        }

        assert_debug_eq(
            Sut::sut(true, "Roseluck")
                .named(99)
                .call(IntoStrRef("Daisy")),
            expect![[r#"(true, Some("Roseluck"), "Daisy", 99)"#]],
        );

        assert_debug_eq(
            Sut.with_self(true).named(99).call("Daisy"),
            expect![[r#"(true, "Daisy", 99)"#]],
        );
    }
}

mod generics {
    use super::*;
    use core::fmt::Debug;

    #[test]
    fn test_struct() {
        #[derive(Debug, Builder)]
        #[allow(dead_code)]
        #[builder(derive(Debug, Clone))]
        struct Sut<'a, 'b, T, U, A, const N1: usize, const N2: usize>
        where
            T: PartialEq + Debug + Clone,
            Self: Sized + 'b,
            U: Debug + Clone,
            A: Debug + Clone,
        {
            #[builder(start_fn)]
            starter: &'a [T; N1],

            #[builder(finish_fn, into)]
            finisher: [&'b U; N2],

            named: A,
        }

        let builder = Sut::builder(&[1_u32, 2, 3]);

        assert_debug_eq(
            builder.clone(),
            expect!["SutBuilder { starter: [1, 2, 3] }"],
        );

        assert_debug_eq(
            builder.named(99).build([&false]),
            expect!["Sut { starter: [1, 2, 3], finisher: [false], named: 99 }"],
        );
    }

    #[test]
    fn test_function() {
        #[builder(derive(Debug, Clone))]
        fn sut<'a, 'b, T, U, A, const N1: usize, const N2: usize>(
            #[builder(start_fn)] starter: &'a [T; N1],
            #[builder(finish_fn, into)] finisher: [&'b U; N2],
            named: A,
        ) -> (&'a [T; N1], [&'b U; N2], A)
        where
            T: PartialEq + Debug + Clone,
            U: Debug + Clone,
            A: Debug + Clone,
        {
            (starter, finisher, named)
        }

        let builder = sut(&[1_u32, 2, 3]);

        assert_debug_eq(
            builder.clone(),
            expect!["SutBuilder { starter: [1, 2, 3] }"],
        );

        assert_debug_eq(
            builder.named(99).call([&false]),
            expect!["([1, 2, 3], [false], 99)"],
        );
    }

    #[test]
    fn test_impl_trait_free_fn() {
        #[builder(derive(Clone, Debug))]
        fn sut(
            #[builder(start_fn)] starter: impl IntoIterator<Item = u32> + Clone + Debug,
            #[builder(finish_fn)] _finisher: impl Debug + Clone + 'static,
        ) -> u32 {
            starter.into_iter().sum()
        }

        let builder = sut([1, 2, 3]);

        assert_debug_eq(
            builder.clone(),
            expect!["SutBuilder { starter: [1, 2, 3] }"],
        );

        assert_eq!(builder.call(()), 6);
    }

    #[test]
    fn test_method() {
        #[derive(Debug, Clone)]
        struct Sut;

        #[bon]
        impl Sut {
            #[builder(derive(Debug, Clone))]
            fn sut<'a, 'b, T, U, A, const N1: usize, const N2: usize>(
                #[builder(start_fn)] starter_1: &'a [T; N1],
                #[builder(finish_fn, into)] starter_2: [&'b U; N2],
                named: A,
            ) -> (&'a [T; N1], [&'b U; N2], A)
            where
                T: PartialEq + Debug + Clone,
                U: Debug + Clone,
                A: Debug + Clone,
            {
                (starter_1, starter_2, named)
            }

            #[builder(derive(Debug, Clone))]
            fn with_self<'a, 'b, T, U, A, const N1: usize, const N2: usize>(
                &self,
                #[builder(start_fn)] starter_1: &'a [T; N1],
                #[builder(finish_fn, into)] starter_2: [&'b U; N2],
                named: A,
            ) -> (&'a [T; N1], [&'b U; N2], A)
            where
                T: PartialEq + Debug + Clone,
                U: Debug + Clone,
                A: Debug + Clone,
            {
                let _ = self;
                (starter_1, starter_2, named)
            }
        }

        let builder = Sut::sut(&[1_u32, 2, 3]);

        assert_debug_eq(
            builder.clone(),
            expect!["SutSutBuilder { starter_1: [1, 2, 3] }"],
        );

        assert_debug_eq(
            builder.named(99).call([&false]),
            expect!["([1, 2, 3], [false], 99)"],
        );

        assert_debug_eq(
            Sut.with_self(&[1_u32, 2, 3]).named(99).call([&false]),
            expect!["([1, 2, 3], [false], 99)"],
        );
    }
}

mod mut_params {
    use crate::prelude::*;

    #[test]
    fn test_function() {
        #[builder]
        fn sut(#[builder(start_fn)] mut x1: u32, #[builder(finish_fn)] mut x2: u32) {
            x1 += 1;
            x2 += 1;
            let _ = (x1, x2);
        }

        sut(1).call(2);
    }

    #[test]
    fn test_method() {
        struct Sut;

        #[bon]
        impl Sut {
            #[builder]
            #[allow(clippy::needless_pass_by_ref_mut)]
            fn sut(&mut self, #[builder(start_fn)] mut x1: u32, #[builder(finish_fn)] mut x2: u32) {
                let _ = self;
                x1 += 1;
                x2 += 1;
                let _ = (x1, x2);
            }
        }

        Sut.sut(1).call(2);
    }
}
