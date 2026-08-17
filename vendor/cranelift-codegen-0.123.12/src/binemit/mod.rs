//! Binary machine code emission.
//!
//! The `binemit` module contains code for translating Cranelift's intermediate representation into
//! binary machine code.

use core::fmt;
#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

/// Offset in bytes from the beginning of the function.
///
/// Cranelift can be used as a cross compiler, so we don't want to use a type like `usize` which
/// depends on the *host* platform, not the *target* platform.
pub type CodeOffset = u32;

/// Addend to add to the symbol value.
pub type Addend = i64;

/// Relocation kinds for every ISA
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub enum Reloc {
    /// absolute 4-byte
    Abs4,
    /// absolute 8-byte
    Abs8,
    /// x86 PC-relative 4-byte
    X86PCRel4,
    /// x86 call to PC-relative 4-byte
    X86CallPCRel4,
    /// x86 call to PLT-relative 4-byte
    X86CallPLTRel4,
    /// x86 GOT PC-relative 4-byte
    X86GOTPCRel4,
    /// The 32-bit offset of the target from the beginning of its section.
    /// Equivalent to `IMAGE_REL_AMD64_SECREL`.
    /// See: [PE Format](https://docs.microsoft.com/en-us/windows/win32/debug/pe-format)
    X86SecRel,
    /// Arm32 call target
    Arm32Call,
    /// Arm64 call target. Encoded as bottom 26 bits of instruction. This
    /// value is sign-extended, multiplied by 4, and added to the PC of
    /// the call instruction to form the destination address.
    Arm64Call,
    /// s390x PC-relative 4-byte offset
    S390xPCRel32Dbl,
    /// s390x PC-relative 4-byte offset to PLT
    S390xPLTRel32Dbl,

    /// Elf x86_64 32 bit signed PC relative offset to two GOT entries for GD symbol.
    ElfX86_64TlsGd,

    /// Mach-O x86_64 32 bit signed PC relative offset to a `__thread_vars` entry.
    MachOX86_64Tlv,

    /// Mach-O Aarch64 TLS
    /// PC-relative distance to the page of the TLVP slot.
    MachOAarch64TlsAdrPage21,

    /// Mach-O Aarch64 TLS
    /// Offset within page of TLVP slot.
    MachOAarch64TlsAdrPageOff12,

    /// Aarch64 TLSDESC Adr Page21
    /// This is equivalent to `R_AARCH64_TLSDESC_ADR_PAGE21` in the [aaelf64](https://github.com/ARM-software/abi-aa/blob/2bcab1e3b22d55170c563c3c7940134089176746/aaelf64/aaelf64.rst#57105thread-local-storage-descriptors)
    Aarch64TlsDescAdrPage21,

    /// Aarch64 TLSDESC Ld64 Lo12
    /// This is equivalent to `R_AARCH64_TLSDESC_LD64_LO12` in the [aaelf64](https://github.com/ARM-software/abi-aa/blob/2bcab1e3b22d55170c563c3c7940134089176746/aaelf64/aaelf64.rst#57105thread-local-storage-descriptors)
    Aarch64TlsDescLd64Lo12,

    /// Aarch64 TLSDESC Add Lo12
    /// This is equivalent to `R_AARCH64_TLSGD_ADD_LO12` in the [aaelf64](https://github.com/ARM-software/abi-aa/blob/2bcab1e3b22d55170c563c3c7940134089176746/aaelf64/aaelf64.rst#57105thread-local-storage-descriptors)
    Aarch64TlsDescAddLo12,

    /// Aarch64 TLSDESC Call
    /// This is equivalent to `R_AARCH64_TLSDESC_CALL` in the [aaelf64](https://github.com/ARM-software/abi-aa/blob/2bcab1e3b22d55170c563c3c7940134089176746/aaelf64/aaelf64.rst#57105thread-local-storage-descriptors)
    Aarch64TlsDescCall,

    /// AArch64 GOT Page
    /// Set the immediate value of an ADRP to bits 32:12 of X; check that –2^32 <= X < 2^32
    /// This is equivalent to `R_AARCH64_ADR_GOT_PAGE` (311) in the  [aaelf64](https://github.com/ARM-software/abi-aa/blob/2bcab1e3b22d55170c563c3c7940134089176746/aaelf64/aaelf64.rst#static-aarch64-relocations)
    Aarch64AdrGotPage21,

    /// AArch64 GOT Low bits

    /// Set the LD/ST immediate field to bits 11:3 of X. No overflow check; check that X&7 = 0
    /// This is equivalent to `R_AARCH64_LD64_GOT_LO12_NC` (312) in the  [aaelf64](https://github.com/ARM-software/abi-aa/blob/2bcab1e3b22d55170c563c3c7940134089176746/aaelf64/aaelf64.rst#static-aarch64-relocations)
    Aarch64Ld64GotLo12Nc,

    /// RISC-V Call PLT: 32-bit PC-relative function call, macros call, tail (PIC)
    ///
    /// Despite having PLT in the name, this relocation is also used for normal calls.
    /// The non-PLT version of this relocation has been deprecated.
    ///
    /// This is the `R_RISCV_CALL_PLT` relocation from the RISC-V ELF psABI document.
    /// <https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-elf.adoc#procedure-calls>
    RiscvCallPlt,

    /// RISC-V TLS GD: High 20 bits of 32-bit PC-relative TLS GD GOT reference,
    ///
    /// This is the `R_RISCV_TLS_GD_HI20` relocation from the RISC-V ELF psABI document.
    /// <https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-elf.adoc#global-dynamic>
    RiscvTlsGdHi20,

    /// Low 12 bits of a 32-bit PC-relative relocation (I-Type instruction)
    ///
    /// This is the `R_RISCV_PCREL_LO12_I` relocation from the RISC-V ELF psABI document.
    /// <https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-elf.adoc#pc-relative-symbol-addresses>
    RiscvPCRelLo12I,

    /// High 20 bits of a 32-bit PC-relative GOT offset relocation
    ///
    /// This is the `R_RISCV_GOT_HI20` relocation from the RISC-V ELF psABI document.
    /// <https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-elf.adoc#pc-relative-symbol-addresses>
    RiscvGotHi20,

    /// s390x TLS GD64 - 64-bit offset of tls_index for GD symbol in GOT
    S390xTlsGd64,
    /// s390x TLS GDCall - marker to enable optimization of TLS calls
    S390xTlsGdCall,

    /// Pulley - call a host function indirectly where the embedder resolving
    /// this relocation needs to fill the 8-bit immediate that's part of the
    /// `call_indirect_host` opcode (an opaque identifier used by the host).
    PulleyCallIndirectHost,
}

impl fmt::Display for Reloc {
    /// Display trait implementation drops the arch, since its used in contexts where the arch is
    /// already unambiguous, e.g. clif syntax with isa specified. In other contexts, use Debug.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Abs4 => write!(f, "Abs4"),
            Self::Abs8 => write!(f, "Abs8"),
            Self::S390xPCRel32Dbl => write!(f, "PCRel32Dbl"),
            Self::S390xPLTRel32Dbl => write!(f, "PLTRel32Dbl"),
            Self::X86PCRel4 => write!(f, "PCRel4"),
            Self::X86CallPCRel4 => write!(f, "CallPCRel4"),
            Self::X86CallPLTRel4 => write!(f, "CallPLTRel4"),
            Self::X86GOTPCRel4 => write!(f, "GOTPCRel4"),
            Self::X86SecRel => write!(f, "SecRel"),
            Self::Arm32Call | Self::Arm64Call => write!(f, "Call"),
            Self::RiscvCallPlt => write!(f, "RiscvCallPlt"),
            Self::RiscvTlsGdHi20 => write!(f, "RiscvTlsGdHi20"),
            Self::RiscvGotHi20 => write!(f, "RiscvGotHi20"),
            Self::RiscvPCRelLo12I => write!(f, "RiscvPCRelLo12I"),
            Self::ElfX86_64TlsGd => write!(f, "ElfX86_64TlsGd"),
            Self::MachOX86_64Tlv => write!(f, "MachOX86_64Tlv"),
            Self::MachOAarch64TlsAdrPage21 => write!(f, "MachOAarch64TlsAdrPage21"),
            Self::MachOAarch64TlsAdrPageOff12 => write!(f, "MachOAarch64TlsAdrPageOff12"),
            Self::Aarch64TlsDescAdrPage21 => write!(f, "Aarch64TlsDescAdrPage21"),
            Self::Aarch64TlsDescLd64Lo12 => write!(f, "Aarch64TlsDescLd64Lo12"),
            Self::Aarch64TlsDescAddLo12 => write!(f, "Aarch64TlsDescAddLo12"),
            Self::Aarch64TlsDescCall => write!(f, "Aarch64TlsDescCall"),
            Self::Aarch64AdrGotPage21 => write!(f, "Aarch64AdrGotPage21"),
            Self::Aarch64Ld64GotLo12Nc => write!(f, "Aarch64AdrGotLo12Nc"),
            Self::S390xTlsGd64 => write!(f, "TlsGd64"),
            Self::S390xTlsGdCall => write!(f, "TlsGdCall"),
            Self::PulleyCallIndirectHost => write!(f, "PulleyCallIndirectHost"),
        }
    }
}

/// Container for information about a vector of compiled code and its supporting read-only data.
///
/// The code starts at offset 0 and is followed optionally by relocatable jump tables and copyable
/// (raw binary) read-only data.  Any padding between sections is always part of the section that
/// precedes the boundary between the sections.
#[derive(Debug, PartialEq)]
pub struct CodeInfo {
    /// Number of bytes in total.
    pub total_size: CodeOffset,
}
