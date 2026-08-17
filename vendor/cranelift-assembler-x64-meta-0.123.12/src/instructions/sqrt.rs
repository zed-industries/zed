use crate::dsl::{Feature::*, Inst, Length::*, Location::*};
use crate::dsl::{align, fmt, inst, r, rex, rw, vex, w};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    vec![
        inst("sqrtss", fmt("A", [rw(xmm1), r(xmm_m32)]), rex([0xF3, 0x0F, 0x51]).r(), _64b | compat | sse).alt(avx, "vsqrtss_b"),
        inst("sqrtsd", fmt("A", [rw(xmm1), r(xmm_m64)]), rex([0xF2, 0x0F, 0x51]).r(), _64b | compat | sse2).alt(avx, "vsqrtsd_b"),
        inst("sqrtps", fmt("A", [w(xmm1), r(align(xmm_m128))]), rex([0x0F, 0x51]).r(), _64b | compat | sse).alt(avx, "vsqrtps_b"),
        inst("sqrtpd", fmt("A", [w(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x51]).r(), _64b | compat | sse2).alt(avx, "vsqrtpd_b"),
        inst("vsqrtss", fmt("B", [w(xmm1), r(xmm2), r(xmm_m32)]), vex(LIG)._f3()._0f().op(0x51).r(), _64b | compat | avx),
        inst("vsqrtsd", fmt("B", [w(xmm1), r(xmm2), r(xmm_m64)]), vex(LIG)._f2()._0f().op(0x51).r(), _64b | compat | avx),
        inst("vsqrtps", fmt("B", [w(xmm1), r(xmm_m128)]), vex(L128)._0f().op(0x51).r(), _64b | compat | avx),
        inst("vsqrtpd", fmt("B", [w(xmm1), r(xmm_m128)]), vex(L128)._66()._0f().op(0x51).r(), _64b | compat | avx),
    ]
}
