use crate::prelude::*;
use expect_test::expect;

#[cfg(feature = "alloc")]
#[test]
fn struct_alloc() {
    use expect_test::expect;

    #[derive(Debug, Builder)]
    struct Sut {
        #[builder(skip = "skip".to_owned())]
        #[allow(dead_code)]
        arg1: String,

        #[builder(skip = vec![42])]
        #[allow(dead_code)]
        arg2: Vec<u32>,
    }

    assert_debug_eq(
        Sut::builder().build(),
        expect![[r#"Sut { arg1: "skip", arg2: [42] }"#]],
    );
}

#[test]
fn struct_no_std() {
    #[derive(Debug, Builder)]
    struct Sut {
        #[builder(skip)]
        #[allow(dead_code)]
        arg1: u32,

        #[builder(skip = 42)]
        #[allow(dead_code)]
        arg2: u32,
    }

    assert_debug_eq(Sut::builder().build(), expect!["Sut { arg1: 0, arg2: 42 }"]);
}

#[test]
fn struct_with_non_skipped_arg() {
    #[derive(Debug, Builder)]
    struct Sut {
        #[builder(skip)]
        #[allow(dead_code)]
        arg1: u32,

        #[allow(dead_code)]
        arg2: u32,
    }

    assert_debug_eq(
        Sut::builder().arg2(24).build(),
        expect!["Sut { arg1: 0, arg2: 24 }"],
    );
}

#[test]
fn struct_generic_skipped() {
    #[derive(Builder)]
    struct Sut<A, B>
    where
        A: Clone + Default,
        B: Clone + Default,
    {
        #[builder(skip)]
        #[allow(dead_code)]
        arg1: A,

        #[builder(skip = <_>::default())]
        #[allow(dead_code)]
        arg2: B,
    }

    let _: Sut<(), ()> = Sut::<(), ()>::builder().build();
}

#[test]
fn interaction_with_positional_members() {
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

        #[builder(skip = [starter_1, starter_2, finisher_1, finisher_2])]
        named_1: [u32; 4],

        #[builder(skip = (32, named_1))]
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
}
