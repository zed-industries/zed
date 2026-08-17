//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_maintenance6.html>

use crate::vk;

impl crate::khr::maintenance6::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorSets2KHR.html>
    #[inline]
    pub unsafe fn cmd_bind_descriptor_sets2(
        &self,
        command_buffer: vk::CommandBuffer,
        bind_descriptor_sets_info: &vk::BindDescriptorSetsInfoKHR<'_>,
    ) {
        (self.fp.cmd_bind_descriptor_sets2_khr)(command_buffer, bind_descriptor_sets_info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPushConstants2KHR.html>
    #[inline]
    pub unsafe fn cmd_push_constants2(
        &self,
        command_buffer: vk::CommandBuffer,
        push_constants_info: &vk::PushConstantsInfoKHR<'_>,
    ) {
        (self.fp.cmd_push_constants2_khr)(command_buffer, push_constants_info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSet2KHR.html>
    #[inline]
    pub unsafe fn cmd_push_descriptor_set2(
        &self,
        command_buffer: vk::CommandBuffer,
        push_descriptor_set_info: &vk::PushDescriptorSetInfoKHR<'_>,
    ) {
        (self.fp.cmd_push_descriptor_set2_khr)(command_buffer, push_descriptor_set_info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetWithTemplate2KHR.html>
    #[inline]
    pub unsafe fn cmd_push_descriptor_set_with_template2(
        &self,
        command_buffer: vk::CommandBuffer,
        push_descriptor_set_with_template_info: &vk::PushDescriptorSetWithTemplateInfoKHR<'_>,
    ) {
        (self.fp.cmd_push_descriptor_set_with_template2_khr)(
            command_buffer,
            push_descriptor_set_with_template_info,
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDescriptorBufferOffsets2EXT.html>
    #[inline]
    pub unsafe fn cmd_set_descriptor_buffer_offsets2(
        &self,
        command_buffer: vk::CommandBuffer,
        set_descriptor_buffer_offsets_info: &vk::SetDescriptorBufferOffsetsInfoEXT<'_>,
    ) {
        (self.fp.cmd_set_descriptor_buffer_offsets2_ext)(
            command_buffer,
            set_descriptor_buffer_offsets_info,
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorBufferEmbeddedSamplers2EXT.html>
    #[inline]
    pub unsafe fn cmd_bind_descriptor_buffer_embedded_samplers2(
        &self,
        command_buffer: vk::CommandBuffer,
        bind_descriptor_buffer_embedded_samplers_info: &vk::BindDescriptorBufferEmbeddedSamplersInfoEXT<'_>,
    ) {
        (self.fp.cmd_bind_descriptor_buffer_embedded_samplers2_ext)(
            command_buffer,
            bind_descriptor_buffer_embedded_samplers_info,
        )
    }
}
