use crate::prelude::*;
use core::fmt;

// Functions don't support `skip` attributes
#[test]
fn fn_init_order() {
    #[builder]
    fn sut(
        #[builder(default = 1)] arg1: u32,
        #[builder(default = 2)] arg2: u32,
        #[builder(default = [arg1, arg2, 3])] arg3: [u32; 3],
    ) -> impl fmt::Debug {
        (arg1, arg2, arg3)
    }

    assert_debug_eq(sut().call(), expect!["(1, 2, [1, 2, 3])"]);

    // Make sure overriding the default works
    assert_debug_eq(sut().arg1(4).call(), expect!["(4, 2, [4, 2, 3])"]);
    assert_debug_eq(sut().arg2(5).call(), expect!["(1, 5, [1, 5, 3])"]);
    assert_debug_eq(sut().arg3([6, 7, 8]).call(), expect!["(1, 2, [6, 7, 8])"]);
}

#[test]
fn struct_init_order() {
    #[derive(Debug, Builder)]
    #[allow(dead_code)]
    struct Sut {
        #[builder(skip = 1)]
        arg1: u32,

        #[builder(default = 2)]
        arg2: u32,

        #[builder(skip = [arg1, arg2, 3])]
        arg3: [u32; 3],
    }

    assert_debug_eq(
        Sut::builder().build(),
        expect!["Sut { arg1: 1, arg2: 2, arg3: [1, 2, 3] }"],
    );

    // Make sure overriding the default works
    assert_debug_eq(
        Sut::builder().arg2(4).build(),
        expect!["Sut { arg1: 1, arg2: 4, arg3: [1, 4, 3] }"],
    );

    #[bon]
    #[allow(clippy::items_after_statements)]
    impl Sut {
        #[builder]
        fn sut(
            #[builder(default = 1)] arg1: u32,
            #[builder(default = 2)] arg2: u32,
            #[builder(default = [arg1, arg2, 3])] arg3: [u32; 3],
        ) -> (u32, u32, [u32; 3]) {
            (arg1, arg2, arg3)
        }
    }

    assert_debug_eq(Sut::sut().call(), expect!["(1, 2, [1, 2, 3])"]);

    // Make sure overriding the default works
    assert_debug_eq(Sut::sut().arg1(4).call(), expect!["(4, 2, [4, 2, 3])"]);
    assert_debug_eq(Sut::sut().arg2(5).call(), expect!["(1, 5, [1, 5, 3])"]);
    assert_debug_eq(
        Sut::sut().arg3([6, 7, 8]).call(),
        expect!["(1, 2, [6, 7, 8])"],
    );
}
