//! Memory management for linear memories.
//!
//! This module implements the runtime data structures that manage linear
//! memories for WebAssembly. There's a number of types here each with various
//! purposes, and this is the high level relationships between types where an
//! arrow here means "builds on top of".
//!
//! ```text
//! ┌─────────────────────┐
//! │                     │
//! │        Memory       ├─────────────┐
//! │                     │             │
//! └──────────┬──────────┘             │
//!            │                        │
//!            │                        │
//!            ▼                        ▼
//! ┌─────────────────────┐     ┌──────────────┐
//! │                     │     │              │
//! │     LocalMemory     │◄────┤ SharedMemory │
//! │                     │     │              │
//! └──────────┬──────────┘     └──────────────┘
//!            │
//!            │
//!            ▼
//! ┌─────────────────────┐
//! │                     │
//! │ RuntimeLinearMemory ├─────────────┬───────────────┐
//! │                     │             │               │
//! └──────────┬──────────┘             │               │
//!            │                        │               │
//!            │                        │               │
//!            ▼                        ▼               ▼
//! ┌─────────────────────┐     ┌──────────────┐     ┌─────┐
//! │                     │     │              │     │     │
//! │      MmapMemory     │     │ StaticMemory │     │ ... │
//! │                     │     │              │     │     │
//! └─────────────────────┘     └──────────────┘     └─────┘
//! ```
//!
//! In more detail:
//!
//! * `Memory` - the root of what's actually stored in a wasm instance. This
//!   implements the high-level embedder APIs one would expect from a wasm
//!   linear memory.
//!
//! * `SharedMemory` - this is one of the variants of a local memory. A shared
//!   memory contains `RwLock<LocalMemory>` where all the real bits happen
//!   within the lock.
//!
//! * `LocalMemory` - this is an owned allocation of a linear memory which
//!   maintains low-level state that's shared between `SharedMemory` and the
//!   instance-local state of `Memory`. One example is that `LocalMemory::grow`
//!   has most of the logic around memory growth.
//!
//! * `RuntimeLinearMemory` - this is a trait which `LocalMemory` delegates to.
//!   This trait is intentionally relatively simple to be exposed in Wasmtime's
//!   embedder API. This is exposed all the way through `wasmtime::Config` so
//!   embedders can provide arbitrary implementations.
//!
//! * `MmapMemory` - this is an implementation of `RuntimeLinearMemory` in terms
//!   of the platform's mmap primitive.
//!
//! * `StaticMemory` - this is an implementation of `RuntimeLinearMemory`
//!   for the pooling allocator where the base pointer is already allocated
//!   and contents are managed through `MemoryImageSlot`.
//!
//! Other important types for memories are `MemoryImage` and `MemoryImageSlot`
//! which manage CoW state for memories. This is implemented at the
//! `LocalMemory` layer.
//!
//! FIXME: don't have both RuntimeLinearMemory and wasmtime::LinearMemory, they
//! should be merged together.
//!
//! FIXME: don't have both RuntimeMemoryCreator and wasmtime::MemoryCreator,
//! they should be merged together.

use crate::prelude::*;
use crate::runtime::vm::vmcontext::VMMemoryDefinition;
#[cfg(has_virtual_memory)]
use crate::runtime::vm::{HostAlignedByteCount, MmapOffset};
use crate::runtime::vm::{MemoryImage, MemoryImageSlot, SendSyncPtr, VMStore};
use alloc::sync::Arc;
use core::{ops::Range, ptr::NonNull};
use wasmtime_environ::Tunables;

#[cfg(feature = "threads")]
use wasmtime_environ::Trap;

#[cfg(has_virtual_memory)]
mod mmap;
#[cfg(has_virtual_memory)]
pub use self::mmap::MmapMemory;

mod malloc;
pub use self::malloc::MallocMemory;

#[cfg(feature = "pooling-allocator")]
mod static_;
#[cfg(feature = "pooling-allocator")]
use self::static_::StaticMemory;

#[cfg(feature = "threads")]
mod shared_memory;
#[cfg(feature = "threads")]
pub use shared_memory::SharedMemory;

#[cfg(not(feature = "threads"))]
mod shared_memory_disabled;
#[cfg(not(feature = "threads"))]
pub use shared_memory_disabled::SharedMemory;

/// A memory allocator
pub trait RuntimeMemoryCreator: Send + Sync {
    /// Create new RuntimeLinearMemory
    fn new_memory(
        &self,
        ty: &wasmtime_environ::Memory,
        tunables: &Tunables,
        minimum: usize,
        maximum: Option<usize>,
    ) -> Result<Box<dyn RuntimeLinearMemory>>;
}

/// A default memory allocator used by Wasmtime
pub struct DefaultMemoryCreator;

impl RuntimeMemoryCreator for DefaultMemoryCreator {
    /// Create new MmapMemory
    fn new_memory(
        &self,
        ty: &wasmtime_environ::Memory,
        tunables: &Tunables,
        minimum: usize,
        maximum: Option<usize>,
    ) -> Result<Box<dyn RuntimeLinearMemory>> {
        #[cfg(has_virtual_memory)]
        if tunables.signals_based_traps
            || tunables.memory_guard_size > 0
            || tunables.memory_reservation > 0
            || tunables.memory_init_cow
        {
            return Ok(Box::new(MmapMemory::new(ty, tunables, minimum, maximum)?));
        }

        let _ = maximum;
        Ok(Box::new(MallocMemory::new(ty, tunables, minimum)?))
    }
}

/// A linear memory and its backing storage.
pub trait RuntimeLinearMemory: Send + Sync {
    /// Returns the number bytes that this linear memory can access.
    fn byte_size(&self) -> usize;

    /// Returns the maximal number of bytes the current allocation can access.
    ///
    /// Growth up to this value should not relocate the base pointer.
    fn byte_capacity(&self) -> usize;

    /// Grow memory to the specified amount of bytes.
    ///
    /// Returns an error if memory can't be grown by the specified amount
    /// of bytes.
    fn grow_to(&mut self, size: usize) -> Result<()>;

    /// Returns a pointer to the base of this linear memory allocation.
    ///
    /// This is either a raw pointer, or a reference to an mmap along with an
    /// offset within it.
    fn base(&self) -> MemoryBase;

    /// Get a `VMMemoryDefinition` for this linear memory.
    fn vmmemory(&self) -> VMMemoryDefinition;

    /// Internal method for Wasmtime when used in conjunction with CoW images.
    /// This is used to inform the underlying memory that the size of memory has
    /// changed.
    ///
    /// Note that this is hidden and panics by default as embedders using custom
    /// memory without CoW images shouldn't have to worry about this.
    #[doc(hidden)]
    fn set_byte_size(&mut self, len: usize) {
        let _ = len;
        panic!("CoW images used with this memory and it doesn't support it");
    }
}

/// The base pointer of a memory allocation.
#[derive(Clone, Debug)]
pub enum MemoryBase {
    /// A raw pointer into memory.
    ///
    /// This may or may not be host-page-aligned.
    Raw(SendSyncPtr<u8>),

    /// An mmap along with an offset into it.
    #[cfg(has_virtual_memory)]
    Mmap(MmapOffset),
}

impl MemoryBase {
    /// Creates a new `MemoryBase` from a raw pointer.
    ///
    /// The pointer must be non-null, and it must be logically `Send + Sync`.
    pub fn new_raw(ptr: *mut u8) -> Self {
        Self::Raw(NonNull::new(ptr).expect("pointer is non-null").into())
    }

    /// Returns the actual memory address in memory that is represented by this
    /// base.
    pub fn as_non_null(&self) -> NonNull<u8> {
        match self {
            Self::Raw(ptr) => ptr.as_non_null(),
            #[cfg(has_virtual_memory)]
            Self::Mmap(mmap_offset) => mmap_offset.as_non_null(),
        }
    }

    /// Same as `as_non_null`, but different return type.
    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.as_non_null().as_ptr()
    }
}

/// Representation of a runtime wasm linear memory.
pub enum Memory {
    Local(LocalMemory),
    Shared(SharedMemory),
}

impl Memory {
    /// Create a new dynamic (movable) memory instance for the specified plan.
    pub fn new_dynamic(
        ty: &wasmtime_environ::Memory,
        tunables: &Tunables,
        creator: &dyn RuntimeMemoryCreator,
        store: &mut dyn VMStore,
        memory_image: Option<&Arc<MemoryImage>>,
    ) -> Result<Self> {
        let (minimum, maximum) = Self::limit_new(ty, Some(store))?;
        let allocation = creator.new_memory(ty, tunables, minimum, maximum)?;

        let memory = LocalMemory::new(ty, tunables, allocation, memory_image)?;
        Ok(if ty.shared {
            Memory::Shared(SharedMemory::wrap(ty, memory)?)
        } else {
            Memory::Local(memory)
        })
    }

    /// Create a new static (immovable) memory instance for the specified plan.
    #[cfg(feature = "pooling-allocator")]
    pub fn new_static(
        ty: &wasmtime_environ::Memory,
        tunables: &Tunables,
        base: MemoryBase,
        base_capacity: usize,
        memory_image: MemoryImageSlot,
        store: &mut dyn VMStore,
    ) -> Result<Self> {
        let (minimum, maximum) = Self::limit_new(ty, Some(store))?;
        let pooled_memory = StaticMemory::new(base, base_capacity, minimum, maximum)?;
        let allocation = Box::new(pooled_memory);

        // Configure some defaults a bit differently for this memory within the
        // `LocalMemory` structure created, notably we already have
        // `memory_image` and regardless of configuration settings this memory
        // can't move its base pointer since it's a fixed allocation.
        let mut memory = LocalMemory::new(ty, tunables, allocation, None)?;
        assert!(memory.memory_image.is_none());
        memory.memory_image = Some(memory_image);
        memory.memory_may_move = false;

        Ok(if ty.shared {
            // FIXME(#4244): not supported with the pooling allocator (which
            // `new_static` is always used with), see `MemoryPool::validate` as
            // well).
            todo!("using shared memory with the pooling allocator is a work in progress");
        } else {
            Memory::Local(memory)
        })
    }

    /// Calls the `store`'s limiter to optionally prevent a memory from being allocated.
    ///
    /// Returns a tuple of the minimum size, optional maximum size, and log(page
    /// size) of the memory, all in bytes.
    pub(crate) fn limit_new(
        ty: &wasmtime_environ::Memory,
        store: Option<&mut dyn VMStore>,
    ) -> Result<(usize, Option<usize>)> {
        let page_size = usize::try_from(ty.page_size()).unwrap();

        // This is the absolute possible maximum that the module can try to
        // allocate, which is our entire address space minus a wasm page. That
        // shouldn't ever actually work in terms of an allocation because
        // presumably the kernel wants *something* for itself, but this is used
        // to pass to the `store`'s limiter for a requested size
        // to approximate the scale of the request that the wasm module is
        // making. This is necessary because the limiter works on `usize` bytes
        // whereas we're working with possibly-overflowing `u64` calculations
        // here. To actually faithfully represent the byte requests of modules
        // we'd have to represent things as `u128`, but that's kinda
        // overkill for this purpose.
        let absolute_max = 0usize.wrapping_sub(page_size);

        // If the minimum memory size overflows the size of our own address
        // space, then we can't satisfy this request, but defer the error to
        // later so the `store` can be informed that an effective oom is
        // happening.
        let minimum = ty
            .minimum_byte_size()
            .ok()
            .and_then(|m| usize::try_from(m).ok());

        // The plan stores the maximum size in units of wasm pages, but we
        // use units of bytes. Unlike for the `minimum` size we silently clamp
        // the effective maximum size to the limits of what we can track. If the
        // maximum size exceeds `usize` or `u64` then there's no need to further
        // keep track of it as some sort of runtime limit will kick in long
        // before we reach the statically declared maximum size.
        let maximum = ty
            .maximum_byte_size()
            .ok()
            .and_then(|m| usize::try_from(m).ok());

        // Inform the store's limiter what's about to happen. This will let the
        // limiter reject anything if necessary, and this also guarantees that
        // we should call the limiter for all requested memories, even if our
        // `minimum` calculation overflowed. This means that the `minimum` we're
        // informing the limiter is lossy and may not be 100% accurate, but for
        // now the expected uses of limiter means that's ok.
        if let Some(store) = store {
            if !store.memory_growing(0, minimum.unwrap_or(absolute_max), maximum)? {
                bail!(
                    "memory minimum size of {} pages exceeds memory limits",
                    ty.limits.min
                );
            }
        }

        // At this point we need to actually handle overflows, so bail out with
        // an error if we made it this far.
        let minimum = minimum.ok_or_else(|| {
            format_err!(
                "memory minimum size of {} pages exceeds memory limits",
                ty.limits.min
            )
        })?;

        Ok((minimum, maximum))
    }

    /// Returns this memory's page size, in bytes.
    pub fn page_size(&self) -> u64 {
        match self {
            Memory::Local(mem) => mem.page_size(),
            Memory::Shared(mem) => mem.page_size(),
        }
    }

    /// Returns the size of this memory, in bytes.
    pub fn byte_size(&self) -> usize {
        match self {
            Memory::Local(mem) => mem.byte_size(),
            Memory::Shared(mem) => mem.byte_size(),
        }
    }

    /// Returns whether or not this memory needs initialization. It
    /// may not if it already has initial content thanks to a CoW
    /// mechanism.
    pub(crate) fn needs_init(&self) -> bool {
        match self {
            Memory::Local(mem) => mem.needs_init(),
            Memory::Shared(mem) => mem.needs_init(),
        }
    }

    /// Grow memory by the specified amount of wasm pages.
    ///
    /// Returns `None` if memory can't be grown by the specified amount
    /// of wasm pages. Returns `Some` with the old size of memory, in bytes, on
    /// successful growth.
    ///
    /// # Safety
    ///
    /// Resizing the memory can reallocate the memory buffer for dynamic memories.
    /// An instance's `VMContext` may have pointers to the memory's base and will
    /// need to be fixed up after growing the memory.
    ///
    /// Generally, prefer using `InstanceHandle::memory_grow`, which encapsulates
    /// this unsafety.
    ///
    /// Ensure that the provided Store is not used to get access any Memory
    /// which lives inside it.
    pub unsafe fn grow(
        &mut self,
        delta_pages: u64,
        store: Option<&mut dyn VMStore>,
    ) -> Result<Option<usize>, Error> {
        let result = match self {
            Memory::Local(mem) => mem.grow(delta_pages, store)?,
            Memory::Shared(mem) => mem.grow(delta_pages, store)?,
        };
        match result {
            Some((old, _new)) => Ok(Some(old)),
            None => Ok(None),
        }
    }

    /// Return a `VMMemoryDefinition` for exposing the memory to compiled wasm code.
    pub fn vmmemory(&self) -> VMMemoryDefinition {
        match self {
            Memory::Local(mem) => mem.vmmemory(),
            // `vmmemory()` is used for writing the `VMMemoryDefinition` of a
            // memory into its `VMContext`; this should never be possible for a
            // shared memory because the only `VMMemoryDefinition` for it should
            // be stored in its own `def` field.
            Memory::Shared(_) => unreachable!(),
        }
    }

    /// Consume the memory, returning its [`MemoryImageSlot`] if any is present.
    /// The image should only be present for a subset of memories created with
    /// [`Memory::new_static()`].
    #[cfg(feature = "pooling-allocator")]
    pub fn unwrap_static_image(self) -> MemoryImageSlot {
        match self {
            Memory::Local(mem) => mem.unwrap_static_image(),
            Memory::Shared(_) => panic!("expected a local memory"),
        }
    }

    /// Is this a shared memory?
    pub fn is_shared_memory(&self) -> bool {
        matches!(self, Memory::Shared(_))
    }

    /// If the [Memory] is a [SharedMemory], unwrap it and return a clone to
    /// that shared memory.
    pub fn as_shared_memory(&self) -> Option<&SharedMemory> {
        match self {
            Memory::Local(_) => None,
            Memory::Shared(mem) => Some(mem),
        }
    }

    /// Implementation of `memory.atomic.notify` for all memories.
    #[cfg(feature = "threads")]
    pub fn atomic_notify(&mut self, addr: u64, count: u32) -> Result<u32, Trap> {
        match self.as_shared_memory() {
            Some(m) => m.atomic_notify(addr, count),
            None => {
                validate_atomic_addr(&self.vmmemory(), addr, 4, 4)?;
                Ok(0)
            }
        }
    }

    /// Implementation of `memory.atomic.wait32` for all memories.
    #[cfg(feature = "threads")]
    pub fn atomic_wait32(
        &mut self,
        addr: u64,
        expected: u32,
        timeout: Option<core::time::Duration>,
    ) -> Result<crate::WaitResult, Trap> {
        match self.as_shared_memory() {
            Some(m) => m.atomic_wait32(addr, expected, timeout),
            None => {
                validate_atomic_addr(&self.vmmemory(), addr, 4, 4)?;
                Err(Trap::AtomicWaitNonSharedMemory)
            }
        }
    }

    /// Implementation of `memory.atomic.wait64` for all memories.
    #[cfg(feature = "threads")]
    pub fn atomic_wait64(
        &mut self,
        addr: u64,
        expected: u64,
        timeout: Option<core::time::Duration>,
    ) -> Result<crate::WaitResult, Trap> {
        match self.as_shared_memory() {
            Some(m) => m.atomic_wait64(addr, expected, timeout),
            None => {
                validate_atomic_addr(&self.vmmemory(), addr, 8, 8)?;
                Err(Trap::AtomicWaitNonSharedMemory)
            }
        }
    }

    /// Returns the range of bytes that WebAssembly should be able to address in
    /// this linear memory. Note that this includes guard pages which wasm can
    /// hit.
    pub fn wasm_accessible(&self) -> Range<usize> {
        match self {
            Memory::Local(mem) => mem.wasm_accessible(),
            Memory::Shared(mem) => mem.wasm_accessible(),
        }
    }
}

/// An owned allocation of a wasm linear memory.
///
/// This might be part of a `Memory` via `Memory::Local` but it might also be
/// the implementation basis for a `SharedMemory` behind an `RwLock` for
/// example.
pub struct LocalMemory {
    alloc: Box<dyn RuntimeLinearMemory>,
    ty: wasmtime_environ::Memory,
    memory_may_move: bool,
    memory_guard_size: usize,
    memory_reservation: usize,

    /// An optional CoW mapping that provides the initial content of this
    /// memory.
    memory_image: Option<MemoryImageSlot>,
}

impl LocalMemory {
    pub fn new(
        ty: &wasmtime_environ::Memory,
        tunables: &Tunables,
        alloc: Box<dyn RuntimeLinearMemory>,
        memory_image: Option<&Arc<MemoryImage>>,
    ) -> Result<LocalMemory> {
        // If a memory image was specified, try to create the MemoryImageSlot on
        // top of our mmap.
        let memory_image = match memory_image {
            #[cfg(has_virtual_memory)]
            Some(image) => {
                // We currently don't support memory_image if
                // `RuntimeLinearMemory::byte_size` is not a multiple of the host page
                // size. See https://github.com/bytecodealliance/wasmtime/issues/9660.
                if let Ok(byte_size) = HostAlignedByteCount::new(alloc.byte_size()) {
                    // memory_image is CoW-based so it is expected to be backed
                    // by an mmap.
                    let mmap_base = match alloc.base() {
                        MemoryBase::Mmap(offset) => offset,
                        MemoryBase::Raw { .. } => {
                            unreachable!("memory_image is Some only for mmap-based memories")
                        }
                    };

                    let mut slot =
                        MemoryImageSlot::create(mmap_base, byte_size, alloc.byte_capacity());
                    slot.instantiate(alloc.byte_size(), Some(image), ty, tunables)?;
                    Some(slot)
                } else {
                    None
                }
            }
            #[cfg(not(has_virtual_memory))]
            Some(_) => unreachable!(),
            None => None,
        };
        Ok(LocalMemory {
            ty: *ty,
            alloc,
            memory_may_move: ty.memory_may_move(tunables),
            memory_image,
            memory_guard_size: tunables.memory_guard_size.try_into().unwrap(),
            memory_reservation: tunables.memory_reservation.try_into().unwrap(),
        })
    }

    pub fn page_size(&self) -> u64 {
        self.ty.page_size()
    }

    /// Grows a memory by `delta_pages`.
    ///
    /// This performs the necessary checks on the growth before delegating to
    /// the underlying `grow_to` implementation.
    ///
    /// The `store` is used only for error reporting.
    pub fn grow(
        &mut self,
        delta_pages: u64,
        mut store: Option<&mut dyn VMStore>,
    ) -> Result<Option<(usize, usize)>, Error> {
        let old_byte_size = self.alloc.byte_size();

        // Wasm spec: when growing by 0 pages, always return the current size.
        if delta_pages == 0 {
            return Ok(Some((old_byte_size, old_byte_size)));
        }

        let page_size = usize::try_from(self.page_size()).unwrap();

        // The largest wasm-page-aligned region of memory is possible to
        // represent in a `usize`. This will be impossible for the system to
        // actually allocate.
        let absolute_max = 0usize.wrapping_sub(page_size);

        // Calculate the byte size of the new allocation. Let it overflow up to
        // `usize::MAX`, then clamp it down to `absolute_max`.
        let new_byte_size = usize::try_from(delta_pages)
            .unwrap_or(usize::MAX)
            .saturating_mul(page_size)
            .saturating_add(old_byte_size)
            .min(absolute_max);

        let maximum = self
            .ty
            .maximum_byte_size()
            .ok()
            .and_then(|n| usize::try_from(n).ok());

        // Store limiter gets first chance to reject memory_growing.
        if let Some(store) = &mut store {
            if !store.memory_growing(old_byte_size, new_byte_size, maximum)? {
                return Ok(None);
            }
        }

        // Save the original base pointer to assert the invariant that growth up
        // to the byte capacity never relocates the base pointer.
        let base_ptr_before = self.alloc.base().as_mut_ptr();
        let required_to_not_move_memory = new_byte_size <= self.alloc.byte_capacity();

        let result = (|| -> Result<()> {
            // Never exceed maximum, even if limiter permitted it.
            if let Some(max) = maximum {
                if new_byte_size > max {
                    bail!("Memory maximum size exceeded");
                }
            }

            // If memory isn't allowed to move then don't let growth happen
            // beyond the initial capacity
            if !self.memory_may_move && new_byte_size > self.alloc.byte_capacity() {
                bail!("Memory maximum size exceeded");
            }

            // If we have a CoW image overlay then let it manage accessible
            // bytes. Once the heap limit is modified inform the underlying
            // allocation that the size has changed.
            //
            // If the growth is going beyond the size of the heap image then
            // discard it. This should only happen for `MmapMemory` where
            // `no_clear_on_drop` is set so the destructor doesn't do anything.
            // For now be maximally sure about this by asserting that memory can
            // indeed move and that we're on unix. If this wants to run
            // somewhere else like Windows or with other allocations this may
            // need adjusting.
            if let Some(image) = &mut self.memory_image {
                if new_byte_size <= self.alloc.byte_capacity() {
                    image.set_heap_limit(new_byte_size)?;
                    self.alloc.set_byte_size(new_byte_size);
                    return Ok(());
                }
                assert!(cfg!(unix));
                assert!(self.memory_may_move);
                self.memory_image = None;
            }

            // And failing all that fall back to the underlying allocation to
            // grow it.
            self.alloc.grow_to(new_byte_size)
        })();

        match result {
            Ok(()) => {
                // On successful growth double-check that the base pointer
                // didn't move if it shouldn't have.
                if required_to_not_move_memory {
                    assert_eq!(base_ptr_before, self.alloc.base().as_mut_ptr());
                }

                Ok(Some((old_byte_size, new_byte_size)))
            }
            Err(e) => {
                // FIXME: shared memories may not have an associated store to
                // report the growth failure to but the error should not be
                // dropped
                // (https://github.com/bytecodealliance/wasmtime/issues/4240).
                if let Some(store) = store {
                    store.memory_grow_failed(e)?;
                }
                Ok(None)
            }
        }
    }

    pub fn vmmemory(&self) -> VMMemoryDefinition {
        self.alloc.vmmemory()
    }

    pub fn byte_size(&self) -> usize {
        self.alloc.byte_size()
    }

    pub fn needs_init(&self) -> bool {
        match &self.memory_image {
            Some(image) => !image.has_image(),
            None => true,
        }
    }

    pub fn wasm_accessible(&self) -> Range<usize> {
        let base = self.alloc.base().as_mut_ptr() as usize;
        // From the base add:
        //
        // * max(capacity, reservation) -- all memory is guaranteed to have at
        //   least `memory_reservation`, but capacity may go beyond that.
        // * memory_guard_size - wasm is allowed to hit the guard page for
        //   sigsegv for example.
        //
        // and this computes the range that wasm is allowed to load from and
        // deterministically trap or succeed.
        let end =
            base + self.alloc.byte_capacity().max(self.memory_reservation) + self.memory_guard_size;
        base..end
    }

    #[cfg(feature = "pooling-allocator")]
    pub fn unwrap_static_image(self) -> MemoryImageSlot {
        self.memory_image.unwrap()
    }
}

/// In the configurations where bounds checks were elided in JIT code (because
/// we are using static memories with virtual memory guard pages) this manual
/// check is here so we don't segfault from Rust. For other configurations,
/// these checks are required anyways.
#[cfg(feature = "threads")]
pub fn validate_atomic_addr(
    def: &VMMemoryDefinition,
    addr: u64,
    access_size: u64,
    access_alignment: u64,
) -> Result<*mut u8, Trap> {
    debug_assert!(access_alignment.is_power_of_two());
    if !(addr % access_alignment == 0) {
        return Err(Trap::HeapMisaligned);
    }

    let length = u64::try_from(def.current_length()).unwrap();
    if !(addr.saturating_add(access_size) < length) {
        return Err(Trap::MemoryOutOfBounds);
    }

    let addr = usize::try_from(addr).unwrap();
    Ok(def.base.as_ptr().wrapping_add(addr))
}
