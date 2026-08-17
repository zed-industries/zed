//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_pipeline_properties.html>

use crate::prelude::*;
use crate::vk;

impl crate::ext::pipeline_properties::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelinePropertiesEXT.html>
    #[inline]
    pub unsafe fn get_pipeline_properties(
        &self,
        pipeline_info: &vk::PipelineInfoEXT<'_>,
        pipeline_properties: &mut impl crate::ext::pipeline_properties::GetPipelinePropertiesEXTParamPipelineProperties,
    ) -> VkResult<()> {
        (self.fp.get_pipeline_properties_ext)(
            self.handle,
            pipeline_info,
            <*mut _>::cast(pipeline_properties),
        )
        .result()
    }
}
