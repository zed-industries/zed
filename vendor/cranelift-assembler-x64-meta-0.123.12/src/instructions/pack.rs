use crate::dsl::{Feature::*, Inst, Length::*, Location::*};
use crate::dsl::{align, fmt, inst, r, rex, rw, vex, w};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    vec![
        // Convert packed signed integers into smaller signed integers using
        // saturation to handle overflow (e.g., `0x7F` or `0x80` when converting
        // from word to byte).
        inst("packsswb", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x63]), _64b | compat | sse2).alt(avx, "vpacksswb_b"),
        inst("packssdw", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x6B]), _64b | compat | sse2).alt(avx, "vpackssdw_b"),
        inst("vpacksswb", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0x63), _64b | compat | avx),
        inst("vpackssdw", fmt("B", [w(xmm1),  r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0x6B), _64b | compat | avx),
        // Convert packed signed integers into smaller unsigned integers using
        // unsigned saturation to handle overflow (e.g., `0xFF` or `0x00` when
        // converting from word to byte).
        inst("packuswb", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x67]), _64b | compat | sse2).alt(avx, "vpackuswb_b"),
        inst("packusdw", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x38, 0x2B]), _64b | compat | sse41).alt(avx, "vpackusdw_b"),
        inst("vpackuswb", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0x67), _64b | compat | avx),
        inst("vpackusdw", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f38().op(0x2B), _64b | compat | avx),
    ]
}
