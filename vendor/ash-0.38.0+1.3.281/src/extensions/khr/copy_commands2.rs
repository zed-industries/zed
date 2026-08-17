//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_copy_commands2.html>

use crate::vk;

impl crate::khr::copy_commands2::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer2KHR.html>
    #[inline]
    pub unsafe fn cmd_copy_buffer2(
        &self,
        command_buffer: vk::CommandBuffer,
        copy_buffer_info: &vk::CopyBufferInfo2KHR<'_>,
    ) {
        (self.fp.cmd_copy_buffer2_khr)(command_buffer, copy_buffer_info)
    }
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage2KHR.html>
    #[inline]
    pub unsafe fn cmd_copy_image2(
        &self,
        command_buffer: vk::CommandBuffer,
        copy_image_info: &vk::CopyImageInfo2KHR<'_>,
    ) {
        (self.fp.cmd_copy_image2_khr)(command_buffer, copy_image_info)
    }
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage2KHR.html>
    #[inline]
    pub unsafe fn cmd_copy_buffer_to_image2(
        &self,
        command_buffer: vk::CommandBuffer,
        copy_buffer_to_image_info: &vk::CopyBufferToImageInfo2KHR<'_>,
    ) {
        (self.fp.cmd_copy_buffer_to_image2_khr)(command_buffer, copy_buffer_to_image_info)
    }
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer2KHR.html>
    #[inline]
    pub unsafe fn cmd_copy_image_to_buffer2(
        &self,
        command_buffer: vk::CommandBuffer,
        copy_image_to_buffer_info: &vk::CopyImageToBufferInfo2KHR<'_>,
    ) {
        (self.fp.cmd_copy_image_to_buffer2_khr)(command_buffer, copy_image_to_buffer_info)
    }
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage2KHR.html>
    #[inline]
    pub unsafe fn cmd_blit_image2(
        &self,
        command_buffer: vk::CommandBuffer,
        blit_image_info: &vk::BlitImageInfo2KHR<'_>,
    ) {
        (self.fp.cmd_blit_image2_khr)(command_buffer, blit_image_info)
    }
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage2KHR.html>
    #[inline]
    pub unsafe fn cmd_resolve_image2(
        &self,
        command_buffer: vk::CommandBuffer,
        resolve_image_info: &vk::ResolveImageInfo2KHR<'_>,
    ) {
        (self.fp.cmd_resolve_image2_khr)(command_buffer, resolve_image_info)
    }
}
