//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_descriptor_buffer.html>

use crate::prelude::*;
use crate::vk;
use core::mem;

impl crate::ext::descriptor_buffer::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSizeEXT.html>
    #[inline]
    pub unsafe fn get_descriptor_set_layout_size(
        &self,
        layout: vk::DescriptorSetLayout,
    ) -> vk::DeviceSize {
        let mut count = mem::MaybeUninit::uninit();
        (self.fp.get_descriptor_set_layout_size_ext)(self.handle, layout, count.as_mut_ptr());
        count.assume_init()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutBindingOffsetEXT.html>
    #[inline]
    pub unsafe fn get_descriptor_set_layout_binding_offset(
        &self,
        layout: vk::DescriptorSetLayout,
        binding: u32,
    ) -> vk::DeviceSize {
        let mut offset = mem::MaybeUninit::uninit();
        (self.fp.get_descriptor_set_layout_binding_offset_ext)(
            self.handle,
            layout,
            binding,
            offset.as_mut_ptr(),
        );
        offset.assume_init()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorEXT.html>
    #[inline]
    pub unsafe fn get_descriptor(
        &self,
        descriptor_info: &vk::DescriptorGetInfoEXT<'_>,
        descriptor: &mut [u8],
    ) {
        (self.fp.get_descriptor_ext)(
            self.handle,
            descriptor_info,
            descriptor.len(),
            descriptor.as_mut_ptr().cast(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorBuffersEXT.html>
    #[inline]
    pub unsafe fn cmd_bind_descriptor_buffers(
        &self,
        command_buffer: vk::CommandBuffer,
        binding_info: &[vk::DescriptorBufferBindingInfoEXT<'_>],
    ) {
        (self.fp.cmd_bind_descriptor_buffers_ext)(
            command_buffer,
            binding_info.len() as u32,
            binding_info.as_ptr(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDescriptorBufferOffsetsEXT.html>
    #[inline]
    pub unsafe fn cmd_set_descriptor_buffer_offsets(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        layout: vk::PipelineLayout,
        first_set: u32,
        buffer_indices: &[u32],
        offsets: &[vk::DeviceSize],
    ) {
        assert_eq!(buffer_indices.len(), offsets.len());
        (self.fp.cmd_set_descriptor_buffer_offsets_ext)(
            command_buffer,
            pipeline_bind_point,
            layout,
            first_set,
            buffer_indices.len() as u32,
            buffer_indices.as_ptr(),
            offsets.as_ptr(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorBufferEmbeddedSamplersEXT.html>
    #[inline]
    pub unsafe fn cmd_bind_descriptor_buffer_embedded_samplers(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        layout: vk::PipelineLayout,
        set: u32,
    ) {
        (self.fp.cmd_bind_descriptor_buffer_embedded_samplers_ext)(
            command_buffer,
            pipeline_bind_point,
            layout,
            set,
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferOpaqueCaptureDescriptorDataEXT.html>
    #[inline]
    pub unsafe fn get_buffer_opaque_capture_descriptor_data(
        &self,
        info: &vk::BufferCaptureDescriptorDataInfoEXT<'_>,
        data: &mut [u8],
    ) -> VkResult<()> {
        (self.fp.get_buffer_opaque_capture_descriptor_data_ext)(
            self.handle,
            info,
            data.as_mut_ptr().cast(),
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageOpaqueCaptureDescriptorDataEXT.html>
    #[inline]
    pub unsafe fn get_image_opaque_capture_descriptor_data(
        &self,
        info: &vk::ImageCaptureDescriptorDataInfoEXT<'_>,
        data: &mut [u8],
    ) -> VkResult<()> {
        (self.fp.get_image_opaque_capture_descriptor_data_ext)(
            self.handle,
            info,
            data.as_mut_ptr().cast(),
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageViewOpaqueCaptureDescriptorDataEXT.html>
    #[inline]
    pub unsafe fn get_image_view_opaque_capture_descriptor_data(
        &self,
        info: &vk::ImageViewCaptureDescriptorDataInfoEXT<'_>,
        data: &mut [u8],
    ) -> VkResult<()> {
        (self.fp.get_image_view_opaque_capture_descriptor_data_ext)(
            self.handle,
            info,
            data.as_mut_ptr().cast(),
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSamplerOpaqueCaptureDescriptorDataEXT.html>
    #[inline]
    pub unsafe fn get_sampler_opaque_capture_descriptor_data(
        &self,
        info: &vk::SamplerCaptureDescriptorDataInfoEXT<'_>,
        data: &mut [u8],
    ) -> VkResult<()> {
        (self.fp.get_sampler_opaque_capture_descriptor_data_ext)(
            self.handle,
            info,
            data.as_mut_ptr().cast(),
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT.html>
    #[inline]
    pub unsafe fn get_acceleration_structure_opaque_capture_descriptor_data(
        &self,
        info: &vk::AccelerationStructureCaptureDescriptorDataInfoEXT<'_>,
        data: &mut [u8],
    ) -> VkResult<()> {
        (self
            .fp
            .get_acceleration_structure_opaque_capture_descriptor_data_ext)(
            self.handle,
            info,
            data.as_mut_ptr().cast(),
        )
        .result()
    }
}
