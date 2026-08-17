/*
 * This file was initially derived from the files
 * `js/src/jit/BacktrackingAllocator.h` and
 * `js/src/jit/BacktrackingAllocator.cpp` in Mozilla Firefox, and was
 * originally licensed under the Mozilla Public License 2.0. We
 * subsequently relicensed it to Apache-2.0 WITH LLVM-exception (see
 * https://github.com/bytecodealliance/regalloc2/issues/7).
 *
 * Since the initial port, the design has been substantially evolved
 * and optimized.
 */

//! Requirements computation.

use super::{Env, LiveBundleIndex};
use crate::{Function, Inst, Operand, OperandConstraint, PReg, ProgPoint};

pub struct RequirementConflict;

#[derive(Clone, Copy, Debug)]
pub enum RequirementConflictAt {
    /// A transition from a stack-constrained to a reg-constrained
    /// segment. The suggested split point is late, to keep the
    /// intervening region with the stackslot (which is cheaper).
    StackToReg(ProgPoint),
    /// A transition from a reg-constraint to a stack-constrained
    /// segment. Mirror of above: the suggested split point is early
    /// (just after the last register use).
    RegToStack(ProgPoint),
    /// Any other transition. The suggested split point is late (just
    /// before the conflicting use), but the split will also trim the
    /// ends and create a split bundle, so the intervening region will
    /// not appear with either side. This is probably for the best
    /// when e.g. the two sides of the split are both constrained to
    /// different physical registers: the part in the middle should be
    /// constrained to neither.
    Other(ProgPoint),
}

impl RequirementConflictAt {
    #[inline(always)]
    pub fn should_trim_edges_around_split(self) -> bool {
        match self {
            RequirementConflictAt::RegToStack(..) | RequirementConflictAt::StackToReg(..) => false,
            RequirementConflictAt::Other(..) => true,
        }
    }

    #[inline(always)]
    pub fn suggested_split_point(self) -> ProgPoint {
        match self {
            RequirementConflictAt::RegToStack(pt)
            | RequirementConflictAt::StackToReg(pt)
            | RequirementConflictAt::Other(pt) => pt,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Requirement {
    FixedReg(PReg),
    FixedStack(PReg),
    Register,
    Any,
}
impl Requirement {
    #[inline(always)]
    pub fn merge(self, other: Requirement) -> Result<Requirement, RequirementConflict> {
        match (self, other) {
            (other, Requirement::Any) | (Requirement::Any, other) => Ok(other),
            (Requirement::Register, Requirement::Register) => Ok(self),
            (Requirement::Register, Requirement::FixedReg(preg))
            | (Requirement::FixedReg(preg), Requirement::Register) => {
                Ok(Requirement::FixedReg(preg))
            }
            (Requirement::FixedReg(a), Requirement::FixedReg(b)) if a == b => Ok(self),
            (Requirement::FixedStack(a), Requirement::FixedStack(b)) if a == b => Ok(self),
            _ => Err(RequirementConflict),
        }
    }

    #[inline(always)]
    pub fn is_stack(self) -> bool {
        match self {
            Requirement::FixedStack(..) => true,
            Requirement::Register | Requirement::FixedReg(..) => false,
            Requirement::Any => false,
        }
    }

    #[inline(always)]
    pub fn is_reg(self) -> bool {
        match self {
            Requirement::Register | Requirement::FixedReg(..) => true,
            Requirement::FixedStack(..) => false,
            Requirement::Any => false,
        }
    }
}

impl<'a, F: Function> Env<'a, F> {
    #[inline(always)]
    pub fn requirement_from_operand(&self, op: Operand) -> Requirement {
        match op.constraint() {
            OperandConstraint::FixedReg(preg) => {
                if self.pregs[preg.index()].is_stack {
                    Requirement::FixedStack(preg)
                } else {
                    Requirement::FixedReg(preg)
                }
            }
            OperandConstraint::Reg | OperandConstraint::Reuse(_) => Requirement::Register,
            OperandConstraint::Any => Requirement::Any,
        }
    }

    pub fn compute_requirement(
        &self,
        bundle: LiveBundleIndex,
    ) -> Result<Requirement, RequirementConflictAt> {
        let mut req = Requirement::Any;
        let mut last_pos = ProgPoint::before(Inst::new(0));
        trace!("compute_requirement: {:?}", bundle);
        let ranges = &self.bundles[bundle].ranges;
        for entry in ranges {
            trace!(" -> LR {:?}: {:?}", entry.index, entry.range);
            for u in &self.ranges[entry.index].uses {
                trace!("  -> use {:?}", u);
                let r = self.requirement_from_operand(u.operand);
                req = req.merge(r).map_err(|_| {
                    trace!("     -> conflict");
                    if req.is_stack() && r.is_reg() {
                        // Suggested split point just before the reg (i.e., late split).
                        RequirementConflictAt::StackToReg(u.pos)
                    } else if req.is_reg() && r.is_stack() {
                        // Suggested split point just after the stack
                        // (i.e., early split). Note that splitting
                        // with a use *right* at the beginning is
                        // interpreted by `split_and_requeue_bundle`
                        // as splitting off the first use.
                        RequirementConflictAt::RegToStack(last_pos)
                    } else {
                        RequirementConflictAt::Other(u.pos)
                    }
                })?;
                last_pos = u.pos;
                trace!("     -> req {:?}", req);
            }
        }
        trace!(" -> final: {:?}", req);
        Ok(req)
    }

    pub fn merge_bundle_requirements(
        &self,
        a: LiveBundleIndex,
        b: LiveBundleIndex,
    ) -> Result<Requirement, RequirementConflict> {
        let req_a = self
            .compute_requirement(a)
            .map_err(|_| RequirementConflict)?;
        let req_b = self
            .compute_requirement(b)
            .map_err(|_| RequirementConflict)?;
        req_a.merge(req_b)
    }
}
