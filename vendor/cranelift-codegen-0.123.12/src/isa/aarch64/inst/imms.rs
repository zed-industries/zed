//! AArch64 ISA definitions: immediate constants.

use crate::ir::types::*;
use crate::isa::aarch64::inst::{OperandSize, ScalarSize};
use crate::machinst::PrettyPrint;

use std::string::String;

/// An immediate that represents the NZCV flags.
#[derive(Clone, Copy, Debug)]
pub struct NZCV {
    /// The negative condition flag.
    n: bool,
    /// The zero condition flag.
    z: bool,
    /// The carry condition flag.
    c: bool,
    /// The overflow condition flag.
    v: bool,
}

impl NZCV {
    /// Create a new NZCV flags representation.
    pub fn new(n: bool, z: bool, c: bool, v: bool) -> NZCV {
        NZCV { n, z, c, v }
    }

    /// Bits for encoding.
    pub fn bits(&self) -> u32 {
        (u32::from(self.n) << 3)
            | (u32::from(self.z) << 2)
            | (u32::from(self.c) << 1)
            | u32::from(self.v)
    }
}

/// An unsigned 5-bit immediate.
#[derive(Clone, Copy, Debug)]
pub struct UImm5 {
    /// The value.
    value: u8,
}

impl UImm5 {
    /// Create an unsigned 5-bit immediate from u8.
    pub fn maybe_from_u8(value: u8) -> Option<UImm5> {
        if value < 32 {
            Some(UImm5 { value })
        } else {
            None
        }
    }

    /// Bits for encoding.
    pub fn bits(&self) -> u32 {
        u32::from(self.value)
    }
}

/// A signed, scaled 7-bit offset.
#[derive(Clone, Copy, Debug)]
pub struct SImm7Scaled {
    /// The value.
    pub value: i16,
    /// multiplied by the size of this type
    pub scale_ty: Type,
}

impl SImm7Scaled {
    /// Create a SImm7Scaled from a raw offset and the known scale type, if
    /// possible.
    pub fn maybe_from_i64(value: i64, scale_ty: Type) -> Option<SImm7Scaled> {
        assert!(scale_ty == I64 || scale_ty == I32 || scale_ty == F64 || scale_ty == I8X16);
        let scale = scale_ty.bytes();
        assert!(scale.is_power_of_two());
        let scale = i64::from(scale);
        let upper_limit = 63 * scale;
        let lower_limit = -(64 * scale);
        if value >= lower_limit && value <= upper_limit && (value & (scale - 1)) == 0 {
            Some(SImm7Scaled {
                value: i16::try_from(value).unwrap(),
                scale_ty,
            })
        } else {
            None
        }
    }

    /// Bits for encoding.
    pub fn bits(&self) -> u32 {
        let ty_bytes: i16 = self.scale_ty.bytes() as i16;
        let scaled: i16 = self.value / ty_bytes;
        assert!(scaled <= 63 && scaled >= -64);
        let scaled: i8 = scaled as i8;
        let encoded: u32 = scaled as u32;
        encoded & 0x7f
    }
}

/// Floating-point unit immediate left shift.
#[derive(Clone, Copy, Debug)]
pub struct FPULeftShiftImm {
    /// Shift amount.
    pub amount: u8,
    /// Lane size in bits.
    pub lane_size_in_bits: u8,
}

impl FPULeftShiftImm {
    /// Create a floating-point unit immediate left shift from u8.
    pub fn maybe_from_u8(amount: u8, lane_size_in_bits: u8) -> Option<Self> {
        debug_assert!(lane_size_in_bits == 32 || lane_size_in_bits == 64);
        if amount < lane_size_in_bits {
            Some(Self {
                amount,
                lane_size_in_bits,
            })
        } else {
            None
        }
    }

    /// Returns the encoding of the immediate.
    pub fn enc(&self) -> u32 {
        debug_assert!(self.lane_size_in_bits.is_power_of_two());
        debug_assert!(self.lane_size_in_bits > self.amount);
        // The encoding of the immediate follows the table below,
        // where xs encode the shift amount.
        //
        // | lane_size_in_bits | encoding |
        // +------------------------------+
        // | 8                 | 0001xxx  |
        // | 16                | 001xxxx  |
        // | 32                | 01xxxxx  |
        // | 64                | 1xxxxxx  |
        //
        // The highest one bit is represented by `lane_size_in_bits`. Since
        // `lane_size_in_bits` is a power of 2 and `amount` is less
        // than `lane_size_in_bits`, they can be ORed
        // together to produced the encoded value.
        u32::from(self.lane_size_in_bits | self.amount)
    }
}

/// Floating-point unit immediate right shift.
#[derive(Clone, Copy, Debug)]
pub struct FPURightShiftImm {
    /// Shift amount.
    pub amount: u8,
    /// Lane size in bits.
    pub lane_size_in_bits: u8,
}

impl FPURightShiftImm {
    /// Create a floating-point unit immediate right shift from u8.
    pub fn maybe_from_u8(amount: u8, lane_size_in_bits: u8) -> Option<Self> {
        debug_assert!(lane_size_in_bits == 32 || lane_size_in_bits == 64);
        if amount > 0 && amount <= lane_size_in_bits {
            Some(Self {
                amount,
                lane_size_in_bits,
            })
        } else {
            None
        }
    }

    /// Returns encoding of the immediate.
    pub fn enc(&self) -> u32 {
        debug_assert_ne!(0, self.amount);
        // The encoding of the immediate follows the table below,
        // where xs encodes the negated shift amount.
        //
        // | lane_size_in_bits | encoding |
        // +------------------------------+
        // | 8                 | 0001xxx  |
        // | 16                | 001xxxx  |
        // | 32                | 01xxxxx  |
        // | 64                | 1xxxxxx  |
        //
        // The shift amount is negated such that a shift amount
        // of 1 (in 64-bit) is encoded as 0b111111 and a shift
        // amount of 64 is encoded as 0b000000,
        // in the bottom 6 bits.
        u32::from((self.lane_size_in_bits * 2) - self.amount)
    }
}

/// a 9-bit signed offset.
#[derive(Clone, Copy, Debug)]
pub struct SImm9 {
    /// The value.
    pub value: i16,
}

impl SImm9 {
    /// Create a signed 9-bit offset from a full-range value, if possible.
    pub fn maybe_from_i64(value: i64) -> Option<SImm9> {
        if value >= -256 && value <= 255 {
            Some(SImm9 {
                value: value as i16,
            })
        } else {
            None
        }
    }

    /// Bits for encoding.
    pub fn bits(&self) -> u32 {
        (self.value as u32) & 0x1ff
    }

    /// Signed value of immediate.
    pub fn value(&self) -> i32 {
        self.value as i32
    }
}

/// An unsigned, scaled 12-bit offset.
#[derive(Clone, Copy, Debug)]
pub struct UImm12Scaled {
    /// The value.
    value: u16,
    /// multiplied by the size of this type
    scale_ty: Type,
}

impl UImm12Scaled {
    /// Create a UImm12Scaled from a raw offset and the known scale type, if
    /// possible.
    pub fn maybe_from_i64(value: i64, scale_ty: Type) -> Option<UImm12Scaled> {
        let scale = scale_ty.bytes();
        assert!(scale.is_power_of_two());
        let scale = scale as i64;
        let limit = 4095 * scale;
        if value >= 0 && value <= limit && (value & (scale - 1)) == 0 {
            Some(UImm12Scaled {
                value: value as u16,
                scale_ty,
            })
        } else {
            None
        }
    }

    /// Create a zero immediate of this format.
    pub fn zero(scale_ty: Type) -> UImm12Scaled {
        UImm12Scaled { value: 0, scale_ty }
    }

    /// Encoded bits.
    pub fn bits(&self) -> u32 {
        (self.value as u32 / self.scale_ty.bytes()) & 0xfff
    }

    /// Value after scaling.
    pub fn value(&self) -> u32 {
        self.value as u32
    }
}

/// A shifted immediate value in 'imm12' format: supports 12 bits, shifted
/// left by 0 or 12 places.
#[derive(Copy, Clone, Debug)]
pub struct Imm12 {
    /// The immediate bits.
    pub bits: u16,
    /// Whether the immediate bits are shifted left by 12 or not.
    pub shift12: bool,
}

impl Imm12 {
    /// Compute a Imm12 from raw bits, if possible.
    pub fn maybe_from_u64(val: u64) -> Option<Imm12> {
        if val & !0xfff == 0 {
            Some(Imm12 {
                bits: val as u16,
                shift12: false,
            })
        } else if val & !(0xfff << 12) == 0 {
            Some(Imm12 {
                bits: (val >> 12) as u16,
                shift12: true,
            })
        } else {
            None
        }
    }

    /// Bits for 2-bit "shift" field in e.g. AddI.
    pub fn shift_bits(&self) -> u32 {
        if self.shift12 { 0b01 } else { 0b00 }
    }

    /// Bits for 12-bit "imm" field in e.g. AddI.
    pub fn imm_bits(&self) -> u32 {
        self.bits as u32
    }

    /// Get the actual value that this immediate corresponds to.
    pub fn value(&self) -> u32 {
        let base = self.bits as u32;
        if self.shift12 { base << 12 } else { base }
    }
}

/// An immediate for logical instructions.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ImmLogic {
    /// The actual value.
    value: u64,
    /// `N` flag.
    pub n: bool,
    /// `S` field: element size and element bits.
    pub r: u8,
    /// `R` field: rotate amount.
    pub s: u8,
    /// Was this constructed for a 32-bit or 64-bit instruction?
    pub size: OperandSize,
}

impl ImmLogic {
    /// Compute an ImmLogic from raw bits, if possible.
    pub fn maybe_from_u64(value: u64, ty: Type) -> Option<ImmLogic> {
        // Note: This function is a port of VIXL's Assembler::IsImmLogical.

        if ty != I64 && ty != I32 {
            return None;
        }
        let operand_size = OperandSize::from_ty(ty);

        let original_value = value;

        let value = if ty == I32 {
            // To handle 32-bit logical immediates, the very easiest thing is to repeat
            // the input value twice to make a 64-bit word. The correct encoding of that
            // as a logical immediate will also be the correct encoding of the 32-bit
            // value.

            // Avoid making the assumption that the most-significant 32 bits are zero by
            // shifting the value left and duplicating it.
            let value = value << 32;
            value | value >> 32
        } else {
            value
        };

        // Logical immediates are encoded using parameters n, imm_s and imm_r using
        // the following table:
        //
        //    N   imms    immr    size        S             R
        //    1  ssssss  rrrrrr    64    UInt(ssssss)  UInt(rrrrrr)
        //    0  0sssss  xrrrrr    32    UInt(sssss)   UInt(rrrrr)
        //    0  10ssss  xxrrrr    16    UInt(ssss)    UInt(rrrr)
        //    0  110sss  xxxrrr     8    UInt(sss)     UInt(rrr)
        //    0  1110ss  xxxxrr     4    UInt(ss)      UInt(rr)
        //    0  11110s  xxxxxr     2    UInt(s)       UInt(r)
        // (s bits must not be all set)
        //
        // A pattern is constructed of size bits, where the least significant S+1 bits
        // are set. The pattern is rotated right by R, and repeated across a 32 or
        // 64-bit value, depending on destination register width.
        //
        // Put another way: the basic format of a logical immediate is a single
        // contiguous stretch of 1 bits, repeated across the whole word at intervals
        // given by a power of 2. To identify them quickly, we first locate the
        // lowest stretch of 1 bits, then the next 1 bit above that; that combination
        // is different for every logical immediate, so it gives us all the
        // information we need to identify the only logical immediate that our input
        // could be, and then we simply check if that's the value we actually have.
        //
        // (The rotation parameter does give the possibility of the stretch of 1 bits
        // going 'round the end' of the word. To deal with that, we observe that in
        // any situation where that happens the bitwise NOT of the value is also a
        // valid logical immediate. So we simply invert the input whenever its low bit
        // is set, and then we know that the rotated case can't arise.)
        let (value, inverted) = if value & 1 == 1 {
            (!value, true)
        } else {
            (value, false)
        };

        if value == 0 {
            return None;
        }

        // The basic analysis idea: imagine our input word looks like this.
        //
        //    0011111000111110001111100011111000111110001111100011111000111110
        //                                                          c  b    a
        //                                                          |<--d-->|
        //
        // We find the lowest set bit (as an actual power-of-2 value, not its index)
        // and call it a. Then we add a to our original number, which wipes out the
        // bottommost stretch of set bits and replaces it with a 1 carried into the
        // next zero bit. Then we look for the new lowest set bit, which is in
        // position b, and subtract it, so now our number is just like the original
        // but with the lowest stretch of set bits completely gone. Now we find the
        // lowest set bit again, which is position c in the diagram above. Then we'll
        // measure the distance d between bit positions a and c (using CLZ), and that
        // tells us that the only valid logical immediate that could possibly be equal
        // to this number is the one in which a stretch of bits running from a to just
        // below b is replicated every d bits.
        fn lowest_set_bit(value: u64) -> u64 {
            let bit = value.trailing_zeros();
            1u64.checked_shl(bit).unwrap_or(0)
        }
        let a = lowest_set_bit(value);
        assert_ne!(0, a);
        let value_plus_a = value.wrapping_add(a);
        let b = lowest_set_bit(value_plus_a);
        let value_plus_a_minus_b = value_plus_a - b;
        let c = lowest_set_bit(value_plus_a_minus_b);

        let (d, clz_a, out_n, mask) = if c != 0 {
            // The general case, in which there is more than one stretch of set bits.
            // Compute the repeat distance d, and set up a bitmask covering the basic
            // unit of repetition (i.e. a word with the bottom d bits set). Also, in all
            // of these cases the N bit of the output will be zero.
            let clz_a = a.leading_zeros();
            let clz_c = c.leading_zeros();
            let d = clz_a - clz_c;
            let mask = (1 << d) - 1;
            (d, clz_a, 0, mask)
        } else {
            (64, a.leading_zeros(), 1, u64::max_value())
        };

        // If the repeat period d is not a power of two, it can't be encoded.
        if !d.is_power_of_two() {
            return None;
        }

        if ((b.wrapping_sub(a)) & !mask) != 0 {
            // If the bit stretch (b - a) does not fit within the mask derived from the
            // repeat period, then fail.
            return None;
        }

        // The only possible option is b - a repeated every d bits. Now we're going to
        // actually construct the valid logical immediate derived from that
        // specification, and see if it equals our original input.
        //
        // To repeat a value every d bits, we multiply it by a number of the form
        // (1 + 2^d + 2^(2d) + ...), i.e. 0x0001000100010001 or similar. These can
        // be derived using a table lookup on CLZ(d).
        const MULTIPLIERS: [u64; 6] = [
            0x0000000000000001,
            0x0000000100000001,
            0x0001000100010001,
            0x0101010101010101,
            0x1111111111111111,
            0x5555555555555555,
        ];
        let multiplier = MULTIPLIERS[(u64::from(d).leading_zeros() - 57) as usize];
        let candidate = b.wrapping_sub(a) * multiplier;

        if value != candidate {
            // The candidate pattern doesn't match our input value, so fail.
            return None;
        }

        // We have a match! This is a valid logical immediate, so now we have to
        // construct the bits and pieces of the instruction encoding that generates
        // it.

        // Count the set bits in our basic stretch. The special case of clz(0) == -1
        // makes the answer come out right for stretches that reach the very top of
        // the word (e.g. numbers like 0xffffc00000000000).
        let clz_b = if b == 0 {
            u32::max_value() // -1
        } else {
            b.leading_zeros()
        };
        let s = clz_a.wrapping_sub(clz_b);

        // Decide how many bits to rotate right by, to put the low bit of that basic
        // stretch in position a.
        let (s, r) = if inverted {
            // If we inverted the input right at the start of this function, here's
            // where we compensate: the number of set bits becomes the number of clear
            // bits, and the rotation count is based on position b rather than position
            // a (since b is the location of the 'lowest' 1 bit after inversion).
            // Need wrapping for when clz_b is max_value() (for when b == 0).
            (d - s, clz_b.wrapping_add(1) & (d - 1))
        } else {
            (s, (clz_a + 1) & (d - 1))
        };

        // Now we're done, except for having to encode the S output in such a way that
        // it gives both the number of set bits and the length of the repeated
        // segment. The s field is encoded like this:
        //
        //     imms    size        S
        //    ssssss    64    UInt(ssssss)
        //    0sssss    32    UInt(sssss)
        //    10ssss    16    UInt(ssss)
        //    110sss     8    UInt(sss)
        //    1110ss     4    UInt(ss)
        //    11110s     2    UInt(s)
        //
        // So we 'or' (2 * -d) with our computed s to form imms.
        let s = ((d * 2).wrapping_neg() | (s - 1)) & 0x3f;
        debug_assert!(u8::try_from(r).is_ok());
        debug_assert!(u8::try_from(s).is_ok());
        Some(ImmLogic {
            value: original_value,
            n: out_n != 0,
            r: r as u8,
            s: s as u8,
            size: operand_size,
        })
    }

    /// Returns bits ready for encoding: (N:1, R:6, S:6)
    pub fn enc_bits(&self) -> u32 {
        ((self.n as u32) << 12) | ((self.r as u32) << 6) | (self.s as u32)
    }

    /// Returns the value that this immediate represents.
    pub fn value(&self) -> u64 {
        self.value
    }

    /// Return an immediate for the bitwise-inverted value.
    pub fn invert(&self) -> ImmLogic {
        // For every ImmLogical immediate, the inverse can also be encoded.
        Self::maybe_from_u64(!self.value, self.size.to_ty()).unwrap()
    }
}

/// An immediate for shift instructions.
#[derive(Copy, Clone, Debug)]
pub struct ImmShift {
    /// 6-bit shift amount.
    pub imm: u8,
}

impl ImmShift {
    /// Create an ImmShift from raw bits, if possible.
    pub fn maybe_from_u64(val: u64) -> Option<ImmShift> {
        if val < 64 {
            Some(ImmShift { imm: val as u8 })
        } else {
            None
        }
    }

    /// Get the immediate value.
    pub fn value(&self) -> u8 {
        self.imm
    }
}

/// A 16-bit immediate for a MOVZ instruction, with a {0,16,32,48}-bit shift.
#[derive(Clone, Copy, Debug)]
pub struct MoveWideConst {
    /// The value.
    pub bits: u16,
    /// Result is `bits` shifted 16*shift bits to the left.
    pub shift: u8,
}

impl MoveWideConst {
    /// Construct a MoveWideConst from an arbitrary 64-bit constant if possible.
    pub fn maybe_from_u64(value: u64) -> Option<MoveWideConst> {
        let mask0 = 0x0000_0000_0000_ffffu64;
        let mask1 = 0x0000_0000_ffff_0000u64;
        let mask2 = 0x0000_ffff_0000_0000u64;
        let mask3 = 0xffff_0000_0000_0000u64;

        if value == (value & mask0) {
            return Some(MoveWideConst {
                bits: (value & mask0) as u16,
                shift: 0,
            });
        }
        if value == (value & mask1) {
            return Some(MoveWideConst {
                bits: ((value >> 16) & mask0) as u16,
                shift: 1,
            });
        }
        if value == (value & mask2) {
            return Some(MoveWideConst {
                bits: ((value >> 32) & mask0) as u16,
                shift: 2,
            });
        }
        if value == (value & mask3) {
            return Some(MoveWideConst {
                bits: ((value >> 48) & mask0) as u16,
                shift: 3,
            });
        }
        None
    }

    /// Create a `MoveWideCosnt` from a given shift, if possible.
    pub fn maybe_with_shift(imm: u16, shift: u8) -> Option<MoveWideConst> {
        let shift_enc = shift / 16;
        if shift_enc > 3 {
            None
        } else {
            Some(MoveWideConst {
                bits: imm,
                shift: shift_enc,
            })
        }
    }

    /// Create a zero immediate of this format.
    pub fn zero() -> MoveWideConst {
        MoveWideConst { bits: 0, shift: 0 }
    }
}

/// Advanced SIMD modified immediate as used by MOVI/MVNI.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ASIMDMovModImm {
    imm: u8,
    shift: u8,
    is_64bit: bool,
    shift_ones: bool,
}

impl ASIMDMovModImm {
    /// Construct an ASIMDMovModImm from an arbitrary 64-bit constant, if possible.
    /// Note that the bits in `value` outside of the range specified by `size` are
    /// ignored; for example, in the case of `ScalarSize::Size8` all bits above the
    /// lowest 8 are ignored.
    pub fn maybe_from_u64(value: u64, size: ScalarSize) -> Option<ASIMDMovModImm> {
        match size {
            ScalarSize::Size8 => Some(ASIMDMovModImm {
                imm: value as u8,
                shift: 0,
                is_64bit: false,
                shift_ones: false,
            }),
            ScalarSize::Size16 => {
                let value = value as u16;

                if value >> 8 == 0 {
                    Some(ASIMDMovModImm {
                        imm: value as u8,
                        shift: 0,
                        is_64bit: false,
                        shift_ones: false,
                    })
                } else if value as u8 == 0 {
                    Some(ASIMDMovModImm {
                        imm: (value >> 8) as u8,
                        shift: 8,
                        is_64bit: false,
                        shift_ones: false,
                    })
                } else {
                    None
                }
            }
            ScalarSize::Size32 => {
                let value = value as u32;

                // Value is of the form 0x00MMFFFF.
                if value & 0xFF00FFFF == 0x0000FFFF {
                    let imm = (value >> 16) as u8;

                    Some(ASIMDMovModImm {
                        imm,
                        shift: 16,
                        is_64bit: false,
                        shift_ones: true,
                    })
                // Value is of the form 0x0000MMFF.
                } else if value & 0xFFFF00FF == 0x000000FF {
                    let imm = (value >> 8) as u8;

                    Some(ASIMDMovModImm {
                        imm,
                        shift: 8,
                        is_64bit: false,
                        shift_ones: true,
                    })
                } else {
                    // Of the 4 bytes, at most one is non-zero.
                    for shift in (0..32).step_by(8) {
                        if value & (0xFF << shift) == value {
                            return Some(ASIMDMovModImm {
                                imm: (value >> shift) as u8,
                                shift,
                                is_64bit: false,
                                shift_ones: false,
                            });
                        }
                    }

                    None
                }
            }
            ScalarSize::Size64 => {
                let mut imm = 0u8;

                // Check if all bytes are either 0 or 0xFF.
                for i in 0..8 {
                    let b = (value >> (i * 8)) as u8;

                    if b == 0 || b == 0xFF {
                        imm |= (b & 1) << i;
                    } else {
                        return None;
                    }
                }

                Some(ASIMDMovModImm {
                    imm,
                    shift: 0,
                    is_64bit: true,
                    shift_ones: false,
                })
            }
            _ => None,
        }
    }

    /// Create a zero immediate of this format.
    pub fn zero(size: ScalarSize) -> Self {
        ASIMDMovModImm {
            imm: 0,
            shift: 0,
            is_64bit: size == ScalarSize::Size64,
            shift_ones: false,
        }
    }

    /// Returns the value that this immediate represents.
    pub fn value(&self) -> (u8, u32, bool) {
        (self.imm, self.shift as u32, self.shift_ones)
    }
}

/// Advanced SIMD modified immediate as used by the vector variant of FMOV.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ASIMDFPModImm {
    imm: u8,
    size: ScalarSize,
}

impl ASIMDFPModImm {
    /// Construct an ASIMDFPModImm from an arbitrary 64-bit constant, if possible.
    pub fn maybe_from_u64(value: u64, size: ScalarSize) -> Option<ASIMDFPModImm> {
        // In all cases immediates are encoded as an 8-bit number 0b_abcdefgh;
        // let `D` be the inverse of the digit `d`.
        match size {
            ScalarSize::Size16 => {
                // In this case the representable immediates are 16-bit numbers of the form
                // 0b_aBbb_cdef_gh00_0000.
                let value = value as u16;
                let b0_5 = (value >> 6) & 0b111111;
                let b6 = (value >> 6) & (1 << 6);
                let b7 = (value >> 8) & (1 << 7);
                let imm = (b0_5 | b6 | b7) as u8;

                if value == Self::value16(imm) {
                    Some(ASIMDFPModImm { imm, size })
                } else {
                    None
                }
            }
            ScalarSize::Size32 => {
                // In this case the representable immediates are 32-bit numbers of the form
                // 0b_aBbb_bbbc_defg_h000 shifted to the left by 16.
                let value = value as u32;
                let b0_5 = (value >> 19) & 0b111111;
                let b6 = (value >> 19) & (1 << 6);
                let b7 = (value >> 24) & (1 << 7);
                let imm = (b0_5 | b6 | b7) as u8;

                if value == Self::value32(imm) {
                    Some(ASIMDFPModImm { imm, size })
                } else {
                    None
                }
            }
            ScalarSize::Size64 => {
                // In this case the representable immediates are 64-bit numbers of the form
                // 0b_aBbb_bbbb_bbcd_efgh shifted to the left by 48.
                let b0_5 = (value >> 48) & 0b111111;
                let b6 = (value >> 48) & (1 << 6);
                let b7 = (value >> 56) & (1 << 7);
                let imm = (b0_5 | b6 | b7) as u8;

                if value == Self::value64(imm) {
                    Some(ASIMDFPModImm { imm, size })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Returns bits ready for encoding.
    pub fn enc_bits(&self) -> u8 {
        self.imm
    }

    /// Returns the 16-bit value that corresponds to an 8-bit encoding.
    fn value16(imm: u8) -> u16 {
        let imm = imm as u16;
        let b0_5 = imm & 0b111111;
        let b6 = (imm >> 6) & 1;
        let b6_inv = b6 ^ 1;
        let b7 = (imm >> 7) & 1;

        b0_5 << 6 | (b6 * 0b11) << 12 | b6_inv << 14 | b7 << 15
    }

    /// Returns the 32-bit value that corresponds to an 8-bit encoding.
    fn value32(imm: u8) -> u32 {
        let imm = imm as u32;
        let b0_5 = imm & 0b111111;
        let b6 = (imm >> 6) & 1;
        let b6_inv = b6 ^ 1;
        let b7 = (imm >> 7) & 1;

        b0_5 << 19 | (b6 * 0b11111) << 25 | b6_inv << 30 | b7 << 31
    }

    /// Returns the 64-bit value that corresponds to an 8-bit encoding.
    fn value64(imm: u8) -> u64 {
        let imm = imm as u64;
        let b0_5 = imm & 0b111111;
        let b6 = (imm >> 6) & 1;
        let b6_inv = b6 ^ 1;
        let b7 = (imm >> 7) & 1;

        b0_5 << 48 | (b6 * 0b11111111) << 54 | b6_inv << 62 | b7 << 63
    }
}

impl PrettyPrint for NZCV {
    fn pretty_print(&self, _: u8) -> String {
        let fmt = |c: char, v| if v { c.to_ascii_uppercase() } else { c };
        format!(
            "#{}{}{}{}",
            fmt('n', self.n),
            fmt('z', self.z),
            fmt('c', self.c),
            fmt('v', self.v)
        )
    }
}

impl PrettyPrint for UImm5 {
    fn pretty_print(&self, _: u8) -> String {
        format!("#{}", self.value)
    }
}

impl PrettyPrint for Imm12 {
    fn pretty_print(&self, _: u8) -> String {
        let shift = if self.shift12 { 12 } else { 0 };
        let value = u32::from(self.bits) << shift;
        format!("#{value}")
    }
}

impl PrettyPrint for SImm7Scaled {
    fn pretty_print(&self, _: u8) -> String {
        format!("#{}", self.value)
    }
}

impl PrettyPrint for FPULeftShiftImm {
    fn pretty_print(&self, _: u8) -> String {
        format!("#{}", self.amount)
    }
}

impl PrettyPrint for FPURightShiftImm {
    fn pretty_print(&self, _: u8) -> String {
        format!("#{}", self.amount)
    }
}

impl PrettyPrint for SImm9 {
    fn pretty_print(&self, _: u8) -> String {
        format!("#{}", self.value)
    }
}

impl PrettyPrint for UImm12Scaled {
    fn pretty_print(&self, _: u8) -> String {
        format!("#{}", self.value)
    }
}

impl PrettyPrint for ImmLogic {
    fn pretty_print(&self, _: u8) -> String {
        format!("#{}", self.value())
    }
}

impl PrettyPrint for ImmShift {
    fn pretty_print(&self, _: u8) -> String {
        format!("#{}", self.imm)
    }
}

impl PrettyPrint for MoveWideConst {
    fn pretty_print(&self, _: u8) -> String {
        if self.shift == 0 {
            format!("#{}", self.bits)
        } else {
            format!("#{}, LSL #{}", self.bits, self.shift * 16)
        }
    }
}

impl PrettyPrint for ASIMDMovModImm {
    fn pretty_print(&self, _: u8) -> String {
        if self.is_64bit {
            debug_assert_eq!(self.shift, 0);

            let enc_imm = self.imm as i8;
            let mut imm = 0u64;

            for i in 0..8 {
                let b = (enc_imm >> i) & 1;

                imm |= (-b as u8 as u64) << (i * 8);
            }

            format!("#{imm}")
        } else if self.shift == 0 {
            format!("#{}", self.imm)
        } else {
            let shift_type = if self.shift_ones { "MSL" } else { "LSL" };
            format!("#{}, {} #{}", self.imm, shift_type, self.shift)
        }
    }
}

impl PrettyPrint for ASIMDFPModImm {
    fn pretty_print(&self, _: u8) -> String {
        match self.size {
            ScalarSize::Size16 => {
                // FIXME(#8312): Use `f16` once it is stable.
                // `value` will always be a normal number. Convert it to a `f32`.
                let value: u32 = Self::value16(self.imm).into();
                let sign = (value & 0x8000) << 16;
                // Adjust the exponent for the difference between the `f16` exponent bias and the
                // `f32` exponent bias.
                let exponent = ((value & 0x7c00) + ((127 - 15) << 10)) << 13;
                let significand = (value & 0x3ff) << 13;
                format!("#{}", f32::from_bits(sign | exponent | significand))
            }
            ScalarSize::Size32 => format!("#{}", f32::from_bits(Self::value32(self.imm))),
            ScalarSize::Size64 => format!("#{}", f64::from_bits(Self::value64(self.imm))),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn imm_logical_test() {
        assert_eq!(None, ImmLogic::maybe_from_u64(0, I64));
        assert_eq!(None, ImmLogic::maybe_from_u64(u64::max_value(), I64));

        assert_eq!(
            Some(ImmLogic {
                value: 1,
                n: true,
                r: 0,
                s: 0,
                size: OperandSize::Size64,
            }),
            ImmLogic::maybe_from_u64(1, I64)
        );

        assert_eq!(
            Some(ImmLogic {
                value: 2,
                n: true,
                r: 63,
                s: 0,
                size: OperandSize::Size64,
            }),
            ImmLogic::maybe_from_u64(2, I64)
        );

        assert_eq!(None, ImmLogic::maybe_from_u64(5, I64));

        assert_eq!(None, ImmLogic::maybe_from_u64(11, I64));

        assert_eq!(
            Some(ImmLogic {
                value: 248,
                n: true,
                r: 61,
                s: 4,
                size: OperandSize::Size64,
            }),
            ImmLogic::maybe_from_u64(248, I64)
        );

        assert_eq!(None, ImmLogic::maybe_from_u64(249, I64));

        assert_eq!(
            Some(ImmLogic {
                value: 1920,
                n: true,
                r: 57,
                s: 3,
                size: OperandSize::Size64,
            }),
            ImmLogic::maybe_from_u64(1920, I64)
        );

        assert_eq!(
            Some(ImmLogic {
                value: 0x7ffe,
                n: true,
                r: 63,
                s: 13,
                size: OperandSize::Size64,
            }),
            ImmLogic::maybe_from_u64(0x7ffe, I64)
        );

        assert_eq!(
            Some(ImmLogic {
                value: 0x30000,
                n: true,
                r: 48,
                s: 1,
                size: OperandSize::Size64,
            }),
            ImmLogic::maybe_from_u64(0x30000, I64)
        );

        assert_eq!(
            Some(ImmLogic {
                value: 0x100000,
                n: true,
                r: 44,
                s: 0,
                size: OperandSize::Size64,
            }),
            ImmLogic::maybe_from_u64(0x100000, I64)
        );

        assert_eq!(
            Some(ImmLogic {
                value: u64::max_value() - 1,
                n: true,
                r: 63,
                s: 62,
                size: OperandSize::Size64,
            }),
            ImmLogic::maybe_from_u64(u64::max_value() - 1, I64)
        );

        assert_eq!(
            Some(ImmLogic {
                value: 0xaaaaaaaaaaaaaaaa,
                n: false,
                r: 1,
                s: 60,
                size: OperandSize::Size64,
            }),
            ImmLogic::maybe_from_u64(0xaaaaaaaaaaaaaaaa, I64)
        );

        assert_eq!(
            Some(ImmLogic {
                value: 0x8181818181818181,
                n: false,
                r: 1,
                s: 49,
                size: OperandSize::Size64,
            }),
            ImmLogic::maybe_from_u64(0x8181818181818181, I64)
        );

        assert_eq!(
            Some(ImmLogic {
                value: 0xffc3ffc3ffc3ffc3,
                n: false,
                r: 10,
                s: 43,
                size: OperandSize::Size64,
            }),
            ImmLogic::maybe_from_u64(0xffc3ffc3ffc3ffc3, I64)
        );

        assert_eq!(
            Some(ImmLogic {
                value: 0x100000001,
                n: false,
                r: 0,
                s: 0,
                size: OperandSize::Size64,
            }),
            ImmLogic::maybe_from_u64(0x100000001, I64)
        );

        assert_eq!(
            Some(ImmLogic {
                value: 0x1111111111111111,
                n: false,
                r: 0,
                s: 56,
                size: OperandSize::Size64,
            }),
            ImmLogic::maybe_from_u64(0x1111111111111111, I64)
        );

        for n in 0..2 {
            let types = if n == 0 { vec![I64, I32] } else { vec![I64] };
            for s in 0..64 {
                for r in 0..64 {
                    let imm = get_logical_imm(n, s, r);
                    for &ty in &types {
                        match ImmLogic::maybe_from_u64(imm, ty) {
                            Some(ImmLogic { value, .. }) => {
                                assert_eq!(imm, value);
                                ImmLogic::maybe_from_u64(!value, ty).unwrap();
                            }
                            None => assert_eq!(0, imm),
                        };
                    }
                }
            }
        }
    }

    // Repeat a value that has `width` bits, across a 64-bit value.
    fn repeat(value: u64, width: u64) -> u64 {
        let mut result = value & ((1 << width) - 1);
        let mut i = width;
        while i < 64 {
            result |= result << i;
            i *= 2;
        }
        result
    }

    // Get the logical immediate, from the encoding N/R/S bits.
    fn get_logical_imm(n: u32, s: u32, r: u32) -> u64 {
        // An integer is constructed from the n, imm_s and imm_r bits according to
        // the following table:
        //
        //  N   imms    immr    size        S             R
        //  1  ssssss  rrrrrr    64    UInt(ssssss)  UInt(rrrrrr)
        //  0  0sssss  xrrrrr    32    UInt(sssss)   UInt(rrrrr)
        //  0  10ssss  xxrrrr    16    UInt(ssss)    UInt(rrrr)
        //  0  110sss  xxxrrr     8    UInt(sss)     UInt(rrr)
        //  0  1110ss  xxxxrr     4    UInt(ss)      UInt(rr)
        //  0  11110s  xxxxxr     2    UInt(s)       UInt(r)
        // (s bits must not be all set)
        //
        // A pattern is constructed of size bits, where the least significant S+1
        // bits are set. The pattern is rotated right by R, and repeated across a
        // 64-bit value.

        if n == 1 {
            if s == 0x3f {
                return 0;
            }
            let bits = (1u64 << (s + 1)) - 1;
            bits.rotate_right(r)
        } else {
            if (s >> 1) == 0x1f {
                return 0;
            }
            let mut width = 0x20;
            while width >= 0x2 {
                if (s & width) == 0 {
                    let mask = width - 1;
                    if (s & mask) == mask {
                        return 0;
                    }
                    let bits = (1u64 << ((s & mask) + 1)) - 1;
                    return repeat(bits.rotate_right(r & mask), width.into());
                }
                width >>= 1;
            }
            unreachable!();
        }
    }

    #[test]
    fn asimd_fp_mod_imm_test() {
        assert_eq!(None, ASIMDFPModImm::maybe_from_u64(0, ScalarSize::Size32));
        assert_eq!(
            None,
            ASIMDFPModImm::maybe_from_u64(0.013671875_f32.to_bits() as u64, ScalarSize::Size32)
        );
        assert_eq!(None, ASIMDFPModImm::maybe_from_u64(0, ScalarSize::Size64));
        assert_eq!(
            None,
            ASIMDFPModImm::maybe_from_u64(10000_f64.to_bits(), ScalarSize::Size64)
        );
    }

    #[test]
    fn asimd_mov_mod_imm_test() {
        assert_eq!(
            None,
            ASIMDMovModImm::maybe_from_u64(513, ScalarSize::Size16)
        );
        assert_eq!(
            None,
            ASIMDMovModImm::maybe_from_u64(4278190335, ScalarSize::Size32)
        );
        assert_eq!(
            None,
            ASIMDMovModImm::maybe_from_u64(8388608, ScalarSize::Size64)
        );

        assert_eq!(
            Some(ASIMDMovModImm {
                imm: 66,
                shift: 16,
                is_64bit: false,
                shift_ones: true,
            }),
            ASIMDMovModImm::maybe_from_u64(4390911, ScalarSize::Size32)
        );
    }
}
