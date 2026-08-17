macro_rules! with_shared_docs {(
    $(#[$before_clarification:meta])*
    ;clarification
    $(#[$before_syntax:meta])*
    ;syntax
    $(#[$after_syntax:meta])*
    ;error_message
    $(#[$after_error_message:meta])*
    ;limitations
    $item:item
) => (
    $(#[$before_clarification])*
    ///
    /// This macro requires the `"assertc"` feature to be exported.
    ///
    $(#[$before_syntax])*
    ///
    /// This macro uses [the same syntax](./fmt/index.html#fmtsyntax)
    /// for the format string and supports the same formatting arguments as the
    /// [`formatc`] macro.
    ///
    $(#[$after_syntax])*
    $(#[$after_error_message])*
    /// # Limitations
    ///
    /// This macro has these limitations:
    ///
    /// - It can only use constants that involve concrete types,
    /// so while a `Type::<u8>::FOO` in an argument would be fine,
    /// `Type::<T>::FOO` would not be (`T` being a type parameter).
    ///
    /// - Integer arguments must have a type inferrable from context,
    /// [as described in the integer arguments section in the root module
    /// ](./index.html#integer-args).
    ///
    /// [`PWrapper`]: ./struct.PWrapper.html
    /// [`formatc`]: ./macro.formatc.html
    /// [`FormatMarker`]: ./marker_traits/trait.FormatMarker.html
    ///
    $item
)}

////////////////////////////////////////////////////////////////////////////////

with_shared_docs! {
    /// Compile-time assertions with formatting.
    ///
    ;clarification
    ;syntax
    ;error_message
    ;limitations
    ///
    /// # Examples
    ///
    /// ### Passing assertion
    ///
    /// ```rust
    ///
    /// use const_format::assertc;
    ///
    /// use std::mem::size_of;
    ///
    /// assertc!(
    ///     size_of::<&str>() == size_of::<&[u8]>(),
    ///     "The size of `&str`({} bytes) and `&[u8]`({} bytes) aren't the same?!?!",
    ///     size_of::<&str>(),
    ///     size_of::<&[u8]>(),
    /// );
    ///
    /// # fn main(){}
    /// ```
    ///
    /// ### Failing assertion
    ///
    /// This example demonstrates a failing assertion,
    /// and how the compiler error looks like as of 2023-10-14.
    ///
    /// ```compile_fail
    ///
    /// use const_format::assertc;
    ///
    /// const L: u64 = 2;
    /// const R: u64 = 2;
    ///
    /// assertc!(L + R == 5, "{} plus {} isn't 5, buddy", L,  R);
    ///
    /// # fn main(){}
    /// ```
    ///
    /// This is the compiler output:
    ///
    /// ```text
    /// error[E0080]: evaluation of constant value failed
    ///   --> const_format/src/macros/assertions/assertc_macros.rs:109:10
    ///    |
    /// 11 | assertc!(L + R == 5, "{} plus {} isn't 5, buddy", L,  R);
    ///    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at '
    /// assertion failed.
    /// 2 plus 2 isn't 5, buddy
    /// ', const_format/src/macros/assertions/assertc_macros.rs:11:10
    /// ```
    ///
    #[cfg_attr(feature = "__docsrs", doc(cfg(feature = "assertc")))]
    #[macro_export]
    macro_rules! assertc {
        ($($parameters:tt)*) => (
            $crate::__assertc_inner!{
                __formatc_if_impl
                ($($parameters)*)
                ($($parameters)*)
            }
        );
    }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! assert_eq_docs {
    (
        $(#[$documentation:meta])*
        ;documentation
        $item:item
    ) => (
        with_shared_docs! {
            $(#[$documentation])*
            ;clarification
            /// # Arguments
            ///
            /// This macro accepts these types for comparison and debug printing:
            ///
            /// - Standard library types for which  [`PWrapper`] wrapping that type
            /// has a `const_eq` method.
            /// This includes all integer types, `&str`, slices/arrays of integers/`&str`,
            /// Options of integers/`&str`, etc.
            ///
            /// - non-standard-library types that implement [`FormatMarker`] with debug formatting<br>
            /// and have a `const fn const_eq(&self, other:&Self) -> bool` inherent method,
            ///
            ;syntax
            ;error_message
            ;limitations
            $item
        }
    )
}

assert_eq_docs! {
    /// Compile-time equality assertion with formatting.
    ///
    ;documentation
    ///
    /// # Examples
    ///
    /// ### Passing assertion
    ///
    /// ```rust
    ///
    /// use const_format::assertc_eq;
    ///
    /// use std::mem::size_of;
    ///
    /// assertc_eq!(size_of::<usize>(), size_of::<[usize;1]>());
    ///
    /// const TWO: u32 = 2;
    /// assertc_eq!(TWO, TWO, "Oh no {} doesn't equal itself!!", TWO);
    ///
    /// # fn main(){}
    /// ```
    ///
    /// ### Failing assertion
    ///
    /// This example demonstrates a failing assertion,
    /// and how the compiler error looks like as of 2023-10-14.
    ///
    /// ```compile_fail
    ///
    /// use const_format::assertc_eq;
    ///
    /// use std::mem::size_of;
    ///
    /// assertc_eq!(size_of::<u32>(), size_of::<u8>());
    ///
    /// # fn main(){}
    /// ```
    ///
    /// This is the compiler output:
    ///
    /// ```text
    /// error[E0080]: evaluation of constant value failed
    ///   --> const_format/src/macros/assertions/assertc_macros.rs:222:13
    ///    |
    /// 10 | assertc_eq!(size_of::<u32>(), size_of::<u8>());
    ///    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at '
    /// assertion failed: `(left == right)`
    ///  left: `4`
    /// right: `1`', const_format/src/macros/assertions/assertc_macros.rs:10:13
    /// ```
    ///
    /// ### Comparing user-defined types
    ///
    /// This example demonstrates how you can assert that two values of a
    /// user-defined type are equal.
    ///
    #[cfg_attr(feature = "derive", doc = "```compile_fail")]
    #[cfg_attr(not(feature = "derive"), doc = "```ignore")]
    ///
    /// use const_format::{Formatter, PWrapper};
    /// use const_format::{ConstDebug, assertc_eq, try_};
    ///
    /// const POINT: Point = Point{ x: 5, y: 8, z: 13 };
    /// const OTHER_POINT: Point = Point{ x: 21, y: 34, z: 55 };
    ///
    /// assertc_eq!(POINT, OTHER_POINT);
    ///
    /// #[derive(ConstDebug)]
    /// pub struct Point {
    ///     pub x: u32,
    ///     pub y: u32,
    ///     pub z: u32,
    /// }
    ///
    /// impl Point {
    ///     pub const fn const_eq(&self, other: &Self) -> bool {
    ///         self.x == other.x &&
    ///         self.y == other.y &&
    ///         self.z == other.z
    ///     }
    /// }
    /// ```
    ///
    /// This is the compiler output:
    ///
    /// ```text
    /// error[E0080]: evaluation of constant value failed
    ///   --> src/macros/assertions.rs:331:14
    ///    |
    /// 12 |  assertc_eq!(POINT, OTHER_POINT);
    ///    |              ^^^^^^^^^^^^^^^^^^ the evaluated program panicked at '
    /// assertion failed: `(left == right)`
    ///  left: `Point {
    ///     x: 5,
    ///     y: 8,
    ///     z: 13,
    /// }`
    /// right: `Point {
    ///     x: 21,
    ///     y: 34,
    ///     z: 55,
    /// }`', src/macros/assertions.rs:12:14
    ///
    /// error: aborting due to previous error
    ///
    /// ```
    ///
    #[cfg_attr(feature = "__docsrs", doc(cfg(feature = "assertc")))]
    #[macro_export]
    macro_rules! assertc_eq {
        ($($parameters:tt)*) => (
            $crate::__assertc_equality_inner!{
                ($($parameters)*)
                ($($parameters)*)
                ( == )
                ("==")
            }
        );
    }
}

assert_eq_docs! {
    /// Compile-time inequality assertion with formatting.
    ///
    ;documentation
    ///
    /// # Examples
    ///
    /// ### Passing assertion
    ///
    /// ```rust
    ///
    /// use const_format::assertc_ne;
    ///
    /// use std::mem::size_of;
    ///
    /// assertc_ne!(size_of::<u32>(), size_of::<[u32; 2]>());
    ///
    /// const TWO: u32 = 2;
    /// const THREE: u32 = 3;
    /// assertc_ne!(TWO, THREE, "Oh no {} somehow equals {}!!", TWO, THREE);
    ///
    /// # fn main(){}
    /// ```
    ///
    /// ### Failing assertion
    ///
    /// This example demonstrates a failing assertion,
    /// and how the compiler error looks like as of 2023-10-14.
    ///
    /// ```compile_fail
    ///
    /// use const_format::assertc_ne;
    ///
    /// use std::mem::size_of;
    ///
    /// type Foo = u32;
    ///
    /// assertc_ne!(size_of::<u32>(), size_of::<Foo>());
    ///
    /// # fn main(){}
    /// ```
    ///
    /// This is the compiler output:
    ///
    /// ```text
    /// error[E0080]: evaluation of constant value failed
    ///   --> const_format/src/macros/assertions/assertc_macros.rs:350:13
    ///    |
    /// 12 | assertc_ne!(size_of::<u32>(), size_of::<Foo>());
    ///    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at '
    /// assertion failed: `(left != right)`
    ///  left: `4`
    /// right: `4`', const_format/src/macros/assertions/assertc_macros.rs:12:13
    /// ```
    ///
    /// ### Comparing user-defined types
    ///
    /// This example demonstrates how you can assert that two values of a
    /// user-defined type are unequal.
    ///
    #[cfg_attr(feature = "derive", doc = "```compile_fail")]
    #[cfg_attr(not(feature = "derive"), doc = "```ignore")]
    ///
    /// use const_format::{Formatter, PWrapper};
    /// use const_format::{ConstDebug, assertc_ne, try_};
    ///
    /// const POINT: Point = Point{ x: 5, y: 8, z: 13 };
    ///
    /// assertc_ne!(POINT, POINT);
    ///
    /// #[derive(ConstDebug)]
    /// pub struct Point {
    ///     pub x: u32,
    ///     pub y: u32,
    ///     pub z: u32,
    /// }
    ///
    /// impl Point {
    ///     pub const fn const_eq(&self, other: &Self) -> bool {
    ///         self.x == other.x &&
    ///         self.y == other.y &&
    ///         self.z == other.z
    ///     }
    /// }
    /// ```
    ///
    /// This is the compiler output:
    ///
    /// ```text
    /// error[E0080]: evaluation of constant value failed
    ///   --> src/macros/assertions.rs:451:14
    ///    |
    /// 11 |  assertc_ne!(POINT, POINT);
    ///    |              ^^^^^^^^^^^^ the evaluated program panicked at '
    /// assertion failed: `(left != right)`
    ///  left: `Point {
    ///     x: 5,
    ///     y: 8,
    ///     z: 13,
    /// }`
    /// right: `Point {
    ///     x: 5,
    ///     y: 8,
    ///     z: 13,
    /// }`', src/macros/assertions.rs:11:14
    ///
    /// error: aborting due to previous error
    ///
    /// ```
    ///
    #[cfg_attr(feature = "__docsrs", doc(cfg(feature = "assertc")))]
    #[macro_export]
    macro_rules! assertc_ne {
        ($($parameters:tt)*) => (
            $crate::__assertc_equality_inner!{
                ($($parameters)*)
                ($($parameters)*)
                ( != )
                ("!=")
            }
        );
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __assertc_equality_inner {
    (
        ($($parameters:tt)*)
        (
            $left:expr,
            $right:expr
            $(, $fmt_literal:expr $(,$fmt_arg:expr)*)? $(,)?
        )
        ($($op:tt)*)
        ($op_str:expr)
    )=>{
        const _: () = {
            use $crate::__cf_osRcTFl4A;
            use $crate::pmr::respan_to as __cf_respan_to;

            const LEFT: $crate::pmr::bool = {
                // Have to use `respan_to` to make the `multiple coerce found` error
                // point at the `$left` argument here.
                use $crate::coerce_to_fmt as __cf_coerce_to_fmt;
                match [&$left, &$right] {
                    __cf_respan_to!(($left) [left, right]) =>
                        __cf_respan_to!(($left) __cf_coerce_to_fmt!(left).const_eq(right)),
                }
            };
            const RIGHT: $crate::pmr::bool = true;

            $crate::__assertc_common!{
                __formatc_if_impl
                ($($parameters)*)
                (LEFT $($op)* RIGHT)
                (
                    concat!(
                        "\nassertion failed: `(left ",
                        $op_str,
                        " right)`\n",
                        " left: `{left_NHPMWYD3NJA:#?}`\n\
                         right: `{right_NHPMWYD3NJA:#?}`",
                        $("\n", $fmt_literal, "\n")?
                    ),
                    $($($fmt_arg,)*)?
                    left_NHPMWYD3NJA = $left,
                    right_NHPMWYD3NJA = $right
                )
            }
        };
    }
}
