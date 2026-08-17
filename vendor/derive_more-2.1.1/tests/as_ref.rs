#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(nightly, feature(never_type))]
#![allow(dead_code)] // some code is tested for type checking only

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{borrow::ToOwned, collections::VecDeque, string::String, vec, vec::Vec};

#[cfg(feature = "std")]
use std::collections::VecDeque;

use core::ptr;

use derive_more::AsRef;

struct Helper(i32, f64, bool);

impl AsRef<i32> for Helper {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

impl AsRef<f64> for Helper {
    fn as_ref(&self) -> &f64 {
        &self.1
    }
}

impl AsRef<bool> for Helper {
    fn as_ref(&self) -> &bool {
        &self.2
    }
}

struct LifetimeHelper<'a>(&'a i32);

impl AsRef<i32> for LifetimeHelper<'static> {
    fn as_ref(&self) -> &i32 {
        self.0
    }
}

struct ConstParamHelper<const N: usize>([i32; N]);

impl AsRef<[i32]> for ConstParamHelper<0> {
    fn as_ref(&self) -> &[i32] {
        self.0.as_ref()
    }
}

trait Some {
    type Assoc;
}

impl Some for () {
    type Assoc = Helper;
}

impl Some for bool {
    type Assoc = String;
}

impl Some for u8 {
    type Assoc = i32;
}

impl Some for str {
    type Assoc = u8;
}

mod single_field {
    use super::*;

    mod tuple {
        use super::*;

        #[test]
        fn nothing() {
            #[derive(AsRef)]
            struct Nothing(String);

            let item = Nothing("test".to_owned());

            assert!(ptr::eq(item.as_ref(), &item.0));
        }

        #[test]
        fn forward() {
            #[derive(AsRef)]
            #[as_ref(forward)]
            struct Forward(String);

            let item = Forward("test".to_owned());

            let rf: &str = item.as_ref();
            assert!(ptr::eq(rf, item.0.as_ref()));
        }

        #[test]
        fn field() {
            #[derive(AsRef)]
            struct Field(#[as_ref] String);

            let item = Field("test".to_owned());

            assert!(ptr::eq(item.as_ref(), &item.0));
        }

        #[test]
        fn field_forward() {
            #[derive(AsRef)]
            struct FieldForward(#[as_ref(forward)] String);

            let item = FieldForward("test".to_owned());

            let rf: &str = item.as_ref();
            assert!(ptr::eq(rf, item.0.as_ref()));
        }

        #[test]
        fn types() {
            #[derive(AsRef)]
            #[as_ref(i32, f64)]
            struct Types(Helper);

            // Asserts that the macro expansion doesn't generate a blanket `AsRef` impl forwarding
            // to the field type, by producing  a trait implementations conflict error during
            // compilation, if it does.
            impl AsRef<bool> for Types {
                fn as_ref(&self) -> &bool {
                    self.0.as_ref()
                }
            }

            // Asserts that the macro expansion doesn't generate an `AsRef` impl for the field type,
            // by producing a trait implementations conflict error during compilation, if it does.
            impl AsRef<Helper> for Types {
                fn as_ref(&self) -> &Helper {
                    &self.0
                }
            }

            let item = Types(Helper(1, 2.0, false));

            let rf: &i32 = item.as_ref();
            assert!(ptr::eq(rf, item.0.as_ref()));

            let rf: &f64 = item.as_ref();
            assert!(ptr::eq(rf, item.0.as_ref()));
        }

        #[test]
        fn types_with_inner() {
            #[derive(AsRef)]
            #[as_ref(i32, Helper)]
            struct TypesWithInner(Helper);

            // Asserts that the macro expansion doesn't generate a blanket `AsRef` impl forwarding
            // to the field type, by producing a trait implementations conflict error during
            // compilation, if it does.
            impl AsRef<bool> for TypesWithInner {
                fn as_ref(&self) -> &bool {
                    self.0.as_ref()
                }
            }

            let item = TypesWithInner(Helper(1, 2.0, false));

            let rf: &i32 = item.as_ref();
            assert!(ptr::eq(rf, item.0.as_ref()));

            let rf: &Helper = item.as_ref();
            assert!(ptr::eq(rf, &item.0));
        }

        #[test]
        fn types_with_renamed_inner() {
            type RenamedFoo = Helper;

            #[derive(AsRef)]
            #[as_ref(i32, RenamedFoo)]
            struct TypesWithRenamedInner(Helper);

            // Asserts that the macro expansion doesn't generate a blanket `AsRef` impl forwarding
            // to the field type, by producing a trait implementations conflict error during
            // compilation, if it does.
            impl AsRef<bool> for TypesWithRenamedInner {
                fn as_ref(&self) -> &bool {
                    self.0.as_ref()
                }
            }

            let item = TypesWithRenamedInner(Helper(1, 2.0, false));

            let rf: &i32 = item.as_ref();
            assert!(ptr::eq(rf, item.0.as_ref()));

            let rf: &Helper = item.as_ref();
            assert!(ptr::eq(rf, &item.0));
        }

        #[test]
        fn field_types() {
            #[derive(AsRef)]
            struct FieldTypes(#[as_ref(i32, f64)] Helper);

            // Asserts that the macro expansion doesn't generate a blanket `AsRef` impl forwarding
            // to the field type, by producing a trait implementations conflict error during
            // compilation, if it does.
            impl AsRef<bool> for FieldTypes {
                fn as_ref(&self) -> &bool {
                    self.0.as_ref()
                }
            }

            // Asserts that the macro expansion doesn't generate an `AsRef` impl for the field type,
            // by producing a trait implementations conflict error during compilation, if it does.
            impl AsRef<Helper> for FieldTypes {
                fn as_ref(&self) -> &Helper {
                    &self.0
                }
            }

            let item = FieldTypes(Helper(1, 2.0, false));

            let rf: &i32 = item.as_ref();
            assert!(ptr::eq(rf, item.0.as_ref()));

            let rf: &f64 = item.as_ref();
            assert!(ptr::eq(rf, item.0.as_ref()));
        }

        #[test]
        fn field_types_with_inner() {
            #[derive(AsRef)]
            struct FieldTypesWithInner(#[as_ref(i32, Helper)] Helper);

            // Asserts that the macro expansion doesn't generate a blanket `AsRef` impl forwarding
            // to the field type, by producing a trait implementations conflict error during
            // compilation, if it does.
            impl AsRef<bool> for FieldTypesWithInner {
                fn as_ref(&self) -> &bool {
                    self.0.as_ref()
                }
            }

            let item = FieldTypesWithInner(Helper(1, 2.0, false));

            let rf: &i32 = item.as_ref();
            assert!(ptr::eq(rf, item.0.as_ref()));

            let rf: &Helper = item.as_ref();
            assert!(ptr::eq(rf, &item.0));
        }

        #[test]
        fn field_types_with_renamed_inner() {
            type RenamedFoo = Helper;

            #[derive(AsRef)]
            struct FieldTypesWithRenamedInner(#[as_ref(i32, RenamedFoo)] Helper);

            // Asserts that the macro expansion doesn't generate a blanket `AsRef` impl forwarding
            // to the field type, by producing a trait implementations conflict error during
            // compilation, if it does.
            impl AsRef<bool> for FieldTypesWithRenamedInner {
                fn as_ref(&self) -> &bool {
                    self.0.as_ref()
                }
            }

            let item = FieldTypesWithRenamedInner(Helper(1, 2.0, false));

            let rf: &i32 = item.as_ref();
            assert!(ptr::eq(rf, item.0.as_ref()));

            let rf: &Helper = item.as_ref();
            assert!(ptr::eq(rf, &item.0));
        }

        mod generic {
            use super::*;

            #[test]
            fn nothing() {
                #[derive(AsRef)]
                struct Nothing<T>(T);

                let item = Nothing("test".to_owned());

                assert!(ptr::eq(item.as_ref(), &item.0));
            }

            #[test]
            fn nothing_assoc() {
                #[derive(AsRef)]
                struct Nothing<T: Some>(T::Assoc);

                let item = Nothing::<bool>("test".to_owned());

                assert!(ptr::eq(item.as_ref(), &item.0));
            }

            #[test]
            fn forward() {
                #[derive(AsRef)]
                #[as_ref(forward)]
                struct Forward<T>(T);

                let item = Forward("test".to_owned());

                let rf: &str = item.as_ref();
                assert!(ptr::eq(rf, item.0.as_ref()));
            }

            #[test]
            fn forward_assoc() {
                #[derive(AsRef)]
                #[as_ref(forward)]
                struct Forward<T: Some>(T::Assoc);

                let item = Forward::<bool>("test".to_owned());

                let rf: &str = item.as_ref();
                assert!(ptr::eq(rf, item.0.as_ref()));
            }

            #[test]
            fn field() {
                #[derive(AsRef)]
                struct Field<T>(#[as_ref] T);

                let item = Field("test".to_owned());

                assert!(ptr::eq(item.as_ref(), &item.0));
            }

            #[test]
            fn field_forward() {
                #[derive(AsRef)]
                struct FieldForward<T>(#[as_ref(forward)] T);

                let item = FieldForward("test".to_owned());

                let rf: &str = item.as_ref();
                assert!(ptr::eq(rf, item.0.as_ref()));
            }

            #[test]
            fn field_forward_assoc() {
                #[derive(AsRef)]
                struct FieldForward<T: Some>(#[as_ref(forward)] T::Assoc);

                let item = FieldForward::<bool>("test".to_owned());

                let rf: &str = item.as_ref();
                assert!(ptr::eq(rf, item.0.as_ref()));
            }

            #[test]
            fn types() {
                #[derive(AsRef)]
                #[as_ref(i32, f64)]
                struct Types<T>(T);

                // Asserts that the macro expansion doesn't generate a blanket `AsRef` impl
                // forwarding to the field type, by producing a trait implementations conflict error
                // during compilation, if it does.
                impl<T: AsRef<bool>> AsRef<bool> for Types<T> {
                    fn as_ref(&self) -> &bool {
                        self.0.as_ref()
                    }
                }

                let item = Types(Helper(1, 2.0, false));

                let rf: &i32 = item.as_ref();
                assert!(ptr::eq(rf, item.0.as_ref()));

                let rf: &f64 = item.as_ref();
                assert!(ptr::eq(rf, item.0.as_ref()));
            }

            #[test]
            fn types_assoc() {
                #[derive(AsRef)]
                #[as_ref(i32, f64)]
                struct Types<T: Some>(T::Assoc);

                // Asserts that the macro expansion doesn't generate a blanket `AsRef` impl
                // forwarding to the field type, by producing a trait implementations conflict error
                // during compilation, if it does.
                impl<T: Some> AsRef<bool> for Types<T>
                where
                    T::Assoc: AsRef<bool>,
                {
                    fn as_ref(&self) -> &bool {
                        self.0.as_ref()
                    }
                }

                let item = Types::<()>(Helper(1, 2.0, false));

                let rf: &i32 = item.as_ref();
                assert!(ptr::eq(rf, item.0.as_ref()));

                let rf: &f64 = item.as_ref();
                assert!(ptr::eq(rf, item.0.as_ref()));
            }

            #[test]
            fn types_inner() {
                #[derive(AsRef)]
                #[as_ref(Vec<T>)]
                struct TypesInner<T>(Vec<T>);

                let item = TypesInner(vec![1i32]);

                assert!(ptr::eq(item.as_ref(), &item.0));
            }

            #[test]
            fn types_inner_assoc() {
                #[derive(AsRef)]
                #[as_ref(Vec<T::Assoc>)]
                struct TypesInner<T: Some>(Vec<T::Assoc>);

                let item = TypesInner::<u8>(vec![1i32]);

                assert!(ptr::eq(item.as_ref(), &item.0));
            }

            #[test]
            fn field_types() {
                #[derive(AsRef)]
                struct FieldTypes<T>(#[as_ref(i32, f64)] T);

                // Asserts that the macro expansion doesn't generate a blanket `AsRef` impl
                // forwarding to the field type, by producing a trait implementations conflict error
                // during compilation, if it does.
                impl<T: AsRef<bool>> AsRef<bool> for FieldTypes<T> {
                    fn as_ref(&self) -> &bool {
                        self.0.as_ref()
                    }
                }

                let item = FieldTypes(Helper(1, 2.0, false));

                let rf: &i32 = item.as_ref();
                assert!(ptr::eq(rf, item.0.as_ref()));

                let rf: &f64 = item.as_ref();
                assert!(ptr::eq(rf, item.0.as_ref()));
            }

            #[test]
            fn field_types_assoc() {
                #[derive(AsRef)]
                struct FieldTypes<T: Some>(#[as_ref(i32, f64)] T::Assoc);

                // Asserts that the macro expansion doesn't generate a blanket `AsRef` impl
                // forwarding to the field type, by producing a trait implementations conflict error
                // during compilation, if it does.
                impl<T: Some> AsRef<bool> for FieldTypes<T>
                where
                    T::Assoc: AsRef<bool>,
                {
                    fn as_ref(&self) -> &bool {
                        self.0.as_ref()
                    }
                }

                let item = FieldTypes::<()>(Helper(1, 2.0, false));

                let rf: &i32 = item.as_ref();
                assert!(ptr::eq(rf, item.0.as_ref()));

                let rf: &f64 = item.as_ref();
                assert!(ptr::eq(rf, item.0.as_ref()));
            }

            #[test]
            fn field_types_inner() {
                #[derive(AsRef)]
                struct FieldTypesInner<T>(#[as_ref(Vec<T>)] Vec<T>);

                let item = FieldTypesInner(vec![1i32]);

                assert!(ptr::eq(item.as_ref(), &item.0));
            }

            #[test]
            fn field_types_inner_assoc() {
                #[derive(AsRef)]
                struct FieldTypesInner<T: Some>(
                    #[as_ref(Vec<<T as Some>::Assoc>)] Vec<<T as Some>::Assoc>,
                );

                let item = FieldTypesInner::<u8>(vec![1i32]);

                assert!(ptr::eq(item.as_ref(), &item.0));
            }

            #[test]
            fn lifetime() {
                #[derive(AsRef)]
                #[as_ref(i32)]
                struct Lifetime<'a>(LifetimeHelper<'a>);

                let item = Lifetime(LifetimeHelper(&0));

                assert!(ptr::eq(item.as_ref(), item.0.as_ref()));
            }

            #[test]
            fn field_lifetime() {
                #[derive(AsRef)]
                struct FieldLifetime<'a>(#[as_ref(i32)] LifetimeHelper<'a>);

                let item = FieldLifetime(LifetimeHelper(&0));

                assert!(ptr::eq(item.as_ref(), item.0.as_ref()));
            }

            #[test]
            fn const_param() {
                #[derive(AsRef)]
                #[as_ref([i32])]
                struct ConstParam<const N: usize>(ConstParamHelper<N>);

                let item = ConstParam(ConstParamHelper([]));

                assert!(ptr::eq(item.as_ref(), item.0.as_ref()));
            }

            #[test]
            fn field_const_param() {
                #[derive(AsRef)]
                struct FieldConstParam<const N: usize>(
                    #[as_ref([i32])] ConstParamHelper<N>,
                );

                let item = FieldConstParam(ConstParamHelper([]));

                assert!(ptr::eq(item.as_ref(), item.0.as_ref()));
            }
        }

        #[cfg(nightly)]
        mod never {
            use super::*;

            #[derive(AsRef)]
            struct Nothing(!);
        }

        mod deprecated {
            use super::*;

            #[derive(AsRef)]
            #[deprecated(note = "struct")]
            struct Deprecated(#[deprecated(note = "field")] i32);
        }
    }

    mod named {
        use super::*;

        #[test]
        fn nothing() {
            #[derive(AsRef)]
            struct Nothing {
                first: String,
            }

            let item = Nothing {
                first: "test".to_owned(),
            };

            assert!(ptr::eq(item.as_ref(), &item.first));
        }

        #[test]
        fn forward() {
            #[derive(AsRef)]
            #[as_ref(forward)]
            struct Forward {
                first: String,
            }

            let item = Forward {
                first: "test".to_owned(),
            };

            let rf: &str = item.as_ref();
            assert!(ptr::eq(rf, item.first.as_ref()));
        }

        #[test]
        fn field() {
            #[derive(AsRef)]
            struct Field {
                #[as_ref]
                first: String,
            }

            let item = Field {
                first: "test".to_owned(),
            };

            assert!(ptr::eq(item.as_ref(), &item.first));
        }

        #[test]
        fn field_forward() {
            #[derive(AsRef)]
            struct FieldForward {
                #[as_ref(forward)]
                first: String,
            }

            let item = FieldForward {
                first: "test".to_owned(),
            };

            let rf: &str = item.as_ref();
            assert!(ptr::eq(rf, item.first.as_ref()));
        }

        #[test]
        fn types() {
            #[derive(AsRef)]
            #[as_ref(i32, f64)]
            struct Types {
                first: Helper,
            }

            // Asserts that the macro expansion doesn't generate a blanket `AsRef` impl forwarding
            // to the field type, by producing a trait implementations conflict error during
            // compilation, if it does.
            impl AsRef<bool> for Types {
                fn as_ref(&self) -> &bool {
                    self.first.as_ref()
                }
            }

            // Asserts that the macro expansion doesn't generate an `AsRef` impl for the field type,
            // by producing a trait implementations conflict error during compilation, if it does.
            impl AsRef<Helper> for Types {
                fn as_ref(&self) -> &Helper {
                    &self.first
                }
            }

            let item = Types {
                first: Helper(1, 2.0, false),
            };

            let rf: &i32 = item.as_ref();
            assert!(ptr::eq(rf, item.first.as_ref()));

            let rf: &f64 = item.as_ref();
            assert!(ptr::eq(rf, item.first.as_ref()));
        }

        #[test]
        fn types_with_inner() {
            #[derive(AsRef)]
            #[as_ref(i32, Helper)]
            struct TypesWithInner {
                first: Helper,
            }

            // Asserts that the macro expansion doesn't generate a blanket `AsRef` impl forwarding
            // to the field type, by producing a trait implementations conflict error during
            // compilation, if it does.
            impl AsRef<bool> for TypesWithInner {
                fn as_ref(&self) -> &bool {
                    self.first.as_ref()
                }
            }

            let item = TypesWithInner {
                first: Helper(1, 2.0, false),
            };

            let rf: &i32 = item.as_ref();
            assert!(ptr::eq(rf, item.first.as_ref()));

            let rf: &Helper = item.as_ref();
            assert!(ptr::eq(rf, &item.first));
        }

        #[test]
        fn types_with_renamed_inner() {
            type RenamedFoo = Helper;

            #[derive(AsRef)]
            #[as_ref(i32, RenamedFoo)]
            struct TypesWithRenamedInner {
                first: Helper,
            }

            // Asserts that the macro expansion doesn't generate a blanket `AsRef` impl forwarding
            // to the field type, by producing a trait implementations conflict error during
            // compilation, if it does.
            impl AsRef<bool> for TypesWithRenamedInner {
                fn as_ref(&self) -> &bool {
                    self.first.as_ref()
                }
            }

            let item = TypesWithRenamedInner {
                first: Helper(1, 2.0, false),
            };

            let rf: &i32 = item.as_ref();
            assert!(ptr::eq(rf, item.first.as_ref()));

            let rf: &Helper = item.as_ref();
            assert!(ptr::eq(rf, &item.first));
        }

        #[test]
        fn field_types() {
            #[derive(AsRef)]
            struct FieldTypes {
                #[as_ref(i32, f64)]
                first: Helper,
            }

            // Asserts that the macro expansion doesn't generate a blanket `AsRef` impl forwarding
            // to the field type, by producing a trait implementations conflict error during
            // compilation, if it does.
            impl AsRef<bool> for FieldTypes {
                fn as_ref(&self) -> &bool {
                    self.first.as_ref()
                }
            }

            // Asserts that the macro expansion doesn't generate an `AsRef` impl for the field type,
            // by producing a trait implementations conflict error during compilation, if it does.
            impl AsRef<Helper> for FieldTypes {
                fn as_ref(&self) -> &Helper {
                    &self.first
                }
            }

            let item = FieldTypes {
                first: Helper(1, 2.0, false),
            };

            let rf: &i32 = item.as_ref();
            assert!(ptr::eq(rf, item.first.as_ref()));

            let rf: &f64 = item.as_ref();
            assert!(ptr::eq(rf, item.first.as_ref()));
        }

        #[test]
        fn field_types_with_inner() {
            #[derive(AsRef)]
            struct FieldTypesWithInner {
                #[as_ref(i32, Helper)]
                first: Helper,
            }

            // Asserts that the macro expansion doesn't generate a blanket `AsRef` impl forwarding
            // to the field type, by producing a trait implementations conflict error during
            // compilation, if it does.
            impl AsRef<bool> for FieldTypesWithInner {
                fn as_ref(&self) -> &bool {
                    self.first.as_ref()
                }
            }

            let item = FieldTypesWithInner {
                first: Helper(1, 2.0, false),
            };

            let rf: &i32 = item.as_ref();
            assert!(ptr::eq(rf, item.first.as_ref()));

            let rf: &Helper = item.as_ref();
            assert!(ptr::eq(rf, &item.first));
        }

        #[test]
        fn field_types_with_renamed_inner() {
            type RenamedFoo = Helper;

            #[derive(AsRef)]
            struct FieldTypesWithRenamedInner {
                #[as_ref(i32, RenamedFoo)]
                first: Helper,
            }

            // Asserts that the macro expansion doesn't generate a blanket `AsRef` impl forwarding
            // to the field type, by producing a trait implementations conflict error during
            // compilation, if it does.
            impl AsRef<bool> for FieldTypesWithRenamedInner {
                fn as_ref(&self) -> &bool {
                    self.first.as_ref()
                }
            }

            let item = FieldTypesWithRenamedInner {
                first: Helper(1, 2.0, false),
            };

            let rf: &i32 = item.as_ref();
            assert!(ptr::eq(rf, item.first.as_ref()));

            let rf: &Helper = item.as_ref();
            assert!(ptr::eq(rf, &item.first));
        }

        mod generic {
            use super::*;

            #[test]
            fn nothing() {
                #[derive(AsRef)]
                struct Nothing<T> {
                    first: T,
                }

                let item = Nothing {
                    first: "test".to_owned(),
                };

                assert!(ptr::eq(item.as_ref(), &item.first));
            }

            #[test]
            fn nothing_assoc() {
                #[derive(AsRef)]
                struct Nothing<T: Some> {
                    first: <T as Some>::Assoc,
                }

                let item = Nothing::<bool> {
                    first: "test".to_owned(),
                };

                assert!(ptr::eq(item.as_ref(), &item.first));
            }

            #[test]
            fn forward() {
                #[derive(AsRef)]
                #[as_ref(forward)]
                struct Forward<T> {
                    first: T,
                }

                let item = Forward {
                    first: "test".to_owned(),
                };

                let rf: &str = item.as_ref();
                assert!(ptr::eq(rf, item.first.as_ref()));
            }

            #[test]
            fn forward_assoc() {
                #[derive(AsRef)]
                #[as_ref(forward)]
                struct Forward<T: Some> {
                    first: T::Assoc,
                }

                let item = Forward::<bool> {
                    first: "test".to_owned(),
                };

                let rf: &str = item.as_ref();
                assert!(ptr::eq(rf, item.first.as_ref()));
            }

            #[test]
            fn field() {
                #[derive(AsRef)]
                struct Field<T> {
                    #[as_ref]
                    first: T,
                }

                let item = Field {
                    first: "test".to_owned(),
                };

                assert!(ptr::eq(item.as_ref(), &item.first));
            }

            #[test]
            fn field_forward() {
                #[derive(AsRef)]
                struct FieldForward<T> {
                    #[as_ref(forward)]
                    first: T,
                }

                let item = FieldForward {
                    first: "test".to_owned(),
                };

                let rf: &str = item.as_ref();
                assert!(ptr::eq(rf, item.first.as_ref()));
            }

            #[test]
            fn field_forward_assoc() {
                #[derive(AsRef)]
                struct FieldForward<T: Some> {
                    #[as_ref(forward)]
                    first: T::Assoc,
                }

                let item = FieldForward::<bool> {
                    first: "test".to_owned(),
                };

                let rf: &str = item.as_ref();
                assert!(ptr::eq(rf, item.first.as_ref()));
            }

            #[test]
            fn types() {
                #[derive(AsRef)]
                #[as_ref(i32, f64)]
                struct Types<T> {
                    first: T,
                }

                // Asserts that the macro expansion doesn't generate a blanket `AsRef` impl
                // forwarding to the field type, by producing a trait implementations conflict error
                // during compilation, if it does.
                impl<T: AsRef<bool>> AsRef<bool> for Types<T> {
                    fn as_ref(&self) -> &bool {
                        self.first.as_ref()
                    }
                }

                let item = Types {
                    first: Helper(1, 2.0, false),
                };

                let rf: &i32 = item.as_ref();
                assert!(ptr::eq(rf, item.first.as_ref()));

                let rf: &f64 = item.as_ref();
                assert!(ptr::eq(rf, item.first.as_ref()));
            }

            #[test]
            fn types_assoc() {
                #[derive(AsRef)]
                #[as_ref(i32, f64)]
                struct Types<T: Some> {
                    first: T::Assoc,
                }

                // Asserts that the macro expansion doesn't generate a blanket `AsRef` impl
                // forwarding to the field type, by producing a trait implementations conflict error
                // during compilation, if it does.
                impl<T: Some> AsRef<bool> for Types<T>
                where
                    T::Assoc: AsRef<bool>,
                {
                    fn as_ref(&self) -> &bool {
                        self.first.as_ref()
                    }
                }

                let item = Types::<()> {
                    first: Helper(1, 2.0, false),
                };

                let rf: &i32 = item.as_ref();
                assert!(ptr::eq(rf, item.first.as_ref()));

                let rf: &f64 = item.as_ref();
                assert!(ptr::eq(rf, item.first.as_ref()));
            }

            #[test]
            fn types_inner() {
                #[derive(AsRef)]
                #[as_ref(Vec<T>)]
                struct TypesInner<T> {
                    first: Vec<T>,
                }

                let item = TypesInner { first: vec![1i32] };

                assert!(ptr::eq(item.as_ref(), &item.first));
            }

            #[test]
            fn types_inner_assoc() {
                #[derive(AsRef)]
                #[as_ref(Vec<T::Assoc>)]
                struct TypesInner<T: Some> {
                    first: Vec<T::Assoc>,
                }

                let item = TypesInner::<u8> { first: vec![1i32] };

                assert!(ptr::eq(item.as_ref(), &item.first));
            }

            #[test]
            fn field_types() {
                #[derive(AsRef)]
                struct FieldTypes<T> {
                    #[as_ref(i32, f64)]
                    first: T,
                }

                // Asserts that the macro expansion doesn't generate a blanket `AsRef` impl
                // forwarding to the field type, by producing a trait implementations conflict error
                // during compilation, if it does.
                impl<T: AsRef<bool>> AsRef<bool> for FieldTypes<T> {
                    fn as_ref(&self) -> &bool {
                        self.first.as_ref()
                    }
                }

                let item = FieldTypes {
                    first: Helper(1, 2.0, false),
                };

                let rf: &i32 = item.as_ref();
                assert!(ptr::eq(rf, item.first.as_ref()));

                let rf: &f64 = item.as_ref();
                assert!(ptr::eq(rf, item.first.as_ref()));
            }

            #[test]
            fn field_types_assoc() {
                #[derive(AsRef)]
                struct FieldTypes<T: Some> {
                    #[as_ref(i32, f64)]
                    first: T::Assoc,
                }

                // Asserts that the macro expansion doesn't generate a blanket `AsRef` impl
                // forwarding to the field type, by producing a trait implementations conflict error
                // during compilation, if it does.
                impl<T: Some> AsRef<bool> for FieldTypes<T>
                where
                    T::Assoc: AsRef<bool>,
                {
                    fn as_ref(&self) -> &bool {
                        self.first.as_ref()
                    }
                }

                let item = FieldTypes::<()> {
                    first: Helper(1, 2.0, false),
                };

                let rf: &i32 = item.as_ref();
                assert!(ptr::eq(rf, item.first.as_ref()));

                let rf: &f64 = item.as_ref();
                assert!(ptr::eq(rf, item.first.as_ref()));
            }

            #[test]
            fn field_types_inner() {
                #[derive(AsRef)]
                struct FieldTypesInner<T> {
                    #[as_ref(Vec<T>)]
                    first: Vec<T>,
                }

                let item = FieldTypesInner { first: vec![1i32] };

                assert!(ptr::eq(item.as_ref(), &item.first));
            }

            #[test]
            fn field_types_inner_assoc() {
                #[derive(AsRef)]
                struct FieldTypesInner<T: Some> {
                    #[as_ref(Vec<<T as Some>::Assoc>)]
                    first: Vec<<T as Some>::Assoc>,
                }

                let item = FieldTypesInner::<u8> { first: vec![1i32] };

                assert!(ptr::eq(item.as_ref(), &item.first));
            }

            #[test]
            fn lifetime() {
                #[derive(AsRef)]
                #[as_ref(i32)]
                struct Lifetime<'a> {
                    first: LifetimeHelper<'a>,
                }

                let item = Lifetime {
                    first: LifetimeHelper(&0),
                };

                assert!(ptr::eq(item.as_ref(), item.first.as_ref()));
            }

            #[test]
            fn field_lifetime() {
                #[derive(AsRef)]
                struct FieldLifetime<'a> {
                    #[as_ref(i32)]
                    first: LifetimeHelper<'a>,
                }

                let item = FieldLifetime {
                    first: LifetimeHelper(&0),
                };

                assert!(ptr::eq(item.as_ref(), item.first.as_ref()));
            }

            #[test]
            fn const_param() {
                #[derive(AsRef)]
                #[as_ref([i32])]
                struct ConstParam<const N: usize> {
                    first: ConstParamHelper<N>,
                }

                let item = ConstParam {
                    first: ConstParamHelper([]),
                };

                assert!(ptr::eq(item.as_ref(), item.first.as_ref()));
            }

            #[test]
            fn field_const_param() {
                #[derive(AsRef)]
                struct FieldConstParam<const N: usize> {
                    #[as_ref([i32])]
                    first: ConstParamHelper<N>,
                }

                let item = FieldConstParam {
                    first: ConstParamHelper([]),
                };

                assert!(ptr::eq(item.as_ref(), item.first.as_ref()));
            }
        }

        #[cfg(nightly)]
        mod never {
            use super::*;

            #[derive(AsRef)]
            struct Nothing {
                first: !,
            }
        }

        mod deprecated {
            use super::*;

            #[derive(AsRef)]
            #[deprecated(note = "struct")]
            struct Deprecated {
                #[deprecated(note = "field")]
                field: i32,
            }
        }
    }
}

mod multi_field {
    use super::*;

    mod tuple {
        use super::*;

        #[test]
        fn nothing() {
            #[derive(AsRef)]
            struct Nothing(String, i32);

            let item = Nothing("test".to_owned(), 0);

            assert!(ptr::eq(item.as_ref(), &item.0));
            assert!(ptr::eq(item.as_ref(), &item.1));
        }

        #[test]
        fn skip() {
            #[derive(AsRef)]
            struct Skip(String, i32, #[as_ref(skip)] f64);

            // Asserts that the macro expansion doesn't generate `AsRef` impl for the skipped field,
            // by producing trait implementations conflict error during compilation, if it does.
            impl AsRef<f64> for Skip {
                fn as_ref(&self) -> &f64 {
                    &self.2
                }
            }

            let item = Skip("test".to_owned(), 0, 0.0);

            assert!(ptr::eq(item.as_ref(), &item.0));
            assert!(ptr::eq(item.as_ref(), &item.1));
        }

        #[test]
        fn field() {
            #[derive(AsRef)]
            struct Field(#[as_ref] String, #[as_ref] i32, f64);

            // Asserts that the macro expansion doesn't generate `AsRef` impl for the third field,
            // by producing trait implementations conflict error during compilation, if it does.
            impl AsRef<f64> for Field {
                fn as_ref(&self) -> &f64 {
                    &self.2
                }
            }

            let item = Field("test".to_owned(), 0, 0.0);

            assert!(ptr::eq(item.as_ref(), &item.0));
            assert!(ptr::eq(item.as_ref(), &item.1));
        }

        #[test]
        fn field_forward() {
            #[derive(AsRef)]
            struct FieldForward(#[as_ref(forward)] String, i32);

            let item = FieldForward("test".to_owned(), 0);

            let rf: &str = item.as_ref();
            assert!(ptr::eq(rf, item.0.as_ref()));
        }

        #[test]
        fn types() {
            type RenamedString = String;

            #[derive(AsRef)]
            struct Types(
                #[as_ref(str, RenamedString)] String,
                #[as_ref([u8])] Vec<u8>,
            );

            // Asserts that the macro expansion doesn't generate `AsRef` impl for the field type, by
            // producing trait implementations conflict error during compilation, if it does.
            impl AsRef<Vec<u8>> for Types {
                fn as_ref(&self) -> &Vec<u8> {
                    &self.1
                }
            }

            let item = Types("test".to_owned(), vec![0]);

            let rf: &str = item.as_ref();
            assert!(ptr::eq(rf, item.0.as_ref()));

            let rf: &String = item.as_ref();
            assert!(ptr::eq(rf, &item.0));

            let rf: &[u8] = item.as_ref();
            assert!(ptr::eq(rf, item.1.as_ref()));
        }

        mod generic {
            use super::*;

            #[test]
            fn nothing() {
                #[derive(AsRef)]
                struct Nothing<T, U>(Vec<T>, VecDeque<U>);

                let item = Nothing(vec![1], VecDeque::from([2]));

                assert!(ptr::eq(item.as_ref(), &item.0));
                assert!(ptr::eq(item.as_ref(), &item.1));
            }

            #[test]
            fn skip() {
                #[derive(AsRef)]
                struct Skip<T, U, V>(Vec<T>, VecDeque<U>, #[as_ref(skip)] V);

                let item = Skip(vec![1], VecDeque::from([2]), 0);

                assert!(ptr::eq(item.as_ref(), &item.0));
                assert!(ptr::eq(item.as_ref(), &item.1));
            }

            #[test]
            fn field() {
                #[derive(AsRef)]
                struct Field<T, U, V>(#[as_ref] Vec<T>, #[as_ref] VecDeque<U>, V);

                let item = Field(vec![1], VecDeque::from([2]), 0);

                assert!(ptr::eq(item.as_ref(), &item.0));
                assert!(ptr::eq(item.as_ref(), &item.1));
            }

            #[test]
            fn field_forward() {
                #[derive(AsRef)]
                struct FieldForward<T, U>(#[as_ref(forward)] T, U);

                let item = FieldForward("test".to_owned(), 0);

                let rf: &str = item.as_ref();
                assert!(ptr::eq(rf, item.0.as_ref()));
            }

            #[test]
            fn field_forward_assoc() {
                #[derive(AsRef)]
                struct FieldForward<T: Some, U>(#[as_ref(forward)] T::Assoc, U);

                let item = FieldForward::<bool, _>("test".to_owned(), 0);

                let rf: &str = item.as_ref();
                assert!(ptr::eq(rf, item.0.as_ref()));
            }

            #[test]
            fn types() {
                #[derive(AsRef)]
                struct Types<T, U>(#[as_ref(str)] T, #[as_ref([u8])] U);

                let item = Types("test".to_owned(), vec![0u8]);

                let rf: &str = item.as_ref();
                assert!(ptr::eq(rf, item.0.as_ref()));

                let rf: &[u8] = item.as_ref();
                assert!(ptr::eq(rf, item.1.as_ref()));
            }

            #[test]
            fn types_assoc() {
                #[derive(AsRef)]
                struct Types<T: Some, U: Some + ?Sized>(
                    #[as_ref(str)] <T as Some>::Assoc,
                    #[as_ref([u8])] Vec<U::Assoc>,
                );

                let item = Types::<bool, str>("test".to_owned(), vec![0u8]);

                let rf: &str = item.as_ref();
                assert!(ptr::eq(rf, item.0.as_ref()));

                let rf: &[u8] = item.as_ref();
                assert!(ptr::eq(rf, item.1.as_ref()));
            }

            #[test]
            fn types_with_inner() {
                #[derive(AsRef)]
                struct TypesWithInner<T, U>(
                    #[as_ref(Vec<T>, [T])] Vec<T>,
                    #[as_ref(str)] U,
                );

                let item = TypesWithInner(vec![1i32], "a".to_owned());

                let rf: &Vec<i32> = item.as_ref();
                assert!(ptr::eq(rf, &item.0));

                let rf: &[i32] = item.as_ref();
                assert!(ptr::eq(rf, item.0.as_ref()));

                let rf: &str = item.as_ref();
                assert!(ptr::eq(rf, item.1.as_ref()));
            }

            #[test]
            fn types_with_inner_assoc() {
                #[derive(AsRef)]
                struct TypesWithInner<T: Some, U: Some>(
                    #[as_ref(Vec<T::Assoc>, [T::Assoc])] Vec<T::Assoc>,
                    #[as_ref(str)] U::Assoc,
                );

                let item = TypesWithInner::<u8, bool>(vec![1i32], "a".to_owned());

                let rf: &Vec<i32> = item.as_ref();
                assert!(ptr::eq(rf, &item.0));

                let rf: &[i32] = item.as_ref();
                assert!(ptr::eq(rf, item.0.as_ref()));

                let rf: &str = item.as_ref();
                assert!(ptr::eq(rf, item.1.as_ref()));
            }

            #[test]
            fn field_non_generic() {
                #[derive(AsRef)]
                struct FieldNonGeneric<T>(#[as_ref([T])] Vec<i32>, T);

                let item = FieldNonGeneric(vec![], 2i32);

                assert!(ptr::eq(item.as_ref(), item.0.as_ref()));
            }

            #[test]
            fn field_non_generic_assoc() {
                #[derive(AsRef)]
                struct FieldNonGeneric<T: Some>(#[as_ref([T::Assoc])] Vec<i32>, T);

                let item = FieldNonGeneric::<u8>(vec![], 2u8);

                assert!(ptr::eq(item.as_ref(), item.0.as_ref()));
            }
        }

        #[cfg(nightly)]
        mod never {
            use super::*;

            #[derive(AsRef)]
            struct Nothing(String, !);
        }
    }

    mod named {
        use super::*;

        #[test]
        fn nothing() {
            #[derive(AsRef)]
            struct Nothing {
                first: String,
                second: i32,
            }

            let item = Nothing {
                first: "test".to_owned(),
                second: 0,
            };

            assert!(ptr::eq(item.as_ref(), &item.first));
            assert!(ptr::eq(item.as_ref(), &item.second));
        }

        #[test]
        fn skip() {
            #[derive(AsRef)]
            struct Skip {
                first: String,
                second: i32,
                #[as_ref(skip)]
                third: f64,
            }

            // Asserts that the macro expansion doesn't generate `AsRef` impl for the skipped field,
            // by producing trait implementations conflict error during compilation, if it does.
            impl AsRef<f64> for Skip {
                fn as_ref(&self) -> &f64 {
                    &self.third
                }
            }

            let item = Skip {
                first: "test".to_owned(),
                second: 0,
                third: 0.0,
            };

            assert!(ptr::eq(item.as_ref(), &item.first));
            assert!(ptr::eq(item.as_ref(), &item.second));
        }

        #[test]
        fn field() {
            #[derive(AsRef)]
            struct Field {
                #[as_ref]
                first: String,
                #[as_ref]
                second: i32,
                third: f64,
            }

            // Asserts that the macro expansion doesn't generate `AsRef` impl for the `third` field,
            // by producing trait implementations conflict error during compilation, if it does.
            impl AsRef<f64> for Field {
                fn as_ref(&self) -> &f64 {
                    &self.third
                }
            }

            let item = Field {
                first: "test".to_owned(),
                second: 0,
                third: 0.0,
            };

            assert!(ptr::eq(item.as_ref(), &item.first));
            assert!(ptr::eq(item.as_ref(), &item.second));
        }

        #[test]
        fn field_forward() {
            #[derive(AsRef)]
            struct FieldForward {
                #[as_ref(forward)]
                first: String,
                second: i32,
            }

            let item = FieldForward {
                first: "test".to_owned(),
                second: 0,
            };

            let rf: &str = item.as_ref();
            assert!(ptr::eq(rf, item.first.as_ref()));
        }

        #[test]
        fn types() {
            type RenamedString = String;

            #[derive(AsRef)]
            struct Types {
                #[as_ref(str, RenamedString)]
                first: String,
                #[as_ref([u8])]
                second: Vec<u8>,
            }

            // Asserts that the macro expansion doesn't generate `AsRef` impl for the field type, by
            // producing trait implementations conflict error during compilation, if it does.
            impl AsRef<Vec<u8>> for Types {
                fn as_ref(&self) -> &Vec<u8> {
                    &self.second
                }
            }

            let item = Types {
                first: "test".to_owned(),
                second: vec![0u8],
            };

            let rf: &str = item.as_ref();
            assert!(ptr::eq(rf, item.first.as_ref()));

            let rf: &String = item.as_ref();
            assert!(ptr::eq(rf, &item.first));

            let rf: &[u8] = item.as_ref();
            assert!(ptr::eq(rf, item.second.as_ref()));
        }

        mod generic {
            use super::*;

            #[test]
            fn nothing() {
                #[derive(AsRef)]
                struct Nothing<T, U> {
                    first: Vec<T>,
                    second: VecDeque<U>,
                }

                let item = Nothing {
                    first: vec![1],
                    second: VecDeque::from([2]),
                };

                assert!(ptr::eq(item.as_ref(), &item.first));
                assert!(ptr::eq(item.as_ref(), &item.second));
            }

            #[test]
            fn skip() {
                #[derive(AsRef)]
                struct Skip<T, U, V> {
                    first: Vec<T>,
                    second: VecDeque<U>,
                    #[as_ref(skip)]
                    third: V,
                }

                let item = Skip {
                    first: vec![1],
                    second: VecDeque::from([2]),
                    third: 0,
                };

                assert!(ptr::eq(item.as_ref(), &item.first));
                assert!(ptr::eq(item.as_ref(), &item.second));
            }

            #[test]
            fn field() {
                #[derive(AsRef)]
                struct Field<T, U, V> {
                    #[as_ref]
                    first: Vec<T>,
                    #[as_ref]
                    second: VecDeque<U>,
                    third: V,
                }

                let item = Field {
                    first: vec![1],
                    second: VecDeque::from([2]),
                    third: 0,
                };

                assert!(ptr::eq(item.as_ref(), &item.first));
                assert!(ptr::eq(item.as_ref(), &item.second));
            }

            #[test]
            fn field_forward() {
                #[derive(AsRef)]
                struct FieldForward<T, U> {
                    #[as_ref(forward)]
                    first: T,
                    second: U,
                }

                let item = FieldForward {
                    first: "test".to_owned(),
                    second: 0,
                };

                let rf: &str = item.as_ref();
                assert!(ptr::eq(rf, item.first.as_ref()));
            }

            #[test]
            fn field_forward_assoc() {
                #[derive(AsRef)]
                struct FieldForward<T: Some, U> {
                    #[as_ref(forward)]
                    first: T::Assoc,
                    second: U,
                }

                let item = FieldForward::<bool, _> {
                    first: "test".to_owned(),
                    second: 0,
                };

                let rf: &str = item.as_ref();
                assert!(ptr::eq(rf, item.first.as_ref()));
            }

            #[test]
            fn types() {
                #[derive(AsRef)]
                struct Types<T, U> {
                    #[as_ref(str)]
                    first: T,
                    #[as_ref([u8])]
                    second: U,
                }

                let item = Types {
                    first: "test".to_owned(),
                    second: vec![0u8],
                };

                let rf: &str = item.as_ref();
                assert!(ptr::eq(rf, item.first.as_ref()));

                let rf: &[u8] = item.as_ref();
                assert!(ptr::eq(rf, item.second.as_ref()));
            }

            #[test]
            fn types_assoc() {
                #[derive(AsRef)]
                struct Types<T: Some, U: Some + ?Sized> {
                    #[as_ref(str)]
                    first: T::Assoc,
                    #[as_ref([u8])]
                    second: Vec<<U as Some>::Assoc>,
                }

                let item = Types::<bool, str> {
                    first: "test".to_owned(),
                    second: vec![0u8],
                };

                let rf: &str = item.as_ref();
                assert!(ptr::eq(rf, item.first.as_ref()));

                let rf: &[u8] = item.as_ref();
                assert!(ptr::eq(rf, item.second.as_ref()));
            }

            #[test]
            fn types_with_inner() {
                #[derive(AsRef)]
                struct TypesWithInner<T, U> {
                    #[as_ref(Vec<T>, [T])]
                    first: Vec<T>,
                    #[as_ref(str)]
                    second: U,
                }

                let item = TypesWithInner {
                    first: vec![1i32],
                    second: "a".to_owned(),
                };

                let rf: &Vec<i32> = item.as_ref();
                assert!(ptr::eq(rf, &item.first));

                let rf: &[i32] = item.as_ref();
                assert!(ptr::eq(rf, item.first.as_ref()));

                let rf: &str = item.as_ref();
                assert!(ptr::eq(rf, item.second.as_ref()));
            }

            #[test]
            fn types_with_inner_assoc() {
                #[derive(AsRef)]
                struct TypesWithInner<T: Some, U: Some> {
                    #[as_ref(Vec<<T as Some>::Assoc>, [<T as Some>::Assoc])]
                    first: Vec<<T as Some>::Assoc>,
                    #[as_ref(str)]
                    second: U::Assoc,
                }

                let item = TypesWithInner::<u8, bool> {
                    first: vec![1i32],
                    second: "a".to_owned(),
                };

                let rf: &Vec<i32> = item.as_ref();
                assert!(ptr::eq(rf, &item.first));

                let rf: &[i32] = item.as_ref();
                assert!(ptr::eq(rf, item.first.as_ref()));

                let rf: &str = item.as_ref();
                assert!(ptr::eq(rf, item.second.as_ref()));
            }

            #[test]
            fn field_non_generic() {
                #[derive(AsRef)]
                struct FieldNonGeneric<T> {
                    #[as_ref([T])]
                    first: Vec<i32>,
                    second: T,
                }

                let item = FieldNonGeneric {
                    first: vec![],
                    second: 2i32,
                };

                assert!(ptr::eq(item.as_ref(), item.first.as_ref()));
            }

            #[test]
            fn field_non_generic_assoc() {
                #[derive(AsRef)]
                struct FieldNonGeneric<T: Some> {
                    #[as_ref([<T as Some>::Assoc])]
                    first: Vec<i32>,
                    second: T,
                }

                let item = FieldNonGeneric::<u8> {
                    first: vec![],
                    second: 2u8,
                };

                assert!(ptr::eq(item.as_ref(), item.first.as_ref()));
            }
        }

        #[cfg(nightly)]
        mod never {
            use super::*;

            #[derive(AsRef)]
            struct Nothing {
                first: !,
                second: i32,
            }
        }
    }
}
