use crate::dsl::{Customization::*, Feature::*, Inst, Location::*};
use crate::dsl::{fmt, implicit, inst, r, rex, rw};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    // This is a bit long so it's extracted out here and shared below since it's
    // just the encoding of `cmpxchg16b` and `lock_cmpxchg16b` that differ.
    let cmpxchg16b_m = fmt("M", [rw(implicit(rax)), rw(implicit(rdx)), r(implicit(rbx)), r(implicit(rcx)), rw(m128)]);

    vec![
        // Note that for xchg the "MR" variants are omitted from the Intel
        // manual as they have the exact same encoding as the "RM" variant.
        // Additionally the "O" variants are omitted as they're just exchanging
        // registers which isn't needed by Cranelift at this time.
        //
        // Also note that these have a custom display implementation to swap the
        // order of the operands to match what Capstone prints.
        inst("xchgb", fmt("RM", [rw(r8), rw(m8)]), rex(0x86).r(), _64b | compat).custom(Display),
        inst("xchgw", fmt("RM", [rw(r16), rw(m16)]), rex([0x66, 0x87]).r(), _64b | compat).custom(Display),
        inst("xchgl", fmt("RM", [rw(r32), rw(m32)]), rex(0x87).r(), _64b | compat).custom(Display),
        inst("xchgq", fmt("RM", [rw(r64), rw(m64)]), rex(0x87).w().r(), _64b).custom(Display),

        inst("cmpxchg16b", cmpxchg16b_m.clone(), rex([0x0f, 0xc7]).digit(1).w(), _64b | cmpxchg16b),
        inst("lock_cmpxchg16b", cmpxchg16b_m.clone(), rex([0xf0, 0x0f, 0xc7]).digit(1).w(), _64b | cmpxchg16b).custom(Mnemonic),

        inst("cmpxchgb", fmt("MR", [rw(rm8), r(r8), rw(implicit(al))]), rex([0x0f, 0xb0]).r(), _64b | compat),
        inst("cmpxchgw", fmt("MR", [rw(rm16), r(r16), rw(implicit(ax))]), rex([0x66, 0x0f, 0xb1]).r(), _64b | compat),
        inst("cmpxchgl", fmt("MR", [rw(rm32), r(r32), rw(implicit(eax))]), rex([0x0f, 0xb1]).r(), _64b | compat),
        inst("cmpxchgq", fmt("MR", [rw(rm64), r(r64), rw(implicit(rax))]), rex([0x0f, 0xb1]).w().r(), _64b | compat),
        inst("lock_cmpxchgb", fmt("MR", [rw(m8), r(r8), rw(implicit(al))]), rex([0xf0, 0x0f, 0xb0]).r(), _64b | compat).custom(Mnemonic),
        inst("lock_cmpxchgw", fmt("MR", [rw(m16), r(r16), rw(implicit(ax))]), rex([0xf0, 0x66, 0x0f, 0xb1]).r(), _64b | compat).custom(Mnemonic),
        inst("lock_cmpxchgl", fmt("MR", [rw(m32), r(r32), rw(implicit(eax))]), rex([0xf0, 0x0f, 0xb1]).r(), _64b | compat).custom(Mnemonic),
        inst("lock_cmpxchgq", fmt("MR", [rw(m64), r(r64), rw(implicit(rax))]), rex([0xf0, 0x0f, 0xb1]).w().r(), _64b | compat).custom(Mnemonic),
    ]
}
