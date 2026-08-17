use crate::dsl::{Customization::*, Feature::*, Inst, Length::*, Location::*, TupleType::*};
use crate::dsl::{align, evex, fmt, implicit, inst, r, rex, rw, sxl, sxq, sxw, vex, w};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    vec![
        // Multiply unsigned; low bits in `rax`, high bits in `rdx`.
        inst("mulb", fmt("M", [rw(implicit(ax)), r(rm8)]), rex(0xF6).digit(4), _64b | compat),
        inst("mulw", fmt("M", [rw(implicit(ax)), w(implicit(dx)), r(rm16)]), rex([0x66, 0xF7]).digit(4), _64b | compat),
        inst("mull", fmt("M", [rw(implicit(eax)), w(implicit(edx)), r(rm32)]), rex(0xF7).digit(4), _64b | compat),
        inst("mulq", fmt("M", [rw(implicit(rax)), w(implicit(rdx)), r(rm64)]), rex(0xF7).w().digit(4), _64b),
        // Multiply signed; low bits in `rax`, high bits in `rdx`.
        inst("imulb", fmt("M", [rw(implicit(ax)), r(rm8)]), rex(0xF6).digit(5), _64b | compat),
        inst("imulw", fmt("M", [rw(implicit(ax)), w(implicit(dx)), r(rm16)]), rex([0x66, 0xF7]).digit(5), _64b | compat),
        inst("imull", fmt("M", [rw(implicit(eax)), w(implicit(edx)), r(rm32)]), rex(0xF7).digit(5), _64b | compat),
        inst("imulq", fmt("M", [rw(implicit(rax)), w(implicit(rdx)), r(rm64)]), rex(0xF7).w().digit(5), _64b),
        inst("imulw", fmt("RM", [rw(r16), r(rm16)]), rex([0x66, 0x0F, 0xAF]), _64b | compat),
        inst("imull", fmt("RM", [rw(r32), r(rm32)]), rex([0x0F, 0xAF]), _64b | compat),
        inst("imulq", fmt("RM", [rw(r64), r(rm64)]), rex([0x0F, 0xAF]).w(), _64b),
        inst("imulw", fmt("RMI_SXB", [w(r16), r(rm16), sxw(imm8)]), rex([0x66, 0x6B]).ib(), _64b | compat),
        inst("imull", fmt("RMI_SXB", [w(r32), r(rm32), sxl(imm8)]), rex(0x6B).ib(), _64b | compat),
        inst("imulq", fmt("RMI_SXB", [w(r64), r(rm64), sxq(imm8)]), rex(0x6B).w().ib(), _64b),
        inst("imulw", fmt("RMI", [w(r16), r(rm16), r(imm16)]), rex([0x66, 0x69]).iw(), _64b | compat),
        inst("imull", fmt("RMI", [w(r32), r(rm32), r(imm32)]), rex(0x69).id(), _64b | compat),
        inst("imulq", fmt("RMI_SXL", [w(r64), r(rm64), sxq(imm32)]), rex(0x69).w().id(), _64b),
        // MULX, a BMI2 extension
        //
        // Note that these have custom visitors for regalloc to handle the case
        // where both output operands are the same. This is where the
        // instruction only calculates the high half of the multiplication, as
        // opposed to when the operands are different to calculate both the high
        // and low halves.
        inst("mulxl", fmt("RVM", [w(r32a), w(r32b), r(rm32), r(implicit(edx))]), vex(LZ)._f2()._0f38().w0().op(0xF6), _64b | compat | bmi2).custom(Visit),
        inst("mulxq", fmt("RVM", [w(r64a), w(r64b), r(rm64), r(implicit(rdx))]), vex(LZ)._f2()._0f38().w1().op(0xF6), _64b | bmi2).custom(Visit),
        // Vector instructions.
        inst("mulss", fmt("A", [rw(xmm1), r(xmm_m32)]), rex([0xF3, 0x0F, 0x59]).r(), _64b | compat | sse).alt(avx, "vmulss_b"),
        inst("mulsd", fmt("A", [rw(xmm1), r(xmm_m64)]), rex([0xF2, 0x0F, 0x59]).r(), _64b | compat | sse2).alt(avx, "vmulsd_b"),
        inst("mulps", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x0F, 0x59]).r(), _64b | compat | sse).alt(avx, "vmulps_b"),
        inst("mulpd", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x59]).r(), _64b | compat | sse2).alt(avx, "vmulpd_b"),
        inst("pmuldq", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x38, 0x28]).r(), _64b | compat | sse41).alt(avx, "vpmuldq_b"),
        inst("pmulhrsw", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x38, 0x0B]).r(), _64b | compat | ssse3).alt(avx, "vpmulhrsw_b"),
        inst("pmulhuw", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0xE4]).r(), _64b | compat | sse2).alt(avx, "vpmulhuw_b"),
        inst("pmulhw", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0xE5]).r(), _64b | compat | sse2).alt(avx, "vpmulhw_b"),
        inst("pmulld", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x38, 0x40]).r(), _64b | compat | sse41).alt(avx, "vpmulld_b"),
        inst("pmullw", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0xD5]).r(), _64b | compat | sse2).alt(avx, "vpmullw_b"),
        inst("pmuludq", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0xF4]).r(), _64b | compat | sse2).alt(avx, "vpmuludq_b"),
        inst("vmulss", fmt("B", [w(xmm1), r(xmm2), r(xmm_m32)]), vex(LIG)._f3()._0f().op(0x59), _64b | compat | avx),
        inst("vmulsd", fmt("B", [w(xmm1), r(xmm2), r(xmm_m64)]), vex(LIG)._f2()._0f().op(0x59), _64b | compat | avx),
        inst("vmulps", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._0f().op(0x59), _64b | compat | avx),
        inst("vmulpd", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0x59), _64b | compat | avx),
        inst("vpmuldq", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f38().op(0x28), _64b | compat | avx),
        inst("vpmulhrsw", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f38().op(0x0B), _64b | compat | avx),
        inst("vpmulhuw", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0xE4), _64b | compat | avx),
        inst("vpmulhw", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0xE5), _64b | compat | avx),
        inst("vpmulld", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f38().op(0x40), _64b | compat | avx),
        inst("vpmullw", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0xD5), _64b | compat | avx),
        inst("vpmuludq", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0xF4), _64b | compat | avx),

        inst("vpmulld", fmt("C", [w(xmm1), r(xmm2), r(xmm_m128)]), evex(L128, Full)._66()._0f38().w0().op(0x40).r(), _64b | compat | avx512vl | avx512f),
        inst("vpmullq", fmt("C", [w(xmm1), r(xmm2), r(xmm_m128)]), evex(L128, Full)._66()._0f38().w1().op(0x40).r(), _64b | compat | avx512vl | avx512dq),
    ]
}
