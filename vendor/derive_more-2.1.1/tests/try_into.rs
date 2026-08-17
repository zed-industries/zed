#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)] // some code is tested for type checking only

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString as _};

use derive_more::TryInto;

// Ensure that the `TryInto` macro is hygienic and doesn't break when `Result`
// has been redefined.
type Result = ();

mod enums {
    use super::*;

    #[test]
    fn complex_with_ignoring() {
        #[derive(TryInto, Clone, Copy, Debug, PartialEq)]
        #[try_into(owned, ref, ref_mut)]
        enum MixedInts {
            SmallInt(i32),
            NamedBigInt {
                int: i64,
            },
            UnsignedWithIgnoredField(#[try_into(ignore)] bool, i64),
            NamedUnsignedWithIgnoredField {
                #[try_into(ignore)]
                useless: bool,
                x: i64,
            },
            TwoSmallInts(i32, i32),
            NamedBigInts {
                x: i64,
                y: i64,
            },
            Unsigned(u32),
            NamedUnsigned {
                x: u32,
            },
            Unit,
            #[try_into(ignore)]
            Unit2,
        }

        let mut i = MixedInts::SmallInt(42);
        assert_eq!(TryInto::<i32>::try_into(i).unwrap(), 42);
        assert_eq!(TryInto::<&i32>::try_into(&i).unwrap(), &42);
        assert_eq!(TryInto::<&mut i32>::try_into(&mut i).unwrap(), &mut 42);
        assert_eq!(
            i64::try_from(i).unwrap_err().to_string(),
            "Only NamedBigInt, UnsignedWithIgnoredField, NamedUnsignedWithIgnoredField can be \
             converted to i64",
        );
        assert_eq!(
            <(i32, i32)>::try_from(i).unwrap_err().to_string(),
            "Only TwoSmallInts can be converted to (i32, i32)",
        );
        assert_eq!(
            <(i64, i64)>::try_from(i).unwrap_err().to_string(),
            "Only NamedBigInts can be converted to (i64, i64)",
        );
        assert_eq!(
            u32::try_from(i).unwrap_err().to_string(),
            "Only Unsigned, NamedUnsigned can be converted to u32",
        );
        assert_eq!(
            <()>::try_from(i).unwrap_err().to_string(),
            "Only Unit can be converted to ()",
        );

        let mut i = MixedInts::NamedBigInt { int: 42 };
        assert_eq!(TryInto::<i64>::try_into(i).unwrap(), 42);
        assert_eq!(TryInto::<&i64>::try_into(&i).unwrap(), &42);
        assert_eq!(TryInto::<&mut i64>::try_into(&mut i).unwrap(), &mut 42);
        assert_eq!(
            i32::try_from(i).unwrap_err().to_string(),
            "Only SmallInt can be converted to i32",
        );
        assert_eq!(
            <(i32, i32)>::try_from(i).unwrap_err().to_string(),
            "Only TwoSmallInts can be converted to (i32, i32)",
        );
        assert_eq!(
            <(i64, i64)>::try_from(i).unwrap_err().to_string(),
            "Only NamedBigInts can be converted to (i64, i64)",
        );
        assert_eq!(
            u32::try_from(i).unwrap_err().to_string(),
            "Only Unsigned, NamedUnsigned can be converted to u32",
        );
        assert_eq!(
            <()>::try_from(i).unwrap_err().to_string(),
            "Only Unit can be converted to ()",
        );

        let mut i = MixedInts::TwoSmallInts(42, 64);
        assert_eq!(TryInto::<(i32, i32)>::try_into(i).unwrap(), (42, 64));
        assert_eq!(TryInto::<(&i32, &i32)>::try_into(&i).unwrap(), (&42, &64));
        assert_eq!(
            TryInto::<(&mut i32, &mut i32)>::try_into(&mut i).unwrap(),
            (&mut 42, &mut 64),
        );
        assert_eq!(
            i32::try_from(i).unwrap_err().to_string(),
            "Only SmallInt can be converted to i32",
        );
        assert_eq!(
            i64::try_from(i).unwrap_err().to_string(),
            "Only NamedBigInt, UnsignedWithIgnoredField, NamedUnsignedWithIgnoredField can be \
             converted to i64",
        );
        assert_eq!(
            <(i64, i64)>::try_from(i).unwrap_err().to_string(),
            "Only NamedBigInts can be converted to (i64, i64)",
        );
        assert_eq!(
            u32::try_from(i).unwrap_err().to_string(),
            "Only Unsigned, NamedUnsigned can be converted to u32",
        );
        assert_eq!(
            <()>::try_from(i).unwrap_err().to_string(),
            "Only Unit can be converted to ()",
        );

        let mut i = MixedInts::NamedBigInts { x: 42, y: 64 };
        assert_eq!(TryInto::<(i64, i64)>::try_into(i).unwrap(), (42, 64));
        assert_eq!(TryInto::<(&i64, &i64)>::try_into(&i).unwrap(), (&42, &64));
        assert_eq!(
            TryInto::<(&mut i64, &mut i64)>::try_into(&mut i).unwrap(),
            (&mut 42, &mut 64),
        );
        assert_eq!(
            i32::try_from(i).unwrap_err().to_string(),
            "Only SmallInt can be converted to i32",
        );
        assert_eq!(
            i64::try_from(i).unwrap_err().to_string(),
            "Only NamedBigInt, UnsignedWithIgnoredField, NamedUnsignedWithIgnoredField can be \
             converted to i64",
        );
        assert_eq!(
            <(i32, i32)>::try_from(i).unwrap_err().to_string(),
            "Only TwoSmallInts can be converted to (i32, i32)",
        );
        assert_eq!(
            u32::try_from(i).unwrap_err().to_string(),
            "Only Unsigned, NamedUnsigned can be converted to u32",
        );
        assert_eq!(
            <()>::try_from(i).unwrap_err().to_string(),
            "Only Unit can be converted to ()",
        );

        let mut i = MixedInts::Unsigned(42);
        assert_eq!(TryInto::<u32>::try_into(i).unwrap(), 42);
        assert_eq!(TryInto::<&u32>::try_into(&i).unwrap(), &42);
        assert_eq!(TryInto::<&mut u32>::try_into(&mut i).unwrap(), &mut 42);
        assert_eq!(
            i32::try_from(i).unwrap_err().to_string(),
            "Only SmallInt can be converted to i32",
        );
        assert_eq!(
            i64::try_from(i).unwrap_err().to_string(),
            "Only NamedBigInt, UnsignedWithIgnoredField, NamedUnsignedWithIgnoredField can be \
             converted to i64",
        );
        assert_eq!(
            <(i32, i32)>::try_from(i).unwrap_err().to_string(),
            "Only TwoSmallInts can be converted to (i32, i32)",
        );
        assert_eq!(
            <(i64, i64)>::try_from(i).unwrap_err().to_string(),
            "Only NamedBigInts can be converted to (i64, i64)",
        );
        assert_eq!(
            <()>::try_from(i).unwrap_err().to_string(),
            "Only Unit can be converted to ()",
        );

        let mut i = MixedInts::NamedUnsigned { x: 42 };
        assert_eq!(TryInto::<u32>::try_into(i).unwrap(), 42);
        assert_eq!(TryInto::<&u32>::try_into(&i).unwrap(), &42);
        assert_eq!(TryInto::<&mut u32>::try_into(&mut i).unwrap(), &mut 42);
        assert_eq!(
            i32::try_from(i).unwrap_err().to_string(),
            "Only SmallInt can be converted to i32",
        );
        assert_eq!(
            i64::try_from(i).unwrap_err().to_string(),
            "Only NamedBigInt, UnsignedWithIgnoredField, NamedUnsignedWithIgnoredField can be \
             converted to i64",
        );
        assert_eq!(
            i64::try_from(i).unwrap_err().to_string(),
            "Only NamedBigInt, UnsignedWithIgnoredField, NamedUnsignedWithIgnoredField can be \
             converted to i64",
        );
        assert_eq!(
            <(i32, i32)>::try_from(i).unwrap_err().to_string(),
            "Only TwoSmallInts can be converted to (i32, i32)",
        );
        assert_eq!(
            <(i64, i64)>::try_from(i).unwrap_err().to_string(),
            "Only NamedBigInts can be converted to (i64, i64)",
        );
        assert_eq!(
            <()>::try_from(i).unwrap_err().to_string(),
            "Only Unit can be converted to ()",
        );

        let mut i = MixedInts::Unit;
        assert!(TryInto::<()>::try_into(i).is_ok());
        assert!(TryInto::<()>::try_into(&i).is_ok());
        assert!(TryInto::<()>::try_into(&mut i).is_ok());
        assert_eq!(
            i32::try_from(i).unwrap_err().to_string(),
            "Only SmallInt can be converted to i32",
        );
        assert_eq!(
            i64::try_from(i).unwrap_err().to_string(),
            "Only NamedBigInt, UnsignedWithIgnoredField, NamedUnsignedWithIgnoredField can be \
             converted to i64",
        );
        assert_eq!(
            <(i32, i32)>::try_from(i).unwrap_err().to_string(),
            "Only TwoSmallInts can be converted to (i32, i32)",
        );
        assert_eq!(
            <(i64, i64)>::try_from(i).unwrap_err().to_string(),
            "Only NamedBigInts can be converted to (i64, i64)",
        );
        assert_eq!(
            u32::try_from(i).unwrap_err().to_string(),
            "Only Unsigned, NamedUnsigned can be converted to u32",
        );
    }

    #[test]
    fn associated_type_name_hygiene() {
        #[derive(Debug, PartialEq)]
        enum EnumWithError {
            Error,
        }

        // Making sure that `TryInto` does not trigger an ambiguous associated item error for
        // `Error`.
        #[derive(TryInto, Debug, PartialEq)]
        enum EnumIntoEnumWithError {
            Foo(EnumWithError),
        }

        assert_eq!(
            TryInto::<EnumWithError>::try_into(EnumIntoEnumWithError::Foo(
                EnumWithError::Error
            ))
            .unwrap(),
            EnumWithError::Error,
        );
    }

    #[test]
    fn error_preserves_input() {
        #[derive(TryInto, Clone, Copy, Debug, PartialEq)]
        #[try_into(owned, ref, ref_mut)]
        enum MixedInts {
            SmallInt(i32),
            NamedBigInt { int: i64 },
        }

        let mut i = MixedInts::SmallInt(42);
        assert_eq!(TryInto::<i64>::try_into(i).unwrap_err().input, i);
        assert_eq!(TryInto::<&i64>::try_into(&i).unwrap_err().input, &i);
        assert_eq!(
            TryInto::<&mut i64>::try_into(&mut i.clone())
                .unwrap_err()
                .input,
            &mut i
        );
    }

    mod generic {
        use super::*;

        #[test]
        fn simple_with_bound() {
            #[derive(Debug, PartialEq)]
            pub struct BarWrapper<V>(V);

            #[derive(TryInto, Debug, PartialEq)]
            #[try_into(ref, ref_mut)]
            pub enum Bar<V>
            where
                Self: 'static,
            {
                V(BarWrapper<V>),
            }

            assert_eq!(
                TryInto::<BarWrapper<_>>::try_into(Bar::V(BarWrapper(1))).unwrap(),
                BarWrapper(1),
            );
            assert_eq!(
                TryInto::<&BarWrapper<_>>::try_into(&Bar::V(BarWrapper(1))).unwrap(),
                &BarWrapper(1),
            );
            assert_eq!(
                TryInto::<&mut BarWrapper<_>>::try_into(&mut Bar::V(BarWrapper(1)))
                    .unwrap(),
                &mut BarWrapper(1),
            );
        }

        #[test]
        fn complex_with_bound() {
            #[derive(Debug, PartialEq)]
            struct Wrapper<'a, const Y: usize, U>(&'a [U; Y]);

            #[derive(TryInto, Debug, PartialEq)]
            #[try_into(ref, ref_mut)]
            enum Foo<'lt: 'static, T: Clone, const X: usize> {
                X(Wrapper<'lt, X, T>),
            }

            assert_eq!(
                TryInto::<Wrapper<1, _>>::try_into(Foo::X(Wrapper(&[1]))).unwrap(),
                Wrapper(&[1]),
            );
            assert_eq!(
                TryInto::<&Wrapper<1, _>>::try_into(&Foo::X(Wrapper(&[1]))).unwrap(),
                &Wrapper(&[1]),
            );
            assert_eq!(
                TryInto::<&mut Wrapper<1, _>>::try_into(&mut Foo::X(Wrapper(&[1])))
                    .unwrap(),
                &mut Wrapper(&[1]),
            );
        }
    }

    mod custom_error {
        use core::fmt;

        use super::*;

        #[derive(Debug)]
        struct CustomError(String);

        impl<T> From<derive_more::TryIntoError<T>> for CustomError {
            fn from(value: derive_more::TryIntoError<T>) -> Self {
                Self(value.to_string())
            }
        }

        impl fmt::Display for CustomError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        fn custom_error_fn<T>() -> fn(derive_more::TryIntoError<T>) -> CustomError {
            CustomError::from
        }

        #[test]
        fn simple() {
            #[derive(TryInto, Clone, Debug)]
            #[try_into(owned, ref, ref_mut)]
            #[try_into(error(CustomError))]
            enum MixedData {
                Int(u32),
                String(String),
            }

            let i = MixedData::Int(42);
            assert_eq!(
                String::try_from(i).unwrap_err().to_string(),
                "Only String can be converted to String",
            );

            let s = MixedData::String("foo".into());
            assert_eq!(
                u32::try_from(s).unwrap_err().to_string(),
                "Only Int can be converted to u32",
            );
        }

        #[test]
        fn with_fn() {
            #[derive(TryInto, Clone, Debug)]
            #[try_into(owned, ref, ref_mut)]
            #[try_into(error(CustomError, CustomError::from))]
            enum MixedData {
                Int(u32),
                String(String),
            }

            let i = MixedData::Int(42);
            assert_eq!(
                String::try_from(i).unwrap_err().to_string(),
                "Only String can be converted to String",
            );

            let s = MixedData::String("foo".into());
            assert_eq!(
                u32::try_from(s).unwrap_err().to_string(),
                "Only Int can be converted to u32",
            );
        }

        #[test]
        fn with_closure() {
            #[derive(TryInto, Clone, Debug)]
            #[try_into(owned, ref, ref_mut)]
            #[try_into(error(CustomError, |e| e.into()))]
            enum MixedData {
                Int(u32),
                String(String),
            }

            let i = MixedData::Int(42);
            assert_eq!(
                String::try_from(i).unwrap_err().to_string(),
                "Only String can be converted to String",
            );

            let s = MixedData::String("foo".into());
            assert_eq!(
                u32::try_from(s).unwrap_err().to_string(),
                "Only Int can be converted to u32",
            );
        }

        #[test]
        fn with_fn_call() {
            #[derive(TryInto, Clone, Debug)]
            #[try_into(owned, ref, ref_mut)]
            #[try_into(error(CustomError, custom_error_fn()))]
            enum MixedData {
                Int(u32),
                String(String),
            }

            let i = MixedData::Int(42);
            assert_eq!(
                String::try_from(i).unwrap_err().to_string(),
                "Only String can be converted to String",
            );

            let s = MixedData::String("foo".into());
            assert_eq!(
                u32::try_from(s).unwrap_err().to_string(),
                "Only Int can be converted to u32",
            );
        }
    }
}
