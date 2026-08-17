#[cfg(feature = "std")]
use alloc::sync::Arc;
use alloc::{boxed::Box, string::String, vec::Vec};
use core::{
    fmt,
    // TODO: Remove when bumping MSRV to 1.80
    mem::size_of_val,
};
#[cfg(feature = "std")]
use std::backtrace::Backtrace;

use log::{debug, warn, Level};
use windows::Win32::{
    Foundation::E_OUTOFMEMORY,
    Graphics::{
        Direct3D12::*,
        Dxgi::{Common::DXGI_FORMAT, DXGI_ERROR_DEVICE_REMOVED},
    },
};

#[cfg(feature = "visualizer")]
mod visualizer;
#[cfg(feature = "visualizer")]
pub use visualizer::AllocatorVisualizer;

use crate::{
    allocator::{
        AllocationType, AllocatorReport, DedicatedBlockAllocator, FreeListAllocator,
        MemoryBlockReport, SubAllocator,
    },
    AllocationError, AllocationSizes, AllocatorDebugSettings, MemoryLocation, Result,
};

/// [`ResourceCategory`] is used for supporting [`D3D12_RESOURCE_HEAP_TIER_1`].
/// [`ResourceCategory`] will be ignored if device supports [`D3D12_RESOURCE_HEAP_TIER_2`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceCategory {
    Buffer,
    RtvDsvTexture,
    OtherTexture,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceStateOrBarrierLayout {
    ResourceState(D3D12_RESOURCE_STATES),
    BarrierLayout(D3D12_BARRIER_LAYOUT),
}

#[derive(Clone, Copy)]
pub struct ResourceCreateDesc<'a> {
    pub name: &'a str,
    pub memory_location: MemoryLocation,
    pub resource_category: ResourceCategory,
    pub resource_desc: &'a D3D12_RESOURCE_DESC,
    pub castable_formats: &'a [DXGI_FORMAT],
    pub clear_value: Option<&'a D3D12_CLEAR_VALUE>,
    pub initial_state_or_layout: ResourceStateOrBarrierLayout,
    pub resource_type: &'a ResourceType<'a>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HeapCategory {
    All,
    Buffer,
    RtvDsvTexture,
    OtherTexture,
}

impl From<ResourceCategory> for HeapCategory {
    fn from(resource_category: ResourceCategory) -> Self {
        match resource_category {
            ResourceCategory::Buffer => Self::Buffer,
            ResourceCategory::RtvDsvTexture => Self::RtvDsvTexture,
            ResourceCategory::OtherTexture => Self::OtherTexture,
        }
    }
}

impl From<&D3D12_RESOURCE_DESC> for ResourceCategory {
    fn from(desc: &D3D12_RESOURCE_DESC) -> Self {
        if desc.Dimension == D3D12_RESOURCE_DIMENSION_BUFFER {
            Self::Buffer
        } else if (desc.Flags
            & (D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET | D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL))
            != D3D12_RESOURCE_FLAG_NONE
        {
            Self::RtvDsvTexture
        } else {
            Self::OtherTexture
        }
    }
}

#[derive(Clone, Debug)]
pub struct AllocationCreateDesc<'a> {
    /// Name of the allocation, for tracking and debugging purposes
    pub name: &'a str,
    /// Location where the memory allocation should be stored
    pub location: MemoryLocation,

    /// Size of allocation, should be queried using [`ID3D12Device::GetResourceAllocationInfo()`]
    pub size: u64,
    /// Alignment of allocation, should be queried using [`ID3D12Device::GetResourceAllocationInfo()`]
    pub alignment: u64,
    /// Resource category based on resource dimension and flags. Can be created from a [`D3D12_RESOURCE_DESC`]
    /// using the [helper `into()` function]. The resource category is ignored when Resource Heap Tier 2 or higher
    /// is supported.
    ///
    /// [helper `into()` function]: ResourceCategory::from()
    pub resource_category: ResourceCategory,
}

impl<'a> AllocationCreateDesc<'a> {
    /// Helper function to construct an [`AllocationCreateDesc`] from an existing
    /// [`D3D12_RESOURCE_DESC`] utilizing [`ID3D12Device::GetResourceAllocationInfo()`].
    pub fn from_d3d12_resource_desc(
        device: &ID3D12Device,
        desc: &D3D12_RESOURCE_DESC,
        name: &'a str,
        location: MemoryLocation,
    ) -> Self {
        // SAFETY: `device` is a valid device handle, and no arguments (like pointers) are passed
        // that could induce UB.
        let allocation_info =
            unsafe { device.GetResourceAllocationInfo(0, core::slice::from_ref(desc)) };
        let resource_category: ResourceCategory = desc.into();

        AllocationCreateDesc {
            name,
            location,
            size: allocation_info.SizeInBytes,
            alignment: allocation_info.Alignment,
            resource_category,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ID3D12DeviceVersion {
    /// Basic device compatible with legacy barriers only, i.e. can only be used in conjunction
    /// with [`ResourceStateOrBarrierLayout::ResourceState`].
    Device(ID3D12Device),
    /// Required for enhanced barrier support, i.e. when using
    /// [`ResourceStateOrBarrierLayout::BarrierLayout`].
    Device10(ID3D12Device10),
    /// Required for castable formats support, implies use of enhanced barriers
    Device12(ID3D12Device12),
}

impl core::ops::Deref for ID3D12DeviceVersion {
    type Target = ID3D12Device;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Device(device) => device,
            Self::Device10(device10) => device10.into(),
            Self::Device12(device12) => device12.into(),
        }
    }
}

#[derive(Debug)]
pub struct AllocatorCreateDesc {
    pub device: ID3D12DeviceVersion,
    pub debug_settings: AllocatorDebugSettings,
    pub allocation_sizes: AllocationSizes,
}

pub enum ResourceType<'a> {
    /// Create a D3D12 [`CommittedResource`].
    ///
    /// [`CommittedResource`]: https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createcommittedresource
    Committed {
        heap_properties: &'a D3D12_HEAP_PROPERTIES,
        heap_flags: D3D12_HEAP_FLAGS,
    },
    /// Create a D3D12 [`PlacedResource`].
    ///
    /// [`PlacedResource`]: https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createplacedresource
    Placed,
}

#[derive(Debug)]
pub struct Resource {
    name: String,
    pub allocation: Option<Allocation>,
    resource: Option<ID3D12Resource>,
    pub memory_location: MemoryLocation,
    memory_type_index: Option<usize>,
    pub size: u64,
}

impl Resource {
    pub fn resource(&self) -> &ID3D12Resource {
        self.resource.as_ref().expect("Resource was already freed.")
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        if self.resource.is_some() {
            warn!("Dropping resource `{}` that was not freed. Call `Allocator::free_resource(resource)` instead.", self.name);
        }
    }
}

#[derive(Debug)]
pub struct CommittedAllocationStatistics {
    pub num_allocations: usize,
    pub total_size: u64,
}

#[derive(Debug)]
pub struct Allocation {
    chunk_id: Option<core::num::NonZeroU64>,
    offset: u64,
    size: u64,
    memory_block_index: usize,
    memory_type_index: usize,
    heap: ID3D12Heap,

    name: Option<Box<str>>,
}

impl Allocation {
    pub fn chunk_id(&self) -> Option<core::num::NonZeroU64> {
        self.chunk_id
    }

    /// Returns the [`ID3D12Heap`] object that is backing this allocation.
    ///
    /// This heap object can be shared with multiple other allocations and shouldn't be allocated from
    /// without this library, because that will lead to undefined behavior.
    ///
    /// # Safety
    /// The result of this function can safely be passed into [`ID3D12Device::CreatePlacedResource()`].
    /// It is exposed for this reason.  Keep in mind to also pass [`Self::offset()`] along to it.
    ///
    /// Also, this [`Allocation`] must not be [`Allocator::free()`]d while such a created resource
    /// on this [`ID3D12Heap`] is still live.
    pub unsafe fn heap(&self) -> &ID3D12Heap {
        &self.heap
    }

    /// Returns the offset of the allocation on the [`ID3D12Heap`].
    /// When creating a placed resource, this offset needs to be supplied as well.
    pub fn offset(&self) -> u64 {
        self.offset
    }

    /// Returns the size of the allocation
    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn is_null(&self) -> bool {
        self.chunk_id.is_none()
    }
}

#[derive(Debug)]
struct MemoryBlock {
    heap: ID3D12Heap,
    size: u64,
    sub_allocator: Box<dyn SubAllocator>,
}
impl MemoryBlock {
    fn new(
        device: &ID3D12Device,
        size: u64,
        heap_properties: &D3D12_HEAP_PROPERTIES,
        heap_category: HeapCategory,
        dedicated: bool,
    ) -> Result<Self> {
        let heap = {
            let mut desc = D3D12_HEAP_DESC {
                SizeInBytes: size,
                Properties: *heap_properties,
                Alignment: D3D12_DEFAULT_MSAA_RESOURCE_PLACEMENT_ALIGNMENT as u64,
                ..Default::default()
            };
            desc.Flags = match heap_category {
                HeapCategory::All => D3D12_HEAP_FLAG_NONE,
                HeapCategory::Buffer => D3D12_HEAP_FLAG_ALLOW_ONLY_BUFFERS,
                HeapCategory::RtvDsvTexture => D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES,
                HeapCategory::OtherTexture => D3D12_HEAP_FLAG_ALLOW_ONLY_NON_RT_DS_TEXTURES,
            };

            let mut heap = None;
            let hr = unsafe { device.CreateHeap(&desc, &mut heap) };
            match hr {
                Err(e) if e.code() == E_OUTOFMEMORY => Err(AllocationError::OutOfMemory),
                Err(e) => Err(AllocationError::Internal(format!(
                    "ID3D12Device::CreateHeap failed: {e}"
                ))),
                Ok(()) => heap.ok_or_else(|| {
                    AllocationError::Internal(
                        "ID3D12Heap pointer is null, but should not be.".into(),
                    )
                }),
            }?
        };

        let sub_allocator: Box<dyn SubAllocator> = if dedicated {
            Box::new(DedicatedBlockAllocator::new(size))
        } else {
            Box::new(FreeListAllocator::new(size))
        };

        Ok(Self {
            heap,
            size,
            sub_allocator,
        })
    }
}

#[derive(Debug)]
struct MemoryType {
    memory_blocks: Vec<Option<MemoryBlock>>,
    committed_allocations: CommittedAllocationStatistics,
    memory_location: MemoryLocation,
    heap_category: HeapCategory,
    heap_properties: D3D12_HEAP_PROPERTIES,
    memory_type_index: usize,
    active_general_blocks: usize,
}

impl MemoryType {
    fn allocate(
        &mut self,
        device: &ID3D12DeviceVersion,
        desc: &AllocationCreateDesc<'_>,
        #[cfg(feature = "std")] backtrace: Arc<Backtrace>,
        allocation_sizes: &AllocationSizes,
    ) -> Result<Allocation> {
        let allocation_type = AllocationType::Linear;

        let is_host = self.heap_properties.Type != D3D12_HEAP_TYPE_DEFAULT;
        let memblock_size = allocation_sizes.get_memblock_size(is_host, self.active_general_blocks);

        let size = desc.size;
        let alignment = desc.alignment;

        // Create a dedicated block for large memory allocations
        if size > memblock_size {
            let mem_block = MemoryBlock::new(
                device,
                size,
                &self.heap_properties,
                self.heap_category,
                true,
            )?;

            let block_index = self.memory_blocks.iter().position(|block| block.is_none());
            let block_index = match block_index {
                Some(i) => {
                    self.memory_blocks[i].replace(mem_block);
                    i
                }
                None => {
                    self.memory_blocks.push(Some(mem_block));
                    self.memory_blocks.len() - 1
                }
            };

            let mem_block = self.memory_blocks[block_index]
                .as_mut()
                .ok_or_else(|| AllocationError::Internal("Memory block must be Some".into()))?;

            let (offset, chunk_id) = mem_block.sub_allocator.allocate(
                size,
                alignment,
                allocation_type,
                1,
                desc.name,
                #[cfg(feature = "std")]
                backtrace,
            )?;

            return Ok(Allocation {
                chunk_id: Some(chunk_id),
                size,
                offset,
                memory_block_index: block_index,
                memory_type_index: self.memory_type_index,
                heap: mem_block.heap.clone(),
                name: Some(desc.name.into()),
            });
        }

        let mut empty_block_index = None;
        for (mem_block_i, mem_block) in self.memory_blocks.iter_mut().enumerate().rev() {
            if let Some(mem_block) = mem_block {
                let allocation = mem_block.sub_allocator.allocate(
                    size,
                    alignment,
                    allocation_type,
                    1,
                    desc.name,
                    #[cfg(feature = "std")]
                    backtrace.clone(),
                );

                match allocation {
                    Ok((offset, chunk_id)) => {
                        return Ok(Allocation {
                            chunk_id: Some(chunk_id),
                            offset,
                            size,
                            memory_block_index: mem_block_i,
                            memory_type_index: self.memory_type_index,
                            heap: mem_block.heap.clone(),
                            name: Some(desc.name.into()),
                        });
                    }
                    Err(AllocationError::OutOfMemory) => {} // Block is full, continue search.
                    Err(err) => return Err(err),            // Unhandled error, return.
                }
            } else if empty_block_index.is_none() {
                empty_block_index = Some(mem_block_i);
            }
        }

        let new_memory_block = MemoryBlock::new(
            device,
            memblock_size,
            &self.heap_properties,
            self.heap_category,
            false,
        )?;

        let new_block_index = if let Some(block_index) = empty_block_index {
            self.memory_blocks[block_index] = Some(new_memory_block);
            block_index
        } else {
            self.memory_blocks.push(Some(new_memory_block));
            self.memory_blocks.len() - 1
        };

        self.active_general_blocks += 1;

        let mem_block = self.memory_blocks[new_block_index]
            .as_mut()
            .ok_or_else(|| AllocationError::Internal("Memory block must be Some".into()))?;
        let allocation = mem_block.sub_allocator.allocate(
            size,
            alignment,
            allocation_type,
            1,
            desc.name,
            #[cfg(feature = "std")]
            backtrace,
        );
        let (offset, chunk_id) = match allocation {
            Err(AllocationError::OutOfMemory) => Err(AllocationError::Internal(
                "Allocation that must succeed failed. This is a bug in the allocator.".into(),
            )),
            a => a,
        }?;

        Ok(Allocation {
            chunk_id: Some(chunk_id),
            offset,
            size,
            memory_block_index: new_block_index,
            memory_type_index: self.memory_type_index,
            heap: mem_block.heap.clone(),
            name: Some(desc.name.into()),
        })
    }

    #[allow(clippy::needless_pass_by_value)]
    fn free(&mut self, allocation: Allocation) -> Result<()> {
        let block_idx = allocation.memory_block_index;

        let mem_block = self.memory_blocks[block_idx]
            .as_mut()
            .ok_or_else(|| AllocationError::Internal("Memory block must be Some.".into()))?;

        mem_block.sub_allocator.free(allocation.chunk_id)?;

        // We only want to destroy this now-empty block if it is either a dedicated/personal
        // allocation, or a block supporting sub-allocations that is not the last one (ensuring
        // there's always at least one block/allocator readily available).
        let is_dedicated_or_not_last_general_block =
            !mem_block.sub_allocator.supports_general_allocations()
                || self.active_general_blocks > 1;
        if mem_block.sub_allocator.is_empty() && is_dedicated_or_not_last_general_block {
            let block = self.memory_blocks[block_idx]
                .take()
                .ok_or_else(|| AllocationError::Internal("Memory block must be Some.".into()))?;

            if block.sub_allocator.supports_general_allocations() {
                self.active_general_blocks -= 1;
            }

            // Note that `block` will be destroyed on `drop` here
        }

        Ok(())
    }
}

pub struct Allocator {
    device: ID3D12DeviceVersion,
    debug_settings: AllocatorDebugSettings,
    memory_types: Vec<MemoryType>,
    allocation_sizes: AllocationSizes,
}

impl Allocator {
    pub fn device(&self) -> &ID3D12DeviceVersion {
        &self.device
    }

    pub fn new(desc: &AllocatorCreateDesc) -> Result<Self> {
        // Perform AddRef on the device
        let device = desc.device.clone();

        // Query device for feature level
        let mut options = Default::default();
        unsafe {
            device.CheckFeatureSupport(
                D3D12_FEATURE_D3D12_OPTIONS,
                <*mut D3D12_FEATURE_DATA_D3D12_OPTIONS>::cast(&mut options),
                size_of_val(&options) as u32,
            )
        }
        .map_err(|e| {
            AllocationError::Internal(format!("ID3D12Device::CheckFeatureSupport failed: {e}"))
        })?;

        let is_heap_tier1 = options.ResourceHeapTier == D3D12_RESOURCE_HEAP_TIER_1;

        let heap_types = [
            (
                MemoryLocation::GpuOnly,
                D3D12_HEAP_PROPERTIES {
                    Type: D3D12_HEAP_TYPE_DEFAULT,
                    ..Default::default()
                },
            ),
            (
                MemoryLocation::CpuToGpu,
                D3D12_HEAP_PROPERTIES {
                    Type: D3D12_HEAP_TYPE_CUSTOM,
                    CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_WRITE_COMBINE,
                    MemoryPoolPreference: D3D12_MEMORY_POOL_L0,
                    ..Default::default()
                },
            ),
            (
                MemoryLocation::GpuToCpu,
                D3D12_HEAP_PROPERTIES {
                    Type: D3D12_HEAP_TYPE_CUSTOM,
                    CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_WRITE_BACK,
                    MemoryPoolPreference: D3D12_MEMORY_POOL_L0,
                    ..Default::default()
                },
            ),
        ];

        let heap_types = if is_heap_tier1 {
            heap_types
                .iter()
                .flat_map(|(memory_location, heap_properties)| {
                    [
                        (HeapCategory::Buffer, *memory_location, *heap_properties),
                        (
                            HeapCategory::RtvDsvTexture,
                            *memory_location,
                            *heap_properties,
                        ),
                        (
                            HeapCategory::OtherTexture,
                            *memory_location,
                            *heap_properties,
                        ),
                    ]
                    .to_vec()
                })
                .collect::<Vec<_>>()
        } else {
            heap_types
                .iter()
                .map(|(memory_location, heap_properties)| {
                    (HeapCategory::All, *memory_location, *heap_properties)
                })
                .collect::<Vec<_>>()
        };

        let memory_types = heap_types
            .iter()
            .enumerate()
            .map(
                |(i, &(heap_category, memory_location, heap_properties))| MemoryType {
                    memory_blocks: Vec::default(),
                    memory_location,
                    heap_category,
                    heap_properties,
                    memory_type_index: i,
                    active_general_blocks: 0,
                    committed_allocations: CommittedAllocationStatistics {
                        num_allocations: 0,
                        total_size: 0,
                    },
                },
            )
            .collect::<Vec<_>>();

        Ok(Self {
            memory_types,
            device,
            debug_settings: desc.debug_settings,
            allocation_sizes: desc.allocation_sizes,
        })
    }

    pub fn allocate(&mut self, desc: &AllocationCreateDesc<'_>) -> Result<Allocation> {
        let size = desc.size;
        let alignment = desc.alignment;

        #[cfg(feature = "std")]
        let backtrace = Arc::new(if self.debug_settings.store_stack_traces {
            Backtrace::force_capture()
        } else {
            Backtrace::disabled()
        });

        if self.debug_settings.log_allocations {
            debug!(
                "Allocating `{}` of {} bytes with an alignment of {}.",
                &desc.name, size, alignment
            );
            #[cfg(feature = "std")]
            if self.debug_settings.log_stack_traces {
                let backtrace = Backtrace::force_capture();
                debug!("Allocation stack trace: {backtrace}");
            }
        }

        if size == 0 || !alignment.is_power_of_two() {
            return Err(AllocationError::InvalidAllocationCreateDesc);
        }

        // Find memory type
        let memory_type = self
            .memory_types
            .iter_mut()
            .find(|memory_type| {
                let is_location_compatible = desc.location == MemoryLocation::Unknown
                    || desc.location == memory_type.memory_location;

                let is_category_compatible = memory_type.heap_category == HeapCategory::All
                    || memory_type.heap_category == desc.resource_category.into();

                is_location_compatible && is_category_compatible
            })
            .ok_or(AllocationError::NoCompatibleMemoryTypeFound)?;

        memory_type.allocate(
            &self.device,
            desc,
            #[cfg(feature = "std")]
            backtrace,
            &self.allocation_sizes,
        )
    }

    pub fn free(&mut self, allocation: Allocation) -> Result<()> {
        if self.debug_settings.log_frees {
            let name = allocation.name.as_deref().unwrap_or("<null>");
            debug!("Freeing `{name}`.");
            #[cfg(feature = "std")]
            if self.debug_settings.log_stack_traces {
                let backtrace = Backtrace::force_capture();
                debug!("Free stack trace: {backtrace}");
            }
        }

        if allocation.is_null() {
            return Ok(());
        }

        self.memory_types[allocation.memory_type_index].free(allocation)?;

        Ok(())
    }

    pub fn rename_allocation(&mut self, allocation: &mut Allocation, name: &str) -> Result<()> {
        allocation.name = Some(name.into());

        if allocation.is_null() {
            return Ok(());
        }

        let mem_type = &mut self.memory_types[allocation.memory_type_index];
        let mem_block = mem_type.memory_blocks[allocation.memory_block_index]
            .as_mut()
            .ok_or_else(|| AllocationError::Internal("Memory block must be Some.".into()))?;

        mem_block
            .sub_allocator
            .rename_allocation(allocation.chunk_id, name)?;

        Ok(())
    }

    pub fn report_memory_leaks(&self, log_level: Level) {
        for (mem_type_i, mem_type) in self.memory_types.iter().enumerate() {
            for (block_i, mem_block) in mem_type.memory_blocks.iter().enumerate() {
                if let Some(mem_block) = mem_block {
                    mem_block
                        .sub_allocator
                        .report_memory_leaks(log_level, mem_type_i, block_i);
                }
            }
        }
    }

    fn d3d12_resource_desc_1(desc: &D3D12_RESOURCE_DESC) -> D3D12_RESOURCE_DESC1 {
        D3D12_RESOURCE_DESC1 {
            Dimension: desc.Dimension,
            Alignment: desc.Alignment,
            Width: desc.Width,
            Height: desc.Height,
            DepthOrArraySize: desc.DepthOrArraySize,
            MipLevels: desc.MipLevels,
            Format: desc.Format,
            SampleDesc: desc.SampleDesc,
            Layout: desc.Layout,
            Flags: desc.Flags,
            // TODO: This is the only new field
            SamplerFeedbackMipRegion: D3D12_MIP_REGION::default(),
        }
    }

    fn resource_allocation_info(
        device: &ID3D12DeviceVersion,
        desc: &ResourceCreateDesc<'_>,
    ) -> D3D12_RESOURCE_ALLOCATION_INFO {
        match device {
            ID3D12DeviceVersion::Device(device) => unsafe {
                device.GetResourceAllocationInfo(0, &[*desc.resource_desc])
            },
            ID3D12DeviceVersion::Device10(device) => unsafe {
                device.GetResourceAllocationInfo(0, &[*desc.resource_desc])
            },
            ID3D12DeviceVersion::Device12(device) => unsafe {
                let resource_desc1 = Self::d3d12_resource_desc_1(desc.resource_desc);

                let resource_descs = &[resource_desc1];

                // We always have one resource desc, hence we only have one mapping castable format array
                let num_castable_formats = desc.castable_formats.len() as u32;
                let num_castable_formats_array = &[num_castable_formats];

                let castable_formats_array = &[desc.castable_formats.as_ptr()];

                let (num_castable_formats_opt, castable_formats_opt) = if num_castable_formats > 0 {
                    (
                        Some(num_castable_formats_array.as_ptr()),
                        Some(castable_formats_array.as_ptr()),
                    )
                } else {
                    (None, None)
                };

                device.GetResourceAllocationInfo3(
                    0,
                    resource_descs.len() as u32,
                    resource_descs.as_ptr(),
                    num_castable_formats_opt,
                    castable_formats_opt,
                    None,
                )
            },
        }
    }

    /// Create a resource according to the provided parameters.
    /// Created resources should be freed at the end of their lifetime by calling [`Self::free_resource()`].
    pub fn create_resource(&mut self, desc: &ResourceCreateDesc<'_>) -> Result<Resource> {
        match desc.resource_type {
            ResourceType::Committed {
                heap_properties,
                heap_flags,
            } => {
                let mut result: Option<ID3D12Resource> = None;

                let clear_value: Option<*const D3D12_CLEAR_VALUE> =
                    desc.clear_value.map(|v| -> *const _ { v });

                if let Err(e) = unsafe {
                    match (&self.device, desc.initial_state_or_layout) {
                        (_, ResourceStateOrBarrierLayout::ResourceState(_))
                            if !desc.castable_formats.is_empty() =>
                        {
                            return Err(AllocationError::CastableFormatsRequiresEnhancedBarriers)
                        }
                        (
                            ID3D12DeviceVersion::Device12(device),
                            ResourceStateOrBarrierLayout::BarrierLayout(initial_layout),
                        ) => {
                            let resource_desc1 = Self::d3d12_resource_desc_1(desc.resource_desc);
                            device.CreateCommittedResource3(
                                *heap_properties,
                                *heap_flags,
                                &resource_desc1,
                                initial_layout,
                                clear_value,
                                None, // TODO
                                Some(desc.castable_formats),
                                &mut result,
                            )
                        }
                        (_, ResourceStateOrBarrierLayout::BarrierLayout(_))
                            if !desc.castable_formats.is_empty() =>
                        {
                            return Err(AllocationError::CastableFormatsRequiresAtLeastDevice12)
                        }
                        (
                            ID3D12DeviceVersion::Device10(device),
                            ResourceStateOrBarrierLayout::BarrierLayout(initial_layout),
                        ) => {
                            let resource_desc1 = Self::d3d12_resource_desc_1(desc.resource_desc);

                            device.CreateCommittedResource3(
                                *heap_properties,
                                *heap_flags,
                                &resource_desc1,
                                initial_layout,
                                clear_value,
                                None, // TODO
                                None,
                                &mut result,
                            )
                        }
                        (_, ResourceStateOrBarrierLayout::BarrierLayout(_)) => {
                            return Err(AllocationError::BarrierLayoutNeedsDevice10)
                        }
                        (device, ResourceStateOrBarrierLayout::ResourceState(initial_state)) => {
                            device.CreateCommittedResource(
                                *heap_properties,
                                *heap_flags,
                                desc.resource_desc,
                                initial_state,
                                clear_value,
                                &mut result,
                            )
                        }
                    }
                } {
                    if e.code() == DXGI_ERROR_DEVICE_REMOVED {
                        return Err(AllocationError::Internal(format!(
                            "ID3D12Device::CreateCommittedResource DEVICE_REMOVED: {:?}",
                            unsafe { self.device.GetDeviceRemovedReason() }
                        )));
                    }
                    return Err(AllocationError::Internal(format!(
                        "ID3D12Device::CreateCommittedResource failed: {e}"
                    )));
                }

                let resource = result.expect("Allocation succeeded but no resource was returned?");

                let allocation_info = Self::resource_allocation_info(&self.device, desc);

                let memory_type = self
                    .memory_types
                    .iter_mut()
                    .find(|memory_type| {
                        let is_location_compatible = desc.memory_location
                            == MemoryLocation::Unknown
                            || desc.memory_location == memory_type.memory_location;

                        let is_category_compatible = memory_type.heap_category == HeapCategory::All
                            || memory_type.heap_category == desc.resource_category.into();

                        is_location_compatible && is_category_compatible
                    })
                    .ok_or(AllocationError::NoCompatibleMemoryTypeFound)?;

                memory_type.committed_allocations.num_allocations += 1;
                memory_type.committed_allocations.total_size += allocation_info.SizeInBytes;

                Ok(Resource {
                    name: desc.name.into(),
                    allocation: None,
                    resource: Some(resource),
                    size: allocation_info.SizeInBytes,
                    memory_location: desc.memory_location,
                    memory_type_index: Some(memory_type.memory_type_index),
                })
            }
            ResourceType::Placed => {
                let allocation_desc = {
                    let allocation_info = Self::resource_allocation_info(&self.device, desc);

                    AllocationCreateDesc {
                        name: desc.name,
                        location: desc.memory_location,
                        size: allocation_info.SizeInBytes,
                        alignment: allocation_info.Alignment,
                        resource_category: desc.resource_category,
                    }
                };

                let allocation = self.allocate(&allocation_desc)?;

                let mut result: Option<ID3D12Resource> = None;
                if let Err(e) = unsafe {
                    match (&self.device, desc.initial_state_or_layout) {
                        (_, ResourceStateOrBarrierLayout::ResourceState(_))
                            if !desc.castable_formats.is_empty() =>
                        {
                            return Err(AllocationError::CastableFormatsRequiresEnhancedBarriers)
                        }
                        (
                            ID3D12DeviceVersion::Device12(device),
                            ResourceStateOrBarrierLayout::BarrierLayout(initial_layout),
                        ) => {
                            let resource_desc1 = Self::d3d12_resource_desc_1(desc.resource_desc);
                            device.CreatePlacedResource2(
                                allocation.heap(),
                                allocation.offset(),
                                &resource_desc1,
                                initial_layout,
                                None,
                                Some(desc.castable_formats),
                                &mut result,
                            )
                        }
                        (_, ResourceStateOrBarrierLayout::BarrierLayout(_))
                            if !desc.castable_formats.is_empty() =>
                        {
                            return Err(AllocationError::CastableFormatsRequiresAtLeastDevice12)
                        }
                        (
                            ID3D12DeviceVersion::Device10(device),
                            ResourceStateOrBarrierLayout::BarrierLayout(initial_layout),
                        ) => {
                            let resource_desc1 = Self::d3d12_resource_desc_1(desc.resource_desc);
                            device.CreatePlacedResource2(
                                allocation.heap(),
                                allocation.offset(),
                                &resource_desc1,
                                initial_layout,
                                None,
                                None,
                                &mut result,
                            )
                        }
                        (_, ResourceStateOrBarrierLayout::BarrierLayout(_)) => {
                            return Err(AllocationError::BarrierLayoutNeedsDevice10)
                        }
                        (device, ResourceStateOrBarrierLayout::ResourceState(initial_state)) => {
                            device.CreatePlacedResource(
                                allocation.heap(),
                                allocation.offset(),
                                desc.resource_desc,
                                initial_state,
                                None,
                                &mut result,
                            )
                        }
                    }
                } {
                    if e.code() == DXGI_ERROR_DEVICE_REMOVED {
                        return Err(AllocationError::Internal(format!(
                            "ID3D12Device::CreatePlacedResource DEVICE_REMOVED: {:?}",
                            unsafe { self.device.GetDeviceRemovedReason() }
                        )));
                    }
                    return Err(AllocationError::Internal(format!(
                        "ID3D12Device::CreatePlacedResource failed: {e}"
                    )));
                }

                let resource = result.expect("Allocation succeeded but no resource was returned?");
                let size = allocation.size();
                Ok(Resource {
                    name: desc.name.into(),
                    allocation: Some(allocation),
                    resource: Some(resource),
                    size,
                    memory_location: desc.memory_location,
                    memory_type_index: None,
                })
            }
        }
    }

    /// Free a resource and its memory.
    pub fn free_resource(&mut self, mut resource: Resource) -> Result<()> {
        // Explicitly drop the resource (which is backed by a refcounted COM object)
        // before freeing allocated memory. Windows-rs performs a Release() on drop().
        let _ = resource
            .resource
            .take()
            .expect("Resource was already freed.");

        if let Some(allocation) = resource.allocation.take() {
            self.free(allocation)
        } else {
            // Dx12 CommittedResources do not have an application managed allocation.
            // We only have to update the tracked allocation count and memory usage.
            if let Some(memory_type_index) = resource.memory_type_index {
                let memory_type = &mut self.memory_types[memory_type_index];

                memory_type.committed_allocations.num_allocations -= 1;
                memory_type.committed_allocations.total_size -= resource.size;
            }
            Ok(())
        }
    }

    pub fn generate_report(&self) -> AllocatorReport {
        let mut allocations = vec![];
        let mut blocks = vec![];
        let mut total_capacity_bytes = 0;

        for memory_type in &self.memory_types {
            for block in memory_type.memory_blocks.iter().flatten() {
                total_capacity_bytes += block.size;
                let first_allocation = allocations.len();
                allocations.extend(block.sub_allocator.report_allocations());
                blocks.push(MemoryBlockReport {
                    size: block.size,
                    allocations: first_allocation..allocations.len(),
                });
            }
        }

        let total_allocated_bytes = allocations.iter().map(|report| report.size).sum();

        AllocatorReport {
            allocations,
            blocks,
            total_allocated_bytes,
            total_capacity_bytes,
        }
    }

    /// Current total capacity of memory blocks allocated on the device, in bytes
    pub fn capacity(&self) -> u64 {
        let mut total_capacity_bytes = 0;

        for memory_type in &self.memory_types {
            for block in memory_type.memory_blocks.iter().flatten() {
                total_capacity_bytes += block.size;
            }
        }

        total_capacity_bytes
    }
}

impl fmt::Debug for Allocator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.generate_report().fmt(f)
    }
}

impl Drop for Allocator {
    fn drop(&mut self) {
        if self.debug_settings.log_leaks_on_shutdown {
            self.report_memory_leaks(Level::Warn);
        }

        // Because Rust drop rules drop members in source-code order (that would be the
        // ID3D12Device before the ID3D12Heaps nested in these memory blocks), free
        // all remaining memory blocks manually first by dropping.
        for mem_type in self.memory_types.iter_mut() {
            mem_type.memory_blocks.clear();
        }
    }
}
