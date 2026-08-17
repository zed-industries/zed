//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_extended_dynamic_state.html>

use crate::vk;
use core::ptr;

impl crate::ext::extended_dynamic_state::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCullModeEXT.html>
    #[inline]
    pub unsafe fn cmd_set_cull_mode(
        &self,
        command_buffer: vk::CommandBuffer,
        cull_mode: vk::CullModeFlags,
    ) {
        (self.fp.cmd_set_cull_mode_ext)(command_buffer, cull_mode)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetFrontFaceEXT.html>
    #[inline]
    pub unsafe fn cmd_set_front_face(
        &self,
        command_buffer: vk::CommandBuffer,
        front_face: vk::FrontFace,
    ) {
        (self.fp.cmd_set_front_face_ext)(command_buffer, front_face)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveTopologyEXT.html>
    #[inline]
    pub unsafe fn cmd_set_primitive_topology(
        &self,
        command_buffer: vk::CommandBuffer,
        primitive_topology: vk::PrimitiveTopology,
    ) {
        (self.fp.cmd_set_primitive_topology_ext)(command_buffer, primitive_topology)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWithCountEXT.html>
    #[inline]
    pub unsafe fn cmd_set_viewport_with_count(
        &self,
        command_buffer: vk::CommandBuffer,
        viewports: &[vk::Viewport],
    ) {
        (self.fp.cmd_set_viewport_with_count_ext)(
            command_buffer,
            viewports.len() as u32,
            viewports.as_ptr(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissorWithCountEXT.html>
    #[inline]
    pub unsafe fn cmd_set_scissor_with_count(
        &self,
        command_buffer: vk::CommandBuffer,
        scissors: &[vk::Rect2D],
    ) {
        (self.fp.cmd_set_scissor_with_count_ext)(
            command_buffer,
            scissors.len() as u32,
            scissors.as_ptr(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers2EXT.html>
    #[inline]
    pub unsafe fn cmd_bind_vertex_buffers2(
        &self,
        command_buffer: vk::CommandBuffer,
        first_binding: u32,
        buffers: &[vk::Buffer],
        offsets: &[vk::DeviceSize],
        sizes: Option<&[vk::DeviceSize]>,
        strides: Option<&[vk::DeviceSize]>,
    ) {
        assert_eq!(offsets.len(), buffers.len());
        let p_sizes = if let Some(sizes) = sizes {
            assert_eq!(sizes.len(), buffers.len());
            sizes.as_ptr()
        } else {
            ptr::null()
        };
        let p_strides = if let Some(strides) = strides {
            assert_eq!(strides.len(), buffers.len());
            strides.as_ptr()
        } else {
            ptr::null()
        };
        (self.fp.cmd_bind_vertex_buffers2_ext)(
            command_buffer,
            first_binding,
            buffers.len() as u32,
            buffers.as_ptr(),
            offsets.as_ptr(),
            p_sizes,
            p_strides,
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthTestEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_depth_test_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_test_enable: bool,
    ) {
        (self.fp.cmd_set_depth_test_enable_ext)(command_buffer, depth_test_enable.into())
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthWriteEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_depth_write_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_write_enable: bool,
    ) {
        (self.fp.cmd_set_depth_write_enable_ext)(command_buffer, depth_write_enable.into())
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthCompareOpEXT.html>
    #[inline]
    pub unsafe fn cmd_set_depth_compare_op(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_compare_op: vk::CompareOp,
    ) {
        (self.fp.cmd_set_depth_compare_op_ext)(command_buffer, depth_compare_op)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBoundsTestEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_depth_bounds_test_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_bounds_test_enable: bool,
    ) {
        (self.fp.cmd_set_depth_bounds_test_enable_ext)(
            command_buffer,
            depth_bounds_test_enable.into(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilTestEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_stencil_test_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        stencil_test_enable: bool,
    ) {
        (self.fp.cmd_set_stencil_test_enable_ext)(command_buffer, stencil_test_enable.into())
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilOpEXT.html>
    #[inline]
    pub unsafe fn cmd_set_stencil_op(
        &self,
        command_buffer: vk::CommandBuffer,
        face_mask: vk::StencilFaceFlags,
        fail_op: vk::StencilOp,
        pass_op: vk::StencilOp,
        depth_fail_op: vk::StencilOp,
        compare_op: vk::CompareOp,
    ) {
        (self.fp.cmd_set_stencil_op_ext)(
            command_buffer,
            face_mask,
            fail_op,
            pass_op,
            depth_fail_op,
            compare_op,
        )
    }
}
