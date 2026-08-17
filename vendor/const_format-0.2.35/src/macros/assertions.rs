#[cfg(feature = "assertc")]
mod assertc_macros;

#[cfg(feature = "assertcp")]
mod assertcp_macros;

#[doc(hidden)]
#[macro_export]
macro_rules! __assertc_inner {
    (
        $fmt_macro:ident
        ($($parameters:tt)*)
        ($cond:expr $(, $fmt_literal:expr $(,$fmt_arg:expr)*)? $(,)?)
    ) => {
        #[allow(non_snake_case)]
        const _: () = {
            use $crate::__cf_osRcTFl4A;

            $crate::__assertc_common!{
                $fmt_macro
                ($($parameters)*)
                ($cond)
                (
                    concat!(
                        "\nassertion failed.\n",
                        $($fmt_literal,)?
                        "\n",
                    )
                    $($(,$fmt_arg)*)?
                )
            }
        };
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __assertc_common {
    (
        $fmt_macro:ident
        ($($span:tt)*)
        ($cond:expr)
        ($($fmt_literal:expr $(,$fmt_arg:expr)*)?)
    ) => (
        const PANIC_IF_TRUE_NHPMWYD3NJA: bool = !($cond);

        const MSG_NHPMWYD3NJA: &str = $crate::pmr::$fmt_macro!(
            (PANIC_IF_TRUE_NHPMWYD3NJA)
            ($($fmt_literal,)?),
            $($($fmt_arg,)*)?
        );

        __cf_osRcTFl4A::pmr::respan_to!{
            ($($span)*)
            __cf_osRcTFl4A::pmr::assert_(PANIC_IF_TRUE_NHPMWYD3NJA, MSG_NHPMWYD3NJA)
        }
    );
}
