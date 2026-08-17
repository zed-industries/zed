#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)] // some code is tested for type checking only

#[cfg(not(feature = "std"))]
extern crate alloc;

/// Since [`assert_ne!()`] macro in [`core`] doesn't use `$left != $right` comparison, but rather
/// checks as `!($left == $right)`, it should be redefined for tests to consider actual
/// [`PartialEq::ne()`] implementations.
///
/// [`assert_ne!()`]: core::assert_ne
#[macro_export]
macro_rules! assert_ne {
    ($left:expr, $right:expr $(,)?) => {
        assert!($left != $right)
    };
}

mod structs {
    mod structural {
        #[cfg(not(feature = "std"))]
        use ::alloc::{boxed::Box, vec, vec::Vec};
        use derive_more::PartialEq;

        #[test]
        fn unit() {
            #[derive(Debug, PartialEq)]
            struct Baz;

            assert_eq!(Baz, Baz);
        }

        #[test]
        fn empty_tuple() {
            #[derive(Debug, PartialEq)]
            struct Foo();

            assert_eq!(Foo(), Foo());
        }

        #[test]
        fn empty_struct() {
            #[derive(Debug, PartialEq)]
            struct Bar {}

            assert_eq!(Bar {}, Bar {});
        }

        #[test]
        fn multi_field_tuple() {
            #[derive(Debug, PartialEq)]
            struct Foo(bool, i32);

            assert_eq!(Foo(true, 0), Foo(true, 0));
            assert_ne!(Foo(true, 0), Foo(false, 0));
            assert_ne!(Foo(true, 0), Foo(true, 1));
            assert_ne!(Foo(true, 0), Foo(false, 1));
        }

        #[test]
        fn multi_field_struct() {
            #[derive(Debug, PartialEq)]
            struct Bar {
                b: bool,
                i: i32,
            }

            assert_eq!(Bar { b: true, i: 0 }, Bar { b: true, i: 0 });
            assert_ne!(Bar { b: true, i: 0 }, Bar { b: false, i: 0 });
            assert_ne!(Bar { b: true, i: 0 }, Bar { b: true, i: 1 });
            assert_ne!(Bar { b: true, i: 0 }, Bar { b: false, i: 1 });
        }

        #[test]
        fn recursive_tuple() {
            #[derive(Debug, PartialEq)]
            struct Foo(Option<Box<Self>>, Vec<Foo>);

            assert_eq!(Foo(None, vec![]), Foo(None, vec![]));
            assert_ne!(Foo(None, vec![]), Foo(None, vec![Foo(None, vec![])]));
            assert_ne!(
                Foo(None, vec![]),
                Foo(Some(Box::new(Foo(None, vec![]))), vec![])
            );
        }

        #[test]
        fn recursive_struct() {
            #[derive(Debug, PartialEq)]
            struct Bar {
                b: Option<Box<Self>>,
                i: Vec<Bar>,
            }

            assert_eq!(Bar { b: None, i: vec![] }, Bar { b: None, i: vec![] });
            assert_ne!(
                Bar { b: None, i: vec![] },
                Bar {
                    b: None,
                    i: vec![Bar { b: None, i: vec![] }],
                },
            );
            assert_ne!(
                Bar { b: None, i: vec![] },
                Bar {
                    b: Some(Box::new(Bar { b: None, i: vec![] })),
                    i: vec![],
                },
            );
        }

        mod skip {
            use derive_more::PartialEq;

            #[derive(Debug)]
            struct NoEq;

            #[test]
            fn fields() {
                #[derive(Debug, PartialEq)]
                struct Foo(#[partial_eq(skip)] NoEq, bool, #[partial_eq(skip)] i32);

                #[derive(Debug, PartialEq)]
                struct Bar {
                    #[partial_eq(skip)]
                    a: NoEq,
                    i: i32,
                    #[partial_eq(skip)]
                    b: bool,
                }

                assert_eq!(Foo(NoEq, true, 0), Foo(NoEq, true, 0));
                assert_eq!(Foo(NoEq, true, 0), Foo(NoEq, true, 1));
                assert_ne!(Foo(NoEq, true, 0), Foo(NoEq, false, 0));

                assert_eq!(
                    Bar {
                        a: NoEq,
                        i: 0,
                        b: true,
                    },
                    Bar {
                        a: NoEq,
                        i: 0,
                        b: true,
                    },
                );
                assert_eq!(
                    Bar {
                        a: NoEq,
                        i: 0,
                        b: true,
                    },
                    Bar {
                        a: NoEq,
                        i: 0,
                        b: false,
                    },
                );
                assert_ne!(
                    Bar {
                        a: NoEq,
                        i: 0,
                        b: true,
                    },
                    Bar {
                        a: NoEq,
                        i: 1,
                        b: true,
                    },
                );
            }

            #[test]
            fn all_fields() {
                #[derive(Debug, PartialEq)]
                struct Foo(#[partial_eq(skip)] NoEq, #[partial_eq(skip)] i32);

                #[derive(Debug, PartialEq)]
                struct Bar {
                    #[partial_eq(skip)]
                    a: NoEq,
                    #[partial_eq(skip)]
                    b: bool,
                }

                assert_eq!(Foo(NoEq, 0), Foo(NoEq, 0));
                assert_eq!(Foo(NoEq, 0), Foo(NoEq, 1));

                assert_eq!(Bar { a: NoEq, b: true }, Bar { a: NoEq, b: true });
                assert_eq!(Bar { a: NoEq, b: true }, Bar { a: NoEq, b: false });
            }

            #[test]
            fn empty_struct() {
                #[derive(Debug, PartialEq)]
                #[partial_eq(skip)]
                struct Foo;

                #[derive(Debug, PartialEq)]
                struct Bar {}

                assert_eq!(Foo, Foo);

                assert_eq!(Bar {}, Bar {});
            }

            #[test]
            fn multifield_struct() {
                #[derive(Debug, PartialEq)]
                #[partial_eq(skip)]
                struct Foo(NoEq, bool);

                #[derive(Debug, PartialEq)]
                #[partial_eq(skip)]
                struct Bar {
                    a: NoEq,
                    b: bool,
                }

                assert_eq!(Foo(NoEq, true), Foo(NoEq, true));
                assert_eq!(Foo(NoEq, true), Foo(NoEq, false));

                assert_eq!(Bar { a: NoEq, b: true }, Bar { a: NoEq, b: true });
                assert_eq!(Bar { a: NoEq, b: true }, Bar { a: NoEq, b: false });
            }

            #[test]
            fn mixed() {
                #[derive(Debug, derive_more::PartialEq)]
                #[partial_eq(skip)]
                struct Foo(NoEq, #[partial_eq(skip)] bool);

                #[derive(Debug, PartialEq)]
                #[partial_eq(skip)]
                struct Bar {
                    a: NoEq,

                    #[partial_eq(skip)]
                    b: bool,
                }

                assert_eq!(Foo(NoEq, true), Foo(NoEq, true));
                assert_eq!(Foo(NoEq, true), Foo(NoEq, false));

                assert_eq!(Bar { a: NoEq, b: true }, Bar { a: NoEq, b: true });
                assert_eq!(Bar { a: NoEq, b: true }, Bar { a: NoEq, b: false });
            }
        }

        mod generic {
            #[cfg(not(feature = "std"))]
            use ::alloc::{boxed::Box, vec, vec::Vec};
            use derive_more::PartialEq;

            trait Some {
                type Assoc;
            }

            impl<T> Some for T {
                type Assoc = bool;
            }

            #[derive(Debug)]
            struct NoEq;

            #[test]
            fn multi_field_tuple() {
                #[derive(Debug, PartialEq)]
                struct Foo<A: Some, B>(A::Assoc, B);

                assert_eq!(Foo::<NoEq, _>(true, 0), Foo(true, 0));
                assert_ne!(Foo::<NoEq, _>(true, 0), Foo(false, 0));
                assert_ne!(Foo::<NoEq, _>(true, 0), Foo(true, 1));
                assert_ne!(Foo::<NoEq, _>(true, 0), Foo(false, 1));
            }

            #[test]
            fn multi_field_struct() {
                #[derive(Debug, PartialEq)]
                struct Bar<A, B: Some> {
                    b: B::Assoc,
                    i: A,
                }

                assert_eq!(Bar::<_, NoEq> { b: true, i: 0 }, Bar { b: true, i: 0 });
                assert_ne!(Bar::<_, NoEq> { b: true, i: 0 }, Bar { b: false, i: 0 });
                assert_ne!(Bar::<_, NoEq> { b: true, i: 0 }, Bar { b: true, i: 1 });
                assert_ne!(Bar::<_, NoEq> { b: true, i: 0 }, Bar { b: false, i: 1 });
            }

            #[test]
            fn lifetime() {
                #[derive(Debug, PartialEq)]
                struct Foo<'a>(&'a str, i32);

                #[derive(Debug, PartialEq)]
                struct Bar<'a> {
                    b: Foo<'a>,
                    i: i32,
                }

                assert_eq!(Foo("hi", 0), Foo("hi", 0));
                assert_ne!(Foo("hi", 0), Foo("bye", 0));
                assert_ne!(Foo("hi", 0), Foo("hi", 1));
                assert_ne!(Foo("hi", 0), Foo("bye", 1));

                assert_eq!(
                    Bar {
                        b: Foo("hi", 0),
                        i: 0,
                    },
                    Bar {
                        b: Foo("hi", 0),
                        i: 0,
                    },
                );
                assert_ne!(
                    Bar {
                        b: Foo("hi", 0),
                        i: 0,
                    },
                    Bar {
                        b: Foo("bye", 0),
                        i: 0,
                    },
                );
                assert_ne!(
                    Bar {
                        b: Foo("hi", 0),
                        i: 0,
                    },
                    Bar {
                        b: Foo("hi", 0),
                        i: 1,
                    },
                );
                assert_ne!(
                    Bar {
                        b: Foo("hi", 0),
                        i: 0,
                    },
                    Bar {
                        b: Foo("bye", 0),
                        i: 1,
                    },
                );
            }

            #[test]
            fn const_param() {
                #[derive(Debug, PartialEq)]
                struct Baz<const N: usize>;

                #[derive(Debug, PartialEq)]
                struct Foo<const N: usize>([i32; N], i8);

                #[derive(Debug, PartialEq)]
                struct Bar<const N: usize> {
                    b: Foo<N>,
                    i: Baz<N>,
                }

                assert_eq!(Baz::<1>, Baz);

                assert_eq!(Foo([3], 0), Foo([3], 0));
                assert_ne!(Foo([3], 0), Foo([4], 0));
                assert_ne!(Foo([3], 0), Foo([3], 1));
                assert_ne!(Foo([3], 0), Foo([4], 1));

                assert_eq!(
                    Bar {
                        b: Foo([3], 0),
                        i: Baz,
                    },
                    Bar {
                        b: Foo([3], 0),
                        i: Baz,
                    },
                );
                assert_ne!(
                    Bar {
                        b: Foo([3], 0),
                        i: Baz,
                    },
                    Bar {
                        b: Foo([3], 1),
                        i: Baz,
                    },
                );
            }

            #[test]
            fn recursive() {
                #[derive(Debug, PartialEq)]
                struct Foo<A: Some, B>(A::Assoc, B, Vec<Foo<A, B>>, Option<Box<Self>>);

                assert_eq!(
                    Foo::<NoEq, _>(true, 2, vec![], None),
                    Foo::<NoEq, _>(true, 2, vec![], None),
                );
                assert_ne!(
                    Foo::<NoEq, _>(true, 2, vec![], None),
                    Foo::<NoEq, _>(false, 2, vec![], None),
                );
                assert_ne!(
                    Foo::<NoEq, _>(true, 2, vec![], None),
                    Foo::<NoEq, _>(true, 0, vec![], None),
                );
                assert_ne!(
                    Foo::<NoEq, _>(true, 2, vec![], None),
                    Foo::<NoEq, _>(true, 2, vec![Foo(true, 2, vec![], None)], None),
                );
                assert_ne!(
                    Foo::<NoEq, _>(true, 2, vec![], None),
                    Foo::<NoEq, _>(
                        true,
                        2,
                        vec![],
                        Some(Box::new(Foo(true, 2, vec![], None)))
                    ),
                );
            }

            mod skip {
                use derive_more::PartialEq;

                #[derive(Debug)]
                struct NoEq;

                #[test]
                fn fields() {
                    #[derive(Debug, PartialEq)]
                    struct Foo<A, B, C>(
                        #[partial_eq(skip)] A,
                        B,
                        #[partial_eq(skip)] C,
                    );

                    #[derive(Debug, PartialEq)]
                    struct Bar<A, B, C> {
                        #[partial_eq(skip)]
                        a: A,
                        i: B,
                        #[partial_eq(skip)]
                        b: C,
                    }

                    assert_eq!(Foo(NoEq, true, 0), Foo(NoEq, true, 0));
                    assert_eq!(Foo(NoEq, true, 0), Foo(NoEq, true, 1));
                    assert_ne!(Foo(NoEq, true, 0), Foo(NoEq, false, 0));

                    assert_eq!(
                        Bar {
                            a: NoEq,
                            i: 0,
                            b: true,
                        },
                        Bar {
                            a: NoEq,
                            i: 0,
                            b: true,
                        },
                    );
                    assert_eq!(
                        Bar {
                            a: NoEq,
                            i: 0,
                            b: true,
                        },
                        Bar {
                            a: NoEq,
                            i: 0,
                            b: false,
                        },
                    );
                    assert_ne!(
                        Bar {
                            a: NoEq,
                            i: 0,
                            b: true,
                        },
                        Bar {
                            a: NoEq,
                            i: 1,
                            b: true,
                        },
                    );
                }

                #[test]
                fn all_fields() {
                    #[derive(Debug, PartialEq)]
                    struct Foo<A, B>(#[partial_eq(skip)] A, #[partial_eq(skip)] B);

                    #[derive(Debug, PartialEq)]
                    struct Bar<A, B> {
                        #[partial_eq(skip)]
                        a: A,
                        #[partial_eq(skip)]
                        b: B,
                    }

                    assert_eq!(Foo(NoEq, 0), Foo(NoEq, 0));
                    assert_eq!(Foo(NoEq, 0), Foo(NoEq, 1));

                    assert_eq!(Bar { a: NoEq, b: true }, Bar { a: NoEq, b: true });
                    assert_eq!(Bar { a: NoEq, b: true }, Bar { a: NoEq, b: false });
                }

                #[test]
                fn multifield_struct() {
                    #[derive(Debug, PartialEq)]
                    #[partial_eq(skip)]
                    struct Foo<A, B>(A, B);

                    #[derive(Debug, PartialEq)]
                    #[partial_eq(skip)]
                    struct Bar<A, B> {
                        a: A,
                        b: B,
                    }

                    assert_eq!(Foo(1, NoEq), Foo(1, NoEq));
                    assert_eq!(Foo(1, NoEq), Foo(0, NoEq));

                    assert_eq!(Bar { a: NoEq, b: true }, Bar { a: NoEq, b: true });
                    assert_eq!(Bar { a: NoEq, b: true }, Bar { a: NoEq, b: false });
                }

                #[test]
                fn mixed() {
                    #[derive(Debug, PartialEq)]
                    #[partial_eq(skip)]
                    struct Foo<A, B>(A, #[partial_eq(skip)] B);

                    #[derive(Debug, PartialEq)]
                    #[partial_eq(skip)]
                    struct Bar<A, B> {
                        a: A,
                        #[partial_eq(skip)]
                        b: B,
                    }

                    assert_eq!(Foo(NoEq, true), Foo(NoEq, true));
                    assert_eq!(Foo(NoEq, true), Foo(NoEq, false));

                    assert_eq!(Bar { a: NoEq, b: true }, Bar { a: NoEq, b: true });
                    assert_eq!(Bar { a: NoEq, b: true }, Bar { a: NoEq, b: false });
                }
            }
        }
    }
}

mod enums {
    mod structural {
        #[cfg(not(feature = "std"))]
        use ::alloc::{boxed::Box, vec, vec::Vec};
        use derive_more::PartialEq;

        #[test]
        fn empty() {
            #[derive(Debug, PartialEq)]
            enum E {}
        }

        #[test]
        fn single_variant_unit() {
            #[derive(Debug, PartialEq)]
            enum E {
                Baz,
            }

            assert_eq!(E::Baz, E::Baz);
        }

        #[test]
        fn single_variant_empty_tuple() {
            #[derive(Debug, PartialEq)]
            enum E {
                Foo(),
            }

            assert_eq!(E::Foo(), E::Foo());
        }

        #[test]
        fn single_variant_empty_struct() {
            #[derive(Debug, PartialEq)]
            enum E {
                Bar {},
            }

            assert_eq!(E::Bar {}, E::Bar {});
        }

        #[test]
        fn single_variant_multi_field_tuple() {
            #[derive(Debug, PartialEq)]
            enum E {
                Foo(bool, i32),
            }

            assert_eq!(E::Foo(true, 0), E::Foo(true, 0));
            assert_ne!(E::Foo(true, 0), E::Foo(false, 0));
            assert_ne!(E::Foo(true, 0), E::Foo(true, 1));
            assert_ne!(E::Foo(true, 0), E::Foo(false, 1));
        }

        #[test]
        fn single_variant_multi_field_struct() {
            #[derive(Debug, PartialEq)]
            enum E {
                Bar { b: bool, i: i32 },
            }

            assert_eq!(E::Bar { b: true, i: 0 }, E::Bar { b: true, i: 0 });
            assert_ne!(E::Bar { b: true, i: 0 }, E::Bar { b: false, i: 0 });
            assert_ne!(E::Bar { b: true, i: 0 }, E::Bar { b: true, i: 1 });
            assert_ne!(E::Bar { b: true, i: 0 }, E::Bar { b: false, i: 1 });
        }

        #[test]
        fn multi_variant_empty_field() {
            #[derive(Debug, PartialEq)]
            enum E {
                Foo(),
                Bar {},
                Baz,
            }

            assert_eq!(E::Foo(), E::Foo());

            assert_eq!(E::Bar {}, E::Bar {});

            assert_eq!(E::Baz, E::Baz);

            assert_ne!(E::Foo(), E::Bar {});
            assert_ne!(E::Bar {}, E::Foo());
            assert_ne!(E::Foo(), E::Baz);
            assert_ne!(E::Baz, E::Foo());
            assert_ne!(E::Bar {}, E::Baz);
            assert_ne!(E::Baz, E::Bar {});
        }

        #[test]
        fn multi_variant_multi_field() {
            #[derive(Debug, PartialEq)]
            enum E {
                Foo(bool, i32),
                Bar { b: bool, i: i32 },
            }

            assert_eq!(E::Foo(true, 0), E::Foo(true, 0));
            assert_ne!(E::Foo(true, 0), E::Foo(false, 0));
            assert_ne!(E::Foo(true, 0), E::Foo(true, 1));
            assert_ne!(E::Foo(true, 0), E::Foo(false, 1));

            assert_eq!(E::Bar { b: true, i: 0 }, E::Bar { b: true, i: 0 });
            assert_ne!(E::Bar { b: true, i: 0 }, E::Bar { b: false, i: 0 });
            assert_ne!(E::Bar { b: true, i: 0 }, E::Bar { b: true, i: 1 });
            assert_ne!(E::Bar { b: true, i: 0 }, E::Bar { b: false, i: 1 });

            assert_ne!(E::Foo(true, 0), E::Bar { b: true, i: 0 });
            assert_ne!(E::Bar { b: false, i: 1 }, E::Foo(false, 1));
        }

        #[test]
        fn multi_variant_empty_and_multi_field() {
            #[derive(Debug, PartialEq)]
            enum E {
                Foo(bool, i32),
                Baz,
            }

            assert_eq!(E::Foo(true, 0), E::Foo(true, 0));
            assert_ne!(E::Foo(true, 0), E::Foo(false, 0));
            assert_ne!(E::Foo(true, 0), E::Foo(true, 1));
            assert_ne!(E::Foo(true, 0), E::Foo(false, 1));

            assert_eq!(E::Baz, E::Baz);

            assert_ne!(E::Foo(true, 0), E::Baz);
            assert_ne!(E::Baz, E::Foo(false, 1));
        }

        #[test]
        fn recursive() {
            #[derive(Debug, PartialEq)]
            enum E {
                Foo(Option<Box<E>>),
                Bar { b: Vec<Self> },
                Baz,
            }

            assert_eq!(E::Foo(None), E::Foo(None));
            assert_ne!(E::Foo(None), E::Foo(Some(Box::new(E::Foo(None)))));
            assert_ne!(E::Foo(None), E::Foo(Some(Box::new(E::Baz))));

            assert_eq!(E::Bar { b: vec![] }, E::Bar { b: vec![] });
            assert_ne!(
                E::Bar { b: vec![] },
                E::Bar {
                    b: vec![E::Bar { b: vec![] }],
                },
            );
            assert_ne!(E::Bar { b: vec![] }, E::Bar { b: vec![E::Baz] });

            assert_eq!(E::Baz, E::Baz);

            assert_ne!(E::Foo(None), E::Bar { b: vec![] });
            assert_ne!(E::Foo(None), E::Baz);
            assert_ne!(E::Bar { b: vec![] }, E::Baz);
        }

        mod skip {
            use derive_more::PartialEq;

            #[derive(Debug)]
            struct NoEq;

            #[test]
            fn fields() {
                #[derive(Debug, PartialEq)]
                enum E {
                    Foo(#[partial_eq(skip)] NoEq, bool, #[partial_eq(skip)] i32),
                    Bar {
                        #[partial_eq(skip)]
                        a: NoEq,
                        i: i32,
                        #[partial_eq(skip)]
                        b: bool,
                    },
                }

                assert_eq!(E::Foo(NoEq, true, 0), E::Foo(NoEq, true, 0));
                assert_eq!(E::Foo(NoEq, true, 0), E::Foo(NoEq, true, 1));
                assert_ne!(E::Foo(NoEq, true, 0), E::Foo(NoEq, false, 0));

                assert_eq!(
                    E::Bar {
                        a: NoEq,
                        i: 0,
                        b: true,
                    },
                    E::Bar {
                        a: NoEq,
                        i: 0,
                        b: true,
                    },
                );
                assert_eq!(
                    E::Bar {
                        a: NoEq,
                        i: 0,
                        b: true,
                    },
                    E::Bar {
                        a: NoEq,
                        i: 0,
                        b: false,
                    },
                );
                assert_ne!(
                    E::Bar {
                        a: NoEq,
                        i: 0,
                        b: true,
                    },
                    E::Bar {
                        a: NoEq,
                        i: 1,
                        b: true,
                    },
                );

                assert_ne!(
                    E::Foo(NoEq, true, 0),
                    E::Bar {
                        a: NoEq,
                        i: 1,
                        b: true,
                    },
                );
            }

            #[test]
            fn all_fields() {
                #[derive(Debug, PartialEq)]
                enum E {
                    Foo(#[partial_eq(skip)] NoEq, #[partial_eq(skip)] i32),
                    Bar {
                        #[partial_eq(skip)]
                        a: NoEq,
                        #[partial_eq(skip)]
                        b: bool,
                    },
                }

                assert_eq!(E::Foo(NoEq, 0), E::Foo(NoEq, 0));
                assert_eq!(E::Foo(NoEq, 0), E::Foo(NoEq, 1));

                assert_eq!(E::Bar { a: NoEq, b: true }, E::Bar { a: NoEq, b: true });
                assert_eq!(E::Bar { a: NoEq, b: true }, E::Bar { a: NoEq, b: false });

                assert_ne!(E::Foo(NoEq, 0), E::Bar { a: NoEq, b: true });
            }

            #[test]
            fn variants() {
                #[derive(Debug, PartialEq)]
                enum E {
                    Foo(bool, i32),
                    #[partial_eq(skip)]
                    Bar {
                        a: NoEq,
                        b: bool,
                    },
                }

                assert_eq!(E::Foo(true, 0), E::Foo(true, 0));
                assert_ne!(E::Foo(true, 0), E::Foo(true, 1));
                assert_ne!(E::Foo(true, 0), E::Foo(false, 0));

                assert_eq!(E::Bar { a: NoEq, b: true }, E::Bar { a: NoEq, b: true });
                assert_eq!(E::Bar { a: NoEq, b: true }, E::Bar { a: NoEq, b: false });

                assert_ne!(E::Foo(true, 0), E::Bar { a: NoEq, b: true });
            }

            #[test]
            fn all_variants() {
                #[derive(Debug, PartialEq)]
                enum E {
                    #[partial_eq(skip)]
                    Foo(bool, i32),
                    #[partial_eq(skip)]
                    Bar { a: NoEq, b: bool },
                    #[partial_eq(skip)]
                    Baz,
                }

                assert_eq!(E::Foo(true, 0), E::Foo(true, 0));
                assert_eq!(E::Foo(true, 0), E::Foo(true, 1));
                assert_eq!(E::Foo(true, 0), E::Foo(false, 0));
                assert_eq!(E::Foo(true, 0), E::Foo(false, 1));

                assert_eq!(E::Bar { a: NoEq, b: true }, E::Bar { a: NoEq, b: true });
                assert_eq!(E::Bar { a: NoEq, b: true }, E::Bar { a: NoEq, b: false });

                assert_eq!(E::Baz, E::Baz);

                assert_ne!(E::Foo(true, 0), E::Bar { a: NoEq, b: true });
                assert_ne!(E::Foo(true, 0), E::Baz);
                assert_ne!(E::Bar { a: NoEq, b: true }, E::Baz);
            }

            #[test]
            fn single_variant() {
                #[derive(Debug, PartialEq)]
                enum E {
                    #[partial_eq(skip)]
                    Bar { a: NoEq, b: bool },
                }

                assert_eq!(E::Bar { a: NoEq, b: true }, E::Bar { a: NoEq, b: true });
                assert_eq!(E::Bar { a: NoEq, b: true }, E::Bar { a: NoEq, b: false });
            }

            #[test]
            fn all_but_single_empty_variant() {
                #[derive(Debug, PartialEq)]
                enum E {
                    #[partial_eq(skip)]
                    Foo(bool, i32),
                    #[partial_eq(skip)]
                    Bar {
                        a: NoEq,
                        b: bool,
                    },
                    Baz,
                }

                assert_eq!(E::Foo(true, 0), E::Foo(true, 0));
                assert_eq!(E::Foo(true, 0), E::Foo(true, 1));
                assert_eq!(E::Foo(true, 0), E::Foo(false, 0));
                assert_eq!(E::Foo(true, 0), E::Foo(false, 1));

                assert_eq!(E::Bar { a: NoEq, b: true }, E::Bar { a: NoEq, b: true });
                assert_eq!(E::Bar { a: NoEq, b: true }, E::Bar { a: NoEq, b: false });

                assert_eq!(E::Baz, E::Baz);

                assert_ne!(E::Foo(true, 0), E::Bar { a: NoEq, b: true });
                assert_ne!(E::Foo(true, 0), E::Baz);
                assert_ne!(E::Bar { a: NoEq, b: true }, E::Baz);
            }

            #[test]
            fn mixed() {
                #[derive(Debug, PartialEq)]
                enum E {
                    #[partial_eq(skip)]
                    Foo(bool, i32),
                    Bar {
                        #[partial_eq(skip)]
                        a: NoEq,
                        b: bool,
                    },
                    Baz,
                }

                assert_eq!(E::Foo(true, 0), E::Foo(true, 0));
                assert_eq!(E::Foo(true, 0), E::Foo(true, 1));
                assert_eq!(E::Foo(true, 0), E::Foo(false, 0));
                assert_eq!(E::Foo(true, 0), E::Foo(false, 1));

                assert_eq!(E::Bar { a: NoEq, b: true }, E::Bar { a: NoEq, b: true });
                assert_ne!(E::Bar { a: NoEq, b: true }, E::Bar { a: NoEq, b: false });

                assert_eq!(E::Baz, E::Baz);

                assert_ne!(E::Foo(true, 0), E::Bar { a: NoEq, b: true });
                assert_ne!(E::Foo(true, 0), E::Baz);
                assert_ne!(E::Bar { a: NoEq, b: true }, E::Baz);
            }
        }

        mod generic {
            #[cfg(not(feature = "std"))]
            use ::alloc::{boxed::Box, vec, vec::Vec};
            use derive_more::PartialEq;

            trait Some {
                type Assoc;
            }

            impl<T> Some for T {
                type Assoc = bool;
            }

            #[derive(Debug)]
            struct NoEq;

            #[test]
            fn single_variant_multi_field_tuple() {
                #[derive(Debug, PartialEq)]
                enum E<A: Some, B> {
                    Foo(A::Assoc, B),
                }

                assert_eq!(E::<NoEq, _>::Foo(true, 0), E::Foo(true, 0));
                assert_ne!(E::<NoEq, _>::Foo(true, 0), E::Foo(false, 0));
                assert_ne!(E::<NoEq, _>::Foo(true, 0), E::Foo(true, 1));
                assert_ne!(E::<NoEq, _>::Foo(true, 0), E::Foo(false, 1));
            }

            #[test]
            fn single_variant_multi_field_struct() {
                #[derive(Debug, PartialEq)]
                enum E<A, B: Some> {
                    Bar { b: B::Assoc, i: A },
                }

                assert_eq!(
                    E::<_, NoEq>::Bar { b: true, i: 0 },
                    E::Bar { b: true, i: 0 }
                );
                assert_ne!(
                    E::<_, NoEq>::Bar { b: true, i: 0 },
                    E::Bar { b: false, i: 0 }
                );
                assert_ne!(
                    E::<_, NoEq>::Bar { b: true, i: 0 },
                    E::Bar { b: true, i: 1 }
                );
                assert_ne!(
                    E::<_, NoEq>::Bar { b: true, i: 0 },
                    E::Bar { b: false, i: 1 }
                );
            }

            #[test]
            fn multi_variant_empty_and_multi_field() {
                #[derive(Debug, PartialEq)]
                enum E<A, B: Some> {
                    Foo(B::Assoc, A),
                    Bar { b: B::Assoc, i: A },
                    Baz,
                }

                assert_eq!(E::<_, NoEq>::Foo(true, 0), E::Foo(true, 0));
                assert_ne!(E::<_, NoEq>::Foo(true, 0), E::Foo(false, 0));
                assert_ne!(E::<_, NoEq>::Foo(true, 0), E::Foo(true, 1));
                assert_ne!(E::<_, NoEq>::Foo(true, 0), E::Foo(false, 1));

                assert_eq!(
                    E::<_, NoEq>::Bar { b: true, i: 0 },
                    E::Bar { b: true, i: 0 }
                );
                assert_ne!(
                    E::<_, NoEq>::Bar { b: true, i: 0 },
                    E::Bar { b: false, i: 0 }
                );
                assert_ne!(
                    E::<_, NoEq>::Bar { b: true, i: 0 },
                    E::Bar { b: true, i: 1 }
                );
                assert_ne!(
                    E::<_, ()>::Bar { b: true, i: 0 },
                    E::Bar { b: false, i: 1 }
                );

                assert_eq!(E::<i32, NoEq>::Baz, E::Baz);

                assert_ne!(E::<_, NoEq>::Foo(true, 0), E::Bar { b: true, i: 0 });
                assert_ne!(E::<_, NoEq>::Bar { b: false, i: 1 }, E::Foo(false, 1));
                assert_ne!(E::<_, NoEq>::Foo(true, 0), E::Baz);
                assert_ne!(E::<_, NoEq>::Baz, E::Foo(false, 1));
                assert_ne!(E::<_, NoEq>::Bar { b: false, i: 1 }, E::Baz);
                assert_ne!(E::<_, NoEq>::Baz, E::Bar { b: true, i: 0 });
            }

            #[test]
            fn lifetime() {
                #[derive(Debug, PartialEq)]
                enum E1<'a> {
                    Foo(&'a str, i32),
                }

                #[derive(Debug, PartialEq)]
                enum E2<'a> {
                    Bar { b: E1<'a>, i: i32 },
                }

                assert_eq!(E1::Foo("hi", 0), E1::Foo("hi", 0));
                assert_ne!(E1::Foo("hi", 0), E1::Foo("bye", 0));
                assert_ne!(E1::Foo("hi", 0), E1::Foo("hi", 1));
                assert_ne!(E1::Foo("hi", 0), E1::Foo("bye", 1));

                assert_eq!(
                    E2::Bar {
                        b: E1::Foo("hi", 0),
                        i: 0,
                    },
                    E2::Bar {
                        b: E1::Foo("hi", 0),
                        i: 0,
                    },
                );
                assert_ne!(
                    E2::Bar {
                        b: E1::Foo("hi", 0),
                        i: 0,
                    },
                    E2::Bar {
                        b: E1::Foo("bye", 0),
                        i: 0,
                    },
                );
                assert_ne!(
                    E2::Bar {
                        b: E1::Foo("hi", 0),
                        i: 0,
                    },
                    E2::Bar {
                        b: E1::Foo("hi", 0),
                        i: 1,
                    },
                );
                assert_ne!(
                    E2::Bar {
                        b: E1::Foo("hi", 0),
                        i: 0,
                    },
                    E2::Bar {
                        b: E1::Foo("bye", 0),
                        i: 1,
                    },
                );
            }

            #[test]
            fn const_param() {
                #[derive(Debug, PartialEq)]
                enum E3<const N: usize> {
                    Baz,
                }

                #[derive(Debug, PartialEq)]
                enum E1<const N: usize> {
                    Foo([i32; N], i8),
                }

                #[derive(Debug, PartialEq)]
                enum E2<const N: usize> {
                    Bar { b: E1<N>, i: E3<N> },
                }

                assert_eq!(E3::<1>::Baz, E3::Baz);

                assert_eq!(E1::Foo([3], 0), E1::Foo([3], 0));
                assert_ne!(E1::Foo([3], 0), E1::Foo([4], 0));
                assert_ne!(E1::Foo([3], 0), E1::Foo([3], 1));
                assert_ne!(E1::Foo([3], 0), E1::Foo([4], 1));

                assert_eq!(
                    E2::Bar {
                        b: E1::Foo([3], 0),
                        i: E3::Baz,
                    },
                    E2::Bar {
                        b: E1::Foo([3], 0),
                        i: E3::Baz,
                    },
                );
                assert_ne!(
                    E2::Bar {
                        b: E1::Foo([3], 0),
                        i: E3::Baz,
                    },
                    E2::Bar {
                        b: E1::Foo([3], 1),
                        i: E3::Baz,
                    },
                );
            }

            #[test]
            fn recursive() {
                #[derive(Debug, PartialEq)]
                enum E<A, B: Some> {
                    Foo(B::Assoc, Vec<Self>),
                    Bar { b: Option<Box<E<A, B>>>, i: A },
                    Baz,
                }

                assert_eq!(E::<(), NoEq>::Foo(true, vec![]), E::Foo(true, vec![]));
                assert_ne!(E::<(), NoEq>::Foo(true, vec![]), E::Foo(false, vec![]));
                assert_ne!(
                    E::<(), NoEq>::Foo(true, vec![]),
                    E::Foo(true, vec![E::Baz]),
                );

                assert_eq!(
                    E::<_, NoEq>::Bar { b: None, i: 3 },
                    E::Bar { b: None, i: 3 },
                );
                assert_ne!(
                    E::<_, NoEq>::Bar { b: None, i: 3 },
                    E::Bar { b: None, i: 0 },
                );
                assert_ne!(
                    E::<_, NoEq>::Bar { b: None, i: 3 },
                    E::Bar {
                        b: Some(Box::new(E::Baz)),
                        i: 3,
                    },
                );

                assert_eq!(E::<(), NoEq>::Baz, E::Baz);

                assert_ne!(E::<_, NoEq>::Foo(true, vec![]), E::Bar { b: None, i: 3 });
                assert_ne!(E::<(), NoEq>::Foo(true, vec![]), E::Baz);
                assert_ne!(E::<_, NoEq>::Bar { b: None, i: 3 }, E::Baz);
            }

            mod skip {
                use derive_more::PartialEq;

                #[derive(Debug)]
                struct NoEq;

                #[test]
                fn fields() {
                    #[derive(Debug, PartialEq)]
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

                    assert_eq!(E::Foo(NoEq, true, 0), E::Foo(NoEq, true, 0));
                    assert_eq!(E::Foo(NoEq, true, 0), E::Foo(NoEq, true, 1));
                    assert_ne!(E::Foo(NoEq, true, 0), E::Foo(NoEq, false, 0));

                    assert_eq!(
                        E::Bar {
                            a: NoEq,
                            i: 0,
                            b: true,
                        },
                        E::Bar {
                            a: NoEq,
                            i: 0,
                            b: true,
                        },
                    );
                    assert_eq!(
                        E::Bar {
                            a: NoEq,
                            i: 0,
                            b: true,
                        },
                        E::Bar {
                            a: NoEq,
                            i: 0,
                            b: false,
                        },
                    );
                    assert_ne!(
                        E::Bar {
                            a: NoEq,
                            i: 0,
                            b: true,
                        },
                        E::Bar {
                            a: NoEq,
                            i: 1,
                            b: true,
                        },
                    );

                    assert_ne!(
                        E::Foo(NoEq, true, 0),
                        E::Bar {
                            a: NoEq,
                            i: 1,
                            b: true,
                        },
                    );
                }

                #[test]
                fn all_fields() {
                    #[derive(Debug, PartialEq)]
                    enum E<A, B, C> {
                        Foo(#[partial_eq(skip)] A, #[partial_eq(skip)] B),
                        Bar {
                            #[partial_eq(skip)]
                            a: A,
                            #[partial_eq(skip)]
                            b: C,
                        },
                    }

                    assert_eq!(E::<_, _, ()>::Foo(NoEq, 0), E::Foo(NoEq, 0));
                    assert_eq!(E::<_, _, ()>::Foo(NoEq, 0), E::Foo(NoEq, 1));

                    assert_eq!(
                        E::<_, (), _>::Bar { a: NoEq, b: true },
                        E::Bar { a: NoEq, b: true },
                    );
                    assert_eq!(
                        E::<_, (), _>::Bar { a: NoEq, b: true },
                        E::Bar { a: NoEq, b: false },
                    );

                    assert_ne!(E::Foo(NoEq, 0), E::Bar { a: NoEq, b: true });
                }

                #[test]
                fn variants() {
                    #[derive(Debug, PartialEq)]
                    enum E<A, B, C> {
                        Foo(A, B),
                        #[partial_eq(skip)]
                        Bar {
                            a: C,
                            b: A,
                        },
                    }

                    assert_eq!(E::<_, _, NoEq>::Foo(true, 0), E::Foo(true, 0));
                    assert_ne!(E::<_, _, NoEq>::Foo(true, 0), E::Foo(true, 1));
                    assert_ne!(E::<_, _, NoEq>::Foo(true, 0), E::Foo(false, 0));

                    assert_eq!(
                        E::<_, i32, _>::Bar { a: NoEq, b: true },
                        E::Bar { a: NoEq, b: true },
                    );
                    assert_eq!(
                        E::<_, i32, _>::Bar { a: NoEq, b: true },
                        E::Bar { a: NoEq, b: false },
                    );

                    assert_ne!(E::Foo(true, 0), E::Bar { a: NoEq, b: true });
                }

                #[test]
                fn all_variants() {
                    #[derive(Debug, PartialEq)]
                    enum E<A, B, C> {
                        #[partial_eq(skip)]
                        Foo(A, B),
                        #[partial_eq(skip)]
                        Bar { a: C, b: A },
                        #[partial_eq(skip)]
                        Baz,
                    }

                    assert_eq!(E::<_, _, NoEq>::Foo(true, 0), E::Foo(true, 0));
                    assert_eq!(E::<_, _, NoEq>::Foo(true, 0), E::Foo(true, 1));
                    assert_eq!(E::<_, _, NoEq>::Foo(true, 0), E::Foo(false, 0));
                    assert_eq!(E::<_, _, NoEq>::Foo(true, 0), E::Foo(false, 1));

                    assert_eq!(
                        E::<_, i32, _>::Bar { a: NoEq, b: true },
                        E::Bar { a: NoEq, b: true },
                    );
                    assert_eq!(
                        E::<_, i32, _>::Bar { a: NoEq, b: true },
                        E::Bar { a: NoEq, b: false },
                    );

                    assert_eq!(E::<bool, i32, NoEq>::Baz, E::Baz);

                    assert_ne!(E::Foo(true, 0), E::Bar { a: NoEq, b: true });
                    assert_ne!(E::<_, _, NoEq>::Foo(true, 0), E::Baz);
                    assert_ne!(E::<_, i32, _>::Bar { a: NoEq, b: true }, E::Baz);
                }

                #[test]
                fn single_variant() {
                    #[derive(Debug, PartialEq)]
                    enum E<A, B> {
                        #[partial_eq(skip)]
                        Bar { a: A, b: B },
                    }

                    assert_eq!(
                        E::Bar { a: NoEq, b: true },
                        E::Bar { a: NoEq, b: true },
                    );
                    assert_eq!(
                        E::Bar { a: NoEq, b: true },
                        E::Bar { a: NoEq, b: false },
                    );
                }

                #[test]
                fn all_but_single_empty_variant() {
                    #[derive(Debug, PartialEq)]
                    enum E<A, B, C> {
                        #[partial_eq(skip)]
                        Foo(A, B),
                        #[partial_eq(skip)]
                        Bar {
                            a: C,
                            b: A,
                        },
                        Baz,
                    }

                    assert_eq!(E::<_, _, NoEq>::Foo(true, 0), E::Foo(true, 0));
                    assert_eq!(E::<_, _, NoEq>::Foo(true, 0), E::Foo(true, 1));
                    assert_eq!(E::<_, _, NoEq>::Foo(true, 0), E::Foo(false, 0));
                    assert_eq!(E::<_, _, NoEq>::Foo(true, 0), E::Foo(false, 1));

                    assert_eq!(
                        E::<_, i32, _>::Bar { a: NoEq, b: true },
                        E::Bar { a: NoEq, b: true },
                    );
                    assert_eq!(
                        E::<_, i32, _>::Bar { a: NoEq, b: true },
                        E::Bar { a: NoEq, b: false },
                    );

                    assert_eq!(E::<bool, i32, NoEq>::Baz, E::Baz);

                    assert_ne!(E::Foo(true, 0), E::Bar { a: NoEq, b: true });
                    assert_ne!(E::<_, _, NoEq>::Foo(true, 0), E::Baz);
                    assert_ne!(E::<_, i32, _>::Bar { a: NoEq, b: true }, E::Baz);
                }

                #[test]
                fn mixed() {
                    #[derive(Debug, PartialEq)]
                    enum E<A, B, C> {
                        #[partial_eq(skip)]
                        Foo(A, B),
                        Bar {
                            #[partial_eq(skip)]
                            a: C,
                            b: A,
                        },
                        Baz,
                    }

                    assert_eq!(E::<_, _, NoEq>::Foo(true, 0), E::Foo(true, 0));
                    assert_eq!(E::<_, _, NoEq>::Foo(true, 0), E::Foo(true, 1));
                    assert_eq!(E::<_, _, NoEq>::Foo(true, 0), E::Foo(false, 0));
                    assert_eq!(E::<_, _, NoEq>::Foo(true, 0), E::Foo(false, 1));

                    assert_eq!(
                        E::<_, i32, _>::Bar { a: NoEq, b: true },
                        E::Bar { a: NoEq, b: true },
                    );
                    assert_ne!(
                        E::<_, i32, _>::Bar { a: NoEq, b: true },
                        E::Bar { a: NoEq, b: false },
                    );

                    assert_eq!(E::<bool, i32, NoEq>::Baz, E::Baz);

                    assert_ne!(E::Foo(true, 0), E::Bar { a: NoEq, b: true });
                    assert_ne!(E::<_, _, NoEq>::Foo(true, 0), E::Baz);
                    assert_ne!(E::<_, i32, _>::Bar { a: NoEq, b: true }, E::Baz);
                }
            }
        }
    }
}
