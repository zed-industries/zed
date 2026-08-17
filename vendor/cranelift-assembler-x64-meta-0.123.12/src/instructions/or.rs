use crate::dsl::{Customization::*, Feature::*, Inst, Length::*, Location::*};
use crate::dsl::{align, fmt, inst, r, rex, rw, sxl, sxq, vex, w};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    vec![
        inst("orb", fmt("I", [rw(al), r(imm8)]), rex(0x0C).ib(), _64b | compat),
        inst("orw", fmt("I", [rw(ax), r(imm16)]), rex([0x66, 0x0D]).iw(), _64b | compat),
        inst("orl", fmt("I", [rw(eax), r(imm32)]), rex(0x0D).id(), _64b | compat),
        inst("orq", fmt("I_SXL", [rw(rax), sxq(imm32)]), rex(0x0D).w().id(), _64b),
        inst("orb", fmt("MI", [rw(rm8), r(imm8)]), rex(0x80).digit(1).ib(), _64b | compat),
        inst("orw", fmt("MI", [rw(rm16), r(imm16)]), rex([0x66, 0x81]).digit(1).iw(), _64b | compat),
        inst("orl", fmt("MI", [rw(rm32), r(imm32)]), rex(0x81).digit(1).id(), _64b | compat),
        inst("orq", fmt("MI_SXL", [rw(rm64), sxq(imm32)]), rex(0x81).w().digit(1).id(), _64b),
        inst("orl", fmt("MI_SXB", [rw(rm32), sxl(imm8)]), rex(0x83).digit(1).ib(), _64b | compat),
        inst("orq", fmt("MI_SXB", [rw(rm64), sxq(imm8)]), rex(0x83).w().digit(1).ib(), _64b),
        inst("orb", fmt("MR", [rw(rm8), r(r8)]), rex(0x08).r(), _64b | compat),
        inst("orw", fmt("MR", [rw(rm16), r(r16)]), rex([0x66, 0x09]).r(), _64b | compat),
        inst("orl", fmt("MR", [rw(rm32), r(r32)]), rex(0x09).r(), _64b | compat),
        inst("orq", fmt("MR", [rw(rm64), r(r64)]), rex(0x09).w().r(), _64b),
        inst("orb", fmt("RM", [rw(r8), r(rm8)]), rex(0x0A).r(), _64b | compat),
        inst("orw", fmt("RM", [rw(r16), r(rm16)]), rex([0x66, 0x0B]).r(), _64b | compat),
        inst("orl", fmt("RM", [rw(r32), r(rm32)]), rex(0x0B).r(), _64b | compat),
        inst("orq", fmt("RM", [rw(r64), r(rm64)]), rex(0x0B).w().r(), _64b),
        // `LOCK`-prefixed memory-writing instructions.
        inst("lock_orb", fmt("MI", [rw(m8), r(imm8)]), rex([0xf0, 0x80]).digit(1).ib(), _64b | compat).custom(Mnemonic),
        inst("lock_orw", fmt("MI", [rw(m16), r(imm16)]), rex([0xf0, 0x66, 0x81]).digit(1).iw(), _64b | compat).custom(Mnemonic),
        inst("lock_orl", fmt("MI", [rw(m32), r(imm32)]), rex([0xf0, 0x81]).digit(1).id(), _64b | compat).custom(Mnemonic),
        inst("lock_orq", fmt("MI_SXL", [rw(m64), sxq(imm32)]), rex([0xf0, 0x81]).w().digit(1).id(), _64b).custom(Mnemonic),
        inst("lock_orl", fmt("MI_SXB", [rw(m32), sxl(imm8)]), rex([0xf0, 0x83]).digit(1).ib(), _64b | compat).custom(Mnemonic),
        inst("lock_orq", fmt("MI_SXB", [rw(m64), sxq(imm8)]), rex([0xf0, 0x83]).w().digit(1).ib(), _64b).custom(Mnemonic),
        inst("lock_orb", fmt("MR", [rw(m8), r(r8)]), rex([0xf0, 0x08]).r(), _64b | compat).custom(Mnemonic),
        inst("lock_orw", fmt("MR", [rw(m16), r(r16)]), rex([0xf0, 0x66, 0x09]).r(), _64b | compat).custom(Mnemonic),
        inst("lock_orl", fmt("MR", [rw(m32), r(r32)]), rex([0xf0, 0x09]).r(), _64b | compat).custom(Mnemonic),
        inst("lock_orq", fmt("MR", [rw(m64), r(r64)]), rex([0xf0, 0x09]).w().r(), _64b).custom(Mnemonic),
        // Vector instructions.
        inst("orps", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x0F, 0x56]).r(), _64b | compat | sse).alt(avx, "vorps_b"),
        inst("orpd", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x56]).r(), _64b | compat | sse2).alt(avx, "vorpd_b"),
        inst("por", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0xEB]).r(), _64b | compat | sse2).alt(avx, "vpor_b"),
        inst("vorps", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._0f().op(0x56).r(), _64b | compat | avx),
        inst("vorpd", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0x56).r(), _64b | compat | avx),
        inst("vpor", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0xEB).r(), _64b | compat | avx),
    ]
}
