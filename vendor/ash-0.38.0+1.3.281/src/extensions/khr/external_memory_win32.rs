//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_memory_win32.html>

use crate::prelude::*;
use crate::vk;
use core::mem;

impl crate::khr::external_memory_win32::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandleKHR.html>
    #[inline]
    pub unsafe fn get_memory_win32_handle(
        &self,
        create_info: &vk::MemoryGetWin32HandleInfoKHR<'_>,
    ) -> VkResult<vk::HANDLE> {
        let mut handle = mem::MaybeUninit::uninit();
        (self.fp.get_memory_win32_handle_khr)(self.handle, create_info, handle.as_mut_ptr())
            .assume_init_on_success(handle)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandlePropertiesKHR.html>
    #[inline]
    pub unsafe fn get_memory_win32_handle_properties(
        &self,
        handle_type: vk::ExternalMemoryHandleTypeFlags,
        handle: vk::HANDLE,
        memory_win32_handle_properties: &mut vk::MemoryWin32HandlePropertiesKHR<'_>,
    ) -> VkResult<()> {
        (self.fp.get_memory_win32_handle_properties_khr)(
            self.handle,
            handle_type,
            handle,
            memory_win32_handle_properties,
        )
        .result()
    }
}
