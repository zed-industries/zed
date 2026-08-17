//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_coverage_reduction_mode.html>

use crate::prelude::*;
use crate::vk;
use core::mem;
use core::ptr;

impl crate::nv::coverage_reduction_mode::Instance {
    /// Retrieve the number of elements to pass to [`get_physical_device_supported_framebuffer_mixed_samples_combinations()`][Self::get_physical_device_supported_framebuffer_mixed_samples_combinations()]
    #[inline]
    pub unsafe fn get_physical_device_supported_framebuffer_mixed_samples_combinations_len(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> VkResult<usize> {
        let mut count = mem::MaybeUninit::uninit();
        (self
            .fp
            .get_physical_device_supported_framebuffer_mixed_samples_combinations_nv)(
            physical_device,
            count.as_mut_ptr(),
            ptr::null_mut(),
        )
        .assume_init_on_success(count)
        .map(|c| c as usize)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html>
    ///
    /// Call [`get_physical_device_supported_framebuffer_mixed_samples_combinations_len()`][Self::get_physical_device_supported_framebuffer_mixed_samples_combinations_len()] to query the number of elements to pass to `out`.
    /// Be sure to [`Default::default()`]-initialize these elements and optionally set their `p_next` pointer.
    #[inline]
    pub unsafe fn get_physical_device_supported_framebuffer_mixed_samples_combinations(
        &self,
        physical_device: vk::PhysicalDevice,
        out: &mut [vk::FramebufferMixedSamplesCombinationNV<'_>],
    ) -> VkResult<()> {
        let mut count = out.len() as u32;
        (self
            .fp
            .get_physical_device_supported_framebuffer_mixed_samples_combinations_nv)(
            physical_device,
            &mut count,
            out.as_mut_ptr(),
        )
        .result()?;
        assert_eq!(count as usize, out.len());
        Ok(())
    }
}
