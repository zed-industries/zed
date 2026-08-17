use core::{mem::ManuallyDrop, ops::Deref};

use alloc::sync::Arc;
use hal::DynResource;

use crate::{
    device::Device,
    global::Global,
    id::{
        AdapterId, BlasId, BufferId, CommandEncoderId, DeviceId, QueueId, SurfaceId, TextureId,
        TextureViewId, TlasId,
    },
    lock::{RankData, RwLockReadGuard},
    resource::RawResourceAccess,
    snatch::SnatchGuard,
};

/// A guard which holds alive a wgpu-core resource and dereferences to the Hal type.
struct SimpleResourceGuard<Resource, HalType> {
    _guard: Resource,
    ptr: *const HalType,
}

impl<Resource, HalType> SimpleResourceGuard<Resource, HalType> {
    /// Creates a new guard from a resource, using a callback to derive the Hal type.
    pub fn new<C>(guard: Resource, callback: C) -> Option<Self>
    where
        C: Fn(&Resource) -> Option<&HalType>,
    {
        // Derive the hal type from the resource and coerce it to a pointer.
        let ptr: *const HalType = callback(&guard)?;

        Some(Self { _guard: guard, ptr })
    }
}

impl<Resource, HalType> Deref for SimpleResourceGuard<Resource, HalType> {
    type Target = HalType;

    fn deref(&self) -> &Self::Target {
        // SAFETY: The pointer is guaranteed to be valid as the original resource is
        // still alive and this guard cannot be used with snatchable resources.
        unsafe { &*self.ptr }
    }
}

unsafe impl<Resource, HalType> Send for SimpleResourceGuard<Resource, HalType>
where
    Resource: Send,
    HalType: Send,
{
}
unsafe impl<Resource, HalType> Sync for SimpleResourceGuard<Resource, HalType>
where
    Resource: Sync,
    HalType: Sync,
{
}

/// A guard which holds alive a snatchable wgpu-core resource and dereferences to the Hal type.
struct SnatchableResourceGuard<Resource, HalType>
where
    Resource: RawResourceAccess,
{
    resource: Arc<Resource>,
    snatch_lock_rank_data: ManuallyDrop<RankData>,
    ptr: *const HalType,
}

impl<Resource, HalType> SnatchableResourceGuard<Resource, HalType>
where
    Resource: RawResourceAccess,
    HalType: 'static,
{
    /// Creates a new guard from a snatchable resource.
    ///
    /// Returns `None` if:
    /// - The resource is not of the expected Hal type.
    /// - The resource has been destroyed.
    pub fn new(resource: Arc<Resource>) -> Option<Self> {
        // Grab the snatchable lock.
        let snatch_guard = resource.device().snatchable_lock.read();

        // Get the raw resource and downcast it to the expected Hal type.
        let underlying = resource
            .raw(&snatch_guard)?
            .as_any()
            .downcast_ref::<HalType>()?;

        // Cast the raw resource to a pointer to get rid of the lifetime
        // connecting us to the snatch guard.
        let ptr: *const HalType = underlying;

        // SAFETY: At this point all panicking or divergance has already happened,
        // so we can safely forget the snatch guard without causing the lock to be left open.
        let snatch_lock_rank_data = SnatchGuard::forget(snatch_guard);

        // SAFETY: We only construct this guard while the snatchable lock is held,
        // as the `drop` implementation of this guard will unsafely release the lock.
        Some(Self {
            resource,
            snatch_lock_rank_data: ManuallyDrop::new(snatch_lock_rank_data),
            ptr,
        })
    }
}

impl<Resource, HalType> Deref for SnatchableResourceGuard<Resource, HalType>
where
    Resource: RawResourceAccess,
{
    type Target = HalType;

    fn deref(&self) -> &Self::Target {
        // SAFETY: The pointer is guaranteed to be valid as the original resource is
        // still alive and the snatchable lock is still being held due to the forgotten
        // snatch guard.
        unsafe { &*self.ptr }
    }
}

impl<Resource, HalType> Drop for SnatchableResourceGuard<Resource, HalType>
where
    Resource: RawResourceAccess,
{
    fn drop(&mut self) {
        // SAFETY:
        // - We are not going to access the rank data anymore.
        let data = unsafe { ManuallyDrop::take(&mut self.snatch_lock_rank_data) };

        // SAFETY:
        // - The pointer is no longer going to be accessed.
        // - The snatchable lock is being held because this type was not created
        //   until after the snatchable lock was forgotten.
        unsafe {
            self.resource
                .device()
                .snatchable_lock
                .force_unlock_read(data)
        };
    }
}

unsafe impl<Resource, HalType> Send for SnatchableResourceGuard<Resource, HalType>
where
    Resource: RawResourceAccess + Send,
    HalType: Send,
{
}
unsafe impl<Resource, HalType> Sync for SnatchableResourceGuard<Resource, HalType>
where
    Resource: RawResourceAccess + Sync,
    HalType: Sync,
{
}

/// A guard which holds alive a device and the device's fence lock, dereferencing to the Hal type.
struct FenceGuard<Fence> {
    device: Arc<Device>,
    fence_lock_rank_data: ManuallyDrop<RankData>,
    ptr: *const Fence,
}

impl<Fence> FenceGuard<Fence>
where
    Fence: 'static,
{
    /// Creates a new guard over a device's fence.
    ///
    /// Returns `None` if:
    /// - The device's fence is not of the expected Hal type.
    pub fn new(device: Arc<Device>) -> Option<Self> {
        // Grab the fence lock.
        let fence_guard = device.fence.read();

        // Get the raw fence and downcast it to the expected Hal type, coercing it to a pointer
        // to get rid of the lifetime connecting us to the fence guard.
        let ptr: *const Fence = fence_guard.as_any().downcast_ref::<Fence>()?;

        // SAFETY: At this point all panicking or divergance has already happened,
        // so we can safely forget the fence guard without causing the lock to be left open.
        let fence_lock_rank_data = RwLockReadGuard::forget(fence_guard);

        // SAFETY: We only construct this guard while the fence lock is held,
        // as the `drop` implementation of this guard will unsafely release the lock.
        Some(Self {
            device,
            fence_lock_rank_data: ManuallyDrop::new(fence_lock_rank_data),
            ptr,
        })
    }
}

impl<Fence> Deref for FenceGuard<Fence> {
    type Target = Fence;

    fn deref(&self) -> &Self::Target {
        // SAFETY: The pointer is guaranteed to be valid as the original device's fence
        // is still alive and the fence lock is still being held due to the forgotten
        // fence guard.
        unsafe { &*self.ptr }
    }
}

impl<Fence> Drop for FenceGuard<Fence> {
    fn drop(&mut self) {
        // SAFETY:
        // - We are not going to access the rank data anymore.
        let data = unsafe { ManuallyDrop::take(&mut self.fence_lock_rank_data) };

        // SAFETY:
        // - The pointer is no longer going to be accessed.
        // - The fence lock is being held because this type was not created
        //   until after the fence lock was forgotten.
        unsafe {
            self.device.fence.force_unlock_read(data);
        };
    }
}

unsafe impl<Fence> Send for FenceGuard<Fence> where Fence: Send {}
unsafe impl<Fence> Sync for FenceGuard<Fence> where Fence: Sync {}

impl Global {
    /// # Safety
    ///
    /// - The raw buffer handle must not be manually destroyed
    pub unsafe fn buffer_as_hal<A: hal::Api>(
        &self,
        id: BufferId,
    ) -> Option<impl Deref<Target = A::Buffer>> {
        profiling::scope!("Buffer::as_hal");

        let hub = &self.hub;

        let buffer = hub.buffers.get(id).get().ok()?;

        SnatchableResourceGuard::new(buffer)
    }

    /// # Safety
    ///
    /// - The raw texture handle must not be manually destroyed
    pub unsafe fn texture_as_hal<A: hal::Api>(
        &self,
        id: TextureId,
    ) -> Option<impl Deref<Target = A::Texture>> {
        profiling::scope!("Texture::as_hal");

        let hub = &self.hub;

        let texture = hub.textures.get(id).get().ok()?;

        SnatchableResourceGuard::new(texture)
    }

    /// # Safety
    ///
    /// - The raw texture view handle must not be manually destroyed
    pub unsafe fn texture_view_as_hal<A: hal::Api>(
        &self,
        id: TextureViewId,
    ) -> Option<impl Deref<Target = A::TextureView>> {
        profiling::scope!("TextureView::as_hal");

        let hub = &self.hub;

        let view = hub.texture_views.get(id).get().ok()?;

        SnatchableResourceGuard::new(view)
    }

    /// # Safety
    ///
    /// - The raw adapter handle must not be manually destroyed
    pub unsafe fn adapter_as_hal<A: hal::Api>(
        &self,
        id: AdapterId,
    ) -> Option<impl Deref<Target = A::Adapter>> {
        profiling::scope!("Adapter::as_hal");

        let hub = &self.hub;
        let adapter = hub.adapters.get(id);

        SimpleResourceGuard::new(adapter, move |adapter| {
            adapter.raw.adapter.as_any().downcast_ref()
        })
    }

    /// # Safety
    ///
    /// - The raw device handle must not be manually destroyed
    pub unsafe fn device_as_hal<A: hal::Api>(
        &self,
        id: DeviceId,
    ) -> Option<impl Deref<Target = A::Device>> {
        profiling::scope!("Device::as_hal");

        let device = self.hub.devices.get(id);

        SimpleResourceGuard::new(device, move |device| device.raw().as_any().downcast_ref())
    }

    /// # Safety
    ///
    /// - The raw fence handle must not be manually destroyed
    pub unsafe fn device_fence_as_hal<A: hal::Api>(
        &self,
        id: DeviceId,
    ) -> Option<impl Deref<Target = A::Fence>> {
        profiling::scope!("Device::fence_as_hal");

        let device = self.hub.devices.get(id);

        FenceGuard::new(device)
    }

    /// # Safety
    /// - The raw surface handle must not be manually destroyed
    pub unsafe fn surface_as_hal<A: hal::Api>(
        &self,
        id: SurfaceId,
    ) -> Option<impl Deref<Target = A::Surface>> {
        profiling::scope!("Surface::as_hal");

        let surface = self.surfaces.get(id);

        SimpleResourceGuard::new(surface, move |surface| {
            surface.raw(A::VARIANT)?.as_any().downcast_ref()
        })
    }

    /// Encode commands using the raw HAL command encoder.
    ///
    /// # Panics
    ///
    /// If the command encoder has already been used with the wgpu encoding API.
    ///
    /// # Safety
    ///
    /// - The raw command encoder handle must not be manually destroyed
    pub unsafe fn command_encoder_as_hal_mut<
        A: hal::Api,
        F: FnOnce(Option<&mut A::CommandEncoder>) -> R,
        R,
    >(
        &self,
        id: CommandEncoderId,
        hal_command_encoder_callback: F,
    ) -> R {
        profiling::scope!("CommandEncoder::as_hal");

        let hub = &self.hub;

        let cmd_enc = hub.command_encoders.get(id);
        let mut cmd_buf_data = cmd_enc.data.lock();
        cmd_buf_data.record_as_hal_mut(|opt_cmd_buf| -> R {
            hal_command_encoder_callback(opt_cmd_buf.and_then(|cmd_buf| {
                cmd_buf
                    .encoder
                    .open()
                    .ok()
                    .and_then(|encoder| encoder.as_any_mut().downcast_mut())
            }))
        })
    }

    /// # Safety
    ///
    /// - The raw queue handle must not be manually destroyed
    pub unsafe fn queue_as_hal<A: hal::Api>(
        &self,
        id: QueueId,
    ) -> Option<impl Deref<Target = A::Queue>> {
        profiling::scope!("Queue::as_hal");

        let queue = self.hub.queues.get(id);

        SimpleResourceGuard::new(queue, move |queue| queue.raw().as_any().downcast_ref())
    }

    /// # Safety
    ///
    /// - The raw blas handle must not be manually destroyed
    pub unsafe fn blas_as_hal<A: hal::Api>(
        &self,
        id: BlasId,
    ) -> Option<impl Deref<Target = A::AccelerationStructure>> {
        profiling::scope!("Blas::as_hal");

        let hub = &self.hub;

        let blas = hub.blas_s.get(id).get().ok()?;

        SnatchableResourceGuard::new(blas)
    }

    /// # Safety
    ///
    /// - The raw tlas handle must not be manually destroyed
    pub unsafe fn tlas_as_hal<A: hal::Api>(
        &self,
        id: TlasId,
    ) -> Option<impl Deref<Target = A::AccelerationStructure>> {
        profiling::scope!("Tlas::as_hal");

        let hub = &self.hub;

        let tlas = hub.tlas_s.get(id).get().ok()?;

        SnatchableResourceGuard::new(tlas)
    }
}
