//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_buffer_device_address.html>

use crate::vk;

impl crate::ext::buffer_device_address::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddressEXT.html>
    #[inline]
    pub unsafe fn get_buffer_device_address(
        &self,
        info: &vk::BufferDeviceAddressInfoEXT<'_>,
    ) -> vk::DeviceAddress {
        (self.fp.get_buffer_device_address_ext)(self.handle, info)
    }
}
