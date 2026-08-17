//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_copy_memory_indirect.html>

use crate::vk;

impl crate::nv::copy_memory_indirect::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryIndirectNV.html>
    ///
    /// `copy_buffer_address` is a buffer device address which is a pointer to an array of
    /// `copy_count` number of [`vk::CopyMemoryIndirectCommandNV`] structures containing the copy
    /// parameters, each `stride` bytes apart.
    #[inline]
    pub unsafe fn cmd_copy_memory_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        copy_buffer_address: vk::DeviceAddress,
        copy_count: u32,
        stride: u32,
    ) {
        (self.fp.cmd_copy_memory_indirect_nv)(
            command_buffer,
            copy_buffer_address,
            copy_count,
            stride,
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryToImageIndirectNV.html>
    ///
    /// `copy_buffer_address` is a buffer device address which is a pointer to an array of
    /// `image_subresources.len()` number of [`vk::CopyMemoryToImageIndirectCommandNV`] structures
    /// containing the copy parameters, each `stride` bytes apart.
    #[inline]
    pub unsafe fn cmd_copy_memory_to_image_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        copy_buffer_address: vk::DeviceAddress,
        stride: u32,
        dst_image: vk::Image,
        dst_image_layout: vk::ImageLayout,
        image_subresources: &[vk::ImageSubresourceLayers],
    ) {
        (self.fp.cmd_copy_memory_to_image_indirect_nv)(
            command_buffer,
            copy_buffer_address,
            image_subresources.len() as u32,
            stride,
            dst_image,
            dst_image_layout,
            image_subresources.as_ptr(),
        )
    }
}
