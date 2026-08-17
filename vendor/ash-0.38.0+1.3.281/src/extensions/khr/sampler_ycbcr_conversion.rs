//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_sampler_ycbcr_conversion.html>

use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use core::mem;

impl crate::khr::sampler_ycbcr_conversion::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSamplerYcbcrConversion.html>
    #[inline]
    pub unsafe fn create_sampler_ycbcr_conversion(
        &self,
        create_info: &vk::SamplerYcbcrConversionCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::SamplerYcbcrConversion> {
        let mut ycbcr_conversion = mem::MaybeUninit::uninit();
        (self.fp.create_sampler_ycbcr_conversion_khr)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            ycbcr_conversion.as_mut_ptr(),
        )
        .assume_init_on_success(ycbcr_conversion)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySamplerYcbcrConversion.html>
    #[inline]
    pub unsafe fn destroy_sampler_ycbcr_conversion(
        &self,
        ycbcr_conversion: vk::SamplerYcbcrConversion,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.fp.destroy_sampler_ycbcr_conversion_khr)(
            self.handle,
            ycbcr_conversion,
            allocation_callbacks.as_raw_ptr(),
        )
    }
}
