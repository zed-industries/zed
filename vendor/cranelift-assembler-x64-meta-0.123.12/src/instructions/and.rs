use crate::dsl::{Customization::*, Feature::*, Inst, Length::*, Location::*};
use crate::dsl::{align, fmt, inst, r, rex, rw, sxl, sxq, vex, w};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    // Note that some versions of the reference manual show `REX + <opcode>`
    // rows that (a) are only intended for documentation purposes, i.e., to note
    // that `r/m8` cannot be encoded to access byte registers AH, BH, CH, DH if
    // a REX prefix is used, and (b) have known errors indicating
    // "sign-extended" when in fact this is not the case. We skip those rows
    // here and indicate the true sign extension operations with a `_SX<from
    // width>` suffix.
    vec![
        inst("andb", fmt("I", [rw(al), r(imm8)]), rex(0x24).ib(), _64b | compat),
        inst("andw", fmt("I", [rw(ax), r(imm16)]), rex([0x66, 0x25]).iw(), _64b | compat),
        inst("andl", fmt("I", [rw(eax), r(imm32)]), rex(0x25).id(), _64b | compat),
        inst("andq", fmt("I_SXL", [rw(rax), sxq(imm32)]), rex(0x25).w().id(), _64b),
        inst("andb", fmt("MI", [rw(rm8), r(imm8)]), rex(0x80).digit(4).ib(), _64b | compat),
        inst("andw", fmt("MI", [rw(rm16), r(imm16)]), rex([0x66, 0x81]).digit(4).iw(), _64b | compat),
        inst("andl", fmt("MI", [rw(rm32), r(imm32)]), rex(0x81).digit(4).id(), _64b | compat),
        inst("andq", fmt("MI_SXL", [rw(rm64), sxq(imm32)]), rex(0x81).w().digit(4).id(), _64b),
        inst("andl", fmt("MI_SXB", [rw(rm32), sxl(imm8)]), rex(0x83).digit(4).ib(), _64b | compat),
        inst("andq", fmt("MI_SXB", [rw(rm64), sxq(imm8)]), rex(0x83).w().digit(4).ib(), _64b),
        inst("andb", fmt("MR", [rw(rm8), r(r8)]), rex(0x20).r(), _64b | compat),
        inst("andw", fmt("MR", [rw(rm16), r(r16)]), rex([0x66, 0x21]).r(), _64b | compat),
        inst("andl", fmt("MR", [rw(rm32), r(r32)]), rex(0x21).r(), _64b | compat),
        inst("andq", fmt("MR", [rw(rm64), r(r64)]), rex(0x21).w().r(), _64b),
        inst("andb", fmt("RM", [rw(r8), r(rm8)]), rex(0x22).r(), _64b | compat),
        inst("andw", fmt("RM", [rw(r16), r(rm16)]), rex([0x66, 0x23]).r(), _64b | compat),
        inst("andl", fmt("RM", [rw(r32), r(rm32)]), rex(0x23).r(), _64b | compat),
        inst("andq", fmt("RM", [rw(r64), r(rm64)]), rex(0x23).w().r(), _64b),
        // BMI1 andn
        inst("andnl", fmt("RVM", [w(r32a), r(r32b), r(rm32)]), vex(LZ)._0f38().w0().op(0xF2), _64b | compat | bmi1),
        inst("andnq", fmt("RVM", [w(r64a), r(r64b), r(rm64)]), vex(LZ)._0f38().w1().op(0xF2), _64b | bmi1),
        // `LOCK`-prefixed memory-writing instructions.
        inst("lock_andb", fmt("MI", [rw(m8), r(imm8)]), rex([0xf0, 0x80]).digit(4).ib(), _64b | compat).custom(Mnemonic),
        inst("lock_andw", fmt("MI", [rw(m16), r(imm16)]), rex([0xf0, 0x66, 0x81]).digit(4).iw(), _64b | compat).custom(Mnemonic),
        inst("lock_andl", fmt("MI", [rw(m32), r(imm32)]), rex([0xf0, 0x81]).digit(4).id(), _64b | compat).custom(Mnemonic),
        inst("lock_andq", fmt("MI_SXL", [rw(m64), sxq(imm32)]), rex([0xf0, 0x81]).w().digit(4).id(), _64b).custom(Mnemonic),
        inst("lock_andl", fmt("MI_SXB", [rw(m32), sxl(imm8)]), rex([0xf0, 0x83]).digit(4).ib(), _64b | compat).custom(Mnemonic),
        inst("lock_andq", fmt("MI_SXB", [rw(m64), sxq(imm8)]), rex([0xf0, 0x83]).w().digit(4).ib(), _64b).custom(Mnemonic),
        inst("lock_andb", fmt("MR", [rw(m8), r(r8)]), rex([0xf0, 0x20]).r(), _64b | compat).custom(Mnemonic),
        inst("lock_andw", fmt("MR", [rw(m16), r(r16)]), rex([0xf0, 0x66, 0x21]).r(), _64b | compat).custom(Mnemonic),
        inst("lock_andl", fmt("MR", [rw(m32), r(r32)]), rex([0xf0, 0x21]).r(), _64b | compat).custom(Mnemonic),
        inst("lock_andq", fmt("MR", [rw(m64), r(r64)]), rex([0xf0, 0x21]).w().r(), _64b).custom(Mnemonic),
        // Vector instructions.
        inst("andps", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x0F, 0x54]).r(), _64b | compat | sse).alt(avx, "vandps_b"),
        inst("andpd", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x54]).r(), _64b | compat | sse2).alt(avx, "vandpd_b"),
        inst("andnps", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x0F, 0x55]).r(), _64b | compat | sse).alt(avx, "vandnps_b"),
        inst("andnpd", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x55]).r(), _64b | compat | sse2).alt(avx, "vandnpd_b"),
        inst("pand", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0xDB]).r(), _64b | compat | sse2).alt(avx, "vpand_b"),
        inst("pandn", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0xDF]).r(), _64b | compat | sse2).alt(avx, "vpandn_b"),
        inst("vandps", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._0f().op(0x54).r(), _64b | compat | avx),
        inst("vandpd", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0x54).r(), _64b | compat | avx),
        inst("vandnps", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._0f().op(0x55).r(), _64b | compat | avx),
        inst("vandnpd", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0x55).r(), _64b | compat | avx),
        inst("vpand", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0xDB).r(), _64b | compat | avx),
        inst("vpandn", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0xDF).r(), _64b | compat | avx),
    ]
}
