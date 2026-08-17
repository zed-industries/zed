use crate::dsl::{Feature::*, Inst, Length::*, Location::*};
use crate::dsl::{align, fmt, inst, r, rex, vex, w};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    vec![
        inst("roundpd", fmt("RMI", [w(xmm1), r(align(xmm_m128)), r(imm8)]), rex([0x66, 0x0F, 0x3A, 0x09]).ib(), _64b | compat | sse41),
        inst("roundps", fmt("RMI", [w(xmm1), r(align(xmm_m128)), r(imm8)]), rex([0x66, 0x0F, 0x3A, 0x08]).ib(), _64b | compat | sse41),
        inst("roundsd", fmt("RMI", [w(xmm1), r(xmm_m64), r(imm8)]), rex([0x66, 0x0F, 0x3A, 0x0B]).ib(), _64b | compat | sse41),
        inst("roundss", fmt("RMI", [w(xmm1), r(xmm_m32), r(imm8)]), rex([0x66, 0x0F, 0x3A, 0x0A]).ib(), _64b | compat | sse41),

        // AVX versions
        inst("vroundpd", fmt("RMI", [w(xmm1), r(xmm_m128), r(imm8)]), vex(L128)._66()._0f3a().ib().op(0x09), _64b | compat | avx),
        inst("vroundps", fmt("RMI", [w(xmm1), r(xmm_m128), r(imm8)]), vex(L128)._66()._0f3a().ib().op(0x08), _64b | compat | avx),
        inst("vroundsd", fmt("RVMI", [w(xmm1), r(xmm2), r(xmm_m64), r(imm8)]), vex(LIG)._66()._0f3a().ib().op(0x0b), _64b | compat | avx),
        inst("vroundss", fmt("RVMI", [w(xmm1), r(xmm2), r(xmm_m32), r(imm8)]), vex(LIG)._66()._0f3a().ib().op(0x0a), _64b | compat | avx),
    ]
}
