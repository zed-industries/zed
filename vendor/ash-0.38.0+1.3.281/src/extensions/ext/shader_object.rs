//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_object.html>

use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use alloc::vec::Vec;
use core::ptr;

impl crate::ext::shader_object::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateShadersEXT.html>
    ///
    /// When this function returns, whether or not it succeeds, it is guaranteed that every returned
    /// element is either [`vk::ShaderEXT::null()`] or a valid [`vk::ShaderEXT`] handle.
    ///
    /// This means that whenever shader creation fails, the application can determine which shader
    /// the returned error pertains to by locating the first [`vk::Handle::is_null()`] element
    /// in the returned [`Vec`]. It also means that an application can reliably clean up from a
    /// failed call by iterating over the returned [`Vec`] and destroying every element that is not
    /// [`vk::Handle::is_null()`].
    #[inline]
    pub unsafe fn create_shaders(
        &self,
        create_infos: &[vk::ShaderCreateInfoEXT<'_>],
        allocator: Option<&vk::AllocationCallbacks<'_>>,
    ) -> Result<Vec<vk::ShaderEXT>, (Vec<vk::ShaderEXT>, vk::Result)> {
        let mut shaders = Vec::with_capacity(create_infos.len());
        let err_code = (self.fp.create_shaders_ext)(
            self.handle,
            create_infos.len() as u32,
            create_infos.as_ptr(),
            allocator.as_raw_ptr(),
            shaders.as_mut_ptr(),
        );
        shaders.set_len(create_infos.len());
        match err_code {
            vk::Result::SUCCESS => Ok(shaders),
            _ => Err((shaders, err_code)),
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyShaderEXT.html>
    #[inline]
    pub unsafe fn destroy_shader(
        &self,
        shader: vk::ShaderEXT,
        allocator: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.fp.destroy_shader_ext)(self.handle, shader, allocator.as_raw_ptr())
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetShaderBinaryDataEXT.html>
    #[inline]
    pub unsafe fn get_shader_binary_data(&self, shader: vk::ShaderEXT) -> VkResult<Vec<u8>> {
        read_into_uninitialized_vector(|count, data: *mut u8| {
            (self.fp.get_shader_binary_data_ext)(self.handle, shader, count, data.cast())
        })
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindShadersEXT.html>
    #[inline]
    pub unsafe fn cmd_bind_shaders(
        &self,
        command_buffer: vk::CommandBuffer,
        stages: &[vk::ShaderStageFlags],
        shaders: &[vk::ShaderEXT],
    ) {
        assert_eq!(stages.len(), shaders.len());
        (self.fp.cmd_bind_shaders_ext)(
            command_buffer,
            stages.len() as u32,
            stages.as_ptr(),
            shaders.as_ptr(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetVertexInputEXT.html>
    #[inline]
    pub unsafe fn cmd_set_vertex_input(
        &self,
        command_buffer: vk::CommandBuffer,
        vertex_binding_descriptions: &[vk::VertexInputBindingDescription2EXT<'_>],
        vertex_attribute_descriptions: &[vk::VertexInputAttributeDescription2EXT<'_>],
    ) {
        (self.fp.cmd_set_vertex_input_ext)(
            command_buffer,
            vertex_binding_descriptions.len() as u32,
            vertex_binding_descriptions.as_ptr(),
            vertex_attribute_descriptions.len() as u32,
            vertex_attribute_descriptions.as_ptr(),
        )
    }

    // --- extended_dynamic_state functions ---

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

    // --- extended_dynamic_state2 functions ---

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPatchControlPointsEXT.html>
    #[inline]
    pub unsafe fn cmd_set_patch_control_points(
        &self,
        command_buffer: vk::CommandBuffer,
        patch_control_points: u32,
    ) {
        (self.fp.cmd_set_patch_control_points_ext)(command_buffer, patch_control_points)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizerDiscardEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_rasterizer_discard_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        rasterizer_discard_enable: bool,
    ) {
        (self.fp.cmd_set_rasterizer_discard_enable_ext)(
            command_buffer,
            rasterizer_discard_enable.into(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBiasEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_depth_bias_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_bias_enable: bool,
    ) {
        (self.fp.cmd_set_depth_bias_enable_ext)(command_buffer, depth_bias_enable.into())
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEXT.html>
    #[inline]
    pub unsafe fn cmd_set_logic_op(
        &self,
        command_buffer: vk::CommandBuffer,
        logic_op: vk::LogicOp,
    ) {
        (self.fp.cmd_set_logic_op_ext)(command_buffer, logic_op)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveRestartEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_primitive_restart_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        primitive_restart_enable: bool,
    ) {
        (self.fp.cmd_set_primitive_restart_enable_ext)(
            command_buffer,
            primitive_restart_enable.into(),
        )
    }

    // --- extended_dynamic_state3 functions ---

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetTessellationDomainOriginEXT.html>
    #[inline]
    pub unsafe fn cmd_set_tessellation_domain_origin(
        &self,
        command_buffer: vk::CommandBuffer,
        domain_origin: vk::TessellationDomainOrigin,
    ) {
        (self.fp.cmd_set_tessellation_domain_origin_ext)(command_buffer, domain_origin)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClampEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_depth_clamp_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_clamp_enable: bool,
    ) {
        (self.fp.cmd_set_depth_clamp_enable_ext)(command_buffer, depth_clamp_enable.into())
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPolygonModeEXT.html>
    #[inline]
    pub unsafe fn cmd_set_polygon_mode(
        &self,
        command_buffer: vk::CommandBuffer,
        polygon_mode: vk::PolygonMode,
    ) {
        (self.fp.cmd_set_polygon_mode_ext)(command_buffer, polygon_mode)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizationSamplesEXT.html>
    #[inline]
    pub unsafe fn cmd_set_rasterization_samples(
        &self,
        command_buffer: vk::CommandBuffer,
        rasterization_samples: vk::SampleCountFlags,
    ) {
        (self.fp.cmd_set_rasterization_samples_ext)(command_buffer, rasterization_samples)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleMaskEXT.html>
    #[inline]
    pub unsafe fn cmd_set_sample_mask(
        &self,
        command_buffer: vk::CommandBuffer,
        samples: vk::SampleCountFlags,
        sample_mask: &[vk::SampleMask],
    ) {
        assert!(
            samples.as_raw().is_power_of_two(),
            "Only one SampleCount bit must be set"
        );
        assert_eq!((samples.as_raw() as usize + 31) / 32, sample_mask.len());
        (self.fp.cmd_set_sample_mask_ext)(command_buffer, samples, sample_mask.as_ptr())
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetAlphaToCoverageEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_alpha_to_coverage_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        alpha_to_coverage_enable: bool,
    ) {
        (self.fp.cmd_set_alpha_to_coverage_enable_ext)(
            command_buffer,
            alpha_to_coverage_enable.into(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetAlphaToOneEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_alpha_to_one_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        alpha_to_one_enable: bool,
    ) {
        (self.fp.cmd_set_alpha_to_one_enable_ext)(command_buffer, alpha_to_one_enable.into())
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_logic_op_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        logic_op_enable: bool,
    ) {
        (self.fp.cmd_set_logic_op_enable_ext)(command_buffer, logic_op_enable.into())
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_color_blend_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        first_attachment: u32,
        color_blend_enables: &[vk::Bool32],
    ) {
        (self.fp.cmd_set_color_blend_enable_ext)(
            command_buffer,
            first_attachment,
            color_blend_enables.len() as u32,
            color_blend_enables.as_ptr(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEquationEXT.html>
    #[inline]
    pub unsafe fn cmd_set_color_blend_equation(
        &self,
        command_buffer: vk::CommandBuffer,
        first_attachment: u32,
        color_blend_equations: &[vk::ColorBlendEquationEXT],
    ) {
        (self.fp.cmd_set_color_blend_equation_ext)(
            command_buffer,
            first_attachment,
            color_blend_equations.len() as u32,
            color_blend_equations.as_ptr(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorWriteMaskEXT.html>
    #[inline]
    pub unsafe fn cmd_set_color_write_mask(
        &self,
        command_buffer: vk::CommandBuffer,
        first_attachment: u32,
        color_write_masks: &[vk::ColorComponentFlags],
    ) {
        (self.fp.cmd_set_color_write_mask_ext)(
            command_buffer,
            first_attachment,
            color_write_masks.len() as u32,
            color_write_masks.as_ptr(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizationStreamEXT.html>
    #[inline]
    pub unsafe fn cmd_set_rasterization_stream(
        &self,
        command_buffer: vk::CommandBuffer,
        rasterization_stream: u32,
    ) {
        (self.fp.cmd_set_rasterization_stream_ext)(command_buffer, rasterization_stream)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetConservativeRasterizationModeEXT.html>
    #[inline]
    pub unsafe fn cmd_set_conservative_rasterization_mode(
        &self,
        command_buffer: vk::CommandBuffer,
        conservative_rasterization_mode: vk::ConservativeRasterizationModeEXT,
    ) {
        (self.fp.cmd_set_conservative_rasterization_mode_ext)(
            command_buffer,
            conservative_rasterization_mode,
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetExtraPrimitiveOverestimationSizeEXT.html>
    #[inline]
    pub unsafe fn cmd_set_extra_primitive_overestimation_size(
        &self,
        command_buffer: vk::CommandBuffer,
        extra_primitive_overestimation_size: f32,
    ) {
        (self.fp.cmd_set_extra_primitive_overestimation_size_ext)(
            command_buffer,
            extra_primitive_overestimation_size,
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClipEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_depth_clip_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_clip_enable: bool,
    ) {
        (self.fp.cmd_set_depth_clip_enable_ext)(command_buffer, depth_clip_enable.into())
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_sample_locations_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        sample_locations_enable: bool,
    ) {
        (self.fp.cmd_set_sample_locations_enable_ext)(
            command_buffer,
            sample_locations_enable.into(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendAdvancedEXT.html>
    #[inline]
    pub unsafe fn cmd_set_color_blend_advanced(
        &self,
        command_buffer: vk::CommandBuffer,
        first_attachment: u32,
        color_blend_advanced: &[vk::ColorBlendAdvancedEXT],
    ) {
        (self.fp.cmd_set_color_blend_advanced_ext)(
            command_buffer,
            first_attachment,
            color_blend_advanced.len() as u32,
            color_blend_advanced.as_ptr(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetProvokingVertexModeEXT.html>
    #[inline]
    pub unsafe fn cmd_set_provoking_vertex_mode(
        &self,
        command_buffer: vk::CommandBuffer,
        provoking_vertex_mode: vk::ProvokingVertexModeEXT,
    ) {
        (self.fp.cmd_set_provoking_vertex_mode_ext)(command_buffer, provoking_vertex_mode)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineRasterizationModeEXT.html>
    #[inline]
    pub unsafe fn cmd_set_line_rasterization_mode(
        &self,
        command_buffer: vk::CommandBuffer,
        line_rasterization_mode: vk::LineRasterizationModeEXT,
    ) {
        (self.fp.cmd_set_line_rasterization_mode_ext)(command_buffer, line_rasterization_mode)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineStippleEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_line_stipple_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        stippled_line_enable: bool,
    ) {
        (self.fp.cmd_set_line_stipple_enable_ext)(command_buffer, stippled_line_enable.into())
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClipNegativeOneToOneEXT.html>
    #[inline]
    pub unsafe fn cmd_set_depth_clip_negative_one_to_one(
        &self,
        command_buffer: vk::CommandBuffer,
        negative_one_to_one: bool,
    ) {
        (self.fp.cmd_set_depth_clip_negative_one_to_one_ext)(
            command_buffer,
            negative_one_to_one.into(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWScalingEnableNV.html>
    #[inline]
    pub unsafe fn cmd_set_viewport_w_scaling_enable_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        viewport_w_scaling_enable: bool,
    ) {
        (self.fp.cmd_set_viewport_w_scaling_enable_nv)(
            command_buffer,
            viewport_w_scaling_enable.into(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportSwizzleNV.html>
    #[inline]
    pub unsafe fn cmd_set_viewport_swizzle_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        first_attachment: u32,
        viewport_swizzles: &[vk::ViewportSwizzleNV],
    ) {
        (self.fp.cmd_set_viewport_swizzle_nv)(
            command_buffer,
            first_attachment,
            viewport_swizzles.len() as u32,
            viewport_swizzles.as_ptr(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageToColorEnableNV.html>
    #[inline]
    pub unsafe fn cmd_set_coverage_to_color_enable_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        coverage_to_color_enable: bool,
    ) {
        (self.fp.cmd_set_coverage_to_color_enable_nv)(
            command_buffer,
            coverage_to_color_enable.into(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageToColorLocationNV.html>
    #[inline]
    pub unsafe fn cmd_set_coverage_to_color_location_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        coverage_to_color_location: u32,
    ) {
        (self.fp.cmd_set_coverage_to_color_location_nv)(command_buffer, coverage_to_color_location)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationModeNV.html>
    #[inline]
    pub unsafe fn cmd_set_coverage_modulation_mode_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        coverage_modulation_mode: vk::CoverageModulationModeNV,
    ) {
        (self.fp.cmd_set_coverage_modulation_mode_nv)(command_buffer, coverage_modulation_mode)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationTableEnableNV.html>
    #[inline]
    pub unsafe fn cmd_set_coverage_modulation_table_enable_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        coverage_modulation_table_enable: bool,
    ) {
        (self.fp.cmd_set_coverage_modulation_table_enable_nv)(
            command_buffer,
            coverage_modulation_table_enable.into(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationTableNV.html>
    #[inline]
    pub unsafe fn cmd_set_coverage_modulation_table_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        coverage_modulation_table: &[f32],
    ) {
        (self.fp.cmd_set_coverage_modulation_table_nv)(
            command_buffer,
            coverage_modulation_table.len() as u32,
            coverage_modulation_table.as_ptr(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetShadingRateImageEnableNV.html>
    #[inline]
    pub unsafe fn cmd_set_shading_rate_image_enable_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        shading_rate_image_enable: bool,
    ) {
        (self.fp.cmd_set_shading_rate_image_enable_nv)(
            command_buffer,
            shading_rate_image_enable.into(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRepresentativeFragmentTestEnableNV.html>
    #[inline]
    pub unsafe fn cmd_set_representative_fragment_test_enable_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        representative_fragment_test_enable: bool,
    ) {
        (self.fp.cmd_set_representative_fragment_test_enable_nv)(
            command_buffer,
            representative_fragment_test_enable.into(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageReductionModeNV.html>
    #[inline]
    pub unsafe fn cmd_set_coverage_reduction_mode_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        coverage_reduction_mode: vk::CoverageReductionModeNV,
    ) {
        (self.fp.cmd_set_coverage_reduction_mode_nv)(command_buffer, coverage_reduction_mode)
    }
}
