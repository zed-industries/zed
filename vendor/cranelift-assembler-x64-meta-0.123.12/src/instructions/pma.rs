use crate::dsl::{Feature::*, Inst, Length::*, Location::*};
use crate::dsl::{align, fmt, inst, r, rex, rw, vex, w};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    vec![
        // Packed multiply-add instructions; from the manual: "Multiplies the
        // individual signed words of the destination operand (first operand) by
        // the corresponding signed words of the source operand (second
        // operand), producing temporary signed, doubleword results. The
        // adjacent doubleword results are then summed and stored in the
        // destination operand. For example, the corresponding low-order words
        // (15-0) and (31-16) in the source and destination operands are
        // multiplied by one another and the doubleword results are added
        // together and stored in the low doubleword of the destination register
        // (31-0). The same operation is performed on the other pairs of
        // adjacent words."
        inst("pmaddwd", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0xF5]), _64b | compat | sse2).alt(avx, "vpmaddwd_b"),
        inst("vpmaddwd", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f().op(0xF5), _64b | compat | avx),
        // Like `pmaddwd`, but this "multiplies vertically each unsigned byte of
        // the destination operand (first operand) with the corresponding signed
        // byte of the source operand (second operand), producing intermediate
        // signed 16-bit integers. Each adjacent pair of signed words is added
        // and the saturated result is packed to the destination operand."
        inst("pmaddubsw", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x38, 0x04]), _64b | compat | ssse3).alt(avx, "vpmaddubsw_b"),
        inst("vpmaddubsw", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f38().op(0x04), _64b | compat | avx),
     ]
}
