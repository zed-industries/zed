//! S390x ISA definitions: immediate constants.

use crate::machinst::PrettyPrint;
use std::string::String;

/// An unsigned 12-bit immediate.
#[derive(Clone, Copy, Debug)]
pub struct UImm12 {
    /// The value.
    value: u16,
}

impl UImm12 {
    pub fn maybe_from_u64(value: u64) -> Option<UImm12> {
        if value < 4096 {
            Some(UImm12 {
                value: value as u16,
            })
        } else {
            None
        }
    }

    /// Create a zero immediate of this format.
    pub fn zero() -> UImm12 {
        UImm12 { value: 0 }
    }

    /// Bits for encoding.
    pub fn bits(&self) -> u32 {
        u32::from(self.value)
    }
}

/// A signed 20-bit immediate.
#[derive(Clone, Copy, Debug)]
pub struct SImm20 {
    /// The value.
    value: i32,
}

impl SImm20 {
    pub fn maybe_from_i64(value: i64) -> Option<SImm20> {
        if value >= -524288 && value < 524288 {
            Some(SImm20 {
                value: value as i32,
            })
        } else {
            None
        }
    }

    pub fn from_uimm12(value: UImm12) -> SImm20 {
        SImm20 {
            value: value.bits() as i32,
        }
    }

    /// Bits for encoding.
    pub fn bits(&self) -> u32 {
        let encoded: u32 = self.value as u32;
        encoded & 0xfffff
    }
}

/// A 16-bit immediate with a {0,16,32,48}-bit shift.
#[derive(Clone, Copy, Debug)]
pub struct UImm16Shifted {
    /// The value.
    pub bits: u16,
    /// Result is `bits` shifted 16*shift bits to the left.
    pub shift: u8,
}

impl UImm16Shifted {
    /// Construct a UImm16Shifted from an arbitrary 64-bit constant if possible.
    pub fn maybe_from_u64(value: u64) -> Option<UImm16Shifted> {
        let mask0 = 0x0000_0000_0000_ffffu64;
        let mask1 = 0x0000_0000_ffff_0000u64;
        let mask2 = 0x0000_ffff_0000_0000u64;
        let mask3 = 0xffff_0000_0000_0000u64;

        if value == (value & mask0) {
            return Some(UImm16Shifted {
                bits: (value & mask0) as u16,
                shift: 0,
            });
        }
        if value == (value & mask1) {
            return Some(UImm16Shifted {
                bits: ((value >> 16) & mask0) as u16,
                shift: 1,
            });
        }
        if value == (value & mask2) {
            return Some(UImm16Shifted {
                bits: ((value >> 32) & mask0) as u16,
                shift: 2,
            });
        }
        if value == (value & mask3) {
            return Some(UImm16Shifted {
                bits: ((value >> 48) & mask0) as u16,
                shift: 3,
            });
        }
        None
    }

    pub fn maybe_with_shift(imm: u16, shift: u8) -> Option<UImm16Shifted> {
        let shift_enc = shift / 16;
        if shift_enc > 3 {
            None
        } else {
            Some(UImm16Shifted {
                bits: imm,
                shift: shift_enc,
            })
        }
    }

    pub fn negate_bits(&self) -> UImm16Shifted {
        UImm16Shifted {
            bits: !self.bits,
            shift: self.shift,
        }
    }
}

/// A 32-bit immediate with a {0,32}-bit shift.
#[derive(Clone, Copy, Debug)]
pub struct UImm32Shifted {
    /// The value.
    pub bits: u32,
    /// Result is `bits` shifted 32*shift bits to the left.
    pub shift: u8,
}

impl UImm32Shifted {
    /// Construct a UImm32Shifted from an arbitrary 64-bit constant if possible.
    pub fn maybe_from_u64(value: u64) -> Option<UImm32Shifted> {
        let mask0 = 0x0000_0000_ffff_ffffu64;
        let mask1 = 0xffff_ffff_0000_0000u64;

        if value == (value & mask0) {
            return Some(UImm32Shifted {
                bits: (value & mask0) as u32,
                shift: 0,
            });
        }
        if value == (value & mask1) {
            return Some(UImm32Shifted {
                bits: ((value >> 32) & mask0) as u32,
                shift: 1,
            });
        }
        None
    }

    pub fn maybe_with_shift(imm: u32, shift: u8) -> Option<UImm32Shifted> {
        let shift_enc = shift / 32;
        if shift_enc > 3 {
            None
        } else {
            Some(UImm32Shifted {
                bits: imm,
                shift: shift_enc,
            })
        }
    }

    pub fn negate_bits(&self) -> UImm32Shifted {
        UImm32Shifted {
            bits: !self.bits,
            shift: self.shift,
        }
    }
}

impl PrettyPrint for UImm12 {
    fn pretty_print(&self, _: u8) -> String {
        format!("{}", self.value)
    }
}

impl PrettyPrint for SImm20 {
    fn pretty_print(&self, _: u8) -> String {
        format!("{}", self.value)
    }
}

impl PrettyPrint for UImm16Shifted {
    fn pretty_print(&self, _: u8) -> String {
        format!("{}", self.bits)
    }
}

impl PrettyPrint for UImm32Shifted {
    fn pretty_print(&self, _: u8) -> String {
        format!("{}", self.bits)
    }
}
