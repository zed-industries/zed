#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)] // some code is tested for type checking only

mod structs {
    use derive_more::AddAssign;

    #[test]
    fn multi_field_tuple() {
        #[derive(AddAssign, Debug, PartialEq)]
        struct MyInts(i32, i32);

        let mut a = MyInts(12, 21);
        a += MyInts(1, 2);

        assert_eq!(a, MyInts(13, 23));
    }

    #[test]
    fn multi_field_struct() {
        #[derive(AddAssign, Debug, PartialEq)]
        struct Point2D {
            x: i32,
            y: i32,
        }

        let mut a = Point2D { x: 12, y: 21 };
        a += Point2D { x: 1, y: 2 };

        assert_eq!(a, Point2D { x: 13, y: 23 });
    }

    mod generic {
        use derive_more::AddAssign;

        trait Some {
            type Assoc;
        }

        impl<T> Some for T {
            type Assoc = u32;
        }

        #[test]
        fn multi_field_tuple() {
            #[derive(AddAssign)]
            struct Foo<A: Some, B>(A::Assoc, B);

            let mut a = Foo::<(), _>(12, 21);
            a += Foo::<(), _>(1, 2);

            assert_eq!((a.0, a.1), (13, 23));
        }

        #[test]
        fn multi_field_struct() {
            #[derive(AddAssign)]
            struct Bar<A, B: Some> {
                b: B::Assoc,
                i: A,
            }

            let mut a = Bar::<_, ()> { b: 12, i: 21 };
            a += Bar::<_, ()> { b: 1, i: 2 };

            assert_eq!((a.b, a.i), (13, 23));
        }
    }

    mod ignore {
        use core::marker::PhantomData;

        use derive_more::AddAssign;

        #[test]
        fn tuple() {
            #[derive(AddAssign)]
            struct TupleWithZst<T = ()>(i32, #[add_assign(ignore)] PhantomData<T>);

            let mut a: TupleWithZst = TupleWithZst(12, PhantomData);
            a += TupleWithZst(2, PhantomData);

            assert_eq!(a.0, 14);
        }

        #[test]
        fn struct_() {
            #[derive(AddAssign)]
            struct StructWithZst<T = ()> {
                x: i32,
                #[add_assign(skip)]
                _marker: PhantomData<T>,
            }

            let mut a: StructWithZst = StructWithZst {
                x: 12,
                _marker: PhantomData,
            };
            a += StructWithZst {
                x: 2,
                _marker: PhantomData,
            };

            assert_eq!(a.x, 14);
        }
    }
}
