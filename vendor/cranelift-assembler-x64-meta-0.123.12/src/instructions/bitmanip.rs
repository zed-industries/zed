use crate::dsl::{Eflags::*, Feature::*, Inst, Length::*, Location::*, TupleType::*};
use crate::dsl::{evex, fmt, implicit, inst, r, rex, rw, vex, w};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    vec![
        inst("bsfw", fmt("RM", [w(r16), r(rm16)]), rex([0x66, 0x0F, 0xBC]).r(), _64b | compat),
        inst("bsfl", fmt("RM", [w(r32), r(rm32)]), rex([0x0F, 0xBC]).r(), _64b | compat),
        inst("bsfq", fmt("RM", [w(r64), r(rm64)]), rex([0x0F, 0xBC]).r().w(), _64b),

        inst("bsrw", fmt("RM", [w(r16), r(rm16)]), rex([0x66, 0x0F, 0xBD]).r(), _64b | compat),
        inst("bsrl", fmt("RM", [w(r32), r(rm32)]), rex([0x0F, 0xBD]).r(), _64b | compat),
        inst("bsrq", fmt("RM", [w(r64), r(rm64)]), rex([0x0F, 0xBD]).r().w(), _64b),

        inst("tzcntw", fmt("A", [w(r16), r(rm16)]), rex([0x66, 0xF3, 0x0F, 0xBC]).r(), _64b | compat | bmi1),
        inst("tzcntl", fmt("A", [w(r32), r(rm32)]), rex([0xF3, 0x0F, 0xBC]).r(), _64b | compat | bmi1),
        inst("tzcntq", fmt("A", [w(r64), r(rm64)]), rex([0xF3, 0x0F, 0xBC]).r().w(), _64b | bmi1),

        inst("lzcntw", fmt("RM", [w(r16), r(rm16)]), rex([0x66, 0xF3, 0x0F, 0xBD]).r(), _64b | compat | lzcnt),
        inst("lzcntl", fmt("RM", [w(r32), r(rm32)]), rex([0xF3, 0x0F, 0xBD]).r(), _64b | compat | lzcnt),
        inst("lzcntq", fmt("RM", [w(r64), r(rm64)]), rex([0xF3, 0x0F, 0xBD]).r().w(), _64b | lzcnt),

        inst("popcntw", fmt("RM", [w(r16), r(rm16)]), rex([0x66, 0xF3, 0x0F, 0xB8]).r(), _64b | compat | popcnt),
        inst("popcntl", fmt("RM", [w(r32), r(rm32)]), rex([0xF3, 0x0F, 0xB8]).r(), _64b | compat | popcnt),
        inst("popcntq", fmt("RM", [w(r64), r(rm64)]), rex([0xF3, 0x0F, 0xB8]).r().w(), _64b | popcnt),

        inst("btw", fmt("MR", [r(rm16), r(r16)]).flags(W), rex([0x66, 0x0F, 0xA3]).r(), _64b | compat),
        inst("btl", fmt("MR", [r(rm32), r(r32)]).flags(W), rex([0x0F, 0xA3]).r(), _64b | compat),
        inst("btq", fmt("MR", [r(rm64), r(r64)]).flags(W), rex([0x0F, 0xA3]).w().r(), _64b),
        inst("btw", fmt("MI", [r(rm16), r(imm8)]).flags(W), rex([0x66, 0x0F, 0xBA]).digit(4).ib(), _64b | compat),
        inst("btl", fmt("MI", [r(rm32), r(imm8)]).flags(W), rex([0x0F, 0xBA]).digit(4).ib(), _64b | compat),
        inst("btq", fmt("MI", [r(rm64), r(imm8)]).flags(W), rex([0x0F, 0xBA]).w().digit(4).ib(), _64b),

        // Note that the Intel manual calls has different names for these
        // instructions than Capstone gives them:
        //
        // * cbtw => cbw
        // * cwtl => cwde
        // * cltq => cwqe
        // * cwtd => cwd
        // * cltd => cdq
        // * cqto => cqo
        inst("cbtw", fmt("ZO", [rw(implicit(ax))]), rex([0x66, 0x98]), _64b | compat),
        inst("cwtl", fmt("ZO", [rw(implicit(eax))]), rex([0x98]), _64b | compat),
        inst("cltq", fmt("ZO", [rw(implicit(rax))]), rex([0x98]).w(), _64b),
        inst("cwtd", fmt("ZO", [w(implicit(dx)), r(implicit(ax))]), rex([0x66, 0x99]), _64b | compat),
        inst("cltd", fmt("ZO", [w(implicit(edx)), r(implicit(eax))]), rex([0x99]), _64b | compat),
        inst("cqto", fmt("ZO", [w(implicit(rdx)), r(implicit(rax))]), rex([0x99]).w(), _64b),

        inst("bswapl", fmt("O", [rw(r32)]), rex([0x0F, 0xC8]).rd(), _64b | compat),
        inst("bswapq", fmt("O", [rw(r64)]), rex([0x0F, 0xC8]).w().ro(), _64b),

        // BMI1 instructions
        inst("blsrl", fmt("VM", [w(r32), r(rm32)]), vex(LZ)._0f38().w0().op(0xF3).digit(1), _64b | compat | bmi1),
        inst("blsrq", fmt("VM", [w(r64), r(rm64)]), vex(LZ)._0f38().w1().op(0xF3).digit(1), _64b | bmi1),
        inst("blsmskl", fmt("VM", [w(r32), r(rm32)]), vex(LZ)._0f38().w0().op(0xF3).digit(2), _64b | compat | bmi1),
        inst("blsmskq", fmt("VM", [w(r64), r(rm64)]), vex(LZ)._0f38().w1().op(0xF3).digit(2), _64b | bmi1),
        inst("blsil", fmt("VM", [w(r32), r(rm32)]), vex(LZ)._0f38().w0().op(0xF3).digit(3), _64b | compat | bmi1),
        inst("blsiq", fmt("VM", [w(r64), r(rm64)]), vex(LZ)._0f38().w1().op(0xF3).digit(3), _64b | compat | bmi1),

        // BMI2 instructions
        inst("bzhil", fmt("RMV", [w(r32a), r(rm32), r(r32b)]), vex(LZ)._0f38().w0().op(0xF5), _64b | compat | bmi2),
        inst("bzhiq", fmt("RMV", [w(r64a), r(rm64), r(r64b)]), vex(LZ)._0f38().w1().op(0xF5), _64b | bmi2),

        inst("vpopcntb", fmt("A", [w(xmm1), r(xmm_m128)]), evex(L128, FullMem)._66()._0f38().w0().op(0x54).r(), _64b | compat | avx512vl | avx512bitalg),
        inst("vpopcntw", fmt("A", [w(xmm1), r(xmm_m128)]), evex(L128, FullMem)._66()._0f38().w1().op(0x54).r(), _64b | compat | avx512vl | avx512bitalg),
        // FIXME: uncomment when avx512vpopcntdq is bound in cranelift
        // inst("vpopcntd", fmt("A", [w(xmm1), r(xmm_m128)]), evex(L128, Full)._66()._0f38().w0().op(0x55).r(), _64b | compat | avx512vl | avx512vpopcntdq),
        // inst("vpopcntq", fmt("A", [w(xmm1), r(xmm_m128)]), evex(L128, Full)._66()._0f38().w1().op(0x55).r(), _64b | compat | avx512vl | avx512vpopcntdq),
    ]
}
