#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)] // some code is tested for type checking only

#[cfg(not(feature = "std"))]
extern crate alloc;

mod structs {
    mod scalar {
        use derive_more::Mul;

        #[test]
        fn single_field_tuple() {
            #[derive(Mul, Debug, PartialEq)]
            struct MyInt(i32);

            assert_eq!(MyInt(-1) * 5, MyInt(-5));
        }

        #[test]
        fn multifield_field_tuple() {
            #[derive(Mul, Debug, PartialEq)]
            struct MyInts(i32, i32);

            assert_eq!(MyInts(-1, 3) * 5, MyInts(-5, 15));
        }

        #[test]
        fn single_field_struct() {
            #[derive(Mul, Debug, PartialEq)]
            struct Point1D {
                x: i32,
            }

            assert_eq!(Point1D { x: -1 } * 5, Point1D { x: -5 });
        }

        #[test]
        fn multi_field_struct() {
            #[derive(Mul, Debug, PartialEq)]
            struct Point2D {
                x: i32,
                y: i32,
            }

            assert_eq!(Point2D { x: -1, y: 3 } * 5, Point2D { x: -5, y: 15 });
        }

        mod generic {
            use derive_more::Mul;

            trait Some {
                type Assoc;
            }

            impl<T> Some for T {
                type Assoc = u32;
            }

            #[test]
            fn multi_field_tuple() {
                #[derive(Mul)]
                struct Foo<A: Some, B>(A::Assoc, B);

                let a = Foo::<(), _>(12, 21);
                let res = a * 3;

                assert_eq!((res.0, res.1), (36, 63));
            }

            #[test]
            fn multi_field_struct() {
                #[derive(Mul)]
                struct Bar<A, B: Some> {
                    b: B::Assoc,
                    i: A,
                }

                let a = Bar::<_, ()> { b: 12, i: 21 };
                let res = a * 5;

                assert_eq!((res.b, res.i), (60, 105));
            }
        }

        mod ignore {
            use core::marker::PhantomData;

            use derive_more::Mul;

            #[test]
            fn tuple() {
                #[derive(Mul)]
                struct TupleWithZst<T = ()>(i32, #[mul(ignore)] PhantomData<T>);

                let a: TupleWithZst = TupleWithZst(12, PhantomData);

                assert_eq!((a * 3).0, 36);
            }

            #[test]
            fn struct_() {
                #[derive(Mul)]
                struct StructWithZst<T = ()> {
                    x: i32,
                    #[mul(skip)]
                    _marker: PhantomData<T>,
                }

                let a: StructWithZst<()> = StructWithZst {
                    x: 12,
                    _marker: PhantomData,
                };

                assert_eq!((a * -3).x, -36);
            }
        }
    }

    mod structural {
        use derive_more::Mul;

        #[test]
        fn single_field_tuple() {
            #[derive(Mul, Debug, PartialEq)]
            #[mul(forward)]
            struct MyInt(i32);

            assert_eq!(MyInt(-1) * MyInt(5), MyInt(-5));
        }

        #[test]
        fn multifield_field_tuple() {
            #[derive(Mul, Debug, PartialEq)]
            #[mul(forward)]
            struct MyInts(i32, i32);

            assert_eq!(MyInts(-1, 3) * MyInts(3, 5), MyInts(-3, 15));
        }

        #[test]
        fn single_field_struct() {
            #[derive(Mul, Debug, PartialEq)]
            #[mul(forward)]
            struct Point1D {
                x: i32,
            }

            assert_eq!(Point1D { x: -1 } * Point1D { x: 5 }, Point1D { x: -5 });
        }

        #[test]
        fn multi_field_struct() {
            #[derive(Mul, Debug, PartialEq)]
            #[mul(forward)]
            struct Point2D {
                x: i32,
                y: i32,
            }

            assert_eq!(
                Point2D { x: -1, y: 3 } * Point2D { x: 3, y: 5 },
                Point2D { x: -3, y: 15 },
            );
        }

        mod generic {
            use derive_more::Mul;

            trait Some {
                type Assoc;
            }

            impl<T> Some for T {
                type Assoc = u32;
            }

            #[test]
            fn multi_field_tuple() {
                #[derive(Mul)]
                #[mul(forward)]
                struct Foo<A: Some, B>(A::Assoc, B);

                let a = Foo::<(), _>(12, 21);
                let b = Foo::<(), _>(1, 2);
                let res = a * b;

                assert_eq!((res.0, res.1), (12, 42));
            }

            #[test]
            fn multi_field_struct() {
                #[derive(Mul)]
                #[mul(forward)]
                struct Bar<A, B: Some> {
                    b: B::Assoc,
                    i: A,
                }

                let a = Bar::<_, ()> { b: 12, i: 21 };
                let b = Bar::<_, ()> { b: 1, i: 2 };
                let res = a * b;

                assert_eq!((res.b, res.i), (12, 42));
            }
        }

        mod ignore {
            use core::marker::PhantomData;

            use derive_more::Mul;

            #[test]
            fn tuple() {
                #[derive(Mul)]
                #[mul(forward)]
                struct TupleWithZst<T = ()>(i32, #[mul(ignore)] PhantomData<T>);

                let a: TupleWithZst = TupleWithZst(12, PhantomData);
                let b: TupleWithZst = TupleWithZst(2, PhantomData);

                assert_eq!((a * b).0, 24);
            }

            #[test]
            fn struct_() {
                #[derive(Mul)]
                #[mul(forward)]
                struct StructWithZst<T = ()> {
                    x: i32,
                    #[mul(skip)]
                    _marker: PhantomData<T>,
                }

                let a: StructWithZst<()> = StructWithZst {
                    x: 12,
                    _marker: PhantomData,
                };
                let b: StructWithZst<()> = StructWithZst {
                    x: 2,
                    _marker: PhantomData,
                };

                assert_eq!((a * b).x, 24);
            }
        }
    }
}

mod enums {
    mod structural {
        #[cfg(not(feature = "std"))]
        use alloc::string::ToString as _;
        use derive_more::Mul;

        #[test]
        fn empty() {
            #[derive(Mul)]
            #[mul(forward)]
            enum Empty {}
        }

        #[test]
        fn multi() {
            #[derive(Mul, Debug, PartialEq)]
            #[mul(forward)]
            enum MixedInts {
                SmallInt(i32),
                BigInt(i64),
                TwoSmallInts(i32, i32),
                NamedSmallInts { x: i32, y: i32 },
                UnsignedOne(u32),
                UnsignedTwo(u32),
                Unit,
            }

            assert_eq!(
                (MixedInts::SmallInt(-1) * MixedInts::SmallInt(2)).unwrap(),
                MixedInts::SmallInt(-2),
            );
            assert_eq!(
                (MixedInts::BigInt(-1) * MixedInts::BigInt(2)).unwrap(),
                MixedInts::BigInt(-2),
            );
            assert_eq!(
                (MixedInts::TwoSmallInts(-1, 3) * MixedInts::TwoSmallInts(2, -5))
                    .unwrap(),
                MixedInts::TwoSmallInts(-2, -15),
            );
            assert_eq!(
                (MixedInts::NamedSmallInts { x: -1, y: 3 }
                    * MixedInts::NamedSmallInts { x: 2, y: -5 })
                .unwrap(),
                MixedInts::NamedSmallInts { x: -2, y: -15 },
            );
            assert_eq!(
                (MixedInts::UnsignedOne(1) * MixedInts::UnsignedOne(2)).unwrap(),
                MixedInts::UnsignedOne(2),
            );
            assert_eq!(
                (MixedInts::UnsignedTwo(1) * MixedInts::UnsignedTwo(2)).unwrap(),
                MixedInts::UnsignedTwo(2),
            );

            assert_eq!(
                (MixedInts::Unit * MixedInts::Unit).unwrap_err().to_string(),
                "Cannot mul() unit variants",
            );
            assert_eq!(
                (MixedInts::SmallInt(-1) * MixedInts::BigInt(2))
                    .unwrap_err()
                    .to_string(),
                "Trying to mul() mismatched enum variants",
            );
        }

        mod ignore {
            #[cfg(not(feature = "std"))]
            use alloc::string::ToString as _;
            use derive_more::Mul;

            #[test]
            fn multi() {
                #[derive(Mul, Debug, PartialEq)]
                #[mul(forward)]
                enum MixedInts {
                    TwoSmallInts(i32, #[mul(skip)] i32),
                    NamedSmallInts {
                        #[mul(ignore)]
                        x: i32,
                        y: i32,
                    },
                    Unit,
                }

                assert_eq!(
                    (MixedInts::TwoSmallInts(-1, 3) * MixedInts::TwoSmallInts(2, -5))
                        .unwrap(),
                    MixedInts::TwoSmallInts(-2, 3),
                );
                assert_eq!(
                    (MixedInts::NamedSmallInts { x: -1, y: 3 }
                        * MixedInts::NamedSmallInts { x: 2, y: -5 })
                    .unwrap(),
                    MixedInts::NamedSmallInts { x: -1, y: -15 },
                );

                assert_eq!(
                    (MixedInts::Unit * MixedInts::Unit).unwrap_err().to_string(),
                    "Cannot mul() unit variants",
                );
                assert_eq!(
                    (MixedInts::TwoSmallInts(-1, 3)
                        * MixedInts::NamedSmallInts { x: -1, y: 3 })
                    .unwrap_err()
                    .to_string(),
                    "Trying to mul() mismatched enum variants",
                );
            }
        }
    }
}
