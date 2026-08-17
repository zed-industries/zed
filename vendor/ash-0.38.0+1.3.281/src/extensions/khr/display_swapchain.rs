//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_display_swapchain.html>

use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use alloc::vec::Vec;

impl crate::khr::display_swapchain::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSharedSwapchainsKHR.html>
    #[inline]
    pub unsafe fn create_shared_swapchains(
        &self,
        create_infos: &[vk::SwapchainCreateInfoKHR<'_>],
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<Vec<vk::SwapchainKHR>> {
        let mut swapchains = Vec::with_capacity(create_infos.len());
        (self.fp.create_shared_swapchains_khr)(
            self.handle,
            create_infos.len() as u32,
            create_infos.as_ptr(),
            allocation_callbacks.as_raw_ptr(),
            swapchains.as_mut_ptr(),
        )
        .set_vec_len_on_success(swapchains, create_infos.len())
    }
}
