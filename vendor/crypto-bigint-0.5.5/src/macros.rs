//! Macro definitions which are a part of the public API.

/// Internal implementation detail of [`const_assert_eq`] and [`const_assert_ne`].
#[doc(hidden)]
#[macro_export]
macro_rules! const_assert_n {
    ($n:expr, $($arg:tt)*) => {{
        // TODO(tarcieri): gensym a name so it's unique per invocation of the macro?
        mod __const_assert {
            pub(super) struct Assert<const N: usize>;

            impl<const N: usize> Assert<N> {
                pub(super) const ASSERT: () = assert!($($arg)*);
            }
        }

        __const_assert::Assert::<$n>::ASSERT
    }};
}

/// Const-friendly assertion that two values are equal.
///
/// ```
/// const _: () = crypto_bigint::const_assert_eq!(0, 0, "zero equals zero");
/// ```
#[macro_export]
macro_rules! const_assert_eq {
    ($left:expr, $right:expr $(,)?) => (
        $crate::const_assert_n!($left, $left == $right)
    );
    ($left:expr, $right:expr, $($arg:tt)+) => (
        $crate::const_assert_n!($left, $left == $right, $($arg)+)
    );
}

/// Const-friendly assertion that two values are NOT equal.
///
/// ```
/// const _: () = crypto_bigint::const_assert_ne!(0, 1, "zero is NOT equal to one");
/// ```
#[macro_export]
macro_rules! const_assert_ne {
    ($left:expr, $right:expr $(,)?) => (
        $crate::const_assert_n!($left, $left != $right)
    );
    ($left:expr, $right:expr, $($arg:tt)+) => (
        $crate::const_assert_n!($left, $left != $right, $($arg)+)
    );
}

/// Calculate the number of limbs required to represent the given number of bits.
// TODO(tarcieri): replace with `generic_const_exprs` (rust-lang/rust#76560) when stable
#[macro_export]
macro_rules! nlimbs {
    ($bits:expr) => {
        $bits / $crate::Limb::BITS
    };
}

#[cfg(test)]
mod tests {
    #[cfg(target_pointer_width = "32")]
    #[test]
    fn nlimbs_for_bits_macro() {
        assert_eq!(nlimbs!(64), 2);
        assert_eq!(nlimbs!(128), 4);
        assert_eq!(nlimbs!(192), 6);
        assert_eq!(nlimbs!(256), 8);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn nlimbs_for_bits_macro() {
        assert_eq!(nlimbs!(64), 1);
        assert_eq!(nlimbs!(128), 2);
        assert_eq!(nlimbs!(192), 3);
        assert_eq!(nlimbs!(256), 4);
    }
}
