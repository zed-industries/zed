//! Memory operands to instructions.

use crate::api::{AsReg, CodeSink, Constant, KnownOffset, Label, TrapCode};
use crate::gpr::{self, NonRspGpr, Size};
use crate::rex::{Disp, RexPrefix, encode_modrm, encode_sib};

/// x64 memory addressing modes.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(any(test, feature = "fuzz"), derive(arbitrary::Arbitrary))]
pub enum Amode<R: AsReg> {
    ImmReg {
        base: R,
        simm32: AmodeOffsetPlusKnownOffset,
        trap: Option<TrapCode>,
    },
    ImmRegRegShift {
        base: R,
        index: NonRspGpr<R>,
        scale: Scale,
        simm32: AmodeOffset,
        trap: Option<TrapCode>,
    },
    RipRelative {
        target: DeferredTarget,
    },
}

impl<R: AsReg> Amode<R> {
    /// Return the [`TrapCode`] associated with this [`Amode`], if any.
    pub fn trap_code(&self) -> Option<TrapCode> {
        match self {
            Amode::ImmReg { trap, .. } | Amode::ImmRegRegShift { trap, .. } => *trap,
            Amode::RipRelative { .. } => None,
        }
    }

    /// Return the [`RexPrefix`] for each variant of this [`Amode`].
    #[must_use]
    pub(crate) fn as_rex_prefix(&self, enc_reg: u8, has_w_bit: bool, uses_8bit: bool) -> RexPrefix {
        match self {
            Amode::ImmReg { base, .. } => {
                RexPrefix::mem_op(enc_reg, base.enc(), has_w_bit, uses_8bit)
            }
            Amode::ImmRegRegShift { base, index, .. } => {
                RexPrefix::three_op(enc_reg, index.enc(), base.enc(), has_w_bit, uses_8bit)
            }
            Amode::RipRelative { .. } => RexPrefix::two_op(enc_reg, 0, has_w_bit, uses_8bit),
        }
    }

    /// Emit the ModR/M, SIB, and displacement suffixes as needed for this
    /// `Amode`.
    pub(crate) fn encode_rex_suffixes(
        &self,
        sink: &mut impl CodeSink,
        enc_reg: u8,
        bytes_at_end: u8,
        evex_scaling: Option<i8>,
    ) {
        emit_modrm_sib_disp(sink, enc_reg, self, bytes_at_end, evex_scaling);
    }

    /// Return the registers for encoding the `b` and `x` bits (e.g., in a VEX
    /// prefix).
    ///
    /// During encoding, the `b` bit is set by the topmost bit (the fourth bit)
    /// of either the `reg` register or, if this is a memory address, the `base`
    /// register. The `x` bit is set by the `index` register, when used.
    pub(crate) fn encode_bx_regs(&self) -> (Option<u8>, Option<u8>) {
        match self {
            Amode::ImmReg { base, .. } => (Some(base.enc()), None),
            Amode::ImmRegRegShift { base, index, .. } => (Some(base.enc()), Some(index.enc())),
            Amode::RipRelative { .. } => (None, None),
        }
    }
}

/// A 32-bit immediate for address offsets.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AmodeOffset(i32);

impl AmodeOffset {
    pub const ZERO: AmodeOffset = AmodeOffset::new(0);

    #[must_use]
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn value(self) -> i32 {
        self.0
    }
}

impl From<i32> for AmodeOffset {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl std::fmt::LowerHex for AmodeOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // This rather complex implementation is necessary to match how
        // `capstone` pretty-prints memory immediates.
        if self.0 == 0 {
            return Ok(());
        }
        if self.0 < 0 {
            write!(f, "-")?;
        }
        if self.0 > 9 || self.0 < -9 {
            write!(f, "0x")?;
        }
        let abs = match self.0.checked_abs() {
            Some(i) => i,
            None => -2_147_483_648,
        };
        std::fmt::LowerHex::fmt(&abs, f)
    }
}

/// An [`AmodeOffset`] immediate with an optional known offset.
///
/// Cranelift does not know certain offsets until emission time. To accommodate
/// Cranelift, this structure stores an optional [`KnownOffset`]. The following
/// happens immediately before emission:
/// - the [`KnownOffset`] is looked up, mapping it to an offset value
/// - the [`Simm32`] value is added to the offset value
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AmodeOffsetPlusKnownOffset {
    pub simm32: AmodeOffset,
    pub offset: Option<KnownOffset>,
}

impl AmodeOffsetPlusKnownOffset {
    pub const ZERO: AmodeOffsetPlusKnownOffset = AmodeOffsetPlusKnownOffset {
        simm32: AmodeOffset::ZERO,
        offset: None,
    };

    /// # Panics
    ///
    /// Panics if the sum of the immediate and the known offset value overflows.
    #[must_use]
    pub fn value(&self, sink: &impl CodeSink) -> i32 {
        let known_offset = match self.offset {
            Some(offset) => sink.known_offset(offset),
            None => 0,
        };
        known_offset
            .checked_add(self.simm32.value())
            .expect("no wrapping")
    }
}

impl std::fmt::LowerHex for AmodeOffsetPlusKnownOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(offset) = self.offset {
            write!(f, "<offset:{offset}>+")?;
        }
        std::fmt::LowerHex::fmt(&self.simm32, f)
    }
}

/// For RIP-relative addressing, keep track of the [`CodeSink`]-specific target.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(any(test, feature = "fuzz"), derive(arbitrary::Arbitrary))]
pub enum DeferredTarget {
    Label(Label),
    Constant(Constant),
    None,
}

impl<R: AsReg> std::fmt::Display for Amode<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pointer_width = Size::Quadword;
        match self {
            Amode::ImmReg { simm32, base, .. } => {
                // Note: size is always 8; the address is 64 bits,
                // even if the addressed operand is smaller.
                let base = base.to_string(Some(pointer_width));
                write!(f, "{simm32:x}({base})")
            }
            Amode::ImmRegRegShift {
                simm32,
                base,
                index,
                scale,
                ..
            } => {
                let base = base.to_string(Some(pointer_width));
                let index = index.to_string(pointer_width);
                let shift = scale.shift();
                if shift > 1 {
                    write!(f, "{simm32:x}({base}, {index}, {shift})")
                } else {
                    write!(f, "{simm32:x}({base}, {index})")
                }
            }
            Amode::RipRelative { .. } => write!(f, "(%rip)"),
        }
    }
}

/// The scaling factor for the index register in certain [`Amode`]s.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(any(test, feature = "fuzz"), derive(arbitrary::Arbitrary))]
pub enum Scale {
    One,
    Two,
    Four,
    Eight,
}

impl Scale {
    /// Create a new [`Scale`] from its hardware encoding.
    ///
    /// # Panics
    ///
    /// Panics if `enc` is not a valid encoding for a scale (0-3).
    #[must_use]
    pub fn new(enc: u8) -> Self {
        match enc {
            0b00 => Scale::One,
            0b01 => Scale::Two,
            0b10 => Scale::Four,
            0b11 => Scale::Eight,
            _ => panic!("invalid scale encoding: {enc}"),
        }
    }

    /// Return the hardware encoding of this [`Scale`].
    fn enc(&self) -> u8 {
        match self {
            Scale::One => 0b00,
            Scale::Two => 0b01,
            Scale::Four => 0b10,
            Scale::Eight => 0b11,
        }
    }

    /// Return how much this [`Scale`] will shift the value in the index
    /// register of the SIB byte.
    ///
    /// This is useful for pretty-printing; when encoding, one usually needs
    /// [`Scale::enc`].
    fn shift(&self) -> u8 {
        1 << self.enc()
    }
}

/// A general-purpose register or memory operand.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(any(test, feature = "fuzz"), derive(arbitrary::Arbitrary))]
#[allow(
    clippy::module_name_repetitions,
    reason = "'GprMem' indicates this has GPR and memory variants"
)]
pub enum GprMem<R: AsReg, M: AsReg> {
    Gpr(R),
    Mem(Amode<M>),
}

impl<R: AsReg, M: AsReg> GprMem<R, M> {
    /// Pretty-print the operand.
    pub fn to_string(&self, size: Size) -> String {
        match self {
            GprMem::Gpr(gpr) => gpr.to_string(Some(size)),
            GprMem::Mem(amode) => amode.to_string(),
        }
    }

    /// Return the [`RexPrefix`] for each variant of this [`GprMem`].
    #[must_use]
    pub(crate) fn as_rex_prefix(&self, enc_reg: u8, has_w_bit: bool, uses_8bit: bool) -> RexPrefix {
        match self {
            GprMem::Gpr(rm) => RexPrefix::two_op(enc_reg, rm.enc(), has_w_bit, uses_8bit),
            GprMem::Mem(amode) => amode.as_rex_prefix(enc_reg, has_w_bit, uses_8bit),
        }
    }

    /// Emit the ModR/M, SIB, and displacement suffixes for this [`GprMem`].
    pub(crate) fn encode_rex_suffixes(
        &self,
        sink: &mut impl CodeSink,
        enc_reg: u8,
        bytes_at_end: u8,
        evex_scaling: Option<i8>,
    ) {
        match self {
            GprMem::Gpr(gpr) => {
                sink.put1(encode_modrm(0b11, enc_reg & 0b111, gpr.enc() & 0b111));
            }
            GprMem::Mem(amode) => {
                amode.encode_rex_suffixes(sink, enc_reg, bytes_at_end, evex_scaling);
            }
        }
    }

    /// Same as `XmmMem::encode_bx_regs`, but for `GprMem`.
    pub(crate) fn encode_bx_regs(&self) -> (Option<u8>, Option<u8>) {
        match self {
            GprMem::Gpr(reg) => (Some(reg.enc()), None),
            GprMem::Mem(amode) => amode.encode_bx_regs(),
        }
    }
}

impl<R: AsReg, M: AsReg> From<R> for GprMem<R, M> {
    fn from(reg: R) -> GprMem<R, M> {
        GprMem::Gpr(reg)
    }
}

impl<R: AsReg, M: AsReg> From<Amode<M>> for GprMem<R, M> {
    fn from(amode: Amode<M>) -> GprMem<R, M> {
        GprMem::Mem(amode)
    }
}

/// An XMM register or memory operand.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(any(test, feature = "fuzz"), derive(arbitrary::Arbitrary))]
#[allow(
    clippy::module_name_repetitions,
    reason = "'XmmMem' indicates this has Xmm and memory variants"
)]
pub enum XmmMem<R: AsReg, M: AsReg> {
    Xmm(R),
    Mem(Amode<M>),
}

impl<R: AsReg, M: AsReg> XmmMem<R, M> {
    /// Pretty-print the operand.
    pub fn to_string(&self) -> String {
        match self {
            XmmMem::Xmm(xmm) => xmm.to_string(None),
            XmmMem::Mem(amode) => amode.to_string(),
        }
    }

    /// Return the [`RexPrefix`] for each variant of this [`XmmMem`].
    #[must_use]
    pub(crate) fn as_rex_prefix(&self, enc_reg: u8, has_w_bit: bool, uses_8bit: bool) -> RexPrefix {
        match self {
            XmmMem::Xmm(rm) => RexPrefix::two_op(enc_reg, rm.enc(), has_w_bit, uses_8bit),
            XmmMem::Mem(amode) => amode.as_rex_prefix(enc_reg, has_w_bit, uses_8bit),
        }
    }

    /// Emit the ModR/M, SIB, and displacement suffixes for this [`XmmMem`].
    pub(crate) fn encode_rex_suffixes(
        &self,
        sink: &mut impl CodeSink,
        enc_reg: u8,
        bytes_at_end: u8,
        evex_scaling: Option<i8>,
    ) {
        match self {
            XmmMem::Xmm(xmm) => {
                sink.put1(encode_modrm(0b11, enc_reg & 0b111, xmm.enc() & 0b111));
            }
            XmmMem::Mem(amode) => {
                amode.encode_rex_suffixes(sink, enc_reg, bytes_at_end, evex_scaling);
            }
        }
    }

    /// Return the registers for encoding the `b` and `x` bits (e.g., in a VEX
    /// prefix).
    ///
    /// During encoding, the `b` bit is set by the topmost bit (the fourth bit)
    /// of either the `reg` register or, if this is a memory address, the `base`
    /// register. The `x` bit is set by the `index` register, when used.
    pub(crate) fn encode_bx_regs(&self) -> (Option<u8>, Option<u8>) {
        match self {
            XmmMem::Xmm(reg) => (Some(reg.enc()), None),
            XmmMem::Mem(amode) => amode.encode_bx_regs(),
        }
    }
}

impl<R: AsReg, M: AsReg> From<R> for XmmMem<R, M> {
    fn from(reg: R) -> XmmMem<R, M> {
        XmmMem::Xmm(reg)
    }
}

impl<R: AsReg, M: AsReg> From<Amode<M>> for XmmMem<R, M> {
    fn from(amode: Amode<M>) -> XmmMem<R, M> {
        XmmMem::Mem(amode)
    }
}

/// Emit the ModRM/SIB/displacement sequence for a memory operand.
pub fn emit_modrm_sib_disp<R: AsReg>(
    sink: &mut impl CodeSink,
    enc_g: u8,
    mem_e: &Amode<R>,
    bytes_at_end: u8,
    evex_scaling: Option<i8>,
) {
    match *mem_e {
        Amode::ImmReg { simm32, base, .. } => {
            let enc_e = base.enc();
            let mut imm = Disp::new(simm32.value(sink), evex_scaling);

            // Most base registers allow for a single ModRM byte plus an
            // optional immediate. If rsp is the base register, however, then a
            // SIB byte must be used.
            let enc_e_low3 = enc_e & 7;
            if enc_e_low3 == gpr::enc::RSP {
                // Displacement from RSP is encoded with a SIB byte where
                // the index and base are both encoded as RSP's encoding of
                // 0b100. This special encoding means that the index register
                // isn't used and the base is 0b100 with or without a
                // REX-encoded 4th bit (e.g. rsp or r12)
                sink.put1(encode_modrm(imm.m0d(), enc_g & 7, 0b100));
                sink.put1(0b00_100_100);
                imm.emit(sink);
            } else {
                // If the base register is rbp and there's no offset then force
                // a 1-byte zero offset since otherwise the encoding would be
                // invalid.
                if enc_e_low3 == gpr::enc::RBP {
                    imm.force_immediate();
                }
                sink.put1(encode_modrm(imm.m0d(), enc_g & 7, enc_e & 7));
                imm.emit(sink);
            }
        }

        Amode::ImmRegRegShift {
            simm32,
            base,
            index,
            scale,
            ..
        } => {
            let enc_base = base.enc();
            let enc_index = index.enc();

            // Encoding of ModRM/SIB bytes don't allow the index register to
            // ever be rsp. Note, though, that the encoding of r12, whose three
            // lower bits match the encoding of rsp, is explicitly allowed with
            // REX bytes so only rsp is disallowed.
            assert!(enc_index != gpr::enc::RSP);

            // If the offset is zero then there is no immediate. Note, though,
            // that if the base register's lower three bits are `101` then an
            // offset must be present. This is a special case in the encoding of
            // the SIB byte and requires an explicit displacement with rbp/r13.
            let mut imm = Disp::new(simm32.value(), evex_scaling);
            if enc_base & 7 == gpr::enc::RBP {
                imm.force_immediate();
            }

            // With the above determined encode the ModRM byte, then the SIB
            // byte, then any immediate as necessary.
            sink.put1(encode_modrm(imm.m0d(), enc_g & 7, 0b100));
            sink.put1(encode_sib(scale.enc(), enc_index & 7, enc_base & 7));
            imm.emit(sink);
        }

        Amode::RipRelative { target } => {
            // RIP-relative is mod=00, rm=101.
            sink.put1(encode_modrm(0b00, enc_g & 7, 0b101));

            // Inform the code sink about the RIP-relative `target` at the
            // current offset, emitting a `LabelUse`, a relocation, or etc as
            // appropriate.
            sink.use_target(target);

            // N.B.: some instructions (XmmRmRImm format for example)
            // have bytes *after* the RIP-relative offset. The
            // addressed location is relative to the end of the
            // instruction, but the relocation is nominally relative
            // to the end of the u32 field. So, to compensate for
            // this, we emit a negative extra offset in the u32 field
            // initially, and the relocation will add to it.
            sink.put4(-(i32::from(bytes_at_end)) as u32);
        }
    }
}
