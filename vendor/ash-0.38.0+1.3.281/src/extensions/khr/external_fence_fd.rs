//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_fence_fd.html>

use crate::prelude::*;
use crate::vk;
use core::mem;

impl crate::khr::external_fence_fd::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportFenceFdKHR.html>
    #[inline]
    pub unsafe fn import_fence_fd(
        &self,
        import_info: &vk::ImportFenceFdInfoKHR<'_>,
    ) -> VkResult<()> {
        (self.fp.import_fence_fd_khr)(self.handle, import_info).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFenceFdKHR.html>
    #[inline]
    pub unsafe fn get_fence_fd(&self, get_info: &vk::FenceGetFdInfoKHR<'_>) -> VkResult<i32> {
        let mut fd = mem::MaybeUninit::uninit();
        (self.fp.get_fence_fd_khr)(self.handle, get_info, fd.as_mut_ptr())
            .assume_init_on_success(fd)
    }
}
