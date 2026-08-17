//! Lowering backend for Pulley.

pub mod isle;

use super::{PulleyBackend, PulleyTargetKind, inst::*};
use crate::{
    ir,
    machinst::{lower::*, *},
};

impl<P> LowerBackend for PulleyBackend<P>
where
    P: PulleyTargetKind,
{
    type MInst = InstAndKind<P>;

    fn lower(&self, ctx: &mut Lower<Self::MInst>, ir_inst: ir::Inst) -> Option<InstOutput> {
        isle::lower(ctx, self, ir_inst)
    }

    fn lower_branch(
        &self,
        ctx: &mut Lower<Self::MInst>,
        ir_inst: ir::Inst,
        targets: &[MachLabel],
    ) -> Option<()> {
        isle::lower_branch(ctx, self, ir_inst, targets)
    }

    fn maybe_pinned_reg(&self) -> Option<Reg> {
        // Pulley does not support this feature right now.
        None
    }

    type FactFlowState = ();
}
