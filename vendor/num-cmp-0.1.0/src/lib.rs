//! The **[`NumCmp`](./trait.NumCmp.html)** trait for comparison between differently typed numbers.
//!
//! ```rust
//! use std::f32;
//! use std::cmp::Ordering;
//! use num_cmp::NumCmp;
//!
//! assert!(NumCmp::num_eq(3u64, 3.0f32));
//! assert!(NumCmp::num_lt(-4.7f64, -4i8));
//! assert!(!NumCmp::num_ge(-3i8, 1u16));
//!
//! // 40_000_000 can be exactly represented in f32, 40_000_001 cannot
//! assert_eq!(NumCmp::num_cmp(40_000_000.0f32, 40_000_000u32), Some(Ordering::Equal));
//! assert_ne!(NumCmp::num_cmp(40_000_001.0f32, 40_000_001u32), Some(Ordering::Equal));
//! assert_eq!(NumCmp::num_cmp(f32::NAN,        40_000_002u32), None);
//! ```
//!
//! The `i128` Cargo feature can be enabled in nightly
//! to get support for `i128` and `u128` types as well,
//! which is being implemented in [Rust issue #35118][issue-35118].
//!
//! [issue-35118]: https://github.com/rust-lang/rust/issues/35118

#![cfg_attr(feature = "i128", feature(i128_type))]
#![deny(missing_docs)]

use std::cmp::Ordering;

/// A trait for comparison between differently typed numbers.
///
/// This trait is implemented for every pair of integer and floating-point types available,
/// including `isize`, `usize` and also (when the `i128` feature is enabled) `i128` and `u128`.
pub trait NumCmp<Other: Copy>: Copy {
    // only used for testing
    #[cfg(test)] fn num_cmp_strategy(self, other: Other) -> &'static str;

    /// Same to `self.partial_cmp(&other)`
    /// but can be used for different numeric types for `self` and `other`.
    fn num_cmp(self, other: Other) -> Option<Ordering>;

    /// Same to `self == other` but can be used for different numeric types for `self` and `other`.
    fn num_eq(self, other: Other) -> bool;

    /// Same to `self != other` but can be used for different numeric types for `self` and `other`.
    fn num_ne(self, other: Other) -> bool;

    /// Same to `self < other` but can be used for different numeric types for `self` and `other`.
    fn num_lt(self, other: Other) -> bool;

    /// Same to `self > other` but can be used for different numeric types for `self` and `other`.
    fn num_gt(self, other: Other) -> bool;

    /// Same to `self <= other` but can be used for different numeric types for `self` and `other`.
    fn num_le(self, other: Other) -> bool;

    /// Same to `self >= other` but can be used for different numeric types for `self` and `other`.
    fn num_ge(self, other: Other) -> bool;
}

// strategy 1: for the same type T, delegate to normal operators.
macro_rules! impl_for_equal_types {
    ($($ty:ty;)*) => ($(
        impl NumCmp<$ty> for $ty {
            #[cfg(test)]
            fn num_cmp_strategy(self, _other: $ty) -> &'static str {
                "strategy 1"
            }

            #[inline]
            fn num_cmp(self, other: $ty) -> Option<Ordering> {
                self.partial_cmp(&other)
            }

            #[inline]
            fn num_eq(self, other: $ty) -> bool {
                self == other
            }

            #[inline]
            fn num_ne(self, other: $ty) -> bool {
                self != other
            }

            #[inline]
            fn num_lt(self, other: $ty) -> bool {
                self < other
            }

            #[inline]
            fn num_gt(self, other: $ty) -> bool {
                self > other
            }

            #[inline]
            fn num_le(self, other: $ty) -> bool {
                self <= other
            }

            #[inline]
            fn num_ge(self, other: $ty) -> bool {
                self >= other
            }
        }
    )*);
}

// strategy 2: for two types where one of them is isize or usize,
// delegate to implementations for the equivalently sized types.
macro_rules! impl_for_size_types {
    ($($size:ty => $nonsize:ty, $other:ty;)*) => ($(
        impl NumCmp<$other> for $size {
            #[cfg(test)]
            fn num_cmp_strategy(self, _other: $other) -> &'static str {
                "strategy 2, size vs other"
            }

            #[inline]
            fn num_cmp(self, other: $other) -> Option<Ordering> {
                (self as $nonsize).num_cmp(other)
            }

            #[inline]
            fn num_eq(self, other: $other) -> bool {
                (self as $nonsize).num_eq(other)
            }

            #[inline]
            fn num_ne(self, other: $other) -> bool {
                (self as $nonsize).num_ne(other)
            }

            #[inline]
            fn num_lt(self, other: $other) -> bool {
                (self as $nonsize).num_lt(other)
            }

            #[inline]
            fn num_gt(self, other: $other) -> bool {
                (self as $nonsize).num_gt(other)
            }

            #[inline]
            fn num_le(self, other: $other) -> bool {
                (self as $nonsize).num_le(other)
            }

            #[inline]
            fn num_ge(self, other: $other) -> bool {
                (self as $nonsize).num_ge(other)
            }
        }

        impl NumCmp<$size> for $other {
            #[cfg(test)]
            fn num_cmp_strategy(self, _other: $size) -> &'static str {
                "strategy 2, nonsize vs size"
            }

            #[inline]
            fn num_cmp(self, other: $size) -> Option<Ordering> {
                self.num_cmp(other as $nonsize)
            }

            #[inline]
            fn num_eq(self, other: $size) -> bool {
                self.num_eq(other as $nonsize)
            }

            #[inline]
            fn num_ne(self, other: $size) -> bool {
                self.num_ne(other as $nonsize)
            }

            #[inline]
            fn num_lt(self, other: $size) -> bool {
                self.num_lt(other as $nonsize)
            }

            #[inline]
            fn num_gt(self, other: $size) -> bool {
                self.num_gt(other as $nonsize)
            }

            #[inline]
            fn num_le(self, other: $size) -> bool {
                self.num_le(other as $nonsize)
            }

            #[inline]
            fn num_ge(self, other: $size) -> bool {
                self.num_ge(other as $nonsize)
            }
        }
    )*);
}

// strategy 3: for two types T and U,
// (without loss of generality) when T can be always exactly casted to U,
// compare them by casting T to U.
macro_rules! impl_for_nonequal_types_with_casting {
    ($($big:ty, $small:ty;)*) => ($(
        impl NumCmp<$small> for $big {
            #[cfg(test)]
            fn num_cmp_strategy(self, _other: $small) -> &'static str {
                "strategy 3, big vs small"
            }

            #[inline]
            fn num_cmp(self, other: $small) -> Option<Ordering> {
                self.partial_cmp(&(other as $big))
            }

            #[inline]
            fn num_eq(self, other: $small) -> bool {
                self == other as $big
            }

            #[inline]
            fn num_ne(self, other: $small) -> bool {
                self != other as $big
            }

            #[inline]
            fn num_lt(self, other: $small) -> bool {
                self < other as $big
            }

            #[inline]
            fn num_gt(self, other: $small) -> bool {
                self > other as $big
            }

            #[inline]
            fn num_le(self, other: $small) -> bool {
                self <= other as $big
            }

            #[inline]
            fn num_ge(self, other: $small) -> bool {
                self >= other as $big
            }
        }

        impl NumCmp<$big> for $small {
            #[cfg(test)]
            fn num_cmp_strategy(self, _other: $big) -> &'static str {
                "strategy 3, small vs big"
            }

            #[inline]
            fn num_cmp(self, other: $big) -> Option<Ordering> {
                (self as $big).partial_cmp(&other)
            }

            #[inline]
            fn num_eq(self, other: $big) -> bool {
                self as $big == other
            }

            #[inline]
            fn num_ne(self, other: $big) -> bool {
                self as $big != other
            }

            #[inline]
            fn num_lt(self, other: $big) -> bool {
                (self as $big) < other
            }

            #[inline]
            fn num_gt(self, other: $big) -> bool {
                self as $big > other
            }

            #[inline]
            fn num_le(self, other: $big) -> bool {
                self as $big <= other
            }

            #[inline]
            fn num_ge(self, other: $big) -> bool {
                self as $big >= other
            }
        }
    )*);
}

// strategy 4: for unsigned type T and signed type U,
// if bit size of T is no less than that of U, 
// check if both operands are positive before doing the normal comparison in unsigned type.
macro_rules! impl_for_nonequal_types_with_different_signedness {
    ($($unsigned:ty, $signed:ty;)*) => ($(
        impl NumCmp<$signed> for $unsigned {
            #[cfg(test)]
            fn num_cmp_strategy(self, _other: $signed) -> &'static str {
                "strategy 4, unsigned vs signed"
            }

            #[inline]
            fn num_cmp(self, other: $signed) -> Option<Ordering> {
                if other < 0 {
                    Some(Ordering::Greater)
                } else {
                    self.partial_cmp(&(other as $unsigned))
                }
            }

            #[inline]
            fn num_eq(self, other: $signed) -> bool {
                other >= 0 && self == other as $unsigned
            }

            #[inline]
            fn num_ne(self, other: $signed) -> bool {
                other < 0 || self != other as $unsigned
            }

            #[inline]
            fn num_lt(self, other: $signed) -> bool {
                other > 0 && self < other as $unsigned
            }

            #[inline]
            fn num_gt(self, other: $signed) -> bool {
                other < 0 || self > other as $unsigned
            }

            #[inline]
            fn num_le(self, other: $signed) -> bool {
                other >= 0 && self <= other as $unsigned
            }

            #[inline]
            fn num_ge(self, other: $signed) -> bool {
                other <= 0 || self >= other as $unsigned
            }
        }

        impl NumCmp<$unsigned> for $signed {
            #[cfg(test)]
            fn num_cmp_strategy(self, _other: $unsigned) -> &'static str {
                "strategy 4, signed vs unsigned"
            }

            #[inline]
            fn num_cmp(self, other: $unsigned) -> Option<Ordering> {
                if self < 0 {
                    Some(Ordering::Less)
                } else {
                    (self as $unsigned).partial_cmp(&other)
                }
            }

            #[inline]
            fn num_eq(self, other: $unsigned) -> bool {
                self >= 0 && self as $unsigned == other
            }

            #[inline]
            fn num_ne(self, other: $unsigned) -> bool {
                self < 0 || self as $unsigned != other
            }

            #[inline]
            fn num_lt(self, other: $unsigned) -> bool {
                self < 0 || (self as $unsigned) < other
            }

            #[inline]
            fn num_gt(self, other: $unsigned) -> bool {
                self > 0 && self as $unsigned > other
            }

            #[inline]
            fn num_le(self, other: $unsigned) -> bool {
                self <= 0 || self as $unsigned <= other
            }

            #[inline]
            fn num_ge(self, other: $unsigned) -> bool {
                self >= 0 && self as $unsigned >= other
            }
        }
    )*);
}

// strategy 5: for two types T and U,
// when T can be casted to U but it may result in precision loss,
// first bound the operand in type U to the domain of type T;
// when testing equality the bound failure means the inequality,
// otherwise we convert to the appropriate value in type T so that the comparison can continue.
//
// since all integral conversion does not lose precision (but can be out of range),
// T is guaranteed to be integral while U is guaranteed to be floating-point.
// it is possible that bounds themselves can be overflown (especially when T=u128, U=f32).
//
// for general comparison we have the following useful observation:
//
//     where `a cmp b` is a general partial ordering operator (like `PartialOrd::partial_cmp`)
//     and `trunc(x)` is `x` rounded towards zero (i.e. trunc(3.5) = 3, trunc(-3.5) = -3),
//     if `a` is an integer, `a cmp b` equals to `(a, trunc(b)) cmp (trunc(b), b)`.
//     (we assume that the tuple is ordered lexicographically.)
//
//     the proof involves an equality `floor(x) <= trunc(x) <= ceil(x)`,
//     and inequalities `n < x <=> n < ceil(x)` and `x < n <=> floor(x) < n` for integer `n`.
//     when `a < trunc(b)` it follows that `a < trunc(b) <= ceil(b)`, which implies `a < b`;
//     when `a > trunc(b)` it follows that `a > trunc(b) >= floor(b)`, which implies `a > b`;
//     when `a = trunc(b)` any inequality `trunc(b) op b` follows that `a = trunc(b) op b`,
//     which clearly implies `a op b` as intended. Q.E.D.
//
// since `a` and `trunc(b)` can be made into the same type after bounds checking,
// this gives very uniform and simple way to compare numbers.
// we of course rely on the fact that the range of `trunc(a)` is smaller than the domain of `a`.

// requires that the float operand is not NaN and in the ($lb, $ub) range
macro_rules! trunc_cmp {
    (int $lhs:expr, $method:ident, float $rhs:expr;
     ($lb:expr) <= ($intty:ty) < ($ub:expr)) => ({
        let rhsint = $rhs.trunc();
        debug_assert!($lb <= rhsint && rhsint < $ub);
        ($lhs, rhsint).$method(&(rhsint as $intty, $rhs))
    });

    (float $lhs:expr, $method:ident, int $rhs:expr;
     ($lb:expr) <= ($intty:ty) < ($ub:expr)) => ({
        let lhsint = $lhs.trunc();
        debug_assert!($lb <= lhsint && lhsint < $ub);
        (lhsint as $intty, $lhs).$method(&($rhs, lhsint))
    });
}

macro_rules! impl_for_int_and_float_types_with_bounds_check {
    ($($float:ty, $int:ty, ($lb:expr) <= _ < ($ub:expr);)*) => ($(
        impl NumCmp<$int> for $float {
            #[cfg(test)]
            fn num_cmp_strategy(self, _other: $int) -> &'static str {
                "strategy 5, float vs int"
            }

            #[inline]
            fn num_cmp(self, other: $int) -> Option<Ordering> {
                if self < $lb {
                    Some(Ordering::Less)
                } else if $ub <= self {
                    Some(Ordering::Greater)
                } else if self == self {
                    trunc_cmp!(float self, partial_cmp, int other; ($lb) <= ($int) < ($ub))
                } else {
                    None
                }
            }

            #[inline]
            fn num_eq(self, other: $int) -> bool {
                $lb <= self && self < $ub && trunc_cmp!(float self, eq, int other;
                                                        ($lb) <= ($int) < ($ub))
            }

            #[inline]
            fn num_ne(self, other: $int) -> bool {
                // we cannot blindly apply De Morgan's law; we need to catch NaN early on
                !($lb <= self && self < $ub) || trunc_cmp!(float self, ne, int other;
                                                           ($lb) <= ($int) < ($ub))
            }

            #[inline]
            fn num_lt(self, other: $int) -> bool {
                self < $ub && (self < $lb || trunc_cmp!(float self, lt, int other;
                                                        ($lb) <= ($int) < ($ub)))
            }

            #[inline]
            fn num_gt(self, other: $int) -> bool {
                $lb <= self && ($ub <= self || trunc_cmp!(float self, gt, int other;
                                                          ($lb) <= ($int) < ($ub)))
            }

            #[inline]
            fn num_le(self, other: $int) -> bool {
                self < $ub && (self < $lb || trunc_cmp!(float self, le, int other;
                                                        ($lb) <= ($int) < ($ub)))
            }

            #[inline]
            fn num_ge(self, other: $int) -> bool {
                $lb <= self && ($ub <= self || trunc_cmp!(float self, ge, int other;
                                                          ($lb) <= ($int) < ($ub)))
            }
        }

        impl NumCmp<$float> for $int {
            #[cfg(test)]
            fn num_cmp_strategy(self, _other: $float) -> &'static str {
                "strategy 5, int vs float"
            }

            #[inline]
            fn num_cmp(self, other: $float) -> Option<Ordering> {
                if other < $lb {
                    Some(Ordering::Greater)
                } else if $ub <= other {
                    Some(Ordering::Less)
                } else if other == other {
                    trunc_cmp!(int self, partial_cmp, float other; ($lb) <= ($int) < ($ub))
                } else {
                    None
                }
            }

            #[inline]
            fn num_eq(self, other: $float) -> bool {
                $lb <= other && other < $ub && trunc_cmp!(int self, eq, float other;
                                                          ($lb) <= ($int) < ($ub))
            }

            #[inline]
            fn num_ne(self, other: $float) -> bool {
                // we cannot blindly apply De Morgan's law; we need to catch NaN early on
                !($lb <= other && other < $ub) || trunc_cmp!(int self, ne, float other;
                                                             ($lb) <= ($int) < ($ub))
            }

            #[inline]
            fn num_lt(self, other: $float) -> bool {
                $lb <= other && ($ub <= other || trunc_cmp!(int self, lt, float other;
                                                            ($lb) <= ($int) < ($ub)))
            }

            #[inline]
            fn num_gt(self, other: $float) -> bool {
                other < $ub && (other < $lb || trunc_cmp!(int self, gt, float other;
                                                          ($lb) <= ($int) < ($ub)))
            }

            #[inline]
            fn num_le(self, other: $float) -> bool {
                $lb <= other && ($ub <= other || trunc_cmp!(int self, le, float other;
                                                            ($lb) <= ($int) < ($ub)))
            }

            #[inline]
            fn num_ge(self, other: $float) -> bool {
                other < $ub && (other < $lb || trunc_cmp!(int self, ge, float other;
                                                          ($lb) <= ($int) < ($ub)))
            }
        }
    )*);
}

// actual implementations.
// there should be 12 * 12 = 144 implementations for the NumCmp trait
// (or 14 * 14 = 196 implementations if the `i128` feature is enabled).

impl_for_equal_types! {
    u8; u16; u32; u64; usize;
    i8; i16; i32; i64; isize;
    f32; f64;
}

#[cfg(feature = "i128")]
impl_for_equal_types! {
    u128;
    i128;
}

#[cfg(target_pointer_width = "32")]
impl_for_size_types! {
    usize => u32, u8;  isize => i32, u8;
    usize => u32, u16; isize => i32, u16;
    usize => u32, u32; isize => i32, u32;
    usize => u32, u64; isize => i32, u64;
    usize => u32, i8;  isize => i32, i8;
    usize => u32, i16; isize => i32, i16;
    usize => u32, i32; isize => i32, i32;
    usize => u32, i64; isize => i32, i64;
    usize => u32, f32; isize => i32, f32;
    usize => u32, f64; isize => i32, f64;
}

#[cfg(target_pointer_width = "64")]
impl_for_size_types! {
    usize => u64, u8;  isize => i64, u8;
    usize => u64, u16; isize => i64, u16;
    usize => u64, u32; isize => i64, u32;
    usize => u64, u64; isize => i64, u64;
    usize => u64, i8;  isize => i64, i8;
    usize => u64, i16; isize => i64, i16;
    usize => u64, i32; isize => i64, i32;
    usize => u64, i64; isize => i64, i64;
    usize => u64, f32; isize => i64, f32;
    usize => u64, f64; isize => i64, f64;
}

#[cfg(all(target_pointer_width = "32", feature = "i128"))]
impl_for_size_types! {
    usize => u32, u128; isize => i32, u128;
    usize => u32, i128; isize => i32, i128;
}

#[cfg(all(target_pointer_width = "64", feature = "i128"))]
impl_for_size_types! {
    usize => u64, u128; isize => i64, u128;
    usize => u64, i128; isize => i64, i128;
}

impl_for_nonequal_types_with_casting! {
    // uN, uM for N > M
    u64, u8;  u32, u8;  u16, u8;
    u64, u16; u32, u16;
    u64, u32;

    // iN, iM for N > M
    i64, i8;  i32, i8;  i16, i8;
    i64, i16; i32, i16;
    i64, i32;

    // iN, uM for N > M
    i64, u8;  i32, u8;  i16, u8;
    i64, u16; i32, u16;
    i64, u32;

    // fN, fM for N > M
    f64, f32;

    // f32, uM for 24 >= M, since f32 can exactly represent all integers (-2^24,2^24)
    // f64, uM for 53 >= M, since f64 can exactly represent all integers (-2^53,2^53)
    f32, u8; f32, u16;
    f64, u8; f64, u16; f64, u32;

    // f32, iM for 24 >= M
    // f64, iM for 53 >= M
    // since iM's range [-2^(M-1),2^(M-1)) includes -2^(M-1), bounds do not change
    f32, i8; f32, i16;
    f64, i8; f64, i16; f64, i32;
}

#[cfg(feature = "i128")]
impl_for_nonequal_types_with_casting! {
    u128, u8; u128, u16; u128, u32; u128, u64;
    i128, u8; i128, u16; i128, u32; i128, u64;
    i128, i8; i128, i16; i128, i32; i128, i64;
}

impl_for_nonequal_types_with_different_signedness! {
    u64, i8;  u32, i8;  u16, i8;  u8, i8;
    u64, i16; u32, i16; u16, i16;
    u64, i32; u32, i32;
    u64, i64;
    usize, isize;
}

#[cfg(feature = "i128")]
impl_for_nonequal_types_with_different_signedness! {
    u128, i8;
    u128, i16;
    u128, i32;
    u128, i64;
    u128, i128;
}

const U32_BOUND_IN_F32: f32 = 4294967296.0;
const I32_BOUND_IN_F32: f32 = 2147483648.0;

const U64_BOUND_IN_F32: f32 = 18446744073709551616.0;
const U64_BOUND_IN_F64: f64 = 18446744073709551616.0;
const I64_BOUND_IN_F32: f32 = 9223372036854775808.0;
const I64_BOUND_IN_F64: f64 = 9223372036854775808.0;

impl_for_int_and_float_types_with_bounds_check! {
    // f32, uM for 24 < M
    // f64, uM for 53 < M
    f32, u32, (0.0) <= _ < (U32_BOUND_IN_F32);
    f32, u64, (0.0) <= _ < (U64_BOUND_IN_F32);
    f64, u64, (0.0) <= _ < (U64_BOUND_IN_F64);

    // f32, iM for 24 < M
    // f64, iM for 53 < M
    f32, i32, (-I32_BOUND_IN_F32) <= _ < (I32_BOUND_IN_F32);
    f32, i64, (-I64_BOUND_IN_F32) <= _ < (I64_BOUND_IN_F32);
    f64, i64, (-I64_BOUND_IN_F64) <= _ < (I64_BOUND_IN_F64);
}

#[cfg(feature = "i128")] const U128_BOUND_IN_F32: f32 = std::f32::INFINITY;
#[cfg(feature = "i128")] const U128_BOUND_IN_F64: f64 = 340282366920938463463374607431768211456.0;
#[cfg(feature = "i128")] const I128_BOUND_IN_F32: f32 = 170141183460469231731687303715884105728.0;
#[cfg(feature = "i128")] const I128_BOUND_IN_F64: f64 = 170141183460469231731687303715884105728.0;

#[cfg(feature = "i128")]
impl_for_int_and_float_types_with_bounds_check! {
    f32, u128, (0.0) <= _ < (U128_BOUND_IN_F32);
    f64, u128, (0.0) <= _ < (U128_BOUND_IN_F64);
    f32, i128, (-I128_BOUND_IN_F32) <= _ < (I128_BOUND_IN_F32);
    f64, i128, (-I128_BOUND_IN_F64) <= _ < (I128_BOUND_IN_F64);
}

#[cfg(test)] mod tests;

