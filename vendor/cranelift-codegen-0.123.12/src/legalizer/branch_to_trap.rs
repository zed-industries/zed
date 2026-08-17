//! Rewrite branch-to-unconditional-trap into conditional trap instructions.
//!
//! Given this instruction:
//!
//! ```clif
//! brif v0, block1, block2
//! ```
//!
//! If we know that `block1` does nothing but immediately trap then we can
//! rewrite that `brif` into the following:
//!
//! ```clif
//! trapz v0, <trapcode>
//! jump block2
//! ```
//!
//! (And we can do the equivalent with `trapz` if `block2` immediately traps).
//!
//! This transformation allows for the conditional trap instructions to be GVN'd
//! and for our egraphs mid-end to generally better optimize the program. We
//! additionally have better codegen in our backends for `trapz` than branches
//! to unconditional traps.

use super::*;

#[derive(Default)]
pub struct BranchToTrap {
    /// The set of blocks that contain exactly one unconditional trap
    /// instruction.
    just_trap_blocks: EntitySet<ir::Block>,
}

impl BranchToTrap {
    /// Analyze the given block.
    ///
    /// The `block` must be terminated by a `trap` instruction.
    pub fn analyze_trapping_block(&mut self, func: &ir::Function, block: ir::Block) {
        if func.layout.block_contains_exactly_one_inst(block) {
            self.just_trap_blocks.insert(block);
        }
    }

    fn just_trap_block_code(&self, func: &ir::Function, block: ir::Block) -> ir::TrapCode {
        debug_assert!(self.just_trap_blocks.contains(block));
        debug_assert!(func.layout.block_contains_exactly_one_inst(block));
        let inst = func.layout.first_inst(block).unwrap();
        match func.dfg.insts[inst] {
            InstructionData::Trap { code, .. } => code,
            _ => unreachable!(),
        }
    }

    /// Process a `brif` instruction, potentially performing our rewrite.
    ///
    /// The `inst` must be a `brif` containing the given `arg` and `blocks`.
    pub fn process_brif(
        &self,
        func: &mut ir::Function,
        inst: ir::Inst,
        arg: ir::Value,
        blocks: [ir::BlockCall; 2],
    ) {
        let consequent = blocks[0].block(&func.dfg.value_lists);
        let alternative = blocks[1].block(&func.dfg.value_lists);

        if self.just_trap_blocks.contains(consequent) {
            let mut pos = FuncCursor::new(func);
            pos.use_srcloc(
                pos.func
                    .layout
                    .first_inst(consequent)
                    .expect("just-trap blocks have exactly one inst"),
            );
            pos.goto_inst(inst);

            let code = self.just_trap_block_code(pos.func, consequent);
            pos.ins().trapnz(arg, code);

            let args: SmallVec<[_; 8]> = blocks[1].args(&pos.func.dfg.value_lists).collect();
            pos.func.dfg.replace(inst).jump(alternative, &args);
        } else if self.just_trap_blocks.contains(alternative) {
            let mut pos = FuncCursor::new(func);
            pos.use_srcloc(
                pos.func
                    .layout
                    .first_inst(alternative)
                    .expect("just-trap blocks have exactly one inst"),
            );
            pos.goto_inst(inst);

            let code = self.just_trap_block_code(pos.func, alternative);
            pos.ins().trapz(arg, code);

            let args: SmallVec<[_; 8]> = blocks[0].args(&pos.func.dfg.value_lists).collect();
            pos.func.dfg.replace(inst).jump(consequent, &args);
        }
    }
}
