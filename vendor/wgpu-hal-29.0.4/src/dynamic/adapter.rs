use alloc::boxed::Box;

use crate::{
    Adapter, Api, DeviceError, OpenDevice, SurfaceCapabilities, TextureFormatCapabilities,
};

use super::{DynDevice, DynQueue, DynResource, DynResourceExt, DynSurface};

pub struct DynOpenDevice {
    pub device: Box<dyn DynDevice>,
    pub queue: Box<dyn DynQueue>,
}

impl<A: Api> From<OpenDevice<A>> for DynOpenDevice {
    fn from(open_device: OpenDevice<A>) -> Self {
        Self {
            device: Box::new(open_device.device),
            queue: Box::new(open_device.queue),
        }
    }
}

pub trait DynAdapter: DynResource {
    unsafe fn open(
        &self,
        features: wgt::Features,
        limits: &wgt::Limits,
        memory_hints: &wgt::MemoryHints,
    ) -> Result<DynOpenDevice, DeviceError>;

    unsafe fn texture_format_capabilities(
        &self,
        format: wgt::TextureFormat,
    ) -> TextureFormatCapabilities;

    unsafe fn surface_capabilities(&self, surface: &dyn DynSurface) -> Option<SurfaceCapabilities>;

    unsafe fn get_presentation_timestamp(&self) -> wgt::PresentationTimestamp;

    fn get_ordered_buffer_usages(&self) -> wgt::BufferUses;

    fn get_ordered_texture_usages(&self) -> wgt::TextureUses;
}

impl<A: Adapter + DynResource> DynAdapter for A {
    unsafe fn open(
        &self,
        features: wgt::Features,
        limits: &wgt::Limits,
        memory_hints: &wgt::MemoryHints,
    ) -> Result<DynOpenDevice, DeviceError> {
        unsafe { A::open(self, features, limits, memory_hints) }.map(|open_device| DynOpenDevice {
            device: Box::new(open_device.device),
            queue: Box::new(open_device.queue),
        })
    }

    unsafe fn texture_format_capabilities(
        &self,
        format: wgt::TextureFormat,
    ) -> TextureFormatCapabilities {
        unsafe { A::texture_format_capabilities(self, format) }
    }

    unsafe fn surface_capabilities(&self, surface: &dyn DynSurface) -> Option<SurfaceCapabilities> {
        let surface = surface.expect_downcast_ref();
        unsafe { A::surface_capabilities(self, surface) }
    }

    unsafe fn get_presentation_timestamp(&self) -> wgt::PresentationTimestamp {
        unsafe { A::get_presentation_timestamp(self) }
    }

    fn get_ordered_buffer_usages(&self) -> wgt::BufferUses {
        A::get_ordered_buffer_usages(self)
    }

    fn get_ordered_texture_usages(&self) -> wgt::TextureUses {
        A::get_ordered_texture_usages(self)
    }
}
