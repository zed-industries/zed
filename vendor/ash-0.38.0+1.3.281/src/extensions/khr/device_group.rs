//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_device_group.html>

#[cfg(doc)]
use crate::khr;
use crate::prelude::*;
use crate::vk;
use alloc::vec::Vec;
use core::mem;

impl crate::khr::device_group::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPeerMemoryFeaturesKHR.html>
    #[inline]
    pub unsafe fn get_device_group_peer_memory_features(
        &self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
    ) -> vk::PeerMemoryFeatureFlags {
        let mut peer_memory_features = mem::MaybeUninit::uninit();
        (self.fp.get_device_group_peer_memory_features_khr)(
            self.handle,
            heap_index,
            local_device_index,
            remote_device_index,
            peer_memory_features.as_mut_ptr(),
        );
        peer_memory_features.assume_init()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDeviceMaskKHR.html>
    #[inline]
    pub unsafe fn cmd_set_device_mask(&self, command_buffer: vk::CommandBuffer, device_mask: u32) {
        (self.fp.cmd_set_device_mask_khr)(command_buffer, device_mask)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchBaseKHR.html>
    #[inline]
    pub unsafe fn cmd_dispatch_base(
        &self,
        command_buffer: vk::CommandBuffer,
        base_group: (u32, u32, u32),
        group_count: (u32, u32, u32),
    ) {
        (self.fp.cmd_dispatch_base_khr)(
            command_buffer,
            base_group.0,
            base_group.1,
            base_group.2,
            group_count.0,
            group_count.1,
            group_count.2,
        )
    }

    /// Requires [`VK_KHR_surface`] to be enabled.
    ///
    /// Also available as [`khr::swapchain::Device::get_device_group_present_capabilities()`] since [Vulkan 1.1].
    ///
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html>
    ///
    /// [Vulkan 1.1]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html
    /// [`VK_KHR_surface`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface.html
    #[inline]
    pub unsafe fn get_device_group_present_capabilities(
        &self,
        device_group_present_capabilities: &mut vk::DeviceGroupPresentCapabilitiesKHR<'_>,
    ) -> VkResult<()> {
        (self.fp.get_device_group_present_capabilities_khr)(
            self.handle,
            device_group_present_capabilities,
        )
        .result()
    }

    /// Requires [`VK_KHR_surface`] to be enabled.
    ///
    /// Also available as [`khr::swapchain::Device::get_device_group_surface_present_modes()`] since [Vulkan 1.1].
    ///
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html>
    ///
    /// [Vulkan 1.1]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html
    /// [`VK_KHR_surface`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface.html
    #[inline]
    pub unsafe fn get_device_group_surface_present_modes(
        &self,
        surface: vk::SurfaceKHR,
    ) -> VkResult<vk::DeviceGroupPresentModeFlagsKHR> {
        let mut modes = mem::MaybeUninit::uninit();
        (self.fp.get_device_group_surface_present_modes_khr)(
            self.handle,
            surface,
            modes.as_mut_ptr(),
        )
        .assume_init_on_success(modes)
    }

    /// On success, returns the next image's index and whether the swapchain is suboptimal for the surface.
    ///
    /// Requires [`VK_KHR_swapchain`] to be enabled.
    ///
    /// Also available as [`khr::swapchain::Device::acquire_next_image2()`] since [Vulkan 1.1].
    ///
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImage2KHR.html>
    ///
    /// [Vulkan 1.1]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html
    /// [`VK_KHR_swapchain`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_swapchain.html
    #[inline]
    pub unsafe fn acquire_next_image2(
        &self,
        acquire_info: &vk::AcquireNextImageInfoKHR<'_>,
    ) -> VkResult<(u32, bool)> {
        let mut index = mem::MaybeUninit::uninit();
        let err_code =
            (self.fp.acquire_next_image2_khr)(self.handle, acquire_info, index.as_mut_ptr());
        match err_code {
            vk::Result::SUCCESS => Ok((index.assume_init(), false)),
            vk::Result::SUBOPTIMAL_KHR => Ok((index.assume_init(), true)),
            _ => Err(err_code),
        }
    }
}

impl crate::khr::device_group::Instance {
    /// Requires [`VK_KHR_surface`] to be enabled.
    ///
    /// Also available as [`khr::swapchain::Instance::get_physical_device_present_rectangles()`] since [Vulkan 1.1].
    ///
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html>
    ///
    /// [Vulkan 1.1]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html
    /// [`VK_KHR_surface`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface.html
    #[inline]
    pub unsafe fn get_physical_device_present_rectangles(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> VkResult<Vec<vk::Rect2D>> {
        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_physical_device_present_rectangles_khr)(
                physical_device,
                surface,
                count,
                data,
            )
        })
    }
}
