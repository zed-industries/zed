// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This module provides integration tests that test the expansion
//! of the derive macro.

//==============================================================================
// Macros:
//==============================================================================

// Borrowed from:
// https://docs.rs/synstructure/0.7.0/src/synstructure/macros.rs.html#104-135
macro_rules! test_derive {
    ($name:path { $($i:tt)* } expands to { $($o:tt)* }) => {
        {
            #[allow(dead_code)]
            fn ensure_compiles() {
                $($i)*
                $($o)*
            }

            test_derive!($name { $($i)* } expands to { $($o)* } no_build);
        }
    };
    ($name:path { $($i:tt)* } expands to { $($o:tt)* } no_build) => {
        {
            let expected = stringify!( $($o)* )
                .parse::<proc_macro2::TokenStream>()
                .expect("output should be a valid TokenStream");

            let i = stringify!( $($i)* );
            let parsed = $crate::syn::parse_str::<$crate::syn::DeriveInput>(i).expect(
                concat!("Failed to parse input to `#[derive(",
                    stringify!($name),
                ")]`")
            );
            let res = $name(parsed);
            assert_eq!(
                format!("{}", res),
                format!("{}", expected)
            );
        }
    };
}

macro_rules! test {
    (no_build $test_name:ident { $($i:tt)* } expands to { $($o:tt)* }) => {
        #[test]
        fn $test_name() {
            test_derive!(
                $crate::derive::impl_proptest_arbitrary { $($i)* }
                expands to { $($o)* } no_build
            );
        }
    };
    ($test_name:ident { $($i:tt)* } expands to { $($o:tt)* }) => {
        #[test]
        fn $test_name() {
            test_derive!(
                $crate::derive::impl_proptest_arbitrary { $($i)* }
                expands to { $($o)* }
            );
        }
    };
}

//==============================================================================
// Unit structs:
//==============================================================================

test! {
    struct_unit_unit {
        #[derive(Debug)]
        struct MyUnitStruct;
    } expands to {
        #[allow(non_local_definitions)]
        #[allow(non_upper_case_globals)]
        #[allow(clippy::arc_with_non_send_sync)]
        const _: () = {
            use proptest as _proptest;
        impl _proptest::arbitrary::Arbitrary for MyUnitStruct {
            type Parameters = ();
            type Strategy = fn() -> Self;

            fn arbitrary_with(_top: Self::Parameters) -> Self::Strategy {
                (|| MyUnitStruct {}) as fn() -> _
            }
        }
        };
    }
}

test! {
    struct_unit_tuple {
        #[derive(Debug)]
        struct MyTupleUnitStruct();
    } expands to {
        #[allow(non_local_definitions)]
        #[allow(non_upper_case_globals)]
        #[allow(clippy::arc_with_non_send_sync)]
        const _: () = {
            use proptest as _proptest;
        impl _proptest::arbitrary::Arbitrary for MyTupleUnitStruct {
            type Parameters = ();
            type Strategy = fn() -> Self;

            fn arbitrary_with(_top: Self::Parameters) -> Self::Strategy {
                (|| MyTupleUnitStruct {}) as fn() -> _
            }
        }
        };
    }
}

test! {
    struct_unit_named {
        #[derive(Debug)]
        struct MyNamedUnitStruct {}
    } expands to {
        #[allow(non_local_definitions)]
        #[allow(non_upper_case_globals)]
        #[allow(clippy::arc_with_non_send_sync)]
        const _: () = {
            use proptest as _proptest;
        impl _proptest::arbitrary::Arbitrary for MyNamedUnitStruct {
            type Parameters = ();
            type Strategy = fn() -> Self;

            fn arbitrary_with(_top: Self::Parameters) -> Self::Strategy {
                (|| MyNamedUnitStruct {}) as fn () -> _
            }
        }
        };
    }
}
