//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_fence_win32.html>

use crate::prelude::*;
use crate::vk;
use core::mem;

impl crate::khr::external_fence_win32::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportFenceWin32HandleKHR.html>
    #[inline]
    pub unsafe fn import_fence_win32_handle(
        &self,
        import_info: &vk::ImportFenceWin32HandleInfoKHR<'_>,
    ) -> VkResult<()> {
        (self.fp.import_fence_win32_handle_khr)(self.handle, import_info).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFenceWin32HandleKHR.html>
    #[inline]
    pub unsafe fn get_fence_win32_handle(
        &self,
        get_info: &vk::FenceGetWin32HandleInfoKHR<'_>,
    ) -> VkResult<vk::HANDLE> {
        let mut handle = mem::MaybeUninit::uninit();
        (self.fp.get_fence_win32_handle_khr)(self.handle, get_info, handle.as_mut_ptr())
            .assume_init_on_success(handle)
    }
}
