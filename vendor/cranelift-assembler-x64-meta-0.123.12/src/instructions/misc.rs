use crate::dsl::{Customization::*, Feature::*, Inst, Location::*};
use crate::dsl::{fmt, inst, r, rex, sxl, w};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    vec![
        inst("mfence", fmt("ZO", []), rex([0x0f, 0xae, 0xf0]), _64b | compat | sse2),
        inst("sfence", fmt("ZO", []), rex([0x0f, 0xae, 0xf8]), _64b | compat),
        inst("lfence", fmt("ZO", []), rex([0x0f, 0xae, 0xe8]), _64b | compat | sse2),

        inst("hlt", fmt("ZO", []), rex([0xf4]), _64b | compat),
        inst("ud2", fmt("ZO", []), rex([0x0f, 0x0b]), _64b | compat).has_trap(),
        inst("int3", fmt("ZO", []), rex([0xcc]), _64b | compat),

        inst("retq", fmt("ZO", []), rex([0xC3]), _64b | compat),
        inst("retq", fmt("I", [r(imm16)]), rex([0xC2]).iw(), _64b | compat),

        inst("leaw", fmt("RM", [w(r16), r(m16)]), rex([0x66, 0x8D]).r(), _64b | compat),
        inst("leal", fmt("RM", [w(r32), r(m32)]), rex([0x8D]).r(), _64b | compat),
        inst("leaq", fmt("RM", [w(r64), r(m64)]), rex([0x8D]).w().r(), _64b),

        inst("callq", fmt("D", [r(sxl(imm32))]), rex([0xE8]).id(), _64b | compat).custom(Display),
        inst("callq", fmt("M", [r(rm64)]), rex([0xFF]).digit(2), _64b).custom(Display),
    ]
}
