//! This module contains the runtime components of the implementation of the
//! stack switching proposal.

mod stack;

use core::{marker::PhantomPinned, ptr::NonNull};

pub use stack::*;

/// A continuation object is a handle to a continuation reference
/// (i.e. an actual stack). A continuation object only be consumed
/// once. The linearity is checked dynamically in the generated code
/// by comparing the revision witness embedded in the pointer to the
/// actual revision counter on the continuation reference.
///
/// In the optimized implementation, the continuation logically
/// represented by a VMContObj not only encompasses the pointed-to
/// VMContRef, but also all of its parents:
///
/// ```text
///
///                     +----------------+
///                 +-->|   VMContRef    |
///                 |   +----------------+
///                 |            ^
///                 |            | parent
///                 |            |
///                 |   +----------------+
///                 |   |   VMContRef    |
///                 |   +----------------+
///                 |            ^
///                 |            | parent
///  last ancestor  |            |
///                 |   +----------------+
///                 +---|   VMContRef    |    <--  VMContObj
///                     +----------------+
/// ```
///
/// For performance reasons, the VMContRef at the bottom of this chain
/// (i.e., the one pointed to by the VMContObj) has a pointer to the
/// other end of the chain (i.e., its last ancestor).
// FIXME(frank-emrich) Does this actually need to be 16-byte aligned any
// more? Now that we use I128 on the Cranelift side (see
// [wasmtime_cranelift::stack_switching::fatpointer::pointer_type]), it
// should be fine to use the natural alignment of the type.
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct VMContObj {
    pub revision: u64,
    pub contref: NonNull<VMContRef>,
}

impl VMContObj {
    pub fn new(contref: NonNull<VMContRef>, revision: u64) -> Self {
        Self { contref, revision }
    }

    /// Construction a VMContinuationObject from a pointer and revision
    ///
    /// The `contref` pointer may be null in which case None will be returned.
    ///
    /// # Safety
    ///
    /// Behavior will be undefined if a pointer to data that is not a
    /// VMContRef is provided.
    pub unsafe fn from_raw_parts(contref: *mut u8, revision: u64) -> Option<Self> {
        NonNull::new(contref.cast::<VMContRef>()).map(|contref| Self::new(contref, revision))
    }
}

unsafe impl Send for VMContObj {}
unsafe impl Sync for VMContObj {}

/// This type is used to save (and subsequently restore) a subset of the data in
/// `VMStoreContext`. See documentation of `VMStackChain` for the exact uses.
#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct VMStackLimits {
    /// Saved version of `stack_limit` field of `VMStoreContext`
    pub stack_limit: usize,
    /// Saved version of `last_wasm_entry_fp` field of `VMStoreContext`
    pub last_wasm_entry_fp: usize,
}

/// This type represents "common" information that we need to save both for the
/// initial stack and each continuation.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMCommonStackInformation {
    /// Saves subset of `VMStoreContext` for this stack. See documentation of
    /// `VMStackChain` for the exact uses.
    pub limits: VMStackLimits,
    /// For the initial stack, this field must only have one of the following values:
    /// - Running
    /// - Parent
    pub state: VMStackState,

    /// Only in use when state is `Parent`. Otherwise, the list must be empty.
    ///
    /// Represents the handlers that this stack installed when resume-ing a
    /// continuation.
    ///
    /// Note that for any resume instruction, we can re-order the handler
    /// clauses without changing behavior such that all the suspend handlers
    /// come first, followed by all the switch handler (while maintaining the
    /// original ordering within the two groups).
    /// Thus, we assume that the given resume instruction has the following
    /// shape:
    ///
    /// (resume $ct
    ///   (on $tag_0 $block_0) ... (on $tag_{n-1} $block_{n-1})
    ///   (on $tag_n switch) ... (on $tag_m switch)
    /// )
    ///
    /// On resume, the handler list is then filled with m + 1 (i.e., one per
    /// handler clause) entries such that the i-th entry, using 0-based
    /// indexing, is the identifier of $tag_i (represented as *mut
    /// VMTagDefinition).
    /// Further, `first_switch_handler_index` (see below) is set to n (i.e., the
    /// 0-based index of the first switch handler).
    ///
    /// Note that the actual data buffer (i.e., the one `handler.data` points
    /// to) is always allocated on the stack that this `CommonStackInformation`
    /// struct describes.
    pub handlers: VMHandlerList,

    /// Only used when state is `Parent`. See documentation of `handlers` above.
    pub first_switch_handler_index: u32,
}

impl VMCommonStackInformation {
    /// Default value with state set to `Running`
    pub fn running_default() -> Self {
        Self {
            limits: VMStackLimits::default(),
            state: VMStackState::Running,
            handlers: VMHandlerList::empty(),
            first_switch_handler_index: 0,
        }
    }
}

impl VMStackLimits {
    /// Default value, but uses the given value for `stack_limit`.
    pub fn with_stack_limit(stack_limit: usize) -> Self {
        Self {
            stack_limit,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
/// Reference to a stack-allocated buffer ("array"), storing data of some type
/// `T`.
pub struct VMHostArray<T> {
    /// Number of currently occupied slots.
    pub length: u32,
    /// Number of slots in the data buffer. Note that this is *not* the size of
    /// the buffer in bytes!
    pub capacity: u32,
    /// The actual data buffer
    pub data: *mut T,
}

impl<T> VMHostArray<T> {
    /// Creates empty `Array`
    pub fn empty() -> Self {
        Self {
            length: 0,
            capacity: 0,
            data: core::ptr::null_mut(),
        }
    }

    /// Makes `Array` empty.
    pub fn clear(&mut self) {
        *self = Self::empty();
    }
}

/// Type used for passing payloads to and from continuations. The actual type
/// argument should be wasmtime::runtime::vm::vmcontext::ValRaw, but we don't
/// have access to that here.
pub type VMPayloads = VMHostArray<u128>;

/// Type for a list of handlers, represented by the handled tag. Thus, the
/// stored data is actually `*mut VMTagDefinition`, but we don't havr access to
/// that here.
pub type VMHandlerList = VMHostArray<*mut u8>;

/// The main type representing a continuation.
#[repr(C)]
pub struct VMContRef {
    /// The `CommonStackInformation` of this continuation's stack.
    pub common_stack_information: VMCommonStackInformation,

    /// The parent of this continuation, which may be another continuation, the
    /// initial stack, or absent (in case of a suspended continuation).
    pub parent_chain: VMStackChain,

    /// Only used if `common_stack_information.state` is `Suspended` or `Fresh`. In
    /// that case, this points to the end of the stack chain (i.e., the
    /// continuation in the parent chain whose own `parent_chain` field is
    /// `VMStackChain::Absent`).
    /// Note that this may be a pointer to itself (if the state is `Fresh`, this is always the case).
    pub last_ancestor: *mut VMContRef,

    /// Revision counter.
    pub revision: u64,

    /// The underlying stack.
    pub stack: VMContinuationStack,

    /// Used to store only
    /// 1. The arguments to the function passed to cont.new
    /// 2. The return values of that function
    ///
    /// Note that the actual data buffer (i.e., the one `args.data` points
    /// to) is always allocated on this continuation's stack.
    pub args: VMPayloads,

    /// Once a continuation has been suspended (using suspend or switch),
    /// this buffer is used to pass payloads to and from the continuation.
    /// More concretely, it is used to
    /// - Pass payloads from a suspend instruction to the corresponding handler.
    /// - Pass payloads to a continuation using cont.bind or resume
    /// - Pass payloads to the continuation being switched to when using switch.
    ///
    /// Note that the actual data buffer (i.e., the one `values.data` points
    /// to) is always allocated on this continuation's stack.
    pub values: VMPayloads,

    /// Tell the compiler that this structure has potential self-references
    /// through the `last_ancestor` pointer.
    _marker: core::marker::PhantomPinned,
}

impl VMContRef {
    pub fn fiber_stack(&self) -> &VMContinuationStack {
        &self.stack
    }

    pub fn detach_stack(&mut self) -> VMContinuationStack {
        core::mem::replace(&mut self.stack, VMContinuationStack::unallocated())
    }

    /// This is effectively a `Default` implementation, without calling it
    /// so. Used to create `VMContRef`s when initializing pooling allocator.
    pub fn empty() -> Self {
        let limits = VMStackLimits::with_stack_limit(Default::default());
        let state = VMStackState::Fresh;
        let handlers = VMHandlerList::empty();
        let common_stack_information = VMCommonStackInformation {
            limits,
            state,
            handlers,
            first_switch_handler_index: 0,
        };
        let parent_chain = VMStackChain::Absent;
        let last_ancestor = core::ptr::null_mut();
        let stack = VMContinuationStack::unallocated();
        let args = VMPayloads::empty();
        let values = VMPayloads::empty();
        let revision = 0;
        let _marker = PhantomPinned;

        Self {
            common_stack_information,
            parent_chain,
            last_ancestor,
            stack,
            args,
            values,
            revision,
            _marker,
        }
    }
}

impl Drop for VMContRef {
    fn drop(&mut self) {
        // Note that continuation references do not own their parents, and we
        // don't drop them here.

        // We would like to enforce the invariant that any continuation that
        // was created for a cont.new (rather than, say, just living in a
        // pool and never being touched), either ran to completion or was
        // cancelled. But failing to do so should yield a custom error,
        // instead of panicking here.
    }
}

// These are required so the WasmFX pooling allocator can store a Vec of
// `VMContRef`s.
unsafe impl Send for VMContRef {}
unsafe impl Sync for VMContRef {}

/// Implements `cont.new` instructions (i.e., creation of continuations).
#[cfg(feature = "stack-switching")]
#[inline(always)]
pub fn cont_new(
    store: &mut dyn crate::vm::VMStore,
    instance: core::pin::Pin<&mut crate::vm::Instance>,
    func: *mut u8,
    param_count: u32,
    result_count: u32,
) -> Result<*mut VMContRef, crate::vm::TrapReason> {
    let caller_vmctx = instance.vmctx();

    let stack_size = store.engine().config().async_stack_size;

    let contref = store.allocate_continuation()?;
    let contref = unsafe { contref.as_mut().unwrap() };

    let tsp = contref.stack.top().unwrap();
    contref.parent_chain = VMStackChain::Absent;
    // The continuation is fresh, which is a special case of being suspended.
    // Thus we need to set the correct end of the continuation chain: itself.
    contref.last_ancestor = contref;

    // The initialization function will allocate the actual args/return value buffer and
    // update this object (if needed).
    let contref_args_ptr = &mut contref.args as *mut _ as *mut VMHostArray<crate::ValRaw>;

    contref.stack.initialize(
        func.cast::<crate::vm::VMFuncRef>(),
        caller_vmctx.as_ptr(),
        contref_args_ptr,
        param_count,
        result_count,
    );

    // Now that the initial stack pointer was set by the initialization
    // function, use it to determine stack limit.
    let stack_pointer = contref.stack.control_context_stack_pointer();
    // Same caveat regarding stack_limit here as described in
    // `wasmtime::runtime::func::EntryStoreContext::enter_wasm`.
    let wasm_stack_limit = core::cmp::max(
        stack_pointer - store.engine().config().max_wasm_stack,
        tsp as usize - stack_size,
    );
    let limits = VMStackLimits::with_stack_limit(wasm_stack_limit);
    let csi = &mut contref.common_stack_information;
    csi.state = VMStackState::Fresh;
    csi.limits = limits;

    log::trace!("Created contref @ {contref:p}");
    Ok(contref)
}

/// This type represents a linked lists ("chain") of stacks, where the a
/// node's successor denotes its parent.
/// Additionally, a `CommonStackInformation` object is associated with
/// each stack in the list.
/// Here, a "stack" is one of the following:
/// - A continuation (i.e., created with cont.new).
/// - The initial stack. This is the stack that we were on when entering
///   Wasm (i.e., when executing
///   `crate::runtime::func::invoke_wasm_and_catch_traps`).
///   This stack never has a parent.
///   In terms of the memory allocation that this stack resides on, it will
///   usually be the main stack, but doesn't have to: If we are running
///   inside a continuation while executing a host call, which in turn
///   re-renters Wasm, the initial stack is actually the stack of that
///   continuation.
///
/// Note that the linked list character of `VMStackChain` arises from the fact
/// that `VMStackChain::Continuation` variants have a pointer to a
/// `VMContRef`, which in turn has a `parent_chain` value of type
/// `VMStackChain`. This is how the stack chain reflects the parent-child
/// relationships between continuations/stacks. This also shows how the
/// initial stack (mentioned above) cannot have a parent.
///
/// There are generally two uses of `VMStackChain`:
///
/// 1. The `stack_chain` field in the `StoreOpaque` contains such a
/// chain of stacks, where the head of the list denotes the stack that is
/// currently executing (either a continuation or the initial stack). Note
/// that in this case, the linked list must contain 0 or more `Continuation`
/// elements, followed by a final `InitialStack` element. In particular,
/// this list always ends with `InitialStack` and never contains an `Absent`
/// variant.
///
/// 2. When a continuation is suspended, its chain of parents eventually
/// ends with an `Absent` variant in its `parent_chain` field. Note that a
/// suspended continuation never appears in the stack chain in the
/// VMContext!
///
///
/// As mentioned before, each stack in a `VMStackChain` has a corresponding
/// `CommonStackInformation` object. For continuations, this is stored in
/// the `common_stack_information` field of the corresponding `VMContRef`.
/// For the initial stack, the `InitialStack` variant contains a pointer to
/// a `CommonStackInformation`. The latter will be allocated allocated on
/// the stack frame that executed by `invoke_wasm_and_catch_traps`.
///
/// The following invariants hold for these `VMStackLimits` objects,
/// and the data in `VMStoreContext`.
///
/// Currently executing stack: For the currently executing stack (i.e., the
/// stack that is at the head of the store's `stack_chain` list), the
/// associated `VMStackLimits` object contains stale/undefined data. Instead,
/// the live data describing the limits for the currently executing stack is
/// always maintained in `VMStoreContext`. Note that as a general rule
/// independently from any execution of continuations, the `last_wasm_exit*`
/// fields in the `VMStoreContext` contain undefined values while executing
/// wasm.
///
/// Parents of currently executing stack: For stacks that appear in the tail
/// of the store's `stack_chain` list (i.e., stacks that are not currently
/// executing themselves, but are an ancestor of the currently executing
/// stack), we have the following: All the fields in the stack's
/// `VMStackLimits` are valid, describing the stack's stack limit, and
/// pointers where executing for that stack entered and exited WASM.
///
/// Suspended continuations: For suspended continuations (including their
/// ancestors), we have the following. Note that the initial stack can never
/// be in this state. The `stack_limit` and `last_enter_wasm_sp` fields of
/// the corresponding `VMStackLimits` object contain valid data, while the
/// `last_exit_wasm_*` fields contain arbitrary values. There is only one
/// exception to this: Note that a continuation that has been created with
/// cont.new, but never been resumed so far, is considered "suspended".
/// However, its `last_enter_wasm_sp` field contains undefined data. This is
/// justified, because when resume-ing a continuation for the first time, a
/// native-to-wasm trampoline is called, which sets up the
/// `last_wasm_entry_sp` in the `VMStoreContext` with the correct value,
/// thus restoring the necessary invariant.
#[derive(Debug, Clone, PartialEq)]
#[repr(usize, C)]
pub enum VMStackChain {
    /// For suspended continuations, denotes the end of their chain of
    /// ancestors.
    Absent = wasmtime_environ::STACK_CHAIN_ABSENT_DISCRIMINANT,
    /// Represents the initial stack (i.e., where we entered Wasm from the
    /// host by executing
    /// `crate::runtime::func::invoke_wasm_and_catch_traps`). Therefore, it
    /// does not have a parent. The `CommonStackInformation` that this
    /// variant points to is stored in the stack frame of
    /// `invoke_wasm_and_catch_traps`.
    InitialStack(*mut VMCommonStackInformation) =
        wasmtime_environ::STACK_CHAIN_INITIAL_STACK_DISCRIMINANT,
    /// Represents a continuation's stack.
    Continuation(*mut VMContRef) = wasmtime_environ::STACK_CHAIN_CONTINUATION_DISCRIMINANT,
}

impl VMStackChain {
    /// Indicates if `self` is a `InitialStack` variant.
    pub fn is_initial_stack(&self) -> bool {
        matches!(self, VMStackChain::InitialStack(_))
    }

    /// Returns an iterator over the continuations in this chain.
    /// We don't implement `IntoIterator` because our iterator is unsafe, so at
    /// least this gives us some way of indicating this, even though the actual
    /// unsafety lies in the `next` function.
    ///
    /// # Safety
    ///
    /// This function is not unsafe per see, but it returns an object
    /// whose usage is unsafe.
    pub unsafe fn into_continuation_iter(self) -> ContinuationIterator {
        ContinuationIterator(self)
    }

    /// Returns an iterator over the stack limits in this chain.
    /// We don't implement `IntoIterator` because our iterator is unsafe, so at
    /// least this gives us some way of indicating this, even though the actual
    /// unsafety lies in the `next` function.
    ///
    /// # Safety
    ///
    /// This function is not unsafe per see, but it returns an object
    /// whose usage is unsafe.
    pub unsafe fn into_stack_limits_iter(self) -> StackLimitsIterator {
        StackLimitsIterator(self)
    }
}

/// Iterator for Continuations in a stack chain.
pub struct ContinuationIterator(VMStackChain);

/// Iterator for VMStackLimits in a stack chain.
pub struct StackLimitsIterator(VMStackChain);

impl Iterator for ContinuationIterator {
    type Item = *mut VMContRef;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            VMStackChain::Absent | VMStackChain::InitialStack(_) => None,
            VMStackChain::Continuation(ptr) => {
                let continuation = unsafe { ptr.as_mut().unwrap() };
                self.0 = continuation.parent_chain.clone();
                Some(ptr)
            }
        }
    }
}

impl Iterator for StackLimitsIterator {
    type Item = *mut VMStackLimits;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            VMStackChain::Absent => None,
            VMStackChain::InitialStack(csi) => {
                let stack_limits = unsafe { &mut (*csi).limits } as *mut VMStackLimits;
                self.0 = VMStackChain::Absent;
                Some(stack_limits)
            }
            VMStackChain::Continuation(ptr) => {
                let continuation = unsafe { ptr.as_mut().unwrap() };
                let stack_limits =
                    (&mut continuation.common_stack_information.limits) as *mut VMStackLimits;
                self.0 = continuation.parent_chain.clone();
                Some(stack_limits)
            }
        }
    }
}

/// Encodes the life cycle of a `VMContRef`.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum VMStackState {
    /// The `VMContRef` has been created, but neither `resume` or `switch` has ever been
    /// called on it. During this stage, we may add arguments using `cont.bind`.
    Fresh = wasmtime_environ::STACK_STATE_FRESH_DISCRIMINANT,
    /// The continuation is running, meaning that it is the one currently
    /// executing code.
    Running = wasmtime_environ::STACK_STATE_RUNNING_DISCRIMINANT,
    /// The continuation is suspended because it executed a resume instruction
    /// that has not finished yet. In other words, it became the parent of
    /// another continuation (which may itself be `Running`, a `Parent`, or
    /// `Suspended`).
    Parent = wasmtime_environ::STACK_STATE_PARENT_DISCRIMINANT,
    /// The continuation was suspended by a `suspend` or `switch` instruction.
    Suspended = wasmtime_environ::STACK_STATE_SUSPENDED_DISCRIMINANT,
    /// The function originally passed to `cont.new` has returned normally.
    /// Note that there is no guarantee that a VMContRef will ever
    /// reach this status, as it may stay suspended until being dropped.
    Returned = wasmtime_environ::STACK_STATE_RETURNED_DISCRIMINANT,
}

#[cfg(test)]
mod tests {
    use core::mem::{offset_of, size_of};

    use wasmtime_environ::{HostPtr, Module, PtrSize, VMOffsets};

    use super::*;

    #[test]
    fn null_pointer_optimization() {
        // The Rust spec does not technically guarantee that the null pointer
        // optimization applies to a struct containing a `NonNull`.
        assert_eq!(size_of::<Option<VMContObj>>(), size_of::<VMContObj>());
    }

    #[test]
    fn check_vm_stack_limits_offsets() {
        let module = Module::new();
        let offsets = VMOffsets::new(HostPtr, &module);
        assert_eq!(
            offset_of!(VMStackLimits, stack_limit),
            usize::from(offsets.ptr.vmstack_limits_stack_limit())
        );
        assert_eq!(
            offset_of!(VMStackLimits, last_wasm_entry_fp),
            usize::from(offsets.ptr.vmstack_limits_last_wasm_entry_fp())
        );
    }

    #[test]
    fn check_vm_common_stack_information_offsets() {
        let module = Module::new();
        let offsets = VMOffsets::new(HostPtr, &module);
        assert_eq!(
            size_of::<VMCommonStackInformation>(),
            usize::from(offsets.ptr.size_of_vmcommon_stack_information())
        );
        assert_eq!(
            offset_of!(VMCommonStackInformation, limits),
            usize::from(offsets.ptr.vmcommon_stack_information_limits())
        );
        assert_eq!(
            offset_of!(VMCommonStackInformation, state),
            usize::from(offsets.ptr.vmcommon_stack_information_state())
        );
        assert_eq!(
            offset_of!(VMCommonStackInformation, handlers),
            usize::from(offsets.ptr.vmcommon_stack_information_handlers())
        );
        assert_eq!(
            offset_of!(VMCommonStackInformation, first_switch_handler_index),
            usize::from(
                offsets
                    .ptr
                    .vmcommon_stack_information_first_switch_handler_index()
            )
        );
    }

    #[test]
    fn check_vm_array_offsets() {
        // Note that the type parameter has no influence on the size and offsets.
        let module = Module::new();
        let offsets = VMOffsets::new(HostPtr, &module);
        assert_eq!(
            size_of::<VMHostArray<()>>(),
            usize::from(offsets.ptr.size_of_vmhostarray())
        );
        assert_eq!(
            offset_of!(VMHostArray<()>, length),
            usize::from(offsets.ptr.vmhostarray_length())
        );
        assert_eq!(
            offset_of!(VMHostArray<()>, capacity),
            usize::from(offsets.ptr.vmhostarray_capacity())
        );
        assert_eq!(
            offset_of!(VMHostArray<()>, data),
            usize::from(offsets.ptr.vmhostarray_data())
        );
    }

    #[test]
    fn check_vm_contref_offsets() {
        let module = Module::new();
        let offsets = VMOffsets::new(HostPtr, &module);
        assert_eq!(
            offset_of!(VMContRef, common_stack_information),
            usize::from(offsets.ptr.vmcontref_common_stack_information())
        );
        assert_eq!(
            offset_of!(VMContRef, parent_chain),
            usize::from(offsets.ptr.vmcontref_parent_chain())
        );
        assert_eq!(
            offset_of!(VMContRef, last_ancestor),
            usize::from(offsets.ptr.vmcontref_last_ancestor())
        );
        // Some 32-bit platforms need this to be 8-byte aligned, some don't.
        // So we need to make sure it always is, without padding.
        assert_eq!(u8::vmcontref_revision(&4) % 8, 0);
        assert_eq!(u8::vmcontref_revision(&8) % 8, 0);
        assert_eq!(
            offset_of!(VMContRef, revision),
            usize::from(offsets.ptr.vmcontref_revision())
        );
        assert_eq!(
            offset_of!(VMContRef, stack),
            usize::from(offsets.ptr.vmcontref_stack())
        );
        assert_eq!(
            offset_of!(VMContRef, args),
            usize::from(offsets.ptr.vmcontref_args())
        );
        assert_eq!(
            offset_of!(VMContRef, values),
            usize::from(offsets.ptr.vmcontref_values())
        );
    }

    #[test]
    fn check_vm_stack_chain_offsets() {
        let module = Module::new();
        let offsets = VMOffsets::new(HostPtr, &module);
        assert_eq!(
            size_of::<VMStackChain>(),
            usize::from(offsets.ptr.size_of_vmstack_chain())
        );
    }
}
