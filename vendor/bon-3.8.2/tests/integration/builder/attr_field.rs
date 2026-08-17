use crate::prelude::*;

#[test]
fn test_struct() {
    #[derive(Builder)]
    #[builder(derive(Debug, Clone))]
    #[allow(dead_code)]
    struct Sut<T> {
        #[builder(start_fn)]
        x1: u32,

        #[builder(start_fn, into)]
        x2: u32,

        #[builder(field)]
        x3: bool,

        #[builder(field = x1 + x2 + u32::from(x3))]
        x4: u32,

        #[builder(field = x4 * 2)]
        x5: u32,

        #[builder(field)]
        x6: Option<T>,

        #[builder(field = core::any::type_name::<T>())]
        x7: &'static str,

        x8: (),
    }

    // Make sure the builder is cloneable
    #[allow(clippy::redundant_clone)]
    let sut = Sut::<()>::builder(1, 2_u32).clone();

    assert_debug_eq(
        &sut,
        expect![[r#"
            SutBuilder {
                x1: 1,
                x2: 2,
                x3: false,
                x4: 3,
                x5: 6,
                x6: None,
                x7: "()",
            }"#]],
    );

    let _ = sut.x8(()).build();
}

// This tests uses a turbofish with `impl Trait`
#[rustversion::since(1.63.0)]
#[test]
fn test_function() {
    use core::fmt;

    #[builder(derive(Debug, Clone))]
    #[allow(unused_variables)]
    fn sut<T: fmt::Debug>(
        #[builder(start_fn)] x1: u32,
        #[builder(start_fn, into)] x2: u32,
        #[builder(field)] x3: bool,
        #[builder(field = x1 + x2 + u32::from(x3))] x4: u32,
        #[builder(field = x4 * 2)] x5: u32,
        #[builder(field)] x6: Option<T>,
        #[builder(field = core::any::type_name::<T>())] x7: &'static str,
        x8: (),
    ) {
    }

    // Make sure the builder is cloneable
    #[allow(clippy::redundant_clone)]
    let sut = sut::<()>(1, 2_u32).clone();

    assert_debug_eq(
        &sut,
        expect![[r#"
            SutBuilder {
                x1: 1,
                x2: 2,
                x3: false,
                x4: 3,
                x5: 6,
                x6: None,
                x7: "()",
            }"#]],
    );

    sut.x8(()).call();
}

// This tests uses a turbofish with `impl Trait`
#[rustversion::since(1.63.0)]
#[test]
fn test_method() {
    use core::fmt;

    #[derive(Debug)]
    struct Sut;

    #[bon]
    impl Sut {
        #[builder(derive(Debug, Clone))]
        #[allow(unused_variables)]
        fn sut<T: fmt::Debug>(
            #[builder(start_fn)] x1: u32,
            #[builder(start_fn, into)] x2: u32,
            #[builder(field)] x3: bool,
            #[builder(field = x1 + x2 + u32::from(x3))] x4: u32,
            #[builder(field = x4 * 2)] x5: u32,
            #[builder(field)] x6: Option<T>,
            #[builder(field = core::any::type_name::<T>())] x7: &'static str,
            x8: (),
        ) {
        }

        #[builder(derive(Debug, Clone))]
        #[allow(unused_variables)]
        fn with_self<T: fmt::Debug>(
            &self,
            #[builder(start_fn)] x1: u32,
            #[builder(start_fn, into)] x2: u32,
            #[builder(field)] x3: bool,
            #[builder(field = x1 + x2 + u32::from(x3))] x4: u32,
            #[builder(field = x4 * 2)] x5: u32,
            #[builder(field)] x6: Option<T>,
            #[builder(field = core::any::type_name::<T>())] x7: &'static str,
            x8: (),
        ) {
            let _ = self;
        }
    }

    // Make sure the builder is cloneable
    #[allow(clippy::redundant_clone)]
    let sut = Sut::sut::<()>(1, 2_u32).clone();

    assert_debug_eq(
        &sut,
        expect![[r#"
            SutSutBuilder {
                x1: 1,
                x2: 2,
                x3: false,
                x4: 3,
                x5: 6,
                x6: None,
                x7: "()",
            }"#]],
    );

    sut.x8(()).call();

    // Make sure the builder is cloneable
    #[allow(clippy::redundant_clone)]
    let sut = Sut.with_self::<()>(1, 2_u32).clone();

    assert_debug_eq(
        &sut,
        expect![[r#"
            SutWithSelfBuilder {
                self_receiver: Sut,
                x1: 1,
                x2: 2,
                x3: false,
                x4: 3,
                x5: 6,
                x6: None,
                x7: "()",
            }"#]],
    );

    sut.x8(()).call();
}
