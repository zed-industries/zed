use crate::dsl::{Feature::*, Inst, Location::*};
use crate::dsl::{fmt, inst, r, rex, sxq, w};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    vec![
        inst("popw", fmt("M", [w(rm16)]), rex([0x66, 0x8F]).digit(0), _64b | compat),
        inst("popq", fmt("M", [w(rm64)]), rex(0x8F).digit(0), _64b),
        inst("popw", fmt("O", [w(r16)]), rex([0x66, 0x58]).rw(), _64b | compat),
        inst("popq", fmt("O", [w(r64)]), rex(0x58).ro(), _64b),
        inst("pushw", fmt("M", [r(rm16)]), rex([0x66, 0xFF]).digit(6), _64b | compat),
        inst("pushq", fmt("M", [r(rm64)]), rex(0xFF).digit(6), _64b),
        inst("pushw", fmt("O", [r(r16)]), rex([0x66, 0x50]).rw(), _64b | compat),
        inst("pushq", fmt("O", [r(r64)]), rex(0x50).ro(), _64b),
        inst("pushq", fmt("I8", [r(sxq(imm8))]), rex(0x6A).ib(), _64b | compat),
        inst("pushw", fmt("I16", [r(imm16)]), rex([0x66, 0x68]).iw(), _64b | compat),
        inst("pushq", fmt("I32", [r(sxq(imm32))]), rex(0x68).id(), _64b | compat),
    ]
}
