#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)] // some code is tested for type checking only

#[cfg(not(feature = "std"))]
extern crate alloc;

mod structs {
    mod structural {
        #[cfg(not(feature = "std"))]
        use ::alloc::{boxed::Box, vec::Vec};
        use derive_more::{Eq, __private::AssertParamIsEq};

        #[test]
        fn unit() {
            #[derive(Eq, PartialEq)]
            struct Baz;

            let _: AssertParamIsEq<Baz>;
        }

        #[test]
        fn empty_tuple() {
            #[derive(Eq, PartialEq)]
            struct Foo();

            let _: AssertParamIsEq<Foo>;
        }

        #[test]
        fn empty_struct() {
            #[derive(Eq, PartialEq)]
            struct Bar {}

            let _: AssertParamIsEq<Bar>;
        }

        #[test]
        fn multi_field_tuple() {
            #[derive(Eq, PartialEq)]
            struct Foo(bool, i32);

            let _: AssertParamIsEq<Foo>;
        }

        #[test]
        fn multi_field_struct() {
            #[derive(Eq, PartialEq)]
            struct Bar {
                b: bool,
                i: i32,
            }

            let _: AssertParamIsEq<Bar>;
        }

        #[test]
        fn recursive_tuple() {
            #[derive(Eq, PartialEq)]
            struct Foo(Box<Self>, Vec<Foo>);

            let _: AssertParamIsEq<Foo>;
        }

        #[test]
        fn recursive_struct() {
            #[derive(Eq, PartialEq)]
            struct Bar {
                b: Box<Self>,
                i: Vec<Bar>,
            }

            let _: AssertParamIsEq<Bar>;
        }

        mod skip {
            use derive_more::{Eq, PartialEq, __private::AssertParamIsEq};

            struct NoEq;

            #[test]
            fn fields() {
                #[derive(Eq, PartialEq)]
                struct Foo(#[eq(skip)] NoEq, bool, #[eq(skip)] i32);

                #[derive(Eq, PartialEq)]
                struct Bar {
                    #[partial_eq(skip)]
                    a: NoEq,
                    i: i32,
                    #[partial_eq(skip)]
                    b: bool,
                }

                let _: AssertParamIsEq<Foo>;
                let _: AssertParamIsEq<Bar>;
            }

            #[test]
            fn all_fields() {
                #[derive(Eq, PartialEq)]
                struct Foo(#[eq(skip)] NoEq, #[eq(skip)] i32);

                #[derive(Eq, PartialEq)]
                struct Bar {
                    #[partial_eq(skip)]
                    a: NoEq,
                    #[partial_eq(skip)]
                    b: bool,
                }

                let _: AssertParamIsEq<Foo>;
                let _: AssertParamIsEq<Bar>;
            }

            #[test]
            fn empty_struct() {
                #[derive(Eq, PartialEq)]
                #[eq(skip)]
                struct Foo;

                #[derive(Eq, PartialEq)]
                #[partial_eq(skip)]
                struct Bar {}

                let _: AssertParamIsEq<Foo>;
                let _: AssertParamIsEq<Bar>;
            }

            #[test]
            fn multifield_struct() {
                #[derive(Eq, PartialEq)]
                #[eq(skip)]
                struct Foo(NoEq, bool);

                #[derive(Eq, PartialEq)]
                #[partial_eq(skip)]
                struct Bar {
                    a: NoEq,
                    b: bool,
                }

                let _: AssertParamIsEq<Foo>;
                let _: AssertParamIsEq<Bar>;
            }

            #[test]
            fn mixed() {
                #[derive(Eq, PartialEq)]
                #[eq(skip)]
                struct Foo(NoEq, #[eq(skip)] bool);

                #[derive(Eq, PartialEq)]
                #[partial_eq(skip)]
                struct Bar {
                    a: NoEq,
                    #[partial_eq(skip)]
                    b: bool,
                }

                let _: AssertParamIsEq<Foo>;
                let _: AssertParamIsEq<Bar>;
            }
        }

        mod generic {
            #[cfg(not(feature = "std"))]
            use ::alloc::{boxed::Box, vec::Vec};
            use derive_more::{Eq, PartialEq, __private::AssertParamIsEq};

            trait Some {
                type Assoc;
            }

            impl<T> Some for T {
                type Assoc = bool;
            }

            #[test]
            fn multi_field_tuple() {
                #[derive(Eq, PartialEq)]
                struct Foo<A: Some, B>(A::Assoc, B);

                let _: AssertParamIsEq<Foo<f32, ()>>;
            }

            #[test]
            fn multi_field_struct() {
                #[derive(Eq, PartialEq)]
                struct Bar<A, B: Some> {
                    b: B::Assoc,
                    i: A,
                }

                let _: AssertParamIsEq<Bar<u8, f32>>;
            }

            #[test]
            fn lifetime() {
                #[derive(Eq, PartialEq)]
                struct Foo<'a>(&'a str, i32);

                #[derive(Eq, PartialEq)]
                struct Bar<'a> {
                    b: Foo<'a>,
                    i: i32,
                }

                let _: AssertParamIsEq<Foo>;
                let _: AssertParamIsEq<Bar>;
            }

            #[test]
            fn const_param() {
                #[derive(Eq, PartialEq)]
                struct Baz<const N: usize>;

                #[derive(Eq, PartialEq)]
                struct Foo<const N: usize>([i32; N], i8);

                #[derive(Eq, PartialEq)]
                struct Bar<const N: usize> {
                    b: Foo<N>,
                    i: Baz<N>,
                }

                let _: AssertParamIsEq<Baz<1>>;
                let _: AssertParamIsEq<Foo<2>>;
                let _: AssertParamIsEq<Bar<3>>;
            }

            #[test]
            fn mixed() {
                #[derive(Eq, PartialEq)]
                struct Foo<'a, T, const N: usize>([&'a T; N]);

                let _: AssertParamIsEq<Foo<i32, 1>>;
            }

            #[test]
            fn recursive() {
                #[derive(Eq, PartialEq)]
                struct Foo<A: Some, B>(A::Assoc, B, Vec<Foo<A, B>>, Box<Self>);

                let _: AssertParamIsEq<Foo<f32, ()>>;
            }

            mod skip {
                use derive_more::{Eq, PartialEq, __private::AssertParamIsEq};

                struct NoEq;

                #[test]
                fn fields() {
                    #[derive(Eq, PartialEq)]
                    struct Foo<A, B, C>(#[eq(skip)] A, B, #[eq(skip)] C);

                    #[derive(Eq, PartialEq)]
                    struct Bar<A, B, C> {
                        #[partial_eq(skip)]
                        a: A,
                        i: B,
                        #[partial_eq(skip)]
                        b: C,
                    }

                    let _: AssertParamIsEq<Foo<NoEq, bool, i32>>;
                    let _: AssertParamIsEq<Bar<NoEq, i32, bool>>;
                }

                #[test]
                fn all_fields() {
                    #[derive(Eq, PartialEq)]
                    struct Foo<A, B>(#[eq(skip)] A, #[eq(skip)] B);

                    #[derive(Eq, PartialEq)]
                    struct Bar<A, B> {
                        #[partial_eq(skip)]
                        a: A,
                        #[partial_eq(skip)]
                        b: B,
                    }

                    let _: AssertParamIsEq<Foo<NoEq, i32>>;
                    let _: AssertParamIsEq<Bar<NoEq, bool>>;
                }

                #[test]
                fn multifield_struct() {
                    #[derive(Eq, PartialEq)]
                    #[eq(skip)]
                    struct Foo<A, B>(A, B);

                    #[derive(Eq, PartialEq)]
                    #[partial_eq(skip)]
                    struct Bar<A, B> {
                        a: A,
                        b: B,
                    }

                    let _: AssertParamIsEq<Foo<NoEq, NoEq>>;
                    let _: AssertParamIsEq<Bar<NoEq, NoEq>>;
                }

                #[test]
                fn mixed() {
                    #[derive(Eq, PartialEq)]
                    #[eq(skip)]
                    struct Foo<A, B>(A, #[eq(skip)] B);

                    #[derive(Eq, PartialEq)]
                    #[partial_eq(skip)]
                    struct Bar<A, B> {
                        a: A,
                        #[partial_eq(skip)]
                        b: B,
                    }

                    let _: AssertParamIsEq<Foo<NoEq, NoEq>>;
                    let _: AssertParamIsEq<Bar<NoEq, NoEq>>;
                }
            }
        }
    }
}

mod enums {
    mod structural {
        #[cfg(not(feature = "std"))]
        use ::alloc::{boxed::Box, vec::Vec};
        use derive_more::{Eq, __private::AssertParamIsEq};

        #[test]
        fn empty() {
            #[derive(Eq, PartialEq)]
            enum E {}

            let _: AssertParamIsEq<E>;
        }

        #[test]
        fn single_variant_unit() {
            #[derive(Eq, PartialEq)]
            enum E {
                Baz,
            }

            let _: AssertParamIsEq<E>;
        }

        #[test]
        fn single_variant_empty_tuple() {
            #[derive(Eq, PartialEq)]
            enum E {
                Foo(),
            }

            let _: AssertParamIsEq<E>;
        }

        #[test]
        fn single_variant_empty_struct() {
            #[derive(Eq, PartialEq)]
            enum E {
                Bar {},
            }

            let _: AssertParamIsEq<E>;
        }

        #[test]
        fn single_variant_multi_field_tuple() {
            #[derive(Eq, PartialEq)]
            enum E {
                Foo(bool, i32),
            }

            let _: AssertParamIsEq<E>;
        }

        #[test]
        fn single_variant_multi_field_struct() {
            #[derive(Eq, PartialEq)]
            enum E {
                Bar { b: bool, i: i32 },
            }

            let _: AssertParamIsEq<E>;
        }

        #[test]
        fn multi_variant_empty_field() {
            #[derive(Eq, PartialEq)]
            enum E {
                Foo(),
                Bar {},
                Baz,
            }

            let _: AssertParamIsEq<E>;
        }

        #[test]
        fn multi_variant_multi_field() {
            #[derive(Eq, PartialEq)]
            enum E {
                Foo(bool, i32),
                Bar { b: bool, i: i32 },
            }

            let _: AssertParamIsEq<E>;
        }

        #[test]
        fn multi_variant_empty_and_multi_field() {
            #[derive(Eq, PartialEq)]
            enum E {
                Foo(bool, i32),
                Baz,
            }

            let _: AssertParamIsEq<E>;
        }

        #[test]
        fn recursive() {
            #[derive(Eq, PartialEq)]
            enum E {
                Foo(Box<E>),
                Bar { b: Vec<Self> },
                Baz,
            }

            let _: AssertParamIsEq<E>;
        }

        mod skip {
            use derive_more::{Eq, PartialEq, __private::AssertParamIsEq};

            struct NoEq;

            #[test]
            fn fields() {
                #[derive(Eq, PartialEq)]
                enum E {
                    Foo(#[eq(skip)] NoEq, bool, #[eq(skip)] i32),
                    Bar {
                        #[partial_eq(skip)]
                        a: NoEq,
                        i: i32,
                        #[partial_eq(skip)]
                        b: bool,
                    },
                }

                let _: AssertParamIsEq<E>;
            }

            #[test]
            fn all_fields() {
                #[derive(Eq, PartialEq)]
                enum E {
                    Foo(#[partial_eq(skip)] NoEq, #[partial_eq(skip)] i32),
                    Bar {
                        #[partial_eq(skip)]
                        a: NoEq,
                        #[partial_eq(skip)]
                        b: bool,
                    },
                }

                let _: AssertParamIsEq<E>;
            }

            #[test]
            fn variants() {
                #[derive(Eq, PartialEq)]
                enum E {
                    Foo(bool, i32),
                    #[eq(skip)]
                    Bar {
                        a: NoEq,
                        b: bool,
                    },
                }

                let _: AssertParamIsEq<E>;
            }

            #[test]
            fn all_variants() {
                #[derive(Eq, PartialEq)]
                enum E {
                    #[eq(skip)]
                    Foo(bool, i32),
                    #[partial_eq(skip)]
                    Bar { a: NoEq, b: bool },
                    #[eq(skip)]
                    Baz,
                }

                let _: AssertParamIsEq<E>;
            }

            #[test]
            fn single_variant() {
                #[derive(Eq, PartialEq)]
                enum E {
                    #[eq(skip)]
                    Bar { a: NoEq, b: bool },
                }

                let _: AssertParamIsEq<E>;
            }

            #[test]
            fn all_but_single_empty_variant() {
                #[derive(Eq, PartialEq)]
                enum E {
                    #[eq(skip)]
                    Foo(bool, i32),
                    #[partial_eq(skip)]
                    Bar {
                        a: NoEq,
                        b: bool,
                    },
                    Baz,
                }

                let _: AssertParamIsEq<E>;
            }

            #[test]
            fn mixed() {
                #[derive(Eq, PartialEq)]
                enum E {
                    #[eq(skip)]
                    Foo(bool, i32),
                    Bar {
                        #[partial_eq(skip)]
                        a: NoEq,
                        b: bool,
                    },
                    Baz,
                }

                let _: AssertParamIsEq<E>;
            }
        }

        mod generic {
            #[cfg(not(feature = "std"))]
            use ::alloc::{boxed::Box, vec::Vec};
            use derive_more::{Eq, PartialEq, __private::AssertParamIsEq};

            trait Some {
                type Assoc;
            }

            impl<T> Some for T {
                type Assoc = bool;
            }

            #[test]
            fn single_variant_multi_field_tuple() {
                #[derive(Eq, PartialEq)]
                enum E<A: Some, B> {
                    Foo(A::Assoc, B),
                }

                let _: AssertParamIsEq<E<f32, ()>>;
            }

            #[test]
            fn single_variant_multi_field_struct() {
                #[derive(Eq, PartialEq)]
                enum E<A, B: Some> {
                    Bar { b: B::Assoc, i: A },
                }

                let _: AssertParamIsEq<E<&'static str, f64>>;
            }

            #[test]
            fn multi_variant_empty_and_multi_field() {
                #[derive(Eq, PartialEq)]
                enum E<A, B: Some> {
                    Foo(B::Assoc, A),
                    Bar { b: B::Assoc, i: A },
                    Baz,
                }

                let _: AssertParamIsEq<E<i64, f64>>;
            }

            #[test]
            fn lifetime() {
                #[derive(Eq, PartialEq)]
                enum E1<'a> {
                    Foo(&'a str, i32),
                }

                #[derive(Eq, PartialEq)]
                enum E2<'a> {
                    Bar { b: E1<'a>, i: i32 },
                }

                let _: AssertParamIsEq<E1>;
                let _: AssertParamIsEq<E2>;
            }

            #[test]
            fn const_param() {
                #[derive(Eq, PartialEq)]
                enum E3<const N: usize> {
                    Baz,
                }

                #[derive(Eq, PartialEq)]
                enum E1<const N: usize> {
                    Foo([i32; N], i8),
                }

                #[derive(Eq, PartialEq)]
                enum E2<const N: usize> {
                    Bar { b: E1<N>, i: E3<N> },
                }

                let _: AssertParamIsEq<E3<1>>;
                let _: AssertParamIsEq<E1<2>>;
                let _: AssertParamIsEq<E2<0>>;
            }

            #[test]
            fn mixed() {
                #[derive(Eq, PartialEq)]
                enum E<'a, A, B: Some, const N: usize> {
                    Foo([&'a A; N]),
                    Baz([B::Assoc; N]),
                }

                let _: AssertParamIsEq<E<i32, f64, 1>>;
            }

            #[test]
            fn recursive() {
                #[derive(Eq, PartialEq)]
                enum E<A, B: Some> {
                    Foo(B::Assoc, Vec<Self>),
                    Bar { b: Box<E<A, B>>, i: A },
                    Baz,
                }

                let _: AssertParamIsEq<E<i64, f64>>;
            }

            mod skip {
                use derive_more::{Eq, PartialEq, __private::AssertParamIsEq};

                struct NoEq;

                #[test]
                fn fields() {
                    #[derive(Eq, PartialEq)]
                    enum E<A, B, C> {
                        Foo(#[partial_eq(skip)] A, B, #[partial_eq(skip)] C),
                        Bar {
                            #[partial_eq(skip)]
                            a: A,
                            i: C,
                            #[partial_eq(skip)]
                            b: B,
                        },
                    }

                    let _: AssertParamIsEq<E<NoEq, bool, i32>>;
                }

                #[test]
                fn all_fields() {
                    #[derive(Eq, PartialEq)]
                    enum E<A, B, C> {
                        Foo(#[partial_eq(skip)] A, #[partial_eq(skip)] B),
                        Bar {
                            #[partial_eq(skip)]
                            a: A,
                            #[partial_eq(skip)]
                            b: C,
                        },
                    }

                    let _: AssertParamIsEq<E<NoEq, bool, i32>>;
                }

                #[test]
                fn variants() {
                    #[derive(Eq, PartialEq)]
                    enum E<A, B, C> {
                        Foo(A, B),
                        #[eq(skip)]
                        Bar {
                            a: C,
                            b: A,
                        },
                    }

                    let _: AssertParamIsEq<E<i32, bool, NoEq>>;
                }

                #[test]
                fn all_variants() {
                    #[derive(Eq, PartialEq)]
                    enum E<A, B, C> {
                        #[eq(skip)]
                        Foo(A, B),
                        #[partial_eq(skip)]
                        Bar { a: C, b: A },
                        #[eq(skip)]
                        Baz,
                    }

                    let _: AssertParamIsEq<E<NoEq, NoEq, f32>>;
                }

                #[test]
                fn single_variant() {
                    #[derive(Eq, PartialEq)]
                    enum E<A, B> {
                        #[eq(skip)]
                        Bar { a: A, b: B },
                    }

                    let _: AssertParamIsEq<E<NoEq, NoEq>>;
                }

                #[test]
                fn all_but_single_empty_variant() {
                    #[derive(Eq, PartialEq)]
                    enum E<A, B, C> {
                        #[eq(skip)]
                        Foo(A, B),
                        #[partial_eq(skip)]
                        Bar {
                            a: C,
                            b: A,
                        },
                        Baz,
                    }

                    let _: AssertParamIsEq<E<NoEq, NoEq, f32>>;
                }

                #[test]
                fn mixed() {
                    #[derive(Eq, PartialEq)]
                    enum E<A, B, C> {
                        #[eq(skip)]
                        Foo(A, B),
                        Bar {
                            #[partial_eq(skip)]
                            a: C,
                            b: A,
                        },
                        Baz,
                    }

                    let _: AssertParamIsEq<E<i32, NoEq, f32>>;
                }
            }
        }
    }
}
