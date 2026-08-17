use core::fmt;

use alloc::{format, string::ToString, vec::Vec};
use serde::{Deserialize, Serialize};

use crate::{Block, Function, Inst, InstRange, MachineEnv, Operand, PRegSet, RegClass, VReg};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum InstOpcode {
    Op,
    Ret,
    Branch,
}

impl fmt::Display for InstOpcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstOpcode::Op => f.write_str("op"),
            InstOpcode::Ret => f.write_str("ret"),
            InstOpcode::Branch => f.write_str("branch"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct InstData {
    op: InstOpcode,
    operands: Vec<Operand>,
    clobbers: PRegSet,
}

/// A wrapper around a `Function` and `MachineEnv` that can be serialized and
/// deserialized.
///
/// The serialized form of this structure is not stable: it is intended to be
/// deserialized with the exact same version of regalloc2 as the one that it
/// was created with.
#[derive(Serialize, Deserialize)]
pub struct SerializableFunction {
    machine_env: MachineEnv,
    entry_block: Block,
    insts: Vec<InstData>,
    blocks: Vec<InstRange>,
    block_preds: Vec<Vec<Block>>,
    block_succs: Vec<Vec<Block>>,
    block_params_in: Vec<Vec<VReg>>,
    block_params_out: Vec<Vec<Vec<VReg>>>,
    num_vregs: usize,
    debug_value_labels: Vec<(VReg, Inst, Inst, u32)>,
    spillslot_size: Vec<usize>,
    multi_spillslot_named_by_last_slot: bool,
    allow_multiple_vreg_defs: bool,
}

impl SerializableFunction {
    /// Creates a new `SerializableFunction` from an arbitrary `Function` and
    /// `MachineEnv`.
    pub fn new(func: &impl Function, machine_env: MachineEnv) -> Self {
        Self {
            machine_env,
            entry_block: func.entry_block(),
            insts: (0..func.num_insts())
                .map(|i| {
                    let inst = Inst::new(i);
                    let op = if func.is_ret(inst) {
                        InstOpcode::Ret
                    } else if func.is_branch(inst) {
                        InstOpcode::Branch
                    } else {
                        InstOpcode::Op
                    };
                    InstData {
                        op,
                        operands: func.inst_operands(inst).to_vec(),
                        clobbers: func.inst_clobbers(inst),
                    }
                })
                .collect(),
            blocks: (0..func.num_blocks())
                .map(|i| {
                    let block = Block::new(i);
                    func.block_insns(block)
                })
                .collect(),
            block_preds: (0..func.num_blocks())
                .map(|i| {
                    let block = Block::new(i);
                    func.block_preds(block).to_vec()
                })
                .collect(),
            block_succs: (0..func.num_blocks())
                .map(|i| {
                    let block = Block::new(i);
                    func.block_succs(block).to_vec()
                })
                .collect(),
            block_params_in: (0..func.num_blocks())
                .map(|i| {
                    let block = Block::new(i);
                    func.block_params(block).to_vec()
                })
                .collect(),
            block_params_out: (0..func.num_blocks())
                .map(|i| {
                    let block = Block::new(i);
                    let inst = func.block_insns(block).last();
                    (0..func.block_succs(block).len())
                        .map(|succ_idx| func.branch_blockparams(block, inst, succ_idx).to_vec())
                        .collect()
                })
                .collect(),
            num_vregs: func.num_vregs(),
            debug_value_labels: func.debug_value_labels().to_vec(),
            spillslot_size: [
                func.spillslot_size(RegClass::Int),
                func.spillslot_size(RegClass::Float),
                func.spillslot_size(RegClass::Vector),
            ]
            .to_vec(),
            multi_spillslot_named_by_last_slot: func.multi_spillslot_named_by_last_slot(),
            allow_multiple_vreg_defs: func.allow_multiple_vreg_defs(),
        }
    }

    /// Returns the `MachineEnv` associated with this function.
    pub fn machine_env(&self) -> &MachineEnv {
        &self.machine_env
    }
}

impl Function for SerializableFunction {
    fn num_insts(&self) -> usize {
        self.insts.len()
    }

    fn num_blocks(&self) -> usize {
        self.blocks.len()
    }

    fn entry_block(&self) -> Block {
        self.entry_block
    }

    fn block_insns(&self, block: Block) -> InstRange {
        self.blocks[block.index()]
    }

    fn block_succs(&self, block: Block) -> &[Block] {
        &self.block_succs[block.index()][..]
    }

    fn block_preds(&self, block: Block) -> &[Block] {
        &self.block_preds[block.index()][..]
    }

    fn block_params(&self, block: Block) -> &[VReg] {
        &self.block_params_in[block.index()][..]
    }

    fn is_ret(&self, insn: Inst) -> bool {
        self.insts[insn.index()].op == InstOpcode::Ret
    }

    fn is_branch(&self, insn: Inst) -> bool {
        self.insts[insn.index()].op == InstOpcode::Branch
    }

    fn branch_blockparams(&self, block: Block, _: Inst, succ: usize) -> &[VReg] {
        &self.block_params_out[block.index()][succ][..]
    }

    fn inst_operands(&self, insn: Inst) -> &[Operand] {
        &self.insts[insn.index()].operands[..]
    }

    fn inst_clobbers(&self, insn: Inst) -> PRegSet {
        self.insts[insn.index()].clobbers
    }

    fn num_vregs(&self) -> usize {
        self.num_vregs
    }

    fn debug_value_labels(&self) -> &[(VReg, Inst, Inst, u32)] {
        &self.debug_value_labels[..]
    }

    fn spillslot_size(&self, regclass: RegClass) -> usize {
        self.spillslot_size[regclass as usize]
    }

    fn multi_spillslot_named_by_last_slot(&self) -> bool {
        self.multi_spillslot_named_by_last_slot
    }

    fn allow_multiple_vreg_defs(&self) -> bool {
        self.allow_multiple_vreg_defs
    }
}

impl fmt::Debug for SerializableFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\n")?;
        write!(f, "  machine_env: {:#?}\n", self.machine_env())?;
        write!(
            f,
            "  spillslot_size(Int): {}\n",
            self.spillslot_size(RegClass::Int)
        )?;
        write!(
            f,
            "  spillslot_size(Float): {}\n",
            self.spillslot_size(RegClass::Float)
        )?;
        write!(
            f,
            "  spillslot_size(Vector): {}\n",
            self.spillslot_size(RegClass::Vector)
        )?;
        write!(
            f,
            "  multi_spillslot_named_by_last_slot: {}\n",
            self.multi_spillslot_named_by_last_slot()
        )?;
        write!(
            f,
            "  allow_multiple_vreg_defs: {}\n",
            self.allow_multiple_vreg_defs()
        )?;
        for (i, blockrange) in self.blocks.iter().enumerate() {
            let succs = self.block_succs[i]
                .iter()
                .map(|b| b.index())
                .collect::<Vec<_>>();
            let preds = self.block_preds[i]
                .iter()
                .map(|b| b.index())
                .collect::<Vec<_>>();
            let params_in = self.block_params_in[i]
                .iter()
                .map(|v| format!("v{}", v.vreg()))
                .collect::<Vec<_>>()
                .join(", ");
            let params_out = self.block_params_out[i]
                .iter()
                .enumerate()
                .map(|(succ_idx, vec)| {
                    let succ = self.block_succs[i][succ_idx];
                    let params = vec
                        .iter()
                        .map(|v| format!("v{}", v.vreg()))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("block{}({})", succ.index(), params)
                })
                .collect::<Vec<_>>()
                .join(", ");
            write!(
                f,
                "  block{i}({params_in}): # succs:{succs:?} preds:{preds:?}\n",
            )?;
            for inst in blockrange.iter() {
                let ops: Vec<_> = self
                    .inst_operands(inst)
                    .iter()
                    .map(|op| op.to_string())
                    .collect();
                let ops = ops.join(", ");
                let clobbers = if self.inst_clobbers(inst) == PRegSet::empty() {
                    format!("")
                } else {
                    let clobbers: Vec<_> = self
                        .inst_clobbers(inst)
                        .into_iter()
                        .map(|preg| format!("Clobber: {preg}"))
                        .collect();
                    format!(", {}", clobbers.join(", "))
                };
                write!(
                    f,
                    "    inst{}: {} {ops}{clobbers}\n",
                    inst.index(),
                    self.insts[inst.index()].op,
                )?;
                if let InstOpcode::Branch = self.insts[inst.index()].op {
                    write!(f, "    params: {}\n", params_out)?;
                }
            }
        }
        write!(f, "}}\n")?;
        Ok(())
    }
}
