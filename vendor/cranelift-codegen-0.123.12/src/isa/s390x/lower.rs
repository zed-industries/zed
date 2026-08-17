//! Lowering rules for S390x.

use crate::ir::Inst as IRInst;
use crate::isa::s390x::S390xBackend;
use crate::isa::s390x::inst::Inst;
use crate::machinst::{InstOutput, Lower, LowerBackend, MachLabel};

pub mod isle;

//=============================================================================
// Lowering-backend trait implementation.

impl LowerBackend for S390xBackend {
    type MInst = Inst;

    fn lower(&self, ctx: &mut Lower<Inst>, ir_inst: IRInst) -> Option<InstOutput> {
        isle::lower(ctx, self, ir_inst)
    }

    fn lower_branch(
        &self,
        ctx: &mut Lower<Inst>,
        ir_inst: IRInst,
        targets: &[MachLabel],
    ) -> Option<()> {
        isle::lower_branch(ctx, self, ir_inst, targets)
    }

    type FactFlowState = ();
}
