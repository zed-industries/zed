use crate::prelude::*;
use core::fmt;

#[test]
fn test_struct() {
    #[derive(Debug, Builder)]
    #[builder(derive(Clone))]
    struct Sut<T> {
        #[builder(required, with = Some)]
        _required: Option<i32>,

        #[builder(required, with = Some, default = Some(()))]
        _optional: Option<()>,

        #[builder(required, with = Some)]
        _generic: Option<T>,

        #[builder(required, with = Some, default = None)]
        _optional_generic: Option<T>,
    }

    let builder = Sut::builder();

    let builder = builder.required(99);

    let _ignore = builder.clone().optional(());
    let builder = builder.maybe_optional(None);

    let builder = builder.generic(2);

    let _ignore = builder.clone().optional_generic(21);
    let builder = builder.maybe_optional_generic(Some(22));

    let sut = builder.build();

    assert_debug_eq(
        &sut,
        expect![[r#"
        Sut {
            _required: Some(
                99,
            ),
            _optional: Some(
                (),
            ),
            _generic: Some(
                2,
            ),
            _optional_generic: Some(
                22,
            ),
        }"#]],
    );
}

#[test]
fn test_function() {
    {
        #[builder(derive(Clone))]
        fn sut<T: fmt::Debug>(
            #[builder(required, with = Some)] required: Option<i32>,
            #[builder(required, with = Some, default = Some(()))] optional: Option<()>,
            #[builder(required, with = Some)] generic: Option<T>,
            #[builder(required, with = Some, default = None)] optional_generic: Option<T>,
        ) -> impl fmt::Debug {
            (required, optional, generic, optional_generic)
        }

        let builder = sut();

        let builder = builder.required(99);

        let _ignore = builder.clone().optional(());
        let builder = builder.maybe_optional(None);

        let builder = builder.generic(2);

        let _ignore = builder.clone().optional_generic(21);
        let builder = builder.maybe_optional_generic(Some(22));

        let sut = builder.call();

        assert_debug_eq(&sut, expect!["(Some(99), Some(()), Some(2), Some(22))"]);
    }
    {
        #[builder]
        fn sut(
            #[builder(required, with = Some)] impl_trait: Option<impl fmt::Debug>,
        ) -> impl fmt::Debug {
            impl_trait
        }

        assert_debug_eq(
            sut().impl_trait("impl Trait").call(),
            expect![[r#"Some("impl Trait")"#]],
        );
    }
}

#[test]
fn test_method() {
    struct Sut;

    #[bon]
    impl Sut {
        #[builder(derive(Clone))]
        fn sut<T: fmt::Debug>(
            #[builder(required, with = Some)] required: Option<i32>,
            #[builder(required, with = Some, default = Some(()))] optional: Option<()>,
            #[builder(required, with = Some)] generic: Option<T>,
            #[builder(required, with = Some, default = None)] optional_generic: Option<T>,
        ) -> impl fmt::Debug {
            (required, optional, generic, optional_generic)
        }

        #[builder(derive(Clone))]
        fn with_self<T: fmt::Debug>(
            &self,
            #[builder(required, with = Some)] required: Option<i32>,
            #[builder(required, with = Some, default = Some(()))] optional: Option<()>,
            #[builder(required, with = Some)] generic: Option<T>,
            #[builder(required, with = Some, default = None)] optional_generic: Option<T>,
        ) -> impl fmt::Debug {
            let _ = self;
            (required, optional, generic, optional_generic)
        }

        #[builder]
        fn sut_impl_trait(
            #[builder(required, with = Some)] impl_trait: Option<impl fmt::Debug>,
        ) -> impl fmt::Debug {
            impl_trait
        }

        #[builder]
        fn with_self_impl_trait(
            &self,
            #[builder(required, with = Some)] impl_trait: Option<impl fmt::Debug>,
        ) -> impl fmt::Debug {
            let _ = self;
            impl_trait
        }
    }

    let builder = Sut::sut();

    let builder = builder.required(99);

    let _ignore = builder.clone().optional(());
    let builder = builder.maybe_optional(None);

    let builder = builder.generic(2);

    let _ignore = builder.clone().optional_generic(21);
    let builder = builder.maybe_optional_generic(Some(22));

    let sut = builder.call();

    assert_debug_eq(&sut, expect!["(Some(99), Some(()), Some(2), Some(22))"]);

    let builder = Sut.with_self();

    let builder = builder.required(99);

    let _ignore = builder.clone().optional(());
    let builder = builder.maybe_optional(None);

    let builder = builder.generic(2);

    let _ignore = builder.clone().optional_generic(21);
    let builder = builder.maybe_optional_generic(Some(22));

    let sut = builder.call();

    assert_debug_eq(&sut, expect!["(Some(99), Some(()), Some(2), Some(22))"]);

    assert_debug_eq(
        Sut::sut_impl_trait().impl_trait("impl Trait").call(),
        expect![[r#"Some("impl Trait")"#]],
    );
    assert_debug_eq(
        Sut.with_self_impl_trait().impl_trait("impl Trait").call(),
        expect![[r#"Some("impl Trait")"#]],
    );
}
