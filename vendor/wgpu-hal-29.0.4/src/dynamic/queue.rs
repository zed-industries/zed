use alloc::{boxed::Box, vec::Vec};

use crate::{
    DeviceError, DynCommandBuffer, DynFence, DynResource, DynSurface, DynSurfaceTexture,
    FenceValue, Queue, SurfaceError,
};

use super::DynResourceExt as _;

pub trait DynQueue: DynResource {
    unsafe fn submit(
        &self,
        command_buffers: &[&dyn DynCommandBuffer],
        surface_textures: &[&dyn DynSurfaceTexture],
        signal_fence: (&mut dyn DynFence, FenceValue),
    ) -> Result<(), DeviceError>;
    unsafe fn present(
        &self,
        surface: &dyn DynSurface,
        texture: Box<dyn DynSurfaceTexture>,
    ) -> Result<(), SurfaceError>;
    unsafe fn get_timestamp_period(&self) -> f32;
}

impl<Q: Queue + DynResource> DynQueue for Q {
    unsafe fn submit(
        &self,
        command_buffers: &[&dyn DynCommandBuffer],
        surface_textures: &[&dyn DynSurfaceTexture],
        signal_fence: (&mut dyn DynFence, FenceValue),
    ) -> Result<(), DeviceError> {
        let command_buffers = command_buffers
            .iter()
            .map(|cb| (*cb).expect_downcast_ref())
            .collect::<Vec<_>>();
        let surface_textures = surface_textures
            .iter()
            .map(|surface| (*surface).expect_downcast_ref())
            .collect::<Vec<_>>();
        let signal_fence = (signal_fence.0.expect_downcast_mut(), signal_fence.1);
        unsafe { Q::submit(self, &command_buffers, &surface_textures, signal_fence) }
    }

    unsafe fn present(
        &self,
        surface: &dyn DynSurface,
        texture: Box<dyn DynSurfaceTexture>,
    ) -> Result<(), SurfaceError> {
        let surface = surface.expect_downcast_ref();
        unsafe { Q::present(self, surface, texture.unbox()) }
    }

    unsafe fn get_timestamp_period(&self) -> f32 {
        unsafe { Q::get_timestamp_period(self) }
    }
}
