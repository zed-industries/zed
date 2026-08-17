use crate::dsl::{Feature::*, Inst, Length::*, Location::*};
use crate::dsl::{align, fmt, inst, r, rex, rw, vex, w};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    vec![
        // Floating-point maximum. Note, from the manual: "if the values being
        // compared are both 0.0s (of either sign), the value in the second
        // operand (source operand) is returned. If a value in the second
        // operand is an SNaN, then SNaN is forwarded unchanged to the
        // destination (that is, a QNaN version of the SNaN is not returned). If
        // only one value is a NaN (SNaN or QNaN) for this instruction, the
        // second operand (source operand), either a NaN or a valid
        // floating-point value, is written to the result. If instead of this
        // behavior, it is required that the NaN source operand (from either the
        // first or second operand) be returned, the action of MAXPS can be
        // emulated using a sequence of instructions, such as, a comparison
        // followed by AND, ANDN, and OR."
        inst("maxss", fmt("A", [rw(xmm1), r(xmm_m32)]), rex([0xF3, 0x0F, 0x5F]).r(), _64b | compat | sse).alt(avx, "vmaxss_b"),
        inst("maxsd", fmt("A", [rw(xmm1), r(xmm_m64)]), rex([0xF2, 0x0F, 0x5F]).r(), _64b | compat | sse2).alt(avx, "vmaxsd_b"),
        inst("maxps", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x0F, 0x5F]).r(), _64b | compat | sse).alt(avx, "vmaxps_b"),
        inst("maxpd", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x5F]).r(), _64b | compat | sse2).alt(avx, "vmaxpd_b"),
        inst("vmaxss", fmt("B", [w(xmm1), r(xmm2), r(xmm_m32)]), vex(LIG)._f3()._0f().op(0x5F).r(), _64b | compat | avx),
        inst("vmaxsd", fmt("B", [w(xmm1), r(xmm2), r(xmm_m64)]), vex(LIG)._f2()._0f().op(0x5F).r(), _64b | compat | avx),
        inst("vmaxps", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._0f().op(0x5F).r(), _64b | compat | avx),
        inst("vmaxpd", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0x5F).r(), _64b | compat | avx),
        // Packed integer maximum.
        inst("pmaxsb", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x38, 0x3C]).r(), _64b | compat | sse41).alt(avx, "vpmaxsb_b"),
        inst("pmaxsw", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0xEE]).r(), _64b | compat | sse2).alt(avx, "vpmaxsw_b"),
        inst("pmaxsd", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x38, 0x3D]).r(), _64b | compat | sse41).alt(avx, "vpmaxsd_b"),
        inst("pmaxub", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0xDE]).r(), _64b | compat | sse2).alt(avx, "vpmaxub_b"),
        inst("pmaxuw", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x38, 0x3E]).r(), _64b | compat | sse41).alt(avx, "vpmaxuw_b"),
        inst("pmaxud", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x38, 0x3F]).r(), _64b | compat | sse41).alt(avx, "vpmaxud_b"),
        inst("vpmaxsb", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f38().op(0x3C).r(), _64b | compat | avx),
        inst("vpmaxsw", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0xEE).r(), _64b | compat | avx),
        inst("vpmaxsd", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f38().op(0x3D).r(), _64b | compat | avx),
        inst("vpmaxub", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0xDE).r(), _64b | compat | avx),
        inst("vpmaxuw", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f38().op(0x3E).r(), _64b | compat | avx),
        inst("vpmaxud", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f38().op(0x3F).r(), _64b | compat | avx),
    ]
}
