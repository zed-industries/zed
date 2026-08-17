//! no_std implementation of fibers.
//!
//! This is a very stripped-down version of the Unix platform support,
//! but without mmap or guard pages, because on no_std systems we do
//! not assume that virtual memory exists.
//!
//! The stack layout is nevertheless the same (modulo the guard page)
//! as on Unix because we share its low-level implementations:
//!
//! ```text
//! 0xB000 +-----------------------+   <- top of stack
//!        | &Cell<RunResult>      |   <- where to store results
//! 0xAff8 +-----------------------+
//!        | *const u8             |   <- last sp to resume from
//! 0xAff0 +-----------------------+   <- 16-byte aligned
//!        |                       |
//!        ~        ...            ~   <- actual native stack space to use
//!        |                       |
//! 0x0000 +-----------------------+
//! ```
//!
//! Here `0xAff8` is filled in temporarily while `resume` is running. The fiber
//! started with 0xB000 as a parameter so it knows how to find this.
//! Additionally `resumes` stores state at 0xAff0 to restart execution, and
//! `suspend`, which has 0xB000 so it can find this, will read that and write
//! its own resumption information into this slot as well.

use crate::stackswitch::*;
use crate::{Result, RunResult, RuntimeFiberStack};
use alloc::boxed::Box;
use alloc::{vec, vec::Vec};
use core::cell::Cell;
use core::ops::Range;

// The no_std implementation is infallible in practice, but we use
// `anyhow::Error` here absent any better alternative.
pub type Error = anyhow::Error;

pub struct FiberStack {
    base: BasePtr,
    len: usize,
    /// Backing storage, if owned. Allocated once at startup and then
    /// not reallocated afterward.
    storage: Vec<u8>,
}

struct BasePtr(*mut u8);

unsafe impl Send for BasePtr {}
unsafe impl Sync for BasePtr {}

const STACK_ALIGN: usize = 16;

/// Align a pointer by incrementing it up to `align - 1`
/// bytes. `align` must be a power of two. Also updates the length as
/// appropriate so that `ptr + len` points to the same endpoint.
fn align_ptr(ptr: *mut u8, len: usize, align: usize) -> (*mut u8, usize) {
    let ptr = ptr as usize;
    let aligned = (ptr + align - 1) & !(align - 1);
    let new_len = len - (aligned - ptr);
    (aligned as *mut u8, new_len)
}

impl FiberStack {
    pub fn new(size: usize, zeroed: bool) -> Result<Self> {
        // Round up the size to at least one page.
        let size = core::cmp::max(4096, size);
        let mut storage = Vec::new();
        storage.reserve_exact(size);
        if zeroed {
            storage.resize(size, 0);
        }
        let (base, len) = align_ptr(storage.as_mut_ptr(), size, STACK_ALIGN);
        Ok(FiberStack {
            storage,
            base: BasePtr(base),
            len,
        })
    }

    pub unsafe fn from_raw_parts(base: *mut u8, guard_size: usize, len: usize) -> Result<Self> {
        Ok(FiberStack {
            storage: vec![],
            base: BasePtr(unsafe { base.offset(isize::try_from(guard_size).unwrap()) }),
            len,
        })
    }

    pub fn is_from_raw_parts(&self) -> bool {
        self.storage.is_empty()
    }

    pub fn from_custom(_custom: Box<dyn RuntimeFiberStack>) -> Result<Self> {
        unimplemented!("Custom fiber stacks not supported in no_std fiber library")
    }

    pub fn top(&self) -> Option<*mut u8> {
        Some(self.base.0.wrapping_byte_add(self.len))
    }

    pub fn range(&self) -> Option<Range<usize>> {
        let base = self.base.0 as usize;
        Some(base..base + self.len)
    }

    pub fn guard_range(&self) -> Option<Range<*mut u8>> {
        None
    }
}

pub struct Fiber;

pub struct Suspend {
    top_of_stack: *mut u8,
}

extern "C" fn fiber_start<F, A, B, C>(arg0: *mut u8, top_of_stack: *mut u8)
where
    F: FnOnce(A, &mut super::Suspend<A, B, C>) -> C,
{
    unsafe {
        let inner = Suspend { top_of_stack };
        let initial = inner.take_resume::<A, B, C>();
        super::Suspend::<A, B, C>::execute(inner, initial, Box::from_raw(arg0.cast::<F>()))
    }
}

impl Fiber {
    pub fn new<F, A, B, C>(stack: &FiberStack, func: F) -> Result<Self>
    where
        F: FnOnce(A, &mut super::Suspend<A, B, C>) -> C,
    {
        // On unsupported platforms `wasmtime_fiber_init` is a panicking shim so
        // return an error saying the host architecture isn't supported instead.
        if !SUPPORTED_ARCH {
            anyhow::bail!("fibers unsupported on this host architecture");
        }
        unsafe {
            let data = Box::into_raw(Box::new(func)).cast();
            wasmtime_fiber_init(stack.top().unwrap(), fiber_start::<F, A, B, C>, data);
        }

        Ok(Self)
    }

    pub(crate) fn resume<A, B, C>(&self, stack: &FiberStack, result: &Cell<RunResult<A, B, C>>) {
        unsafe {
            // Store where our result is going at the very tip-top of the
            // stack, otherwise known as our reserved slot for this information.
            //
            // In the diagram above this is updating address 0xAff8
            let addr = stack.top().unwrap().cast::<usize>().offset(-1);
            addr.write(result as *const _ as usize);

            assert!(SUPPORTED_ARCH);
            wasmtime_fiber_switch(stack.top().unwrap());

            // null this out to help catch use-after-free
            addr.write(0);
        }
    }

    pub(crate) unsafe fn drop<A, B, C>(&mut self) {}
}

impl Suspend {
    pub(crate) fn switch<A, B, C>(&mut self, result: RunResult<A, B, C>) -> A {
        unsafe {
            // Calculate 0xAff8 and then write to it
            (*self.result_location::<A, B, C>()).set(result);

            wasmtime_fiber_switch(self.top_of_stack);

            self.take_resume::<A, B, C>()
        }
    }

    pub(crate) fn exit<A, B, C>(&mut self, result: RunResult<A, B, C>) {
        self.switch(result);
        unreachable!();
    }

    unsafe fn take_resume<A, B, C>(&self) -> A {
        unsafe {
            match (*self.result_location::<A, B, C>()).replace(RunResult::Executing) {
                RunResult::Resuming(val) => val,
                _ => panic!("not in resuming state"),
            }
        }
    }

    unsafe fn result_location<A, B, C>(&self) -> *const Cell<RunResult<A, B, C>> {
        unsafe {
            let ret = self.top_of_stack.cast::<*const u8>().offset(-1).read();
            assert!(!ret.is_null());
            ret.cast()
        }
    }
}
