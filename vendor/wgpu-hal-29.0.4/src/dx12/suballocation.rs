use alloc::sync::Arc;

use gpu_allocator::{d3d12::AllocationCreateDesc, MemoryLocation};
use parking_lot::Mutex;
use windows::Win32::Graphics::{Direct3D12, Dxgi};

use crate::{
    auxil::dxgi::{name::ObjectExt as _, result::HResult as _},
    dx12::conv,
    AllocationSizes,
};

#[derive(Debug)]
pub(crate) enum AllocationType {
    Buffer,
    Texture,
    AccelerationStructure,
}

#[derive(Debug)]
enum AllocationInner {
    /// This resource is suballocated from a heap.
    Placed {
        inner: gpu_allocator::d3d12::Allocation,
    },
    /// This resource is a committed resource and does not belong to a
    /// suballocated heap. We store an approximate size, so we can manage our counters
    /// correctly.
    ///
    /// This is only used for Intel Xe drivers, which have a bug that
    /// prevents suballocation from working correctly.
    Committed { size: u64 },
}

#[derive(Debug)]
pub(crate) struct Allocation {
    inner: AllocationInner,
    ty: AllocationType,
}

impl Allocation {
    pub fn placed(inner: gpu_allocator::d3d12::Allocation, ty: AllocationType) -> Self {
        Self {
            inner: AllocationInner::Placed { inner },
            ty,
        }
    }

    pub fn none(ty: AllocationType, size: u64) -> Self {
        Self {
            inner: AllocationInner::Committed { size },
            ty,
        }
    }

    pub fn size(&self) -> u64 {
        match self.inner {
            AllocationInner::Placed { ref inner } => inner.size(),
            AllocationInner::Committed { size } => size,
        }
    }
}

#[derive(Clone)]
pub(crate) struct Allocator {
    inner: Arc<Mutex<gpu_allocator::d3d12::Allocator>>,
    device_memblock_size: u64,
    host_memblock_size: u64,
    pub memory_budget_thresholds: wgt::MemoryBudgetThresholds,
}

impl Allocator {
    pub(crate) fn new(
        raw: &Direct3D12::ID3D12Device,
        memory_hints: &wgt::MemoryHints,
        memory_budget_thresholds: wgt::MemoryBudgetThresholds,
    ) -> Result<Self, crate::DeviceError> {
        let allocation_sizes = AllocationSizes::from_memory_hints(memory_hints);
        let device_memblock_size = allocation_sizes.min_device_memblock_size;
        let host_memblock_size = allocation_sizes.min_host_memblock_size;

        let allocator_desc = gpu_allocator::d3d12::AllocatorCreateDesc {
            device: gpu_allocator::d3d12::ID3D12DeviceVersion::Device(raw.clone()),
            debug_settings: Default::default(),
            allocation_sizes: allocation_sizes.into(),
        };

        let allocator = gpu_allocator::d3d12::Allocator::new(&allocator_desc).inspect_err(|e| {
            log::error!("Failed to create d3d12 allocator, error: {e}");
        })?;

        Ok(Self {
            inner: Arc::new(Mutex::new(allocator)),
            device_memblock_size,
            host_memblock_size,
            memory_budget_thresholds,
        })
    }

    pub(crate) fn generate_report(&self) -> wgt::AllocatorReport {
        let mut upstream = self.inner.lock().generate_report();

        let allocations = upstream
            .allocations
            .iter_mut()
            .map(|alloc| wgt::AllocationReport {
                name: core::mem::take(&mut alloc.name),
                offset: alloc.offset,
                size: alloc.size,
            })
            .collect();

        let blocks = upstream
            .blocks
            .iter()
            .map(|block| wgt::MemoryBlockReport {
                size: block.size,
                allocations: block.allocations.clone(),
            })
            .collect();

        wgt::AllocatorReport {
            allocations,
            blocks,
            total_allocated_bytes: upstream.total_allocated_bytes,
            total_reserved_bytes: upstream.total_capacity_bytes,
        }
    }
}

/// To allow us to construct buffers from both a `Device` and `CommandEncoder`
/// without needing each function to take a million arguments, we create a
/// borrowed context struct that contains the relevant members.
pub(crate) struct DeviceAllocationContext<'a> {
    pub(crate) raw: &'a Direct3D12::ID3D12Device,
    pub(crate) shared: &'a super::DeviceShared,
    pub(crate) mem_allocator: &'a Allocator,
    pub(crate) counters: &'a wgt::HalCounters,
}

impl<'a> From<&'a super::Device> for DeviceAllocationContext<'a> {
    fn from(device: &'a super::Device) -> Self {
        Self {
            raw: &device.raw,
            shared: &device.shared,
            mem_allocator: &device.mem_allocator,
            counters: &device.counters,
        }
    }
}

impl<'a> From<&'a super::CommandEncoder> for DeviceAllocationContext<'a> {
    fn from(encoder: &'a super::CommandEncoder) -> Self {
        Self {
            raw: &encoder.device,
            shared: &encoder.shared,
            mem_allocator: &encoder.mem_allocator,
            counters: &encoder.counters,
        }
    }
}

impl<'a> DeviceAllocationContext<'a> {
    ///////////////////////
    // Resource Creation //
    ///////////////////////

    pub(crate) fn create_buffer(
        &self,
        desc: &crate::BufferDescriptor,
    ) -> Result<(Direct3D12::ID3D12Resource, Allocation), crate::DeviceError> {
        let is_cpu_read = desc.usage.contains(wgt::BufferUses::MAP_READ);
        let is_cpu_write = desc.usage.contains(wgt::BufferUses::MAP_WRITE);

        let location = match (is_cpu_read, is_cpu_write) {
            (true, true) => MemoryLocation::CpuToGpu,
            (true, false) => MemoryLocation::GpuToCpu,
            (false, true) => MemoryLocation::CpuToGpu,
            (false, false) => MemoryLocation::GpuOnly,
        };

        let raw_desc = conv::map_buffer_descriptor(desc);
        let allocation_info =
            self.error_if_would_oom_on_resource_allocation(&raw_desc, location)?;

        let (resource, allocation) = if self.shared.private_caps.suballocation_supported {
            self.create_placed_buffer(desc, raw_desc, allocation_info, location)?
        } else {
            self.create_committed_buffer(raw_desc, location)?
        };

        if let Some(label) = desc.label {
            resource.set_name(label)?;
        }

        self.counters.buffer_memory.add(allocation.size() as isize);

        Ok((resource, allocation))
    }

    pub(crate) fn create_texture(
        &self,
        desc: &crate::TextureDescriptor,
        raw_desc: Direct3D12::D3D12_RESOURCE_DESC,
    ) -> Result<(Direct3D12::ID3D12Resource, Allocation), crate::DeviceError> {
        let location = MemoryLocation::GpuOnly;
        let allocation_info =
            self.error_if_would_oom_on_resource_allocation(&raw_desc, location)?;

        let (resource, allocation) = if self.shared.private_caps.suballocation_supported {
            self.create_placed_texture(desc, raw_desc, allocation_info, location)?
        } else {
            self.create_committed_texture(desc, raw_desc)?
        };

        if let Some(label) = desc.label {
            resource.set_name(label)?;
        }

        self.counters.texture_memory.add(allocation.size() as isize);

        Ok((resource, allocation))
    }

    pub(crate) fn create_acceleration_structure(
        &self,
        desc: &crate::AccelerationStructureDescriptor,
        raw_desc: Direct3D12::D3D12_RESOURCE_DESC,
    ) -> Result<(Direct3D12::ID3D12Resource, Allocation), crate::DeviceError> {
        let location = MemoryLocation::GpuOnly;
        let allocation_info =
            self.error_if_would_oom_on_resource_allocation(&raw_desc, location)?;

        let (resource, allocation) = if self.shared.private_caps.suballocation_supported {
            self.create_placed_acceleration_structure(desc, raw_desc, allocation_info, location)?
        } else {
            self.create_committed_acceleration_structure(desc, raw_desc)?
        };

        if let Some(label) = desc.label {
            resource.set_name(label)?;
        }

        self.counters
            .acceleration_structure_memory
            .add(allocation.size() as isize);

        Ok((resource, allocation))
    }

    //////////////////////////
    // Resource Destruction //
    //////////////////////////

    pub(crate) fn free_resource(
        &self,
        resource: Direct3D12::ID3D12Resource,
        allocation: Allocation,
    ) {
        // Make sure the resource is released before we free the allocation.
        drop(resource);

        let counter = match allocation.ty {
            AllocationType::Buffer => &self.counters.buffer_memory,
            AllocationType::Texture => &self.counters.texture_memory,
            AllocationType::AccelerationStructure => &self.counters.acceleration_structure_memory,
        };
        counter.sub(allocation.size() as isize);

        if let AllocationInner::Placed { inner } = allocation.inner {
            match self.mem_allocator.inner.lock().free(inner) {
                Ok(_) => (),
                // TODO: Don't panic here
                Err(e) => panic!("Failed to destroy dx12 {:?}, {e}", allocation.ty),
            };
        }
    }

    ///////////////////////////////
    // Placed Resource Creation ///
    ///////////////////////////////

    fn create_placed_buffer(
        &self,
        desc: &crate::BufferDescriptor<'_>,
        raw_desc: Direct3D12::D3D12_RESOURCE_DESC,
        allocation_info: Direct3D12::D3D12_RESOURCE_ALLOCATION_INFO,
        location: MemoryLocation,
    ) -> Result<(Direct3D12::ID3D12Resource, Allocation), crate::DeviceError> {
        let name = desc.label.unwrap_or("Unlabeled buffer");

        let mut allocator = self.mem_allocator.inner.lock();

        let allocation_desc = AllocationCreateDesc {
            name,
            location,
            size: allocation_info.SizeInBytes,
            alignment: allocation_info.Alignment,
            resource_category: gpu_allocator::d3d12::ResourceCategory::from(&raw_desc),
        };

        let allocation = allocator.allocate(&allocation_desc)?;
        let mut resource = None;
        unsafe {
            self.raw.CreatePlacedResource(
                allocation.heap(),
                allocation.offset(),
                &raw_desc,
                Direct3D12::D3D12_RESOURCE_STATE_COMMON,
                None,
                &mut resource,
            )
        }
        .into_device_result("Placed buffer creation")?;

        let resource = resource.ok_or(crate::DeviceError::Unexpected)?;
        let wrapped_allocation = Allocation::placed(allocation, AllocationType::Buffer);

        Ok((resource, wrapped_allocation))
    }

    fn create_placed_texture(
        &self,
        desc: &crate::TextureDescriptor<'_>,
        raw_desc: Direct3D12::D3D12_RESOURCE_DESC,
        allocation_info: Direct3D12::D3D12_RESOURCE_ALLOCATION_INFO,
        location: MemoryLocation,
    ) -> Result<(Direct3D12::ID3D12Resource, Allocation), crate::DeviceError> {
        let name = desc.label.unwrap_or("Unlabeled texture");

        let mut allocator = self.mem_allocator.inner.lock();

        let allocation_desc = AllocationCreateDesc {
            name,
            location,
            size: allocation_info.SizeInBytes,
            alignment: allocation_info.Alignment,
            resource_category: gpu_allocator::d3d12::ResourceCategory::from(&raw_desc),
        };

        let allocation = allocator.allocate(&allocation_desc)?;
        let mut resource = None;
        unsafe {
            self.raw.CreatePlacedResource(
                allocation.heap(),
                allocation.offset(),
                &raw_desc,
                Direct3D12::D3D12_RESOURCE_STATE_COMMON,
                None, // clear value
                &mut resource,
            )
        }
        .into_device_result("Placed texture creation")?;

        let resource = resource.ok_or(crate::DeviceError::Unexpected)?;
        let wrapped_allocation = Allocation::placed(allocation, AllocationType::Texture);

        Ok((resource, wrapped_allocation))
    }

    fn create_placed_acceleration_structure(
        &self,
        desc: &crate::AccelerationStructureDescriptor<'_>,
        raw_desc: Direct3D12::D3D12_RESOURCE_DESC,
        allocation_info: Direct3D12::D3D12_RESOURCE_ALLOCATION_INFO,
        location: MemoryLocation,
    ) -> Result<(Direct3D12::ID3D12Resource, Allocation), crate::DeviceError> {
        let name = desc.label.unwrap_or("Unlabeled acceleration structure");

        let mut allocator = self.mem_allocator.inner.lock();

        let allocation_desc = AllocationCreateDesc {
            name,
            location,
            size: allocation_info.SizeInBytes,
            alignment: allocation_info.Alignment,
            resource_category: gpu_allocator::d3d12::ResourceCategory::from(&raw_desc),
        };

        let allocation = allocator.allocate(&allocation_desc)?;
        let mut resource = None;
        unsafe {
            self.raw.CreatePlacedResource(
                allocation.heap(),
                allocation.offset(),
                &raw_desc,
                Direct3D12::D3D12_RESOURCE_STATE_RAYTRACING_ACCELERATION_STRUCTURE,
                None,
                &mut resource,
            )
        }
        .into_device_result("Placed acceleration structure creation")?;

        let resource = resource.ok_or(crate::DeviceError::Unexpected)?;
        let wrapped_allocation =
            Allocation::placed(allocation, AllocationType::AccelerationStructure);

        Ok((resource, wrapped_allocation))
    }

    /////////////////////////////////
    // Committed Resource Creation //
    /////////////////////////////////

    fn create_committed_buffer(
        &self,
        raw_desc: Direct3D12::D3D12_RESOURCE_DESC,
        location: MemoryLocation,
    ) -> Result<(Direct3D12::ID3D12Resource, Allocation), crate::DeviceError> {
        let is_uma = matches!(
            self.shared.private_caps.memory_architecture,
            crate::dx12::MemoryArchitecture::Unified { .. }
        );

        let heap_properties = Direct3D12::D3D12_HEAP_PROPERTIES {
            Type: Direct3D12::D3D12_HEAP_TYPE_CUSTOM,
            CPUPageProperty: match location {
                MemoryLocation::GpuOnly => Direct3D12::D3D12_CPU_PAGE_PROPERTY_NOT_AVAILABLE,
                MemoryLocation::CpuToGpu => Direct3D12::D3D12_CPU_PAGE_PROPERTY_WRITE_COMBINE,
                MemoryLocation::GpuToCpu => Direct3D12::D3D12_CPU_PAGE_PROPERTY_WRITE_BACK,
                _ => unreachable!(),
            },
            MemoryPoolPreference: match (is_uma, location) {
                // On dedicated GPUs, we only use L1 for GPU-only allocations.
                (false, MemoryLocation::GpuOnly) => Direct3D12::D3D12_MEMORY_POOL_L1,
                (_, _) => Direct3D12::D3D12_MEMORY_POOL_L0,
            },
            CreationNodeMask: 0,
            VisibleNodeMask: 0,
        };

        let mut resource = None;

        unsafe {
            self.raw.CreateCommittedResource(
                &heap_properties,
                if self.shared.private_caps.heap_create_not_zeroed {
                    Direct3D12::D3D12_HEAP_FLAG_CREATE_NOT_ZEROED
                } else {
                    Direct3D12::D3D12_HEAP_FLAG_NONE
                },
                &raw_desc,
                Direct3D12::D3D12_RESOURCE_STATE_COMMON,
                None,
                &mut resource,
            )
        }
        .into_device_result("Committed buffer creation")?;

        let resource = resource.ok_or(crate::DeviceError::Unexpected)?;
        let wrapped_allocation = Allocation::none(AllocationType::Buffer, raw_desc.Width);

        Ok((resource, wrapped_allocation))
    }

    fn create_committed_texture(
        &self,
        desc: &crate::TextureDescriptor,
        raw_desc: Direct3D12::D3D12_RESOURCE_DESC,
    ) -> Result<(Direct3D12::ID3D12Resource, Allocation), crate::DeviceError> {
        let heap_properties = Direct3D12::D3D12_HEAP_PROPERTIES {
            Type: Direct3D12::D3D12_HEAP_TYPE_CUSTOM,
            CPUPageProperty: Direct3D12::D3D12_CPU_PAGE_PROPERTY_NOT_AVAILABLE,
            MemoryPoolPreference: match self.shared.private_caps.memory_architecture {
                crate::dx12::MemoryArchitecture::NonUnified => Direct3D12::D3D12_MEMORY_POOL_L1,
                crate::dx12::MemoryArchitecture::Unified { .. } => Direct3D12::D3D12_MEMORY_POOL_L0,
            },
            CreationNodeMask: 0,
            VisibleNodeMask: 0,
        };

        let mut resource = None;

        unsafe {
            self.raw.CreateCommittedResource(
                &heap_properties,
                if self.shared.private_caps.heap_create_not_zeroed {
                    Direct3D12::D3D12_HEAP_FLAG_CREATE_NOT_ZEROED
                } else {
                    Direct3D12::D3D12_HEAP_FLAG_NONE
                },
                &raw_desc,
                Direct3D12::D3D12_RESOURCE_STATE_COMMON,
                None, // clear value
                &mut resource,
            )
        }
        .into_device_result("Committed texture creation")?;

        let resource = resource.ok_or(crate::DeviceError::Unexpected)?;
        let wrapped_allocation = Allocation::none(
            AllocationType::Texture,
            desc.format.theoretical_memory_footprint(desc.size),
        );

        Ok((resource, wrapped_allocation))
    }

    fn create_committed_acceleration_structure(
        &self,
        desc: &crate::AccelerationStructureDescriptor,
        raw_desc: Direct3D12::D3D12_RESOURCE_DESC,
    ) -> Result<(Direct3D12::ID3D12Resource, Allocation), crate::DeviceError> {
        let heap_properties = Direct3D12::D3D12_HEAP_PROPERTIES {
            Type: Direct3D12::D3D12_HEAP_TYPE_CUSTOM,
            CPUPageProperty: Direct3D12::D3D12_CPU_PAGE_PROPERTY_NOT_AVAILABLE,
            MemoryPoolPreference: match self.shared.private_caps.memory_architecture {
                crate::dx12::MemoryArchitecture::NonUnified => Direct3D12::D3D12_MEMORY_POOL_L1,
                crate::dx12::MemoryArchitecture::Unified { .. } => Direct3D12::D3D12_MEMORY_POOL_L0,
            },
            CreationNodeMask: 0,
            VisibleNodeMask: 0,
        };

        let mut resource = None;

        unsafe {
            self.raw.CreateCommittedResource(
                &heap_properties,
                if self.shared.private_caps.heap_create_not_zeroed {
                    Direct3D12::D3D12_HEAP_FLAG_CREATE_NOT_ZEROED
                } else {
                    Direct3D12::D3D12_HEAP_FLAG_NONE
                },
                &raw_desc,
                Direct3D12::D3D12_RESOURCE_STATE_RAYTRACING_ACCELERATION_STRUCTURE,
                None,
                &mut resource,
            )
        }
        .into_device_result("Committed acceleration structure creation")?;

        let resource = resource.ok_or(crate::DeviceError::Unexpected)?;
        let wrapped_allocation = Allocation::none(AllocationType::AccelerationStructure, desc.size);

        Ok((resource, wrapped_allocation))
    }

    fn error_if_would_oom_on_resource_allocation(
        &self,
        desc: &Direct3D12::D3D12_RESOURCE_DESC,
        location: MemoryLocation,
    ) -> Result<Direct3D12::D3D12_RESOURCE_ALLOCATION_INFO, crate::DeviceError> {
        let allocation_info = unsafe {
            self.raw
                .GetResourceAllocationInfo(0, core::slice::from_ref(desc))
        };

        // Some versions of WARP return SizeInBytes == 0 for very large
        // allocations. Proceeding to attempt to allocate a zero-sized resource
        // will result in a device lost error, so it seems preferable to return
        // an out of memory error now.
        if allocation_info.SizeInBytes == 0 {
            return Err(crate::DeviceError::OutOfMemory);
        }

        let Some(threshold) = self
            .mem_allocator
            .memory_budget_thresholds
            .for_resource_creation
        else {
            return Ok(allocation_info);
        };

        let memory_segment_group = match location {
            MemoryLocation::Unknown => unreachable!(),
            MemoryLocation::GpuOnly => Dxgi::DXGI_MEMORY_SEGMENT_GROUP_LOCAL,
            MemoryLocation::CpuToGpu | MemoryLocation::GpuToCpu => {
                match self.shared.private_caps.memory_architecture {
                    super::MemoryArchitecture::Unified { .. } => {
                        Dxgi::DXGI_MEMORY_SEGMENT_GROUP_LOCAL
                    }
                    super::MemoryArchitecture::NonUnified => {
                        Dxgi::DXGI_MEMORY_SEGMENT_GROUP_NON_LOCAL
                    }
                }
            }
        };

        let info = self
            .shared
            .adapter
            .query_video_memory_info(memory_segment_group)?;

        let memblock_size = match location {
            MemoryLocation::Unknown => unreachable!(),
            MemoryLocation::GpuOnly => self.mem_allocator.device_memblock_size,
            MemoryLocation::CpuToGpu | MemoryLocation::GpuToCpu => {
                self.mem_allocator.host_memblock_size
            }
        };

        if info
            .CurrentUsage
            .checked_add(allocation_info.SizeInBytes.max(memblock_size))
            .is_none_or(|usage| usage >= info.Budget / 100 * threshold as u64)
        {
            return Err(crate::DeviceError::OutOfMemory);
        }

        Ok(allocation_info)
    }
}
