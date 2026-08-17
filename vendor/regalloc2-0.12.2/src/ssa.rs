/*
 * Released under the terms of the Apache 2.0 license with LLVM
 * exception. See `LICENSE` for details.
 */

//! SSA-related utilities.

use alloc::vec;

use crate::cfg::CFGInfo;
use crate::{Block, Function, FxHashSet, Inst, OperandKind, RegAllocError, VReg};

pub fn validate_ssa<F: Function>(f: &F, cfginfo: &CFGInfo) -> Result<(), RegAllocError> {
    // For every block param and inst def, check that this is the only def.
    let mut defined_in = vec![Block::invalid(); f.num_vregs()];
    for block in 0..f.num_blocks() {
        let block = Block::new(block);
        let mut def = |vreg: VReg, inst| {
            if vreg.vreg() >= defined_in.len() {
                trace!("VRegs not numbered consecutively {:?}", vreg);
                return Err(RegAllocError::SSA(vreg, inst));
            }
            if defined_in[vreg.vreg()].is_valid() {
                trace!("Multiple def constraints for {:?}", vreg);
                Err(RegAllocError::SSA(vreg, inst))
            } else {
                defined_in[vreg.vreg()] = block;
                Ok(())
            }
        };
        for &param in f.block_params(block) {
            def(param, Inst::invalid())?;
        }
        for inst in f.block_insns(block).iter() {
            for operand in f.inst_operands(inst) {
                if let OperandKind::Def = operand.kind() {
                    def(operand.vreg(), inst)?;
                }
            }
        }
    }

    // Walk the blocks in arbitrary order. Check, for every use, that
    // the def is either in the same block in an earlier inst, or is
    // defined (by inst or blockparam) in some other block that
    // dominates this one.
    let mut local = FxHashSet::default();
    for block in 0..f.num_blocks() {
        let block = Block::new(block);
        local.clear();
        local.extend(f.block_params(block));

        for iix in f.block_insns(block).iter() {
            let operands = f.inst_operands(iix);
            for operand in operands {
                // Fixed registers uses will likely not be SSA, but they also
                // won't receive assignments.
                if operand.as_fixed_nonallocatable().is_some() {
                    continue;
                }

                match operand.kind() {
                    OperandKind::Use => {
                        let def_block = defined_in[operand.vreg().vreg()];
                        let okay = def_block.is_valid()
                            && if def_block == block {
                                local.contains(&operand.vreg())
                            } else {
                                cfginfo.dominates(def_block, block)
                            };
                        if !okay {
                            trace!("Invalid use {:?}", operand.vreg());
                            return Err(RegAllocError::SSA(operand.vreg(), iix));
                        }
                    }
                    OperandKind::Def => {
                        // Check all the uses in this instruction
                        // first, before recording its defs below.
                    }
                }
            }

            // In SSA form, an instruction can't use a VReg that it
            // also defines. So only record this instruction's defs
            // after its uses have been checked.
            for operand in operands {
                if let OperandKind::Def = operand.kind() {
                    local.insert(operand.vreg());
                }
            }
        }
    }

    // Check that the length of branch args matches the sum of the
    // number of blockparams in their succs, and that the end of every
    // block ends in this branch or in a ret, and that there are no
    // other branches or rets in the middle of the block.
    for block in 0..f.num_blocks() {
        let block = Block::new(block);
        let insns = f.block_insns(block);
        for insn in insns.iter() {
            if insn == insns.last() {
                if !(f.is_branch(insn) || f.is_ret(insn)) {
                    trace!("block {:?} is not terminated by a branch or ret!", block);
                    return Err(RegAllocError::BB(block));
                }
                if f.is_branch(insn) {
                    for (i, &succ) in f.block_succs(block).iter().enumerate() {
                        let blockparams_in = f.block_params(succ);
                        let blockparams_out = f.branch_blockparams(block, insn, i);
                        if blockparams_in.len() != blockparams_out.len() {
                            trace!(
                                "Mismatch on block params, found {} expected {}",
                                blockparams_out.len(),
                                blockparams_in.len()
                            );
                            return Err(RegAllocError::Branch(insn));
                        }
                    }
                }
            } else {
                if f.is_branch(insn) || f.is_ret(insn) {
                    trace!("Block terminator found in the middle of a block");
                    return Err(RegAllocError::BB(block));
                }
            }
        }
    }

    // Check that the entry block has no block args: otherwise it is
    // undefined what their value would be.
    if f.block_params(f.entry_block()).len() > 0 {
        trace!("Entry block contains block args");
        return Err(RegAllocError::BB(f.entry_block()));
    }

    Ok(())
}
