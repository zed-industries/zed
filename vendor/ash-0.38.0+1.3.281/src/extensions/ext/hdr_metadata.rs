//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_hdr_metadata.html>

use crate::vk;

impl crate::ext::hdr_metadata::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetHdrMetadataEXT.html>
    #[inline]
    pub unsafe fn set_hdr_metadata(
        &self,
        swapchains: &[vk::SwapchainKHR],
        metadata: &[vk::HdrMetadataEXT<'_>],
    ) {
        assert_eq!(swapchains.len(), metadata.len());
        (self.fp.set_hdr_metadata_ext)(
            self.handle,
            swapchains.len() as u32,
            swapchains.as_ptr(),
            metadata.as_ptr(),
        )
    }
}
