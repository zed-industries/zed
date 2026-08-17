//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_cuda_kernel_launch.html>

use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use alloc::vec::Vec;
use core::mem;

impl crate::nv::cuda_kernel_launch::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateCudaModuleNV.html>
    #[inline]
    pub unsafe fn create_cuda_module(
        &self,
        create_info: &vk::CudaModuleCreateInfoNV<'_>,
        allocator: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::CudaModuleNV> {
        let mut module = mem::MaybeUninit::uninit();
        (self.fp.create_cuda_module_nv)(
            self.handle,
            create_info,
            allocator.as_raw_ptr(),
            module.as_mut_ptr(),
        )
        .assume_init_on_success(module)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetCudaModuleCacheNV.html>
    #[inline]
    pub unsafe fn get_cuda_module_cache(&self, module: vk::CudaModuleNV) -> VkResult<Vec<u8>> {
        read_into_uninitialized_vector(|cache_size, cache_data: *mut u8| {
            (self.fp.get_cuda_module_cache_nv)(self.handle, module, cache_size, cache_data.cast())
        })
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateCudaFunctionNV.html>
    #[inline]
    pub unsafe fn create_cuda_function(
        &self,
        create_info: &vk::CudaFunctionCreateInfoNV<'_>,
        allocator: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::CudaFunctionNV> {
        let mut function = mem::MaybeUninit::uninit();
        (self.fp.create_cuda_function_nv)(
            self.handle,
            create_info,
            allocator.as_raw_ptr(),
            function.as_mut_ptr(),
        )
        .assume_init_on_success(function)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyCudaModuleNV.html>
    #[inline]
    pub unsafe fn destroy_cuda_module(
        &self,
        module: vk::CudaModuleNV,
        allocator: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.fp.destroy_cuda_module_nv)(self.handle, module, allocator.as_raw_ptr())
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyCudaFunctionNV.html>
    #[inline]
    pub unsafe fn destroy_cuda_function(
        &self,
        function: vk::CudaFunctionNV,
        allocator: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.fp.destroy_cuda_function_nv)(self.handle, function, allocator.as_raw_ptr())
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCudaLaunchKernelNV.html>
    #[inline]
    pub unsafe fn cmd_cuda_launch_kernel(
        &self,
        command_buffer: vk::CommandBuffer,
        launch_info: &vk::CudaLaunchInfoNV<'_>,
    ) {
        (self.fp.cmd_cuda_launch_kernel_nv)(command_buffer, launch_info)
    }
}
