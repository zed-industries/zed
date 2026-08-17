macro_rules! with_shared_docs {(
    $(#[$before_clarification:meta])*
    ;clarification
    $(#[$before_syntax:meta])*
    ;syntax
    $(#[$after_syntax:meta])*
    ;limitations
    $item:item
) => (
    $(#[$before_clarification])*
    ///
    /// [For **examples** look here](#examples)
    ///
    /// This macro requires the `"assertcp"` feature to be exported.<br>
    ///
    $(#[$before_syntax])*
    /// # Syntax
    ///
    /// This macro uses the same syntax
    /// for the format string and formatting arguments as the
    /// [`formatcp`](crate::formatcp) macro.
    ///
    $(#[$after_syntax])*
    /// # Limitations
    ///
    /// This macro can only take constants of these types as arguments:
    ///
    /// - `&str`
    ///
    /// - `i*`/`u*` (all the primitive integer types).
    ///
    /// - `char`
    ///
    /// - `bool`
    ///
    /// This macro also has these limitations:
    ///
    /// - It can only use constants that involve concrete types,
    /// so while a `Type::<u8>::FOO` in an argument would be fine,
    /// `Type::<T>::FOO` would not be (`T` being a type parameter).
    ///
    /// - Integer arguments must have a type inferrable from context,
    /// [as described in the integer arguments section in the root module
    /// ](./index.html#integer-args).
    ///
    $item
)}

with_shared_docs! {
    /// Compile-time assertion with formatting.
    ///
    ;clarification
    ;syntax
    ;limitations
    ///
    /// # Examples
    ///
    /// ### Passing assertion
    ///
    /// ```rust
    /// use const_format::assertcp;
    ///
    /// use std::mem::align_of;
    ///
    /// assertcp!(
    ///     align_of::<&str>() == align_of::<usize>(),
    ///     "The alignment of `&str`({} bytes) and `usize`({} bytes) isn't the same?!?!",
    ///     align_of::<&str>(),
    ///     align_of::<usize>(),
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
    /// use const_format::assertcp;
    ///
    /// const L: u64 = 2;
    /// const R: u32 = 5;
    ///
    /// assertcp!(L.pow(R) == 64, "{L} to the {R} isn't 64, it's {}", L.pow(R));
    ///
    /// # fn main(){}
    /// ```
    ///
    /// This is the compiler output:
    ///
    /// ```text
    /// error[E0080]: evaluation of constant value failed
    ///  --> const_format/src/macros/assertions/assertcp_macros.rs:116:11
    ///   |
    /// 9 | assertcp!(L.pow(R) == 64, "{L} to the {R} isn't 64, it's {}", L.pow(R));
    ///   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at '
    /// assertion failed.
    /// 2 to the 5 isn't 64, it's 32
    /// ', const_format/src/macros/assertions/assertcp_macros.rs:9:11
    ///
    /// ```
    ///
    #[cfg_attr(feature = "__docsrs", doc(cfg(feature = "assertcp")))]
    #[macro_export]
    macro_rules! assertcp {
        ($($parameters:tt)*) => (
            $crate::__assertc_inner!{
                __formatcp_if_impl
                ($($parameters)*)
                ($($parameters)*)
            }
        );
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __assertcp_equality_inner {
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
        #[allow(non_snake_case)]
        const _: () = {
            use $crate::__cf_osRcTFl4A;
            const ARGS_NHPMWYD3NJA:
                ($crate::pmr::bool, $crate::pmr::PArgument, $crate::pmr::PArgument)
            = {
                let left = $crate::PWrapper($left);
                let right = $crate::pmr::PConvWrapper($right);
                let cond = left.const_eq(&right.0);
                let fmt = $crate::pmr::FormattingFlags::NEW.set_alternate(true);
                (
                    cond,
                    $crate::pmr::PConvWrapper(left.0).to_pargument_debug(fmt),
                    right.to_pargument_debug(fmt),
                )
            };

            $crate::__assertc_common!{
                __formatcp_if_impl
                ($($parameters)*)
                (ARGS_NHPMWYD3NJA.0 $($op)* true)
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
                    left_NHPMWYD3NJA = ARGS_NHPMWYD3NJA.1,
                    right_NHPMWYD3NJA = ARGS_NHPMWYD3NJA.2
                )
            }
        };
    }
}

with_shared_docs! {
    /// Compile-time equality assertion with formatting.
    ///
    ;clarification
    ;syntax
    ;limitations
    ///
    /// # Examples
    ///
    /// ### Passing assertion
    ///
    /// ```rust
    /// use const_format::assertcp_eq;
    ///
    /// const NAME: &str = "Bob";
    ///
    /// assertcp_eq!(NAME, "Bob", "Guessed wrong, the right value is {}", NAME);
    ///
    /// const SUM: u8 = 1 + 2 + 3;
    /// assertcp_eq!(6u8, SUM, "Guessed wrong, the right value is {}", SUM);
    /// ```
    ///
    /// ### Failing assertion
    ///
    /// This example demonstrates a failing assertion,
    /// and how the compiler error looks like as of 2023-10-14.
    ///
    /// ```compile_fail
    /// use const_format::assertcp_eq;
    ///
    /// use std::mem::size_of;
    ///
    /// #[repr(C)]
    /// struct Type(u16, u16, u16);
    ///
    /// assertcp_eq!(size_of::<Type>(), size_of::<[u16; 2]>(), "Whoops, `Type` is too large");
    ///
    /// ```
    ///
    /// This is the compiler output:
    ///
    /// ```text
    /// error[E0080]: evaluation of constant value failed
    ///   --> const_format/src/macros/assertions/assertcp_macros.rs:235:14
    ///    |
    /// 12 | assertcp_eq!(size_of::<Type>(), size_of::<[u16; 2]>(), "Whoops, `Type` is too large");
    ///    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at '
    /// assertion failed: `(left == right)`
    ///  left: `6`
    /// right: `4`
    /// Whoops, `Type` is too large
    /// ', const_format/src/macros/assertions/assertcp_macros.rs:12:14
    ///
    /// ```
    ///
    #[cfg_attr(feature = "__docsrs", doc(cfg(feature = "assertcp")))]
    #[macro_export]
    macro_rules! assertcp_eq {
        ($($parameters:tt)*) => (
            $crate::__assertcp_equality_inner!{
                ($($parameters)*)
                ($($parameters)*)
                ( == )
                ("==")
            }
        );
    }
}
with_shared_docs! {
    /// Compile-time inequality assertion with formatting.
    ///
    ;clarification
    ;syntax
    ;limitations
    ///
    /// # Examples
    ///
    /// ### Passing assertion
    ///
    /// ```rust
    /// use const_format::assertcp_ne;
    ///
    /// assertcp_ne!(std::mem::size_of::<usize>(), 1usize, "Oh no, usize is tiny!");
    ///
    /// const CHAR: char = ';';
    /// assertcp_ne!(CHAR, '.', "CHAR must not be a dot!");
    /// ```
    ///
    /// ### Failing assertion
    ///
    /// This example demonstrates a failing assertion,
    /// and how the compiler error looks like as of 2023-10-14.
    ///
    /// ```compile_fail
    /// use const_format::assertcp_ne;
    ///
    /// const NAME: &str = "";
    /// assertcp_ne!(NAME, "", "NAME must not be empty!");
    ///
    /// ```
    ///
    /// This is the compiler output:
    ///
    /// ```text
    /// error[E0080]: evaluation of constant value failed
    ///  --> const_format/src/macros/assertions/assertcp_macros.rs:297:14
    ///   |
    /// 8 | assertcp_ne!(NAME, "", "NAME must not be empty!");
    ///   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at '
    /// assertion failed: `(left != right)`
    ///  left: `""`
    /// right: `""`
    /// NAME must not be empty!
    /// ', const_format/src/macros/assertions/assertcp_macros.rs:8:14
    /// ```
    ///
    #[cfg_attr(feature = "__docsrs", doc(cfg(feature = "assertcp")))]
    #[macro_export]
    macro_rules! assertcp_ne {
        ($($parameters:tt)*) => (
            $crate::__assertcp_equality_inner!{
                ($($parameters)*)
                ($($parameters)*)
                ( != )
                ("!=")
            }
        );
    }
}
