//! Immediate operands to instructions.

use crate::api::CodeSink;
use std::fmt;

/// This helper function prints the unsigned hexadecimal representation of the
/// immediate value: e.g., this prints `$0xfe` to represent both the signed `-2`
/// and the unsigned `254`.
macro_rules! hexify {
    ($n:expr) => {
        format!("$0x{:x}", $n)
    };
}

/// Like `hexify!`, but this performs a sign extension.
macro_rules! hexify_sign_extend {
    ($n:expr, $from:ty => $to:ty) => {{
        let from: $from = $n; // Assert the type we expect.
        let to = <$to>::from(from);
        format!("$0x{:x}", to)
    }};
}

/// An 8-bit immediate operand.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(any(test, feature = "fuzz"), derive(arbitrary::Arbitrary))]
pub struct Imm8(u8);

impl Imm8 {
    #[must_use]
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn encode(&self, sink: &mut impl CodeSink) {
        sink.put1(self.0);
    }
}

impl From<u8> for Imm8 {
    fn from(imm8: u8) -> Self {
        Self(imm8)
    }
}

impl TryFrom<i32> for Imm8 {
    type Error = std::num::TryFromIntError;
    fn try_from(simm32: i32) -> Result<Self, Self::Error> {
        Ok(Self(u8::try_from(simm32)?))
    }
}

impl fmt::Display for Imm8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "$0x{:x}", self.0)
    }
}

/// A _signed_ 8-bit immediate operand (suitable for sign extension).
#[derive(Clone, Copy, Debug)]
#[cfg_attr(any(test, feature = "fuzz"), derive(arbitrary::Arbitrary))]
pub struct Simm8(i8);

impl Simm8 {
    #[must_use]
    pub fn new(value: i8) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn value(&self) -> i8 {
        self.0
    }

    pub fn encode(&self, sink: &mut impl CodeSink) {
        sink.put1(self.0 as u8);
    }

    #[must_use]
    pub fn to_string(&self, extend: Extension) -> String {
        use Extension::{None, SignExtendLong, SignExtendQuad, SignExtendWord};
        match extend {
            None => hexify!(self.0),
            SignExtendWord => hexify_sign_extend!(self.0, i8 => i16),
            SignExtendLong => hexify_sign_extend!(self.0, i8 => i32),
            SignExtendQuad => hexify_sign_extend!(self.0, i8 => i64),
        }
    }
}

impl From<i8> for Simm8 {
    fn from(simm8: i8) -> Self {
        Self(simm8)
    }
}

impl TryFrom<i32> for Simm8 {
    type Error = std::num::TryFromIntError;
    fn try_from(simm32: i32) -> Result<Self, Self::Error> {
        Ok(Self(i8::try_from(simm32)?))
    }
}

/// A 16-bit immediate operand.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(any(test, feature = "fuzz"), derive(arbitrary::Arbitrary))]
pub struct Imm16(u16);

impl Imm16 {
    #[must_use]
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn value(&self) -> u16 {
        self.0
    }

    pub fn encode(&self, sink: &mut impl CodeSink) {
        sink.put2(self.0);
    }
}

impl From<u16> for Imm16 {
    fn from(imm16: u16) -> Self {
        Self(imm16)
    }
}

impl TryFrom<i32> for Imm16 {
    type Error = std::num::TryFromIntError;
    fn try_from(simm32: i32) -> Result<Self, Self::Error> {
        Ok(Self(u16::try_from(simm32)?))
    }
}

impl fmt::Display for Imm16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "$0x{:x}", self.0)
    }
}

/// A _signed_ 16-bit immediate operand (suitable for sign extension).
#[derive(Copy, Clone, Debug)]
#[cfg_attr(any(test, feature = "fuzz"), derive(arbitrary::Arbitrary))]
pub struct Simm16(i16);

impl Simm16 {
    #[must_use]
    pub fn new(value: i16) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn value(&self) -> i16 {
        self.0
    }

    pub fn encode(&self, sink: &mut impl CodeSink) {
        sink.put2(self.0 as u16);
    }

    #[must_use]
    pub fn to_string(&self, extend: Extension) -> String {
        use Extension::{None, SignExtendLong, SignExtendQuad, SignExtendWord};
        match extend {
            None => hexify!(self.0),
            SignExtendWord => unreachable!("the 16-bit value is already 16 bits"),
            SignExtendLong => hexify_sign_extend!(self.0, i16 => i32),
            SignExtendQuad => hexify_sign_extend!(self.0, i16 => i64),
        }
    }
}

impl From<i16> for Simm16 {
    fn from(simm16: i16) -> Self {
        Self(simm16)
    }
}

impl TryFrom<i32> for Simm16 {
    type Error = std::num::TryFromIntError;
    fn try_from(simm32: i32) -> Result<Self, Self::Error> {
        Ok(Self(i16::try_from(simm32)?))
    }
}

/// A 32-bit immediate operand.
///
/// Note that, "in 64-bit mode, the typical size of immediate operands remains
/// 32 bits. When the operand size is 64 bits, the processor sign-extends all
/// immediates to 64 bits prior to their use" (Intel SDM Vol. 2, 2.2.1.5).
#[derive(Copy, Clone, Debug)]
#[cfg_attr(any(test, feature = "fuzz"), derive(arbitrary::Arbitrary))]
pub struct Imm32(u32);

impl Imm32 {
    #[must_use]
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn encode(&self, sink: &mut impl CodeSink) {
        sink.put4(self.0);
    }
}

impl From<u32> for Imm32 {
    fn from(imm32: u32) -> Self {
        Self(imm32)
    }
}

impl From<i32> for Imm32 {
    fn from(simm32: i32) -> Self {
        // TODO: should this be a `TryFrom`?
        Self(simm32 as u32)
    }
}

impl fmt::Display for Imm32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "$0x{:x}", self.0)
    }
}

/// A _signed_ 32-bit immediate operand (suitable for sign extension).
///
/// Note that, "in 64-bit mode, the typical size of immediate operands remains
/// 32 bits. When the operand size is 64 bits, the processor sign-extends all
/// immediates to 64 bits prior to their use" (Intel SDM Vol. 2, 2.2.1.5).
#[derive(Copy, Clone, Debug)]
#[cfg_attr(any(test, feature = "fuzz"), derive(arbitrary::Arbitrary))]
pub struct Simm32(i32);

impl Simm32 {
    #[must_use]
    pub fn new(value: i32) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn value(&self) -> i32 {
        self.0
    }

    pub fn encode(&self, sink: &mut impl CodeSink) {
        sink.put4(self.0 as u32);
    }

    #[must_use]
    pub fn to_string(&self, extend: Extension) -> String {
        use Extension::{None, SignExtendLong, SignExtendQuad, SignExtendWord};
        match extend {
            None => hexify!(self.0),
            SignExtendWord => unreachable!("cannot sign extend a 32-bit value to 16 bits"),
            SignExtendLong => unreachable!("the 32-bit value is already 32 bits"),
            SignExtendQuad => hexify_sign_extend!(self.0, i32 => i64),
        }
    }
}

impl From<i32> for Simm32 {
    fn from(simm32: i32) -> Self {
        Self(simm32)
    }
}

/// A 64-bit immediate operand.
///
/// This form is quite rare; see certain `mov` instructions.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(any(test, feature = "fuzz"), derive(arbitrary::Arbitrary))]
pub struct Imm64(u64);

impl Imm64 {
    #[must_use]
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn value(&self) -> u64 {
        self.0
    }

    pub fn encode(&self, sink: &mut impl CodeSink) {
        sink.put8(self.0);
    }
}

impl From<u64> for Imm64 {
    fn from(imm64: u64) -> Self {
        Self(imm64)
    }
}

impl fmt::Display for Imm64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "$0x{:x}", self.0)
    }
}

/// Define the ways an immediate may be sign- or zero-extended.
#[derive(Clone, Copy, Debug)]
pub enum Extension {
    None,
    SignExtendQuad,
    SignExtendLong,
    SignExtendWord,
}
