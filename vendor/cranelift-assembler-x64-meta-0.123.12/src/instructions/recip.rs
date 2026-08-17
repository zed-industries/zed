use crate::dsl::{Feature::*, Inst, Length::*, Location::*};
use crate::dsl::{align, fmt, inst, r, rex, vex, w};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    vec![
        inst("rcpps", fmt("RM", [w(xmm1), r(align(xmm_m128))]), rex([0x0F, 0x53]).r(), _64b | compat | sse).alt(avx, "vrcpps_rm"),
        inst("rcpss", fmt("RM", [w(xmm1), r(xmm_m32)]), rex([0xF3, 0x0F, 0x53]).r(), _64b | compat | sse),
        inst("rsqrtps", fmt("RM", [w(xmm1), r(align(xmm_m128))]), rex([0x0F, 0x52]).r(), _64b | compat | sse).alt(avx, "vrsqrtps_rm"),
        inst("rsqrtss", fmt("RM", [w(xmm1), r(xmm_m32)]), rex([0xF3, 0x0F, 0x52]).r(), _64b | compat | sse),

        inst("vrcpps", fmt("RM", [w(xmm1), r(xmm_m128)]), vex(L128)._0f().op(0x53).r(), _64b | compat | avx),
        inst("vrcpss", fmt("RVM", [w(xmm1), r(xmm2), r(xmm_m32)]), vex(LIG)._f3()._0f().op(0x53).r(), _64b | compat | avx),
        inst("vrsqrtps", fmt("RM", [w(xmm1), r(xmm_m128)]), vex(L128)._0f().op(0x52).r(), _64b | compat | avx),
        inst("vrsqrtss", fmt("RVM", [w(xmm1), r(xmm2), r(xmm_m32)]), vex(LIG)._f3()._0f().op(0x52).r(), _64b | compat | avx),
       ]
}
