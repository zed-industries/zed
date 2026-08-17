use crate::prelude::*;
use core::marker::PhantomData;

#[test]
fn generic_struct() {
    #[derive(Debug, Builder)]
    #[allow(dead_code)]
    struct Sut<'a, 'b, T, U> {
        a: &'a str,
        b: &'b str,
        c: T,
        d: U,
        e: [u8; 3],
    }

    let actual = Sut::builder().a("a").b("b").c(42).d("d").e([0; 3]).build();

    assert_debug_eq(
        actual,
        expect![[r#"Sut { a: "a", b: "b", c: 42, d: "d", e: [0, 0, 0] }"#]],
    );
}

#[test]
fn return_type_only_generic_param() {
    #[builder]
    fn sut<T: Default>() -> T {
        T::default()
    }

    let _: i32 = sut().call();
}

#[test]
fn unsized_generics_in_params() {
    #[builder]
    fn sut<T: ?Sized>(arg: &T) {
        let _ = arg;
    }

    sut().arg(&42).call();
}

#[test]
fn unsized_generics_in_return_type() {
    #[builder]
    fn sut<T: ?Sized>() -> PhantomData<T> {
        PhantomData
    }

    sut::<u32>().call();
}

// This is based on the issue https://github.com/rust-lang/rust/issues/129701
#[test]
fn assoc_types_work_in_params() {
    trait Trait {
        type Assoc;
    }

    #[builder]
    fn sut<T: Trait>(_val: T::Assoc) {}

    impl Trait for () {
        type Assoc = ();
    }

    sut::<()>().val(()).call();
}

// This is based on the issue https://github.com/rust-lang/rust/issues/129701
#[test]
fn assoc_types_work_in_return_type() {
    trait Trait {
        type Assoc;
    }

    #[builder]
    fn sut<T: Trait>() -> T::Assoc
    where
        T::Assoc: Default,
    {
        T::Assoc::default()
    }

    impl Trait for () {
        type Assoc = ();
    }

    sut::<()>().call();
}

// This is based on the issue https://github.com/elastio/bon/issues/16
#[test]
fn self_only_generic_param() {
    struct Sut<'a, 'b: 'a, T> {
        bar: Option<T>,
        str: &'a str,
        other_ref: &'b (),
    }

    #[bon]
    impl<T> Sut<'_, '_, T> {
        #[builder]
        fn new() -> Self {
            Self {
                bar: None,
                str: "littlepip",
                other_ref: &(),
            }
        }
    }

    // Make sure `new` method is hidden
    Sut::<core::convert::Infallible>::__orig_new();

    // Make sure the builder type name matches the type of builder when
    // `#[builder]` is placed on top of a struct
    let _: SutBuilder<'_, '_, core::convert::Infallible> = Sut::builder();

    let actual = Sut::<core::convert::Infallible>::builder().build();

    assert!(actual.bar.is_none());
    assert_eq!(actual.str, "littlepip");
    let _: &() = actual.other_ref;
}

#[test]
fn impl_block_with_self_in_const_generics() {
    #[derive(Default)]
    struct Sut<const N: usize>;

    impl<const N: usize> Sut<N> {
        const fn val(&self) -> usize {
            let _ = self;
            42
        }
    }

    #[bon]
    impl Sut<{ Sut::<3>.val() }>
    where
        Self:,
    {
        #[builder]
        fn method(self) -> usize {
            self.val()
        }
    }

    assert_eq!(Sut::<42>.method().call(), 42);
}

#[test]
fn generics_with_lifetimes() {
    #[builder]
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn sut<T>(arg: &&&&&T) {
        let _ = arg;
    }

    sut().arg(&&&&&&&&&&42).call();
}

// This is based on the issue https://github.com/elastio/bon/issues/106
#[test]
fn default_generic_type_params() {
    #[derive(bon::Builder)]
    struct Sut<T = u32, U = ()> {
        #[builder(skip)]
        _phantom: ::core::marker::PhantomData<(T, U)>,
    }

    let builder: SutBuilder = Sut::builder();
    let _: Sut = builder.build();
}

#[test]
fn const_generics() {
    #[derive(Debug, Builder)]
    #[allow(dead_code)]
    struct Sut<'a, T, const N: usize> {
        a: &'a str,
        b: T,
        c: [u8; N],
    }

    let actual = Sut::builder().a("a").b(42).c([0; 3]).build();

    assert_debug_eq(actual, expect![[r#"Sut { a: "a", b: 42, c: [0, 0, 0] }"#]]);
}

#[test]
fn default_generic_const_params() {
    #[derive(bon::Builder)]
    struct Sut<const N: usize = 99, const K: usize = 0> {}

    let builder: SutBuilder = Sut::builder();
    let _: Sut = builder.build();
}

#[test]
fn lifetimes_with_bounds() {
    #[builder]
    fn sut<'a, 'b: 'a, T: 'a, U: 'b>(arg: &'a T, arg2: &'b U) {
        let _ = arg;
        let _ = arg2;
    }

    sut().arg(&42).arg2(&42).call();
}

// This is based on the issue https://github.com/elastio/bon/issues/206
mod lifetimes_used_only_in_type_predicates {
    use crate::prelude::*;

    trait Trait<'a> {}

    impl Trait<'_> for u32 {}

    #[test]
    fn function() {
        #[builder]
        fn sut<'a, T: Trait<'a>>(x1: T) -> T {
            x1
        }

        assert_eq!(sut().x1(2).call(), 2);
    }

    #[test]
    fn generic_method() {
        struct Sut;

        #[bon]
        impl Sut {
            #[builder]
            fn method<'a, T: Trait<'a>>(x1: T) -> T {
                x1
            }

            #[builder]
            fn with_self<'a, T: Trait<'a>>(&self, x1: T) -> T {
                let _ = self;
                x1
            }
        }

        assert_eq!(Sut::method().x1(2).call(), 2);
        assert_eq!(Sut.with_self().x1(2).call(), 2);
    }

    #[test]
    fn generic_impl() {
        struct Sut<T>(T);

        #[bon]
        impl<'a, T: Trait<'a>> Sut<T> {
            #[builder]
            fn method(x1: T) -> T {
                x1
            }

            #[builder]
            fn with_self(&self, x1: T) -> T {
                let _ = self;
                x1
            }
        }

        assert_eq!(Sut::<u32>::method().x1(2).call(), 2);
        assert_eq!(Sut(1).with_self().x1(2).call(), 2);
    }
}

mod unused_lifetimes {
    use crate::prelude::*;

    #[test]
    fn function() {
        #[builder]
        #[allow(clippy::extra_unused_lifetimes, unused_lifetimes)]
        fn sut<'a, 'b>() {}

        sut().call();
    }

    #[test]
    fn generic_method() {
        struct Sut;

        #[bon]
        impl Sut {
            #[builder]
            #[allow(clippy::extra_unused_lifetimes, unused_lifetimes)]
            fn method<'a, 'b>() {}

            #[builder]
            #[allow(clippy::extra_unused_lifetimes, unused_lifetimes)]
            fn with_self<'a, 'b>(&self) {
                let _ = self;
            }
        }

        Sut::method().call();
        Sut.with_self().call();
    }

    #[test]
    fn generic_impl() {
        struct Sut<T>(T);

        // Interestingly, this code doesn't produce an `unused_lifetimes` warning
        // because the generated starting functions are inserted into this impl
        // block and they do use the lifetimes by storing them in the builder.
        // There isn't a good way to deal with this problem.
        //
        // We could generate the starting functions in a separate impl block, but
        // then it would break the lexical order of methods as they are declared
        // in this impl block in regards to how they are displayed in `rustdoc`.
        //
        // Also, rustdoc permits documentation on the impl block itself, so if
        // we create a separate impl block for the starting functions, that
        // would be rendered as separate impl blocks in `rustdoc` as well and we
        // would need to do something about the docs on the original impl block,
        // (e.g. copy them to the impl block for starting functions?).
        //
        // Anyway, the problem of unused lifetimes lint false-negative is probably
        // not that serious to justify the complexity of the solution to fix it.
        #[bon]
        impl<'a, 'b, T> Sut<T> {
            #[builder]
            fn method() {}

            #[builder]
            fn with_self(&self) {
                let _ = self;
            }
        }

        Sut::<u32>::method().call();
        Sut(1).with_self().call();
    }
}
