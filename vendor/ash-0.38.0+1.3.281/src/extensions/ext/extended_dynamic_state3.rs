//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_extended_dynamic_state3.html>

use crate::vk;

impl crate::ext::extended_dynamic_state3::Device {
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
