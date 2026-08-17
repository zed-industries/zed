use {
    alloc::{collections::VecDeque, vec::Vec},
    core::{
        convert::TryFrom as _,
        fmt::{self, Debug, Display},
    },
    gpu_descriptor_types::{
        CreatePoolError, DescriptorDevice, DescriptorPoolCreateFlags, DescriptorTotalCount,
        DeviceAllocationError,
    },
    hashbrown::HashMap,
};

bitflags::bitflags! {
    /// Flags to augment descriptor set allocation.
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub struct DescriptorSetLayoutCreateFlags: u32 {
        /// Specified that descriptor set must be allocated from\
        /// pool with `DescriptorPoolCreateFlags::UPDATE_AFTER_BIND`.
        ///
        /// This flag must be specified when and only when layout was created with matching backend-specific flag,
        /// that allows layout to have UpdateAfterBind bindings.
        const UPDATE_AFTER_BIND = 0x2;
    }
}

/// Descriptor set from allocator.
#[derive(Debug)]
pub struct DescriptorSet<S> {
    raw: S,
    pool_id: u64,
    size: DescriptorTotalCount,
    update_after_bind: bool,
}

impl<S> DescriptorSet<S> {
    /// Returns reference to raw descriptor set.
    pub fn raw(&self) -> &S {
        &self.raw
    }

    /// Returns mutable reference to raw descriptor set.
    ///
    /// # Safety
    ///
    /// Object must not be replaced.
    pub unsafe fn raw_mut(&mut self) -> &mut S {
        &mut self.raw
    }
}

/// AllocationError that may occur during descriptor sets allocation.
#[derive(Debug)]
pub enum AllocationError {
    /// Backend reported that device memory has been exhausted.\
    /// Deallocating device memory or other resources may increase chance
    /// that another allocation would succeed.
    OutOfDeviceMemory,

    /// Backend reported that host memory has been exhausted.\
    /// Deallocating host memory may increase chance that another allocation would succeed.
    OutOfHostMemory,

    /// The total number of descriptors across all pools created\
    /// with flag `CREATE_UPDATE_AFTER_BIND_BIT` set exceeds `max_update_after_bind_descriptors_in_all_pools`
    /// Or fragmentation of the underlying hardware resources occurs.
    Fragmentation,
}

impl Display for AllocationError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AllocationError::OutOfDeviceMemory => fmt.write_str("Device memory exhausted"),
            AllocationError::OutOfHostMemory => fmt.write_str("Host memory exhausted"),
            AllocationError::Fragmentation => fmt.write_str("Fragmentation"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AllocationError {}

impl From<CreatePoolError> for AllocationError {
    fn from(err: CreatePoolError) -> Self {
        match err {
            CreatePoolError::OutOfDeviceMemory => AllocationError::OutOfDeviceMemory,
            CreatePoolError::OutOfHostMemory => AllocationError::OutOfHostMemory,
            CreatePoolError::Fragmentation => AllocationError::Fragmentation,
        }
    }
}

const MIN_SETS: u32 = 64;
const MAX_SETS: u32 = 512;

#[derive(Debug)]
struct DescriptorPool<P> {
    raw: P,

    /// Number of sets allocated from pool.
    allocated: u32,

    /// Expected number of sets available.
    available: u32,
}

#[derive(Debug)]
struct DescriptorBucket<P> {
    offset: u64,
    pools: VecDeque<DescriptorPool<P>>,
    total: u32,
    update_after_bind: bool,
    size: DescriptorTotalCount,
}

impl<P> Drop for DescriptorBucket<P> {
    #[cfg(feature = "tracing")]
    fn drop(&mut self) {
        #[cfg(feature = "std")]
        {
            if std::thread::panicking() {
                return;
            }
        }
        if self.total > 0 {
            tracing::error!("Descriptor sets were not deallocated");
        }
    }

    #[cfg(all(not(feature = "tracing"), feature = "std"))]
    fn drop(&mut self) {
        if std::thread::panicking() {
            return;
        }
        if self.total > 0 {
            eprintln!("Descriptor sets were not deallocated")
        }
    }

    #[cfg(all(not(feature = "tracing"), not(feature = "std")))]
    fn drop(&mut self) {
        if self.total > 0 {
            panic!("Descriptor sets were not deallocated")
        }
    }
}

impl<P> DescriptorBucket<P> {
    fn new(update_after_bind: bool, size: DescriptorTotalCount) -> Self {
        DescriptorBucket {
            offset: 0,
            pools: VecDeque::new(),
            total: 0,
            update_after_bind,
            size,
        }
    }

    fn new_pool_size(&self, minimal_set_count: u32) -> (DescriptorTotalCount, u32) {
        let mut max_sets = MIN_SETS // at least MIN_SETS
            .max(minimal_set_count) // at least enough for allocation
            .max(self.total.min(MAX_SETS)) // at least as much as was allocated so far capped to MAX_SETS
            .checked_next_power_of_two() // rounded up to nearest 2^N
            .unwrap_or(i32::MAX as u32);

        max_sets = (u32::MAX / self.size.sampler.max(1)).min(max_sets);
        max_sets = (u32::MAX / self.size.combined_image_sampler.max(1)).min(max_sets);
        max_sets = (u32::MAX / self.size.sampled_image.max(1)).min(max_sets);
        max_sets = (u32::MAX / self.size.storage_image.max(1)).min(max_sets);
        max_sets = (u32::MAX / self.size.uniform_texel_buffer.max(1)).min(max_sets);
        max_sets = (u32::MAX / self.size.storage_texel_buffer.max(1)).min(max_sets);
        max_sets = (u32::MAX / self.size.uniform_buffer.max(1)).min(max_sets);
        max_sets = (u32::MAX / self.size.storage_buffer.max(1)).min(max_sets);
        max_sets = (u32::MAX / self.size.uniform_buffer_dynamic.max(1)).min(max_sets);
        max_sets = (u32::MAX / self.size.storage_buffer_dynamic.max(1)).min(max_sets);
        max_sets = (u32::MAX / self.size.input_attachment.max(1)).min(max_sets);
        max_sets = (u32::MAX / self.size.acceleration_structure.max(1)).min(max_sets);
        max_sets = (u32::MAX / self.size.inline_uniform_block_bytes.max(1)).min(max_sets);
        max_sets = (u32::MAX / self.size.inline_uniform_block_bindings.max(1)).min(max_sets);

        let mut pool_size = DescriptorTotalCount {
            sampler: self.size.sampler * max_sets,
            combined_image_sampler: self.size.combined_image_sampler * max_sets,
            sampled_image: self.size.sampled_image * max_sets,
            storage_image: self.size.storage_image * max_sets,
            uniform_texel_buffer: self.size.uniform_texel_buffer * max_sets,
            storage_texel_buffer: self.size.storage_texel_buffer * max_sets,
            uniform_buffer: self.size.uniform_buffer * max_sets,
            storage_buffer: self.size.storage_buffer * max_sets,
            uniform_buffer_dynamic: self.size.uniform_buffer_dynamic * max_sets,
            storage_buffer_dynamic: self.size.storage_buffer_dynamic * max_sets,
            input_attachment: self.size.input_attachment * max_sets,
            acceleration_structure: self.size.acceleration_structure * max_sets,
            inline_uniform_block_bytes: self.size.inline_uniform_block_bytes * max_sets,
            inline_uniform_block_bindings: self.size.inline_uniform_block_bindings * max_sets,
        };

        if pool_size == Default::default() {
            pool_size.sampler = 1;
        }

        (pool_size, max_sets)
    }

    unsafe fn allocate<L, S>(
        &mut self,
        device: &impl DescriptorDevice<L, P, S>,
        layout: &L,
        mut count: u32,
        allocated_sets: &mut Vec<DescriptorSet<S>>,
    ) -> Result<(), AllocationError> {
        debug_assert!(usize::try_from(count).is_ok(), "Must be ensured by caller");

        if count == 0 {
            return Ok(());
        }

        for (index, pool) in self.pools.iter_mut().enumerate().rev() {
            if pool.available == 0 {
                continue;
            }

            let allocate = pool.available.min(count);

            #[cfg(feature = "tracing")]
            tracing::trace!("Allocate `{}` sets from exising pool", allocate);

            let result = device.alloc_descriptor_sets(
                &mut pool.raw,
                (0..allocate).map(|_| layout),
                &mut Allocation {
                    size: self.size,
                    update_after_bind: self.update_after_bind,
                    pool_id: index as u64 + self.offset,
                    sets: allocated_sets,
                },
            );

            match result {
                Ok(()) => {}
                Err(DeviceAllocationError::OutOfDeviceMemory) => {
                    return Err(AllocationError::OutOfDeviceMemory)
                }
                Err(DeviceAllocationError::OutOfHostMemory) => {
                    return Err(AllocationError::OutOfHostMemory)
                }
                Err(DeviceAllocationError::FragmentedPool) => {
                    // Should not happen, but better this than panicing.
                    #[cfg(feature = "tracing")]
                    tracing::error!("Unexpectedly failed to allocated descriptor sets due to pool fragmentation");
                    pool.available = 0;
                    continue;
                }
                Err(DeviceAllocationError::OutOfPoolMemory) => {
                    pool.available = 0;
                    continue;
                }
            }

            count -= allocate;
            pool.available -= allocate;
            pool.allocated += allocate;
            self.total += allocate;

            if count == 0 {
                return Ok(());
            }
        }

        while count > 0 {
            let (pool_size, max_sets) = self.new_pool_size(count);
            #[cfg(feature = "tracing")]
            tracing::trace!(
                "Create new pool with {} sets and {:?} descriptors",
                max_sets,
                pool_size,
            );

            let mut raw = device.create_descriptor_pool(
                &pool_size,
                max_sets,
                if self.update_after_bind {
                    DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET
                        | DescriptorPoolCreateFlags::UPDATE_AFTER_BIND
                } else {
                    DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET
                },
            )?;

            let pool_id = self.pools.len() as u64 + self.offset;

            let allocate = max_sets.min(count);
            let result = device.alloc_descriptor_sets(
                &mut raw,
                (0..allocate).map(|_| layout),
                &mut Allocation {
                    pool_id,
                    size: self.size,
                    update_after_bind: self.update_after_bind,
                    sets: allocated_sets,
                },
            );

            match result {
                Ok(()) => {}
                Err(err) => {
                    device.destroy_descriptor_pool(raw);
                    match err {
                        DeviceAllocationError::OutOfDeviceMemory => {
                            return Err(AllocationError::OutOfDeviceMemory)
                        }
                        DeviceAllocationError::OutOfHostMemory => {
                            return Err(AllocationError::OutOfHostMemory)
                        }
                        DeviceAllocationError::FragmentedPool => {
                            // Should not happen, but better this than panicing.
                            #[cfg(feature = "tracing")]
                            tracing::error!("Unexpectedly failed to allocated descriptor sets due to pool fragmentation");
                        }
                        DeviceAllocationError::OutOfPoolMemory => {}
                    }
                    panic!("Failed to allocate descriptor sets from fresh pool");
                }
            }

            count -= allocate;
            self.pools.push_back(DescriptorPool {
                raw,
                allocated: allocate,
                available: max_sets - allocate,
            });
            self.total += allocate;
        }

        Ok(())
    }

    unsafe fn free<L, S>(
        &mut self,
        device: &impl DescriptorDevice<L, P, S>,
        raw_sets: impl IntoIterator<Item = S>,
        pool_id: u64,
    ) {
        let pool = usize::try_from(pool_id - self.offset)
            .ok()
            .and_then(|index| self.pools.get_mut(index))
            .expect("Invalid pool id");

        let mut raw_sets = raw_sets.into_iter();
        let mut count = 0;
        device.dealloc_descriptor_sets(&mut pool.raw, raw_sets.by_ref().inspect(|_| count += 1));

        debug_assert!(
            raw_sets.next().is_none(),
            "Device must deallocated all sets from iterator"
        );

        pool.available += count;
        pool.allocated -= count;
        self.total -= count;
        #[cfg(feature = "tracing")]
        tracing::trace!("Freed {} from descriptor bucket", count);

        while let Some(pool) = self.pools.pop_front() {
            if self.pools.is_empty() || pool.allocated != 0 {
                self.pools.push_front(pool);
                break;
            }

            #[cfg(feature = "tracing")]
            tracing::trace!("Destroying old descriptor pool");

            device.destroy_descriptor_pool(pool.raw);
            self.offset += 1;
        }
    }

    unsafe fn cleanup<L, S>(&mut self, device: &impl DescriptorDevice<L, P, S>) {
        while let Some(pool) = self.pools.pop_front() {
            if pool.allocated != 0 {
                self.pools.push_front(pool);
                break;
            }

            #[cfg(feature = "tracing")]
            tracing::trace!("Destroying old descriptor pool");

            device.destroy_descriptor_pool(pool.raw);
            self.offset += 1;
        }
    }
}

/// Descriptor allocator.
/// Can be used to allocate descriptor sets for any layout.
#[derive(Debug)]
pub struct DescriptorAllocator<P, S> {
    buckets: HashMap<(DescriptorTotalCount, bool), DescriptorBucket<P>>,
    sets_cache: Vec<DescriptorSet<S>>,
    raw_sets_cache: Vec<S>,
    max_update_after_bind_descriptors_in_all_pools: u32,
    current_update_after_bind_descriptors_in_all_pools: u32,
    total: u32,
}

impl<P, S> Drop for DescriptorAllocator<P, S> {
    fn drop(&mut self) {
        if self.buckets.drain().any(|(_, bucket)| bucket.total != 0) {
            #[cfg(feature = "tracing")]
            tracing::error!(
                "`DescriptorAllocator` is dropped while some descriptor sets were not deallocated"
            );
        }
    }
}

impl<P, S> DescriptorAllocator<P, S> {
    /// Create new allocator instance.
    pub fn new(max_update_after_bind_descriptors_in_all_pools: u32) -> Self {
        DescriptorAllocator {
            buckets: HashMap::default(),
            total: 0,
            sets_cache: Vec::new(),
            raw_sets_cache: Vec::new(),
            max_update_after_bind_descriptors_in_all_pools,
            current_update_after_bind_descriptors_in_all_pools: 0,
        }
    }

    /// Allocate descriptor set with specified layout.
    ///
    /// # Safety
    ///
    /// * Same `device` instance must be passed to all method calls of
    ///   one `DescriptorAllocator` instance.
    /// * `flags` must match flags that were used to create the layout.
    /// * `layout_descriptor_count` must match descriptor numbers in the layout.
    pub unsafe fn allocate<L, D>(
        &mut self,
        device: &D,
        layout: &L,
        flags: DescriptorSetLayoutCreateFlags,
        layout_descriptor_count: &DescriptorTotalCount,
        count: u32,
    ) -> Result<Vec<DescriptorSet<S>>, AllocationError>
    where
        S: Debug,
        L: Debug,
        D: DescriptorDevice<L, P, S>,
    {
        if count == 0 {
            return Ok(Vec::new());
        }

        let descriptor_count = count * layout_descriptor_count.total();

        let update_after_bind = flags.contains(DescriptorSetLayoutCreateFlags::UPDATE_AFTER_BIND);

        if update_after_bind
            && self.max_update_after_bind_descriptors_in_all_pools
                - self.current_update_after_bind_descriptors_in_all_pools
                < descriptor_count
        {
            return Err(AllocationError::Fragmentation);
        }

        #[cfg(feature = "tracing")]
        tracing::trace!(
            "Allocating {} sets with layout {:?} @ {:?}",
            count,
            layout,
            layout_descriptor_count
        );

        let bucket = self
            .buckets
            .entry((*layout_descriptor_count, update_after_bind))
            .or_insert_with(|| DescriptorBucket::new(update_after_bind, *layout_descriptor_count));
        match bucket.allocate(device, layout, count, &mut self.sets_cache) {
            Ok(()) => {
                self.total += descriptor_count;
                if update_after_bind {
                    self.current_update_after_bind_descriptors_in_all_pools += descriptor_count;
                }

                Ok(core::mem::take(&mut self.sets_cache))
            }
            Err(err) => {
                debug_assert!(self.raw_sets_cache.is_empty());

                // Free sets allocated so far.
                let mut last = None;

                for set in self.sets_cache.drain(..) {
                    if Some(set.pool_id) != last {
                        if let Some(last_id) = last {
                            // Free contiguous range of sets from one pool in one go.
                            bucket.free(device, self.raw_sets_cache.drain(..), last_id);
                        }
                    }
                    last = Some(set.pool_id);
                    self.raw_sets_cache.push(set.raw);
                }

                if let Some(last_id) = last {
                    bucket.free(device, self.raw_sets_cache.drain(..), last_id);
                }

                Err(err)
            }
        }
    }

    /// Free descriptor sets.
    ///
    /// # Safety
    ///
    /// * Same `device` instance must be passed to all method calls of
    ///   one `DescriptorAllocator` instance.
    /// * None of descriptor sets can be referenced in any pending command buffers.
    /// * All command buffers where at least one of descriptor sets referenced
    ///   move to invalid state.
    pub unsafe fn free<L, D, I>(&mut self, device: &D, sets: I)
    where
        D: DescriptorDevice<L, P, S>,
        I: IntoIterator<Item = DescriptorSet<S>>,
    {
        debug_assert!(self.raw_sets_cache.is_empty());

        let mut last_key = (EMPTY_COUNT, false);
        let mut last_pool_id = None;

        let mut descriptor_count = 0;

        // Batch freeing of adjacent descriptor sets that belong to the same bucket and pool.
        for set in sets {
            descriptor_count += set.size.total();

            if last_key != (set.size, set.update_after_bind) || last_pool_id != Some(set.pool_id) {
                if let Some(pool_id) = last_pool_id {
                    self.free_raw_sets_cache(device, &last_key, pool_id, descriptor_count);
                    descriptor_count = 0;
                }

                last_key = (set.size, set.update_after_bind);
                last_pool_id = Some(set.pool_id);
            }
            self.raw_sets_cache.push(set.raw);
        }

        if let Some(pool_id) = last_pool_id {
            self.free_raw_sets_cache(device, &last_key, pool_id, descriptor_count);
        }
    }

    /// Frees the cached descriptor sets which must be allocated from the same bucket and pool.
    unsafe fn free_raw_sets_cache<L, D>(
        &mut self,
        device: &D,
        bucket_key: &(DescriptorTotalCount, bool),
        pool_id: u64,
        descriptor_count: u32,
    ) where
        D: DescriptorDevice<L, P, S>,
    {
        let bucket = self
            .buckets
            .get_mut(bucket_key)
            .expect("Set must be allocated from this allocator");

        debug_assert!(u32::try_from(self.raw_sets_cache.len())
            .ok()
            .is_some_and(|count| count <= bucket.total));

        bucket.free(device, self.raw_sets_cache.drain(..), pool_id);

        self.total -= descriptor_count;
        if bucket.update_after_bind {
            self.current_update_after_bind_descriptors_in_all_pools -= descriptor_count;
        }
    }

    /// Perform cleanup to allow resources reuse.
    ///
    /// # Safety
    ///
    /// * Same `device` instance must be passed to all method calls of
    ///   one `DescriptorAllocator` instance.
    pub unsafe fn cleanup<L>(&mut self, device: &impl DescriptorDevice<L, P, S>) {
        for bucket in self.buckets.values_mut() {
            bucket.cleanup(device)
        }
        self.buckets.retain(|_, bucket| !bucket.pools.is_empty());
    }
}

/// Empty descriptor per_type.
const EMPTY_COUNT: DescriptorTotalCount = DescriptorTotalCount {
    sampler: 0,
    combined_image_sampler: 0,
    sampled_image: 0,
    storage_image: 0,
    uniform_texel_buffer: 0,
    storage_texel_buffer: 0,
    uniform_buffer: 0,
    storage_buffer: 0,
    uniform_buffer_dynamic: 0,
    storage_buffer_dynamic: 0,
    input_attachment: 0,
    acceleration_structure: 0,
    inline_uniform_block_bytes: 0,
    inline_uniform_block_bindings: 0,
};

struct Allocation<'a, S> {
    update_after_bind: bool,
    size: DescriptorTotalCount,
    pool_id: u64,
    sets: &'a mut Vec<DescriptorSet<S>>,
}

impl<S> Extend<S> for Allocation<'_, S> {
    fn extend<T: IntoIterator<Item = S>>(&mut self, iter: T) {
        let update_after_bind = self.update_after_bind;
        let size = self.size;
        let pool_id = self.pool_id;
        self.sets.extend(iter.into_iter().map(|raw| DescriptorSet {
            raw,
            pool_id,
            update_after_bind,
            size,
        }))
    }
}
