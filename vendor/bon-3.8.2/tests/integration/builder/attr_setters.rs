mod name {
    use crate::prelude::*;

    #[test]
    fn test_struct() {
        #[derive(Builder)]
        #[allow(dead_code)]
        struct Sut {
            #[builder(setters(name = arg1_renamed))]
            arg1: bool,

            #[builder(setters(name = arg2_renamed))]
            arg2: Option<()>,

            #[builder(setters(name = arg3_renamed), default)]
            arg3: u32,
        }

        use sut_builder::*;

        #[allow(type_alias_bounds)]
        type _AssocTypes<T: State> = (T::Arg1, T::Arg2, T::Arg3);

        let _ = Sut::builder().arg1_renamed(true);

        let _ = Sut::builder().arg2_renamed(());
        let _ = Sut::builder().maybe_arg2_renamed(Some(()));

        let _ = Sut::builder().arg3_renamed(42);
        let _ = Sut::builder().maybe_arg3_renamed(Some(42));

        // The name in the state must remain the same
        let _: SutBuilder<SetArg3<SetArg2<SetArg1>>> = Sut::builder()
            .arg1_renamed(true)
            .arg2_renamed(())
            .arg3_renamed(42);
    }

    #[test]
    fn test_function() {
        #[builder]
        fn sut(
            #[builder(setters(name = arg1_renamed))] _arg1: bool,
            #[builder(setters(name = arg2_renamed))] _arg2: Option<()>,
            #[builder(setters(name = arg3_renamed), default)] _arg3: u32,
        ) {
        }

        use sut_builder::*;

        #[allow(type_alias_bounds)]
        type _AssocTypes<T: State> = (T::Arg1, T::Arg2, T::Arg3);

        let _ = sut().arg1_renamed(true);

        let _ = sut().arg2_renamed(());
        let _ = sut().maybe_arg2_renamed(Some(()));

        let _ = sut().arg3_renamed(42);
        let _ = sut().maybe_arg3_renamed(Some(42));

        // The name in the state must remain the same
        let _: SutBuilder<SetArg3<SetArg2<SetArg1>>> =
            sut().arg1_renamed(true).arg2_renamed(()).arg3_renamed(42);
    }

    #[test]
    fn test_method() {
        struct Sut;

        #[bon]
        impl Sut {
            #[builder]
            fn sut(
                #[builder(setters(name = arg1_renamed))] _arg1: bool,
                #[builder(setters(name = arg2_renamed))] _arg2: Option<()>,
                #[builder(setters(name = arg3_renamed), default)] _arg3: u32,
            ) {
            }

            #[builder]
            fn with_self(
                &self,
                #[builder(setters(name = arg1_renamed))] _arg1: bool,
                #[builder(setters(name = arg2_renamed))] _arg2: Option<()>,
                #[builder(setters(name = arg3_renamed), default)] _arg3: u32,
            ) {
                let _ = self;
            }
        }

        {
            use sut_sut_builder::*;

            #[allow(type_alias_bounds)]
            type _AssocTypes<T: State> = (T::Arg1, T::Arg2, T::Arg3);

            let _ = Sut::sut().arg1_renamed(true);

            let _ = Sut::sut().arg2_renamed(());
            let _ = Sut::sut().maybe_arg2_renamed(Some(()));

            let _ = Sut::sut().arg3_renamed(42);
            let _ = Sut::sut().maybe_arg3_renamed(Some(42));

            // The name in the state must remain the same
            let _: SutSutBuilder<SetArg3<SetArg2<SetArg1>>> = Sut::sut()
                .arg1_renamed(true)
                .arg2_renamed(())
                .arg3_renamed(42);
        }

        {
            use sut_with_self_builder::*;

            #[allow(type_alias_bounds)]
            type _AssocTypes<T: State> = (T::Arg1, T::Arg2, T::Arg3);

            let sut = Sut;

            let _ = sut.with_self().arg1_renamed(true);

            let _ = sut.with_self().arg2_renamed(());
            let _ = sut.with_self().maybe_arg2_renamed(Some(()));

            let _ = sut.with_self().arg3_renamed(42);
            let _ = sut.with_self().maybe_arg3_renamed(Some(42));

            // The name in the state must remain the same
            let _: SutWithSelfBuilder<'_, SetArg3<SetArg2<SetArg1>>> = sut
                .with_self()
                .arg1_renamed(true)
                .arg2_renamed(())
                .arg3_renamed(42);
        }
    }
}

mod option_fn_name_and_some_fn_name {
    use crate::prelude::*;

    #[test]
    fn test_struct() {
        #[derive(Builder)]
        #[builder(derive(Clone))]
        #[allow(dead_code)]
        struct Sut {
            #[builder(setters(some_fn = arg1_some))]
            arg1: Option<()>,

            #[builder(setters(option_fn = arg2_option))]
            arg2: Option<()>,

            #[builder(setters(some_fn = arg3_some, option_fn = arg3_option))]
            arg3: Option<()>,

            #[builder(setters(some_fn(name = arg4_some), option_fn(name = arg4_option)))]
            arg4: Option<()>,

            #[builder(default, setters(some_fn = arg5_some))]
            arg5: (),

            #[builder(default, setters(option_fn = arg6_option))]
            arg6: (),

            #[builder(default, setters(some_fn = arg7_some, option_fn = arg7_option))]
            arg7: (),

            #[builder(default, setters(some_fn(name = arg8_some), option_fn(name = arg8_option)))]
            arg8: (),
        }

        use sut_builder::*;

        let _ = Sut::builder().arg1_some(());
        let _ = Sut::builder().maybe_arg1(Some(()));

        let _ = Sut::builder().arg2(());
        let _ = Sut::builder().arg2_option(Some(()));

        let _ = Sut::builder().arg3_some(());
        let _ = Sut::builder().arg3_option(Some(()));

        let _ = Sut::builder().arg4_some(());
        let _ = Sut::builder().arg4_option(Some(()));

        let _ = Sut::builder().arg5_some(());
        let _ = Sut::builder().maybe_arg5(Some(()));

        let _ = Sut::builder().arg6(());
        let _ = Sut::builder().arg6_option(Some(()));

        let _ = Sut::builder().arg7_some(());
        let _ = Sut::builder().arg7_option(Some(()));

        let _ = Sut::builder().arg8_some(());
        let _ = Sut::builder().arg8_option(Some(()));

        #[allow(clippy::type_complexity)]
        let _: SutBuilder<
            SetArg8<SetArg7<SetArg6<SetArg5<SetArg4<SetArg3<SetArg2<SetArg1>>>>>>>,
        > = Sut::builder()
            .arg1_some(())
            .arg2(())
            .arg3_some(())
            .arg4_some(())
            .arg5_some(())
            .arg6(())
            .arg7_some(())
            .arg8_some(());
    }

    #[test]
    fn test_function() {
        #[builder(derive(Clone))]
        fn sut(
            #[builder(setters(some_fn = arg1_some))] _arg1: Option<()>,
            #[builder(setters(option_fn = arg2_option))] _arg2: Option<()>,
            #[builder(setters(some_fn = arg3_some, option_fn = arg3_option))] _arg3: Option<()>,
            #[builder(setters(some_fn(name = arg4_some), option_fn(name = arg4_option)))]
            _arg4: Option<()>,

            #[builder(default, setters(some_fn = arg5_some))] _arg5: (),
            #[builder(default, setters(option_fn = arg6_option))] _arg6: (),
            #[builder(default, setters(some_fn = arg7_some, option_fn = arg7_option))] _arg7: (),
            #[builder(default, setters(some_fn(name = arg8_some), option_fn(name = arg8_option)))]
            _arg8: (),
        ) {
        }

        use sut_builder::*;

        let _ = sut().arg1_some(());
        let _ = sut().maybe_arg1(Some(()));

        let _ = sut().arg2(());
        let _ = sut().arg2_option(Some(()));

        let _ = sut().arg3_some(());
        let _ = sut().arg3_option(Some(()));

        let _ = sut().arg4_some(());
        let _ = sut().arg4_option(Some(()));

        let _ = sut().arg5_some(());
        let _ = sut().maybe_arg5(Some(()));

        let _ = sut().arg6(());
        let _ = sut().arg6_option(Some(()));

        let _ = sut().arg7_some(());
        let _ = sut().arg7_option(Some(()));

        let _ = sut().arg8_some(());
        let _ = sut().arg8_option(Some(()));

        #[allow(clippy::type_complexity)]
        let _: SutBuilder<
            SetArg8<SetArg7<SetArg6<SetArg5<SetArg4<SetArg3<SetArg2<SetArg1>>>>>>>,
        > = sut()
            .arg1_some(())
            .arg2(())
            .arg3_some(())
            .arg4_some(())
            .arg5_some(())
            .arg6(())
            .arg7_some(())
            .arg8_some(());
    }

    #[test]
    fn test_method() {
        struct Sut;

        #[bon]
        impl Sut {
            #[builder(derive(Clone))]
            fn sut(
                #[builder(setters(some_fn = arg1_some))] _arg1: Option<()>,
                #[builder(setters(option_fn = arg2_option))] _arg2: Option<()>,
                #[builder(setters(some_fn = arg3_some, option_fn = arg3_option))] _arg3: Option<()>,
                #[builder(setters(some_fn(name = arg4_some), option_fn(name = arg4_option)))]
               _arg4: Option<()>,

                #[builder(default, setters(some_fn = arg5_some))] _arg5: (),
                #[builder(default, setters(option_fn = arg6_option))] _arg6: (),
                #[builder(default, setters(some_fn = arg7_some, option_fn = arg7_option))] _arg7: (
                ),
                #[builder(default, setters(some_fn(name = arg8_some), option_fn(name = arg8_option)))]
                _arg8: (),
            ) {
            }

            #[builder(derive(Clone))]
            fn with_self(
                &self,
                #[builder(setters(some_fn = arg1_some))] _arg1: Option<()>,
                #[builder(setters(option_fn = arg2_option))] _arg2: Option<()>,
                #[builder(setters(some_fn = arg3_some, option_fn = arg3_option))] _arg3: Option<()>,
                #[builder(setters(some_fn(name = arg4_some), option_fn(name = arg4_option)))]
                _arg4: Option<()>,

                #[builder(default, setters(some_fn = arg5_some))] _arg5: (),
                #[builder(default, setters(option_fn = arg6_option))] _arg6: (),
                #[builder(default, setters(some_fn = arg7_some, option_fn = arg7_option))] _arg7: (
                ),
                #[builder(default, setters(some_fn(name = arg8_some), option_fn(name = arg8_option)))]
                _arg8: (),
            ) {
                let _ = self;
            }
        }

        {
            use sut_sut_builder::*;

            let _ = Sut::sut().arg1_some(());
            let _ = Sut::sut().maybe_arg1(Some(()));

            let _ = Sut::sut().arg2(());
            let _ = Sut::sut().arg2_option(Some(()));

            let _ = Sut::sut().arg3_some(());
            let _ = Sut::sut().arg3_option(Some(()));

            let _ = Sut::sut().arg4_some(());
            let _ = Sut::sut().arg4_option(Some(()));

            let _ = Sut::sut().arg5_some(());
            let _ = Sut::sut().maybe_arg5(Some(()));

            let _ = Sut::sut().arg6(());
            let _ = Sut::sut().arg6_option(Some(()));

            let _ = Sut::sut().arg7_some(());
            let _ = Sut::sut().arg7_option(Some(()));

            let _ = Sut::sut().arg8_some(());
            let _ = Sut::sut().arg8_option(Some(()));

            #[allow(clippy::type_complexity)]
            let _: SutSutBuilder<
                SetArg8<SetArg7<SetArg6<SetArg5<SetArg4<SetArg3<SetArg2<SetArg1>>>>>>>,
            > = Sut::sut()
                .arg1_some(())
                .arg2(())
                .arg3_some(())
                .arg4_some(())
                .arg5_some(())
                .arg6(())
                .arg7_some(())
                .arg8_some(());
        }

        {
            use sut_with_self_builder::*;

            let _ = Sut.with_self().arg1_some(());
            let _ = Sut.with_self().maybe_arg1(Some(()));

            let _ = Sut.with_self().arg2(());
            let _ = Sut.with_self().arg2_option(Some(()));

            let _ = Sut.with_self().arg3_some(());
            let _ = Sut.with_self().arg3_option(Some(()));

            let _ = Sut.with_self().arg4_some(());
            let _ = Sut.with_self().arg4_option(Some(()));

            let _ = Sut.with_self().arg5_some(());
            let _ = Sut.with_self().maybe_arg5(Some(()));

            let _ = Sut.with_self().arg6(());
            let _ = Sut.with_self().arg6_option(Some(()));

            let _ = Sut.with_self().arg7_some(());
            let _ = Sut.with_self().arg7_option(Some(()));

            let _ = Sut.with_self().arg8_some(());
            let _ = Sut.with_self().arg8_option(Some(()));

            #[allow(clippy::type_complexity)]
            let _: SutWithSelfBuilder<
                '_,
                SetArg8<SetArg7<SetArg6<SetArg5<SetArg4<SetArg3<SetArg2<SetArg1>>>>>>>,
            > = Sut
                .with_self()
                .arg1_some(())
                .arg2(())
                .arg3_some(())
                .arg4_some(())
                .arg5_some(())
                .arg6(())
                .arg7_some(())
                .arg8_some(());
        }
    }
}

mod self_references_in_docs {
    use crate::prelude::*;

    #[test]
    fn test_struct() {
        /// [`Self`] link
        #[derive(Builder)]
        struct Sut {
            /// [`Self`] link
            #[builder(setters(doc {}))]
            _arg1: u32,

            /// [`Self`] link
            #[builder(setters(
                option_fn(doc {}),
                some_fn(doc {})
            ))]
            _arg2: Option<u32>,
        }

        let _ = Sut::builder().arg1(42);
    }

    #[test]
    fn test_function() {
        /// [`Self`] link
        #[builder]
        fn sut(
            /// [`Self`] link
            #[builder(setters(doc {}))]
            _arg1: u32,
        ) {
        }

        let _ = sut().arg1(42);
    }
}
