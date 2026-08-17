use alloc::{boxed::Box, vec::Vec};

use crate::{Api, Capabilities, ExposedAdapter, Instance, InstanceError};

use super::{DynAdapter, DynResource, DynResourceExt as _, DynSurface};

pub struct DynExposedAdapter {
    pub adapter: Box<dyn DynAdapter>,
    pub info: wgt::AdapterInfo,
    pub features: wgt::Features,
    pub capabilities: Capabilities,
}

impl DynExposedAdapter {
    /// Returns the backend this adapter is using.
    pub fn backend(&self) -> wgt::Backend {
        self.info.backend
    }
}

impl<A: Api> From<ExposedAdapter<A>> for DynExposedAdapter {
    fn from(exposed_adapter: ExposedAdapter<A>) -> Self {
        Self {
            adapter: Box::new(exposed_adapter.adapter),
            info: exposed_adapter.info,
            features: exposed_adapter.features,
            capabilities: exposed_adapter.capabilities,
        }
    }
}

pub trait DynInstance: DynResource {
    unsafe fn create_surface(
        &self,
        display_handle: raw_window_handle::RawDisplayHandle,
        window_handle: raw_window_handle::RawWindowHandle,
    ) -> Result<Box<dyn DynSurface>, InstanceError>;

    unsafe fn enumerate_adapters(
        &self,
        surface_hint: Option<&dyn DynSurface>,
    ) -> Vec<DynExposedAdapter>;
}

impl<I: Instance + DynResource> DynInstance for I {
    unsafe fn create_surface(
        &self,
        display_handle: raw_window_handle::RawDisplayHandle,
        window_handle: raw_window_handle::RawWindowHandle,
    ) -> Result<Box<dyn DynSurface>, InstanceError> {
        unsafe { I::create_surface(self, display_handle, window_handle) }
            .map(|surface| -> Box<dyn DynSurface> { Box::new(surface) })
    }

    unsafe fn enumerate_adapters(
        &self,
        surface_hint: Option<&dyn DynSurface>,
    ) -> Vec<DynExposedAdapter> {
        let surface_hint = surface_hint.map(|s| s.expect_downcast_ref());
        unsafe { I::enumerate_adapters(self, surface_hint) }
            .into_iter()
            .map(|exposed| DynExposedAdapter {
                adapter: Box::new(exposed.adapter),
                info: exposed.info,
                features: exposed.features,
                capabilities: exposed.capabilities,
            })
            .collect()
    }
}
