#![allow(clippy::needless_pass_by_value)]

mod ast;
mod attribute_matcher;
mod compiler;
mod error;
mod match_info;
mod parser;
mod program;
mod stack;

use self::program::AddressRange;
use self::stack::StackDirective;
use crate::html::{LocalName, Namespace};
use crate::memory::{MemoryLimitExceededError, SharedMemoryLimiter};
use crate::transform_stream::AuxStartTagInfo;
use encoding_rs::Encoding;

pub use self::ast::*;
pub(crate) use self::attribute_matcher::AttributeMatcher;
pub(crate) use self::compiler::Compiler;
pub use self::error::SelectorError;
pub(crate) use self::match_info::{DenseHashSet, MatchId, MatchInfo};
pub use self::parser::Selector;
pub(crate) use self::program::{ExecutionBranch, Program, TryExecResult};
pub(crate) use self::stack::{ChildCounter, ElementData, Stack, StackItem};

pub(crate) type AuxStartTagInfoRequest<E> = Box<
    dyn FnOnce(
            &mut SelectorMatchingVm<E>,
            AuxStartTagInfo<'_>,
            &mut dyn FnMut(MatchInfo),
        ) -> Result<(), MemoryLimitExceededError>
        + Send,
>;

pub(crate) enum VmError<E: ElementData> {
    InfoRequest(AuxStartTagInfoRequest<E>),
    MemoryLimitExceeded(MemoryLimitExceededError),
}

type RecoveryPointHandler<T, E> =
    fn(&mut SelectorMatchingVm<E>, &mut ExecutionCtx<'static, E>, &AttributeMatcher<'_>, T);

#[derive(Default)]
struct JumpPtr {
    instr_set_idx: usize,
    offset: usize,
}

#[derive(Default)]
struct HereditaryJumpPtr {
    stack_offset: usize,
    instr_set_idx: usize,
    offset: usize,
}

struct Bailout<T> {
    at_addr: usize,
    recovery_point: T,
}

/// A container for tracking state from various places on the stack.
pub(crate) struct SelectorState<'i> {
    pub cumulative: &'i ChildCounter,
    pub typed: Option<&'i ChildCounter>,
}

struct ExecutionCtx<'i, E: ElementData> {
    stack_item: StackItem<'i, E>,
    with_content: bool,
    ns: Namespace,
    enable_esi_tags: bool,
}

impl<'i, E: ElementData> ExecutionCtx<'i, E> {
    #[inline]
    pub fn new(local_name: LocalName<'i>, ns: Namespace, enable_esi_tags: bool) -> Self {
        ExecutionCtx {
            stack_item: StackItem::new(local_name),
            with_content: true,
            ns,
            enable_esi_tags,
        }
    }

    #[inline(never)]
    pub fn add_execution_branch(&mut self, branch: &ExecutionBranch) {
        self.stack_item
            .element_data
            .matched_ids_mut()
            .union(&branch.matched_ids);

        if self.with_content {
            if let Some(ref jumps) = branch.jumps {
                self.stack_item.jumps.push(jumps.to_owned());
            }

            if let Some(ref hereditary_jumps) = branch.hereditary_jumps {
                self.stack_item
                    .hereditary_jumps
                    .push(hereditary_jumps.to_owned());
            }
        }
    }

    #[inline(never)]
    pub fn handle_matched_ids(&mut self, match_handler: &mut dyn FnMut(MatchInfo)) {
        for match_id in self.stack_item.element_data.matched_ids_mut().iter() {
            match_handler(MatchInfo {
                match_id,
                with_content: self.with_content,
            });
        }
    }

    #[inline]
    pub fn into_owned(self) -> ExecutionCtx<'static, E> {
        ExecutionCtx {
            stack_item: self.stack_item.into_owned(),
            with_content: self.with_content,
            ns: self.ns,
            enable_esi_tags: self.enable_esi_tags,
        }
    }
}

macro_rules! aux_info_request {
    ($req:expr) => {
        Err(VmError::InfoRequest(Box::new($req)))
    };
}

pub(crate) struct SelectorMatchingVm<E: ElementData> {
    program: Program,
    stack: Stack<E>,
    enable_esi_tags: bool,
}

impl<E> SelectorMatchingVm<E>
where
    E: ElementData + Send,
{
    #[inline]
    #[must_use]
    pub fn new(
        ast: Ast,
        encoding: &'static Encoding,
        memory_limiter: SharedMemoryLimiter,
        enable_esi_tags: bool,
    ) -> Self {
        let program = Compiler::new(encoding).compile(ast);

        Self {
            stack: Stack::new(memory_limiter, program.enable_nth_of_type),
            program,
            enable_esi_tags,
        }
    }

    pub fn exec_for_start_tag(
        &mut self,
        local_name: LocalName<'_>,
        ns: Namespace,
        match_handler: &mut dyn FnMut(MatchInfo),
    ) -> Result<(), VmError<E>> {
        use StackDirective::*;

        self.stack.add_child(&local_name);

        let mut ctx = ExecutionCtx::new(local_name, ns, self.enable_esi_tags);

        match Stack::get_stack_directive(&ctx.stack_item, ctx.ns, ctx.enable_esi_tags) {
            PopImmediately => {
                ctx.with_content = false;
                self.exec_without_attrs(ctx, match_handler)
            }
            PushIfNotSelfClosing => {
                let ctx = ctx.into_owned();

                aux_info_request!(move |this, aux_info, match_handler| this
                    .exec_after_immediate_aux_info_request(ctx, aux_info, match_handler))
            }
            Push => self.exec_without_attrs(ctx, match_handler),
        }
    }

    #[inline]
    pub fn exec_for_end_tag(
        &mut self,
        local_name: LocalName<'_>,
        unmatched_element_data_handler: impl FnMut(E),
    ) {
        self.stack
            .pop_up_to(local_name, unmatched_element_data_handler);
    }

    #[inline]
    pub fn current_element_data_mut(&mut self) -> Option<&mut E> {
        self.stack.current_element_data_mut()
    }

    fn exec_after_immediate_aux_info_request(
        &mut self,
        mut ctx: ExecutionCtx<'static, E>,
        aux_info: AuxStartTagInfo<'_>,
        match_handler: &mut dyn FnMut(MatchInfo),
    ) -> Result<(), MemoryLimitExceededError> {
        let attr_matcher = AttributeMatcher::new(*aux_info.input, aux_info.attr_buffer, ctx.ns);

        ctx.with_content = !aux_info.self_closing;

        self.exec_instr_set_with_attrs(&self.program.entry_points, &attr_matcher, &mut ctx, 0);

        self.exec_jumps_with_attrs(&attr_matcher, &mut ctx, JumpPtr::default());

        self.exec_hereditary_jumps_with_attrs(
            &attr_matcher,
            &mut ctx,
            HereditaryJumpPtr::default(),
        );

        ctx.handle_matched_ids(match_handler);

        if ctx.with_content {
            self.stack.push_item(ctx.stack_item)?;
        }

        Ok(())
    }

    fn bailout<T: 'static + Send>(
        ctx: ExecutionCtx<'_, E>,
        bailout: Bailout<T>,
        recovery_point_handler: RecoveryPointHandler<T, E>,
    ) -> Result<(), VmError<E>> {
        let mut ctx = ctx.into_owned();

        aux_info_request!(move |this, aux_info, match_handler| {
            let attr_matcher = AttributeMatcher::new(*aux_info.input, aux_info.attr_buffer, ctx.ns);

            this.complete_instr_execution_with_attrs(bailout.at_addr, &attr_matcher, &mut ctx);

            recovery_point_handler(this, &mut ctx, &attr_matcher, bailout.recovery_point);

            ctx.handle_matched_ids(match_handler);

            if ctx.with_content {
                this.stack.push_item(ctx.stack_item)?;
            }

            Ok(())
        })
    }

    fn recover_after_bailout_in_entry_points(
        &mut self,
        ctx: &mut ExecutionCtx<'static, E>,
        attr_matcher: &AttributeMatcher<'_>,
        recovery_point: usize,
    ) {
        self.exec_instr_set_with_attrs(
            &self.program.entry_points,
            attr_matcher,
            ctx,
            recovery_point,
        );

        self.exec_jumps_with_attrs(attr_matcher, ctx, JumpPtr::default());

        self.exec_hereditary_jumps_with_attrs(attr_matcher, ctx, HereditaryJumpPtr::default());
    }

    fn recover_after_bailout_in_jumps(
        &mut self,
        ctx: &mut ExecutionCtx<'static, E>,
        attr_matcher: &AttributeMatcher<'_>,
        recovery_point: JumpPtr,
    ) {
        self.exec_jumps_with_attrs(attr_matcher, ctx, recovery_point);

        self.exec_hereditary_jumps_with_attrs(attr_matcher, ctx, HereditaryJumpPtr::default());
    }

    #[inline]
    fn recover_after_bailout_in_hereditary_jumps(
        &mut self,
        ctx: &mut ExecutionCtx<'static, E>,
        attr_matcher: &AttributeMatcher<'_>,
        recovery_point: HereditaryJumpPtr,
    ) {
        self.exec_hereditary_jumps_with_attrs(attr_matcher, ctx, recovery_point);
    }

    fn exec_without_attrs(
        &mut self,
        mut ctx: ExecutionCtx<'_, E>,
        match_handler: &mut dyn FnMut(MatchInfo),
    ) -> Result<(), VmError<E>> {
        if let Err(b) =
            self.try_exec_instr_set_without_attrs(self.program.entry_points.clone(), &mut ctx)
        {
            return Self::bailout(ctx, b, Self::recover_after_bailout_in_entry_points);
        }

        if let Err(b) = self.try_exec_jumps_without_attrs(&mut ctx) {
            return Self::bailout(ctx, b, Self::recover_after_bailout_in_jumps);
        }

        if let Err(b) = self.try_exec_hereditary_jumps_without_attrs(&mut ctx) {
            return Self::bailout(ctx, b, Self::recover_after_bailout_in_hereditary_jumps);
        }

        ctx.handle_matched_ids(match_handler);

        if ctx.with_content {
            self.stack
                .push_item(ctx.stack_item.into_owned())
                .map_err(VmError::MemoryLimitExceeded)
        } else {
            Ok(())
        }
    }

    #[inline]
    fn complete_instr_execution_with_attrs(
        &self,
        addr: usize,
        attr_matcher: &AttributeMatcher<'_>,
        ctx: &mut ExecutionCtx<'_, E>,
    ) {
        let state = self.stack.build_state(&ctx.stack_item.local_name);
        if let Some(branch) =
            self.program.instructions[addr].complete_exec_with_attrs(&state, attr_matcher)
        {
            ctx.add_execution_branch(branch);
        }
    }

    #[inline]
    fn try_exec_instr_set_without_attrs(
        &self,
        addr_range: AddressRange,
        ctx: &mut ExecutionCtx<'_, E>,
    ) -> Result<(), Bailout<usize>> {
        let start = addr_range.start;
        let state = self.stack.build_state(&ctx.stack_item.local_name);

        for addr in addr_range {
            match self.program.instructions[addr]
                .try_exec_without_attrs(&state, &ctx.stack_item.local_name)
            {
                TryExecResult::Branch(branch) => ctx.add_execution_branch(branch),
                TryExecResult::AttributesRequired => {
                    return Err(Bailout {
                        at_addr: addr,
                        recovery_point: addr - start + 1,
                    });
                }
                TryExecResult::Fail => (),
            }
        }

        Ok(())
    }

    #[inline]
    fn exec_instr_set_with_attrs(
        &self,
        addr_range: &AddressRange,
        attr_matcher: &AttributeMatcher<'_>,
        ctx: &mut ExecutionCtx<'_, E>,
        offset: usize,
    ) {
        let state = self.stack.build_state(&ctx.stack_item.local_name);
        for addr in addr_range.start + offset..addr_range.end {
            let instr = &self.program.instructions[addr];

            if let Some(branch) = instr.exec(&state, &ctx.stack_item.local_name, attr_matcher) {
                ctx.add_execution_branch(branch);
            }
        }
    }

    fn try_exec_jumps_without_attrs(
        &self,
        ctx: &mut ExecutionCtx<'_, E>,
    ) -> Result<(), Bailout<JumpPtr>> {
        if let Some(parent) = self.stack.items().last() {
            for (i, jumps) in parent.jumps.iter().enumerate() {
                self.try_exec_instr_set_without_attrs(jumps.clone(), ctx)
                    .map_err(|b| Bailout {
                        at_addr: b.at_addr,
                        recovery_point: JumpPtr {
                            instr_set_idx: i,
                            offset: b.recovery_point,
                        },
                    })?;
            }
        }

        Ok(())
    }

    fn exec_jumps_with_attrs(
        &self,
        attr_matcher: &AttributeMatcher<'_>,
        ctx: &mut ExecutionCtx<'_, E>,
        ptr: JumpPtr,
    ) {
        // NOTE: find pointed jumps instruction set and execute it with the offset.
        if let Some(parent) = self.stack.items().last() {
            if let Some(ptr_jumps) = parent.jumps.get(ptr.instr_set_idx) {
                self.exec_instr_set_with_attrs(ptr_jumps, attr_matcher, ctx, ptr.offset);

                // NOTE: execute remaining jumps instruction sets as usual.
                for jumps in parent.jumps.iter().skip(ptr.instr_set_idx + 1) {
                    self.exec_instr_set_with_attrs(jumps, attr_matcher, ctx, 0);
                }
            }
        }
    }

    fn try_exec_hereditary_jumps_without_attrs(
        &self,
        ctx: &mut ExecutionCtx<'_, E>,
    ) -> Result<(), Bailout<HereditaryJumpPtr>> {
        for (i, ancestor) in self.stack.items().iter().rev().enumerate() {
            for (j, jumps) in ancestor.hereditary_jumps.iter().enumerate() {
                self.try_exec_instr_set_without_attrs(jumps.clone(), ctx)
                    .map_err(move |b| Bailout {
                        at_addr: b.at_addr,
                        recovery_point: HereditaryJumpPtr {
                            stack_offset: i,
                            instr_set_idx: j,
                            offset: b.recovery_point,
                        },
                    })?;
            }

            if !ancestor.has_ancestor_with_hereditary_jumps {
                break;
            }
        }

        Ok(())
    }

    fn exec_hereditary_jumps_with_attrs(
        &self,
        attr_matcher: &AttributeMatcher<'_>,
        ctx: &mut ExecutionCtx<'_, E>,
        ptr: HereditaryJumpPtr,
    ) {
        let items = self.stack.items();

        if items.is_empty() {
            return;
        }

        let ptr_ancestor_idx = items.len() - 1 - ptr.stack_offset;

        // NOTE: first find pointed ancestor, then jump instruction
        // set and execute it with the offset.
        if let Some(ptr_ancestor) = items.get(ptr_ancestor_idx) {
            if let Some(ptr_jumps) = ptr_ancestor.hereditary_jumps.get(ptr.instr_set_idx) {
                self.exec_instr_set_with_attrs(ptr_jumps, attr_matcher, ctx, ptr.offset);

                // NOTE: execute the rest of jump instruction sets in the pointed ancestor as usual.
                for jumps in ptr_ancestor
                    .hereditary_jumps
                    .iter()
                    .skip(ptr.instr_set_idx + 1)
                {
                    self.exec_instr_set_with_attrs(jumps, attr_matcher, ctx, 0);
                }
            }

            // NOTE: execute hereditary jumps in remaining ancestors as usual.
            if ptr_ancestor.has_ancestor_with_hereditary_jumps {
                for ancestor in items.iter().rev().skip(ptr.stack_offset + 1) {
                    for jumps in &ancestor.hereditary_jumps {
                        self.exec_instr_set_with_attrs(jumps, attr_matcher, ctx, 0);
                    }

                    if !ancestor.has_ancestor_with_hereditary_jumps {
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests;
