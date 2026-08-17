#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)] // some code is tested for type checking only

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::string::ToString;
use core::{convert::Infallible, marker::PhantomData};

use derive_more::with_trait::FromStr;

trait Some {
    type Assoc;
}

impl<T> Some for T {
    type Assoc = i32;
}

mod structs {
    use super::*;

    mod forward {
        use super::*;

        #[test]
        fn unnamed() {
            #[derive(Debug, Eq, FromStr, PartialEq)]
            struct Int(i32);

            assert_eq!("3".parse::<Int>().unwrap(), Int(3));
            assert_eq!("0".parse::<Int>().unwrap(), Int(0));
            assert_eq!("2147483647".parse::<Int>().unwrap(), Int(i32::MAX));
            assert_eq!("-2147483648".parse::<Int>().unwrap(), Int(i32::MIN));

            assert_eq!(
                "2147483648".parse::<Int>().unwrap_err().to_string(),
                "number too large to fit in target type",
            );
            assert_eq!(
                "-2147483649".parse::<Int>().unwrap_err().to_string(),
                "number too small to fit in target type",
            );
            assert_eq!(
                "wow".parse::<Int>().unwrap_err().to_string(),
                "invalid digit found in string",
            );
        }

        #[test]
        fn named() {
            #[derive(Debug, Eq, FromStr, PartialEq)]
            struct Point1D {
                x: i32,
            }

            assert_eq!("3".parse::<Point1D>().unwrap(), Point1D { x: 3 });
            assert_eq!("0".parse::<Point1D>().unwrap(), Point1D { x: 0 });
            assert_eq!(
                "2147483647".parse::<Point1D>().unwrap(),
                Point1D { x: i32::MAX },
            );
            assert_eq!(
                "-2147483648".parse::<Point1D>().unwrap(),
                Point1D { x: i32::MIN },
            );

            assert_eq!(
                "2147483648".parse::<Point1D>().unwrap_err().to_string(),
                "number too large to fit in target type",
            );
            assert_eq!(
                "-2147483649".parse::<Point1D>().unwrap_err().to_string(),
                "number too small to fit in target type",
            );
            assert_eq!(
                "wow".parse::<Point1D>().unwrap_err().to_string(),
                "invalid digit found in string",
            );
        }

        mod generic {
            use super::*;

            #[test]
            fn unnamed() {
                #[derive(Debug, Eq, FromStr, PartialEq)]
                struct Int<I>(I);

                assert_eq!("3".parse::<Int<i32>>().unwrap(), Int(3));
                assert_eq!("0".parse::<Int<i32>>().unwrap(), Int(0));
                assert_eq!("2147483647".parse::<Int<i32>>().unwrap(), Int(i32::MAX));
                assert_eq!("-2147483648".parse::<Int<i32>>().unwrap(), Int(i32::MIN));

                assert_eq!(
                    "2147483648".parse::<Int<i32>>().unwrap_err().to_string(),
                    "number too large to fit in target type",
                );
                assert_eq!(
                    "-2147483649".parse::<Int<i32>>().unwrap_err().to_string(),
                    "number too small to fit in target type",
                );
                assert_eq!(
                    "wow".parse::<Int<i32>>().unwrap_err().to_string(),
                    "invalid digit found in string",
                );
            }

            #[test]
            fn named() {
                #[derive(Debug, Eq, FromStr, PartialEq)]
                struct Point1D<I> {
                    x: I,
                }

                assert_eq!("3".parse::<Point1D<i32>>().unwrap(), Point1D { x: 3 });
                assert_eq!("0".parse::<Point1D<i32>>().unwrap(), Point1D { x: 0 });
                assert_eq!(
                    "2147483647".parse::<Point1D<i32>>().unwrap(),
                    Point1D { x: i32::MAX },
                );
                assert_eq!(
                    "-2147483648".parse::<Point1D<i32>>().unwrap(),
                    Point1D { x: i32::MIN },
                );

                assert_eq!(
                    "2147483648"
                        .parse::<Point1D<i32>>()
                        .unwrap_err()
                        .to_string(),
                    "number too large to fit in target type",
                );
                assert_eq!(
                    "-2147483649"
                        .parse::<Point1D<i32>>()
                        .unwrap_err()
                        .to_string(),
                    "number too small to fit in target type",
                );
                assert_eq!(
                    "wow".parse::<Point1D<i32>>().unwrap_err().to_string(),
                    "invalid digit found in string",
                );
            }

            #[test]
            fn assoc() {
                #[derive(Debug, Eq, FromStr, PartialEq)]
                struct Point1D<I: Some> {
                    x: I::Assoc,
                }

                assert_eq!("3".parse::<Point1D<()>>().unwrap(), Point1D { x: 3 });
                assert_eq!("0".parse::<Point1D<()>>().unwrap(), Point1D { x: 0 });
                assert_eq!(
                    "2147483647".parse::<Point1D<()>>().unwrap(),
                    Point1D { x: i32::MAX },
                );
                assert_eq!(
                    "-2147483648".parse::<Point1D<()>>().unwrap(),
                    Point1D { x: i32::MIN },
                );

                assert_eq!(
                    "2147483648".parse::<Point1D<()>>().unwrap_err().to_string(),
                    "number too large to fit in target type",
                );
                assert_eq!(
                    "-2147483649"
                        .parse::<Point1D<()>>()
                        .unwrap_err()
                        .to_string(),
                    "number too small to fit in target type",
                );
                assert_eq!(
                    "wow".parse::<Point1D<()>>().unwrap_err().to_string(),
                    "invalid digit found in string",
                );
            }

            #[test]
            fn lifetime() {
                #[derive(Debug, Eq, PartialEq)]
                struct Empty<'a>(PhantomData<&'a i32>);

                impl FromStr for Empty<'_> {
                    type Err = Infallible;

                    fn from_str(_: &str) -> Result<Self, Self::Err> {
                        Ok(Self(PhantomData))
                    }
                }

                #[derive(Debug, Eq, FromStr, PartialEq)]
                struct Wrapper<'a>(Empty<'a>);

                assert_eq!(
                    "3".parse::<Wrapper>().unwrap(),
                    Wrapper(Empty(PhantomData)),
                );
            }

            #[test]
            fn const_param() {
                #[derive(Debug, Eq, FromStr, PartialEq)]
                struct Int<const N: usize>(i32);

                #[derive(Debug, Eq, FromStr, PartialEq)]
                struct Point1D<const N: usize> {
                    x: Int<N>,
                }

                assert_eq!("3".parse::<Int<1>>().unwrap(), Int(3));
                assert_eq!("0".parse::<Int<1>>().unwrap(), Int(0));
                assert_eq!("2147483647".parse::<Int<1>>().unwrap(), Int(i32::MAX));
                assert_eq!("-2147483648".parse::<Int<1>>().unwrap(), Int(i32::MIN));

                assert_eq!(
                    "2147483648".parse::<Int<1>>().unwrap_err().to_string(),
                    "number too large to fit in target type",
                );
                assert_eq!(
                    "-2147483649".parse::<Int<1>>().unwrap_err().to_string(),
                    "number too small to fit in target type",
                );
                assert_eq!(
                    "wow".parse::<Int<1>>().unwrap_err().to_string(),
                    "invalid digit found in string",
                );

                assert_eq!("3".parse::<Point1D<3>>().unwrap(), Point1D { x: Int(3) });
                assert_eq!("0".parse::<Point1D<3>>().unwrap(), Point1D { x: Int(0) });
                assert_eq!(
                    "2147483647".parse::<Point1D<3>>().unwrap(),
                    Point1D { x: Int(i32::MAX) },
                );
                assert_eq!(
                    "-2147483648".parse::<Point1D<3>>().unwrap(),
                    Point1D { x: Int(i32::MIN) },
                );

                assert_eq!(
                    "2147483648".parse::<Point1D<3>>().unwrap_err().to_string(),
                    "number too large to fit in target type",
                );
                assert_eq!(
                    "-2147483649".parse::<Point1D<3>>().unwrap_err().to_string(),
                    "number too small to fit in target type",
                );
                assert_eq!(
                    "wow".parse::<Point1D<3>>().unwrap_err().to_string(),
                    "invalid digit found in string",
                );
            }
        }

        mod custom_error {
            use core::num::ParseIntError;

            use super::*;

            #[derive(Debug, PartialEq)]
            struct CustomError(ParseIntError);

            impl CustomError {
                fn new(err: ParseIntError) -> Self {
                    Self(err)
                }
            }

            impl From<ParseIntError> for CustomError {
                fn from(value: ParseIntError) -> Self {
                    Self(value)
                }
            }

            fn custom_error_fn() -> fn(ParseIntError) -> CustomError {
                CustomError::new
            }

            mod unnamed {
                use super::*;

                #[test]
                fn only_ty() {
                    #[derive(Debug, FromStr)]
                    #[from_str(error(CustomError))]
                    struct MyInt(i32);

                    assert_eq!(
                        "foo".parse::<MyInt>().unwrap_err(),
                        CustomError("foo".parse::<i32>().unwrap_err()),
                    );
                }

                #[test]
                fn with_fn() {
                    #[derive(Debug, FromStr)]
                    #[from_str(error(CustomError, CustomError::new))]
                    struct MyInt(i32);

                    assert_eq!(
                        "foo".parse::<MyInt>().unwrap_err(),
                        CustomError("foo".parse::<i32>().unwrap_err()),
                    );
                }

                #[expect(clippy::redundant_closure, reason = "intended for testing")]
                #[test]
                fn with_closure() {
                    #[derive(Debug, FromStr)]
                    #[from_str(error(CustomError, |e| CustomError::new(e)))]
                    struct MyInt(i32);

                    assert_eq!(
                        "foo".parse::<MyInt>().unwrap_err(),
                        CustomError("foo".parse::<i32>().unwrap_err()),
                    );
                }

                #[test]
                fn with_fn_call() {
                    #[derive(Debug, FromStr)]
                    #[from_str(error(CustomError, custom_error_fn()))]
                    struct MyInt(i32);

                    assert_eq!(
                        "foo".parse::<MyInt>().unwrap_err(),
                        CustomError("foo".parse::<i32>().unwrap_err()),
                    );
                }
            }

            mod named {
                use super::*;

                #[test]
                fn only_ty() {
                    #[derive(Debug, FromStr)]
                    #[from_str(error(CustomError))]
                    struct MyInt {
                        value: i32,
                    }

                    assert_eq!(
                        "foo".parse::<MyInt>().unwrap_err(),
                        CustomError("foo".parse::<i32>().unwrap_err()),
                    );
                }

                #[test]
                fn with_fn() {
                    #[derive(Debug, FromStr)]
                    #[from_str(error(CustomError, CustomError::new))]
                    struct MyInt {
                        value: i32,
                    }

                    assert_eq!(
                        "foo".parse::<MyInt>().unwrap_err(),
                        CustomError("foo".parse::<i32>().unwrap_err()),
                    );
                }

                #[expect(clippy::redundant_closure, reason = "intended for testing")]
                #[test]
                fn with_closure() {
                    #[derive(Debug, FromStr)]
                    #[from_str(error(CustomError, |e| CustomError::new(e)))]
                    struct MyInt {
                        value: i32,
                    }

                    assert_eq!(
                        "foo".parse::<MyInt>().unwrap_err(),
                        CustomError("foo".parse::<i32>().unwrap_err()),
                    );
                }

                #[test]
                fn with_fn_call() {
                    #[derive(Debug, FromStr)]
                    #[from_str(error(CustomError, custom_error_fn()))]
                    struct MyInt {
                        value: i32,
                    }

                    assert_eq!(
                        "foo".parse::<MyInt>().unwrap_err(),
                        CustomError("foo".parse::<i32>().unwrap_err()),
                    );
                }
            }
        }
    }

    mod flat {
        use super::*;

        #[test]
        fn unit() {
            #[derive(Debug, Eq, FromStr, PartialEq)]
            struct Foo;

            assert_eq!("Foo".parse::<Foo>().unwrap(), Foo);
        }

        #[test]
        fn empty_tuple() {
            #[derive(Debug, Eq, FromStr, PartialEq)]
            struct Bar();

            assert_eq!("Bar".parse::<Bar>().unwrap(), Bar());
        }

        #[test]
        fn empty_struct() {
            #[derive(Debug, Eq, FromStr, PartialEq)]
            struct Baz {}

            assert_eq!("Baz".parse::<Baz>().unwrap(), Baz {});
        }

        #[test]
        fn case_insensitive() {
            #[derive(Debug, Eq, FromStr, PartialEq)]
            struct Foo;

            assert_eq!("Foo".parse::<Foo>().unwrap(), Foo);
            assert_eq!("FOO".parse::<Foo>().unwrap(), Foo);
            assert_eq!("foo".parse::<Foo>().unwrap(), Foo);

            assert_eq!(
                "baz".parse::<Foo>().unwrap_err().to_string(),
                "Invalid `Foo` string representation",
            );
            assert_eq!(
                "other".parse::<Foo>().unwrap_err().to_string(),
                "Invalid `Foo` string representation",
            );
        }

        mod rename_all {
            use super::*;

            #[test]
            fn case_sensitive() {
                #[derive(Debug, Eq, FromStr, PartialEq)]
                #[from_str(rename_all = "PascalCase")]
                struct Foo;

                assert_eq!("Foo".parse::<Foo>().unwrap(), Foo);

                assert_eq!(
                    "foo".parse::<Foo>().unwrap_err().to_string(),
                    "Invalid `Foo` string representation",
                );
                assert_eq!(
                    "FOO".parse::<Foo>().unwrap_err().to_string(),
                    "Invalid `Foo` string representation",
                );
                assert_eq!(
                    "FoO".parse::<Foo>().unwrap_err().to_string(),
                    "Invalid `Foo` string representation",
                );
                assert_eq!(
                    "other".parse::<Foo>().unwrap_err().to_string(),
                    "Invalid `Foo` string representation",
                );
            }

            mod casing {
                use super::*;

                macro_rules! casing_test {
                    ($name:ident, $casing:literal, $input:literal) => {
                        mod $name {
                            use super::*;

                            #[test]
                            fn top_level() {
                                #[derive(Debug, Eq, FromStr, PartialEq)]
                                #[from_str(rename_all = $casing)]
                                struct FooBar;

                                assert_eq!($input.parse::<FooBar>().unwrap(), FooBar);
                            }
                        }
                    };
                }

                casing_test!(lower_case, "lowercase", "foobar");
                casing_test!(upper_case, "UPPERCASE", "FOOBAR");
                casing_test!(pascal_case, "PascalCase", "FooBar");
                casing_test!(camel_case, "camelCase", "fooBar");
                casing_test!(snake_case, "snake_case", "foo_bar");
                casing_test!(screaming_snake_case, "SCREAMING_SNAKE_CASE", "FOO_BAR");
                casing_test!(kebab_case, "kebab-case", "foo-bar");
                casing_test!(screaming_kebab_case, "SCREAMING-KEBAB-CASE", "FOO-BAR");
            }
        }

        mod custom_error {
            use derive_more::FromStrError;

            use super::*;

            #[derive(Debug)]
            struct CustomError(FromStrError);

            impl CustomError {
                fn new(err: FromStrError) -> Self {
                    Self(err)
                }
            }

            impl From<FromStrError> for CustomError {
                fn from(value: FromStrError) -> Self {
                    Self(value)
                }
            }

            fn custom_error_fn() -> fn(FromStrError) -> CustomError {
                CustomError::new
            }

            #[test]
            fn only_ty() {
                #[derive(Debug, Eq, FromStr, PartialEq)]
                #[from_str(error(CustomError))]
                struct Foo;

                assert_eq!(
                    match "bar".parse::<Foo>().unwrap_err() {
                        CustomError(e) => e.to_string(),
                    },
                    "Invalid `Foo` string representation",
                );
            }

            #[test]
            fn with_fn() {
                #[derive(Debug, Eq, FromStr, PartialEq)]
                #[from_str(error(CustomError, CustomError::new))]
                struct Foo;

                assert_eq!(
                    match "bar".parse::<Foo>().unwrap_err() {
                        CustomError(e) => e.to_string(),
                    },
                    "Invalid `Foo` string representation",
                );
            }

            #[expect(clippy::redundant_closure, reason = "intended for testing")]
            #[test]
            fn with_closure() {
                #[derive(Debug, Eq, FromStr, PartialEq)]
                #[from_str(error(CustomError, |e| CustomError::new(e)))]
                struct Foo;

                assert_eq!(
                    match "bar".parse::<Foo>().unwrap_err() {
                        CustomError(e) => e.to_string(),
                    },
                    "Invalid `Foo` string representation",
                );
            }

            #[test]
            fn with_fn_call() {
                #[derive(Debug, Eq, FromStr, PartialEq)]
                #[from_str(error(CustomError, custom_error_fn()))]
                struct Foo;

                assert_eq!(
                    match "bar".parse::<Foo>().unwrap_err() {
                        CustomError(e) => e.to_string(),
                    },
                    "Invalid `Foo` string representation",
                );
            }
        }
    }
}

mod enums {
    use super::*;

    mod flat {
        use super::*;

        /// Assertion that `FromStr` does not trigger an ambiguous associated item error for `Err`.
        #[derive(FromStr)]
        enum EnumWithErr {
            Err,
        }

        #[test]
        fn empty() {
            #[derive(Debug, Eq, FromStr, PartialEq)]
            enum Enum {}

            assert_eq!(
                "other".parse::<Enum>().unwrap_err().to_string(),
                "Invalid `Enum` string representation",
            );
        }

        #[test]
        fn unit() {
            #[derive(Debug, Eq, FromStr, PartialEq)]
            enum Enum {
                Foo,
            }

            assert_eq!("Foo".parse::<Enum>().unwrap(), Enum::Foo);
            assert_eq!("FOO".parse::<Enum>().unwrap(), Enum::Foo);
            assert_eq!("foo".parse::<Enum>().unwrap(), Enum::Foo);

            assert_eq!(
                "baz".parse::<Enum>().unwrap_err().to_string(),
                "Invalid `Enum` string representation",
            );
            assert_eq!(
                "other".parse::<Enum>().unwrap_err().to_string(),
                "Invalid `Enum` string representation",
            );
        }

        #[test]
        fn empty_tuple() {
            #[derive(Debug, Eq, FromStr, PartialEq)]
            enum Enum {
                Foo(),
            }

            assert_eq!("Foo".parse::<Enum>().unwrap(), Enum::Foo());
            assert_eq!("FOO".parse::<Enum>().unwrap(), Enum::Foo());
            assert_eq!("foo".parse::<Enum>().unwrap(), Enum::Foo());

            assert_eq!(
                "baz".parse::<Enum>().unwrap_err().to_string(),
                "Invalid `Enum` string representation",
            );
            assert_eq!(
                "other".parse::<Enum>().unwrap_err().to_string(),
                "Invalid `Enum` string representation",
            );
        }

        #[test]
        fn empty_struct() {
            #[derive(Debug, Eq, FromStr, PartialEq)]
            enum Enum {
                Foo {},
            }

            assert_eq!("Foo".parse::<Enum>().unwrap(), Enum::Foo {});
            assert_eq!("FOO".parse::<Enum>().unwrap(), Enum::Foo {});
            assert_eq!("foo".parse::<Enum>().unwrap(), Enum::Foo {});

            assert_eq!(
                "baz".parse::<Enum>().unwrap_err().to_string(),
                "Invalid `Enum` string representation",
            );
            assert_eq!(
                "other".parse::<Enum>().unwrap_err().to_string(),
                "Invalid `Enum` string representation",
            );
        }

        #[test]
        fn case_insensitive() {
            #[derive(Debug, Eq, FromStr, PartialEq)]
            enum Enum {
                Foo,
                Bar,
            }

            assert_eq!("Foo".parse::<Enum>().unwrap(), Enum::Foo);
            assert_eq!("FOO".parse::<Enum>().unwrap(), Enum::Foo);
            assert_eq!("foo".parse::<Enum>().unwrap(), Enum::Foo);

            assert_eq!("Bar".parse::<Enum>().unwrap(), Enum::Bar);
            assert_eq!("baR".parse::<Enum>().unwrap(), Enum::Bar);
            assert_eq!("bar".parse::<Enum>().unwrap(), Enum::Bar);
            assert_eq!("BAR".parse::<Enum>().unwrap(), Enum::Bar);

            assert_eq!(
                "baz".parse::<Enum>().unwrap_err().to_string(),
                "Invalid `Enum` string representation",
            );
            assert_eq!(
                "other".parse::<Enum>().unwrap_err().to_string(),
                "Invalid `Enum` string representation",
            );
        }

        #[test]
        fn case_sensitive() {
            #[derive(Debug, Eq, FromStr, PartialEq)]
            enum Enum {
                Baz,
                BaZ,
                Bar,
            }

            assert_eq!("Baz".parse::<Enum>().unwrap(), Enum::Baz);
            assert_eq!("BaZ".parse::<Enum>().unwrap(), Enum::BaZ);

            assert_eq!("Bar".parse::<Enum>().unwrap(), Enum::Bar);
            assert_eq!("baR".parse::<Enum>().unwrap(), Enum::Bar);
            assert_eq!("bar".parse::<Enum>().unwrap(), Enum::Bar);
            assert_eq!("BAR".parse::<Enum>().unwrap(), Enum::Bar);

            assert_eq!(
                "baz".parse::<Enum>().unwrap_err().to_string(),
                "Invalid `Enum` string representation",
            );
            assert_eq!(
                "other".parse::<Enum>().unwrap_err().to_string(),
                "Invalid `Enum` string representation",
            );
        }

        mod rename_all {
            use super::*;

            #[test]
            fn variant_case_sensitive() {
                #[derive(Debug, Eq, FromStr, PartialEq)]
                enum Enum {
                    Foo,
                    #[from_str(rename_all = "UPPERCASE")]
                    Bar,
                }

                assert_eq!("foo".parse::<Enum>().unwrap(), Enum::Foo);
                assert_eq!("Foo".parse::<Enum>().unwrap(), Enum::Foo);
                assert_eq!("FOO".parse::<Enum>().unwrap(), Enum::Foo);

                assert_eq!("BAR".parse::<Enum>().unwrap(), Enum::Bar);

                assert_eq!(
                    "Bar".parse::<Enum>().unwrap_err().to_string(),
                    "Invalid `Enum` string representation",
                );
                assert_eq!(
                    "bar".parse::<Enum>().unwrap_err().to_string(),
                    "Invalid `Enum` string representation",
                );
                assert_eq!(
                    "other".parse::<Enum>().unwrap_err().to_string(),
                    "Invalid `Enum` string representation",
                );
            }

            #[test]
            fn similar_variant_both_case_sensitive() {
                #[derive(Debug, Eq, FromStr, PartialEq)]
                enum Enum {
                    Xyz,
                    #[from_str(rename_all = "kebab-case")]
                    XyZ,
                }

                assert_eq!("Xyz".parse::<Enum>().unwrap(), Enum::Xyz);
                assert_eq!("xy-z".parse::<Enum>().unwrap(), Enum::XyZ);

                assert_eq!(
                    "xyz".parse::<Enum>().unwrap_err().to_string(),
                    "Invalid `Enum` string representation",
                );
                assert_eq!(
                    "XyZ".parse::<Enum>().unwrap_err().to_string(),
                    "Invalid `Enum` string representation",
                );
                assert_eq!(
                    "other".parse::<Enum>().unwrap_err().to_string(),
                    "Invalid `Enum` string representation",
                );
            }

            #[test]
            fn similar_string_both_case_sensitive() {
                #[allow(non_camel_case_types)]
                #[derive(Debug, Eq, FromStr, PartialEq)]
                enum Enum {
                    Abc,
                    #[from_str(rename_all = "lowercase")]
                    AB_C,
                }

                assert_eq!("Abc".parse::<Enum>().unwrap(), Enum::Abc);
                assert_eq!("abc".parse::<Enum>().unwrap(), Enum::AB_C);

                assert_eq!(
                    "ABC".parse::<Enum>().unwrap_err().to_string(),
                    "Invalid `Enum` string representation",
                );
                assert_eq!(
                    "AB_C".parse::<Enum>().unwrap_err().to_string(),
                    "Invalid `Enum` string representation",
                );
                assert_eq!(
                    "other".parse::<Enum>().unwrap_err().to_string(),
                    "Invalid `Enum` string representation",
                );
            }

            #[test]
            fn variant_overrides_top_level() {
                #[derive(Debug, Eq, FromStr, PartialEq)]
                #[from_str(rename_all = "snake_case")]
                enum Enum {
                    Foo,
                    #[from_str(rename_all = "UPPERCASE")]
                    Bar,
                    Baz,
                    BaZ,
                }

                assert_eq!("foo".parse::<Enum>().unwrap(), Enum::Foo);
                assert_eq!("BAR".parse::<Enum>().unwrap(), Enum::Bar);
                assert_eq!("baz".parse::<Enum>().unwrap(), Enum::Baz);
                assert_eq!("ba_z".parse::<Enum>().unwrap(), Enum::BaZ);

                assert_eq!(
                    "Foo".parse::<Enum>().unwrap_err().to_string(),
                    "Invalid `Enum` string representation",
                );
                assert_eq!(
                    "Bar".parse::<Enum>().unwrap_err().to_string(),
                    "Invalid `Enum` string representation",
                );
                assert_eq!(
                    "bar".parse::<Enum>().unwrap_err().to_string(),
                    "Invalid `Enum` string representation",
                );
                assert_eq!(
                    "Baz".parse::<Enum>().unwrap_err().to_string(),
                    "Invalid `Enum` string representation",
                );
                assert_eq!(
                    "BaZ".parse::<Enum>().unwrap_err().to_string(),
                    "Invalid `Enum` string representation",
                );
                assert_eq!(
                    "other".parse::<Enum>().unwrap_err().to_string(),
                    "Invalid `Enum` string representation",
                );
            }

            mod casing {
                use super::*;

                macro_rules! casing_test {
                    ($name:ident, $casing:literal, $VariantOne:literal, $Two:literal) => {
                        mod $name {
                            use super::*;

                            #[test]
                            fn top_level() {
                                #[derive(Debug, Eq, FromStr, PartialEq)]
                                #[from_str(rename_all = $casing)]
                                enum Enum {
                                    VariantOne,
                                    Two,
                                }

                                assert_eq!(
                                    $VariantOne.parse::<Enum>().unwrap(),
                                    Enum::VariantOne,
                                );
                                assert_eq!($Two.parse::<Enum>().unwrap(), Enum::Two);
                            }

                            #[test]
                            fn variant_level() {
                                #[derive(Debug, Eq, FromStr, PartialEq)]
                                #[from_str(rename_all = "lowercase")] // ignored
                                enum Enum {
                                    #[from_str(rename_all = $casing)]
                                    VariantOne,
                                    #[from_str(rename_all = $casing)]
                                    Two,
                                }

                                assert_eq!(
                                    $VariantOne.parse::<Enum>().unwrap(),
                                    Enum::VariantOne,
                                );
                                assert_eq!($Two.parse::<Enum>().unwrap(), Enum::Two);
                            }

                            #[test]
                            fn only_variant_level() {
                                #[derive(Debug, Eq, FromStr, PartialEq)]
                                enum Enum {
                                    #[from_str(rename_all = $casing)]
                                    VariantOne,
                                    #[from_str(rename_all = $casing)]
                                    Two,
                                }

                                assert_eq!(
                                    $VariantOne.parse::<Enum>().unwrap(),
                                    Enum::VariantOne,
                                );
                                assert_eq!($Two.parse::<Enum>().unwrap(), Enum::Two);
                            }
                        }
                    };
                }

                casing_test!(lower_case, "lowercase", "variantone", "two");
                casing_test!(upper_case, "UPPERCASE", "VARIANTONE", "TWO");
                casing_test!(pascal_case, "PascalCase", "VariantOne", "Two");
                casing_test!(camel_case, "camelCase", "variantOne", "two");
                casing_test!(snake_case, "snake_case", "variant_one", "two");
                casing_test!(
                    screaming_snake_case,
                    "SCREAMING_SNAKE_CASE",
                    "VARIANT_ONE",
                    "TWO"
                );
                casing_test!(kebab_case, "kebab-case", "variant-one", "two");
                casing_test!(
                    screaming_kebab_case,
                    "SCREAMING-KEBAB-CASE",
                    "VARIANT-ONE",
                    "TWO"
                );
            }
        }

        mod custom_error {
            use derive_more::FromStrError;

            use super::*;

            #[derive(Debug)]
            struct CustomError(FromStrError);

            impl CustomError {
                fn new(err: FromStrError) -> Self {
                    Self(err)
                }
            }

            impl From<FromStrError> for CustomError {
                fn from(value: FromStrError) -> Self {
                    Self(value)
                }
            }

            fn custom_error_fn() -> fn(FromStrError) -> CustomError {
                CustomError::new
            }

            macro_rules! assertions {
                () => {
                    assert_eq!("foo".parse::<Enum>().unwrap(), Enum::Foo);
                    assert_eq!("Foo".parse::<Enum>().unwrap(), Enum::Foo);
                    assert_eq!("FOO".parse::<Enum>().unwrap(), Enum::Foo);

                    assert_eq!("Bar".parse::<Enum>().unwrap(), Enum::Bar);
                    assert_eq!("bar".parse::<Enum>().unwrap(), Enum::Bar);

                    assert_eq!("Baz".parse::<Enum>().unwrap(), Enum::Baz);
                    assert_eq!("BaZ".parse::<Enum>().unwrap(), Enum::BaZ);

                    assert_eq!(
                        match "baz".parse::<Enum>().unwrap_err() {
                            CustomError(e) => e.to_string(),
                        },
                        "Invalid `Enum` string representation",
                    );
                    assert_eq!(
                        match "other".parse::<Enum>().unwrap_err() {
                            CustomError(e) => e.to_string(),
                        },
                        "Invalid `Enum` string representation",
                    );
                };
            }

            #[test]
            fn only_ty() {
                #[derive(Debug, Eq, FromStr, PartialEq)]
                #[from_str(error(CustomError))]
                enum Enum {
                    Foo,
                    Bar,
                    Baz,
                    BaZ,
                }

                assertions!();
            }

            #[test]
            fn with_fn() {
                #[derive(Debug, Eq, FromStr, PartialEq)]
                #[from_str(error(CustomError, CustomError::new))]
                enum Enum {
                    Foo,
                    Bar,
                    Baz,
                    BaZ,
                }

                assertions!();
            }

            #[expect(clippy::redundant_closure, reason = "intended for testing")]
            #[test]
            fn with_closure() {
                #[derive(Debug, Eq, FromStr, PartialEq)]
                #[from_str(error(CustomError, |e| CustomError::new(e)))]
                enum Enum {
                    Foo,
                    Bar,
                    Baz,
                    BaZ,
                }

                assertions!();
            }

            #[test]
            fn with_fn_call() {
                #[derive(Debug, Eq, FromStr, PartialEq)]
                #[from_str(error(CustomError, custom_error_fn()))]
                enum Enum {
                    Foo,
                    Bar,
                    Baz,
                    BaZ,
                }

                assertions!();
            }
        }
    }
}
