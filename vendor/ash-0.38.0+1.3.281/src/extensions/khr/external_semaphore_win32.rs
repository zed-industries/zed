//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_semaphore_win32.html>

use crate::prelude::*;
use crate::vk;
use core::mem;

impl crate::khr::external_semaphore_win32::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreWin32HandleKHR.html>
    #[inline]
    pub unsafe fn import_semaphore_win32_handle(
        &self,
        import_info: &vk::ImportSemaphoreWin32HandleInfoKHR<'_>,
    ) -> VkResult<()> {
        (self.fp.import_semaphore_win32_handle_khr)(self.handle, import_info).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreWin32HandleKHR.html>
    #[inline]
    pub unsafe fn get_semaphore_win32_handle(
        &self,
        get_info: &vk::SemaphoreGetWin32HandleInfoKHR<'_>,
    ) -> VkResult<vk::HANDLE> {
        let mut handle = mem::MaybeUninit::uninit();
        (self.fp.get_semaphore_win32_handle_khr)(self.handle, get_info, handle.as_mut_ptr())
            .assume_init_on_success(handle)
    }
}
