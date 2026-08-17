//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_buffer_device_address.html>

use crate::vk;

impl crate::khr::buffer_device_address::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddressKHR.html>
    #[inline]
    pub unsafe fn get_buffer_device_address(
        &self,
        info: &vk::BufferDeviceAddressInfoKHR<'_>,
    ) -> vk::DeviceAddress {
        (self.fp.get_buffer_device_address_khr)(self.handle, info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferOpaqueCaptureAddressKHR.html>
    #[inline]
    pub unsafe fn get_buffer_opaque_capture_address(
        &self,
        info: &vk::BufferDeviceAddressInfoKHR<'_>,
    ) -> u64 {
        (self.fp.get_buffer_opaque_capture_address_khr)(self.handle, info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddressKHR.html>
    #[inline]
    pub unsafe fn get_device_memory_opaque_capture_address(
        &self,
        info: &vk::DeviceMemoryOpaqueCaptureAddressInfoKHR<'_>,
    ) -> u64 {
        (self.fp.get_device_memory_opaque_capture_address_khr)(self.handle, info)
    }
}
