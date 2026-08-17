mod conflicts_in_bodies {
    use crate::prelude::*;

    #[test]
    #[allow(clippy::items_after_statements)]
    fn test_struct() {
        #[derive(Builder, Clone, Copy)]
        #[allow(dead_code)]
        struct S {
            field: u32,
        }

        let s = S::builder().field(1).build();

        #[derive(Builder, Clone, Copy)]
        #[allow(dead_code)]
        struct State {
            field: S,
        }

        let state = State::builder().field(s).build();

        #[derive(Builder, Clone, Copy)]
        #[allow(dead_code)]
        struct BuilderState {
            field1: S,
            field2: State,
        }

        let builder_state = BuilderState::builder().field1(s).field2(state).build();

        #[derive(Builder, Clone, Copy)]
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        struct S_ {
            field1: S,
            field2: State,
            field3: BuilderState,
        }

        let s_ = S_::builder()
            .field1(s)
            .field2(state)
            .field3(builder_state)
            .build();

        #[derive(Builder, Clone, Copy)]
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        struct S__ {
            field1: S,
            field2: State,
            field3: BuilderState,
            field4: S_,
        }

        let _ = S__::builder()
            .field1(s)
            .field2(state)
            .field3(builder_state)
            .field4(s_)
            .build();
    }

    #[test]
    #[allow(clippy::items_after_statements)]
    fn test_function() {
        struct S;
        struct State;
        struct BuilderState;

        {
            #[builder]
            fn sut(_field: S) {}

            sut().field(S).call();
        }

        {
            #[builder]
            fn sut(_field: S) {
                let _ = State;
            }

            sut().field(S).call();
        }

        {
            #[builder]
            fn sut(_field1: S, _field2: State) {
                let _ = {
                    {
                        ((), BuilderState)
                    }
                };
            }

            sut().field1(S).field2(State).call();
        }

        {
            #[builder]
            fn sut<S_>(_field1: S, _field2: State, _field3: BuilderState) {}

            sut::<()>()
                .field1(S)
                .field2(State)
                .field3(BuilderState)
                .call();
        }

        {
            #[builder]
            fn sut<S_, S__>(_field1: S, _field2: State, _field3: BuilderState) {}

            sut::<(), ()>()
                .field1(S)
                .field2(State)
                .field3(BuilderState)
                .call();
        }
    }

    #[test]
    #[allow(clippy::items_after_statements)]
    fn test_method() {
        struct State;
        struct BuilderState;

        {
            struct S;

            #[bon]
            impl S {
                #[builder]
                fn sut() {}
            }

            S::sut().call();
        }

        {
            struct S;

            #[bon]
            impl S {
                #[builder]
                fn sut() {
                    let _ = State;
                }

                #[builder]
                fn with_self(&self) {
                    let _ = self;
                }
            }

            S::sut().call();
            S.with_self().call();
        }

        {
            struct S;

            #[bon]
            impl S {
                #[builder]
                fn sut(_field2: State) {}

                #[builder]
                fn with_self(&self) {
                    let _ = self;
                    let _ = {
                        {
                            ((), BuilderState)
                        }
                    };
                }
            }

            S::sut().field2(State).call();
            S.with_self().call();
        }

        {
            struct S;

            #[bon]
            impl S {
                #[builder]
                fn sut<S_>(_field2: State, _field3: BuilderState) {}
            }

            S::sut::<()>().field2(State).field3(BuilderState).call();
        }

        {
            struct S;

            #[bon]
            impl S {
                #[builder]
                fn sut<S_, S__>(_field2: State, _field3: BuilderState) {}
            }

            S::sut::<(), ()>().field2(State).field3(BuilderState).call();
        }
    }
}

mod conflicts_in_attrs {
    use crate::prelude::*;

    struct S;

    impl S {
        fn s(&self) -> u32 {
            let _ = self;
            2
        }
    }

    struct State;

    impl State {
        fn state(&self) -> u32 {
            let _ = self;
            2
        }
    }

    struct BuilderState;

    impl BuilderState {
        fn builder_state(&self) -> u32 {
            let _ = self;
            2
        }
    }

    #[test]
    fn test_struct() {
        {
            #[derive(Builder)]
            #[allow(dead_code)]
            struct Sut {
                #[builder(with = |s: S| s.s())]
                field: u32,
            }

            let _ = Sut::builder().field(S).build();
        }
        {
            #[derive(Builder)]
            #[allow(dead_code)]
            struct Sut {
                #[builder(with = |s: S| s.s())]
                field1: u32,

                #[builder(default = State.state())]
                field2: u32,
            }

            let _ = Sut::builder().field1(S).maybe_field2(Some(43)).build();
        }
        {
            #[derive(Builder)]
            #[allow(dead_code)]
            struct Sut {
                #[builder(with = |s: S| s.s())]
                field1: u32,

                #[builder(default = State.state())]
                field2: u32,

                #[builder(skip = BuilderState.builder_state())]
                field3: u32,
            }

            let _ = Sut::builder().field1(S).maybe_field2(Some(43)).build();
        }
    }

    #[test]
    fn test_function() {
        {
            #[builder]
            fn sut(#[builder(with = |s: S| s.s())] _field: u32) {}

            sut().field(S).call();
        }
        {
            #[builder]
            fn sut(
                #[builder(with = |s: S| s.s())] _field1: u32,

                #[builder(default = State.state())] _field2: u32,
            ) {
            }

            sut().field1(S).maybe_field2(Some(43)).call();
        }
    }

    #[test]
    fn test_method() {
        {
            struct Sut;

            #[bon]
            impl Sut {
                #[builder]
                fn sut(#[builder(with = |s: S| s.s())] _field: u32) {}
            }

            Sut::sut().field(S).call();
        }
        {
            struct Sut;

            #[bon]
            impl Sut {
                #[builder]
                fn sut(
                    #[builder(with = |s: S| s.s())] _field1: u32,

                    #[builder(default = State.state())] _field2: u32,
                ) {
                }
            }

            Sut::sut().field1(S).maybe_field2(Some(43)).call();
        }
    }
}
