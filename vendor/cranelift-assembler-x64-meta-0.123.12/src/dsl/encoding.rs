//! A DSL for describing x64 encodings.
//!
//! Intended use:
//! - construct an encoding using an abbreviated helper, e.g., [`rex`]
//! - then, configure the encoding using builder methods, e.g., [`Rex::w`]
//!
//! ```
//! # use cranelift_assembler_x64_meta::dsl::rex;
//! let enc = rex(0x25).w().id();
//! assert_eq!(enc.to_string(), "REX.W + 0x25 id")
//! ```
//!
//! This module references the Intel® 64 and IA-32 Architectures Software
//! Development Manual, Volume 2: [link].
//!
//! [link]: https://software.intel.com/content/www/us/en/develop/articles/intel-sdm.html

use super::{Operand, OperandKind};
use core::fmt;

/// An abbreviated constructor for REX-encoded instructions.
#[must_use]
pub fn rex(opcode: impl Into<Opcodes>) -> Rex {
    Rex {
        opcodes: opcode.into(),
        w: WBit::W0,
        modrm: None,
        imm: Imm::None,
        opcode_mod: None,
    }
}

/// An abbreviated constructor for VEX-encoded instructions.
#[must_use]
pub fn vex(length: Length) -> Vex {
    Vex {
        length,
        pp: None,
        mmmmm: None,
        w: WBit::WIG,
        opcode: u8::MAX,
        modrm: None,
        imm: Imm::None,
        is4: false,
    }
}

/// An abbreviated constructor for EVEX-encoded instructions.
#[must_use]
pub fn evex(length: Length, tuple_type: TupleType) -> Evex {
    Evex {
        length,
        pp: None,
        mmm: None,
        w: WBit::WIG,
        opcode: u8::MAX,
        modrm: None,
        imm: Imm::None,
        tuple_type,
    }
}

/// Enumerate the ways x64 encodes instructions.
pub enum Encoding {
    Rex(Rex),
    Vex(Vex),
    Evex(Evex),
}

impl Encoding {
    /// Check that the encoding is valid for the given operands; this can find
    /// issues earlier, before generating any Rust code.
    pub fn validate(&self, operands: &[Operand]) {
        match self {
            Encoding::Rex(rex) => rex.validate(operands),
            Encoding::Vex(vex) => vex.validate(operands),
            Encoding::Evex(evex) => evex.validate(operands),
        }
    }

    /// Return the opcode for this encoding.
    pub fn opcode(&self) -> u8 {
        match self {
            Encoding::Rex(rex) => rex.opcodes.opcode(),
            Encoding::Vex(vex) => vex.opcode,
            Encoding::Evex(evex) => evex.opcode,
        }
    }
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Encoding::Rex(rex) => write!(f, "{rex}"),
            Encoding::Vex(vex) => write!(f, "{vex}"),
            Encoding::Evex(evex) => write!(f, "{evex}"),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ModRmKind {
    /// Models `/digit`.
    ///
    /// From the reference manual: "a digit between 0 and 7 indicates that the
    /// ModR/M byte of the instruction uses only the r/m (register or memory)
    /// operand. The reg field contains the digit that provides an extension to
    /// the instruction's opcode."
    Digit(u8),

    /// Models `/r`.
    ///
    /// From the reference manual: "indicates that the ModR/M byte of the
    /// instruction contains a register operand and an r/m operand."
    Reg,
}

impl ModRmKind {
    /// Return the digit extending the opcode, if available.
    #[must_use]
    pub fn digit(&self) -> Option<u8> {
        match self {
            Self::Digit(digit) => Some(*digit),
            _ => None,
        }
    }

    /// Return the digit extending the opcode.
    ///
    /// # Panics
    ///
    /// Panics if not extension was defined.
    pub fn unwrap_digit(&self) -> u8 {
        self.digit().expect("expected an extension digit")
    }
}

impl fmt::Display for ModRmKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ModRmKind::Digit(digit) => write!(f, "/{digit}"),
            ModRmKind::Reg => write!(f, "/r"),
        }
    }
}

/// The traditional x64 encoding.
///
/// We use the "REX" name here in a slightly unorthodox way: "REX" is the name
/// for the optional _byte_ extending the number of available registers, e.g.,
/// but we use it here to distinguish this from other encoding formats (e.g.,
/// VEX, EVEX). The "REX" _byte_ is still optional in this encoding and only
/// emitted when necessary.
pub struct Rex {
    /// The opcodes for this instruction.
    ///
    /// Multi-byte opcodes are handled by passing an array of opcodes (including
    /// prefixes like `0x66` and escape bytes like `0x0f`) to the constructor.
    /// E.g., `66 0F 54` (`ANDPD`) is expressed as follows:
    ///
    /// ```
    /// # use cranelift_assembler_x64_meta::dsl::rex;
    /// let enc = rex([0x66, 0x0f, 0x54]);
    /// ```
    pub opcodes: Opcodes,
    /// Indicates setting the REX.W bit.
    ///
    /// From the reference manual: "Indicates the use of a REX prefix that
    /// affects operand size or instruction semantics. The ordering of the REX
    /// prefix and other optional/mandatory instruction prefixes are discussed
    /// in chapter 2. Note that REX prefixes that promote legacy instructions to
    /// 64-bit behavior are not listed explicitly in the opcode column."
    pub w: WBit,
    /// Indicates modifications to the ModR/M byte.
    pub modrm: Option<ModRmKind>,
    /// The number of bits used as an immediate operand to the instruction.
    pub imm: Imm,
    /// Used for `+rb`, `+rw`, `+rd`, and `+ro` instructions, which encode `reg`
    /// bits in the opcode byte; if `Some`, this contains the expected bit width
    /// of `reg`.
    ///
    /// From the reference manual: "[...] the lower 3 bits of the opcode byte is
    /// used to encode the register operand without a modR/M byte. The
    /// instruction lists the corresponding hexadecimal value of the opcode byte
    /// with low 3 bits as 000b. In non-64-bit mode, a register code, from 0
    /// through 7, is added to the hexadecimal value of the opcode byte. In
    /// 64-bit mode, indicates the four bit field of REX.b and opcode[2:0] field
    /// encodes the register operand of the instruction. “+ro” is applicable
    /// only in 64-bit mode."
    pub opcode_mod: Option<OpcodeMod>,
}

impl Rex {
    /// Set the `REX.W` bit.
    #[must_use]
    pub fn w(self) -> Self {
        Self {
            w: WBit::W1,
            ..self
        }
    }

    /// Set the ModR/M byte to contain a register operand and an r/m operand;
    /// equivalent to `/r` in the reference manual.
    #[must_use]
    pub fn r(self) -> Self {
        Self {
            modrm: Some(ModRmKind::Reg),
            ..self
        }
    }

    /// Set the digit extending the opcode; equivalent to `/<digit>` in the
    /// reference manual.
    ///
    /// # Panics
    ///
    /// Panics if `extension` is too large.
    #[must_use]
    pub fn digit(self, extension: u8) -> Self {
        assert!(extension <= 0b111, "must fit in 3 bits");
        Self {
            modrm: Some(ModRmKind::Digit(extension)),
            ..self
        }
    }

    /// Retrieve the digit extending the opcode, if available.
    #[must_use]
    pub fn unwrap_digit(&self) -> Option<u8> {
        match self.modrm {
            Some(ModRmKind::Digit(digit)) => Some(digit),
            _ => None,
        }
    }

    /// Append a byte-sized immediate operand (8-bit); equivalent to `ib` in the
    /// reference manual.
    ///
    /// # Panics
    ///
    /// Panics if an immediate operand is already set.
    #[must_use]
    pub fn ib(self) -> Self {
        assert_eq!(self.imm, Imm::None);
        Self {
            imm: Imm::ib,
            ..self
        }
    }

    /// Append a word-sized immediate operand (16-bit); equivalent to `iw` in
    /// the reference manual.
    ///
    /// # Panics
    ///
    /// Panics if an immediate operand is already set.
    #[must_use]
    pub fn iw(self) -> Self {
        assert_eq!(self.imm, Imm::None);
        Self {
            imm: Imm::iw,
            ..self
        }
    }

    /// Append a doubleword-sized immediate operand (32-bit); equivalent to `id`
    /// in the reference manual.
    ///
    /// # Panics
    ///
    /// Panics if an immediate operand is already set.
    #[must_use]
    pub fn id(self) -> Self {
        assert_eq!(self.imm, Imm::None);
        Self {
            imm: Imm::id,
            ..self
        }
    }

    /// Append a quadword-sized immediate operand (64-bit); equivalent to `io`
    /// in the reference manual.
    ///
    /// # Panics
    ///
    /// Panics if an immediate operand is already set.
    #[must_use]
    pub fn io(self) -> Self {
        assert_eq!(self.imm, Imm::None);
        Self {
            imm: Imm::io,
            ..self
        }
    }

    /// Modify the opcode byte with bits from an 8-bit `reg`; equivalent to
    /// `+rb` in the reference manual.
    #[must_use]
    pub fn rb(self) -> Self {
        Self {
            opcode_mod: Some(OpcodeMod::rb),
            ..self
        }
    }

    /// Modify the opcode byte with bits from a 16-bit `reg`; equivalent to
    /// `+rw` in the reference manual.
    #[must_use]
    pub fn rw(self) -> Self {
        Self {
            opcode_mod: Some(OpcodeMod::rw),
            ..self
        }
    }

    /// Modify the opcode byte with bits from a 32-bit `reg`; equivalent to
    /// `+rd` in the reference manual.
    #[must_use]
    pub fn rd(self) -> Self {
        Self {
            opcode_mod: Some(OpcodeMod::rd),
            ..self
        }
    }

    /// Modify the opcode byte with bits from a 64-bit `reg`; equivalent to
    /// `+ro` in the reference manual.
    #[must_use]
    pub fn ro(self) -> Self {
        Self {
            opcode_mod: Some(OpcodeMod::ro),
            ..self
        }
    }

    /// Check a subset of the rules for valid encodings outlined in chapter 2,
    /// _Instruction Format_, of the Intel® 64 and IA-32 Architectures Software
    /// Developer’s Manual, Volume 2A.
    fn validate(&self, operands: &[Operand]) {
        if let Some(OperandKind::Imm(op)) = operands
            .iter()
            .map(|o| o.location.kind())
            .find(|k| matches!(k, OperandKind::Imm(_)))
        {
            assert_eq!(
                op.bits(),
                self.imm.bits(),
                "for an immediate, the encoding width must match the declared operand width"
            );
        }

        if let Some(opcode_mod) = &self.opcode_mod {
            assert!(
                self.opcodes.primary & 0b111 == 0,
                "the lower three bits of the opcode byte should be 0"
            );
            assert!(
                operands
                    .iter()
                    .all(|o| o.location.bits() == opcode_mod.bits().into()),
                "the opcode modifier width must match the operand widths"
            );
        }

        assert!(!matches!(self.w, WBit::WIG));
    }
}

impl From<Rex> for Encoding {
    fn from(rex: Rex) -> Encoding {
        Encoding::Rex(rex)
    }
}

impl fmt::Display for Rex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(group1) = &self.opcodes.prefixes.group1 {
            write!(f, "{group1} + ")?;
        }
        if let Some(group2) = &self.opcodes.prefixes.group2 {
            write!(f, "{group2} + ")?;
        }
        if let Some(group3) = &self.opcodes.prefixes.group3 {
            write!(f, "{group3} + ")?;
        }
        if let Some(group4) = &self.opcodes.prefixes.group4 {
            write!(f, "{group4} + ")?;
        }
        if self.w.as_bool() {
            write!(f, "REX.W + ")?;
        }
        if self.opcodes.escape {
            write!(f, "0x0F + ")?;
        }
        write!(f, "{:#04X}", self.opcodes.primary)?;
        if let Some(secondary) = self.opcodes.secondary {
            write!(f, " {secondary:#04X}")?;
        }
        if let Some(modrm) = self.modrm {
            write!(f, " {modrm}")?;
        }
        if let Some(opcode_mod) = &self.opcode_mod {
            write!(f, " {opcode_mod}")?;
        }
        if self.imm != Imm::None {
            write!(f, " {}", self.imm)?;
        }
        Ok(())
    }
}

/// Describe an instruction's opcodes. From section 2.1.2 "Opcodes" in the
/// reference manual:
///
/// > A primary opcode can be 1, 2, or 3 bytes in length. An additional 3-bit
/// > opcode field is sometimes encoded in the ModR/M byte. Smaller fields can
/// > be defined within the primary opcode. Such fields define the direction of
/// > operation, size of displacements, register encoding, condition codes, or
/// > sign extension. Encoding fields used by an opcode vary depending on the
/// > class of operation.
/// >
/// > Two-byte opcode formats for general-purpose and SIMD instructions consist
/// > of one of the following:
/// > - An escape opcode byte `0FH` as the primary opcode and a second opcode
/// >   byte.
/// > - A mandatory prefix (`66H`, `F2H`, or `F3H`), an escape opcode byte, and
/// >   a second opcode byte (same as previous bullet).
/// >
/// > For example, `CVTDQ2PD` consists of the following sequence: `F3 0F E6`.
/// > The first byte is a mandatory prefix (it is not considered as a repeat
/// > prefix).
/// >
/// > Three-byte opcode formats for general-purpose and SIMD instructions
/// > consist of one of the following:
/// > - An escape opcode byte `0FH` as the primary opcode, plus two additional
/// >   opcode bytes.
/// > - A mandatory prefix (`66H`, `F2H`, or `F3H`), an escape opcode byte, plus
/// >   two additional opcode bytes (same as previous bullet).
/// >
/// > For example, `PHADDW` for XMM registers consists of the following
/// > sequence: `66 0F 38 01`. The first byte is the mandatory prefix.
pub struct Opcodes {
    /// The prefix bytes for this instruction.
    pub prefixes: Prefixes,
    /// Indicates the use of an escape opcode byte, `0x0f`.
    pub escape: bool,
    /// The primary opcode.
    pub primary: u8,
    /// Some instructions (e.g., SIMD) may have a secondary opcode.
    pub secondary: Option<u8>,
}

impl Opcodes {
    /// Return the main opcode for this instruction.
    ///
    /// Note that [`Rex`]-encoded instructions have a complex opcode scheme (see
    /// [`Opcodes`] documentation); the opcode one is usually looking for is the
    /// last one. This returns the last opcode: the secondary opcode if one is
    /// available and the primary otherwise.
    fn opcode(&self) -> u8 {
        if let Some(secondary) = self.secondary {
            secondary
        } else {
            self.primary
        }
    }
}

impl From<u8> for Opcodes {
    fn from(primary: u8) -> Opcodes {
        Opcodes {
            prefixes: Prefixes::default(),
            escape: false,
            primary,
            secondary: None,
        }
    }
}

impl<const N: usize> From<[u8; N]> for Opcodes {
    fn from(bytes: [u8; N]) -> Self {
        let (prefixes, remaining) = Prefixes::parse(&bytes);
        let (escape, primary, secondary) = match remaining {
            [primary] => (false, *primary, None),
            [0x0f, primary] => (true, *primary, None),
            [0x0f, primary, secondary] => (true, *primary, Some(*secondary)),
            _ => panic!(
                "invalid opcodes after prefix; expected [opcode], [0x0f, opcode], or [0x0f, opcode, opcode], found {remaining:x?}"
            ),
        };
        Self {
            prefixes,
            escape,
            primary,
            secondary,
        }
    }
}

/// The allowed prefixes for an instruction. From the reference manual (section
/// 2.1.1):
///
/// > Instruction prefixes are divided into four groups, each with a set of
/// > allowable prefix codes. For each instruction, it is only useful to include
/// > up to one prefix code from each of the four groups (Groups 1, 2, 3, 4).
/// > Groups 1 through 4 may be placed in any order relative to each other.
#[derive(Default)]
pub struct Prefixes {
    pub group1: Option<Group1Prefix>,
    pub group2: Option<Group2Prefix>,
    pub group3: Option<Group3Prefix>,
    pub group4: Option<Group4Prefix>,
}

impl Prefixes {
    /// Parse a slice of `bytes` into a set of prefixes, returning both the
    /// configured [`Prefixes`] as well as any remaining bytes.
    fn parse(mut bytes: &[u8]) -> (Self, &[u8]) {
        let mut prefixes = Self::default();
        while !bytes.is_empty() && prefixes.try_assign(bytes[0]).is_ok() {
            bytes = &bytes[1..];
        }
        (prefixes, bytes)
    }

    /// Attempt to parse a `byte` as a prefix and, if successful, assigns it to
    /// the correct prefix group.
    ///
    /// # Panics
    ///
    /// This function panics if the prefix for a group is already set; this
    /// disallows specifying multiple prefixes per group.
    fn try_assign(&mut self, byte: u8) -> Result<(), ()> {
        if let Ok(p) = Group1Prefix::try_from(byte) {
            assert!(self.group1.is_none());
            self.group1 = Some(p);
            Ok(())
        } else if let Ok(p) = Group2Prefix::try_from(byte) {
            assert!(self.group2.is_none());
            self.group2 = Some(p);
            Ok(())
        } else if let Ok(p) = Group3Prefix::try_from(byte) {
            assert!(self.group3.is_none());
            self.group3 = Some(p);
            Ok(())
        } else if let Ok(p) = Group4Prefix::try_from(byte) {
            assert!(self.group4.is_none());
            self.group4 = Some(p);
            Ok(())
        } else {
            Err(())
        }
    }

    /// Check if any prefix is present.
    pub fn is_empty(&self) -> bool {
        self.group1.is_none()
            && self.group2.is_none()
            && self.group3.is_none()
            && self.group4.is_none()
    }
}

pub enum Group1Prefix {
    /// The LOCK prefix (`0xf0`). From the reference manual:
    ///
    /// > The LOCK prefix (F0H) forces an operation that ensures exclusive use
    /// > of shared memory in a multiprocessor environment. See "LOCK—Assert
    /// > LOCK# Signal Prefix" in Chapter 3, Instruction Set Reference, A-L, for
    /// > a description of this prefix.
    Lock,
    /// A REPNE/REPNZ prefix (`0xf2`) or a BND prefix under certain conditions.
    /// `REP*` prefixes apply only to string and input/output instructions but
    /// can be used as mandatory prefixes in other kinds of instructions (e.g.,
    /// SIMD) From the reference manual:
    ///
    /// > Repeat prefixes (F2H, F3H) cause an instruction to be repeated for
    /// > each element of a string. Use these prefixes only with string and I/O
    /// > instructions (MOVS, CMPS, SCAS, LODS, STOS, INS, and OUTS). Use of
    /// > repeat prefixes and/or undefined opcodes with other Intel 64 or IA-32
    /// > instructions is reserved; such use may cause unpredictable behavior.
    /// >
    /// > Some instructions may use F2H, F3H as a mandatory prefix to express
    /// > distinct functionality.
    REPNorBND,
    /// A REPE/REPZ prefix (`0xf3`); `REP*` prefixes apply only to string and
    /// input/output instructions but can be used as mandatory prefixes in other
    /// kinds of instructions (e.g., SIMD). See `REPNorBND` for more details.
    REP_,
}

impl TryFrom<u8> for Group1Prefix {
    type Error = u8;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        Ok(match byte {
            0xF0 => Group1Prefix::Lock,
            0xF2 => Group1Prefix::REPNorBND,
            0xF3 => Group1Prefix::REP_,
            byte => return Err(byte),
        })
    }
}

impl fmt::Display for Group1Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Group1Prefix::Lock => write!(f, "0xF0"),
            Group1Prefix::REPNorBND => write!(f, "0xF2"),
            Group1Prefix::REP_ => write!(f, "0xF3"),
        }
    }
}

/// Contains the segment override prefixes or a (deprecated) branch hint when
/// used on a `Jcc` instruction. Note that using the segment override prefixes
/// on a branch instruction is reserved. See section 2.1.1, "Instruction
/// Prefixes," in the reference manual.
pub enum Group2Prefix {
    /// The CS segment override prefix (`0x2e`); also the "branch not taken"
    /// hint.
    CSorBNT,
    /// The SS segment override prefix (`0x36`).
    SS,
    /// The DS segment override prefix (`0x3e`); also the "branch taken" hint.
    DSorBT,
    /// The ES segment override prefix (`0x26`).
    ES,
    /// The FS segment override prefix (`0x64`).
    FS,
    /// The GS segment override prefix (`0x65`).
    GS,
}

impl TryFrom<u8> for Group2Prefix {
    type Error = u8;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        Ok(match byte {
            0x2E => Group2Prefix::CSorBNT,
            0x36 => Group2Prefix::SS,
            0x3E => Group2Prefix::DSorBT,
            0x26 => Group2Prefix::ES,
            0x64 => Group2Prefix::FS,
            0x65 => Group2Prefix::GS,
            byte => return Err(byte),
        })
    }
}

impl fmt::Display for Group2Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Group2Prefix::CSorBNT => write!(f, "0x2E"),
            Group2Prefix::SS => write!(f, "0x36"),
            Group2Prefix::DSorBT => write!(f, "0x3E"),
            Group2Prefix::ES => write!(f, "0x26"),
            Group2Prefix::FS => write!(f, "0x64"),
            Group2Prefix::GS => write!(f, "0x65"),
        }
    }
}

/// Contains the operand-size override prefix (`0x66`); also used as a SIMD
/// prefix. From the reference manual:
///
/// > The operand-size override prefix allows a program to switch between 16-
/// > and 32-bit operand sizes. Either size can be the default; use of the
/// > prefix selects the non-default size. Some SSE2/SSE3/SSSE3/SSE4
/// > instructions and instructions using a three-byte sequence of primary
/// > opcode bytes may use 66H as a mandatory prefix to express distinct
/// > functionality.
pub enum Group3Prefix {
    OperandSizeOverride,
}

impl TryFrom<u8> for Group3Prefix {
    type Error = u8;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        Ok(match byte {
            0x66 => Group3Prefix::OperandSizeOverride,
            byte => return Err(byte),
        })
    }
}

impl fmt::Display for Group3Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Group3Prefix::OperandSizeOverride => write!(f, "0x66"),
        }
    }
}

/// Contains the address-size override prefix (`0x67`). From the reference
/// manual:
///
/// > The address-size override prefix (67H) allows programs to switch between
/// > 16- and 32-bit addressing. Either size can be the default; the prefix
/// > selects the non-default size.
pub enum Group4Prefix {
    AddressSizeOverride,
}

impl TryFrom<u8> for Group4Prefix {
    type Error = u8;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        Ok(match byte {
            0x67 => Group4Prefix::AddressSizeOverride,
            byte => return Err(byte),
        })
    }
}

impl fmt::Display for Group4Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Group4Prefix::AddressSizeOverride => write!(f, "0x67"),
        }
    }
}

/// Indicate the size of an immediate operand. From the reference manual:
///
/// > A 1-byte (ib), 2-byte (iw), 4-byte (id) or 8-byte (io) immediate operand
/// > to the instruction that follows the opcode, ModR/M bytes or scale-indexing
/// > bytes. The opcode determines if the operand is a signed value. All words,
/// > doublewords, and quadwords are given with the low-order byte first.
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types, reason = "makes DSL definitions easier to read")]
pub enum Imm {
    None,
    ib,
    iw,
    id,
    io,
}

impl Imm {
    fn bits(&self) -> u16 {
        match self {
            Self::None => 0,
            Self::ib => 8,
            Self::iw => 16,
            Self::id => 32,
            Self::io => 64,
        }
    }
}

impl fmt::Display for Imm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::None => write!(f, ""),
            Self::ib => write!(f, "ib"),
            Self::iw => write!(f, "iw"),
            Self::id => write!(f, "id"),
            Self::io => write!(f, "io"),
        }
    }
}

/// Indicate the size of the `reg` used when modifying the lower three bits of
/// the opcode byte; this corresponds to the `+rb`, `+rw`, `+rd`, and `+ro`
/// modifiers in the reference manual.
///
/// ```
/// # use cranelift_assembler_x64_meta::dsl::{rex};
/// // The `bswap` instruction extends the opcode byte:
/// let enc = rex([0x0F, 0xC8]).rd();
/// assert_eq!(enc.to_string(), "0x0F + 0xC8 +rd");
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types, reason = "makes DSL definitions easier to read")]
pub enum OpcodeMod {
    rb,
    rw,
    rd,
    ro,
}

impl OpcodeMod {
    fn bits(&self) -> u8 {
        match self {
            Self::rb => 8,
            Self::rw => 16,
            Self::rd => 32,
            Self::ro => 64,
        }
    }
}

impl fmt::Display for OpcodeMod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::rb => write!(f, "+rb"),
            Self::rw => write!(f, "+rw"),
            Self::rd => write!(f, "+rd"),
            Self::ro => write!(f, "+ro"),
        }
    }
}

/// Contains the legacy prefixes allowed for VEX-encoded instructions.
///
/// VEX encodes a subset of [`Group1Prefix`] and `0x66` (see [`Group3Prefix`])
/// as part of the `pp` bit field.
#[derive(Clone, Copy, PartialEq)]
pub enum VexPrefix {
    _66,
    _F2,
    _F3,
}

impl VexPrefix {
    /// Encode the `pp` bits.
    #[inline(always)]
    pub(crate) fn bits(self) -> u8 {
        match self {
            Self::_66 => 0b01,
            Self::_F3 => 0b10,
            Self::_F2 => 0b11,
        }
    }
}

impl fmt::Display for VexPrefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::_66 => write!(f, "66"),
            Self::_F3 => write!(f, "F3"),
            Self::_F2 => write!(f, "F2"),
        }
    }
}

/// Contains the escape sequences allowed for VEX-encoded instructions.
///
/// VEX encodes these in the `mmmmmm` bit field.
#[derive(Clone, Copy, PartialEq)]
pub enum VexEscape {
    _0F,
    _0F3A,
    _0F38,
}

impl VexEscape {
    /// Encode the `m-mmmm` bits.
    #[inline(always)]
    pub(crate) fn bits(&self) -> u8 {
        match self {
            Self::_0F => 0b01,
            Self::_0F38 => 0b10,
            Self::_0F3A => 0b11,
        }
    }
}

impl fmt::Display for VexEscape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::_0F => write!(f, "0F"),
            Self::_0F3A => write!(f, "0F3A"),
            Self::_0F38 => write!(f, "0F38"),
        }
    }
}

/// Contains vector length definitions.
///
/// VEX encodes these in the `L` bit field, a single bit with `128-bit = 0` and
/// `256-bit = 1`. For convenience, we also include the `LIG` and `LZ` syntax,
/// used by the reference manual, and always set these to `0`.
///
/// EVEX encodes this in the `L'L` bits, two bits that typically indicate the
/// vector length for packed vector instructions but can also be used for
/// rounding control for floating-point instructions with rounding semantics
/// (see section 2.7.1 in the reference manual).
pub enum Length {
    /// 128-bit vector length.
    L128,
    /// 256-bit vector length.
    L256,
    /// 512-bit vector length; invalid for VEX instructions.
    L512,
    /// Force the length bits to `0`, but not necessarily for 128-bit operation.
    /// From the reference manual: "The VEX.L must be encoded to be 0B, an #UD
    /// occurs if VEX.L is not zero."
    LZ,
    /// The length bits are ignored (e.g., for floating point scalar
    /// instructions). This assembler will emit `0`.
    LIG,
}

impl Length {
    /// Encode the `VEX.L` bit.
    pub fn vex_bits(&self) -> u8 {
        match self {
            Self::L128 | Self::LIG | Self::LZ => 0b0,
            Self::L256 => 0b1,
            Self::L512 => unreachable!("VEX does not support 512-bit vector length"),
        }
    }

    /// Encode the `EVEX.L'L` bits.
    ///
    /// See section 2.7.10, Vector Length Orthogonality, in the reference manual
    pub fn evex_bits(&self) -> u8 {
        match self {
            Self::L128 | Self::LIG | Self::LZ => 0b00,
            Self::L256 => 0b01,
            Self::L512 => 0b10,
        }
    }
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::L128 => write!(f, "128"),
            Self::L256 => write!(f, "256"),
            Self::L512 => write!(f, "512"),
            Self::LIG => write!(f, "LIG"),
            Self::LZ => write!(f, "LZ"),
        }
    }
}

/// Model the `W` bit.
pub enum WBit {
    /// The `W` bit is ignored; equivalent to `.WIG` in the manual.
    WIG,
    /// The `W` bit is set to `0`; equivalent to `.W0` in the manual.
    W0,
    /// The `W` bit is set to `1`; equivalent to `.W1` in the manual.
    W1,
}

impl WBit {
    /// Return `true` if the `W` bit is ignored; this is useful to check in the
    /// DSL for the default case.
    fn is_ignored(&self) -> bool {
        match self {
            Self::WIG => true,
            Self::W0 | Self::W1 => false,
        }
    }

    /// Return `true` if the `W` bit is set (`W1`); otherwise, return `false`.
    pub(crate) fn as_bool(&self) -> bool {
        match self {
            Self::W1 => true,
            Self::W0 | Self::WIG => false,
        }
    }
}

impl fmt::Display for WBit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::WIG => write!(f, "WIG"),
            Self::W0 => write!(f, "W0"),
            Self::W1 => write!(f, "W1"),
        }
    }
}

/// The VEX encoding, introduced for AVX instructions.
///
/// ```
/// # use cranelift_assembler_x64_meta::dsl::{vex, Length::L128};
/// // To encode a BLENDPD instruction in the manual: VEX.128.66.0F3A.WIG 0D /r ib
/// let enc = vex(L128)._66()._0f3a().wig().op(0x0D).r().ib();
/// assert_eq!(enc.to_string(), "VEX.128.66.0F3A.WIG 0x0D /r ib");
/// ```
pub struct Vex {
    /// The length of the operand (e.g., 128-bit or 256-bit).
    pub length: Length,
    /// Any SIMD prefixes, but encoded in the `VEX.pp` bit field.
    pub pp: Option<VexPrefix>,
    /// Any leading map bytes, but encoded in the `VEX.mmmmm` bit field.
    pub mmmmm: Option<VexEscape>,
    /// The `W` bit.
    pub w: WBit,
    /// VEX-encoded instructions have a single-byte opcode. Other prefix-related
    /// bytes (see [`Opcodes`]) are encoded in the VEX prefixes (see `pp`,
    /// `mmmmmm`). From the reference manual: "One (and only one) opcode byte
    /// follows the 2 or 3 byte VEX."
    pub opcode: u8,
    /// See [`Rex.modrm`](Rex.modrm).
    pub modrm: Option<ModRmKind>,
    /// See [`Rex.imm`](Rex.imm).
    pub imm: Imm,
    /// See [`Vex::is4`]
    pub is4: bool,
}

impl Vex {
    /// Set the `pp` field to use [`VexPrefix::_66`]; equivalent to `.66` in the
    /// manual.
    pub fn _66(self) -> Self {
        assert!(self.pp.is_none());
        Self {
            pp: Some(VexPrefix::_66),
            ..self
        }
    }

    /// Set the `pp` field to use [`VexPrefix::_F2`]; equivalent to `.F2` in the
    /// manual.
    pub fn _f2(self) -> Self {
        assert!(self.pp.is_none());
        Self {
            pp: Some(VexPrefix::_F2),
            ..self
        }
    }

    /// Set the `pp` field to use [`VexPrefix::_F3`]; equivalent to `.F3` in the
    /// manual.
    pub fn _f3(self) -> Self {
        assert!(self.pp.is_none());
        Self {
            pp: Some(VexPrefix::_F3),
            ..self
        }
    }

    /// Set the `mmmmmm` field to use [`VexEscape::_0F`]; equivalent to `.0F` in
    /// the manual.
    pub fn _0f(self) -> Self {
        assert!(self.mmmmm.is_none());
        Self {
            mmmmm: Some(VexEscape::_0F),
            ..self
        }
    }

    /// Set the `mmmmmm` field to use [`VexEscape::_0F3A`]; equivalent to
    /// `.0F3A` in the manual.
    pub fn _0f3a(self) -> Self {
        assert!(self.mmmmm.is_none());
        Self {
            mmmmm: Some(VexEscape::_0F3A),
            ..self
        }
    }

    /// Set the `mmmmmm` field to use [`VexEscape::_0F38`]; equivalent to
    /// `.0F38` in the manual.
    pub fn _0f38(self) -> Self {
        assert!(self.mmmmm.is_none());
        Self {
            mmmmm: Some(VexEscape::_0F38),
            ..self
        }
    }

    /// Set the `W` bit to `0`; equivalent to `.W0` in the manual.
    pub fn w0(self) -> Self {
        assert!(self.w.is_ignored());
        Self {
            w: WBit::W0,
            ..self
        }
    }

    /// Set the `W` bit to `1`; equivalent to `.W1` in the manual.
    pub fn w1(self) -> Self {
        assert!(self.w.is_ignored());
        Self {
            w: WBit::W1,
            ..self
        }
    }

    /// Ignore the `W` bit; equivalent to `.WIG` in the manual.
    pub fn wig(self) -> Self {
        assert!(self.w.is_ignored());
        Self {
            w: WBit::WIG,
            ..self
        }
    }

    /// Set the single opcode for this VEX-encoded instruction.
    pub fn op(self, opcode: u8) -> Self {
        assert_eq!(self.opcode, u8::MAX);
        Self { opcode, ..self }
    }

    /// Set the ModR/M byte to contain a register operand; see [`Rex::r`].
    pub fn r(self) -> Self {
        assert!(self.modrm.is_none());
        Self {
            modrm: Some(ModRmKind::Reg),
            ..self
        }
    }

    /// Append a byte-sized immediate operand (8-bit); equivalent to `ib` in the
    /// reference manual.
    ///
    /// # Panics
    ///
    /// Panics if an immediate operand is already set.
    #[must_use]
    pub fn ib(self) -> Self {
        assert_eq!(self.imm, Imm::None);
        Self {
            imm: Imm::ib,
            ..self
        }
    }

    /// Append a word-sized immediate operand (16-bit); equivalent to `iw` in
    /// the reference manual.
    ///
    /// # Panics
    ///
    /// Panics if an immediate operand is already set.
    #[must_use]
    pub fn iw(self) -> Self {
        assert_eq!(self.imm, Imm::None);
        Self {
            imm: Imm::iw,
            ..self
        }
    }

    /// Append a doubleword-sized immediate operand (32-bit); equivalent to `id`
    /// in the reference manual.
    ///
    /// # Panics
    ///
    /// Panics if an immediate operand is already set.
    #[must_use]
    pub fn id(self) -> Self {
        assert_eq!(self.imm, Imm::None);
        Self {
            imm: Imm::id,
            ..self
        }
    }

    /// Append a quadword-sized immediate operand (64-bit); equivalent to `io`
    /// in the reference manual.
    ///
    /// # Panics
    ///
    /// Panics if an immediate operand is already set.
    #[must_use]
    pub fn io(self) -> Self {
        assert_eq!(self.imm, Imm::None);
        Self {
            imm: Imm::io,
            ..self
        }
    }

    /// Set the digit extending the opcode; equivalent to `/<digit>` in the
    /// reference manual.
    ///
    /// # Panics
    ///
    /// Panics if `extension` is too large.
    #[must_use]
    pub fn digit(self, extension: u8) -> Self {
        assert!(extension <= 0b111, "must fit in 3 bits");
        Self {
            modrm: Some(ModRmKind::Digit(extension)),
            ..self
        }
    }

    /// An 8-bit immediate byte is present containing a source register
    /// specifier in either imm8[7:4] (for 64-bit
    /// mode) or imm8[6:4] (for 32-bit mode), and instruction-specific payload
    /// in imm8[3:0].
    pub fn is4(self) -> Self {
        Self { is4: true, ..self }
    }

    fn validate(&self, _operands: &[Operand]) {
        assert!(self.opcode != u8::MAX);
        assert!(self.mmmmm.is_some());
        assert!(!matches!(self.length, Length::L512));
    }

    /// Retrieve the digit extending the opcode, if available.
    #[must_use]
    pub fn unwrap_digit(&self) -> Option<u8> {
        match self.modrm {
            Some(ModRmKind::Digit(digit)) => Some(digit),
            _ => None,
        }
    }
}

impl From<Vex> for Encoding {
    fn from(vex: Vex) -> Encoding {
        Encoding::Vex(vex)
    }
}

impl fmt::Display for Vex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VEX.{}", self.length)?;
        if let Some(pp) = self.pp {
            write!(f, ".{pp}")?;
        }
        if let Some(mmmmm) = self.mmmmm {
            write!(f, ".{mmmmm}")?;
        }
        write!(f, ".{} {:#04X}", self.w, self.opcode)?;
        if let Some(modrm) = self.modrm {
            write!(f, " {modrm}")?;
        }
        if self.imm != Imm::None {
            write!(f, " {}", self.imm)?;
        }
        Ok(())
    }
}

pub struct Evex {
    /// The vector length of the operand (e.g., 128-bit, 256-bit, or 512-bit).
    pub length: Length,
    /// Any SIMD prefixes, but encoded in the `EVEX.pp` bit field (see similar:
    /// [`Vex::pp`]).
    pub pp: Option<VexPrefix>,
    /// The `mmm` bits.
    ///
    /// Bits `1:0` are identical to the lowest 2 bits of `VEX.mmmmm`; EVEX adds
    /// one more bit here. From the reference manual: "provides access to up to
    /// eight decoding maps. Currently, only the following decoding maps are
    /// supported: 1, 2, 3, 5, and 6. Map ids 1, 2, and 3 are denoted by 0F,
    /// 0F38, and 0F3A, respectively, in the instruction encoding descriptions."
    pub mmm: Option<VexEscape>,
    /// The `W` bit.
    pub w: WBit,
    /// EVEX-encoded instructions opcode byte"
    pub opcode: u8,
    /// See [`Rex.modrm`](Rex.modrm).
    pub modrm: Option<ModRmKind>,
    /// See [`Rex.imm`](Rex.imm).
    pub imm: Imm,
    /// The "Tuple Type" corresponding to scaling of the 8-bit displacement
    /// parameter for memory operands. See [`TupleType`] for more information.
    pub tuple_type: TupleType,
}

impl Evex {
    /// Set the `pp` field to use [`VexPrefix::_66`]; equivalent to `.66` in the
    /// manual.
    pub fn _66(self) -> Self {
        assert!(self.pp.is_none());
        Self {
            pp: Some(VexPrefix::_66),
            ..self
        }
    }

    /// Set the `pp` field to use [`VexPrefix::_F2`]; equivalent to `.F2` in the
    /// manual.
    pub fn _f2(self) -> Self {
        assert!(self.pp.is_none());
        Self {
            pp: Some(VexPrefix::_F2),
            ..self
        }
    }

    /// Set the `pp` field to use [`VexPrefix::_F3`]; equivalent to `.F3` in the
    /// manual.
    pub fn _f3(self) -> Self {
        assert!(self.pp.is_none());
        Self {
            pp: Some(VexPrefix::_F3),
            ..self
        }
    }

    /// Set the `mmmmmm` field to use [`VexEscape::_0F`]; equivalent to `.0F` in
    /// the manual.
    pub fn _0f(self) -> Self {
        assert!(self.mmm.is_none());
        Self {
            mmm: Some(VexEscape::_0F),
            ..self
        }
    }

    /// Set the `mmmmmm` field to use [`VexEscape::_0F3A`]; equivalent to
    /// `.0F3A` in the manual.
    pub fn _0f3a(self) -> Self {
        assert!(self.mmm.is_none());
        Self {
            mmm: Some(VexEscape::_0F3A),
            ..self
        }
    }

    /// Set the `mmmmmm` field to use [`VexEscape::_0F38`]; equivalent to
    /// `.0F38` in the manual.
    pub fn _0f38(self) -> Self {
        assert!(self.mmm.is_none());
        Self {
            mmm: Some(VexEscape::_0F38),
            ..self
        }
    }

    /// Set the `W` bit to `0`; equivalent to `.W0` in the manual.
    pub fn w0(self) -> Self {
        assert!(self.w.is_ignored());
        Self {
            w: WBit::W0,
            ..self
        }
    }

    /// Set the `W` bit to `1`; equivalent to `.W1` in the manual.
    pub fn w1(self) -> Self {
        assert!(self.w.is_ignored());
        Self {
            w: WBit::W1,
            ..self
        }
    }

    /// Ignore the `W` bit; equivalent to `.WIG` in the manual.
    pub fn wig(self) -> Self {
        assert!(self.w.is_ignored());
        Self {
            w: WBit::WIG,
            ..self
        }
    }

    /// Set the single opcode for this VEX-encoded instruction.
    pub fn op(self, opcode: u8) -> Self {
        assert_eq!(self.opcode, u8::MAX);
        Self { opcode, ..self }
    }

    /// Set the ModR/M byte to contain a register operand; see [`Rex::r`].
    pub fn r(self) -> Self {
        assert!(self.modrm.is_none());
        Self {
            modrm: Some(ModRmKind::Reg),
            ..self
        }
    }

    fn validate(&self, _operands: &[Operand]) {
        assert!(self.opcode != u8::MAX);
        assert!(self.mmm.is_some());
    }

    /// Retrieve the digit extending the opcode, if available.
    #[must_use]
    pub fn unwrap_digit(&self) -> Option<u8> {
        match self.modrm {
            Some(ModRmKind::Digit(digit)) => Some(digit),
            _ => None,
        }
    }

    /// Set the digit extending the opcode; equivalent to `/<digit>` in the
    /// reference manual.
    ///
    /// # Panics
    ///
    /// Panics if `extension` is too large.
    #[must_use]
    pub fn digit(self, extension: u8) -> Self {
        assert!(extension <= 0b111, "must fit in 3 bits");
        Self {
            modrm: Some(ModRmKind::Digit(extension)),
            ..self
        }
    }

    /// Append a byte-sized immediate operand (8-bit); equivalent to `ib` in the
    /// reference manual.
    ///
    /// # Panics
    ///
    /// Panics if an immediate operand is already set.
    #[must_use]
    pub fn ib(self) -> Self {
        assert_eq!(self.imm, Imm::None);
        Self {
            imm: Imm::ib,
            ..self
        }
    }
}

impl From<Evex> for Encoding {
    fn from(evex: Evex) -> Encoding {
        Encoding::Evex(evex)
    }
}

impl fmt::Display for Evex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EVEX.{}", self.length)?;
        if let Some(pp) = self.pp {
            write!(f, ".{pp}")?;
        }
        if let Some(mmmmm) = self.mmm {
            write!(f, ".{mmmmm}")?;
        }
        write!(f, ".{} {:#04X}", self.w, self.opcode)?;
        if let Some(modrm) = self.modrm {
            write!(f, " {modrm}")?;
        }
        if self.imm != Imm::None {
            write!(f, " {}", self.imm)?;
        }
        Ok(())
    }
}

/// Tuple Type definitions used in EVEX encodings.
///
/// This enumeration corresponds to table 2-34 and 2-35 in the Intel manual.
/// This is a property of all instruction formats listed in the encoding table
/// for each instruction.
#[expect(missing_docs, reason = "matching manual names")]
pub enum TupleType {
    Full,
    Half,
    FullMem,
    Tuple1Scalar,
    Tuple1Fixed,
    Tuple2,
    Tuple4,
    Tuple8,
    HalfMem,
    QuarterMem,
    EigthMem,
    Mem128,
    Movddup,
}
