use super::SelectorState;
use super::attribute_matcher::AttributeMatcher;
use super::compiler::{CompiledAttributeExpr, CompiledLocalNameExpr};
use crate::html::LocalName;
use crate::selectors_vm::DenseHashSet;
use std::ops::Range;

pub(crate) type AddressRange = Range<usize>;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ExecutionBranch {
    pub matched_ids: DenseHashSet,
    pub jumps: Option<AddressRange>,
    pub hereditary_jumps: Option<AddressRange>,
}

/// The result of trying to execute an instruction without having parsed all attributes
pub(crate) enum TryExecResult<'i> {
    /// A successful match, contains the branch to move to
    Branch(&'i ExecutionBranch),
    /// A partially successful match, but requires attributes to complete
    AttributesRequired,
    /// A failed match, doesn't require attributes to complete
    Fail,
}

pub(crate) struct Instruction {
    pub associated_branch: ExecutionBranch,
    pub local_name_exprs: Box<[CompiledLocalNameExpr]>,
    pub attribute_exprs: Box<[CompiledAttributeExpr]>,
}

impl Instruction {
    pub fn noop() -> Self {
        Self {
            associated_branch: ExecutionBranch {
                matched_ids: DenseHashSet::Inline(0),
                jumps: None,
                hereditary_jumps: None,
            },
            local_name_exprs: Default::default(),
            attribute_exprs: Default::default(),
        }
    }

    pub fn try_exec_without_attrs<'i>(
        &'i self,
        state: &SelectorState<'_>,
        local_name: &LocalName<'_>,
    ) -> TryExecResult<'i> {
        if self.local_name_exprs.iter().all(|e| e(state, local_name)) {
            if self.attribute_exprs.is_empty() {
                TryExecResult::Branch(&self.associated_branch)
            } else {
                TryExecResult::AttributesRequired
            }
        } else {
            TryExecResult::Fail
        }
    }

    pub fn complete_exec_with_attrs<'i>(
        &'i self,
        state: &SelectorState<'_>,
        attr_matcher: &AttributeMatcher<'_>,
    ) -> Option<&'i ExecutionBranch> {
        if self.attribute_exprs.iter().all(|e| e(state, attr_matcher)) {
            Some(&self.associated_branch)
        } else {
            None
        }
    }

    pub fn exec<'i>(
        &'i self,
        state: &SelectorState<'_>,
        local_name: &LocalName<'_>,
        attr_matcher: &AttributeMatcher<'_>,
    ) -> Option<&'i ExecutionBranch> {
        let is_match = self.local_name_exprs.iter().all(|e| e(state, local_name))
            && self.attribute_exprs.iter().all(|e| e(state, attr_matcher));

        if is_match {
            Some(&self.associated_branch)
        } else {
            None
        }
    }
}

pub(crate) struct Program {
    pub instructions: Box<[Instruction]>,
    pub entry_points: AddressRange,
    /// Enables tracking child types for nth-of-type selectors.
    /// This is disabled if no nth-of-type selectors are used in the program.
    pub enable_nth_of_type: bool,
}
