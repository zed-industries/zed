use crate::OperandConstraint::{self, *};
use crate::OperandKind::{self, *};
use crate::{
    run, Algorithm, Allocation, Block, Function, Inst, InstRange, MachineEnv, Operand, OperandPos,
    PReg, PRegSet, ProgPoint, RegClass, RegallocOptions, VReg,
};
use alloc::vec;
use alloc::vec::Vec;

#[test]
fn test_debug_locations1() {
    let mach_env = mach_env(10);
    let mut options = RegallocOptions::default();
    options.validate_ssa = true;
    options.algorithm = Algorithm::Fastalloc;
    let mut f = RealFunction::new(vec![BlockBuildInfo {
        insts: vec![
            /* 0. */ vec![op(Def, 0, FixedReg(p(0)))],
            /* 1. */
            vec![
                op(Def, 1, FixedReg(p(0))),
                op(Use, 0, FixedReg(p(0))),
                op(Use, 0, Reg),
            ],
            /* 2. */
            vec![
                op(Def, 2, FixedReg(p(8))),
                op(Use, 0, FixedReg(p(2))),
                op(Use, 1, FixedReg(p(0))),
            ],
            /* 3. */ vec![op(Def, 3, FixedReg(p(9))), op(Use, 0, FixedReg(p(9)))],
        ],
    }]);
    f.debug_value_labels = vec![
        (v(0), i(0), i(4), 32),
        (v(2), i(2), i(4), 70),
        (v(2), i(2), i(4), 71),
        (v(3), i(3), i(4), 34),
    ];
    let result = run(&f, &mach_env, &options).unwrap();
    assert_eq!(
        result.debug_locations,
        vec![
            (
                32,
                ProgPoint::after(i(0)),
                ProgPoint::after(i(3)),
                alloc(p(9))
            ),
            (
                34,
                ProgPoint::after(i(3)),
                ProgPoint::before(i(4)),
                alloc(p(9))
            ),
            (
                70,
                ProgPoint::after(i(2)),
                ProgPoint::before(i(3)),
                alloc(p(8))
            ),
            (
                71,
                ProgPoint::after(i(2)),
                ProgPoint::before(i(3)),
                alloc(p(8))
            ),
        ]
    );
}

#[test]
fn test_debug_locations2() {
    let mach_env = mach_env(2);
    let mut options = RegallocOptions::default();
    options.validate_ssa = true;
    options.algorithm = Algorithm::Fastalloc;
    let mut f = RealFunction::new(vec![BlockBuildInfo {
        insts: vec![
            /* 0. */ vec![op(Def, 2, FixedReg(p(0)))],
            /* 1. */ vec![op(Def, 0, FixedReg(p(0)))],
            /* 2. */ vec![op(Def, 1, FixedReg(p(1)))],
            /* 3. */ vec![op(Use, 0, FixedReg(p(0))), op(Use, 0, FixedReg(p(1)))],
            /* 4. */ vec![op(Use, 1, FixedReg(p(1)))],
        ],
    }]);
    f.debug_value_labels = vec![
        (v(0), i(1), i(4), 10),
        (v(1), i(0), i(1), 11),
        (v(1), i(2), i(3), 23),
    ];
    let result = run(&f, &mach_env, &options).unwrap();
    assert_eq!(result.debug_locations.len(), 2);
    assert_eq!(
        result.debug_locations[0],
        (
            10,
            ProgPoint::after(i(1)),
            ProgPoint::after(i(3)),
            alloc(p(0))
        )
    );
    assert_eq!(result.debug_locations[1].0, 23);
    assert_eq!(result.debug_locations[1].1, ProgPoint::after(i(2)));
    assert_eq!(result.debug_locations[1].2, ProgPoint::after(i(4)));
    assert!(matches!(result.debug_locations[1].3.as_stack(), Some(_)));
}

impl RealFunction {
    fn new(blocks: Vec<BlockBuildInfo>) -> Self {
        assert!(blocks.len() <= 2, "Just for testing purposes");
        let mut f = Self::default();
        let mut max_vreg_num_seen = 0;
        for block in blocks.iter() {
            f.blocks.push(RealBlock {
                params: vec![],
                preds: vec![],
                succs: vec![],
            });
            let start_inst_idx = f.insts.len();
            for inst in block.insts.iter() {
                f.insts.push(RealInst {
                    inst: Inst::new(f.insts.len()),
                    kind: RealInstKind::Normal,
                });
                let start_op_idx = f.operands.len();
                for op in inst.iter() {
                    max_vreg_num_seen = max_vreg_num_seen.max(op.vreg().vreg());
                    f.operands.push(*op);
                }
                f.operand_ranges.push((start_op_idx, f.operands.len()));
            }
            if !block.insts.is_empty() {
                f.insts.last_mut().unwrap().kind = RealInstKind::Ret;
            }
            f.inst_ranges.push((start_inst_idx, f.insts.len()));
        }
        f.num_vregs = max_vreg_num_seen + 1;
        f
    }
}

fn mach_env(no_of_regs: usize) -> MachineEnv {
    MachineEnv {
        preferred_regs_by_class: [
            (0..no_of_regs)
                .map(|no| PReg::new(no, RegClass::Int))
                .collect(),
            vec![],
            vec![],
        ],
        non_preferred_regs_by_class: [vec![], vec![], vec![]],
        scratch_by_class: [None, None, None],
        fixed_stack_slots: vec![],
    }
}

fn op(kind: OperandKind, vreg_num: usize, constraint: OperandConstraint) -> Operand {
    Operand::new(
        VReg::new(vreg_num, RegClass::Int),
        constraint,
        kind,
        match kind {
            Use => OperandPos::Early,
            Def => OperandPos::Late,
        },
    )
}

fn alloc(preg: PReg) -> Allocation {
    Allocation::reg(preg)
}

fn v(vreg_num: usize) -> VReg {
    VReg::new(vreg_num, RegClass::Int)
}

fn i(inst: usize) -> Inst {
    Inst::new(inst)
}

fn p(hw_enc: usize) -> PReg {
    PReg::new(hw_enc, RegClass::Int)
}

struct BlockBuildInfo {
    insts: Vec<Vec<Operand>>,
}

#[derive(Default)]
struct RealFunction {
    blocks: Vec<RealBlock>,
    insts: Vec<RealInst>,
    operands: Vec<Operand>,
    operand_ranges: Vec<(usize, usize)>,
    inst_ranges: Vec<(usize, usize)>,
    num_vregs: usize,
    debug_value_labels: Vec<(VReg, Inst, Inst, u32)>,
}

struct RealBlock {
    params: Vec<VReg>,
    preds: Vec<Block>,
    succs: Vec<Block>,
}

struct RealInst {
    inst: Inst,
    kind: RealInstKind,
}

impl RealInst {
    fn is_branch(&self) -> bool {
        match self.kind {
            RealInstKind::Branch(_, _) => true,
            _ => false,
        }
    }

    fn is_ret(&self) -> bool {
        match self.kind {
            RealInstKind::Ret => true,
            _ => false,
        }
    }
}

enum RealInstKind {
    Normal,
    Branch(Block, Vec<VReg>),
    Ret,
}

impl Function for RealFunction {
    fn num_insts(&self) -> usize {
        self.insts.len()
    }

    fn num_blocks(&self) -> usize {
        self.blocks.len()
    }

    fn block_insns(&self, block: crate::Block) -> crate::InstRange {
        let (start, end) = self.inst_ranges[block.index()];
        if start != end {
            InstRange::new(
                self.insts[start].inst,
                Inst::new(self.insts[end - 1].inst.index() + 1),
            )
        } else {
            InstRange::new(Inst::new(0), Inst::new(0))
        }
    }

    fn allow_multiple_vreg_defs(&self) -> bool {
        false
    }

    fn block_params(&self, block: crate::Block) -> &[VReg] {
        &self.blocks[block.index()].params
    }

    fn block_preds(&self, block: crate::Block) -> &[crate::Block] {
        &self.blocks[block.index()].preds
    }

    fn block_succs(&self, block: Block) -> &[Block] {
        &self.blocks[block.index()].succs
    }

    fn debug_value_labels(&self) -> &[(VReg, Inst, Inst, u32)] {
        &self.debug_value_labels
    }

    fn entry_block(&self) -> Block {
        Block::new(0)
    }

    fn inst_clobbers(&self, _insn: Inst) -> crate::PRegSet {
        PRegSet::empty()
    }

    fn inst_operands(&self, insn: Inst) -> &[Operand] {
        let (start, end) = self.operand_ranges[insn.index()];
        &self.operands[start..end]
    }

    fn is_branch(&self, insn: Inst) -> bool {
        self.insts[insn.index()].is_branch()
    }

    fn is_ret(&self, insn: Inst) -> bool {
        self.insts[insn.index()].is_ret()
    }

    fn multi_spillslot_named_by_last_slot(&self) -> bool {
        false
    }

    fn num_vregs(&self) -> usize {
        self.num_vregs
    }

    fn spillslot_size(&self, regclass: crate::RegClass) -> usize {
        match regclass {
            RegClass::Int => 2,
            RegClass::Float => 4,
            RegClass::Vector => 8,
        }
    }

    fn branch_blockparams(&self, _block: Block, _insn: Inst, _succ_idx: usize) -> &[VReg] {
        &[]
    }
}
