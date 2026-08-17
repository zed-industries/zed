// We intentionally exercise cloning from `#[builder(derive(Clone))]` here
// to make sure that it works.
#![allow(clippy::redundant_clone)]

use crate::prelude::*;

#[test]
fn smoke_fn() {
    #[builder(derive(Clone, Debug, Into))]
    fn sut(_arg1: bool, _arg2: Option<()>, _arg3: Option<&str>, _arg4: Option<u32>) -> u32 {
        99
    }

    let actual = sut().arg1(true).arg3("value").maybe_arg4(None).clone();

    assert_debug_eq(
        &actual,
        expect![[r#"SutBuilder { arg1: true, arg3: "value" }"#]],
    );

    let actual: u32 = From::from(actual);

    assert_debug_eq(actual, expect!["99"]);
}

#[test]
fn smoke_struct() {
    #[derive(Builder, Debug)]
    #[builder(derive(Clone, Debug, Into))]
    struct Sut<'a> {
        _arg1: bool,
        _arg2: Option<()>,
        _arg3: Option<&'a str>,
        _arg4: Option<u32>,
    }

    let actual = Sut::builder()
        .arg1(true)
        .arg3("value")
        .maybe_arg4(None)
        .clone();

    assert_debug_eq(
        &actual,
        expect![[r#"SutBuilder { arg1: true, arg3: "value" }"#]],
    );

    let actual: Sut<'_> = From::from(actual);

    assert_debug_eq(
        actual,
        expect![[r#"
        Sut {
            _arg1: true,
            _arg2: None,
            _arg3: Some(
                "value",
            ),
            _arg4: None,
        }"#]],
    );
}

#[test]
fn builder_with_receiver() {
    #[derive(Clone, Debug)]
    struct Sut {
        #[allow(dead_code)]
        name: &'static str,
    }

    #[bon]
    impl Sut {
        #[builder(derive(Clone, Debug, Into))]
        fn method(&self, other_name: &'static str, values: &[u8]) -> u32 {
            let _ = (self, other_name, values);
            99
        }
    }

    let actual = Sut { name: "Blackjack" }
        .method()
        .other_name("P21")
        .values(&[1, 2, 3])
        .clone();

    assert_debug_eq(
        &actual,
        expect![[r#"
            SutMethodBuilder {
                self_receiver: Sut {
                    name: "Blackjack",
                },
                other_name: "P21",
                values: [
                    1,
                    2,
                    3,
                ],
            }"#]],
    );

    let actual: u32 = From::from(actual);

    assert_debug_eq(actual, expect!["99"]);
}

#[test]
fn skipped_members() {
    struct NoDebug;

    #[derive(Builder)]
    #[builder(derive(Debug, Clone, Into))]
    struct Sut {
        arg1: bool,

        #[builder(skip = NoDebug)]
        _arg2: NoDebug,
    }

    let actual = Sut::builder().arg1(true).clone();

    assert_debug_eq(&actual, expect!["SutBuilder { arg1: true }"]);

    let actual: Sut = From::from(actual);

    assert!(actual.arg1);
}

#[test]
fn empty_builder() {
    #[derive(Builder)]
    #[builder(derive(Clone, Debug, Into))]
    struct Sut {}

    #[allow(clippy::redundant_clone)]
    let actual = Sut::builder().clone();

    assert_debug_eq(&actual, expect!["SutBuilder"]);

    let Sut {} = From::from(actual);
}

mod generics {
    use crate::prelude::*;

    #[test]
    fn test_struct() {
        #[derive(Builder, Debug)]
        #[builder(derive(Clone, Debug, Into))]
        struct Sut<T>
        where
            T: Clone,
        {
            _arg1: T,
        }

        let actual = Sut::builder().arg1(42).clone();

        assert_debug_eq(&actual, expect!["SutBuilder { arg1: 42 }"]);

        let actual: Sut<_> = From::from(actual);

        assert_debug_eq(actual, expect![[r#"Sut { _arg1: 42 }"#]]);
    }

    #[test]
    fn test_function() {
        #[builder(derive(Clone, Debug, Into))]
        fn sut<T>(_arg1: T) -> u32
        where
            T: Clone,
        {
            99
        }

        let actual = sut::<u32>().arg1(42).clone();

        assert_debug_eq(&actual, expect!["SutBuilder { arg1: 42 }"]);

        let actual: u32 = From::from(actual);

        assert_debug_eq(actual, expect!["99"]);
    }

    #[test]
    fn test_method() {
        #[derive(Clone, Debug)]
        struct Sut<T>(T);

        #[bon]
        impl<T> Sut<T>
        where
            T: Clone,
        {
            #[builder(derive(Clone, Debug, Into))]
            fn sut<U>(_arg1: U) -> u32 {
                99
            }

            #[builder(derive(Clone, Debug, Into))]
            fn with_self<U>(&self, _arg1: U) -> u32 {
                let _ = self;
                99
            }
        }

        let actual = Sut::<()>::sut::<u32>().arg1(42).clone();

        assert_debug_eq(&actual, expect!["SutSutBuilder { arg1: 42 }"]);

        let actual: u32 = From::from(actual);

        assert_debug_eq(actual, expect!["99"]);

        let actual = Sut(true).with_self::<u32>().arg1(42).clone();

        assert_debug_eq(
            &actual,
            expect!["SutWithSelfBuilder { self_receiver: Sut(true), arg1: 42 }"],
        );

        let actual: u32 = From::from(actual);

        assert_debug_eq(actual, expect!["99"]);
    }
}

mod positional_members {
    use crate::prelude::*;

    #[test]
    fn test_struct() {
        #[derive(Builder)]
        #[builder(derive(Clone, Debug))]
        #[allow(dead_code)]
        struct Sut {
            #[builder(start_fn)]
            start_fn_arg: bool,

            #[builder(finish_fn)]
            finish_fn_arg: &'static str,

            named: u32,
        }

        let actual = Sut::builder(true);

        assert_debug_eq(actual.clone(), expect!["SutBuilder { start_fn_arg: true }"]);

        assert_debug_eq(
            actual.named(42).clone(),
            expect!["SutBuilder { start_fn_arg: true, named: 42 }"],
        );
    }

    #[test]
    fn test_function() {
        #[builder(derive(Clone, Debug))]
        #[allow(unused_variables)]
        fn sut(
            #[builder(start_fn)] start_fn_arg: bool,
            #[builder(finish_fn)] finish_fn_arg: &'static str,
            named: u32,
        ) {
        }

        let actual = sut(true);

        assert_debug_eq(actual.clone(), expect!["SutBuilder { start_fn_arg: true }"]);

        assert_debug_eq(
            actual.named(42).clone(),
            expect!["SutBuilder { start_fn_arg: true, named: 42 }"],
        );
    }

    #[test]
    fn test_method() {
        #[derive(Debug)]
        struct Sut;

        #[bon]
        #[allow(unused_variables)]
        impl Sut {
            #[builder(derive(Clone, Debug))]
            fn sut(
                #[builder(start_fn)] start_fn_arg: bool,
                #[builder(finish_fn)] finish_fn_arg: &'static str,
                named: u32,
            ) {
            }

            #[builder(derive(Clone, Debug))]
            fn with_self(
                &self,
                #[builder(start_fn)] start_fn_arg: bool,
                #[builder(finish_fn)] finish_fn_arg: &'static str,
                named: u32,
            ) {
                let _ = self;
            }
        }

        let actual = Sut::sut(true);

        assert_debug_eq(
            actual.clone(),
            expect!["SutSutBuilder { start_fn_arg: true }"],
        );
        assert_debug_eq(
            actual.named(42).clone(),
            expect!["SutSutBuilder { start_fn_arg: true, named: 42 }"],
        );

        let actual = Sut.with_self(true);

        assert_debug_eq(
            actual.clone(),
            expect![[r#"
                SutWithSelfBuilder {
                    self_receiver: Sut,
                    start_fn_arg: true,
                }"#]],
        );
        assert_debug_eq(
            actual.named(42).clone(),
            expect![[r#"
                SutWithSelfBuilder {
                    self_receiver: Sut,
                    start_fn_arg: true,
                    named: 42,
                }"#]],
        );
    }
}

mod attr_bounds_empty {
    use crate::prelude::*;

    struct NoTraitImpls;

    #[test]
    fn test_struct() {
        #[derive(Builder)]
        #[builder(derive(Clone(bounds()), Debug))]
        struct Sut<'a, T> {
            _arg: &'a T,
        }

        let _ = Sut::builder().arg(&NoTraitImpls).clone();
    }

    #[test]
    fn test_function() {
        #[builder(derive(Clone(bounds()), Debug))]
        fn sut<T>(_arg: &T) {}

        let _ = sut::<NoTraitImpls>().arg(&NoTraitImpls).clone();
    }

    #[test]
    fn test_method() {
        #[derive(Clone, Debug)]
        struct Sut;

        #[bon]
        impl Sut {
            #[builder(derive(Clone(bounds()), Debug))]
            fn sut<T>(_arg: &T) {}
        }

        let _ = Sut::sut::<NoTraitImpls>().arg(&NoTraitImpls).clone();
    }
}

mod attr_bounds_non_empty {
    use crate::prelude::*;

    struct NoTraitImpls;

    #[test]
    fn test_struct() {
        #[derive(Builder)]
        #[builder(derive(Clone(bounds(&'a T: Clone, &'a &'a T: Clone)), Debug))]
        struct Sut<'a, T> {
            _arg: &'a T,
        }

        let _ = Sut::builder().arg(&NoTraitImpls).clone();
    }

    #[test]
    fn test_function() {
        #[builder(derive(Clone(bounds(&'a T: Clone, &'a &'a T: Clone)), Debug))]
        #[allow(clippy::needless_lifetimes, single_use_lifetimes)]
        fn sut<'a, T>(_arg: &'a T) {}

        let _ = sut::<NoTraitImpls>().arg(&NoTraitImpls).clone();
    }

    #[test]
    fn test_method() {
        #[derive(Clone, Debug)]
        struct Sut;

        #[bon]
        impl Sut {
            #[builder(derive(Clone(bounds(&'a T: Clone, &'a &'a T: Clone)), Debug))]
            #[allow(clippy::needless_lifetimes, single_use_lifetimes)]
            fn sut<'a, T>(_arg: &'a T) {}
        }

        let _ = Sut::sut::<NoTraitImpls>().arg(&NoTraitImpls).clone();
    }
}
