//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_ANDROID_external_memory_android_hardware_buffer.html>

use crate::prelude::*;
use crate::vk;
use core::mem;

impl crate::android::external_memory_android_hardware_buffer::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAndroidHardwareBufferPropertiesANDROID.html>
    #[inline]
    pub unsafe fn get_android_hardware_buffer_properties(
        &self,
        buffer: *const vk::AHardwareBuffer,
        properties: &mut vk::AndroidHardwareBufferPropertiesANDROID<'_>,
    ) -> VkResult<()> {
        (self.fp.get_android_hardware_buffer_properties_android)(self.handle, buffer, properties)
            .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryAndroidHardwareBufferANDROID.html>
    #[inline]
    pub unsafe fn get_memory_android_hardware_buffer(
        &self,
        info: &vk::MemoryGetAndroidHardwareBufferInfoANDROID<'_>,
    ) -> VkResult<*mut vk::AHardwareBuffer> {
        let mut buffer = mem::MaybeUninit::uninit();
        (self.fp.get_memory_android_hardware_buffer_android)(self.handle, info, buffer.as_mut_ptr())
            .assume_init_on_success(buffer)
    }
}
