use crate::prelude::*;
use alloc::sync::Arc;
use core::ops::Range;
use wasmtime_fiber::{RuntimeFiberStack, RuntimeFiberStackCreator};

/// A stack creator. Can be used to provide a stack creator to wasmtime
/// which supplies stacks for async support.
///
/// # Safety
///
/// This trait is unsafe, as memory safety depends on a proper implementation
/// of memory management. Stacks created by the StackCreator should always be
/// treated as owned by an wasmtime instance, and any modification of them
/// outside of wasmtime invoked routines is unsafe and may lead to corruption.
///
/// Note that this is a relatively new and experimental feature and it is
/// recommended to be familiar with wasmtime runtime code to use it.
pub unsafe trait StackCreator: Send + Sync {
    /// Create a new `StackMemory` object with the specified size.
    ///
    /// The `size` parameter is the expected size of the stack without any guard pages.
    ///
    /// The `zeroed` parameter is whether the stack's memory should be zeroed,
    /// as a defense-in-depth measure.
    ///
    /// Note there should be at least one guard page of protected memory at the bottom
    /// of the stack to catch potential stack overflow scenarios. Additionally, stacks should be
    /// page aligned and zero filled.
    fn new_stack(&self, size: usize, zeroed: bool) -> Result<Box<dyn StackMemory>, Error>;
}

#[derive(Clone)]
pub(crate) struct StackCreatorProxy(pub Arc<dyn StackCreator>);

unsafe impl RuntimeFiberStackCreator for StackCreatorProxy {
    fn new_stack(&self, size: usize, zeroed: bool) -> Result<Box<dyn RuntimeFiberStack>, Error> {
        let stack = self.0.new_stack(size, zeroed)?;
        Ok(Box::new(FiberStackProxy(stack)) as Box<dyn RuntimeFiberStack>)
    }
}

/// A stack memory. This trait provides an interface for raw memory buffers
/// which are used by wasmtime inside of stacks which wasmtime executes
/// WebAssembly in for async support. By implementing this trait together
/// with StackCreator, one can supply wasmtime with custom allocated host
/// managed stacks.
///
/// # Safety
///
/// The memory should be page aligned and a multiple of page size.
/// To prevent possible silent overflows, the memory should be protected by a
/// guard page. Additionally the safety concerns explained in ['Memory'], for
/// accessing the memory apply here as well.
///
/// Note that this is a relatively new and experimental feature and it is
/// recommended to be familiar with wasmtime runtime code to use it.
pub unsafe trait StackMemory: Send + Sync {
    /// The top of the allocated stack.
    ///
    /// This address should be page aligned.
    fn top(&self) -> *mut u8;
    /// The range of where this stack resides in memory, excluding guard pages.
    fn range(&self) -> Range<usize>;
    /// The range of memory where the guard region of this stack resides.
    fn guard_range(&self) -> Range<*mut u8>;
}

pub(crate) struct FiberStackProxy(pub Box<dyn StackMemory>);

unsafe impl RuntimeFiberStack for FiberStackProxy {
    fn top(&self) -> *mut u8 {
        self.0.top()
    }

    fn range(&self) -> Range<usize> {
        self.0.range()
    }

    fn guard_range(&self) -> Range<*mut u8> {
        self.0.guard_range()
    }
}
