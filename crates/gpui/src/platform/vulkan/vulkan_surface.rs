use std::{any::Any, ffi::CStr};

use ash::{vk, Entry, Instance};

pub(crate) trait VulkanExtensionLoader {
    fn as_any(&self) -> &dyn Any;
}

pub(crate) trait VulkanSurface {
    fn extension_name(&self) -> &CStr;
    fn extension_loader(
        &self,
        entry: &Entry,
        instance: &Instance,
    ) -> Box<dyn VulkanExtensionLoader>;
    fn create_surface(&self, loader: &Box<dyn VulkanExtensionLoader>) -> vk::SurfaceKHR;
    fn presentation_support(
        &self,
        loader: &Box<dyn VulkanExtensionLoader>,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
    ) -> bool;
}
