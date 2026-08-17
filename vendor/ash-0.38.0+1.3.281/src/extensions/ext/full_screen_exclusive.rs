//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_full_screen_exclusive.html>

use crate::prelude::*;
use crate::vk;
use alloc::vec::Vec;
use core::mem;

impl crate::ext::full_screen_exclusive::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireFullScreenExclusiveModeEXT.html>
    #[inline]
    pub unsafe fn acquire_full_screen_exclusive_mode(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> VkResult<()> {
        (self.fp.acquire_full_screen_exclusive_mode_ext)(self.handle, swapchain).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleaseFullScreenExclusiveModeEXT.html>
    #[inline]
    pub unsafe fn release_full_screen_exclusive_mode(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> VkResult<()> {
        (self.fp.release_full_screen_exclusive_mode_ext)(self.handle, swapchain).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html>
    #[inline]
    pub unsafe fn get_device_group_surface_present_modes2(
        &self,
        surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR<'_>,
    ) -> VkResult<vk::DeviceGroupPresentModeFlagsKHR> {
        let mut present_modes = mem::MaybeUninit::uninit();
        (self.fp.get_device_group_surface_present_modes2_ext)(
            self.handle,
            surface_info,
            present_modes.as_mut_ptr(),
        )
        .assume_init_on_success(present_modes)
    }
}

impl crate::ext::full_screen_exclusive::Instance {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html>
    #[inline]
    pub unsafe fn get_physical_device_surface_present_modes2(
        &self,
        physical_device: vk::PhysicalDevice,
        surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR<'_>,
    ) -> VkResult<Vec<vk::PresentModeKHR>> {
        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_physical_device_surface_present_modes2_ext)(
                physical_device,
                surface_info,
                count,
                data,
            )
        })
    }
}
