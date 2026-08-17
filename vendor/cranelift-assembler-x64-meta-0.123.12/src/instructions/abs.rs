use crate::dsl::{Feature::*, Inst, Length::*, Location::*, TupleType::*};
use crate::dsl::{align, evex, fmt, inst, r, rex, vex, w};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    vec![
        inst("pabsb", fmt("A", [w(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x38, 0x1C]), _64b | compat | ssse3).alt(avx, "vpabsb_a"),
        inst("vpabsb", fmt("A", [w(xmm1), r(xmm_m128)]), vex(L128)._66()._0f38().op(0x1C), _64b | compat | avx),
        // FIXME: uncomment once the avx512bw feature is bound
        // inst("vpabsb", fmt("B", [w(xmm1), r(xmm_m128)]), evex(L128, FullMem)._66()._0f38().wig().op(0x1C).r(), _64b | compat | avx512vl | avx512bw),
        inst("pabsw", fmt("A", [w(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x38, 0x1D]), _64b | compat | ssse3).alt(avx, "vpabsw_a"),
        inst("vpabsw", fmt("A", [w(xmm1), r(xmm_m128)]), vex(L128)._66()._0f38().op(0x1D), _64b | compat | avx),
        // FIXME: uncomment once the avx512bw feature is bound
        // inst("vpabsw", fmt("B", [w(xmm1), r(xmm_m128)]), evex(L128, FullMem)._66()._0f38().wig().op(0x1D).r(), _64b | compat | avx512vl | avx512bw),
        inst("pabsd", fmt("A", [w(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x38, 0x1E]), _64b | compat | ssse3).alt(avx, "vpabsd_a"),
        inst("vpabsd", fmt("A", [w(xmm1), r(xmm_m128)]), vex(L128)._66()._0f38().op(0x1E), _64b | compat | avx),
        inst("vpabsd", fmt("C", [w(xmm1), r(xmm_m128)]), evex(L128, Full)._66()._0f38().w0().op(0x1E).r(), _64b | compat | avx512vl | avx512f),
        inst("vpabsq", fmt("C", [w(xmm1), r(xmm_m128)]), evex(L128, Full)._66()._0f38().w1().op(0x1F).r(), _64b | compat | avx512vl | avx512f),
    ]
}
