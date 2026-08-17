#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)] // some code is tested for type checking only

#[cfg(not(feature = "std"))]
extern crate alloc;

mod structs {
    use derive_more::Add;

    #[test]
    fn multi_field_tuple() {
        #[derive(Add, Debug, PartialEq)]
        struct MyInts(i32, i32);

        let a = MyInts(12, 21);
        let b = MyInts(1, 2);

        assert_eq!(a + b, MyInts(13, 23));
    }

    #[test]
    fn multi_field_struct() {
        #[derive(Add, Debug, PartialEq)]
        struct Point2D {
            x: i32,
            y: i32,
        }

        let a = Point2D { x: 12, y: 21 };
        let b = Point2D { x: 1, y: 2 };

        assert_eq!(a + b, Point2D { x: 13, y: 23 });
    }

    mod generic {
        use derive_more::Add;

        trait Some {
            type Assoc;
        }

        impl<T> Some for T {
            type Assoc = u32;
        }

        #[test]
        fn multi_field_tuple() {
            #[derive(Add)]
            struct Foo<A: Some, B>(A::Assoc, B);

            let a = Foo::<(), _>(12, 21);
            let b = Foo::<(), _>(1, 2);
            let res = a + b;

            assert_eq!((res.0, res.1), (13, 23));
        }

        #[test]
        fn multi_field_struct() {
            #[derive(Add)]
            struct Bar<A, B: Some> {
                b: B::Assoc,
                i: A,
            }

            let a = Bar::<_, ()> { b: 12, i: 21 };
            let b = Bar::<_, ()> { b: 1, i: 2 };
            let res = a + b;

            assert_eq!((res.b, res.i), (13, 23));
        }
    }

    mod ignore {
        use core::marker::PhantomData;

        use derive_more::Add;

        #[test]
        fn tuple() {
            #[derive(Add)]
            struct TupleWithZst<T = ()>(i32, #[add(ignore)] PhantomData<T>);

            let a: TupleWithZst = TupleWithZst(12, PhantomData);
            let b: TupleWithZst = TupleWithZst(2, PhantomData);

            assert_eq!((a + b).0, 14);
        }

        #[test]
        fn struct_() {
            #[derive(Add)]
            struct StructWithZst<T = ()> {
                x: i32,
                #[add(skip)]
                _marker: PhantomData<T>,
            }

            let a: StructWithZst = StructWithZst {
                x: 12,
                _marker: PhantomData,
            };
            let b: StructWithZst = StructWithZst {
                x: 2,
                _marker: PhantomData,
            };

            assert_eq!((a + b).x, 14);
        }
    }
}

mod enums {
    #[cfg(not(feature = "std"))]
    use alloc::string::ToString as _;
    use derive_more::Add;

    #[test]
    fn empty() {
        #[derive(Add)]
        enum Empty {}
    }

    #[test]
    fn multi() {
        #[derive(Add, Debug, PartialEq)]
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
            (MixedInts::SmallInt(-1) + MixedInts::SmallInt(2)).unwrap(),
            MixedInts::SmallInt(1),
        );
        assert_eq!(
            (MixedInts::BigInt(-1) + MixedInts::BigInt(2)).unwrap(),
            MixedInts::BigInt(1),
        );
        assert_eq!(
            (MixedInts::TwoSmallInts(-1, 3) + MixedInts::TwoSmallInts(2, -5)).unwrap(),
            MixedInts::TwoSmallInts(1, -2),
        );
        assert_eq!(
            (MixedInts::NamedSmallInts { x: -1, y: 3 }
                + MixedInts::NamedSmallInts { x: 2, y: -5 })
            .unwrap(),
            MixedInts::NamedSmallInts { x: 1, y: -2 },
        );
        assert_eq!(
            (MixedInts::UnsignedOne(1) + MixedInts::UnsignedOne(2)).unwrap(),
            MixedInts::UnsignedOne(3),
        );
        assert_eq!(
            (MixedInts::UnsignedTwo(1) + MixedInts::UnsignedTwo(2)).unwrap(),
            MixedInts::UnsignedTwo(3),
        );

        assert_eq!(
            (MixedInts::Unit + MixedInts::Unit).unwrap_err().to_string(),
            "Cannot add() unit variants",
        );
        assert_eq!(
            (MixedInts::SmallInt(-1) + MixedInts::BigInt(2))
                .unwrap_err()
                .to_string(),
            "Trying to add() mismatched enum variants",
        );
    }

    mod ignore {
        #[cfg(not(feature = "std"))]
        use alloc::string::ToString as _;
        use derive_more::Add;

        #[test]
        fn multi() {
            #[derive(Add, Debug, PartialEq)]
            enum MixedInts {
                TwoSmallInts(i32, #[add(skip)] i32),
                NamedSmallInts {
                    #[add(ignore)]
                    x: i32,
                    y: i32,
                },
                Unit,
            }

            assert_eq!(
                (MixedInts::TwoSmallInts(-1, 3) + MixedInts::TwoSmallInts(2, -5))
                    .unwrap(),
                MixedInts::TwoSmallInts(1, 3),
            );
            assert_eq!(
                (MixedInts::NamedSmallInts { x: -1, y: 3 }
                    + MixedInts::NamedSmallInts { x: 2, y: -5 })
                .unwrap(),
                MixedInts::NamedSmallInts { x: -1, y: -2 },
            );

            assert_eq!(
                (MixedInts::Unit + MixedInts::Unit).unwrap_err().to_string(),
                "Cannot add() unit variants",
            );
            assert_eq!(
                (MixedInts::TwoSmallInts(-1, 3)
                    + MixedInts::NamedSmallInts { x: -1, y: 3 })
                .unwrap_err()
                .to_string(),
                "Trying to add() mismatched enum variants",
            );
        }
    }
}
