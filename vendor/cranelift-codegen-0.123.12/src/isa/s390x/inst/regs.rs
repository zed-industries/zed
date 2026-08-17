//! S390x ISA definitions: registers.

use alloc::string::String;
use regalloc2::PReg;

use crate::isa::s390x::inst::{RegPair, WritableRegPair};
use crate::machinst::*;

//=============================================================================
// Registers, the Universe thereof, and printing

/// Get a reference to a GPR (integer register).
pub const fn gpr(num: u8) -> Reg {
    Reg::from_real_reg(gpr_preg(num))
}

pub(crate) const fn gpr_preg(num: u8) -> PReg {
    assert!(num < 16);
    PReg::new(num as usize, RegClass::Int)
}

/// Get a writable reference to a GPR.
pub fn writable_gpr(num: u8) -> Writable<Reg> {
    Writable::from_reg(gpr(num))
}

/// Get a reference to a VR (vector register).
pub const fn vr(num: u8) -> Reg {
    Reg::from_real_reg(vr_preg(num))
}

pub(crate) const fn vr_preg(num: u8) -> PReg {
    assert!(num < 32);
    PReg::new(num as usize, RegClass::Float)
}

/// Get a writable reference to a VR.
pub fn writable_vr(num: u8) -> Writable<Reg> {
    Writable::from_reg(vr(num))
}

/// Test whether a vector register is overlapping an FPR.
pub fn is_fpr(r: Reg) -> bool {
    let r = r.to_real_reg().unwrap();
    assert!(r.class() == RegClass::Float);
    return r.hw_enc() < 16;
}

/// Get a reference to the stack-pointer register.
pub fn stack_reg() -> Reg {
    gpr(15)
}

/// Get a writable reference to the stack-pointer register.
pub fn writable_stack_reg() -> Writable<Reg> {
    Writable::from_reg(stack_reg())
}

/// Get a reference to the first temporary, sometimes "spill temporary", register. This register is
/// used to compute the address of a spill slot when a direct offset addressing mode from FP is not
/// sufficient (+/- 2^11 words). We exclude this register from regalloc and reserve it for this
/// purpose for simplicity; otherwise we need a multi-stage analysis where we first determine how
/// many spill slots we have, then perhaps remove the reg from the pool and recompute regalloc.
///
/// We use r1 for this because it's a scratch register but is slightly special (used for linker
/// veneers). We're free to use it as long as we don't expect it to live through call instructions.
pub fn spilltmp_reg() -> Reg {
    gpr(1)
}

/// Get a writable reference to the spilltmp reg.
pub fn writable_spilltmp_reg() -> Writable<Reg> {
    Writable::from_reg(spilltmp_reg())
}

pub fn zero_reg() -> Reg {
    gpr(0)
}

pub fn show_reg(reg: Reg) -> String {
    if let Some(rreg) = reg.to_real_reg() {
        match rreg.class() {
            RegClass::Int => format!("%r{}", rreg.hw_enc()),
            RegClass::Float => format!("%v{}", rreg.hw_enc()),
            RegClass::Vector => unreachable!(),
        }
    } else {
        format!("%{reg:?}")
    }
}

pub fn maybe_show_fpr(reg: Reg) -> Option<String> {
    if let Some(rreg) = reg.to_real_reg() {
        if is_fpr(reg) {
            return Some(format!("%f{}", rreg.hw_enc()));
        }
    }
    None
}

pub fn pretty_print_reg(reg: Reg) -> String {
    show_reg(reg)
}

pub fn pretty_print_regpair(pair: RegPair) -> String {
    let hi = pair.hi;
    let lo = pair.lo;
    if let Some(hi_reg) = hi.to_real_reg() {
        if let Some(lo_reg) = lo.to_real_reg() {
            assert!(
                hi_reg.hw_enc() + 1 == lo_reg.hw_enc(),
                "Invalid regpair: {} {}",
                show_reg(hi),
                show_reg(lo)
            );
            return show_reg(hi);
        }
    }

    format!("{}/{}", show_reg(hi), show_reg(lo))
}

pub fn pretty_print_fp_regpair(pair: RegPair) -> String {
    let hi = pair.hi;
    let lo = pair.lo;
    if let Some(hi_reg) = hi.to_real_reg() {
        if let Some(lo_reg) = lo.to_real_reg() {
            assert!(
                hi_reg.hw_enc() + 2 == lo_reg.hw_enc(),
                "Invalid regpair: {} {}",
                show_reg(hi),
                show_reg(lo)
            );
            return maybe_show_fpr(hi).unwrap();
        }
    }

    format!("{}/{}", show_reg(hi), show_reg(lo))
}

pub fn pretty_print_reg_mod(rd: Writable<Reg>, ri: Reg) -> String {
    let output = rd.to_reg();
    let input = ri;
    if output == input {
        show_reg(output)
    } else {
        format!("{}<-{}", show_reg(output), show_reg(input))
    }
}

pub fn pretty_print_regpair_mod(rd: WritableRegPair, ri: RegPair) -> String {
    let rd_hi = rd.hi.to_reg();
    let rd_lo = rd.lo.to_reg();
    let ri_hi = ri.hi;
    let ri_lo = ri.lo;
    if rd_hi == ri_hi {
        show_reg(rd_hi)
    } else {
        format!(
            "{}/{}<-{}/{}",
            show_reg(rd_hi),
            show_reg(rd_lo),
            show_reg(ri_hi),
            show_reg(ri_lo)
        )
    }
}

pub fn pretty_print_regpair_mod_lo(rd: WritableRegPair, ri: Reg) -> String {
    let rd_hi = rd.hi.to_reg();
    let rd_lo = rd.lo.to_reg();
    if rd_lo == ri {
        show_reg(rd_hi)
    } else {
        format!(
            "{}/{}<-_/{}",
            show_reg(rd_hi),
            show_reg(rd_lo),
            show_reg(ri),
        )
    }
}

pub fn pretty_print_fpr(reg: Reg) -> (String, Option<String>) {
    (show_reg(reg), maybe_show_fpr(reg))
}
