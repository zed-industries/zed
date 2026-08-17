//! Pulley registers.

use crate::U6;
use core::hash::Hash;
use core::marker::PhantomData;
use core::{fmt, ops::Range};

use cranelift_bitset::ScalarBitSet;

/// Trait for common register operations.
pub trait Reg: Sized + Copy + Eq + Ord + Hash + Into<AnyReg> + fmt::Debug + fmt::Display {
    /// Range of valid register indices.
    const RANGE: Range<u8>;

    /// Convert a register index to a register, without bounds checking.
    unsafe fn new_unchecked(index: u8) -> Self;

    /// Convert a register index to a register, with bounds checking.
    fn new(index: u8) -> Option<Self> {
        if Self::RANGE.contains(&index) {
            Some(unsafe { Self::new_unchecked(index) })
        } else {
            None
        }
    }

    /// Convert a register to its index.
    fn to_u8(self) -> u8;

    /// Convert a register to its index.
    fn index(self) -> usize {
        self.to_u8().into()
    }
}

macro_rules! impl_reg {
    ($reg_ty:ty, $any:ident, $range:expr) => {
        impl From<$reg_ty> for AnyReg {
            fn from(r: $reg_ty) -> Self {
                AnyReg::$any(r)
            }
        }

        impl fmt::Display for $reg_ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(&self, f)
            }
        }

        impl Reg for $reg_ty {
            const RANGE: Range<u8> = $range;

            unsafe fn new_unchecked(index: u8) -> Self {
                unsafe { core::mem::transmute(index) }
            }

            fn to_u8(self) -> u8 {
                self as u8
            }
        }
    };
}

/// An `x` register: integers.
#[repr(u8)]
#[derive(Debug,Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[expect(missing_docs, reason = "self-describing variants")]
#[expect(non_camel_case_types, reason = "matching in-asm register names")]
#[rustfmt::skip]
pub enum XReg {
    x0,  x1,  x2,  x3,  x4,  x5,  x6,  x7,  x8,  x9,
    x10, x11, x12, x13, x14, x15, x16, x17, x18, x19,
    x20, x21, x22, x23, x24, x25, x26, x27, x28, x29,

    /// The special `sp` stack pointer register.
    sp,

    /// The special `spilltmp0` scratch register.
    spilltmp0,

}

impl XReg {
    /// Index of the first "special" register.
    pub const SPECIAL_START: u8 = XReg::sp as u8;

    /// Is this `x` register a special register?
    pub fn is_special(self) -> bool {
        matches!(self, Self::sp | Self::spilltmp0)
    }
}

#[test]
fn assert_special_start_is_right() {
    for i in 0..XReg::SPECIAL_START {
        assert!(!XReg::new(i).unwrap().is_special());
    }
    for i in XReg::SPECIAL_START.. {
        match XReg::new(i) {
            Some(r) => assert!(r.is_special()),
            None => break,
        }
    }
}

/// An `f` register: floats.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[expect(missing_docs, reason = "self-describing variants")]
#[expect(non_camel_case_types, reason = "matching in-asm register names")]
#[rustfmt::skip]
pub enum FReg {
    f0,  f1,  f2,  f3,  f4,  f5,  f6,  f7,  f8,  f9,
    f10, f11, f12, f13, f14, f15, f16, f17, f18, f19,
    f20, f21, f22, f23, f24, f25, f26, f27, f28, f29,
    f30, f31,
}

/// A `v` register: vectors.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[expect(missing_docs, reason = "self-describing variants")]
#[expect(non_camel_case_types, reason = "matching in-asm register names")]
#[rustfmt::skip]
pub enum VReg {
    v0,  v1,  v2,  v3,  v4,  v5,  v6,  v7,  v8,  v9,
    v10, v11, v12, v13, v14, v15, v16, v17, v18, v19,
    v20, v21, v22, v23, v24, v25, v26, v27, v28, v29,
    v30, v31,
}

impl_reg!(XReg, X, 0..32);
impl_reg!(FReg, F, 0..32);
impl_reg!(VReg, V, 0..32);

/// Any register, regardless of class.
///
/// Never appears inside an instruction -- instructions always name a particular
/// class of register -- but this is useful for testing and things like that.
#[expect(missing_docs, reason = "self-describing variants")]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum AnyReg {
    X(XReg),
    F(FReg),
    V(VReg),
}

impl fmt::Display for AnyReg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl fmt::Debug for AnyReg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AnyReg::X(r) => fmt::Debug::fmt(r, f),
            AnyReg::F(r) => fmt::Debug::fmt(r, f),
            AnyReg::V(r) => fmt::Debug::fmt(r, f),
        }
    }
}

/// Operands to a binary operation, packed into a 16-bit word (5 bits per register).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct BinaryOperands<D, S1 = D, S2 = D> {
    /// The destination register, packed in bits 0..5.
    pub dst: D,
    /// The first source register, packed in bits 5..10.
    pub src1: S1,
    /// The second source register, packed in bits 10..15.
    pub src2: S2,
}

impl<D, S1, S2> BinaryOperands<D, S1, S2> {
    /// Convenience constructor for applying `Into`
    pub fn new(dst: impl Into<D>, src1: impl Into<S1>, src2: impl Into<S2>) -> Self {
        Self {
            dst: dst.into(),
            src1: src1.into(),
            src2: src2.into(),
        }
    }
}

impl<D: Reg, S1: Reg, S2: Reg> BinaryOperands<D, S1, S2> {
    /// Convert to dense 16 bit encoding.
    pub fn to_bits(self) -> u16 {
        let dst = self.dst.to_u8();
        let src1 = self.src1.to_u8();
        let src2 = self.src2.to_u8();
        (dst as u16) | ((src1 as u16) << 5) | ((src2 as u16) << 10)
    }

    /// Convert from dense 16 bit encoding. The topmost bit is ignored.
    pub fn from_bits(bits: u16) -> Self {
        Self {
            dst: D::new((bits & 0b11111) as u8).unwrap(),
            src1: S1::new(((bits >> 5) & 0b11111) as u8).unwrap(),
            src2: S2::new(((bits >> 10) & 0b11111) as u8).unwrap(),
        }
    }
}

impl<D: Reg, S1: Reg> BinaryOperands<D, S1, U6> {
    /// Convert to dense 16 bit encoding.
    pub fn to_bits(self) -> u16 {
        let dst = self.dst.to_u8();
        let src1 = self.src1.to_u8();
        let src2 = u8::from(self.src2);
        (dst as u16) | ((src1 as u16) << 5) | ((src2 as u16) << 10)
    }

    /// Convert from dense 16 bit encoding. The topmost bit is ignored.
    pub fn from_bits(bits: u16) -> Self {
        Self {
            dst: D::new((bits & 0b11111) as u8).unwrap(),
            src1: S1::new(((bits >> 5) & 0b11111) as u8).unwrap(),
            src2: U6::new(((bits >> 10) & 0b111111) as u8).unwrap(),
        }
    }
}

/// A set of "upper half" registers, packed into a 16-bit bitset.
///
/// Registers stored in this bitset are offset by 16 and represent the upper
/// half of the 32 registers for each class.
pub struct UpperRegSet<R> {
    bitset: ScalarBitSet<u16>,
    phantom: PhantomData<R>,
}

impl<R: Reg> UpperRegSet<R> {
    /// Create a `RegSet` from a `ScalarBitSet`.
    pub fn from_bitset(bitset: ScalarBitSet<u16>) -> Self {
        Self {
            bitset,
            phantom: PhantomData,
        }
    }

    /// Convert a `UpperRegSet` into a `ScalarBitSet`.
    pub fn to_bitset(self) -> ScalarBitSet<u16> {
        self.bitset
    }
}

impl<R: Reg> From<ScalarBitSet<u16>> for UpperRegSet<R> {
    fn from(bitset: ScalarBitSet<u16>) -> Self {
        Self {
            bitset,
            phantom: PhantomData,
        }
    }
}

impl<R: Reg> From<UpperRegSet<R>> for ScalarBitSet<u16> {
    fn from(upper: UpperRegSet<R>) -> ScalarBitSet<u16> {
        upper.bitset
    }
}

impl<R: Reg> IntoIterator for UpperRegSet<R> {
    type Item = R;
    type IntoIter = UpperRegSetIntoIter<R>;

    fn into_iter(self) -> Self::IntoIter {
        UpperRegSetIntoIter {
            iter: self.bitset.into_iter(),
            _marker: PhantomData,
        }
    }
}

/// Returned iterator from `UpperRegSet::into_iter`
pub struct UpperRegSetIntoIter<R> {
    iter: cranelift_bitset::scalar::Iter<u16>,
    _marker: PhantomData<R>,
}

impl<R: Reg> Iterator for UpperRegSetIntoIter<R> {
    type Item = R;
    fn next(&mut self) -> Option<R> {
        Some(R::new(self.iter.next()? + 16).unwrap())
    }
}

impl<R: Reg> DoubleEndedIterator for UpperRegSetIntoIter<R> {
    fn next_back(&mut self) -> Option<R> {
        Some(R::new(self.iter.next_back()? + 16).unwrap())
    }
}

impl<R: Reg> Default for UpperRegSet<R> {
    fn default() -> Self {
        Self {
            bitset: Default::default(),
            phantom: Default::default(),
        }
    }
}

impl<R: Reg> Copy for UpperRegSet<R> {}
impl<R: Reg> Clone for UpperRegSet<R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<R: Reg> PartialEq for UpperRegSet<R> {
    fn eq(&self, other: &Self) -> bool {
        self.bitset == other.bitset
    }
}
impl<R: Reg> Eq for UpperRegSet<R> {}

impl<R: Reg> fmt::Debug for UpperRegSet<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(*self).finish()
    }
}

#[cfg(feature = "arbitrary")]
impl<'a, R: Reg> arbitrary::Arbitrary<'a> for UpperRegSet<R> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        ScalarBitSet::arbitrary(u).map(Self::from)
    }
}

/// Immediate used for the "o32" addresing mode.
///
/// This addressing mode represents a host address stored in `self.addr` which
/// is byte-offset by `self.offset`.
///
/// This addressing mode cannot generate a trap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct AddrO32 {
    /// The base address of memory.
    pub addr: XReg,
    /// A byte offset from `addr`.
    pub offset: i32,
}

/// Immediate used for the "z" addresing mode.
///
/// This addressing mode represents a host address stored in `self.addr` which
/// is byte-offset by `self.offset`.
///
/// If the `addr` specified is NULL then operating on this value will generate a
/// trap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct AddrZ {
    /// The base address of memory, or NULL.
    pub addr: XReg,
    /// A byte offset from `addr`.
    pub offset: i32,
}

/// Immediate used for the "g32" addressing mode.
///
/// This addressing mode represents the computation of a WebAssembly address for
/// a 32-bit linear memory. This automatically folds a bounds-check into the
/// address computation to generate a trap if the address is out-of-bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct AddrG32 {
    /// The register holding the base address of the linear memory that is being
    /// accessed.
    pub host_heap_base: XReg,

    /// The register holding the byte bound limit of the heap being accessed.
    pub host_heap_bound: XReg,

    /// The register holding a 32-bit WebAssembly address into linear memory.
    ///
    /// This is zero-extended on 64-bit platforms when performing the bounds
    /// check.
    pub wasm_addr: XReg,

    /// A static byte offset from `host_heap_base` that is added to `wasm_addr`
    /// when computing the bounds check.
    pub offset: u16,
}

impl AddrG32 {
    /// Decodes this immediate from a 32-bit integer.
    pub fn from_bits(bits: u32) -> AddrG32 {
        let host_heap_base = XReg::new(((bits >> 26) & 0b11111) as u8).unwrap();
        let bound_reg = XReg::new(((bits >> 21) & 0b11111) as u8).unwrap();
        let wasm_addr = XReg::new(((bits >> 16) & 0b11111) as u8).unwrap();
        AddrG32 {
            host_heap_base,
            host_heap_bound: bound_reg,
            wasm_addr,
            offset: bits as u16,
        }
    }

    /// Encodes this immediate into a 32-bit integer.
    pub fn to_bits(&self) -> u32 {
        u32::from(self.offset)
            | (u32::from(self.wasm_addr.to_u8()) << 16)
            | (u32::from(self.host_heap_bound.to_u8()) << 21)
            | (u32::from(self.host_heap_base.to_u8()) << 26)
    }
}

/// Similar structure to the [`AddrG32`] addressing mode but "g32bne" also
/// represents that the bound to linear memory is stored itself in memory.
///
/// This instruction will load the heap bound from memory and then perform the
/// same bounds check that [`AddrG32`] does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct AddrG32Bne {
    /// The register holding the base address of the linear memory that is being
    /// accessed.
    pub host_heap_base: XReg,

    /// The register holding the address of where the heap bound is located in
    /// host memory.
    pub host_heap_bound_addr: XReg,

    /// The static offset from `self.host_heap_bound_addr` that the bound is
    /// located at.
    pub host_heap_bound_offset: u8,

    /// The register holding a 32-bit WebAssembly address into linear memory.
    ///
    /// This is zero-extended on 64-bit platforms when performing the bounds
    /// check.
    pub wasm_addr: XReg,

    /// A static byte offset from `host_heap_base` that is added to `wasm_addr`
    /// when computing the bounds check.
    ///
    /// Note that this is an 8-bit immediate instead of a 16-bit immediate
    /// unlike [`AddrG32`]. That's just to pack this structure into a 32-bit
    /// value for now but otherwise should be reasonable to extend to a larger
    /// width in the future if necessary.
    pub offset: u8,
}

impl AddrG32Bne {
    /// Decodes [`AddrG32Bne`] from the 32-bit immediate provided.
    pub fn from_bits(bits: u32) -> AddrG32Bne {
        let host_heap_base = XReg::new(((bits >> 26) & 0b11111) as u8).unwrap();
        let bound_reg = XReg::new(((bits >> 21) & 0b11111) as u8).unwrap();
        let wasm_addr = XReg::new(((bits >> 16) & 0b11111) as u8).unwrap();
        AddrG32Bne {
            host_heap_base,
            host_heap_bound_addr: bound_reg,
            host_heap_bound_offset: (bits >> 8) as u8,
            wasm_addr,
            offset: bits as u8,
        }
    }

    /// Encodes this immediate into a 32-bit integer.
    pub fn to_bits(&self) -> u32 {
        u32::from(self.offset)
            | (u32::from(self.host_heap_bound_offset) << 8)
            | (u32::from(self.wasm_addr.to_u8()) << 16)
            | (u32::from(self.host_heap_bound_addr.to_u8()) << 21)
            | (u32::from(self.host_heap_base.to_u8()) << 26)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn special_x_regs() {
        assert!(XReg::sp.is_special());
        assert!(XReg::spilltmp0.is_special());
    }

    #[test]
    fn not_special_x_regs() {
        for i in 0..27 {
            assert!(!XReg::new(i).unwrap().is_special());
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)] // takes 30s+ in miri
    fn binary_operands() {
        let mut i = 0;
        for src2 in XReg::RANGE {
            for src1 in XReg::RANGE {
                for dst in XReg::RANGE {
                    let operands = BinaryOperands {
                        dst: XReg::new(dst).unwrap(),
                        src1: XReg::new(src1).unwrap(),
                        src2: XReg::new(src2).unwrap(),
                    };
                    assert_eq!(operands.to_bits(), i);
                    assert_eq!(BinaryOperands::<XReg>::from_bits(i), operands);
                    assert_eq!(BinaryOperands::<XReg>::from_bits(0x8000 | i), operands);
                    i += 1;
                }
            }
        }
    }
}
