//! The unix fiber implementation has some platform-specific details
//! (naturally) but there's a few details of the stack layout which are common
//! amongst all platforms using this file. Remember that none of this applies to
//! Windows, which is entirely separate.
//!
//! The stack is expected to look pretty standard with a guard page at the end.
//! Currently allocation happens in this file but this is probably going to be
//! refactored to happen somewhere else. Otherwise though the stack layout is
//! expected to look like so:
//!
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
//! 0x1000 +-----------------------+
//!        |  guard page           |
//! 0x0000 +-----------------------+
//! ```
//!
//! Here `0xAff8` is filled in temporarily while `resume` is running. The fiber
//! started with 0xB000 as a parameter so it knows how to find this.
//! Additionally `resumes` stores state at 0xAff0 to restart execution, and
//! `suspend`, which has 0xB000 so it can find this, will read that and write
//! its own resumption information into this slot as well.

use crate::stackswitch::*;
use crate::{RunResult, RuntimeFiberStack};
use std::boxed::Box;
use std::cell::Cell;
use std::io;
use std::ops::Range;
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};

pub type Error = io::Error;

pub struct FiberStack {
    base: BasePtr,
    len: usize,

    /// Stored here to ensure that when this `FiberStack` the backing storage,
    /// if any, is additionally dropped.
    storage: FiberStackStorage,
}

struct BasePtr(*mut u8);

unsafe impl Send for BasePtr {}
unsafe impl Sync for BasePtr {}

enum FiberStackStorage {
    Mmap(MmapFiberStack),
    Unmanaged(usize),
    Custom(Box<dyn RuntimeFiberStack>),
}

// FIXME: this is a duplicate copy of what's already in the `wasmtime` crate. If
// this changes that should change over there, and ideally one day we should
// probably deduplicate the two.
fn host_page_size() -> usize {
    static PAGE_SIZE: AtomicUsize = AtomicUsize::new(0);

    return match PAGE_SIZE.load(Ordering::Relaxed) {
        0 => {
            let size = unsafe { libc::sysconf(libc::_SC_PAGESIZE).try_into().unwrap() };
            assert!(size != 0);
            PAGE_SIZE.store(size, Ordering::Relaxed);
            size
        }
        n => n,
    };
}

impl FiberStack {
    pub fn new(size: usize, zeroed: bool) -> io::Result<Self> {
        let page_size = host_page_size();
        // The anonymous `mmap`s we use for `FiberStackStorage` are alawys
        // zeroed.
        let _ = zeroed;

        // See comments in `mod asan` below for why asan has a different stack
        // allocation strategy.
        if cfg!(asan) {
            return Self::from_custom(asan::new_fiber_stack(size)?);
        }
        let stack = MmapFiberStack::new(size)?;

        // An `MmapFiberStack` allocates a guard page at the bottom of the
        // region so the base and length of our stack are both offset by a
        // single page.
        Ok(FiberStack {
            base: BasePtr(stack.mapping_base.wrapping_byte_add(page_size)),
            len: stack.mapping_len - page_size,
            storage: FiberStackStorage::Mmap(stack),
        })
    }

    pub unsafe fn from_raw_parts(base: *mut u8, guard_size: usize, len: usize) -> io::Result<Self> {
        // See comments in `mod asan` below for why asan has a different stack
        // allocation strategy.
        if cfg!(asan) {
            return Self::from_custom(asan::new_fiber_stack(len)?);
        }
        Ok(FiberStack {
            base: BasePtr(unsafe { base.add(guard_size) }),
            len,
            storage: FiberStackStorage::Unmanaged(guard_size),
        })
    }

    pub fn is_from_raw_parts(&self) -> bool {
        matches!(self.storage, FiberStackStorage::Unmanaged(_))
    }

    pub fn from_custom(custom: Box<dyn RuntimeFiberStack>) -> io::Result<Self> {
        let range = custom.range();
        let page_size = host_page_size();
        let start_ptr = range.start as *mut u8;
        assert!(
            start_ptr.align_offset(page_size) == 0,
            "expected fiber stack base ({start_ptr:?}) to be page aligned ({page_size:#x})",
        );
        let end_ptr = range.end as *const u8;
        assert!(
            end_ptr.align_offset(page_size) == 0,
            "expected fiber stack end ({end_ptr:?}) to be page aligned ({page_size:#x})",
        );
        Ok(FiberStack {
            base: BasePtr(start_ptr),
            len: range.len(),
            storage: FiberStackStorage::Custom(custom),
        })
    }

    pub fn top(&self) -> Option<*mut u8> {
        Some(self.base.0.wrapping_byte_add(self.len))
    }

    pub fn range(&self) -> Option<Range<usize>> {
        let base = self.base.0 as usize;
        Some(base..base + self.len)
    }

    pub fn guard_range(&self) -> Option<Range<*mut u8>> {
        match &self.storage {
            FiberStackStorage::Unmanaged(guard_size) => unsafe {
                let start = self.base.0.sub(*guard_size);
                Some(start..self.base.0)
            },
            FiberStackStorage::Mmap(mmap) => Some(mmap.mapping_base..self.base.0),
            FiberStackStorage::Custom(custom) => Some(custom.guard_range()),
        }
    }
}

struct MmapFiberStack {
    mapping_base: *mut u8,
    mapping_len: usize,
}

unsafe impl Send for MmapFiberStack {}
unsafe impl Sync for MmapFiberStack {}

impl MmapFiberStack {
    fn new(size: usize) -> io::Result<Self> {
        // Round up our stack size request to the nearest multiple of the
        // page size.
        let page_size = host_page_size();
        let size = if size == 0 {
            page_size
        } else {
            (size + (page_size - 1)) & (!(page_size - 1))
        };

        unsafe {
            // Add in one page for a guard page and then ask for some memory.
            let mmap_len = size + page_size;
            let mmap = rustix::mm::mmap_anonymous(
                ptr::null_mut(),
                mmap_len,
                rustix::mm::ProtFlags::empty(),
                rustix::mm::MapFlags::PRIVATE,
            )?;

            rustix::mm::mprotect(
                mmap.byte_add(page_size),
                size,
                rustix::mm::MprotectFlags::READ | rustix::mm::MprotectFlags::WRITE,
            )?;

            Ok(MmapFiberStack {
                mapping_base: mmap.cast(),
                mapping_len: mmap_len,
            })
        }
    }
}

impl Drop for MmapFiberStack {
    fn drop(&mut self) {
        unsafe {
            let ret = rustix::mm::munmap(self.mapping_base.cast(), self.mapping_len);
            debug_assert!(ret.is_ok());
        }
    }
}

pub struct Fiber;

pub struct Suspend {
    top_of_stack: *mut u8,
    previous: asan::PreviousStack,
}

extern "C" fn fiber_start<F, A, B, C>(arg0: *mut u8, top_of_stack: *mut u8)
where
    F: FnOnce(A, &mut super::Suspend<A, B, C>) -> C,
{
    unsafe {
        // Complete the `start_switch` AddressSanitizer handshake which would
        // have been started in `Fiber::resume`.
        let previous = asan::fiber_start_complete();

        let inner = Suspend {
            top_of_stack,
            previous,
        };
        let initial = inner.take_resume::<A, B, C>();
        super::Suspend::<A, B, C>::execute(inner, initial, Box::from_raw(arg0.cast::<F>()))
    }
}

impl Fiber {
    pub fn new<F, A, B, C>(stack: &FiberStack, func: F) -> io::Result<Self>
    where
        F: FnOnce(A, &mut super::Suspend<A, B, C>) -> C,
    {
        // On unsupported platforms `wasmtime_fiber_init` is a panicking shim so
        // return an error saying the host architecture isn't supported instead.
        if !SUPPORTED_ARCH {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "fibers not supported on this host architecture",
            ));
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

            asan::fiber_switch(
                stack.top().unwrap(),
                false,
                &mut asan::PreviousStack::new(stack),
            );

            // null this out to help catch use-after-free
            addr.write(0);
        }
    }

    pub(crate) unsafe fn drop<A, B, C>(&mut self) {}
}

impl Suspend {
    pub(crate) fn switch<A, B, C>(&mut self, result: RunResult<A, B, C>) -> A {
        unsafe {
            let is_finishing = match &result {
                RunResult::Returned(_) | RunResult::Panicked(_) => true,
                RunResult::Executing | RunResult::Resuming(_) | RunResult::Yield(_) => false,
            };
            // Calculate 0xAff8 and then write to it
            (*self.result_location::<A, B, C>()).set(result);

            asan::fiber_switch(self.top_of_stack, is_finishing, &mut self.previous);

            self.take_resume::<A, B, C>()
        }
    }

    pub(crate) fn exit<A, B, C>(&mut self, result: RunResult<A, B, C>) {
        self.switch(result);
        unreachable!()
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

/// Support for AddressSanitizer to support stack manipulations we do in this
/// fiber implementation.
///
/// This module uses, when fuzzing is enabled, special intrinsics provided by
/// the sanitizer runtime called `__sanitizer_{start,finish}_switch_fiber`.
/// These aren't really super heavily documented and the current implementation
/// is inspired by googling the functions and looking at Boost & Julia's usage
/// of them as well as the documentation for these functions in their own
/// header file in the LLVM source tree. The general idea is that they're
/// called around every stack switch with some other fiddly bits as well.
#[cfg(asan)]
mod asan {
    use super::{FiberStack, MmapFiberStack, RuntimeFiberStack, host_page_size};
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    use std::mem::ManuallyDrop;
    use std::ops::Range;
    use std::sync::Mutex;

    /// State for the "previous stack" maintained by asan itself and fed in for
    /// custom stacks.
    pub struct PreviousStack {
        bottom: *const u8,
        size: usize,
    }

    impl PreviousStack {
        pub fn new(stack: &FiberStack) -> PreviousStack {
            let range = stack.range().unwrap();
            PreviousStack {
                bottom: range.start as *const u8,
                // Discount the two pointers we store at the top of the stack,
                // so subtract two pointers.
                size: range.len() - 2 * std::mem::size_of::<*const u8>(),
            }
        }
    }

    impl Default for PreviousStack {
        fn default() -> PreviousStack {
            PreviousStack {
                bottom: std::ptr::null(),
                size: 0,
            }
        }
    }

    /// Switches the current stack to `top_of_stack`
    ///
    /// * `top_of_stack` - for going to fibers this is calculated and for
    ///   restoring back to the original stack this was saved during the initial
    ///   transition.
    /// * `is_finishing` - whether or not we're switching off a fiber for the
    ///   final time; customizes how asan intrinsics are invoked.
    /// * `prev` - the stack we're switching to initially and saves the
    ///   stack to return to upon resumption.
    pub unsafe fn fiber_switch(
        top_of_stack: *mut u8,
        is_finishing: bool,
        prev: &mut PreviousStack,
    ) {
        assert!(super::SUPPORTED_ARCH);
        let mut private_asan_pointer = std::ptr::null_mut();

        // If this fiber is finishing then NULL is passed to asan to let it know
        // that it can deallocate the "fake stack" that it's tracking for this
        // fiber.
        let private_asan_pointer_ref = if is_finishing {
            None
        } else {
            Some(&mut private_asan_pointer)
        };

        // NB: in fiddling with asan an optimizations and such it appears that
        // these functions need to be "very close to each other". If other Rust
        // functions are invoked or added as an abstraction here that appears to
        // trigger false positives in ASAN. That leads to the design of this
        // module as-is where this function exists to have these three
        // functions very close to one another.
        unsafe {
            __sanitizer_start_switch_fiber(private_asan_pointer_ref, prev.bottom, prev.size);
            super::wasmtime_fiber_switch(top_of_stack);
            __sanitizer_finish_switch_fiber(private_asan_pointer, &mut prev.bottom, &mut prev.size);
        }
    }

    /// Hook for when a fiber first starts, used to configure ASAN.
    pub unsafe fn fiber_start_complete() -> PreviousStack {
        let mut ret = PreviousStack::default();
        unsafe {
            __sanitizer_finish_switch_fiber(std::ptr::null_mut(), &mut ret.bottom, &mut ret.size);
        }
        ret
    }

    // These intrinsics are provided by the address sanitizer runtime. Their C
    // signatures were translated into Rust-isms here with `Option` and `&mut`.
    unsafe extern "C" {
        fn __sanitizer_start_switch_fiber(
            private_asan_pointer_save: Option<&mut *mut u8>,
            bottom: *const u8,
            size: usize,
        );
        fn __sanitizer_finish_switch_fiber(
            private_asan_pointer: *mut u8,
            bottom_old: &mut *const u8,
            size_old: &mut usize,
        );
    }

    /// This static is a workaround for llvm/llvm-project#53891, notably this is
    /// a global cache of all fiber stacks.
    ///
    /// The problem with ASAN is that if we allocate memory for a stack, use it
    /// as a stack, deallocate the stack, and then when that memory is later
    /// mapped as normal heap memory. This is possible due to `mmap` reusing
    /// addresses and it ends up confusing ASAN. In this situation ASAN will
    /// have false positives about stack overflows saying that writes to
    /// freshly-allocated memory, which just happened to historically be a
    /// stack, are a stack overflow.
    ///
    /// This static works around the issue by ensuring that, only when asan is
    /// enabled, all stacks are cached globally. Stacks are never deallocated
    /// and forever retained here. This only works if the number of stacks
    /// retained here is relatively small to prevent OOM from continuously
    /// running programs. That's hopefully the case as ASAN is mostly used in
    /// OSS-Fuzz and our fuzzers only fuzz one thing at a time per thread
    /// meaning that this should only ever be a relatively small set of stacks.
    static FIBER_STACKS: Mutex<Vec<MmapFiberStack>> = Mutex::new(Vec::new());

    pub fn new_fiber_stack(size: usize) -> std::io::Result<Box<dyn RuntimeFiberStack>> {
        let page_size = host_page_size();
        let needed_size = size + page_size;
        let mut stacks = FIBER_STACKS.lock().unwrap();

        let stack = match stacks.iter().position(|i| needed_size <= i.mapping_len) {
            // If an appropriately sized stack was already allocated, then use
            // that one.
            Some(i) => stacks.remove(i),
            // ... otherwise allocate a brand new stack.
            None => MmapFiberStack::new(size)?,
        };
        let stack = AsanFiberStack {
            mmap: ManuallyDrop::new(stack),
        };
        Ok(Box::new(stack))
    }

    /// Custom structure used to prevent the interior mmap-allocated stack from
    /// actually getting unmapped.
    ///
    /// On drop this stack will return the interior stack to the global
    /// `FIBER_STACKS` list.
    struct AsanFiberStack {
        mmap: ManuallyDrop<MmapFiberStack>,
    }

    unsafe impl RuntimeFiberStack for AsanFiberStack {
        fn top(&self) -> *mut u8 {
            self.mmap
                .mapping_base
                .wrapping_byte_add(self.mmap.mapping_len)
        }

        fn range(&self) -> Range<usize> {
            let base = self.mmap.mapping_base as usize;
            let end = base + self.mmap.mapping_len;
            base + host_page_size()..end
        }

        fn guard_range(&self) -> Range<*mut u8> {
            self.mmap.mapping_base..self.mmap.mapping_base.wrapping_add(host_page_size())
        }
    }

    impl Drop for AsanFiberStack {
        fn drop(&mut self) {
            let stack = unsafe { ManuallyDrop::take(&mut self.mmap) };
            FIBER_STACKS.lock().unwrap().push(stack);
        }
    }
}

// Shim module that's the same as above but only has stubs.
#[cfg(not(asan))]
mod asan_disabled {
    use super::{FiberStack, RuntimeFiberStack};
    use std::boxed::Box;

    #[derive(Default)]
    pub struct PreviousStack;

    impl PreviousStack {
        #[inline]
        pub fn new(_stack: &FiberStack) -> PreviousStack {
            PreviousStack
        }
    }

    pub unsafe fn fiber_switch(
        top_of_stack: *mut u8,
        _is_finishing: bool,
        _prev: &mut PreviousStack,
    ) {
        assert!(super::SUPPORTED_ARCH);
        unsafe {
            super::wasmtime_fiber_switch(top_of_stack);
        }
    }

    #[inline]
    pub unsafe fn fiber_start_complete() -> PreviousStack {
        PreviousStack
    }

    pub fn new_fiber_stack(_size: usize) -> std::io::Result<Box<dyn RuntimeFiberStack>> {
        unimplemented!()
    }
}

#[cfg(not(asan))]
use asan_disabled as asan;
