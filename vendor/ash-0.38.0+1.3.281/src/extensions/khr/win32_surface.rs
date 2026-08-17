//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_win32_surface.html>

use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use core::mem;

impl crate::khr::win32_surface::Instance {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateWin32SurfaceKHR.html>
    #[inline]
    pub unsafe fn create_win32_surface(
        &self,
        create_info: &vk::Win32SurfaceCreateInfoKHR<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::SurfaceKHR> {
        let mut surface = mem::MaybeUninit::uninit();
        (self.fp.create_win32_surface_khr)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            surface.as_mut_ptr(),
        )
        .assume_init_on_success(surface)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWin32PresentationSupportKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_win32_presentation_support(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
    ) -> bool {
        let b = (self.fp.get_physical_device_win32_presentation_support_khr)(
            physical_device,
            queue_family_index,
        );

        b > 0
    }
}
