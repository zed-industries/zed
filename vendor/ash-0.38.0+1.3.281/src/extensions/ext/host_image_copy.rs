//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_host_image_copy.html>

use crate::prelude::*;
use crate::vk;
#[cfg(doc)]
use crate::{ext, khr};

impl crate::ext::host_image_copy::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCopyMemoryToImageEXT.html>
    #[inline]
    pub unsafe fn copy_memory_to_image(
        &self,
        copy_memory_to_image_info: &vk::CopyMemoryToImageInfoEXT<'_>,
    ) -> VkResult<()> {
        (self.fp.copy_memory_to_image_ext)(self.handle, copy_memory_to_image_info).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCopyImageToMemoryEXT.html>
    #[inline]
    pub unsafe fn copy_image_to_memory(
        &self,
        copy_image_to_memory_info: &vk::CopyImageToMemoryInfoEXT<'_>,
    ) -> VkResult<()> {
        (self.fp.copy_image_to_memory_ext)(self.handle, copy_image_to_memory_info).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCopyImageToImageEXT.html>
    #[inline]
    pub unsafe fn copy_image_to_image(
        &self,
        copy_image_to_image_info: &vk::CopyImageToImageInfoEXT<'_>,
    ) -> VkResult<()> {
        (self.fp.copy_image_to_image_ext)(self.handle, copy_image_to_image_info).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkTransitionImageLayoutEXT.html>
    #[inline]
    pub unsafe fn transition_image_layout(
        &self,
        transitions: &[vk::HostImageLayoutTransitionInfoEXT<'_>],
    ) -> VkResult<()> {
        (self.fp.transition_image_layout_ext)(
            self.handle,
            transitions.len() as u32,
            transitions.as_ptr(),
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout2EXT.html>
    ///
    /// Also available as [`khr::maintenance5::Device::get_image_subresource_layout2()`]
    /// when [`VK_KHR_maintenance5`] is enabled.
    ///
    /// Also available as [`ext::image_compression_control::Device::get_image_subresource_layout2()`]
    /// when [`VK_EXT_image_compression_control`] is enabled.
    ///
    /// [`VK_KHR_maintenance5`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_maintenance5.html
    /// [`VK_EXT_image_compression_control`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_image_compression_control.html
    #[inline]
    pub unsafe fn get_image_subresource_layout2(
        &self,
        image: vk::Image,
        subresource: &vk::ImageSubresource2EXT<'_>,
        layout: &mut vk::SubresourceLayout2EXT<'_>,
    ) {
        (self.fp.get_image_subresource_layout2_ext)(self.handle, image, subresource, layout)
    }
}
