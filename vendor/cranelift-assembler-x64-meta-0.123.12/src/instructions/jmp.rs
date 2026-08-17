use crate::dsl::{Customization::*, Feature::*, Inst, Location::*};
use crate::dsl::{fmt, inst, r, rex, sxq};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    vec![
        inst("jmpq", fmt("M", [r(rm64)]), rex([0xFF]).digit(4), _64b).custom(Display),

        inst("jmp", fmt("D8", [r(sxq(imm8))]), rex([0xEB]).ib(), _64b | compat).custom(Display),
        inst("jmp", fmt("D32", [r(sxq(imm32))]), rex([0xE9]).id(), _64b | compat).custom(Display),

        // Note that the Intel manual lists many mnemonics for this family of
        // instructions which are duplicates of other mnemonics. The order here
        // matches the order in the manual and comments are left when variants
        // are omitted due to the instructions being duplicates of another.
        inst("ja", fmt("D8", [r(sxq(imm8))]), rex([0x77]).ib(), _64b | compat).custom(Display),
        inst("ja", fmt("D32", [r(sxq(imm32))]), rex([0x0F, 0x87]).id(), _64b | compat).custom(Display),
        inst("jae", fmt("D8", [r(sxq(imm8))]), rex([0x73]).ib(), _64b | compat).custom(Display),
        inst("jae", fmt("D32", [r(sxq(imm32))]), rex([0x0F, 0x83]).id(), _64b | compat).custom(Display),
        inst("jb", fmt("D8", [r(sxq(imm8))]), rex([0x72]).ib(), _64b | compat).custom(Display),
        inst("jb", fmt("D32", [r(sxq(imm32))]), rex([0x0F, 0x82]).id(), _64b | compat).custom(Display),
        inst("jbe", fmt("D8", [r(sxq(imm8))]), rex([0x76]).ib(), _64b | compat).custom(Display),
        inst("jbe", fmt("D32", [r(sxq(imm32))]), rex([0x0F, 0x86]).id(), _64b | compat).custom(Display),
        // jc == jb
        // TODO: jcx
        // TODO: jecx
        // TODO: jrcx
        inst("je", fmt("D8", [r(sxq(imm8))]), rex([0x74]).ib(), _64b | compat).custom(Display),
        inst("je", fmt("D32", [r(sxq(imm32))]), rex([0x0F, 0x84]).id(), _64b | compat).custom(Display),
        inst("jg", fmt("D8", [r(sxq(imm8))]), rex([0x7F]).ib(), _64b | compat).custom(Display),
        inst("jg", fmt("D32", [r(sxq(imm32))]), rex([0x0F, 0x8F]).id(), _64b | compat).custom(Display),
        inst("jge", fmt("D8", [r(sxq(imm8))]), rex([0x7D]).ib(), _64b | compat).custom(Display),
        inst("jge", fmt("D32", [r(sxq(imm32))]), rex([0x0F, 0x8D]).id(), _64b | compat).custom(Display),
        inst("jl", fmt("D8", [r(sxq(imm8))]), rex([0x7C]).ib(), _64b | compat).custom(Display),
        inst("jl", fmt("D32", [r(sxq(imm32))]), rex([0x0F, 0x8C]).id(), _64b | compat).custom(Display),
        inst("jle", fmt("D8", [r(sxq(imm8))]), rex([0x7E]).ib(), _64b | compat).custom(Display),
        inst("jle", fmt("D32", [r(sxq(imm32))]), rex([0x0F, 0x8E]).id(), _64b | compat).custom(Display),
        // jna == jbe
        // jnae == jb
        // jnb == jae
        // jnbe == ja
        // jnc == jae
        inst("jne", fmt("D8", [r(sxq(imm8))]), rex([0x75]).ib(), _64b | compat).custom(Display),
        inst("jne", fmt("D32", [r(sxq(imm32))]), rex([0x0F, 0x85]).id(), _64b | compat).custom(Display),
        // jng == jle
        // jnge == jl
        // jnl == jge
        // jnle == jg
        inst("jno", fmt("D8", [r(sxq(imm8))]), rex([0x71]).ib(), _64b | compat).custom(Display),
        inst("jno", fmt("D32", [r(sxq(imm32))]), rex([0x0F, 0x81]).id(), _64b | compat).custom(Display),
        inst("jnp", fmt("D8", [r(sxq(imm8))]), rex([0x7B]).ib(), _64b | compat).custom(Display),
        inst("jnp", fmt("D32", [r(sxq(imm32))]), rex([0x0F, 0x8B]).id(), _64b | compat).custom(Display),
        inst("jns", fmt("D8", [r(sxq(imm8))]), rex([0x79]).ib(), _64b | compat).custom(Display),
        inst("jns", fmt("D32", [r(sxq(imm32))]), rex([0x0F, 0x89]).id(), _64b | compat).custom(Display),
        // jnz == jne
        inst("jo", fmt("D8", [r(sxq(imm8))]), rex([0x70]).ib(), _64b | compat).custom(Display),
        inst("jo", fmt("D32", [r(sxq(imm32))]), rex([0x0F, 0x80]).id(), _64b | compat).custom(Display),
        inst("jp", fmt("D8", [r(sxq(imm8))]), rex([0x7A]).ib(), _64b | compat).custom(Display),
        inst("jp", fmt("D32", [r(sxq(imm32))]), rex([0x0F, 0x8A]).id(), _64b | compat).custom(Display),
        // jpe == jp
        // jpo == jnp
        inst("js", fmt("D8", [r(sxq(imm8))]), rex([0x78]).ib(), _64b | compat).custom(Display),
        inst("js", fmt("D32", [r(sxq(imm32))]), rex([0x0F, 0x88]).id(), _64b | compat).custom(Display),
        // jz == je
    ]
}
