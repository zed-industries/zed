use crate::dsl::{Customization::*, Feature::*, Inst, Length::*, Location::*};
use crate::dsl::{align, fmt, inst, r, rex, rw, sxl, sxq, vex, w};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    vec![
        inst("xorb", fmt("I", [rw(al), r(imm8)]), rex(0x34).ib(), _64b | compat),
        inst("xorw", fmt("I", [rw(ax), r(imm16)]), rex([0x66, 0x35]).iw(), _64b | compat),
        inst("xorl", fmt("I", [rw(eax), r(imm32)]), rex(0x35).id(), _64b | compat),
        inst("xorq", fmt("I_SXL", [rw(rax), sxq(imm32)]), rex(0x35).w().id(), _64b),
        inst("xorb", fmt("MI", [rw(rm8), r(imm8)]), rex(0x80).digit(6).ib(), _64b | compat),
        inst("xorw", fmt("MI", [rw(rm16), r(imm16)]), rex([0x66, 0x81]).digit(6).iw(), _64b | compat),
        inst("xorl", fmt("MI", [rw(rm32), r(imm32)]), rex(0x81).digit(6).id(), _64b | compat),
        inst("xorq", fmt("MI_SXL", [rw(rm64), sxq(imm32)]), rex(0x81).w().digit(6).id(), _64b),
        inst("xorl", fmt("MI_SXB", [rw(rm32), sxl(imm8)]), rex(0x83).digit(6).ib(), _64b | compat),
        inst("xorq", fmt("MI_SXB", [rw(rm64), sxq(imm8)]), rex(0x83).w().digit(6).ib(), _64b),
        inst("xorb", fmt("MR", [rw(rm8), r(r8)]), rex(0x30).r(), _64b | compat),
        inst("xorw", fmt("MR", [rw(rm16), r(r16)]), rex([0x66, 0x31]).r(), _64b | compat),
        inst("xorl", fmt("MR", [rw(rm32), r(r32)]), rex(0x31).r(), _64b | compat),
        inst("xorq", fmt("MR", [rw(rm64), r(r64)]), rex(0x31).w().r(), _64b),
        inst("xorb", fmt("RM", [rw(r8), r(rm8)]), rex(0x32).r(), _64b | compat),
        inst("xorw", fmt("RM", [rw(r16), r(rm16)]), rex([0x66, 0x33]).r(), _64b | compat),
        inst("xorl", fmt("RM", [rw(r32), r(rm32)]), rex(0x33).r(), _64b | compat),
        inst("xorq", fmt("RM", [rw(r64), r(rm64)]), rex(0x33).w().r(), _64b),
        // `LOCK`-prefixed memory-writing instructions.
        inst("lock_xorb", fmt("MI", [rw(m8), r(imm8)]), rex([0xf0, 0x80]).digit(6).ib(), _64b | compat).custom(Mnemonic),
        inst("lock_xorw", fmt("MI", [rw(m16), r(imm16)]), rex([0xf0, 0x66, 0x81]).digit(6).iw(), _64b | compat).custom(Mnemonic),
        inst("lock_xorl", fmt("MI", [rw(m32), r(imm32)]), rex([0xf0, 0x81]).digit(6).id(), _64b | compat).custom(Mnemonic),
        inst("lock_xorq", fmt("MI_SXL", [rw(m64), sxq(imm32)]), rex([0xf0, 0x81]).w().digit(6).id(), _64b).custom(Mnemonic),
        inst("lock_xorl", fmt("MI_SXB", [rw(m32), sxl(imm8)]), rex([0xf0, 0x83]).digit(6).ib(), _64b | compat).custom(Mnemonic),
        inst("lock_xorq", fmt("MI_SXB", [rw(m64), sxq(imm8)]), rex([0xf0, 0x83]).w().digit(6).ib(), _64b).custom(Mnemonic),
        inst("lock_xorb", fmt("MR", [rw(m8), r(r8)]), rex([0xf0, 0x30]).r(), _64b | compat).custom(Mnemonic),
        inst("lock_xorw", fmt("MR", [rw(m16), r(r16)]), rex([0xf0, 0x66, 0x31]).r(), _64b | compat).custom(Mnemonic),
        inst("lock_xorl", fmt("MR", [rw(m32), r(r32)]), rex([0xf0, 0x31]).r(), _64b | compat).custom(Mnemonic),
        inst("lock_xorq", fmt("MR", [rw(m64), r(r64)]), rex([0xf0, 0x31]).w().r(), _64b).custom(Mnemonic),
        // Vector instructions.
        inst("xorps", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x0F, 0x57]).r(), _64b | compat | sse).alt(avx, "vxorps_b"),
        inst("xorpd", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x57]).r(), _64b | compat | sse2).alt(avx, "vxorpd_b"),
        inst("pxor", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0xEF]).r(), _64b | compat | sse2).alt(avx, "vpxor_b"),
        inst("vxorps", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._0f().op(0x57).r(), _64b | compat | avx),
        inst("vxorpd", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0x57).r(), _64b | compat | avx),
        inst("vpxor", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0xEF).r(), _64b | compat | avx),
    ]
}
