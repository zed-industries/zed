//! Implements a memory pool using a single allocated memory slab.
//!
//! The pooling instance allocator maps one large slab of memory in advance and
//! allocates WebAssembly memories from this slab--a [`MemoryPool`]. Each
//! WebAssembly memory is allocated in its own slot (see uses of `index` and
//! [`SlotId`] in this module):
//!
//! ```text
//! ┌──────┬──────┬──────┬──────┬──────┐
//! │Slot 0│Slot 1│Slot 2│Slot 3│......│
//! └──────┴──────┴──────┴──────┴──────┘
//! ```
//!
//! Diving deeper, we note that a [`MemoryPool`] protects Wasmtime from
//! out-of-bounds memory accesses by inserting inaccessible guard regions
//! between memory slots. These guard regions are configured to raise a signal
//! if they are accessed--a WebAssembly out-of-bounds (OOB) memory access. The
//! [`MemoryPool`] documentation has a more detailed chart but one can think of
//! memory slots being laid out like the following:
//!
//! ```text
//! ┌─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┐
//! │Guard│Mem 0│Guard│Mem 1│Guard│Mem 2│.....│Guard│
//! └─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┘
//! ```
//!
//! But we can be more efficient about guard regions: with memory protection
//! keys (MPK) enabled, the interleaved guard regions can be smaller. If we
//! surround a memory with memories from other instances and each instance is
//! protected by different protection keys, the guard region can be smaller AND
//! the pool will still raise a signal on an OOB access. This complicates how we
//! lay out memory slots: we must store memories from the same instance in the
//! same "stripe". Each stripe is protected by a different protection key.
//!
//! This concept, dubbed [ColorGuard] in the original paper, relies on careful
//! calculation of the memory sizes to prevent any "overlapping access" (see
//! [`calculate`]): there are limited protection keys available (15) so the next
//! memory using the same key must be at least as far away as the guard region
//! we would insert otherwise. This ends up looking like the following, where a
//! store for instance 0 (`I0`) "stripes" two memories (`M0` and `M1`) with the
//! same protection key 1 and far enough apart to signal an OOB access:
//!
//! ```text
//! ┌─────┬─────┬─────┬─────┬────────────────┬─────┬─────┬─────┐
//! │.....│I0:M1│.....│.....│.<enough slots>.│I0:M2│.....│.....│
//! ├─────┼─────┼─────┼─────┼────────────────┼─────┼─────┼─────┤
//! │.....│key 1│key 2│key 3│..<more keys>...│key 1│key 2│.....│
//! └─────┴─────┴─────┴─────┴────────────────┴─────┴─────┴─────┘
//! ```
//!
//! [ColorGuard]: https://plas2022.github.io/files/pdf/SegueColorGuard.pdf

use super::{
    MemoryAllocationIndex,
    index_allocator::{MemoryInModule, ModuleAffinityIndexAllocator, SlotId},
};
use crate::prelude::*;
use crate::runtime::vm::{
    CompiledModuleId, InstanceAllocationRequest, InstanceLimits, Memory, MemoryBase,
    MemoryImageSlot, Mmap, MmapOffset, PoolingInstanceAllocatorConfig, mmap::AlignedLength,
};
use crate::{
    MpkEnabled,
    runtime::vm::mpk::{self, ProtectionKey, ProtectionMask},
    vm::HostAlignedByteCount,
};
use std::mem;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use wasmtime_environ::{DefinedMemoryIndex, Module, Tunables};

/// A set of allocator slots.
///
/// The allocated slots can be split by striping them: e.g., with two stripe
/// colors 0 and 1, we would allocate all even slots using stripe 0 and all odd
/// slots using stripe 1.
///
/// This is helpful for the use of protection keys: (a) if a request comes to
/// allocate multiple instances, we can allocate them all from the same stripe
/// and (b) if a store wants to allocate more from the same stripe it can.
#[derive(Debug)]
struct Stripe {
    allocator: ModuleAffinityIndexAllocator,
    pkey: Option<ProtectionKey>,
}

/// Represents a pool of WebAssembly linear memories.
///
/// A linear memory is divided into accessible pages and guard pages. A memory
/// pool contains linear memories: each memory occupies a slot in an
/// allocated slab (i.e., `mapping`):
///
/// ```text
///          layout.max_memory_bytes                 layout.slot_bytes
///                    |                                   |
///              ◄─────┴────►                  ◄───────────┴──────────►
/// ┌───────────┬────────────┬───────────┐     ┌───────────┬───────────┬───────────┐
/// | PROT_NONE |            | PROT_NONE | ... |           | PROT_NONE | PROT_NONE |
/// └───────────┴────────────┴───────────┘     └───────────┴───────────┴───────────┘
/// |           |◄──────────────────┬─────────────────────────────────► ◄────┬────►
/// |           |                   |                                        |
/// mapping     |            `layout.num_slots` memories         layout.post_slab_guard_size
///             |
///   layout.pre_slab_guard_size
/// ```
#[derive(Debug)]
pub struct MemoryPool {
    mapping: Arc<Mmap<AlignedLength>>,
    /// This memory pool is stripe-aware. If using  memory protection keys, this
    /// will contain one stripe per available key; otherwise, a single stripe
    /// with an empty key.
    stripes: Vec<Stripe>,

    /// If using a copy-on-write allocation scheme, the slot management. We
    /// dynamically transfer ownership of a slot to a Memory when in use.
    image_slots: Vec<Mutex<ImageSlot>>,

    /// A description of the various memory sizes used in allocating the
    /// `mapping` slab.
    layout: SlabLayout,

    /// The maximum number of memories that a single core module instance may
    /// use.
    ///
    /// NB: this is needed for validation but does not affect the pool's size.
    memories_per_instance: usize,

    /// How much linear memory, in bytes, to keep resident after resetting for
    /// use with the next instance. This much memory will be `memset` to zero
    /// when a linear memory is deallocated.
    ///
    /// Memory exceeding this amount in the wasm linear memory will be released
    /// with `madvise` back to the kernel.
    ///
    /// Only applicable on Linux.
    pub(super) keep_resident: HostAlignedByteCount,

    /// Keep track of protection keys handed out to initialized stores; this
    /// allows us to round-robin the assignment of stores to stripes.
    next_available_pkey: AtomicUsize,
}

/// The state of memory for each slot in this pool.
#[derive(Debug)]
enum ImageSlot {
    /// This slot is guaranteed to be entirely unmapped.
    ///
    /// This is the initial state of all slots.
    Unmapped,

    /// The state of this slot is unknown.
    ///
    /// This encompasses a number of situations such as:
    ///
    /// * The slot is currently in use.
    /// * The slot was attempted to be in use, but allocation failed.
    /// * The slot was used but not deallocated properly.
    ///
    /// All of these situations are lumped into this one variant indicating
    /// that, at a base level, no knowledge is known about this slot. Using a
    /// slot in this state first requires resetting all memory in this slot by
    /// mapping anonymous memory on top of the entire slot.
    Unknown,

    /// This slot was previously used and `MemoryImageSlot` maintains the state
    /// about what this slot was last configured as.
    ///
    /// Future use of this slot will use `MemoryImageSlot` to continue to
    /// re-instantiate and reuse images and such. This state is entered after
    /// and allocated slot is successfully deallcoated.
    PreviouslyUsed(MemoryImageSlot),
}

impl MemoryPool {
    /// Create a new `MemoryPool`.
    pub fn new(config: &PoolingInstanceAllocatorConfig, tunables: &Tunables) -> Result<Self> {
        if u64::try_from(config.limits.max_memory_size).unwrap() > tunables.memory_reservation {
            bail!(
                "maximum memory size of {:#x} bytes exceeds the configured \
                 memory reservation of {:#x} bytes",
                config.limits.max_memory_size,
                tunables.memory_reservation
            );
        }
        let pkeys = match config.memory_protection_keys {
            MpkEnabled::Auto => {
                if mpk::is_supported() {
                    mpk::keys(config.max_memory_protection_keys)
                } else {
                    &[]
                }
            }
            MpkEnabled::Enable => {
                if mpk::is_supported() {
                    mpk::keys(config.max_memory_protection_keys)
                } else {
                    bail!("mpk is disabled on this system")
                }
            }
            MpkEnabled::Disable => &[],
        };

        // This is a tricky bit of global state: when creating a memory pool
        // that uses memory protection keys, we ensure here that any host code
        // will have access to all keys (i.e., stripes). It's only when we enter
        // the WebAssembly guest code (see `StoreInner::call_hook`) that we
        // enforce which keys/stripes can be accessed. Be forewarned about the
        // assumptions here:
        // - we expect this "allow all" configuration to reset the default
        //   process state (only allow key 0) _before_ any memories are accessed
        // - and we expect no other code (e.g., host-side code) to modify this
        //   global MPK configuration
        if !pkeys.is_empty() {
            mpk::allow(ProtectionMask::all());
        }

        // Create a slab layout and allocate it as a completely inaccessible
        // region to start--`PROT_NONE`.
        let constraints = SlabConstraints::new(&config.limits, tunables, pkeys.len())?;
        let layout = calculate(&constraints)?;
        log::debug!(
            "creating memory pool: {constraints:?} -> {layout:?} (total: {})",
            layout.total_slab_bytes()?
        );
        let mut mapping =
            Mmap::accessible_reserved(HostAlignedByteCount::ZERO, layout.total_slab_bytes()?)
                .context("failed to create memory pool mapping")?;

        // Then, stripe the memory with the available protection keys. This is
        // unnecessary if there is only one stripe color.
        if layout.num_stripes >= 2 {
            let mut cursor = layout.pre_slab_guard_bytes;
            let pkeys = &pkeys[..layout.num_stripes];
            for i in 0..constraints.num_slots {
                let pkey = &pkeys[i % pkeys.len()];
                let region = unsafe {
                    mapping.slice_mut(
                        cursor.byte_count()..cursor.byte_count() + layout.slot_bytes.byte_count(),
                    )
                };
                pkey.protect(region)?;
                cursor = cursor
                    .checked_add(layout.slot_bytes)
                    .context("cursor + slot_bytes overflows")?;
            }
            debug_assert_eq!(
                cursor
                    .checked_add(layout.post_slab_guard_bytes)
                    .context("cursor + post_slab_guard_bytes overflows")?,
                layout.total_slab_bytes()?
            );
        }

        let image_slots: Vec<_> = std::iter::repeat_with(|| Mutex::new(ImageSlot::Unmapped))
            .take(constraints.num_slots)
            .collect();

        let create_stripe = |i| {
            let num_slots = constraints.num_slots / layout.num_stripes
                + usize::from(constraints.num_slots % layout.num_stripes > i);
            let allocator = ModuleAffinityIndexAllocator::new(
                num_slots.try_into().unwrap(),
                config.max_unused_warm_slots,
            );
            Stripe {
                allocator,
                pkey: pkeys.get(i).cloned(),
            }
        };

        debug_assert!(layout.num_stripes > 0);
        let stripes: Vec<_> = (0..layout.num_stripes).map(create_stripe).collect();

        let pool = Self {
            stripes,
            mapping: Arc::new(mapping),
            image_slots,
            layout,
            memories_per_instance: usize::try_from(config.limits.max_memories_per_module).unwrap(),
            keep_resident: HostAlignedByteCount::new_rounded_up(
                config.linear_memory_keep_resident,
            )?,
            next_available_pkey: AtomicUsize::new(0),
        };

        Ok(pool)
    }

    /// Return a protection key that stores can use for requesting new
    pub fn next_available_pkey(&self) -> Option<ProtectionKey> {
        let index = self.next_available_pkey.fetch_add(1, Ordering::SeqCst) % self.stripes.len();
        debug_assert!(
            self.stripes.len() < 2 || self.stripes[index].pkey.is_some(),
            "if we are using stripes, we cannot have an empty protection key"
        );
        self.stripes[index].pkey
    }

    /// Validate whether this memory pool supports the given module.
    pub fn validate_memories(&self, module: &Module) -> Result<()> {
        let memories = module.num_defined_memories();
        if memories > self.memories_per_instance {
            bail!(
                "defined memories count of {} exceeds the per-instance limit of {}",
                memories,
                self.memories_per_instance,
            );
        }

        for (i, memory) in module.memories.iter().skip(module.num_imported_memories) {
            self.validate_memory(memory).with_context(|| {
                format!(
                    "memory index {} is unsupported in this pooling allocator configuration",
                    i.as_u32()
                )
            })?;
        }
        Ok(())
    }

    /// Validate one memory for this pool.
    pub fn validate_memory(&self, memory: &wasmtime_environ::Memory) -> Result<()> {
        let min = memory.minimum_byte_size().with_context(|| {
            format!("memory has a minimum byte size that cannot be represented in a u64",)
        })?;
        if min > u64::try_from(self.layout.max_memory_bytes.byte_count()).unwrap() {
            bail!(
                "memory has a minimum byte size of {} which exceeds the limit of {} bytes",
                min,
                self.layout.max_memory_bytes,
            );
        }
        if memory.shared {
            // FIXME(#4244): since the pooling allocator owns the memory
            // allocation (which is torn down with the instance), that
            // can't be used with shared memory where threads or the host
            // might persist the memory beyond the lifetime of the instance
            // itself.
            bail!("memory is shared which is not supported in the pooling allocator");
        }
        Ok(())
    }

    /// Are zero slots in use right now?
    pub fn is_empty(&self) -> bool {
        self.stripes.iter().all(|s| s.allocator.is_empty())
    }

    /// Allocate a single memory for the given instance allocation request.
    pub fn allocate(
        &self,
        request: &mut InstanceAllocationRequest,
        ty: &wasmtime_environ::Memory,
        tunables: &Tunables,
        memory_index: Option<DefinedMemoryIndex>,
    ) -> Result<(MemoryAllocationIndex, Memory)> {
        let stripe_index = if let Some(pkey) = &request.pkey {
            pkey.as_stripe()
        } else {
            debug_assert!(self.stripes.len() < 2);
            0
        };

        let striped_allocation_index = self.stripes[stripe_index]
            .allocator
            .alloc(memory_index.and_then(|mem_idx| {
                request
                    .runtime_info
                    .unique_id()
                    .map(|id| MemoryInModule(id, mem_idx))
            }))
            .map(|slot| StripedAllocationIndex(u32::try_from(slot.index()).unwrap()))
            .ok_or_else(|| {
                super::PoolConcurrencyLimitError::new(
                    self.stripes[stripe_index].allocator.len(),
                    format!("memory stripe {stripe_index}"),
                )
            })?;
        let allocation_index =
            striped_allocation_index.as_unstriped_slot_index(stripe_index, self.stripes.len());

        match (|| {
            // Double-check that the runtime requirements of the memory are
            // satisfied by the configuration of this pooling allocator. This
            // should be returned as an error through `validate_memory_plans`
            // but double-check here to be sure.
            assert!(
                tunables.memory_reservation + tunables.memory_guard_size
                    <= u64::try_from(self.layout.bytes_to_next_stripe_slot().byte_count()).unwrap()
            );

            let base = self.get_base(allocation_index);
            let base_capacity = self.layout.max_memory_bytes;

            let mut slot = self.take_memory_image_slot(allocation_index)?;
            let image = match memory_index {
                Some(memory_index) => request.runtime_info.memory_image(memory_index)?,
                None => None,
            };
            let initial_size = ty
                .minimum_byte_size()
                .expect("min size checked in validation");

            // If instantiation fails, we can propagate the error
            // upward and drop the slot. This will cause the Drop
            // handler to attempt to map the range with PROT_NONE
            // memory, to reserve the space while releasing any
            // stale mappings. The next use of this slot will then
            // create a new slot that will try to map over
            // this, returning errors as well if the mapping
            // errors persist. The unmap-on-drop is best effort;
            // if it fails, then we can still soundly continue
            // using the rest of the pool and allowing the rest of
            // the process to continue, because we never perform a
            // mmap that would leave an open space for someone
            // else to come in and map something.
            let initial_size = usize::try_from(initial_size).unwrap();
            slot.instantiate(initial_size, image, ty, tunables)?;

            Memory::new_static(
                ty,
                tunables,
                MemoryBase::Mmap(base),
                base_capacity.byte_count(),
                slot,
                unsafe { &mut *request.store.get().unwrap() },
            )
        })() {
            Ok(memory) => Ok((allocation_index, memory)),
            Err(e) => {
                self.stripes[stripe_index]
                    .allocator
                    .free(SlotId(striped_allocation_index.0));
                Err(e)
            }
        }
    }

    /// Deallocate a previously-allocated memory.
    ///
    /// # Safety
    ///
    /// The memory must have been previously allocated from this pool and
    /// assigned the given index, must currently be in an allocated state, and
    /// must never be used again.
    ///
    /// The caller must have already called `clear_and_remain_ready` on the
    /// memory's image and flushed any enqueued decommits for this memory.
    pub unsafe fn deallocate(
        &self,
        allocation_index: MemoryAllocationIndex,
        image: MemoryImageSlot,
    ) {
        self.return_memory_image_slot(allocation_index, image);

        let (stripe_index, striped_allocation_index) =
            StripedAllocationIndex::from_unstriped_slot_index(allocation_index, self.stripes.len());
        self.stripes[stripe_index]
            .allocator
            .free(SlotId(striped_allocation_index.0));
    }

    /// Purging everything related to `module`.
    pub fn purge_module(&self, module: CompiledModuleId) {
        // This primarily means clearing out all of its memory images present in
        // the virtual address space. Go through the index allocator for slots
        // affine to `module` and reset them, freeing up the index when we're
        // done.
        //
        // Note that this is only called when the specified `module` won't be
        // allocated further (the module is being dropped) so this shouldn't hit
        // any sort of infinite loop since this should be the final operation
        // working with `module`.
        //
        // TODO: We are given a module id, but key affinity by pair of module id
        // and defined memory index. We are missing any defined memory index or
        // count of how many memories the module defines here. Therefore, we
        // probe up to the maximum number of memories per instance. This is fine
        // because that maximum is generally relatively small. If this method
        // somehow ever gets hot because of unnecessary probing, we should
        // either pass in the actual number of defined memories for the given
        // module to this method, or keep a side table of all slots that are
        // associated with a module (not just module and memory). The latter
        // would require care to make sure that its maintenance wouldn't be too
        // expensive for normal allocation/free operations.
        for stripe in &self.stripes {
            for i in 0..self.memories_per_instance {
                use wasmtime_environ::EntityRef;
                let memory_index = DefinedMemoryIndex::new(i);
                while let Some(id) = stripe
                    .allocator
                    .alloc_affine_and_clear_affinity(module, memory_index)
                {
                    // Attempt to acquire the `MemoryImageSlot` state for this
                    // slot, and then if we have that try to remove the image,
                    // and then if all that succeeds put the slot back in.
                    //
                    // If anything fails then the slot will be in an "unknown"
                    // state which means that on next use it'll be remapped with
                    // anonymous memory.
                    let index = MemoryAllocationIndex(id.0);
                    if let Ok(mut slot) = self.take_memory_image_slot(index) {
                        if slot.remove_image().is_ok() {
                            self.return_memory_image_slot(index, slot);
                        }
                    }

                    stripe.allocator.free(id);
                }
            }
        }
    }

    fn get_base(&self, allocation_index: MemoryAllocationIndex) -> MmapOffset {
        assert!(allocation_index.index() < self.layout.num_slots);
        let offset = self
            .layout
            .slot_bytes
            .checked_mul(allocation_index.index())
            .and_then(|c| c.checked_add(self.layout.pre_slab_guard_bytes))
            .expect("slot_bytes * index + pre_slab_guard_bytes overflows");
        self.mapping.offset(offset).expect("offset is in bounds")
    }

    /// Take ownership of the given image slot.
    ///
    /// This method is used when a `MemoryAllocationIndex` has been allocated
    /// and the state of the slot needs to be acquired. This will lazily
    /// allocate a `MemoryImageSlot` which describes the current (and possibly
    /// prior) state of the slot.
    ///
    /// During deallocation this structure is passed back to
    /// `return_memory_image_slot`.
    ///
    /// Note that this is a fallible method because using a slot might require
    /// resetting the memory that was previously there. This reset operation
    /// is a fallible operation that may not succeed. If it fails then this
    /// slot cannot be used at this time.
    fn take_memory_image_slot(
        &self,
        allocation_index: MemoryAllocationIndex,
    ) -> Result<MemoryImageSlot> {
        let (maybe_slot, needs_reset) = {
            let mut slot = self.image_slots[allocation_index.index()].lock().unwrap();
            match mem::replace(&mut *slot, ImageSlot::Unknown) {
                ImageSlot::Unmapped => (None, false),
                ImageSlot::Unknown => (None, true),
                ImageSlot::PreviouslyUsed(state) => (Some(state), false),
            }
        };
        let mut slot = maybe_slot.unwrap_or_else(|| {
            MemoryImageSlot::create(
                self.get_base(allocation_index),
                HostAlignedByteCount::ZERO,
                self.layout.max_memory_bytes.byte_count(),
            )
        });

        // For `Unknown` slots it means that `slot` is brand new and isn't
        // actually tracking the state of the previous slot, so reset it
        // entirely with anonymous memory to wipe the slate clean and start
        // from zero. This should only happen if allocation of the previous
        // slot failed, for example.
        if needs_reset {
            slot.reset_with_anon_memory()?;
        }
        Ok(slot)
    }

    /// Return ownership of the given image slot.
    fn return_memory_image_slot(
        &self,
        allocation_index: MemoryAllocationIndex,
        slot: MemoryImageSlot,
    ) {
        assert!(!slot.is_dirty());

        let prev = mem::replace(
            &mut *self.image_slots[allocation_index.index()].lock().unwrap(),
            ImageSlot::PreviouslyUsed(slot),
        );
        assert!(matches!(prev, ImageSlot::Unknown));
    }
}

/// The index of a memory allocation within an `InstanceAllocator`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct StripedAllocationIndex(u32);

impl StripedAllocationIndex {
    fn from_unstriped_slot_index(
        index: MemoryAllocationIndex,
        num_stripes: usize,
    ) -> (usize, Self) {
        let stripe_index = index.index() % num_stripes;
        let num_stripes: u32 = num_stripes.try_into().unwrap();
        let index_within_stripe = Self(index.0 / num_stripes);
        (stripe_index, index_within_stripe)
    }

    fn as_unstriped_slot_index(self, stripe: usize, num_stripes: usize) -> MemoryAllocationIndex {
        let num_stripes: u32 = num_stripes.try_into().unwrap();
        let stripe: u32 = stripe.try_into().unwrap();
        MemoryAllocationIndex(self.0 * num_stripes + stripe)
    }
}

#[derive(Clone, Debug)]
struct SlabConstraints {
    /// Essentially, the `static_memory_bound`: this is an assumption that the
    /// runtime and JIT compiler make about how much space will be guarded
    /// between slots.
    expected_slot_bytes: HostAlignedByteCount,
    /// The maximum size of any memory in the pool. Always a non-zero multiple
    /// of the page size.
    max_memory_bytes: HostAlignedByteCount,
    num_slots: usize,
    num_pkeys_available: usize,
    guard_bytes: HostAlignedByteCount,
    guard_before_slots: bool,
}

impl SlabConstraints {
    fn new(
        limits: &InstanceLimits,
        tunables: &Tunables,
        num_pkeys_available: usize,
    ) -> Result<Self> {
        // `memory_reservation` is the configured number of bytes for a
        // static memory slot (see `Config::memory_reservation`); even
        // if the memory never grows to this size (e.g., it has a lower memory
        // maximum), codegen will assume that this unused memory is mapped
        // `PROT_NONE`. Typically `memory_reservation` is 4GiB which helps
        // elide most bounds checks. `MemoryPool` must respect this bound,
        // though not explicitly: if we can achieve the same effect via
        // MPK-protected stripes, the slot size can be lower than the
        // `memory_reservation`.
        let expected_slot_bytes =
            HostAlignedByteCount::new_rounded_up_u64(tunables.memory_reservation)
                .context("memory reservation is too large")?;

        // Page-align the maximum size of memory since that's the granularity that
        // permissions are going to be controlled at.
        let max_memory_bytes = HostAlignedByteCount::new_rounded_up(limits.max_memory_size)
            .context("maximum size of memory is too large")?;

        let guard_bytes = HostAlignedByteCount::new_rounded_up_u64(tunables.memory_guard_size)
            .context("guard region is too large")?;

        let num_slots = limits
            .total_memories
            .try_into()
            .context("too many memories")?;

        let constraints = SlabConstraints {
            max_memory_bytes,
            num_slots,
            expected_slot_bytes,
            num_pkeys_available,
            guard_bytes,
            guard_before_slots: tunables.guard_before_linear_memory,
        };
        Ok(constraints)
    }
}

#[derive(Debug)]
struct SlabLayout {
    /// The total number of slots available in the memory pool slab.
    num_slots: usize,
    /// The size of each slot in the memory pool; this contains the maximum
    /// memory size (i.e., from WebAssembly or Wasmtime configuration) plus any
    /// guard region after the memory to catch OOB access. On these guard
    /// regions, note that:
    /// - users can configure how aggressively (or not) to elide bounds checks
    ///   via `Config::memory_guard_size` (see also:
    ///   `memory_and_guard_size`)
    /// - memory protection keys can compress the size of the guard region by
    ///   placing slots from a different key (i.e., a stripe) in the guard
    ///   region; this means the slot itself can be smaller and we can allocate
    ///   more of them.
    slot_bytes: HostAlignedByteCount,
    /// The maximum size that can become accessible, in bytes, for each linear
    /// memory. Guaranteed to be a whole number of Wasm pages.
    max_memory_bytes: HostAlignedByteCount,
    /// If necessary, the number of bytes to reserve as a guard region at the
    /// beginning of the slab.
    pre_slab_guard_bytes: HostAlignedByteCount,
    /// Like `pre_slab_guard_bytes`, but at the end of the slab.
    post_slab_guard_bytes: HostAlignedByteCount,
    /// The number of stripes needed in the slab layout.
    num_stripes: usize,
}

impl SlabLayout {
    /// Return the total size of the slab, using the final layout (where `n =
    /// num_slots`):
    ///
    /// ```text
    /// ┌────────────────────┬──────┬──────┬───┬──────┬─────────────────────┐
    /// │pre_slab_guard_bytes│slot 1│slot 2│...│slot n│post_slab_guard_bytes│
    /// └────────────────────┴──────┴──────┴───┴──────┴─────────────────────┘
    /// ```
    fn total_slab_bytes(&self) -> Result<HostAlignedByteCount> {
        self.slot_bytes
            .checked_mul(self.num_slots)
            .and_then(|c| c.checked_add(self.pre_slab_guard_bytes))
            .and_then(|c| c.checked_add(self.post_slab_guard_bytes))
            .context("total size of memory reservation exceeds addressable memory")
    }

    /// Returns the number of Wasm bytes from the beginning of one slot to the
    /// next slot in the same stripe--this is the striped equivalent of
    /// `static_memory_bound`. Recall that between slots of the same stripe we
    /// will see a slot from every other stripe.
    ///
    /// For example, in a 3-stripe pool, this function measures the distance
    /// from the beginning of slot 1 to slot 4, which are of the same stripe:
    ///
    /// ```text
    ///  ◄────────────────────►
    /// ┌────────┬──────┬──────┬────────┬───┐
    /// │*slot 1*│slot 2│slot 3│*slot 4*│...|
    /// └────────┴──────┴──────┴────────┴───┘
    /// ```
    fn bytes_to_next_stripe_slot(&self) -> HostAlignedByteCount {
        self.slot_bytes
            .checked_mul(self.num_stripes)
            .expect("constructor checks that self.slot_bytes * self.num_stripes is in bounds")
    }
}

fn calculate(constraints: &SlabConstraints) -> Result<SlabLayout> {
    let SlabConstraints {
        max_memory_bytes,
        num_slots,
        expected_slot_bytes,
        num_pkeys_available,
        guard_bytes,
        guard_before_slots,
    } = *constraints;

    // If the user specifies a guard region, we always need to allocate a
    // `PROT_NONE` region for it before any memory slots. Recall that we can
    // avoid bounds checks for loads and stores with immediates up to
    // `guard_bytes`, but we rely on Wasmtime to emit bounds checks for any
    // accesses greater than this.
    let pre_slab_guard_bytes = if guard_before_slots {
        guard_bytes
    } else {
        HostAlignedByteCount::ZERO
    };

    // To calculate the slot size, we start with the default configured size and
    // attempt to chip away at this via MPK protection. Note here how we begin
    // to define a slot as "all of the memory and guard region."
    let faulting_region_bytes = expected_slot_bytes
        .max(max_memory_bytes)
        .checked_add(guard_bytes)
        .context("faulting region is too large")?;

    let (num_stripes, slot_bytes) = if guard_bytes == 0 || max_memory_bytes == 0 || num_slots == 0 {
        // In the uncommon case where the memory/guard regions are empty or we don't need any slots , we
        // will not need any stripes: we just lay out the slots back-to-back
        // using a single stripe.
        (1, faulting_region_bytes.byte_count())
    } else if num_pkeys_available < 2 {
        // If we do not have enough protection keys to stripe the memory, we do
        // the same. We can't elide any of the guard bytes because we aren't
        // overlapping guard regions with other stripes...
        (1, faulting_region_bytes.byte_count())
    } else {
        // ...but if we can create at least two stripes, we can use another
        // stripe (i.e., a different pkey) as this slot's guard region--this
        // reduces the guard bytes each slot has to allocate. We must make
        // sure, though, that if the size of that other stripe(s) does not
        // fully cover `guard_bytes`, we keep those around to prevent OOB
        // access.

        // We first calculate the number of stripes we need: we want to
        // minimize this so that there is less chance of a single store
        // running out of slots with its stripe--we need at least two,
        // though. But this is not just an optimization; we need to handle
        // the case when there are fewer slots than stripes. E.g., if our
        // pool is configured with only three slots (`num_memory_slots =
        // 3`), we will run into failures if we attempt to set up more than
        // three stripes.
        let needed_num_stripes = faulting_region_bytes
            .checked_div(max_memory_bytes)
            .expect("if condition above implies max_memory_bytes is non-zero")
            + usize::from(
                faulting_region_bytes
                    .checked_rem(max_memory_bytes)
                    .expect("if condition above implies max_memory_bytes is non-zero")
                    != 0,
            );
        assert!(needed_num_stripes > 0);
        let num_stripes = num_pkeys_available.min(needed_num_stripes).min(num_slots);

        // Next, we try to reduce the slot size by "overlapping" the stripes: we
        // can make slot `n` smaller since we know that slot `n+1` and following
        // are in different stripes and will look just like `PROT_NONE` memory.
        // Recall that codegen expects a guarantee that at least
        // `faulting_region_bytes` will catch OOB accesses via segfaults.
        let needed_slot_bytes = faulting_region_bytes
            .byte_count()
            .checked_div(num_stripes)
            .unwrap_or(faulting_region_bytes.byte_count())
            .max(max_memory_bytes.byte_count());
        assert!(needed_slot_bytes >= max_memory_bytes.byte_count());

        (num_stripes, needed_slot_bytes)
    };

    // The page-aligned slot size; equivalent to `memory_and_guard_size`.
    let slot_bytes =
        HostAlignedByteCount::new_rounded_up(slot_bytes).context("slot size is too large")?;

    // We may need another guard region (like `pre_slab_guard_bytes`) at the end
    // of our slab to maintain our `faulting_region_bytes` guarantee. We could
    // be conservative and just create it as large as `faulting_region_bytes`,
    // but because we know that the last slot's `slot_bytes` make up the first
    // part of that region, we reduce the final guard region by that much.
    let post_slab_guard_bytes = faulting_region_bytes.saturating_sub(slot_bytes);

    // Check that we haven't exceeded the slab we can calculate given the limits
    // of `usize`.
    let layout = SlabLayout {
        num_slots,
        slot_bytes,
        max_memory_bytes,
        pre_slab_guard_bytes,
        post_slab_guard_bytes,
        num_stripes,
    };
    match layout.total_slab_bytes() {
        Ok(_) => Ok(layout),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    const WASM_PAGE_SIZE: u32 = wasmtime_environ::Memory::DEFAULT_PAGE_SIZE;

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn test_memory_pool() -> Result<()> {
        let pool = MemoryPool::new(
            &PoolingInstanceAllocatorConfig {
                limits: InstanceLimits {
                    total_memories: 5,
                    max_tables_per_module: 0,
                    max_memories_per_module: 3,
                    table_elements: 0,
                    max_memory_size: WASM_PAGE_SIZE as usize,
                    ..Default::default()
                },
                ..Default::default()
            },
            &Tunables {
                memory_reservation: WASM_PAGE_SIZE as u64,
                memory_guard_size: 0,
                ..Tunables::default_host()
            },
        )?;

        assert_eq!(pool.layout.slot_bytes, WASM_PAGE_SIZE as usize);
        assert_eq!(pool.layout.num_slots, 5);
        assert_eq!(pool.layout.max_memory_bytes, WASM_PAGE_SIZE as usize);

        let base = pool.mapping.as_ptr() as usize;

        for i in 0..5 {
            let index = MemoryAllocationIndex(i);
            let ptr = pool.get_base(index).as_mut_ptr();
            assert_eq!(
                ptr as usize - base,
                i as usize * pool.layout.slot_bytes.byte_count()
            );
        }

        Ok(())
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_pooling_allocator_striping() {
        if !mpk::is_supported() {
            println!("skipping `test_pooling_allocator_striping` test; mpk is not supported");
            return;
        }

        // Force the use of MPK.
        let config = PoolingInstanceAllocatorConfig {
            memory_protection_keys: MpkEnabled::Enable,
            ..PoolingInstanceAllocatorConfig::default()
        };
        let pool = MemoryPool::new(&config, &Tunables::default_host()).unwrap();
        assert!(pool.stripes.len() >= 2);

        let max_memory_slots = config.limits.total_memories;
        dbg!(pool.stripes[0].allocator.num_empty_slots());
        dbg!(pool.stripes[1].allocator.num_empty_slots());
        let available_memory_slots: usize = pool
            .stripes
            .iter()
            .map(|s| s.allocator.num_empty_slots())
            .sum();
        assert_eq!(
            max_memory_slots,
            u32::try_from(available_memory_slots).unwrap()
        );
    }

    #[test]
    fn check_known_layout_calculations() {
        for num_pkeys_available in 0..16 {
            for num_memory_slots in [0, 1, 10, 64] {
                for expected_slot_bytes in [0, 1 << 30 /* 1GB */, 4 << 30 /* 4GB */] {
                    let expected_slot_bytes =
                        HostAlignedByteCount::new(expected_slot_bytes).unwrap();
                    for max_memory_bytes in
                        [0, 1 * WASM_PAGE_SIZE as usize, 10 * WASM_PAGE_SIZE as usize]
                    {
                        // Note new rather than new_rounded_up here -- for now,
                        // WASM_PAGE_SIZE is 64KiB, which is a multiple of the
                        // host page size on all platforms.
                        let max_memory_bytes = HostAlignedByteCount::new(max_memory_bytes).unwrap();
                        for guard_bytes in [0, 2 << 30 /* 2GB */] {
                            let guard_bytes = HostAlignedByteCount::new(guard_bytes).unwrap();
                            for guard_before_slots in [true, false] {
                                let constraints = SlabConstraints {
                                    max_memory_bytes,
                                    num_slots: num_memory_slots,
                                    expected_slot_bytes,
                                    num_pkeys_available,
                                    guard_bytes,
                                    guard_before_slots,
                                };
                                match calculate(&constraints) {
                                    Ok(layout) => {
                                        assert_slab_layout_invariants(constraints, layout)
                                    }
                                    Err(e) => {
                                        // Only allow failure on 32-bit
                                        // platforms where the calculation
                                        // exceeded the size of the address
                                        // space
                                        assert!(
                                            cfg!(target_pointer_width = "32")
                                                && e.to_string()
                                                    .contains("exceeds addressable memory"),
                                            "bad error: {e:?}"
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    proptest! {
        #[test]
        #[cfg_attr(miri, ignore)]
        fn check_random_layout_calculations(c in constraints()) {
            if let Ok(l) = calculate(&c) {
                assert_slab_layout_invariants(c, l);
            }
        }
    }

    fn constraints() -> impl Strategy<Value = SlabConstraints> {
        (
            any::<HostAlignedByteCount>(),
            any::<usize>(),
            any::<HostAlignedByteCount>(),
            any::<usize>(),
            any::<HostAlignedByteCount>(),
            any::<bool>(),
        )
            .prop_map(
                |(
                    max_memory_bytes,
                    num_memory_slots,
                    expected_slot_bytes,
                    num_pkeys_available,
                    guard_bytes,
                    guard_before_slots,
                )| {
                    SlabConstraints {
                        max_memory_bytes,
                        num_slots: num_memory_slots,
                        expected_slot_bytes,
                        num_pkeys_available,
                        guard_bytes,
                        guard_before_slots,
                    }
                },
            )
    }

    fn assert_slab_layout_invariants(c: SlabConstraints, s: SlabLayout) {
        // Check that all the sizes add up.
        assert_eq!(
            s.total_slab_bytes().unwrap(),
            s.pre_slab_guard_bytes
                .checked_add(s.slot_bytes.checked_mul(c.num_slots).unwrap())
                .and_then(|c| c.checked_add(s.post_slab_guard_bytes))
                .unwrap(),
            "the slab size does not add up: {c:?} => {s:?}"
        );
        assert!(
            s.slot_bytes >= s.max_memory_bytes,
            "slot is not big enough: {c:?} => {s:?}"
        );

        // The HostAlignedByteCount newtype wrapper ensures that the various
        // byte values are page-aligned.

        // Check that we use no more or less stripes than needed.
        assert!(s.num_stripes >= 1, "not enough stripes: {c:?} => {s:?}");
        if c.num_pkeys_available == 0 || c.num_slots == 0 {
            assert_eq!(
                s.num_stripes, 1,
                "expected at least one stripe: {c:?} => {s:?}"
            );
        } else {
            assert!(
                s.num_stripes <= c.num_pkeys_available,
                "layout has more stripes than available pkeys: {c:?} => {s:?}"
            );
            assert!(
                s.num_stripes <= c.num_slots,
                "layout has more stripes than memory slots: {c:?} => {s:?}"
            );
        }

        // Check that we use the minimum number of stripes/protection keys.
        // - if the next MPK-protected slot is bigger or the same as the
        //   required guard region, we only need two stripes
        // - if the next slot is smaller than the guard region, we only need
        //   enough stripes to add up to at least that guard region size.
        if c.num_pkeys_available > 1 && !c.max_memory_bytes.is_zero() {
            assert!(
                s.num_stripes <= (c.guard_bytes.checked_div(c.max_memory_bytes).unwrap() + 2),
                "calculated more stripes than needed: {c:?} => {s:?}"
            );
        }

        // Check that the memory-striping will not allow OOB access.
        // - we may have reduced the slot size from `expected_slot_bytes` to
        //   `slot_bytes` assuming MPK striping; we check that our guaranteed
        //   "faulting region" is respected
        // - the last slot won't have MPK striping after it; we check that the
        //   `post_slab_guard_bytes` accounts for this
        assert!(
            s.bytes_to_next_stripe_slot()
                >= c.expected_slot_bytes
                    .max(c.max_memory_bytes)
                    .checked_add(c.guard_bytes)
                    .unwrap(),
            "faulting region not large enough: {c:?} => {s:?}"
        );
        assert!(
            s.slot_bytes.checked_add(s.post_slab_guard_bytes).unwrap() >= c.expected_slot_bytes,
            "last slot may allow OOB access: {c:?} => {s:?}"
        );
    }
}
