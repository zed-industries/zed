use crate::dsl::{Feature::*, Inst, Length::*, Location::*};
use crate::dsl::{align, fmt, inst, r, rex, rw, vex, w};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    vec![
        // Floating-point minimum. Note, this has some tricky NaN and sign bit
        // behavior; see `max.rs`.
        inst("minss", fmt("A", [rw(xmm1), r(xmm_m32)]), rex([0xF3, 0x0F, 0x5D]).r(), _64b | compat | sse).alt(avx, "vminss_b"),
        inst("minsd", fmt("A", [rw(xmm1), r(xmm_m64)]), rex([0xF2, 0x0F, 0x5D]).r(), _64b | compat | sse2).alt(avx, "vminsd_b"),
        inst("minps", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x0F, 0x5D]).r(), _64b | compat | sse).alt(avx, "vminps_b"),
        inst("minpd", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x5D]).r(), _64b | compat | sse2).alt(avx, "vminpd_b"),
        inst("vminss", fmt("B", [w(xmm1), r(xmm2), r(xmm_m32)]), vex(LIG)._f3()._0f().op(0x5D).r(), _64b | compat | avx),
        inst("vminsd", fmt("B", [w(xmm1), r(xmm2), r(xmm_m64)]), vex(LIG)._f2()._0f().op(0x5D).r(), _64b | compat | avx),
        inst("vminps", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._0f().op(0x5D).r(), _64b | compat | avx),
        inst("vminpd", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0x5D).r(), _64b | compat | avx),
        // Packed integer minimum.
        inst("pminsb", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x38, 0x38]).r(), _64b | compat | sse41).alt(avx, "vpminsb_b"),
        inst("pminsw", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0xEA]).r(), _64b | compat | sse2).alt(avx, "vpminsw_b"),
        inst("pminsd", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x38, 0x39]).r(), _64b | compat | sse41).alt(avx, "vpminsd_b"),
        inst("pminub", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0xDA]).r(), _64b | compat | sse2).alt(avx, "vpminub_b"),
        inst("pminuw", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x38, 0x3A]).r(), _64b | compat | sse41).alt(avx, "vpminuw_b"),
        inst("pminud", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x38, 0x3B]).r(), _64b | compat | sse41).alt(avx, "vpminud_b"),
        inst("vpminsb", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f38().op(0x38).r(), _64b | compat | avx),
        inst("vpminsw", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0xEA).r(), _64b | compat | avx),
        inst("vpminsd", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f38().op(0x39).r(), _64b | compat | avx),
        inst("vpminub", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0xDA).r(), _64b | compat | avx),
        inst("vpminuw", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f38().op(0x3A).r(), _64b | compat | avx),
        inst("vpminud", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f38().op(0x3B).r(), _64b | compat | avx),
    ]
}
