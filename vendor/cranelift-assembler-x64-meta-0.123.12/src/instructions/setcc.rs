use crate::dsl::{Eflags::*, Feature::*, Inst, Location::*};
use crate::dsl::{fmt, inst, rex, w};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    vec![
        // Note that the Intel manual lists many mnemonics for this family of
        // instructions which are duplicates of other mnemonics. The order here
        // matches the order in the manual and comments are left when variants
        // are omitted due to the instructions being duplicates of another.
        //
        // Also note that the `digit(0)` annotation here is not mentioned in the
        // manual's description for the encoding of these instructions. This is
        // due to:
        //
        // > The reg field of the ModR/M byte is not used for the SETCC
        // > instruction and those opcode bits are ignored by the processor.
        //
        // Here 0 is used in the reg field to match what other assemblers look
        // like they're doing of setting the reg bits to zero.
        inst("seta", fmt("M", [w(rm8)]).flags(R), rex([0x0f, 0x97]).digit(0), _64b | compat),
        inst("setae", fmt("M", [w(rm8)]).flags(R), rex([0x0f, 0x93]).digit(0), _64b | compat),
        inst("setb", fmt("M", [w(rm8)]).flags(R), rex([0x0f, 0x92]).digit(0), _64b | compat),
        inst("setbe", fmt("M", [w(rm8)]).flags(R), rex([0x0f, 0x96]).digit(0), _64b | compat),
        // NB: setc* is omitted here as it has the same encoding as setb*
        inst("sete", fmt("M", [w(rm8)]).flags(R), rex([0x0f, 0x94]).digit(0), _64b | compat),
        inst("setg", fmt("M", [w(rm8)]).flags(R), rex([0x0f, 0x9f]).digit(0), _64b | compat),
        inst("setge", fmt("M", [w(rm8)]).flags(R), rex([0x0f, 0x9d]).digit(0), _64b | compat),
        inst("setl", fmt("M", [w(rm8)]).flags(R), rex([0x0f, 0x9c]).digit(0), _64b | compat),
        inst("setle", fmt("M", [w(rm8)]).flags(R), rex([0x0f, 0x9e]).digit(0), _64b | compat),
        // NB: setna* is omitted here as it has the same encoding as setbe*
        // NB: setnae* is omitted here as it has the same encoding as setb*
        // NB: setnb* is omitted here as it has the same encoding as setae*
        // NB: setnbe* is omitted here as it has the same encoding as seta*
        // NB: setnc* is omitted here as it has the same encoding as setae*
        inst("setne", fmt("M", [w(rm8)]).flags(R), rex([0x0f, 0x95]).digit(0), _64b | compat),
        // NB: setng* is omitted here as it has the same encoding as setle*
        // NB: setnge* is omitted here as it has the same encoding as setl*
        // NB: setnl* is omitted here as it has the same encoding as setge*
        // NB: setnle* is omitted here as it has the same encoding as setg*
        inst("setno", fmt("M", [w(rm8)]).flags(R), rex([0x0f, 0x91]).digit(0), _64b | compat),
        inst("setnp", fmt("M", [w(rm8)]).flags(R), rex([0x0f, 0x9b]).digit(0), _64b | compat),
        inst("setns", fmt("M", [w(rm8)]).flags(R), rex([0x0f, 0x99]).digit(0), _64b | compat),
        // NB: setnz* is omitted here as it has the same encoding as setne*
        inst("seto", fmt("M", [w(rm8)]).flags(R), rex([0x0f, 0x90]).digit(0), _64b | compat),
        inst("setp", fmt("M", [w(rm8)]).flags(R), rex([0x0f, 0x9a]).digit(0), _64b | compat),
        // NB: setpe* is omitted here as it has the same encoding as setp*
        // NB: setpo* is omitted here as it has the same encoding as setnp*
        inst("sets", fmt("M", [w(rm8)]).flags(R), rex([0x0f, 0x98]).digit(0), _64b | compat),
        // NB: setz* is omitted here as it has the same encoding as sete*
    ]
}
