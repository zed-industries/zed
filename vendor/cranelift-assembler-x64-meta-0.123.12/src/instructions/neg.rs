use crate::dsl::{Feature::*, Inst, Location::*};
use crate::dsl::{fmt, inst, rex, rw};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    vec![
        // Two's complement negation.
        inst("negb", fmt("M", [rw(rm8)]), rex(0xF6).digit(3), _64b | compat),
        inst("negw", fmt("M", [rw(rm16)]), rex([0x66, 0xF7]).digit(3), _64b | compat),
        inst("negl", fmt("M", [rw(rm32)]), rex(0xF7).digit(3), _64b | compat),
        inst("negq", fmt("M", [rw(rm64)]), rex(0xF7).w().digit(3), _64b),
        // One's complement negation.
        inst("notb", fmt("M", [rw(rm8)]), rex(0xF6).digit(2), _64b | compat),
        inst("notw", fmt("M", [rw(rm16)]), rex([0x66, 0xF7]).digit(2), _64b | compat),
        inst("notl", fmt("M", [rw(rm32)]), rex(0xF7).digit(2), _64b | compat),
        inst("notq", fmt("M", [rw(rm64)]), rex(0xF7).w().digit(2), _64b),
    ]
}
