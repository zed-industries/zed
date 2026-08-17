// #[rustfmt::skip] is being used because `rustfmt` poorly formats `#[doc = concat!(..)]`. See
// https://github.com/rust-lang/rustfmt/issues/5062 for more information.

use crate::{decimal::CalculationResult, ops, Decimal};
use core::ops::{Add, Div, Mul, Rem, Sub};
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedRem, CheckedSub, Inv};

// Macros and `Decimal` implementations

#[rustfmt::skip]
macro_rules! impl_checked {
    ($long:literal, $short:literal, $fun:ident, $impl:ident) => {
        #[doc = concat!(
            "Checked ",
            $long,
            ". Computes `self ",
            $short,
            " other`, returning `None` if overflow occurred."
        )]
        #[inline(always)]
        #[must_use]
        pub fn $fun(self, other: Decimal) -> Option<Decimal> {
            match ops::$impl(&self, &other) {
                CalculationResult::Ok(result) => Some(result),
                _ => None,
            }
        }
    };
}

#[rustfmt::skip]
macro_rules! impl_saturating {
    ($long:literal, $short:literal, $fun:ident, $impl:ident, $cmp:ident) => {
        #[doc = concat!(
            "Saturating ",
            $long,
            ". Computes `self ",
            $short,
            " other`, saturating at the relevant upper or lower boundary.",
        )]
        #[inline(always)]
        #[must_use]
        pub fn $fun(self, other: Decimal) -> Decimal {
            if let Some(elem) = self.$impl(other) {
                elem
            } else {
                $cmp(&self, &other)
            }
        }
    };
}

macro_rules! impl_checked_and_saturating {
    (
        $op_long:literal,
        $op_short:literal,
        $checked_fun:ident,
        $checked_impl:ident,

        $saturating_fun:ident,
        $saturating_cmp:ident
    ) => {
        impl_checked!($op_long, $op_short, $checked_fun, $checked_impl);
        impl_saturating!(
            $op_long,
            $op_short,
            $saturating_fun,
            $checked_fun,
            $saturating_cmp
        );
    };
}

impl Decimal {
    impl_checked_and_saturating!(
        "addition",
        "+",
        checked_add,
        add_impl,
        saturating_add,
        if_a_is_positive_then_max
    );
    impl_checked_and_saturating!(
        "multiplication",
        "*",
        checked_mul,
        mul_impl,
        saturating_mul,
        if_xnor_then_max
    );
    impl_checked_and_saturating!(
        "subtraction",
        "-",
        checked_sub,
        sub_impl,
        saturating_sub,
        if_a_is_positive_then_max
    );

    impl_checked!("division", "/", checked_div, div_impl);
    impl_checked!("remainder", "%", checked_rem, rem_impl);
}

// Macros and trait implementations

macro_rules! forward_all_binop {
    (impl $imp:ident for $res:ty, $method:ident) => {
        forward_val_val_binop!(impl $imp for $res, $method);
        forward_ref_val_binop!(impl $imp for $res, $method);
        forward_val_ref_binop!(impl $imp for $res, $method);
    };
}

macro_rules! forward_ref_val_binop {
    (impl $imp:ident for $res:ty, $method:ident) => {
        impl<'a> $imp<$res> for &'a $res {
            type Output = $res;

            #[inline]
            fn $method(self, other: $res) -> $res {
                self.$method(&other)
            }
        }
    };
}

macro_rules! forward_val_ref_binop {
    (impl $imp:ident for $res:ty, $method:ident) => {
        impl<'a> $imp<&'a $res> for $res {
            type Output = $res;

            #[inline]
            fn $method(self, other: &$res) -> $res {
                (&self).$method(other)
            }
        }
    };
}

macro_rules! forward_val_val_binop {
    (impl $imp:ident for $res:ty, $method:ident) => {
        impl $imp<$res> for $res {
            type Output = $res;

            #[inline]
            fn $method(self, other: $res) -> $res {
                (&self).$method(&other)
            }
        }
    };
}

forward_all_binop!(impl Add for Decimal, add);
impl Add<&Decimal> for &Decimal {
    type Output = Decimal;

    #[inline(always)]
    fn add(self, other: &Decimal) -> Decimal {
        match ops::add_impl(self, other) {
            CalculationResult::Ok(sum) => sum,
            _ => panic!("Addition overflowed"),
        }
    }
}

impl CheckedAdd for Decimal {
    #[inline]
    fn checked_add(&self, v: &Decimal) -> Option<Decimal> {
        Decimal::checked_add(*self, *v)
    }
}

impl CheckedSub for Decimal {
    #[inline]
    fn checked_sub(&self, v: &Decimal) -> Option<Decimal> {
        Decimal::checked_sub(*self, *v)
    }
}

impl CheckedMul for Decimal {
    #[inline]
    fn checked_mul(&self, v: &Decimal) -> Option<Decimal> {
        Decimal::checked_mul(*self, *v)
    }
}

impl CheckedDiv for Decimal {
    #[inline]
    fn checked_div(&self, v: &Decimal) -> Option<Decimal> {
        Decimal::checked_div(*self, *v)
    }
}

impl CheckedRem for Decimal {
    #[inline]
    fn checked_rem(&self, v: &Decimal) -> Option<Decimal> {
        Decimal::checked_rem(*self, *v)
    }
}

impl Inv for Decimal {
    type Output = Self;

    #[inline]
    fn inv(self) -> Self {
        Decimal::ONE / self
    }
}

forward_all_binop!(impl Div for Decimal, div);
impl Div<&Decimal> for &Decimal {
    type Output = Decimal;

    #[inline]
    fn div(self, other: &Decimal) -> Decimal {
        match ops::div_impl(self, other) {
            CalculationResult::Ok(quot) => quot,
            CalculationResult::Overflow => panic!("Division overflowed"),
            CalculationResult::DivByZero => panic!("Division by zero"),
        }
    }
}

forward_all_binop!(impl Mul for Decimal, mul);
impl Mul<&Decimal> for &Decimal {
    type Output = Decimal;

    #[inline]
    fn mul(self, other: &Decimal) -> Decimal {
        match ops::mul_impl(self, other) {
            CalculationResult::Ok(prod) => prod,
            _ => panic!("Multiplication overflowed"),
        }
    }
}

forward_all_binop!(impl Rem for Decimal, rem);
impl Rem<&Decimal> for &Decimal {
    type Output = Decimal;

    #[inline]
    fn rem(self, other: &Decimal) -> Decimal {
        match ops::rem_impl(self, other) {
            CalculationResult::Ok(rem) => rem,
            CalculationResult::Overflow => panic!("Division overflowed"),
            CalculationResult::DivByZero => panic!("Division by zero"),
        }
    }
}

forward_all_binop!(impl Sub for Decimal, sub);
impl Sub<&Decimal> for &Decimal {
    type Output = Decimal;

    #[inline(always)]
    fn sub(self, other: &Decimal) -> Decimal {
        match ops::sub_impl(self, other) {
            CalculationResult::Ok(sum) => sum,
            _ => panic!("Subtraction overflowed"),
        }
    }
}

// This function signature is expected by `impl_saturating`, thus the reason of `_b`.
#[inline(always)]
const fn if_a_is_positive_then_max(a: &Decimal, _b: &Decimal) -> Decimal {
    if a.is_sign_positive() {
        Decimal::MAX
    } else {
        Decimal::MIN
    }
}

// Used by saturating multiplications.
//
// If the `a` and `b` combination represents a XNOR bit operation, returns MAX. Otherwise,
// returns MIN.
#[inline(always)]
const fn if_xnor_then_max(a: &Decimal, b: &Decimal) -> Decimal {
    match (a.is_sign_positive(), b.is_sign_positive()) {
        (true, true) => Decimal::MAX,
        (true, false) => Decimal::MIN,
        (false, true) => Decimal::MIN,
        (false, false) => Decimal::MAX,
    }
}

#[cfg(test)]
mod tests {
    use crate::Decimal;

    #[test]
    fn checked_methods_have_correct_output() {
        assert_eq!(Decimal::MAX.checked_add(Decimal::MAX), None);
        assert_eq!(Decimal::MAX.checked_add(Decimal::MIN), Some(Decimal::ZERO));
        assert_eq!(Decimal::MAX.checked_div(Decimal::ZERO), None);
        assert_eq!(Decimal::MAX.checked_mul(Decimal::MAX), None);
        assert_eq!(Decimal::MAX.checked_mul(Decimal::MIN), None);
        assert_eq!(Decimal::MAX.checked_rem(Decimal::ZERO), None);
        assert_eq!(Decimal::MAX.checked_sub(Decimal::MAX), Some(Decimal::ZERO));
        assert_eq!(Decimal::MAX.checked_sub(Decimal::MIN), None);

        assert_eq!(Decimal::MIN.checked_add(Decimal::MAX), Some(Decimal::ZERO));
        assert_eq!(Decimal::MIN.checked_add(Decimal::MIN), None);
        assert_eq!(Decimal::MIN.checked_div(Decimal::ZERO), None);
        assert_eq!(Decimal::MIN.checked_mul(Decimal::MAX), None);
        assert_eq!(Decimal::MIN.checked_mul(Decimal::MIN), None);
        assert_eq!(Decimal::MIN.checked_rem(Decimal::ZERO), None);
        assert_eq!(Decimal::MIN.checked_sub(Decimal::MAX), None);
        assert_eq!(Decimal::MIN.checked_sub(Decimal::MIN), Some(Decimal::ZERO));
    }

    #[test]
    fn saturated_methods_have_correct_output() {
        assert_eq!(Decimal::MAX.saturating_add(Decimal::MAX), Decimal::MAX);
        assert_eq!(Decimal::MAX.saturating_add(Decimal::MIN), Decimal::ZERO);
        assert_eq!(Decimal::MAX.saturating_mul(Decimal::MAX), Decimal::MAX);
        assert_eq!(Decimal::MAX.saturating_mul(Decimal::MIN), Decimal::MIN);
        assert_eq!(Decimal::MAX.saturating_sub(Decimal::MAX), Decimal::ZERO);
        assert_eq!(Decimal::MAX.saturating_sub(Decimal::MIN), Decimal::MAX);

        assert_eq!(Decimal::MIN.saturating_add(Decimal::MAX), Decimal::ZERO);
        assert_eq!(Decimal::MIN.saturating_add(Decimal::MIN), Decimal::MIN);
        assert_eq!(Decimal::MIN.saturating_mul(Decimal::MAX), Decimal::MIN);
        assert_eq!(Decimal::MIN.saturating_mul(Decimal::MIN), Decimal::MAX);
        assert_eq!(Decimal::MIN.saturating_sub(Decimal::MAX), Decimal::MIN);
        assert_eq!(Decimal::MIN.saturating_sub(Decimal::MIN), Decimal::ZERO);
    }
}
