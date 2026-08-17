use crate::dsl::{Feature::*, Inst, Length::*, Location::*};
use crate::dsl::{align, fmt, inst, r, rex, rw, vex, w};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    vec![
        // Unpack floating-point.
        inst("unpcklps", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x0F, 0x14]).r(), _64b | compat | sse).alt(avx, "vunpcklps_b"),
        inst("unpcklpd", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x14]).r(), _64b | compat | sse2).alt(avx, "vunpcklpd_b"),
        inst("unpckhps", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x0F, 0x15]).r(), _64b | compat | sse).alt(avx, "vunpckhps_b"),
        inst("vunpcklps", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._0f().op(0x14).r(), _64b | compat | avx),
        inst("vunpcklpd", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0x14).r(), _64b | compat | avx),
        inst("vunpckhps", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._0f().op(0x15).r(), _64b | compat | avx),
        // Unpack packed integers.
        inst("punpckhbw", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x68]).r(), _64b | compat | sse2).alt(avx, "vpunpckhbw_b"),
        inst("punpckhwd", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x69]).r(), _64b | compat | sse2).alt(avx, "vpunpckhwd_b"),
        inst("punpckhdq", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x6A]).r(), _64b | compat | sse2).alt(avx, "vpunpckhdq_b"),
        inst("punpckhqdq", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x6D]).r(), _64b | compat | sse2).alt(avx, "vpunpckhqdq_b"),
        inst("punpcklwd", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x61]).r(), _64b | compat | sse2).alt(avx, "vpunpcklwd_b"),
        inst("punpcklbw", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x60]).r(), _64b | compat | sse2).alt(avx, "vpunpcklbw_b"),
        inst("punpckldq", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x62]).r(), _64b | compat | sse2).alt(avx, "vpunpckldq_b"),
        inst("punpcklqdq", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x6C]).r(), _64b | compat | sse2).alt(avx, "vpunpcklqdq_b"),
        inst("vpunpckhbw", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0x68).r(), _64b | compat | avx),
        inst("vpunpckhwd", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0x69).r(), _64b | compat | avx),
        inst("vpunpckhdq", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0x6A).r(), _64b | compat | avx),
        inst("vpunpckhqdq", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0x6D).r(), _64b | compat | avx),
        inst("vpunpcklwd", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0x61).r(), _64b | compat | avx),
        inst("vpunpcklbw", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0x60).r(), _64b | compat | avx),
        inst("vpunpckldq", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0x62).r(), _64b | compat | avx),
        inst("vpunpcklqdq", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0x6C).r(), _64b | compat | avx),
    ]
}
