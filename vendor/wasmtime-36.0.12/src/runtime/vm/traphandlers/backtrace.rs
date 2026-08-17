//! Backtrace and stack walking functionality for Wasm.
//!
//! Walking the Wasm stack is comprised of
//!
//! 1. identifying sequences of contiguous Wasm frames on the stack
//!    (i.e. skipping over native host frames), and
//!
//! 2. walking the Wasm frames within such a sequence.
//!
//! To perform (1) we maintain the entry stack pointer (SP) and exit frame
//! pointer (FP) and program counter (PC) each time we call into Wasm and Wasm
//! calls into the host via trampolines (see
//! `crates/wasmtime/src/runtime/vm/trampolines`). The most recent entry is
//! stored in `VMStoreContext` and older entries are saved in
//! `CallThreadState`. This lets us identify ranges of contiguous Wasm frames on
//! the stack.
//!
//! To solve (2) and walk the Wasm frames within a region of contiguous Wasm
//! frames on the stack, we configure Cranelift's `preserve_frame_pointers =
//! true` setting. Then we can do simple frame pointer traversal starting at the
//! exit FP and stopping once we reach the entry SP (meaning that the next older
//! frame is a host frame).

use crate::prelude::*;
use crate::runtime::store::StoreOpaque;
use crate::runtime::vm::stack_switching::VMStackChain;
use crate::runtime::vm::{
    Unwind, VMStoreContext,
    traphandlers::{CallThreadState, tls},
};
#[cfg(all(feature = "gc", feature = "stack-switching"))]
use crate::vm::stack_switching::{VMContRef, VMStackState};
use core::ops::ControlFlow;
use wasmtime_unwinder::Frame;

/// A WebAssembly stack trace.
#[derive(Debug)]
pub struct Backtrace(Vec<Frame>);

impl Backtrace {
    /// Returns an empty backtrace
    pub fn empty() -> Backtrace {
        Backtrace(Vec::new())
    }

    /// Capture the current Wasm stack in a backtrace.
    pub fn new(store: &StoreOpaque) -> Backtrace {
        let vm_store_context = store.vm_store_context();
        let unwind = store.unwinder();
        tls::with(|state| match state {
            Some(state) => unsafe {
                Self::new_with_trap_state(vm_store_context, unwind, state, None)
            },
            None => Backtrace(vec![]),
        })
    }

    /// Capture the current Wasm stack trace.
    ///
    /// If Wasm hit a trap, and we calling this from the trap handler, then the
    /// Wasm exit trampoline didn't run, and we use the provided PC and FP
    /// instead of looking them up in `VMStoreContext`.
    pub(crate) unsafe fn new_with_trap_state(
        vm_store_context: *const VMStoreContext,
        unwind: &dyn Unwind,
        state: &CallThreadState,
        trap_pc_and_fp: Option<(usize, usize)>,
    ) -> Backtrace {
        let mut frames = vec![];
        unsafe {
            Self::trace_with_trap_state(vm_store_context, unwind, state, trap_pc_and_fp, |frame| {
                frames.push(frame);
                ControlFlow::Continue(())
            });
        }
        Backtrace(frames)
    }

    /// Walk the current Wasm stack, calling `f` for each frame we walk.
    #[cfg(feature = "gc")]
    pub fn trace(store: &StoreOpaque, f: impl FnMut(Frame) -> ControlFlow<()>) {
        let vm_store_context = store.vm_store_context();
        let unwind = store.unwinder();
        tls::with(|state| match state {
            Some(state) => unsafe {
                Self::trace_with_trap_state(vm_store_context, unwind, state, None, f)
            },
            None => {}
        });
    }

    // Walk the stack of the given continuation, which must be suspended, and
    // all of its parent continuations (if any).
    #[cfg(all(feature = "gc", feature = "stack-switching"))]
    pub fn trace_suspended_continuation(
        store: &StoreOpaque,
        continuation: &VMContRef,
        f: impl FnMut(Frame) -> ControlFlow<()>,
    ) {
        log::trace!("====== Capturing Backtrace (suspended continuation) ======");

        assert_eq!(
            continuation.common_stack_information.state,
            VMStackState::Suspended
        );

        let unwind = store.unwinder();

        let pc = continuation.stack.control_context_instruction_pointer();
        let fp = continuation.stack.control_context_frame_pointer();
        let trampoline_fp = continuation
            .common_stack_information
            .limits
            .last_wasm_entry_fp;

        unsafe {
            // FIXME(frank-emrich) Casting from *const to *mut pointer is
            // terrible, but we won't actually modify any of the continuations
            // here.
            let stack_chain =
                VMStackChain::Continuation(continuation as *const VMContRef as *mut VMContRef);

            if let ControlFlow::Break(()) =
                Self::trace_through_continuations(unwind, stack_chain, pc, fp, trampoline_fp, f)
            {
                log::trace!("====== Done Capturing Backtrace (closure break) ======");
                return;
            }
        }

        log::trace!("====== Done Capturing Backtrace (reached end of stack chain) ======");
    }

    /// Walk the current Wasm stack, calling `f` for each frame we walk.
    ///
    /// If Wasm hit a trap, and we calling this from the trap handler, then the
    /// Wasm exit trampoline didn't run, and we use the provided PC and FP
    /// instead of looking them up in `VMStoreContext`.
    pub(crate) unsafe fn trace_with_trap_state(
        vm_store_context: *const VMStoreContext,
        unwind: &dyn Unwind,
        state: &CallThreadState,
        trap_pc_and_fp: Option<(usize, usize)>,
        mut f: impl FnMut(Frame) -> ControlFlow<()>,
    ) {
        log::trace!("====== Capturing Backtrace ======");

        let (last_wasm_exit_pc, last_wasm_exit_fp) = match trap_pc_and_fp {
            // If we exited Wasm by catching a trap, then the Wasm-to-host
            // trampoline did not get a chance to save the last Wasm PC and FP,
            // and we need to use the plumbed-through values instead.
            Some((pc, fp)) => {
                assert!(core::ptr::eq(
                    vm_store_context,
                    state.vm_store_context.as_ptr()
                ));
                (pc, fp)
            }
            // Either there is no Wasm currently on the stack, or we exited Wasm
            // through the Wasm-to-host trampoline.
            None => unsafe {
                let pc = *(*vm_store_context).last_wasm_exit_pc.get();
                let fp = *(*vm_store_context).last_wasm_exit_fp.get();
                (pc, fp)
            },
        };

        let stack_chain = unsafe { (*(*vm_store_context).stack_chain.get()).clone() };

        // The first value in `activations` is for the most recently running
        // wasm. We thus provide the stack chain of `first_wasm_state` to
        // traverse the potential continuation stacks. For the subsequent
        // activations, we unconditionally use `None` as the corresponding stack
        // chain. This is justified because only the most recent execution of
        // wasm may execute off the initial stack (see comments in
        // `wasmtime::invoke_wasm_and_catch_traps` for details).
        let activations =
            core::iter::once((stack_chain, last_wasm_exit_pc, last_wasm_exit_fp, unsafe {
                *(*vm_store_context).last_wasm_entry_fp.get()
            }))
            .chain(
                state
                    .iter()
                    .flat_map(|state| state.iter())
                    .filter(|state| {
                        core::ptr::eq(vm_store_context, state.vm_store_context.as_ptr())
                    })
                    .map(|state| unsafe {
                        (
                            state.old_stack_chain(),
                            state.old_last_wasm_exit_pc(),
                            state.old_last_wasm_exit_fp(),
                            state.old_last_wasm_entry_fp(),
                        )
                    }),
            )
            .take_while(|(chain, pc, fp, sp)| {
                if *pc == 0 {
                    debug_assert_eq!(*fp, 0);
                    debug_assert_eq!(*sp, 0);
                } else {
                    debug_assert_ne!(chain.clone(), VMStackChain::Absent)
                }
                *pc != 0
            });

        for (chain, pc, fp, sp) in activations {
            let res =
                unsafe { Self::trace_through_continuations(unwind, chain, pc, fp, sp, &mut f) };
            if let ControlFlow::Break(()) = res {
                log::trace!("====== Done Capturing Backtrace (closure break) ======");
                return;
            }
        }

        log::trace!("====== Done Capturing Backtrace (reached end of activations) ======");
    }

    /// Traces through a sequence of stacks, creating a backtrace for each one,
    /// beginning at the given `pc` and `fp`.
    ///
    /// If `chain` is `InitialStack`, we are tracing through the initial stack,
    /// and this function behaves like `trace_through_wasm`.
    /// Otherwise, we can interpret `chain` as a linked list of stacks, which
    /// ends with the initial stack. We then trace through each of these stacks
    /// individually, up to (and including) the initial stack.
    unsafe fn trace_through_continuations(
        unwind: &dyn Unwind,
        chain: VMStackChain,
        pc: usize,
        fp: usize,
        trampoline_fp: usize,
        mut f: impl FnMut(Frame) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        use crate::runtime::vm::stack_switching::{VMContRef, VMStackLimits};

        // Handle the stack that is currently running (which may be a
        // continuation or the initial stack).
        unsafe {
            wasmtime_unwinder::visit_frames(unwind, pc, fp, trampoline_fp, &mut f)?;
        }

        // Note that the rest of this function has no effect if `chain` is
        // `Some(VMStackChain::InitialStack(_))` (i.e., there is only one stack to
        // trace through: the initial stack)

        assert_ne!(chain, VMStackChain::Absent);
        let stack_limits_vec: Vec<*mut VMStackLimits> =
            unsafe { chain.clone().into_stack_limits_iter().collect() };
        let continuations_vec: Vec<*mut VMContRef> =
            unsafe { chain.clone().into_continuation_iter().collect() };

        // The VMStackLimits of the currently running stack (whether that's a
        // continuation or the initial stack) contains undefined data, the
        // information about that stack is saved in the Store's
        // `VMStoreContext` and handled at the top of this function
        // already. That's why we ignore `stack_limits_vec[0]`.
        //
        // Note that a continuation stack's control context stores
        // information about how to resume execution *in its parent*. Thus,
        // we combine the information from continuations_vec[i] with
        // stack_limits_vec[i + 1] below to get information about a
        // particular stack.
        //
        // There must be exactly one more `VMStackLimits` object than there
        // are continuations, due to the initial stack having one, too.
        assert_eq!(stack_limits_vec.len(), continuations_vec.len() + 1);

        for i in 0..continuations_vec.len() {
            // The continuation whose control context we want to
            // access, to get information about how to continue
            // execution in its parent.
            let continuation = unsafe { &*continuations_vec[i] };

            // The stack limits describing the parent of `continuation`.
            let parent_limits = unsafe { &*stack_limits_vec[i + 1] };

            // The parent of `continuation` if present not the last in the chain.
            let parent_continuation = continuations_vec.get(i + 1).map(|&c| unsafe { &*c });

            let fiber_stack = continuation.fiber_stack();
            let resume_pc = fiber_stack.control_context_instruction_pointer();
            let resume_fp = fiber_stack.control_context_frame_pointer();

            // If the parent is indeed a continuation, we know the
            // boundaries of its stack and can perform some extra debugging
            // checks.
            let parent_stack_range = parent_continuation.and_then(|p| p.fiber_stack().range());
            parent_stack_range.inspect(|parent_stack_range| {
                debug_assert!(parent_stack_range.contains(&resume_fp));
                debug_assert!(parent_stack_range.contains(&parent_limits.last_wasm_entry_fp));
                debug_assert!(parent_stack_range.contains(&parent_limits.stack_limit));
            });

            unsafe {
                wasmtime_unwinder::visit_frames(
                    unwind,
                    resume_pc,
                    resume_fp,
                    parent_limits.last_wasm_entry_fp,
                    &mut f,
                )?
            }
        }
        ControlFlow::Continue(())
    }

    /// Iterate over the frames inside this backtrace.
    pub fn frames<'a>(
        &'a self,
    ) -> impl ExactSizeIterator<Item = &'a Frame> + DoubleEndedIterator + 'a {
        self.0.iter()
    }
}
