use crate::dsl::{Feature::*, Inst, Length::*, Location::*};
use crate::dsl::{align, fmt, inst, r, rex, rw, vex, w};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    vec![
        // Packed align right.
        inst("palignr", fmt("A", [rw(xmm1), r(align(xmm_m128)), r(imm8)]), rex([0x66, 0x0F, 0x3A, 0x0F]).ib(), _64b | compat | ssse3).alt(avx, "vpalignr_b"),
        inst("vpalignr", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128), r(imm8)]), vex(L128)._66()._0f3a().ib().op(0x0F), _64b | compat | avx),
    ]
}
