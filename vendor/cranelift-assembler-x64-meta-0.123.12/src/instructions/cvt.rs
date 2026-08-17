use crate::dsl::{Customization::*, Feature::*, Inst, Length::*, Location::*, TupleType::*};
use crate::dsl::{align, evex, fmt, inst, r, rex, rw, vex, w};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    vec![
        // From 32-bit floating point.
        inst("cvtps2pd", fmt("A", [w(xmm1), r(xmm_m64)]), rex([0x0F, 0x5A]).r(), _64b | compat | sse2),
        inst("cvttps2dq", fmt("A", [w(xmm1), r(align(xmm_m128))]), rex([0xF3, 0x0F, 0x5B]).r(), _64b | compat | sse2),
        inst("cvtss2sd", fmt("A", [rw(xmm1), r(xmm_m32)]), rex([0xF3, 0x0F, 0x5A]).r(), _64b | compat | sse2),
        inst("cvtss2si", fmt("A", [w(r32), r(xmm_m32)]), rex([0xF3, 0x0F, 0x2D]).r(), _64b | compat | sse),
        inst("cvtss2si", fmt("AQ", [w(r64), r(xmm_m32)]), rex([0xF3, 0x0F, 0x2D]).w().r(), _64b | sse),
        inst("cvttss2si", fmt("A", [w(r32), r(xmm_m32)]), rex([0xF3, 0x0F, 0x2C]).r(), _64b | compat | sse),
        inst("cvttss2si", fmt("AQ", [w(r64), r(xmm_m32)]), rex([0xF3, 0x0F, 0x2C]).w().r(), _64b | sse),

        inst("vcvtps2pd", fmt("A", [w(xmm1), r(xmm_m64)]), vex(L128)._0f().op(0x5A).r(), _64b | compat | avx),
        inst("vcvttps2dq", fmt("A", [w(xmm1), r(xmm_m128)]), vex(L128)._f3()._0f().op(0x5B).r(), _64b | compat | avx),
        inst("vcvtss2sd", fmt("B", [w(xmm1), r(xmm2), r(xmm_m32)]), vex(LIG)._f3()._0f().op(0x5A).r(), _64b | compat | avx),
        inst("vcvtss2si", fmt("A", [w(r32), r(xmm_m32)]), vex(LIG)._f3()._0f().w0().op(0x2D).r(), _64b | compat | avx),
        inst("vcvtss2si", fmt("AQ", [w(r64), r(xmm_m32)]), vex(LIG)._f3()._0f().w1().op(0x2D).r(), _64b | avx),
        inst("vcvttss2si", fmt("A", [w(r32), r(xmm_m32)]), vex(LIG)._f3()._0f().w0().op(0x2C).r(), _64b | compat | avx),
        inst("vcvttss2si", fmt("AQ", [w(r64), r(xmm_m32)]), vex(LIG)._f3()._0f().w1().op(0x2C).r(), _64b | avx),

        // From 64-bit floating point.
        inst("cvtpd2ps", fmt("A", [w(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x5A]).r(), _64b | compat | sse2),
        inst("cvttpd2dq", fmt("A", [w(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0xE6]).r(), _64b | compat | sse2),
        inst("cvtsd2ss", fmt("A", [rw(xmm1), r(xmm_m64)]), rex([0xF2, 0x0F, 0x5A]).r(), _64b | compat | sse2),
        inst("cvtsd2si", fmt("A", [w(r32), r(xmm_m64)]), rex([0xF2, 0x0F, 0x2D]).r(), _64b | compat | sse2),
        inst("cvtsd2si", fmt("AQ", [w(r64), r(xmm_m64)]), rex([0xF2, 0x0F, 0x2D]).w().r(), _64b | sse2),
        inst("cvttsd2si", fmt("A", [w(r32), r(xmm_m64)]), rex([0xF2, 0x0F, 0x2C]).r(), _64b | compat | sse2),
        inst("cvttsd2si", fmt("AQ", [w(r64), r(xmm_m64)]), rex([0xF2, 0x0F, 0x2C]).w().r(), _64b | sse2),

        inst("vcvtpd2ps", fmt("A", [w(xmm1), r(xmm_m128)]), vex(L128)._66()._0f().op(0x5A).r(), _64b | compat | avx).custom(Mnemonic),
        inst("vcvttpd2dq", fmt("A", [w(xmm1), r(xmm_m128)]), vex(L128)._66()._0f().op(0xE6).r(), _64b | compat | avx).custom(Mnemonic),
        inst("vcvtsd2ss", fmt("B", [w(xmm1), r(xmm2), r(xmm_m64)]), vex(LIG)._f2()._0f().op(0x5A).r(), _64b | compat | avx),
        inst("vcvtsd2si", fmt("A", [w(r32), r(xmm_m64)]), vex(LIG)._f2()._0f().w0().op(0x2D).r(), _64b | compat | avx),
        inst("vcvtsd2si", fmt("AQ", [w(r64), r(xmm_m64)]), vex(LIG)._f2()._0f().w1().op(0x2D).r(), _64b | avx),
        inst("vcvttsd2si", fmt("A", [w(r32), r(xmm_m64)]), vex(LIG)._f2()._0f().w0().op(0x2C).r(), _64b | compat | avx),
        inst("vcvttsd2si", fmt("AQ", [w(r64), r(xmm_m64)]), vex(LIG)._f2()._0f().w1().op(0x2C).r(), _64b | avx),

        // From signed 32-bit integer.
        inst("cvtdq2ps", fmt("A", [w(xmm1), r(align(xmm_m128))]), rex([0x0F, 0x5B]).r(), _64b | compat | sse2),
        inst("cvtdq2pd", fmt("A", [w(xmm1), r(xmm_m64)]), rex([0xF3, 0x0F, 0xE6]).r(), _64b | compat | sse2),
        inst("cvtsi2ssl", fmt("A", [rw(xmm1), r(rm32)]), rex([0xF3, 0x0F, 0x2A]).r(), _64b | compat | sse),
        inst("cvtsi2ssq", fmt("A", [rw(xmm1), r(rm64)]), rex([0xF3, 0x0F, 0x2A]).w().r(), _64b | sse),
        inst("cvtsi2sdl", fmt("A", [rw(xmm1), r(rm32)]), rex([0xF2, 0x0F, 0x2A]).r(), _64b | compat | sse2),
        inst("cvtsi2sdq", fmt("A", [rw(xmm1), r(rm64)]), rex([0xF2, 0x0F, 0x2A]).w().r(), _64b | sse2),

        inst("vcvtdq2pd", fmt("A", [w(xmm1), r(xmm_m64)]), vex(L128)._f3()._0f().op(0xE6).r(), _64b | compat | avx),
        inst("vcvtdq2ps", fmt("A", [w(xmm1), r(xmm_m128)]), vex(L128)._0f().op(0x5B).r(), _64b | compat | avx),
        inst("vcvtsi2sdl", fmt("B", [w(xmm1), r(xmm2), r(rm32)]), vex(LIG)._f2()._0f().w0().op(0x2A).r(), _64b | compat | avx),
        inst("vcvtsi2sdq", fmt("B", [w(xmm1), r(xmm2), r(rm64)]), vex(LIG)._f2()._0f().w1().op(0x2A).r(), _64b | avx),
        inst("vcvtsi2ssl", fmt("B", [w(xmm1), r(xmm2), r(rm32)]), vex(LIG)._f3()._0f().w0().op(0x2A).r(), _64b | compat | avx),
        inst("vcvtsi2ssq", fmt("B", [w(xmm1), r(xmm2), r(rm64)]), vex(LIG)._f3()._0f().w1().op(0x2A).r(), _64b | avx),

        // Currently omitted as Cranelift doesn't need them but could be added
        // in the future:
        //
        // * cvtpd2dq
        // * cvtpd2pi
        // * cvtpi2pd
        // * cvtpi2ps
        // * cvtps2dq
        // * cvtps2pi
        // * cvttpd2pi
        // * cvttps2pi

        inst("vcvtudq2ps", fmt("A", [w(xmm1), r(xmm_m128)]), evex(L128, Full)._f2()._0f().w0().op(0x7A).r(), _64b | avx512vl | avx512f),
    ]
}
