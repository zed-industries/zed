//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_cooperative_matrix.html>

use crate::prelude::*;
use crate::vk;
use alloc::vec::Vec;

impl crate::khr::cooperative_matrix::Instance {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_cooperative_matrix_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> VkResult<Vec<vk::CooperativeMatrixPropertiesKHR<'_>>> {
        read_into_defaulted_vector(|count, data| {
            (self
                .fp
                .get_physical_device_cooperative_matrix_properties_khr)(
                physical_device,
                count,
                data,
            )
        })
    }
}
