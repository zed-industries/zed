/*
 * Released under the terms of the Apache 2.0 license with LLVM
 * exception. See `LICENSE` for details.
 */

use crate::{
    domtree, postorder, Allocation, Block, Function, Inst, InstRange, MachineEnv, Operand,
    OperandConstraint, OperandKind, OperandPos, PReg, PRegSet, RegClass, VReg,
};

use alloc::vec::Vec;
use alloc::{format, vec};

use super::arbitrary::Result as ArbitraryResult;
use super::arbitrary::{Arbitrary, Unstructured};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstOpcode {
    Op,
    Ret,
    Branch,
}

#[derive(Clone, Debug)]
pub struct InstData {
    op: InstOpcode,
    operands: Vec<Operand>,
    clobbers: Vec<PReg>,
}

impl InstData {
    pub fn branch() -> InstData {
        InstData {
            op: InstOpcode::Branch,
            operands: vec![],
            clobbers: vec![],
        }
    }
    pub fn ret() -> InstData {
        InstData {
            op: InstOpcode::Ret,
            operands: vec![],
            clobbers: vec![],
        }
    }
}

#[derive(Clone)]
pub struct Func {
    insts: Vec<InstData>,
    blocks: Vec<InstRange>,
    block_preds: Vec<Vec<Block>>,
    block_succs: Vec<Vec<Block>>,
    block_params_in: Vec<Vec<VReg>>,
    block_params_out: Vec<Vec<Vec<VReg>>>,
    num_vregs: usize,
    reftype_vregs: Vec<VReg>,
    debug_value_labels: Vec<(VReg, Inst, Inst, u32)>,
}

impl Function for Func {
    fn num_insts(&self) -> usize {
        self.insts.len()
    }

    fn num_blocks(&self) -> usize {
        self.blocks.len()
    }

    fn entry_block(&self) -> Block {
        debug_assert!(self.blocks.len() > 0);
        Block::new(0)
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

    fn debug_value_labels(&self) -> &[(VReg, Inst, Inst, u32)] {
        &self.debug_value_labels[..]
    }

    fn inst_operands(&self, insn: Inst) -> &[Operand] {
        &self.insts[insn.index()].operands[..]
    }

    fn inst_clobbers(&self, insn: Inst) -> PRegSet {
        let mut set = PRegSet::default();
        for &preg in &self.insts[insn.index()].clobbers {
            set = set.with(preg);
        }
        set
    }

    fn num_vregs(&self) -> usize {
        self.num_vregs
    }

    fn spillslot_size(&self, regclass: RegClass) -> usize {
        match regclass {
            // Test the case where 2 classes share the same
            RegClass::Int => 1,
            RegClass::Float => 1,
            RegClass::Vector => 2,
        }
    }
}

struct FuncBuilder {
    postorder: Vec<Block>,
    idom: Vec<Block>,
    f: Func,
    insts_per_block: Vec<Vec<InstData>>,
}

impl FuncBuilder {
    fn new() -> Self {
        FuncBuilder {
            postorder: vec![],
            idom: vec![],
            f: Func {
                block_preds: vec![],
                block_succs: vec![],
                block_params_in: vec![],
                block_params_out: vec![],
                insts: vec![],
                blocks: vec![],
                num_vregs: 0,
                reftype_vregs: vec![],
                debug_value_labels: vec![],
            },
            insts_per_block: vec![],
        }
    }

    pub fn add_block(&mut self) -> Block {
        let b = Block::new(self.f.blocks.len());
        self.f
            .blocks
            .push(InstRange::new(Inst::new(0), Inst::new(0)));
        self.f.block_preds.push(vec![]);
        self.f.block_succs.push(vec![]);
        self.f.block_params_in.push(vec![]);
        self.f.block_params_out.push(vec![]);
        self.insts_per_block.push(vec![]);
        b
    }

    pub fn add_inst(&mut self, block: Block, data: InstData) {
        self.insts_per_block[block.index()].push(data);
    }

    pub fn add_edge(&mut self, from: Block, to: Block) {
        self.f.block_succs[from.index()].push(to);
        self.f.block_preds[to.index()].push(from);
    }

    pub fn set_block_params_in(&mut self, block: Block, params: &[VReg]) {
        self.f.block_params_in[block.index()] = params.iter().cloned().collect();
    }

    pub fn set_block_params_out(&mut self, block: Block, params: Vec<Vec<VReg>>) {
        self.f.block_params_out[block.index()] = params;
    }

    fn compute_doms(&mut self) {
        let f = &self.f;
        let _ = postorder::calculate(
            self.f.blocks.len(),
            Block::new(0),
            &mut vec![],
            &mut self.postorder,
            |block| &f.block_succs[block.index()][..],
        );
        domtree::calculate(
            self.f.blocks.len(),
            |block| &f.block_preds[block.index()][..],
            &self.postorder[..],
            &mut vec![],
            &mut self.idom,
            Block::new(0),
        );
    }

    fn finalize(mut self) -> Func {
        for (blocknum, blockrange) in self.f.blocks.iter_mut().enumerate() {
            let begin_inst = self.f.insts.len();
            for inst in &self.insts_per_block[blocknum] {
                self.f.insts.push(inst.clone());
            }
            let end_inst = self.f.insts.len();
            *blockrange = InstRange::new(Inst::new(begin_inst), Inst::new(end_inst));
        }

        self.f
    }
}

impl Arbitrary<'_> for RegClass {
    fn arbitrary(u: &mut Unstructured) -> ArbitraryResult<Self> {
        Ok(*u.choose(&[RegClass::Int, RegClass::Float, RegClass::Vector])?)
    }
}

impl Arbitrary<'_> for OperandConstraint {
    fn arbitrary(u: &mut Unstructured) -> ArbitraryResult<Self> {
        Ok(*u.choose(&[OperandConstraint::Any, OperandConstraint::Reg])?)
    }
}

fn choose_dominating_block(
    idom: &[Block],
    mut block: Block,
    allow_self: bool,
    u: &mut Unstructured,
) -> ArbitraryResult<Block> {
    debug_assert!(block.is_valid());
    let orig_block = block;
    loop {
        if (allow_self || block != orig_block) && bool::arbitrary(u)? {
            break;
        }
        if idom[block.index()].is_invalid() {
            break;
        }
        block = idom[block.index()];
    }
    let block = if block != orig_block || allow_self {
        block
    } else {
        Block::invalid()
    };
    Ok(block)
}

#[derive(Clone, Copy, Debug)]
pub struct Options {
    pub reused_inputs: bool,
    pub fixed_regs: bool,
    pub fixed_nonallocatable: bool,
    pub clobbers: bool,
    pub reftypes: bool,
    pub callsite_ish_constraints: bool,
}

impl core::default::Default for Options {
    fn default() -> Self {
        Options {
            reused_inputs: false,
            fixed_regs: false,
            fixed_nonallocatable: false,
            clobbers: false,
            reftypes: false,
            callsite_ish_constraints: false,
        }
    }
}

impl Arbitrary<'_> for Func {
    fn arbitrary(u: &mut Unstructured) -> ArbitraryResult<Func> {
        Func::arbitrary_with_options(u, &Options::default())
    }
}

impl Func {
    pub fn arbitrary_with_options(u: &mut Unstructured, opts: &Options) -> ArbitraryResult<Func> {
        // General strategy:
        // 1. Create an arbitrary CFG.
        // 2. Create a list of vregs to define in each block.
        // 3. Define some of those vregs in each block as blockparams.
        // 4. Populate blocks with ops that define the rest of the vregs.
        //    - For each use, choose an available vreg: either one
        //      already defined (via blockparam or inst) in this block,
        //      or one defined in a dominating block.

        let mut builder = FuncBuilder::new();
        for _ in 0..u.int_in_range(1..=100)? {
            builder.add_block();
        }
        let num_blocks = builder.f.blocks.len();

        // Generate a CFG. Create a "spine" of either single blocks,
        // with links to the next; or fork patterns, with the left
        // fork linking to the next and the right fork in `out_blocks`
        // to be connected below. This creates an arbitrary CFG with
        // split critical edges, which is a property that we require
        // for the regalloc.
        let mut from = 0;
        let mut out_blocks = vec![];
        let mut in_blocks = vec![];
        while from < num_blocks {
            in_blocks.push(from);
            if num_blocks > 3 && from < num_blocks - 3 && bool::arbitrary(u)? {
                // To avoid critical edges, we use from+1 as an edge
                // block, and advance `from` an extra block; `from+2`
                // will be the next normal iteration.
                builder.add_edge(Block::new(from), Block::new(from + 1));
                builder.add_edge(Block::new(from), Block::new(from + 2));
                builder.add_edge(Block::new(from + 2), Block::new(from + 3));
                out_blocks.push(from + 1);
                from += 2;
            } else if from < num_blocks - 1 {
                builder.add_edge(Block::new(from), Block::new(from + 1));
            }
            from += 1;
        }
        for pred in out_blocks {
            let succ = *u.choose(&in_blocks[..])?;
            builder.add_edge(Block::new(pred), Block::new(succ));
        }

        builder.compute_doms();

        for block in 0..num_blocks {
            builder.f.block_preds[block].clear();
        }
        for block in 0..num_blocks {
            for &succ in &builder.f.block_succs[block] {
                builder.f.block_preds[succ.index()].push(Block::new(block));
            }
        }

        builder.compute_doms();

        let alloc_vreg = |builder: &mut FuncBuilder, u: &mut Unstructured| {
            let vreg = VReg::new(builder.f.num_vregs, RegClass::arbitrary(u)?);
            builder.f.num_vregs += 1;
            Ok(vreg)
        };

        let mut vregs_by_block = vec![];
        let mut vregs_by_block_to_be_defined = vec![];
        let mut block_params = vec![vec![]; num_blocks];
        for block in 0..num_blocks {
            let mut vregs = vec![];
            for _ in 0..u.int_in_range(5..=15)? {
                let vreg = alloc_vreg(&mut builder, u)?;
                vregs.push(vreg);
                if opts.reftypes && bool::arbitrary(u)? {
                    builder.f.reftype_vregs.push(vreg);
                }
                if bool::arbitrary(u)? {
                    let assumed_end_inst = 10 * num_blocks;
                    let mut start = u.int_in_range::<usize>(0..=assumed_end_inst)?;
                    for _ in 0..10 {
                        if start >= assumed_end_inst {
                            break;
                        }
                        let end = u.int_in_range::<usize>(start..=assumed_end_inst)?;
                        let label = u.int_in_range::<u32>(0..=100)?;
                        builder.f.debug_value_labels.push((
                            vreg,
                            Inst::new(start),
                            Inst::new(end),
                            label,
                        ));
                        start = end;
                    }
                }
            }
            vregs_by_block.push(vregs.clone());
            let mut vregs_to_be_defined = vec![];
            let mut max_block_params = u.int_in_range(0..=core::cmp::min(3, vregs.len() / 3))?;
            for &vreg in &vregs {
                if block > 0 && bool::arbitrary(u)? && max_block_params > 0 {
                    block_params[block].push(vreg);
                    max_block_params -= 1;
                } else {
                    vregs_to_be_defined.push(vreg);
                }
            }
            vregs_to_be_defined.reverse();
            vregs_by_block_to_be_defined.push(vregs_to_be_defined);
            builder.set_block_params_in(Block::new(block), &block_params[block][..]);
        }

        for block in 0..num_blocks {
            let mut avail = block_params[block].clone();
            let mut remaining_nonlocal_uses = u.int_in_range(0..=3)?;
            while let Some(vreg) = vregs_by_block_to_be_defined[block].pop() {
                let def_constraint = OperandConstraint::arbitrary(u)?;
                let def_pos = if bool::arbitrary(u)? {
                    OperandPos::Early
                } else {
                    OperandPos::Late
                };
                let mut operands = vec![Operand::new(
                    vreg,
                    def_constraint,
                    OperandKind::Def,
                    def_pos,
                )];
                let mut allocations = vec![Allocation::none()];
                for _ in 0..u.int_in_range(0..=10)? {
                    let vreg = if avail.len() > 0
                        && (remaining_nonlocal_uses == 0 || bool::arbitrary(u)?)
                    {
                        *u.choose(&avail[..])?
                    } else {
                        let def_block = choose_dominating_block(
                            &builder.idom[..],
                            Block::new(block),
                            /* allow_self = */ false,
                            u,
                        )?;
                        if !def_block.is_valid() {
                            // No vregs already defined, and no pred blocks that dominate us
                            // (perhaps we are the entry block): just stop generating inputs.
                            break;
                        }
                        remaining_nonlocal_uses -= 1;
                        *u.choose(&vregs_by_block[def_block.index()])?
                    };
                    let use_constraint = OperandConstraint::arbitrary(u)?;
                    operands.push(Operand::new(
                        vreg,
                        use_constraint,
                        OperandKind::Use,
                        OperandPos::Early,
                    ));
                    allocations.push(Allocation::none());
                }
                let mut clobbers: Vec<PReg> = vec![];
                if operands.len() > 1 && opts.reused_inputs && bool::arbitrary(u)? {
                    // Make the def a reused input.
                    let op = operands[0];
                    debug_assert_eq!(op.kind(), OperandKind::Def);
                    let reused = u.int_in_range(1..=(operands.len() - 1))?;
                    if op.class() == operands[reused].class() {
                        operands[0] = Operand::new(
                            op.vreg(),
                            OperandConstraint::Reuse(reused),
                            op.kind(),
                            OperandPos::Late,
                        );
                        // Make sure reused input is a Reg.
                        let op = operands[reused];
                        operands[reused] = Operand::new(
                            op.vreg(),
                            OperandConstraint::Reg,
                            op.kind(),
                            OperandPos::Early,
                        );
                    }
                } else if opts.fixed_regs && bool::arbitrary(u)? {
                    let mut fixed_early = vec![];
                    let mut fixed_late = vec![];
                    for _ in 0..u.int_in_range(0..=operands.len() - 1)? {
                        // Pick an operand and make it a fixed reg.
                        let i = u.int_in_range(0..=(operands.len() - 1))?;
                        let op = operands[i];
                        let fixed_reg = PReg::new(u.int_in_range(0..=62)?, op.class());
                        if op.kind() == OperandKind::Def && op.pos() == OperandPos::Early {
                            // Early-defs with fixed constraints conflict with
                            // any other fixed uses of the same preg.
                            if fixed_late.contains(&fixed_reg) {
                                continue;
                            }
                        }
                        if op.kind() == OperandKind::Use && op.pos() == OperandPos::Late {
                            // Late-use with fixed constraints conflict with
                            // any other fixed uses of the same preg.
                            if fixed_early.contains(&fixed_reg) {
                                continue;
                            }
                        }
                        let fixed_list = match op.pos() {
                            OperandPos::Early => &mut fixed_early,
                            OperandPos::Late => &mut fixed_late,
                        };
                        if fixed_list.contains(&fixed_reg) {
                            continue;
                        }
                        fixed_list.push(fixed_reg);
                        operands[i] = Operand::new(
                            op.vreg(),
                            OperandConstraint::FixedReg(fixed_reg),
                            op.kind(),
                            op.pos(),
                        );
                    }

                    if opts.callsite_ish_constraints && bool::arbitrary(u)? {
                        // Define some new vregs with `any`
                        // constraints.
                        for _ in 0..u.int_in_range(0..=20)? {
                            let vreg = alloc_vreg(&mut builder, u)?;
                            operands.push(Operand::new(
                                vreg,
                                OperandConstraint::Any,
                                OperandKind::Def,
                                OperandPos::Late,
                            ));
                        }

                        // Create some clobbers, avoiding regs named
                        // by operand constraints. Note that the sum
                        // of the maximum clobber count here (10) and
                        // maximum operand count above (10) is less
                        // than the number of registers in any single
                        // class, so the resulting problem is always
                        // allocatable.
                        for _ in 0..u.int_in_range(0..=10)? {
                            let reg = u.int_in_range(0..=30)?;
                            let preg = PReg::new(reg, RegClass::arbitrary(u)?);
                            if operands
                                .iter()
                                .any(|op| match (op.kind(), op.constraint()) {
                                    (OperandKind::Def, OperandConstraint::FixedReg(fixed)) => {
                                        fixed == preg
                                    }
                                    _ => false,
                                })
                            {
                                continue;
                            }
                            clobbers.push(preg);
                        }
                    }
                } else if opts.clobbers && bool::arbitrary(u)? {
                    for _ in 0..u.int_in_range(0..=5)? {
                        let reg = u.int_in_range(0..=30)?;
                        if clobbers.iter().any(|r| r.hw_enc() == reg) {
                            continue;
                        }
                        clobbers.push(PReg::new(reg, RegClass::arbitrary(u)?));
                    }
                } else if opts.fixed_nonallocatable && bool::arbitrary(u)? {
                    operands.push(Operand::fixed_nonallocatable(PReg::new(
                        63,
                        RegClass::arbitrary(u)?,
                    )));
                }

                builder.add_inst(
                    Block::new(block),
                    InstData {
                        op: InstOpcode::Op,
                        operands,
                        clobbers,
                    },
                );
                avail.push(vreg);
            }

            // Define the branch with blockparam args that must end
            // the block.
            if builder.f.block_succs[block].len() > 0 {
                let mut params = vec![];
                for &succ in &builder.f.block_succs[block] {
                    let mut args = vec![];
                    for i in 0..builder.f.block_params_in[succ.index()].len() {
                        let dom_block = choose_dominating_block(
                            &builder.idom[..],
                            Block::new(block),
                            false,
                            u,
                        )?;

                        // Look for a vreg with a suitable class. If no
                        // suitable vreg is available then we error out, which
                        // causes the fuzzer to skip this function.
                        let vregs = if dom_block.is_valid() && bool::arbitrary(u)? {
                            &vregs_by_block[dom_block.index()][..]
                        } else {
                            &avail[..]
                        };
                        let suitable_vregs: Vec<_> = vregs
                            .iter()
                            .filter(|vreg| {
                                vreg.class() == builder.f.block_params_in[succ.index()][i].class()
                            })
                            .copied()
                            .collect();
                        let vreg = u.choose(&suitable_vregs)?;
                        args.push(*vreg);
                    }
                    params.push(args);
                }
                builder.set_block_params_out(Block::new(block), params);
                builder.add_inst(Block::new(block), InstData::branch());
            } else {
                builder.add_inst(Block::new(block), InstData::ret());
            }
        }

        builder.f.debug_value_labels.sort_unstable();

        Ok(builder.finalize())
    }
}

impl core::fmt::Debug for Func {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{{\n")?;
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
                "  block{}({}): # succs:{:?} preds:{:?}\n",
                i, params_in, succs, preds
            )?;
            for inst in blockrange.iter() {
                write!(
                    f,
                    "    inst{}: {:?} ops:{:?} clobber:{:?}\n",
                    inst.index(),
                    self.insts[inst.index()].op,
                    self.insts[inst.index()].operands,
                    self.insts[inst.index()].clobbers
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

pub fn machine_env() -> MachineEnv {
    fn regs(r: core::ops::Range<usize>, c: RegClass) -> Vec<PReg> {
        r.map(|i| PReg::new(i, c)).collect()
    }
    let preferred_regs_by_class: [Vec<PReg>; 3] = [
        regs(0..24, RegClass::Int),
        regs(0..24, RegClass::Float),
        regs(0..24, RegClass::Vector),
    ];
    let non_preferred_regs_by_class: [Vec<PReg>; 3] = [
        regs(24..32, RegClass::Int),
        regs(24..32, RegClass::Float),
        regs(24..32, RegClass::Vector),
    ];
    let scratch_by_class: [Option<PReg>; 3] = [None, None, None];
    let fixed_stack_slots = (32..63)
        .flat_map(|i| {
            [
                PReg::new(i, RegClass::Int),
                PReg::new(i, RegClass::Float),
                PReg::new(i, RegClass::Vector),
            ]
        })
        .collect();
    // Register 63 is reserved for use as a fixed non-allocatable register.
    MachineEnv {
        preferred_regs_by_class,
        non_preferred_regs_by_class,
        scratch_by_class,
        fixed_stack_slots,
    }
}
