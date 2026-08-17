//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_timeline_semaphore.html>

use crate::prelude::*;
use crate::vk;
use core::mem;

impl crate::khr::timeline_semaphore::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreCounterValue.html>
    #[inline]
    pub unsafe fn get_semaphore_counter_value(&self, semaphore: vk::Semaphore) -> VkResult<u64> {
        let mut value = mem::MaybeUninit::uninit();
        (self.fp.get_semaphore_counter_value_khr)(self.handle, semaphore, value.as_mut_ptr())
            .assume_init_on_success(value)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWaitSemaphores.html>
    #[inline]
    pub unsafe fn wait_semaphores(
        &self,
        wait_info: &vk::SemaphoreWaitInfo<'_>,
        timeout: u64,
    ) -> VkResult<()> {
        (self.fp.wait_semaphores_khr)(self.handle, wait_info, timeout).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSignalSemaphore.html>
    #[inline]
    pub unsafe fn signal_semaphore(
        &self,
        signal_info: &vk::SemaphoreSignalInfo<'_>,
    ) -> VkResult<()> {
        (self.fp.signal_semaphore_khr)(self.handle, signal_info).result()
    }
}
