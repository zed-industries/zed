#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)] // some code is tested for type checking only

mod structs {
    mod scalar {
        use derive_more::MulAssign;

        #[test]
        fn single_field_tuple() {
            #[derive(MulAssign, Debug, PartialEq)]
            struct MyInt(i32);

            let mut a = MyInt(-1);
            a *= 5;

            assert_eq!(a, MyInt(-5));
        }

        #[test]
        fn multifield_field_tuple() {
            #[derive(MulAssign, Debug, PartialEq)]
            struct MyInts(i32, i32);

            let mut a = MyInts(-1, 3);
            a *= 5;

            assert_eq!(a, MyInts(-5, 15));
        }

        #[test]
        fn single_field_struct() {
            #[derive(MulAssign, Debug, PartialEq)]
            struct Point1D {
                x: i32,
            }

            let mut a = Point1D { x: -1 };
            a *= 5;

            assert_eq!(a, Point1D { x: -5 });
        }

        #[test]
        fn multi_field_struct() {
            #[derive(MulAssign, Debug, PartialEq)]
            struct Point2D {
                x: i32,
                y: i32,
            }

            let mut a = Point2D { x: -1, y: 3 };
            a *= 5;

            assert_eq!(a, Point2D { x: -5, y: 15 });
        }

        mod generic {
            use derive_more::MulAssign;

            trait Some {
                type Assoc;
            }

            impl<T> Some for T {
                type Assoc = u32;
            }

            #[test]
            fn multi_field_tuple() {
                #[derive(MulAssign)]
                struct Foo<A: Some, B>(A::Assoc, B);

                let mut a = Foo::<(), _>(12, 21);
                a *= 3;

                assert_eq!((a.0, a.1), (36, 63));
            }

            #[test]
            fn multi_field_struct() {
                #[derive(MulAssign)]
                struct Bar<A, B: Some> {
                    b: B::Assoc,
                    i: A,
                }

                let mut a = Bar::<_, ()> { b: 12, i: 21 };
                a *= 5;

                assert_eq!((a.b, a.i), (60, 105));
            }
        }

        mod ignore {
            use core::marker::PhantomData;

            use derive_more::MulAssign;

            #[test]
            fn tuple() {
                #[derive(MulAssign)]
                struct TupleWithZst<T = ()>(i32, #[mul_assign(ignore)] PhantomData<T>);

                let mut a: TupleWithZst = TupleWithZst(12, PhantomData);
                a *= 3;

                assert_eq!(a.0, 36);
            }

            #[test]
            fn struct_() {
                #[derive(MulAssign)]
                struct StructWithZst<T = ()> {
                    x: i32,
                    #[mul_assign(skip)]
                    _marker: PhantomData<T>,
                }

                let mut a: StructWithZst<()> = StructWithZst {
                    x: 12,
                    _marker: PhantomData,
                };
                a *= -3;

                assert_eq!(a.x, -36);
            }
        }
    }

    mod structural {
        use derive_more::MulAssign;

        #[test]
        fn single_field_tuple() {
            #[derive(MulAssign, Debug, PartialEq)]
            #[mul_assign(forward)]
            struct MyInt(i32);

            let mut a = MyInt(-1);
            a *= MyInt(5);

            assert_eq!(a, MyInt(-5));
        }

        #[test]
        fn multifield_field_tuple() {
            #[derive(MulAssign, Debug, PartialEq)]
            #[mul_assign(forward)]
            struct MyInts(i32, i32);

            let mut a = MyInts(-1, 3);
            a *= MyInts(3, 5);

            assert_eq!(a, MyInts(-3, 15));
        }

        #[test]
        fn single_field_struct() {
            #[derive(MulAssign, Debug, PartialEq)]
            #[mul_assign(forward)]
            struct Point1D {
                x: i32,
            }

            let mut a = Point1D { x: -1 };
            a *= Point1D { x: 5 };

            assert_eq!(a, Point1D { x: -5 });
        }

        #[test]
        fn multi_field_struct() {
            #[derive(MulAssign, Debug, PartialEq)]
            #[mul_assign(forward)]
            struct Point2D {
                x: i32,
                y: i32,
            }

            let mut a = Point2D { x: -1, y: 3 };
            a *= Point2D { x: 3, y: 5 };

            assert_eq!(a, Point2D { x: -3, y: 15 });
        }

        mod generic {
            use derive_more_impl::MulAssign;

            trait Some {
                type Assoc;
            }

            impl<T> Some for T {
                type Assoc = u32;
            }

            #[test]
            fn multi_field_tuple() {
                #[derive(MulAssign)]
                #[mul_assign(forward)]
                struct Foo<A: Some, B>(A::Assoc, B);

                let mut a = Foo::<(), _>(12, 21);
                a *= Foo::<(), _>(1, 2);

                assert_eq!((a.0, a.1), (12, 42));
            }

            #[test]
            fn multi_field_struct() {
                #[derive(MulAssign)]
                #[mul_assign(forward)]
                struct Bar<A, B: Some> {
                    b: B::Assoc,
                    i: A,
                }

                let mut a = Bar::<_, ()> { b: 12, i: 21 };
                a *= Bar::<_, ()> { b: 1, i: 2 };

                assert_eq!((a.b, a.i), (12, 42));
            }
        }

        mod ignore {
            use core::marker::PhantomData;

            use derive_more::MulAssign;

            #[test]
            fn tuple() {
                #[derive(MulAssign)]
                #[mul_assign(forward)]
                struct TupleWithZst<T = ()>(i32, #[mul_assign(ignore)] PhantomData<T>);

                let mut a: TupleWithZst = TupleWithZst(12, PhantomData);
                a *= TupleWithZst(2, PhantomData);

                assert_eq!(a.0, 24);
            }

            #[test]
            fn struct_() {
                #[derive(MulAssign)]
                #[mul_assign(forward)]
                struct StructWithZst<T = ()> {
                    x: i32,
                    #[mul_assign(skip)]
                    _marker: PhantomData<T>,
                }

                let mut a: StructWithZst<()> = StructWithZst {
                    x: 12,
                    _marker: PhantomData,
                };
                a *= StructWithZst {
                    x: 2,
                    _marker: PhantomData,
                };

                assert_eq!(a.x, 24);
            }
        }
    }
}
