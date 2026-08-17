//! Sampler management for DX12.
//!
//! Nearly identical to the Vulkan sampler cache, with added descriptor heap management.

use alloc::vec::Vec;

use hashbrown::{hash_map::Entry, HashMap};

use ordered_float::OrderedFloat;
use parking_lot::Mutex;
use windows::Win32::Graphics::Direct3D12::*;

use crate::dx12::HResult;

/// The index of a sampler in the global sampler heap.
///
/// This is a type-safe, transparent wrapper around a u32.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct SamplerIndex(u32);

/// [`D3D12_SAMPLER_DESC`] is not hashable, so we wrap it in a newtype that is.
///
/// We use [`OrderedFloat`] to allow for floating point values to be compared and
/// hashed in a defined way.
#[derive(Debug, Copy, Clone)]
struct HashableSamplerDesc(D3D12_SAMPLER_DESC);

impl PartialEq for HashableSamplerDesc {
    fn eq(&self, other: &Self) -> bool {
        self.0.Filter == other.0.Filter
            && self.0.AddressU == other.0.AddressU
            && self.0.AddressV == other.0.AddressV
            && self.0.AddressW == other.0.AddressW
            && OrderedFloat(self.0.MipLODBias) == OrderedFloat(other.0.MipLODBias)
            && self.0.MaxAnisotropy == other.0.MaxAnisotropy
            && self.0.ComparisonFunc == other.0.ComparisonFunc
            && self.0.BorderColor.map(OrderedFloat) == other.0.BorderColor.map(OrderedFloat)
            && OrderedFloat(self.0.MinLOD) == OrderedFloat(other.0.MinLOD)
            && OrderedFloat(self.0.MaxLOD) == OrderedFloat(other.0.MaxLOD)
    }
}

impl Eq for HashableSamplerDesc {}

impl core::hash::Hash for HashableSamplerDesc {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.Filter.0.hash(state);
        self.0.AddressU.0.hash(state);
        self.0.AddressV.0.hash(state);
        self.0.AddressW.0.hash(state);
        OrderedFloat(self.0.MipLODBias).hash(state);
        self.0.MaxAnisotropy.hash(state);
        self.0.ComparisonFunc.0.hash(state);
        self.0.BorderColor.map(OrderedFloat).hash(state);
        OrderedFloat(self.0.MinLOD).hash(state);
        OrderedFloat(self.0.MaxLOD).hash(state);
    }
}

/// Entry in the sampler cache.
struct CacheEntry {
    index: SamplerIndex,
    ref_count: u32,
}

/// Container for the mutable management state of the sampler heap.
///
/// We have this separated, using interior mutability, to allow for the outside world
/// to access the heap directly without needing to take the lock.
pub(crate) struct SamplerHeapState {
    /// Mapping from the sampler description to the index within the heap and the refcount.
    mapping: HashMap<HashableSamplerDesc, CacheEntry>,
    /// List of free sampler indices.
    freelist: Vec<SamplerIndex>,
}

/// Global sampler heap for the device.
///
/// As D3D12 only allows 2048 samplers to be in a single heap, we need to cache
/// samplers aggressively and refer to them in shaders by index.
pub(crate) struct SamplerHeap {
    /// Mutable management state of the sampler heap.
    state: Mutex<SamplerHeapState>,

    /// The heap itself.
    heap: ID3D12DescriptorHeap,
    /// The CPU-side handle to the first descriptor in the heap.
    ///
    /// Both the CPU and GPU handles point to the same descriptor, just in
    /// different contexts.
    heap_cpu_start_handle: D3D12_CPU_DESCRIPTOR_HANDLE,
    /// The GPU-side handle to the first descriptor in the heap.
    ///
    /// Both the CPU and GPU handles point to the same descriptor, just in
    /// different contexts.
    heap_gpu_start_handle: D3D12_GPU_DESCRIPTOR_HANDLE,

    /// This is the device-specific size of sampler descriptors.
    descriptor_stride: u32,
}

impl SamplerHeap {
    pub fn new(
        device: &ID3D12Device,
        private_caps: &super::PrivateCapabilities,
    ) -> Result<Self, crate::DeviceError> {
        profiling::scope!("SamplerHeap::new");

        // WARP can report this as 2M or more. We clamp it to 64k to be safe.
        const SAMPLER_HEAP_SIZE_CLAMP: u32 = 64 * 1024;

        let max_unique_samplers = private_caps
            .max_sampler_descriptor_heap_size
            .min(SAMPLER_HEAP_SIZE_CLAMP);

        let desc = D3D12_DESCRIPTOR_HEAP_DESC {
            Type: D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER,
            NumDescriptors: max_unique_samplers,
            Flags: D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE,
            NodeMask: 0,
        };
        let heap = unsafe { device.CreateDescriptorHeap::<ID3D12DescriptorHeap>(&desc) }
            .into_device_result("Failed to create global GPU-Visible Sampler Descriptor Heap")?;

        let heap_cpu_start_handle = unsafe { heap.GetCPUDescriptorHandleForHeapStart() };
        let heap_gpu_start_handle = unsafe { heap.GetGPUDescriptorHandleForHeapStart() };

        let descriptor_stride =
            unsafe { device.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER) };

        Ok(Self {
            state: Mutex::new(SamplerHeapState {
                mapping: HashMap::new(),
                // Reverse so that samplers get allocated starting from zero.
                freelist: (0..max_unique_samplers).map(SamplerIndex).rev().collect(),
            }),
            heap,
            heap_cpu_start_handle,
            heap_gpu_start_handle,
            descriptor_stride,
        })
    }

    /// Returns a reference to the raw descriptor heap.
    pub fn heap(&self) -> &ID3D12DescriptorHeap {
        &self.heap
    }

    /// Returns a reference the handle to be bound to the descriptor table.
    pub fn gpu_descriptor_table(&self) -> D3D12_GPU_DESCRIPTOR_HANDLE {
        self.heap_gpu_start_handle
    }

    /// Add a sampler with the given description to the heap.
    ///
    /// If the sampler already exists, the refcount is incremented and the existing index is returned.
    ///
    /// If the sampler does not exist, a new sampler is created and the index is returned.
    ///
    /// If the heap is full, an error is returned.
    pub fn create_sampler(
        &self,
        device: &ID3D12Device,
        desc: D3D12_SAMPLER_DESC,
    ) -> Result<SamplerIndex, crate::DeviceError> {
        profiling::scope!("SamplerHeap::create_sampler");

        let hashable_desc = HashableSamplerDesc(desc);

        // Eagarly dereference the lock to allow split borrows.
        let state = &mut *self.state.lock();

        // Lookup the sampler in the mapping.
        match state.mapping.entry(hashable_desc) {
            Entry::Occupied(occupied_entry) => {
                // We have found a match, so increment the refcount and return the index.
                let entry = occupied_entry.into_mut();
                entry.ref_count += 1;
                Ok(entry.index)
            }
            Entry::Vacant(vacant_entry) => {
                // We need to create a new sampler.

                // Try to get a new index from the freelist.
                let Some(index) = state.freelist.pop() else {
                    // If the freelist is empty, we have hit the maximum number of samplers.
                    log::error!("There is no more room in the global sampler heap for more unique samplers. Your device supports a maximum of {} unique samplers.", state.mapping.len());
                    return Err(crate::DeviceError::OutOfMemory);
                };

                // Compute the CPU side handle for the new sampler.
                let handle = D3D12_CPU_DESCRIPTOR_HANDLE {
                    ptr: self.heap_cpu_start_handle.ptr
                        + self.descriptor_stride as usize * index.0 as usize,
                };

                unsafe {
                    device.CreateSampler(&desc, handle);
                }

                // Insert the new sampler into the mapping.
                vacant_entry.insert(CacheEntry {
                    index,
                    ref_count: 1,
                });

                Ok(index)
            }
        }
    }

    /// Decrement the refcount of the sampler with the given description.
    ///
    /// If the refcount reaches zero, the sampler is destroyed and the index is returned to the freelist.
    ///
    /// The provided index is checked against the index of the sampler with the given description, ensuring
    /// that there isn't a clerical error from the caller.
    pub fn destroy_sampler(&self, desc: D3D12_SAMPLER_DESC, provided_index: SamplerIndex) {
        profiling::scope!("SamplerHeap::destroy_sampler");

        // Eagarly dereference the lock to allow split borrows.
        let state = &mut *self.state.lock();

        // Get the index of the sampler to destroy.
        let Entry::Occupied(mut hash_map_entry) = state.mapping.entry(HashableSamplerDesc(desc))
        else {
            log::error!(
                "Tried to destroy a sampler that doesn't exist. Sampler description: {desc:#?}"
            );
            return;
        };
        let cache_entry = hash_map_entry.get_mut();

        // Ensure that the provided index matches the index of the sampler to destroy.
        assert_eq!(
            cache_entry.index, provided_index,
            "Mismatched sampler index, this is an implementation bug"
        );

        // Decrement the refcount of the sampler.
        cache_entry.ref_count -= 1;

        // If we are the last reference, remove the sampler from the mapping and return the index to the freelist.
        //
        // As samplers only exist as descriptors in the heap, there is nothing needed to be done to destroy the sampler.
        if cache_entry.ref_count == 0 {
            state.freelist.push(cache_entry.index);
            hash_map_entry.remove();
        }
    }
}
