use alloc::vec::Vec;
use core::fmt;

use bit_set::BitSet;
use parking_lot::Mutex;
use range_alloc::RangeAllocator;
use windows::Win32::Graphics::Direct3D12;

use crate::auxil::dxgi::result::HResult as _;

const HEAP_SIZE_FIXED: usize = 64;

#[derive(Copy, Clone)]
pub(super) struct DualHandle {
    cpu: Direct3D12::D3D12_CPU_DESCRIPTOR_HANDLE,
    pub gpu: Direct3D12::D3D12_GPU_DESCRIPTOR_HANDLE,
    /// How large the block allocated to this handle is.
    count: u64,
}

impl fmt::Debug for DualHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DualHandle")
            .field("cpu", &self.cpu.ptr)
            .field("gpu", &self.gpu.ptr)
            .field("count", &self.count)
            .finish()
    }
}

type DescriptorIndex = u64;

pub(super) struct GeneralHeap {
    pub raw: Direct3D12::ID3D12DescriptorHeap,
    ty: Direct3D12::D3D12_DESCRIPTOR_HEAP_TYPE,
    handle_size: u64,
    total_handles: u64,
    start: DualHandle,
    ranges: Mutex<RangeAllocator<DescriptorIndex>>,
}

impl GeneralHeap {
    pub(super) fn new(
        device: &Direct3D12::ID3D12Device,
        ty: Direct3D12::D3D12_DESCRIPTOR_HEAP_TYPE,
        total_handles: u64,
    ) -> Result<Self, crate::DeviceError> {
        let raw = {
            profiling::scope!("ID3D12Device::CreateDescriptorHeap");
            let desc = Direct3D12::D3D12_DESCRIPTOR_HEAP_DESC {
                Type: ty,
                NumDescriptors: total_handles as u32,
                Flags: Direct3D12::D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE,
                NodeMask: 0,
            };
            unsafe { device.CreateDescriptorHeap::<Direct3D12::ID3D12DescriptorHeap>(&desc) }
                .into_device_result("Descriptor heap creation")?
        };

        let start = DualHandle {
            cpu: unsafe { raw.GetCPUDescriptorHandleForHeapStart() },
            gpu: unsafe { raw.GetGPUDescriptorHandleForHeapStart() },
            count: 0,
        };

        Ok(Self {
            raw,
            ty,
            handle_size: unsafe { device.GetDescriptorHandleIncrementSize(ty) } as u64,
            total_handles,
            start,
            ranges: Mutex::new(RangeAllocator::new(0..total_handles)),
        })
    }

    pub(super) fn at(&self, index: DescriptorIndex, count: u64) -> DualHandle {
        assert!(index < self.total_handles);
        DualHandle {
            cpu: self.cpu_descriptor_at(index),
            gpu: self.gpu_descriptor_at(index),
            count,
        }
    }

    fn cpu_descriptor_at(&self, index: u64) -> Direct3D12::D3D12_CPU_DESCRIPTOR_HANDLE {
        Direct3D12::D3D12_CPU_DESCRIPTOR_HANDLE {
            ptr: self.start.cpu.ptr + (self.handle_size * index) as usize,
        }
    }

    fn gpu_descriptor_at(&self, index: u64) -> Direct3D12::D3D12_GPU_DESCRIPTOR_HANDLE {
        Direct3D12::D3D12_GPU_DESCRIPTOR_HANDLE {
            ptr: self.start.gpu.ptr + self.handle_size * index,
        }
    }

    pub(super) fn allocate_slice(&self, count: u64) -> Result<DescriptorIndex, crate::DeviceError> {
        let range = self.ranges.lock().allocate_range(count).map_err(|err| {
            log::error!("Unable to allocate descriptors: {err:?}");
            crate::DeviceError::OutOfMemory
        })?;
        Ok(range.start)
    }

    /// Free handles previously given out by this `DescriptorHeapSlice`.
    /// Do not use this with handles not given out by this `DescriptorHeapSlice`.
    pub(crate) fn free_slice(&self, handle: DualHandle) {
        let start = (handle.gpu.ptr - self.start.gpu.ptr) / self.handle_size;
        self.ranges.lock().free_range(start..start + handle.count);
    }
}

/// Fixed-size free-list allocator for CPU descriptors.
struct FixedSizeHeap {
    _raw: Direct3D12::ID3D12DescriptorHeap,
    /// Bit flag representation of available handles in the heap.
    ///
    ///  0 - Occupied
    ///  1 - free
    availability: u64,
    handle_size: usize,
    start: Direct3D12::D3D12_CPU_DESCRIPTOR_HANDLE,
}

impl FixedSizeHeap {
    fn new(
        device: &Direct3D12::ID3D12Device,
        ty: Direct3D12::D3D12_DESCRIPTOR_HEAP_TYPE,
    ) -> Result<Self, crate::DeviceError> {
        let desc = Direct3D12::D3D12_DESCRIPTOR_HEAP_DESC {
            Type: ty,
            NumDescriptors: HEAP_SIZE_FIXED as u32,
            Flags: Direct3D12::D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
            NodeMask: 0,
        };
        let heap =
            unsafe { device.CreateDescriptorHeap::<Direct3D12::ID3D12DescriptorHeap>(&desc) }
                .into_device_result("Descriptor heap creation")?;

        Ok(Self {
            handle_size: unsafe { device.GetDescriptorHandleIncrementSize(ty) } as usize,
            availability: !0, // all free!
            start: unsafe { heap.GetCPUDescriptorHandleForHeapStart() },
            _raw: heap,
        })
    }

    fn alloc_handle(
        &mut self,
    ) -> Result<Direct3D12::D3D12_CPU_DESCRIPTOR_HANDLE, crate::DeviceError> {
        // Find first free slot.
        let slot = self.availability.trailing_zeros() as usize;
        if slot >= HEAP_SIZE_FIXED {
            log::error!("Failed to allocate a handle form a fixed size heap");
            return Err(crate::DeviceError::OutOfMemory);
        }
        // Set the slot as occupied.
        self.availability ^= 1 << slot;

        Ok(Direct3D12::D3D12_CPU_DESCRIPTOR_HANDLE {
            ptr: self.start.ptr + self.handle_size * slot,
        })
    }

    fn free_handle(&mut self, handle: Direct3D12::D3D12_CPU_DESCRIPTOR_HANDLE) {
        let slot = (handle.ptr - self.start.ptr) / self.handle_size;
        assert!(slot < HEAP_SIZE_FIXED);
        assert_eq!(self.availability & (1 << slot), 0);
        self.availability ^= 1 << slot;
    }

    fn is_full(&self) -> bool {
        self.availability == 0
    }
}

#[derive(Clone, Copy)]
pub(super) struct Handle {
    pub raw: Direct3D12::D3D12_CPU_DESCRIPTOR_HANDLE,
    heap_index: usize,
}

impl fmt::Debug for Handle {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Handle")
            .field("ptr", &self.raw.ptr)
            .field("heap_index", &self.heap_index)
            .finish()
    }
}

pub(super) struct CpuPool {
    device: Direct3D12::ID3D12Device,
    ty: Direct3D12::D3D12_DESCRIPTOR_HEAP_TYPE,
    heaps: Vec<FixedSizeHeap>,
    available_heap_indices: BitSet,
}

impl CpuPool {
    pub(super) fn new(
        device: Direct3D12::ID3D12Device,
        ty: Direct3D12::D3D12_DESCRIPTOR_HEAP_TYPE,
    ) -> Self {
        Self {
            device,
            ty,
            heaps: Vec::new(),
            available_heap_indices: BitSet::new(),
        }
    }

    pub(super) fn alloc_handle(&mut self) -> Result<Handle, crate::DeviceError> {
        let heap_index = self
            .available_heap_indices
            .iter()
            .next()
            .unwrap_or(self.heaps.len());

        // Allocate a new heap
        if heap_index == self.heaps.len() {
            self.heaps.push(FixedSizeHeap::new(&self.device, self.ty)?);
            self.available_heap_indices.insert(heap_index);
        }

        let heap = &mut self.heaps[heap_index];
        let handle = Handle {
            raw: heap.alloc_handle()?,
            heap_index,
        };
        if heap.is_full() {
            self.available_heap_indices.remove(heap_index);
        }

        Ok(handle)
    }

    pub(super) fn free_handle(&mut self, handle: Handle) {
        self.heaps[handle.heap_index].free_handle(handle.raw);
        self.available_heap_indices.insert(handle.heap_index);
    }
}

pub(super) struct CpuHeapInner {
    pub _raw: Direct3D12::ID3D12DescriptorHeap,
    pub stage: Vec<Direct3D12::D3D12_CPU_DESCRIPTOR_HANDLE>,
}

pub(super) struct CpuHeap {
    pub inner: Mutex<CpuHeapInner>,
    start: Direct3D12::D3D12_CPU_DESCRIPTOR_HANDLE,
    handle_size: u32,
    total: u32,
}

unsafe impl Send for CpuHeap {}
unsafe impl Sync for CpuHeap {}

impl CpuHeap {
    pub(super) fn new(
        device: &Direct3D12::ID3D12Device,
        ty: Direct3D12::D3D12_DESCRIPTOR_HEAP_TYPE,
        total: u32,
    ) -> Result<Self, crate::DeviceError> {
        let handle_size = unsafe { device.GetDescriptorHandleIncrementSize(ty) };
        let desc = Direct3D12::D3D12_DESCRIPTOR_HEAP_DESC {
            Type: ty,
            NumDescriptors: total,
            Flags: Direct3D12::D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
            NodeMask: 0,
        };
        let raw = unsafe { device.CreateDescriptorHeap::<Direct3D12::ID3D12DescriptorHeap>(&desc) }
            .into_device_result("CPU descriptor heap creation")?;

        let start = unsafe { raw.GetCPUDescriptorHandleForHeapStart() };

        Ok(Self {
            inner: Mutex::new(CpuHeapInner {
                _raw: raw,
                stage: Vec::new(),
            }),
            start,
            handle_size,
            total,
        })
    }

    pub(super) fn at(&self, index: u32) -> Direct3D12::D3D12_CPU_DESCRIPTOR_HANDLE {
        debug_assert!(
            index < self.total,
            "Index ({index}) out of bounds {total}",
            total = self.total
        );
        Direct3D12::D3D12_CPU_DESCRIPTOR_HANDLE {
            ptr: self.start.ptr + (self.handle_size * index) as usize,
        }
    }
}

impl fmt::Debug for CpuHeap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CpuHeap")
            .field("start", &self.start.ptr)
            .field("handle_size", &self.handle_size)
            .field("total", &self.total)
            .finish()
    }
}

pub(super) unsafe fn upload(
    device: &Direct3D12::ID3D12Device,
    src: &CpuHeapInner,
    dst: &GeneralHeap,
    dummy_copy_counts: &[u32],
) -> Result<DualHandle, crate::DeviceError> {
    let count = src.stage.len() as u32;
    let index = dst.allocate_slice(count as u64)?;
    unsafe {
        device.CopyDescriptors(
            1,
            &dst.cpu_descriptor_at(index),
            Some(&count),
            count,
            src.stage.as_ptr(),
            Some(dummy_copy_counts.as_ptr()),
            dst.ty,
        )
    };
    Ok(dst.at(index, count as u64))
}
