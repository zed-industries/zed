use crate::dsl::{Feature::*, Inst, Length::*, Location::*, TupleType::*};
use crate::dsl::{align, evex, fmt, inst, r, rex, rw, vex, w};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    // Note that `p{extr,ins}r{w,b}` below operate on 32-bit registers but a
    // smaller-width memory location. This means that disassembly in Capstone
    // doesn't match `rm8`, for example. For now pretend both of these are
    // `rm32` to get disassembly matching Capstone.
    let r32m8 = rm32;
    let r32m16 = rm32;

    vec![
        // Extract from a single XMM lane.
        inst("extractps", fmt("A", [w(rm32), r(xmm1), r(imm8)]), rex([0x66, 0x0F, 0x3A, 0x17]).r().ib(), _64b | compat | sse41).alt(avx, "vextractps_b"),
        inst("pextrb", fmt("A", [w(r32m8), r(xmm2), r(imm8)]), rex([0x66, 0x0F, 0x3A, 0x14]).r().ib(), _64b | compat | sse41).alt(avx, "vpextrb_a"),
        inst("pextrw", fmt("A", [w(r32), r(xmm2), r(imm8)]), rex([0x66, 0x0F, 0xC5]).r().ib(), _64b | compat | sse2).alt(avx, "vpextrw_a"),
        inst("pextrw", fmt("B", [w(r32m16), r(xmm2), r(imm8)]), rex([0x66, 0x0F, 0x3A, 0x15]).r().ib(), _64b | compat | sse41).alt(avx, "vpextrw_b"),
        inst("pextrd", fmt("A", [w(rm32), r(xmm2), r(imm8)]), rex([0x66, 0x0F, 0x3A, 0x16]).r().ib(), _64b | compat | sse41).alt(avx, "vpextrd_a"),
        inst("pextrq", fmt("A", [w(rm64), r(xmm2), r(imm8)]), rex([0x66, 0x0F, 0x3A, 0x16]).w().r().ib(), _64b | sse41).alt(avx, "vpextrq_a"),
        inst("vextractps", fmt("B", [w(rm32), r(xmm1), r(imm8)]), vex(L128)._66()._0f3a().wig().op(0x17).r().ib(), _64b | compat | avx),
        inst("vpextrb", fmt("A", [w(r32m8), r(xmm2), r(imm8)]), vex(L128)._66()._0f3a().w0().op(0x14).r().ib(), _64b | compat | avx),
        inst("vpextrw", fmt("A", [w(r32), r(xmm2), r(imm8)]), vex(L128)._66()._0f().w0().op(0xC5).r().ib(), _64b | compat | avx),
        inst("vpextrw", fmt("B", [w(r32m16), r(xmm2), r(imm8)]), vex(L128)._66()._0f3a().w0().op(0x15).r().ib(), _64b | compat | avx),
        inst("vpextrd", fmt("A", [w(rm32), r(xmm2), r(imm8)]), vex(L128)._66()._0f3a().w0().op(0x16).r().ib(), _64b | compat | avx),
        inst("vpextrq", fmt("A", [w(rm64), r(xmm2), r(imm8)]), vex(L128)._66()._0f3a().w1().op(0x16).r().ib(), _64b | compat | avx),

        // Insert into a single XMM lane.
        inst("insertps", fmt("A", [rw(xmm1), r(xmm_m32), r(imm8)]), rex([0x66, 0x0F, 0x3A, 0x21]).r().ib(), _64b | compat | sse41).alt(avx, "vinsertps_b"),
        inst("pinsrb", fmt("A", [rw(xmm1), r(r32m8), r(imm8)]), rex([0x66, 0x0F, 0x3A, 0x20]).r().ib(), _64b | compat | sse41),
        inst("pinsrw", fmt("A", [rw(xmm1), r(r32m16), r(imm8)]), rex([0x66, 0x0F, 0xC4]).r().ib(), _64b | compat | sse2),
        inst("pinsrd", fmt("A", [rw(xmm1), r(rm32), r(imm8)]), rex([0x66, 0x0F, 0x3A, 0x22]).r().ib(), _64b | compat | sse41),
        inst("pinsrq", fmt("A", [rw(xmm1), r(rm64), r(imm8)]), rex([0x66, 0x0F, 0x3A, 0x22]).r().ib().w(), _64b | sse41),
        inst("vinsertps", fmt("B", [w(xmm1), r(xmm2), r(xmm_m32), r(imm8)]), vex(L128)._66()._0f3a().wig().op(0x21).r().ib(), _64b | compat | avx),
        inst("vpinsrb", fmt("B", [w(xmm1), r(xmm2), r(r32m8), r(imm8)]), vex(L128)._66()._0f3a().w0().op(0x20).r().ib(), _64b | compat | avx),
        inst("vpinsrw", fmt("B", [w(xmm1), r(xmm2), r(r32m16), r(imm8)]), vex(L128)._66()._0f().w0().op(0xC4).r().ib(), _64b | compat | avx),
        inst("vpinsrd", fmt("B", [w(xmm1), r(xmm2), r(rm32), r(imm8)]), vex(L128)._66()._0f3a().w0().op(0x22).r().ib(), _64b | compat | avx),
        inst("vpinsrq", fmt("B", [w(xmm1), r(xmm2), r(rm64), r(imm8)]), vex(L128)._66()._0f3a().w1().op(0x22).r().ib(), _64b | avx),

        // Extract sign masks from the floating-point lanes.
        inst("movmskps", fmt("RM", [w(r32), r(xmm2)]), rex([0x0F, 0x50]).r(), _64b | compat | sse).alt(avx, "vmovmskps_rm"),
        inst("movmskpd", fmt("RM", [w(r32), r(xmm2)]), rex([0x66, 0x0F, 0x50]).r(), _64b | compat | sse2).alt(avx, "vmovmskpd_rm"),
        inst("pmovmskb", fmt("RM", [w(r32), r(xmm2)]), rex([0x66, 0x0F, 0xD7]).r(), _64b | compat | sse2).alt(avx, "vpmovmskb_rm"),
        inst("vmovmskps", fmt("RM", [w(r32), r(xmm2)]), vex(L128)._0f().op(0x50).r(), _64b | compat | avx),
        inst("vmovmskpd", fmt("RM", [w(r32), r(xmm2)]), vex(L128)._66()._0f().op(0x50).r(), _64b | compat | avx),
        inst("vpmovmskb", fmt("RM", [w(r32), r(xmm2)]), vex(L128)._66()._0f().op(0xD7).r(), _64b | compat | avx),

        // Move two lower 32-bit floats to the high two lanes.
        inst("movhps", fmt("A", [rw(xmm1), r(m64)]), rex([0x0F, 0x16]).r(), _64b | compat | sse).alt(avx, "vmovhps_b"),
        inst("movlhps", fmt("RM", [rw(xmm1), r(xmm2)]), rex([0x0F, 0x16]).r(), _64b | compat | sse).alt(avx, "vmovlhps_rvm"),
        inst("vmovhps", fmt("B", [w(xmm2), r(xmm1), r(m64)]), vex(L128)._0f().op(0x16).r(), _64b | compat | avx),
        inst("vmovlhps", fmt("RVM", [w(xmm1), r(xmm2), r(xmm3)]), vex(L128)._0f().op(0x16).r(), _64b | compat | avx),

        // Duplicate the lower 64 bits of the source into 128 bits of the destination.
        inst("movddup", fmt("A", [w(xmm1), r(xmm_m64)]), rex([0xF2, 0x0F, 0x12]).r(), _64b | compat | sse3).alt(avx, "vmovddup_a"),
        inst("vmovddup", fmt("A", [w(xmm1), r(xmm_m64)]), vex(L128)._f2()._0f().op(0x12).r(), _64b | compat | avx),

        // Blend lanes in various ways.
        inst("pblendw", fmt("RMI", [rw(xmm1), r(align(xmm_m128)), r(imm8)]), rex([0x66, 0x0F, 0x3A, 0x0E]).r().ib(), _64b | compat | sse41).alt(avx, "vpblendw_rvmi"),
        inst("pblendvb", fmt("RM", [rw(xmm1), r(align(xmm_m128)), r(xmm0)]), rex([0x66, 0x0F, 0x38, 0x10]).r(), _64b | compat | sse41),
        inst("blendvps", fmt("RM0", [rw(xmm1), r(align(xmm_m128)), r(xmm0)]), rex([0x66, 0x0F, 0x38, 0x14]).r(), _64b | compat | sse41),
        inst("blendvpd", fmt("RM0", [rw(xmm1), r(align(xmm_m128)), r(xmm0)]), rex([0x66, 0x0F, 0x38, 0x15]).r(), _64b | compat | sse41),
        inst("vpblendw", fmt("RVMI", [w(xmm1), r(xmm2), r(xmm_m128), r(imm8)]), vex(L128)._66()._0f3a().w0().op(0x0E).r().ib(), _64b | compat | avx),
        inst("vpblendvb", fmt("RVMR", [w(xmm1), r(xmm2), r(xmm_m128), r(xmm3)]), vex(L128)._66()._0f3a().w0().op(0x4C).r().is4(), _64b | compat | avx),
        inst("vblendvps", fmt("RVMR", [w(xmm1), r(xmm2), r(xmm_m128), r(xmm3)]), vex(L128)._66()._0f3a().w0().op(0x4A).r().is4(), _64b | compat | avx),
        inst("vblendvpd", fmt("RVMR", [w(xmm1), r(xmm2), r(xmm_m128), r(xmm3)]), vex(L128)._66()._0f3a().w0().op(0x4B).r().is4(), _64b | compat | avx),

        // Shuffle lanes in various ways.
        inst("shufpd", fmt("A", [rw(xmm1), r(align(xmm_m128)), r(imm8)]), rex([0x66, 0x0F, 0xC6]).ib(), _64b | compat | sse2).alt(avx, "vshufpd_b"),
        inst("vshufpd", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128), r(imm8)]), vex(L128)._66()._0f().ib().op(0xC6), _64b | compat | avx),
        inst("shufps", fmt("A", [rw(xmm1), r(align(xmm_m128)), r(imm8)]), rex([0x0F, 0xC6]).ib(), _64b | compat | sse).alt(avx, "vshufps_b"),
        inst("vshufps", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128), r(imm8)]), vex(L128)._0f().ib().op(0xC6), _64b | compat | avx),
        inst("pshufb", fmt("A", [rw(xmm1), r(align(xmm_m128))]), rex([0x66, 0x0F, 0x38, 0x00]), _64b | compat | ssse3).alt(avx, "vpshufb_b"),
        inst("pshufd", fmt("A", [w(xmm1), r(align(xmm_m128)), r(imm8)]), rex([0x66, 0x0F, 0x70]).r().ib(), _64b | compat | sse2).alt(avx, "vpshufd_a"),
        inst("pshuflw", fmt("A", [w(xmm1), r(align(xmm_m128)), r(imm8)]), rex([0xF2, 0x0F, 0x70]).r().ib(), _64b | compat | sse2).alt(avx, "vpshuflw_a"),
        inst("pshufhw", fmt("A", [w(xmm1), r(align(xmm_m128)), r(imm8)]), rex([0xF3, 0x0F, 0x70]).r().ib(), _64b | compat | sse2).alt(avx, "vpshufhw_a"),
        inst("vpshufb", fmt("B", [w(xmm1), r(xmm2), r(xmm_m128)]), vex(L128)._66()._0f38().op(0x00), _64b | compat | avx),
        inst("vpshufd", fmt("A", [w(xmm1), r(xmm_m128), r(imm8)]), vex(L128)._66()._0f().op(0x70).r().ib(), _64b | compat | avx),
        inst("vpshuflw", fmt("A", [w(xmm1), r(xmm_m128), r(imm8)]), vex(L128)._f2()._0f().op(0x70).r().ib(), _64b | compat | avx),
        inst("vpshufhw", fmt("A", [w(xmm1), r(xmm_m128), r(imm8)]), vex(L128)._f3()._0f().op(0x70).r().ib(), _64b | compat | avx),

        // Broadcast a single lane to all lanes of the destination.
        inst("vbroadcastss", fmt("A_M", [w(xmm1), r(m32)]), vex(L128)._66()._0f38().w0().op(0x18).r(), _64b | compat | avx),
        inst("vbroadcastss", fmt("A_R", [w(xmm1), r(xmm2)]), vex(L128)._66()._0f38().w0().op(0x18).r(), _64b | compat | avx2),
        inst("vpbroadcastb", fmt("A", [w(xmm1), r(xmm_m8)]), vex(L128)._66()._0f38().w0().op(0x78).r(), _64b | compat | avx2),
        inst("vpbroadcastw", fmt("A", [w(xmm1), r(xmm_m16)]), vex(L128)._66()._0f38().w0().op(0x79).r(), _64b | compat | avx2),
        inst("vpbroadcastd", fmt("A", [w(xmm1), r(xmm_m32)]), vex(L128)._66()._0f38().w0().op(0x58).r(), _64b | compat | avx2),
        inst("vpbroadcastq", fmt("A", [w(xmm1), r(xmm_m64)]), vex(L128)._66()._0f38().w0().op(0x59).r(), _64b | compat | avx2),

        // AVX-512 permutations
        inst("vpermi2b", fmt("A", [rw(xmm1), r(xmm2), r(xmm_m128)]), evex(L128, FullMem)._66()._0f38().w0().op(0x75).r(), _64b | compat | avx512vl | avx512vbmi),
    ]
}
