//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_info.html>

use crate::prelude::*;
use crate::vk;
use alloc::vec::Vec;
use core::mem;

impl crate::amd::shader_info::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetShaderInfoAMD.html> with [`vk::ShaderInfoTypeAMD::STATISTICS`]
    #[inline]
    pub unsafe fn get_shader_info_statistics(
        &self,
        pipeline: vk::Pipeline,
        shader_stage: vk::ShaderStageFlags,
    ) -> VkResult<vk::ShaderStatisticsInfoAMD> {
        let mut info = mem::MaybeUninit::<vk::ShaderStatisticsInfoAMD>::uninit();
        let mut size = mem::size_of_val(&info);
        (self.fp.get_shader_info_amd)(
            self.handle,
            pipeline,
            shader_stage,
            vk::ShaderInfoTypeAMD::STATISTICS,
            &mut size,
            info.as_mut_ptr().cast(),
        )
        .result()?;
        assert_eq!(size, mem::size_of_val(&info));
        Ok(info.assume_init())
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetShaderInfoAMD.html> with [`vk::ShaderInfoTypeAMD::BINARY`]
    #[inline]
    pub unsafe fn get_shader_info_binary(
        &self,
        pipeline: vk::Pipeline,
        shader_stage: vk::ShaderStageFlags,
    ) -> VkResult<Vec<u8>> {
        read_into_uninitialized_vector(|count, data: *mut u8| {
            (self.fp.get_shader_info_amd)(
                self.handle,
                pipeline,
                shader_stage,
                vk::ShaderInfoTypeAMD::BINARY,
                count,
                data.cast(),
            )
        })
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetShaderInfoAMD.html> with [`vk::ShaderInfoTypeAMD::DISASSEMBLY`]
    #[inline]
    pub unsafe fn get_shader_info_disassembly(
        &self,
        pipeline: vk::Pipeline,
        shader_stage: vk::ShaderStageFlags,
    ) -> VkResult<Vec<u8>> {
        read_into_uninitialized_vector(|count, data: *mut u8| {
            (self.fp.get_shader_info_amd)(
                self.handle,
                pipeline,
                shader_stage,
                vk::ShaderInfoTypeAMD::DISASSEMBLY,
                count,
                data.cast(),
            )
        })
    }
}
