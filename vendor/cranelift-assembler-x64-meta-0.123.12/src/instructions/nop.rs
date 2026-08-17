use crate::dsl::{Customization::*, Feature::*, Inst, Location::*};
use crate::dsl::{fmt, inst, r, rex};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    vec![
        // Provide the manual-defined versions of `NOP`, though we skip the
        // `rm16` format since it has the same encoding as `rm32`.
        inst("nop", fmt("ZO", []), rex(0x90), _64b | compat),
        inst("nopl", fmt("M", [r(rm32)]), rex([0x0F, 0x1F]).digit(0), _64b | compat),
        // Though the manual specifies limited encodings of `NOP` (above), it
        // recommends specific multi-byte sequenced got emitting `NOP`s between
        // 2 and 9 bytes long. The following "helper" instructions emit those
        // recommended sequences using custom functions.
        inst("nop", fmt("1B", []), rex(0x90), _64b | compat).custom(Encode | Display),
        inst("nop", fmt("2B", []), rex([0x66, 0x90]), _64b | compat).custom(Encode | Display),
        inst("nop", fmt("3B", []), rex([0x0F, 0x1F]), _64b | compat).custom(Encode | Display),
        inst("nop", fmt("4B", []), rex([0x0F, 0x1F]), _64b | compat).custom(Encode | Display),
        inst("nop", fmt("5B", []), rex([0x0F, 0x1F]), _64b | compat).custom(Encode | Display),
        inst("nop", fmt("6B", []), rex([0x66, 0x0F, 0x1F]), _64b | compat).custom(Encode | Display),
        inst("nop", fmt("7B", []), rex([0x0F, 0x1F]), _64b | compat).custom(Encode | Display),
        inst("nop", fmt("8B", []), rex([0x0F, 0x1F]), _64b | compat).custom(Encode | Display),
        inst("nop", fmt("9B", []), rex([0x66, 0x0F, 0x1F]), _64b | compat).custom(Encode | Display),
    ]
}
