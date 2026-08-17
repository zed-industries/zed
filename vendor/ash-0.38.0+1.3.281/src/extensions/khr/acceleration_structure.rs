//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_acceleration_structure.html>

use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use alloc::vec::Vec;
use core::mem;

impl crate::khr::acceleration_structure::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureKHR.html>
    #[inline]
    pub unsafe fn create_acceleration_structure(
        &self,
        create_info: &vk::AccelerationStructureCreateInfoKHR<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::AccelerationStructureKHR> {
        let mut accel_struct = mem::MaybeUninit::uninit();
        (self.fp.create_acceleration_structure_khr)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            accel_struct.as_mut_ptr(),
        )
        .assume_init_on_success(accel_struct)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureKHR.html>
    #[inline]
    pub unsafe fn destroy_acceleration_structure(
        &self,
        accel_struct: vk::AccelerationStructureKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.fp.destroy_acceleration_structure_khr)(
            self.handle,
            accel_struct,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructuresKHR.html>
    #[inline]
    pub unsafe fn cmd_build_acceleration_structures(
        &self,
        command_buffer: vk::CommandBuffer,
        infos: &[vk::AccelerationStructureBuildGeometryInfoKHR<'_>],
        build_range_infos: &[&[vk::AccelerationStructureBuildRangeInfoKHR]],
    ) {
        assert_eq!(infos.len(), build_range_infos.len());

        let build_range_infos = build_range_infos
            .iter()
            .zip(infos.iter())
            .map(|(range_info, info)| {
                assert_eq!(range_info.len(), info.geometry_count as usize);
                range_info.as_ptr()
            })
            .collect::<Vec<_>>();

        (self.fp.cmd_build_acceleration_structures_khr)(
            command_buffer,
            infos.len() as _,
            infos.as_ptr(),
            build_range_infos.as_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructuresIndirectKHR.html>
    #[inline]
    pub unsafe fn cmd_build_acceleration_structures_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        infos: &[vk::AccelerationStructureBuildGeometryInfoKHR<'_>],
        indirect_device_addresses: &[vk::DeviceAddress],
        indirect_strides: &[u32],
        max_primitive_counts: &[&[u32]],
    ) {
        assert_eq!(infos.len(), indirect_device_addresses.len());
        assert_eq!(infos.len(), indirect_strides.len());
        assert_eq!(infos.len(), max_primitive_counts.len());

        let max_primitive_counts = max_primitive_counts
            .iter()
            .zip(infos.iter())
            .map(|(cnt, info)| {
                assert_eq!(cnt.len(), info.geometry_count as usize);
                cnt.as_ptr()
            })
            .collect::<Vec<_>>();

        (self.fp.cmd_build_acceleration_structures_indirect_khr)(
            command_buffer,
            infos.len() as _,
            infos.as_ptr(),
            indirect_device_addresses.as_ptr(),
            indirect_strides.as_ptr(),
            max_primitive_counts.as_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBuildAccelerationStructuresKHR.html>
    #[inline]
    pub unsafe fn build_acceleration_structures(
        &self,
        deferred_operation: vk::DeferredOperationKHR,
        infos: &[vk::AccelerationStructureBuildGeometryInfoKHR<'_>],
        build_range_infos: &[&[vk::AccelerationStructureBuildRangeInfoKHR]],
    ) -> VkResult<()> {
        assert_eq!(infos.len(), build_range_infos.len());

        let build_range_infos = build_range_infos
            .iter()
            .zip(infos.iter())
            .map(|(range_info, info)| {
                assert_eq!(range_info.len(), info.geometry_count as usize);
                range_info.as_ptr()
            })
            .collect::<Vec<_>>();

        (self.fp.build_acceleration_structures_khr)(
            self.handle,
            deferred_operation,
            infos.len() as _,
            infos.as_ptr(),
            build_range_infos.as_ptr(),
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCopyAccelerationStructureKHR.html>
    #[inline]
    pub unsafe fn copy_acceleration_structure(
        &self,
        deferred_operation: vk::DeferredOperationKHR,
        info: &vk::CopyAccelerationStructureInfoKHR<'_>,
    ) -> VkResult<()> {
        (self.fp.copy_acceleration_structure_khr)(self.handle, deferred_operation, info).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCopyAccelerationStructureToMemoryKHR.html>
    #[inline]
    pub unsafe fn copy_acceleration_structure_to_memory(
        &self,
        deferred_operation: vk::DeferredOperationKHR,
        info: &vk::CopyAccelerationStructureToMemoryInfoKHR<'_>,
    ) -> VkResult<()> {
        (self.fp.copy_acceleration_structure_to_memory_khr)(self.handle, deferred_operation, info)
            .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCopyMemoryToAccelerationStructureKHR.html>
    #[inline]
    pub unsafe fn copy_memory_to_acceleration_structure(
        &self,
        deferred_operation: vk::DeferredOperationKHR,
        info: &vk::CopyMemoryToAccelerationStructureInfoKHR<'_>,
    ) -> VkResult<()> {
        (self.fp.copy_memory_to_acceleration_structure_khr)(self.handle, deferred_operation, info)
            .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWriteAccelerationStructuresPropertiesKHR.html>
    #[inline]
    pub unsafe fn write_acceleration_structures_properties(
        &self,
        acceleration_structures: &[vk::AccelerationStructureKHR],
        query_type: vk::QueryType,
        data: &mut [u8],
        stride: usize,
    ) -> VkResult<()> {
        (self.fp.write_acceleration_structures_properties_khr)(
            self.handle,
            acceleration_structures.len() as _,
            acceleration_structures.as_ptr(),
            query_type,
            data.len(),
            data.as_mut_ptr().cast(),
            stride,
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureKHR.html>
    #[inline]
    pub unsafe fn cmd_copy_acceleration_structure(
        &self,
        command_buffer: vk::CommandBuffer,
        info: &vk::CopyAccelerationStructureInfoKHR<'_>,
    ) {
        (self.fp.cmd_copy_acceleration_structure_khr)(command_buffer, info);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureToMemoryKHR.html>
    #[inline]
    pub unsafe fn cmd_copy_acceleration_structure_to_memory(
        &self,
        command_buffer: vk::CommandBuffer,
        info: &vk::CopyAccelerationStructureToMemoryInfoKHR<'_>,
    ) {
        (self.fp.cmd_copy_acceleration_structure_to_memory_khr)(command_buffer, info);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryToAccelerationStructureKHR.html>
    #[inline]
    pub unsafe fn cmd_copy_memory_to_acceleration_structure(
        &self,
        command_buffer: vk::CommandBuffer,
        info: &vk::CopyMemoryToAccelerationStructureInfoKHR<'_>,
    ) {
        (self.fp.cmd_copy_memory_to_acceleration_structure_khr)(command_buffer, info);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureHandleKHR.html>
    #[inline]
    pub unsafe fn get_acceleration_structure_device_address(
        &self,
        info: &vk::AccelerationStructureDeviceAddressInfoKHR<'_>,
    ) -> vk::DeviceAddress {
        (self.fp.get_acceleration_structure_device_address_khr)(self.handle, info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesKHR.html>
    #[inline]
    pub unsafe fn cmd_write_acceleration_structures_properties(
        &self,
        command_buffer: vk::CommandBuffer,
        structures: &[vk::AccelerationStructureKHR],
        query_type: vk::QueryType,
        query_pool: vk::QueryPool,
        first_query: u32,
    ) {
        (self.fp.cmd_write_acceleration_structures_properties_khr)(
            command_buffer,
            structures.len() as _,
            structures.as_ptr(),
            query_type,
            query_pool,
            first_query,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceAccelerationStructureCompatibilityKHR.html>
    #[inline]
    pub unsafe fn get_device_acceleration_structure_compatibility(
        &self,
        version: &vk::AccelerationStructureVersionInfoKHR<'_>,
    ) -> vk::AccelerationStructureCompatibilityKHR {
        let mut compatibility = mem::MaybeUninit::uninit();
        (self.fp.get_device_acceleration_structure_compatibility_khr)(
            self.handle,
            version,
            compatibility.as_mut_ptr(),
        );
        compatibility.assume_init()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureBuildSizesKHR.html>
    #[inline]
    pub unsafe fn get_acceleration_structure_build_sizes(
        &self,
        build_type: vk::AccelerationStructureBuildTypeKHR,
        build_info: &vk::AccelerationStructureBuildGeometryInfoKHR<'_>,
        max_primitive_counts: &[u32],
        size_info: &mut vk::AccelerationStructureBuildSizesInfoKHR<'_>,
    ) {
        assert_eq!(max_primitive_counts.len(), build_info.geometry_count as _);

        (self.fp.get_acceleration_structure_build_sizes_khr)(
            self.handle,
            build_type,
            build_info,
            max_primitive_counts.as_ptr(),
            size_info,
        )
    }
}
