use crate::prelude::*;
use core::fmt;

#[cfg(feature = "alloc")]
#[test]
fn fn_alloc() {
    #[builder]
    fn sut(
        #[builder(default = "default".to_owned())] arg1: String,
        #[builder(default = vec![42])] arg2: Vec<u32>,
        #[builder(default = "foo", into)] arg3: String,
    ) -> impl fmt::Debug {
        (arg1, arg2, arg3)
    }

    assert_debug_eq(sut().call(), expect![[r#"("default", [42], "foo")"#]]);

    assert_debug_eq(
        sut().arg1("arg1".to_owned()).call(),
        expect![[r#"("arg1", [42], "foo")"#]],
    );

    assert_debug_eq(
        sut().maybe_arg1(None).call(),
        expect![[r#"("default", [42], "foo")"#]],
    );

    assert_debug_eq(
        sut().maybe_arg3(None::<String>).call(),
        expect![[r#"("default", [42], "foo")"#]],
    );
}

#[cfg(feature = "alloc")]
#[test]
fn struct_alloc() {
    use bon::bon;
    use expect_test::expect;

    #[derive(Debug, Builder)]
    struct Sut {
        #[builder(default = "default".to_owned())]
        arg1: String,

        #[builder(default = vec![42])]
        arg2: Vec<u32>,

        #[builder(default = "foo", into)]
        arg3: String,
    }

    assert_debug_eq(
        Sut::builder().build(),
        expect![[r#"Sut { arg1: "default", arg2: [42], arg3: "foo" }"#]],
    );

    assert_debug_eq(
        Sut::builder().arg1("arg1".to_owned()).build(),
        expect![[r#"Sut { arg1: "arg1", arg2: [42], arg3: "foo" }"#]],
    );

    assert_debug_eq(
        Sut::builder().maybe_arg1(None::<String>).build(),
        expect![[r#"Sut { arg1: "default", arg2: [42], arg3: "foo" }"#]],
    );

    #[bon]
    #[allow(clippy::items_after_statements)]
    impl Sut {
        #[builder]
        fn assoc(
            self,
            #[builder(default = "default".to_owned())] arg1: String,
            #[builder(default = vec![43])] arg2: Vec<u32>,
            #[builder(default = "foo", into)] arg3: String,
        ) -> Self {
            Self {
                arg1: format!("{}+{arg1}", self.arg1),
                arg2: self.arg2.into_iter().chain(arg2).collect(),
                arg3: format!("{}+{arg3}", self.arg3),
            }
        }
    }

    let sut = || Sut::builder().build();

    assert_debug_eq(
        sut().assoc().call(),
        expect![[r#"
            Sut {
                arg1: "default+default",
                arg2: [
                    42,
                    43,
                ],
                arg3: "foo+foo",
            }"#]],
    );

    assert_debug_eq(
        sut().assoc().arg1("arg1".to_owned()).call(),
        expect![[r#"
            Sut {
                arg1: "default+arg1",
                arg2: [
                    42,
                    43,
                ],
                arg3: "foo+foo",
            }"#]],
    );

    assert_debug_eq(
        sut().assoc().maybe_arg1(None).call(),
        expect![[r#"
            Sut {
                arg1: "default+default",
                arg2: [
                    42,
                    43,
                ],
                arg3: "foo+foo",
            }"#]],
    );

    assert_debug_eq(
        sut().assoc().maybe_arg3(None::<String>).call(),
        expect![[r#"
            Sut {
                arg1: "default+default",
                arg2: [
                    42,
                    43,
                ],
                arg3: "foo+foo",
            }"#]],
    );
}

#[test]
fn fn_no_std() {
    #[builder]
    fn sut(#[builder(default)] arg1: u32, #[builder(default = 42)] arg2: u32) -> impl fmt::Debug {
        (arg1, arg2)
    }

    assert_debug_eq(sut().call(), expect!["(0, 42)"]);
    assert_debug_eq(sut().arg1(12).call(), expect!["(12, 42)"]);
    assert_debug_eq(sut().maybe_arg1(None::<u32>).call(), expect!["(0, 42)"]);
}

#[test]
fn struct_no_std() {
    use bon::bon;

    #[derive(Debug, Builder)]
    struct Sut {
        #[builder(default)]
        arg1: u32,

        #[builder(default = 42)]
        arg2: u32,
    }

    assert_debug_eq(Sut::builder().build(), expect!["Sut { arg1: 0, arg2: 42 }"]);

    assert_debug_eq(
        Sut::builder().arg1(12).build(),
        expect!["Sut { arg1: 12, arg2: 42 }"],
    );

    assert_debug_eq(
        Sut::builder().maybe_arg1(None::<u32>).build(),
        expect!["Sut { arg1: 0, arg2: 42 }"],
    );

    #[bon]
    #[allow(clippy::items_after_statements)]
    impl Sut {
        #[builder]
        fn assoc(self, #[builder(default)] arg1: u32, #[builder(default = 43)] arg2: u32) -> Self {
            Self {
                arg1: self.arg1 + arg1,
                arg2: self.arg2 + arg2,
            }
        }
    }

    let sut = || Sut::builder().build();

    assert_debug_eq(sut().assoc().call(), expect!["Sut { arg1: 0, arg2: 85 }"]);
    assert_debug_eq(
        sut().assoc().arg1(12).call(),
        expect!["Sut { arg1: 12, arg2: 85 }"],
    );
    assert_debug_eq(
        sut().assoc().maybe_arg1(None::<u32>).call(),
        expect!["Sut { arg1: 0, arg2: 85 }"],
    );
}

#[test]
fn fn_generic_default() {
    #[builder]
    fn sut(
        #[builder(default)] arg1: impl Clone + Default,
        #[builder(default = <_>::default())] arg2: impl Clone + Default,
    ) {
        drop(arg1);
        drop(arg2);
    }

    sut::<(), ()>().call();
}

mod interaction_with_positional_members {
    use crate::prelude::*;
    use core::fmt;

    #[test]
    fn test_struct() {
        #[derive(Builder, Debug)]
        #[allow(dead_code)]
        struct Sut {
            #[builder(start_fn)]
            starter_1: u32,

            #[builder(start_fn)]
            starter_2: u32,

            #[builder(finish_fn)]
            finisher_1: u32,

            #[builder(finish_fn)]
            finisher_2: u32,

            #[builder(default = [starter_1, starter_2, finisher_1, finisher_2])]
            named_1: [u32; 4],

            #[builder(default = (32, named_1))]
            named_2: (u32, [u32; 4]),
        }

        assert_debug_eq(
            Sut::builder(1, 2).build(3, 4),
            expect![[r#"
                Sut {
                    starter_1: 1,
                    starter_2: 2,
                    finisher_1: 3,
                    finisher_2: 4,
                    named_1: [
                        1,
                        2,
                        3,
                        4,
                    ],
                    named_2: (
                        32,
                        [
                            1,
                            2,
                            3,
                            4,
                        ],
                    ),
                }"#]],
        );

        assert_debug_eq(
            Sut::builder(1, 2).named_1([5, 6, 7, 8]).build(3, 4),
            expect![[r#"
                Sut {
                    starter_1: 1,
                    starter_2: 2,
                    finisher_1: 3,
                    finisher_2: 4,
                    named_1: [
                        5,
                        6,
                        7,
                        8,
                    ],
                    named_2: (
                        32,
                        [
                            5,
                            6,
                            7,
                            8,
                        ],
                    ),
                }"#]],
        );
    }

    #[test]
    fn test_function() {
        #[builder]
        #[allow(clippy::type_complexity)]
        fn sut(
            #[builder(start_fn)] starter_1: u32,
            #[builder(start_fn)] starter_2: u32,
            #[builder(finish_fn)] finisher_1: u32,
            #[builder(finish_fn)] finisher_2: u32,
            #[builder(default = [starter_1, starter_2, finisher_1, finisher_2])] //
            named_1: [u32; 4],
            #[builder(default = (32, named_1))] named_2: (u32, [u32; 4]),
        ) -> impl fmt::Debug {
            (
                starter_1, starter_2, finisher_1, finisher_2, named_1, named_2,
            )
        }

        assert_debug_eq(
            sut(1, 2).call(3, 4),
            expect!["(1, 2, 3, 4, [1, 2, 3, 4], (32, [1, 2, 3, 4]))"],
        );

        assert_debug_eq(
            sut(1, 2).named_1([5, 6, 7, 8]).call(3, 4),
            expect!["(1, 2, 3, 4, [5, 6, 7, 8], (32, [5, 6, 7, 8]))"],
        );
    }

    #[test]
    fn test_method() {
        struct Sut;

        #[bon]
        impl Sut {
            #[builder]
            #[allow(clippy::type_complexity)]
            fn sut(
                #[builder(start_fn)] starter_1: u32,
                #[builder(start_fn)] starter_2: u32,
                #[builder(finish_fn)] finisher_1: u32,
                #[builder(finish_fn)] finisher_2: u32,
                #[builder(default = [starter_1, starter_2, finisher_1, finisher_2])] //
                named_1: [u32; 4],
                #[builder(default = (32, named_1))] named_2: (u32, [u32; 4]),
            ) -> impl fmt::Debug {
                (
                    starter_1, starter_2, finisher_1, finisher_2, named_1, named_2,
                )
            }
        }

        assert_debug_eq(
            Sut::sut(1, 2).call(3, 4),
            expect!["(1, 2, 3, 4, [1, 2, 3, 4], (32, [1, 2, 3, 4]))"],
        );

        assert_debug_eq(
            Sut::sut(1, 2).named_1([5, 6, 7, 8]).call(3, 4),
            expect!["(1, 2, 3, 4, [5, 6, 7, 8], (32, [5, 6, 7, 8]))"],
        );
    }
}
