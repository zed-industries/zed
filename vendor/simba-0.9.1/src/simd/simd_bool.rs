use crate::simd::SimdValue;
use std::ops::{BitAnd, BitOr, BitXor, Not};

/// Lane-wise generalization of `bool` for SIMD booleans.
///
/// This trait implemented by `bool` as well as SIMD boolean types like `portable_simd::mask32x4`.
/// It is designed to abstract the behavior of booleans so it can work with multi-lane boolean
/// values in an AoSoA setting.
pub trait SimdBool:
    Copy
    + BitAnd<Self, Output = Self>
    + BitOr<Self, Output = Self>
    + BitXor<Self, Output = Self>
    + Not<Output = Self>
{
    /// A bit mask representing the boolean state of each lanes of `self`.
    ///
    /// The `i-th` bit of the result is `1` iff. the `i-th` lane of `self` is `true`.
    fn bitmask(self) -> u64;
    /// Lane-wise bitwise and of the vector elements.
    fn and(self) -> bool;
    /// Lane-wise bitwise or of the vector elements.
    fn or(self) -> bool;
    /// Lane-wise bitwise xor of the vector elements.
    fn xor(self) -> bool;
    /// Are all vector lanes true?
    fn all(self) -> bool;
    /// Is any vector lane true?
    fn any(self) -> bool;
    /// Are all vector lanes false?
    fn none(self) -> bool;
    /// Merges the value of `if_value()` and `else_value()` depending on the lanes of `self`.
    ///
    /// - For each lane of `self` containing `1`, the result will contain the corresponding lane of `if_value()`.
    /// - For each lane of `self` containing `0`, the result will contain the corresponding lane of `else_value()`.
    ///
    /// The implementor of this trait is free to choose on what cases `if_value` and `else_value` are actually
    /// called.
    fn if_else<Res: SimdValue<SimdBool = Self>>(
        self,
        if_value: impl FnOnce() -> Res,
        else_value: impl FnOnce() -> Res,
    ) -> Res;

    /// Merges the value of `if_value()` and `else_if.1()` and `else_value()` depending on the lanes of `self` and `else_if.0()`.
    ///
    /// - For each lane of `self` containing `1`, the result will contain the corresponding lane of `if_value()`.
    /// - For each lane of `self` containing `0` but with a corresponding lane of `else_if.0()` containing `1`, the result will contain the corresponding lane of `else_if.1()`.
    /// - For each lane of `self` containing `0` but with a corresponding lane of `else_if.0()` containing `0`, the result will contain the corresponding lane of `else_value()`.
    ///
    /// The implementor of this trait is free to choose on what cases any of those closures are implemented.
    fn if_else2<Res: SimdValue<SimdBool = Self>>(
        self,
        if_value: impl FnOnce() -> Res,
        else_if: (impl FnOnce() -> Self, impl FnOnce() -> Res),
        else_value: impl FnOnce() -> Res,
    ) -> Res;

    /// Merges the value of `if_value()` and `else_if.1()` and `else_else_if.1()` and `else_value()` depending on the lanes of `self` and `else_if.0()` and `else_else_if.0()`.
    ///
    /// - For each lane of `self` containing `1`, the result will contain the corresponding lane of `if_value()`.
    /// - For each lane of `self` containing `0` but with a corresponding lane of `else_if.0()` containing `1`, the result will contain the corresponding lane of `else_if.1()`.
    /// - For each lane of `self` containing `0` and `else_if.0()` containing `0` and `else_else_if.0()` containing `1`, the result will contain the corresponding lane of `else_else_if.1()`.
    /// - Other lanes will contain the corresponding lane of `else_value()`.
    ///
    /// The implementor of this trait is free to choose on what cases any of those closures are implemented.
    fn if_else3<Res: SimdValue<SimdBool = Self>>(
        self,
        if_value: impl FnOnce() -> Res,
        else_if: (impl FnOnce() -> Self, impl FnOnce() -> Res),
        else_else_if: (impl FnOnce() -> Self, impl FnOnce() -> Res),
        else_value: impl FnOnce() -> Res,
    ) -> Res;
}

impl SimdBool for bool {
    #[inline(always)]
    fn bitmask(self) -> u64 {
        self as u64
    }

    #[inline(always)]
    fn and(self) -> bool {
        self
    }

    #[inline(always)]
    fn or(self) -> bool {
        self
    }

    #[inline(always)]
    fn xor(self) -> bool {
        self
    }

    #[inline(always)]
    fn all(self) -> bool {
        self
    }

    #[inline(always)]
    fn any(self) -> bool {
        self
    }

    #[inline(always)]
    fn none(self) -> bool {
        !self
    }

    #[inline(always)]
    fn if_else<Res: SimdValue<SimdBool = Self>>(
        self,
        if_value: impl FnOnce() -> Res,
        else_value: impl FnOnce() -> Res,
    ) -> Res {
        if self {
            if_value()
        } else {
            else_value()
        }
    }

    #[inline(always)]
    fn if_else2<Res: SimdValue<SimdBool = Self>>(
        self,
        if_value: impl FnOnce() -> Res,
        else_if: (impl FnOnce() -> Self, impl FnOnce() -> Res),
        else_value: impl FnOnce() -> Res,
    ) -> Res {
        if self {
            if_value()
        } else if else_if.0() {
            else_if.1()
        } else {
            else_value()
        }
    }

    #[inline(always)]
    fn if_else3<Res: SimdValue<SimdBool = Self>>(
        self,
        if_value: impl FnOnce() -> Res,
        else_if: (impl FnOnce() -> Self, impl FnOnce() -> Res),
        else_else_if: (impl FnOnce() -> Self, impl FnOnce() -> Res),
        else_value: impl FnOnce() -> Res,
    ) -> Res {
        if self {
            if_value()
        } else if else_if.0() {
            else_if.1()
        } else if else_else_if.0() {
            else_else_if.1()
        } else {
            else_value()
        }
    }
}
