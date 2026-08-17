//! Register definitions for regalloc2.
//!
//! We define 16 GPRs, with indices equal to the hardware encoding,
//! and 16 XMM registers.
//!
//! Note also that we make use of pinned VRegs to refer to PRegs.

use crate::machinst::{RealReg, Reg};
use alloc::string::ToString;
use regalloc2::{PReg, RegClass, VReg};
use std::string::String;

// Hardware encodings (note the special rax, rcx, rdx, rbx order).

pub const ENC_RAX: u8 = 0;
pub const ENC_RCX: u8 = 1;
pub const ENC_RDX: u8 = 2;
pub const ENC_RBX: u8 = 3;
pub const ENC_RSP: u8 = 4;
pub const ENC_RBP: u8 = 5;
pub const ENC_RSI: u8 = 6;
pub const ENC_RDI: u8 = 7;
pub const ENC_R8: u8 = 8;
pub const ENC_R9: u8 = 9;
pub const ENC_R10: u8 = 10;
pub const ENC_R11: u8 = 11;
pub const ENC_R12: u8 = 12;
pub const ENC_R13: u8 = 13;
pub const ENC_R14: u8 = 14;
pub const ENC_R15: u8 = 15;

// Constructors for Regs.

const fn gpr(enc: u8) -> Reg {
    let preg = gpr_preg(enc);
    Reg::from_virtual_reg(VReg::new(preg.index(), RegClass::Int))
}
pub(crate) const fn gpr_preg(enc: u8) -> PReg {
    PReg::new(enc as usize, RegClass::Int)
}

pub(crate) const fn rsi() -> Reg {
    gpr(ENC_RSI)
}
pub(crate) const fn rdi() -> Reg {
    gpr(ENC_RDI)
}
pub(crate) const fn rax() -> Reg {
    gpr(ENC_RAX)
}
pub(crate) const fn rcx() -> Reg {
    gpr(ENC_RCX)
}
pub(crate) const fn rdx() -> Reg {
    gpr(ENC_RDX)
}
pub(crate) const fn r8() -> Reg {
    gpr(ENC_R8)
}
pub(crate) const fn r9() -> Reg {
    gpr(ENC_R9)
}
pub(crate) const fn r10() -> Reg {
    gpr(ENC_R10)
}
pub(crate) const fn r11() -> Reg {
    gpr(ENC_R11)
}
pub(crate) const fn r12() -> Reg {
    gpr(ENC_R12)
}
pub(crate) const fn r13() -> Reg {
    gpr(ENC_R13)
}
pub(crate) const fn r14() -> Reg {
    gpr(ENC_R14)
}
pub(crate) const fn rbx() -> Reg {
    gpr(ENC_RBX)
}

pub(crate) const fn r15() -> Reg {
    gpr(ENC_R15)
}

pub(crate) const fn rsp() -> Reg {
    gpr(ENC_RSP)
}
pub(crate) const fn rbp() -> Reg {
    gpr(ENC_RBP)
}

/// The pinned register on this architecture.
/// It must be the same as Spidermonkey's HeapReg, as found in this file.
/// https://searchfox.org/mozilla-central/source/js/src/jit/x64/Assembler-x64.h#99
pub(crate) const fn pinned_reg() -> Reg {
    r15()
}

const fn fpr(enc: u8) -> Reg {
    let preg = fpr_preg(enc);
    Reg::from_virtual_reg(VReg::new(preg.index(), RegClass::Float))
}

pub(crate) const fn fpr_preg(enc: u8) -> PReg {
    PReg::new(enc as usize, RegClass::Float)
}

pub(crate) const fn xmm0() -> Reg {
    fpr(0)
}
pub(crate) const fn xmm1() -> Reg {
    fpr(1)
}
pub(crate) const fn xmm2() -> Reg {
    fpr(2)
}
pub(crate) const fn xmm3() -> Reg {
    fpr(3)
}
pub(crate) const fn xmm4() -> Reg {
    fpr(4)
}
pub(crate) const fn xmm5() -> Reg {
    fpr(5)
}
pub(crate) const fn xmm6() -> Reg {
    fpr(6)
}
pub(crate) const fn xmm7() -> Reg {
    fpr(7)
}
pub(crate) const fn xmm8() -> Reg {
    fpr(8)
}
pub(crate) const fn xmm9() -> Reg {
    fpr(9)
}
pub(crate) const fn xmm10() -> Reg {
    fpr(10)
}
pub(crate) const fn xmm11() -> Reg {
    fpr(11)
}
pub(crate) const fn xmm12() -> Reg {
    fpr(12)
}
pub(crate) const fn xmm13() -> Reg {
    fpr(13)
}
pub(crate) const fn xmm14() -> Reg {
    fpr(14)
}
pub(crate) const fn xmm15() -> Reg {
    fpr(15)
}

/// Give the name of a RealReg.
pub fn realreg_name(reg: RealReg) -> &'static str {
    let preg = PReg::from(reg);
    match preg.class() {
        RegClass::Int => match preg.hw_enc() as u8 {
            ENC_RAX => "%rax",
            ENC_RBX => "%rbx",
            ENC_RCX => "%rcx",
            ENC_RDX => "%rdx",
            ENC_RSI => "%rsi",
            ENC_RDI => "%rdi",
            ENC_RBP => "%rbp",
            ENC_RSP => "%rsp",
            ENC_R8 => "%r8",
            ENC_R9 => "%r9",
            ENC_R10 => "%r10",
            ENC_R11 => "%r11",
            ENC_R12 => "%r12",
            ENC_R13 => "%r13",
            ENC_R14 => "%r14",
            ENC_R15 => "%r15",
            _ => panic!("Invalid PReg: {preg:?}"),
        },
        RegClass::Float => match preg.hw_enc() {
            0 => "%xmm0",
            1 => "%xmm1",
            2 => "%xmm2",
            3 => "%xmm3",
            4 => "%xmm4",
            5 => "%xmm5",
            6 => "%xmm6",
            7 => "%xmm7",
            8 => "%xmm8",
            9 => "%xmm9",
            10 => "%xmm10",
            11 => "%xmm11",
            12 => "%xmm12",
            13 => "%xmm13",
            14 => "%xmm14",
            15 => "%xmm15",
            _ => panic!("Invalid PReg: {preg:?}"),
        },
        RegClass::Vector => unreachable!(),
    }
}

pub fn show_reg(reg: Reg) -> String {
    if let Some(rreg) = reg.to_real_reg() {
        realreg_name(rreg).to_string()
    } else {
        format!("%{reg:?}")
    }
}

/// If `ireg` denotes an I64-classed reg, make a best-effort attempt to show its name at some
/// smaller size (4, 2 or 1 bytes).
pub fn show_ireg_sized(reg: Reg, size: u8) -> String {
    let mut s = show_reg(reg);

    if reg.class() != RegClass::Int || size == 8 {
        // We can't do any better.
        return s;
    }

    if reg.is_real() {
        // Change (eg) "rax" into "eax", "ax" or "al" as appropriate.  This is something one could
        // describe diplomatically as "a kludge", but it's only debug code.
        let remapper = match s.as_str() {
            "%rax" => Some(["%eax", "%ax", "%al"]),
            "%rbx" => Some(["%ebx", "%bx", "%bl"]),
            "%rcx" => Some(["%ecx", "%cx", "%cl"]),
            "%rdx" => Some(["%edx", "%dx", "%dl"]),
            "%rsi" => Some(["%esi", "%si", "%sil"]),
            "%rdi" => Some(["%edi", "%di", "%dil"]),
            "%rbp" => Some(["%ebp", "%bp", "%bpl"]),
            "%rsp" => Some(["%esp", "%sp", "%spl"]),
            "%r8" => Some(["%r8d", "%r8w", "%r8b"]),
            "%r9" => Some(["%r9d", "%r9w", "%r9b"]),
            "%r10" => Some(["%r10d", "%r10w", "%r10b"]),
            "%r11" => Some(["%r11d", "%r11w", "%r11b"]),
            "%r12" => Some(["%r12d", "%r12w", "%r12b"]),
            "%r13" => Some(["%r13d", "%r13w", "%r13b"]),
            "%r14" => Some(["%r14d", "%r14w", "%r14b"]),
            "%r15" => Some(["%r15d", "%r15w", "%r15b"]),
            _ => None,
        };
        if let Some(smaller_names) = remapper {
            match size {
                4 => s = smaller_names[0].into(),
                2 => s = smaller_names[1].into(),
                1 => s = smaller_names[2].into(),
                _ => panic!("show_ireg_sized: real"),
            }
        }
    } else {
        // Add a "l", "w" or "b" suffix to RegClass::I64 vregs used at narrower widths.
        let suffix = match size {
            4 => "l",
            2 => "w",
            1 => "b",
            _ => panic!("show_ireg_sized: virtual"),
        };
        s = s + suffix;
    }

    s
}

// N.B.: this is not an `impl PrettyPrint for Reg` because it is
// specific to x64; other backends have analogous functions. The
// disambiguation happens statically by virtue of higher-level,
// x64-specific, types calling the right `pretty_print_reg`. (In other
// words, we can't pretty-print a `Reg` all by itself in a build that
// may have multiple backends; but we can pretty-print one as part of
// an x64 Inst or x64 RegMemImm.)
pub fn pretty_print_reg(reg: Reg, size: u8) -> String {
    show_ireg_sized(reg, size)
}
