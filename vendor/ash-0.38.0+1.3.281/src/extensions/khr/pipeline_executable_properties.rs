//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_pipeline_executable_properties.html>

use crate::prelude::*;
use crate::vk;
use alloc::vec::Vec;

impl crate::khr::pipeline_executable_properties::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableInternalRepresentationsKHR.html>
    #[inline]
    pub unsafe fn get_pipeline_executable_internal_representations(
        &self,
        executable_info: &vk::PipelineExecutableInfoKHR<'_>,
    ) -> VkResult<Vec<vk::PipelineExecutableInternalRepresentationKHR<'_>>> {
        read_into_defaulted_vector(|count, data| {
            (self.fp.get_pipeline_executable_internal_representations_khr)(
                self.handle,
                executable_info,
                count,
                data,
            )
        })
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutablePropertiesKHR.html>
    #[inline]
    pub unsafe fn get_pipeline_executable_properties(
        &self,
        pipeline_info: &vk::PipelineInfoKHR<'_>,
    ) -> VkResult<Vec<vk::PipelineExecutablePropertiesKHR<'_>>> {
        read_into_defaulted_vector(|count, data| {
            (self.fp.get_pipeline_executable_properties_khr)(
                self.handle,
                pipeline_info,
                count,
                data,
            )
        })
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableStatisticsKHR.html>
    #[inline]
    pub unsafe fn get_pipeline_executable_statistics(
        &self,
        executable_info: &vk::PipelineExecutableInfoKHR<'_>,
    ) -> VkResult<Vec<vk::PipelineExecutableStatisticKHR<'_>>> {
        read_into_defaulted_vector(|count, data| {
            (self.fp.get_pipeline_executable_statistics_khr)(
                self.handle,
                executable_info,
                count,
                data,
            )
        })
    }
}
