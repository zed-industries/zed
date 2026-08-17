#![allow(clippy::trivially_copy_pass_by_ref)]
use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use alloc::vec::Vec;
use core::ffi;
use core::mem;
use core::ptr;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDevice.html>
#[derive(Clone)]
pub struct Device {
    pub(crate) handle: vk::Device,

    pub(crate) device_fn_1_0: crate::DeviceFnV1_0,
    pub(crate) device_fn_1_1: crate::DeviceFnV1_1,
    pub(crate) device_fn_1_2: crate::DeviceFnV1_2,
    pub(crate) device_fn_1_3: crate::DeviceFnV1_3,
}

impl Device {
    pub unsafe fn load(instance_fn: &crate::InstanceFnV1_0, device: vk::Device) -> Self {
        Self::load_with(
            |name| mem::transmute((instance_fn.get_device_proc_addr)(device, name.as_ptr())),
            device,
        )
    }

    pub unsafe fn load_with(
        mut load_fn: impl FnMut(&ffi::CStr) -> *const ffi::c_void,
        device: vk::Device,
    ) -> Self {
        Self::from_parts_1_3(
            device,
            crate::DeviceFnV1_0::load(&mut load_fn),
            crate::DeviceFnV1_1::load(&mut load_fn),
            crate::DeviceFnV1_2::load(&mut load_fn),
            crate::DeviceFnV1_3::load(&mut load_fn),
        )
    }

    #[inline]
    pub fn from_parts_1_3(
        handle: vk::Device,
        device_fn_1_0: crate::DeviceFnV1_0,
        device_fn_1_1: crate::DeviceFnV1_1,
        device_fn_1_2: crate::DeviceFnV1_2,
        device_fn_1_3: crate::DeviceFnV1_3,
    ) -> Self {
        Self {
            handle,

            device_fn_1_0,
            device_fn_1_1,
            device_fn_1_2,
            device_fn_1_3,
        }
    }

    #[inline]
    pub fn handle(&self) -> vk::Device {
        self.handle
    }
}

/// Vulkan core 1.3
impl Device {
    #[inline]
    pub fn fp_v1_3(&self) -> &crate::DeviceFnV1_3 {
        &self.device_fn_1_3
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreatePrivateDataSlot.html>
    #[inline]
    pub unsafe fn create_private_data_slot(
        &self,
        create_info: &vk::PrivateDataSlotCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::PrivateDataSlot> {
        let mut private_data_slot = mem::MaybeUninit::uninit();
        (self.device_fn_1_3.create_private_data_slot)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            private_data_slot.as_mut_ptr(),
        )
        .assume_init_on_success(private_data_slot)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyPrivateDataSlot.html>
    #[inline]
    pub unsafe fn destroy_private_data_slot(
        &self,
        private_data_slot: vk::PrivateDataSlot,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.device_fn_1_3.destroy_private_data_slot)(
            self.handle,
            private_data_slot,
            allocation_callbacks.as_raw_ptr(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetPrivateData.html>
    #[inline]
    pub unsafe fn set_private_data<T: vk::Handle>(
        &self,
        object: T,
        private_data_slot: vk::PrivateDataSlot,
        data: u64,
    ) -> VkResult<()> {
        (self.device_fn_1_3.set_private_data)(
            self.handle,
            T::TYPE,
            object.as_raw(),
            private_data_slot,
            data,
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPrivateData.html>
    #[inline]
    pub unsafe fn get_private_data<T: vk::Handle>(
        &self,
        object: T,
        private_data_slot: vk::PrivateDataSlot,
    ) -> u64 {
        let mut data = mem::MaybeUninit::uninit();
        (self.device_fn_1_3.get_private_data)(
            self.handle,
            T::TYPE,
            object.as_raw(),
            private_data_slot,
            data.as_mut_ptr(),
        );
        data.assume_init()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier2.html>
    #[inline]
    pub unsafe fn cmd_pipeline_barrier2(
        &self,
        command_buffer: vk::CommandBuffer,
        dependency_info: &vk::DependencyInfo<'_>,
    ) {
        (self.device_fn_1_3.cmd_pipeline_barrier2)(command_buffer, dependency_info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent2.html>
    #[inline]
    pub unsafe fn cmd_reset_event2(
        &self,
        command_buffer: vk::CommandBuffer,
        event: vk::Event,
        stage_mask: vk::PipelineStageFlags2,
    ) {
        (self.device_fn_1_3.cmd_reset_event2)(command_buffer, event, stage_mask)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent2.html>
    #[inline]
    pub unsafe fn cmd_set_event2(
        &self,
        command_buffer: vk::CommandBuffer,
        event: vk::Event,
        dependency_info: &vk::DependencyInfo<'_>,
    ) {
        (self.device_fn_1_3.cmd_set_event2)(command_buffer, event, dependency_info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents2.html>
    #[inline]
    pub unsafe fn cmd_wait_events2(
        &self,
        command_buffer: vk::CommandBuffer,
        events: &[vk::Event],
        dependency_infos: &[vk::DependencyInfo<'_>],
    ) {
        assert_eq!(events.len(), dependency_infos.len());
        (self.device_fn_1_3.cmd_wait_events2)(
            command_buffer,
            events.len() as u32,
            events.as_ptr(),
            dependency_infos.as_ptr(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp2.html>
    #[inline]
    pub unsafe fn cmd_write_timestamp2(
        &self,
        command_buffer: vk::CommandBuffer,
        stage: vk::PipelineStageFlags2,
        query_pool: vk::QueryPool,
        query: u32,
    ) {
        (self.device_fn_1_3.cmd_write_timestamp2)(command_buffer, stage, query_pool, query)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit2.html>
    #[inline]
    pub unsafe fn queue_submit2(
        &self,
        queue: vk::Queue,
        submits: &[vk::SubmitInfo2<'_>],
        fence: vk::Fence,
    ) -> VkResult<()> {
        (self.device_fn_1_3.queue_submit2)(queue, submits.len() as u32, submits.as_ptr(), fence)
            .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer2.html>
    #[inline]
    pub unsafe fn cmd_copy_buffer2(
        &self,
        command_buffer: vk::CommandBuffer,
        copy_buffer_info: &vk::CopyBufferInfo2<'_>,
    ) {
        (self.device_fn_1_3.cmd_copy_buffer2)(command_buffer, copy_buffer_info)
    }
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage2.html>
    #[inline]
    pub unsafe fn cmd_copy_image2(
        &self,
        command_buffer: vk::CommandBuffer,
        copy_image_info: &vk::CopyImageInfo2<'_>,
    ) {
        (self.device_fn_1_3.cmd_copy_image2)(command_buffer, copy_image_info)
    }
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage2.html>
    #[inline]
    pub unsafe fn cmd_copy_buffer_to_image2(
        &self,
        command_buffer: vk::CommandBuffer,
        copy_buffer_to_image_info: &vk::CopyBufferToImageInfo2<'_>,
    ) {
        (self.device_fn_1_3.cmd_copy_buffer_to_image2)(command_buffer, copy_buffer_to_image_info)
    }
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer2.html>
    #[inline]
    pub unsafe fn cmd_copy_image_to_buffer2(
        &self,
        command_buffer: vk::CommandBuffer,
        copy_image_to_buffer_info: &vk::CopyImageToBufferInfo2<'_>,
    ) {
        (self.device_fn_1_3.cmd_copy_image_to_buffer2)(command_buffer, copy_image_to_buffer_info)
    }
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage2.html>
    #[inline]
    pub unsafe fn cmd_blit_image2(
        &self,
        command_buffer: vk::CommandBuffer,
        blit_image_info: &vk::BlitImageInfo2<'_>,
    ) {
        (self.device_fn_1_3.cmd_blit_image2)(command_buffer, blit_image_info)
    }
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage2.html>
    #[inline]
    pub unsafe fn cmd_resolve_image2(
        &self,
        command_buffer: vk::CommandBuffer,
        resolve_image_info: &vk::ResolveImageInfo2<'_>,
    ) {
        (self.device_fn_1_3.cmd_resolve_image2)(command_buffer, resolve_image_info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRendering.html>
    #[inline]
    pub unsafe fn cmd_begin_rendering(
        &self,
        command_buffer: vk::CommandBuffer,
        rendering_info: &vk::RenderingInfo<'_>,
    ) {
        (self.device_fn_1_3.cmd_begin_rendering)(command_buffer, rendering_info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndRendering.html>
    #[inline]
    pub unsafe fn cmd_end_rendering(&self, command_buffer: vk::CommandBuffer) {
        (self.device_fn_1_3.cmd_end_rendering)(command_buffer)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCullMode.html>
    #[inline]
    pub unsafe fn cmd_set_cull_mode(
        &self,
        command_buffer: vk::CommandBuffer,
        cull_mode: vk::CullModeFlags,
    ) {
        (self.device_fn_1_3.cmd_set_cull_mode)(command_buffer, cull_mode)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetFrontFace.html>
    #[inline]
    pub unsafe fn cmd_set_front_face(
        &self,
        command_buffer: vk::CommandBuffer,
        front_face: vk::FrontFace,
    ) {
        (self.device_fn_1_3.cmd_set_front_face)(command_buffer, front_face)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveTopology.html>
    #[inline]
    pub unsafe fn cmd_set_primitive_topology(
        &self,
        command_buffer: vk::CommandBuffer,
        primitive_topology: vk::PrimitiveTopology,
    ) {
        (self.device_fn_1_3.cmd_set_primitive_topology)(command_buffer, primitive_topology)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWithCount.html>
    #[inline]
    pub unsafe fn cmd_set_viewport_with_count(
        &self,
        command_buffer: vk::CommandBuffer,
        viewports: &[vk::Viewport],
    ) {
        (self.device_fn_1_3.cmd_set_viewport_with_count)(
            command_buffer,
            viewports.len() as u32,
            viewports.as_ptr(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissorWithCount.html>
    #[inline]
    pub unsafe fn cmd_set_scissor_with_count(
        &self,
        command_buffer: vk::CommandBuffer,
        scissors: &[vk::Rect2D],
    ) {
        (self.device_fn_1_3.cmd_set_scissor_with_count)(
            command_buffer,
            scissors.len() as u32,
            scissors.as_ptr(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers2.html>
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
        (self.device_fn_1_3.cmd_bind_vertex_buffers2)(
            command_buffer,
            first_binding,
            buffers.len() as u32,
            buffers.as_ptr(),
            offsets.as_ptr(),
            p_sizes,
            p_strides,
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthTestEnable.html>
    #[inline]
    pub unsafe fn cmd_set_depth_test_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_test_enable: bool,
    ) {
        (self.device_fn_1_3.cmd_set_depth_test_enable)(command_buffer, depth_test_enable.into())
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthWriteEnable.html>
    #[inline]
    pub unsafe fn cmd_set_depth_write_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_write_enable: bool,
    ) {
        (self.device_fn_1_3.cmd_set_depth_write_enable)(command_buffer, depth_write_enable.into())
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthCompareOp.html>
    #[inline]
    pub unsafe fn cmd_set_depth_compare_op(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_compare_op: vk::CompareOp,
    ) {
        (self.device_fn_1_3.cmd_set_depth_compare_op)(command_buffer, depth_compare_op)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBoundsTestEnable.html>
    #[inline]
    pub unsafe fn cmd_set_depth_bounds_test_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_bounds_test_enable: bool,
    ) {
        (self.device_fn_1_3.cmd_set_depth_bounds_test_enable)(
            command_buffer,
            depth_bounds_test_enable.into(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilTestEnable.html>
    #[inline]
    pub unsafe fn cmd_set_stencil_test_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        stencil_test_enable: bool,
    ) {
        (self.device_fn_1_3.cmd_set_stencil_test_enable)(command_buffer, stencil_test_enable.into())
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilOp.html>
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
        (self.device_fn_1_3.cmd_set_stencil_op)(
            command_buffer,
            face_mask,
            fail_op,
            pass_op,
            depth_fail_op,
            compare_op,
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizerDiscardEnable.html>
    #[inline]
    pub unsafe fn cmd_set_rasterizer_discard_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        rasterizer_discard_enable: bool,
    ) {
        (self.device_fn_1_3.cmd_set_rasterizer_discard_enable)(
            command_buffer,
            rasterizer_discard_enable.into(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBiasEnable.html>
    #[inline]
    pub unsafe fn cmd_set_depth_bias_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_bias_enable: bool,
    ) {
        (self.device_fn_1_3.cmd_set_depth_bias_enable)(command_buffer, depth_bias_enable.into())
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveRestartEnable.html>
    #[inline]
    pub unsafe fn cmd_set_primitive_restart_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        primitive_restart_enable: bool,
    ) {
        (self.device_fn_1_3.cmd_set_primitive_restart_enable)(
            command_buffer,
            primitive_restart_enable.into(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceBufferMemoryRequirements.html>
    #[inline]
    pub unsafe fn get_device_buffer_memory_requirements(
        &self,
        memory_requirements: &vk::DeviceBufferMemoryRequirements<'_>,
        out: &mut vk::MemoryRequirements2<'_>,
    ) {
        (self.device_fn_1_3.get_device_buffer_memory_requirements)(
            self.handle,
            memory_requirements,
            out,
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageMemoryRequirements.html>
    #[inline]
    pub unsafe fn get_device_image_memory_requirements(
        &self,
        memory_requirements: &vk::DeviceImageMemoryRequirements<'_>,
        out: &mut vk::MemoryRequirements2<'_>,
    ) {
        (self.device_fn_1_3.get_device_image_memory_requirements)(
            self.handle,
            memory_requirements,
            out,
        )
    }

    /// Retrieve the number of elements to pass to [`get_device_image_sparse_memory_requirements()`][Self::get_device_image_sparse_memory_requirements()]
    #[inline]
    pub unsafe fn get_device_image_sparse_memory_requirements_len(
        &self,
        memory_requirements: &vk::DeviceImageMemoryRequirements<'_>,
    ) -> usize {
        let mut count = mem::MaybeUninit::uninit();
        (self
            .device_fn_1_3
            .get_device_image_sparse_memory_requirements)(
            self.handle,
            memory_requirements,
            count.as_mut_ptr(),
            ptr::null_mut(),
        );
        count.assume_init() as usize
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageSparseMemoryRequirements.html>
    ///
    /// Call [`get_device_image_sparse_memory_requirements_len()`][Self::get_device_image_sparse_memory_requirements_len()] to query the number of elements to pass to `out`.
    /// Be sure to [`Default::default()`]-initialize these elements and optionally set their `p_next` pointer.
    #[inline]
    pub unsafe fn get_device_image_sparse_memory_requirements(
        &self,
        memory_requirements: &vk::DeviceImageMemoryRequirements<'_>,
        out: &mut [vk::SparseImageMemoryRequirements2<'_>],
    ) {
        let mut count = out.len() as u32;
        (self
            .device_fn_1_3
            .get_device_image_sparse_memory_requirements)(
            self.handle,
            memory_requirements,
            &mut count,
            out.as_mut_ptr(),
        );
        assert_eq!(count as usize, out.len());
    }
}

/// Vulkan core 1.2
impl Device {
    #[inline]
    pub fn fp_v1_2(&self) -> &crate::DeviceFnV1_2 {
        &self.device_fn_1_2
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectCount.html>
    #[inline]
    pub unsafe fn cmd_draw_indirect_count(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self.device_fn_1_2.cmd_draw_indirect_count)(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirectCount.html>
    #[inline]
    pub unsafe fn cmd_draw_indexed_indirect_count(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self.device_fn_1_2.cmd_draw_indexed_indirect_count)(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass2.html>
    #[inline]
    pub unsafe fn create_render_pass2(
        &self,
        create_info: &vk::RenderPassCreateInfo2<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::RenderPass> {
        let mut renderpass = mem::MaybeUninit::uninit();
        (self.device_fn_1_2.create_render_pass2)(
            self.handle(),
            create_info,
            allocation_callbacks.as_raw_ptr(),
            renderpass.as_mut_ptr(),
        )
        .assume_init_on_success(renderpass)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass2.html>
    #[inline]
    pub unsafe fn cmd_begin_render_pass2(
        &self,
        command_buffer: vk::CommandBuffer,
        render_pass_begin_info: &vk::RenderPassBeginInfo<'_>,
        subpass_begin_info: &vk::SubpassBeginInfo<'_>,
    ) {
        (self.device_fn_1_2.cmd_begin_render_pass2)(
            command_buffer,
            render_pass_begin_info,
            subpass_begin_info,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass2.html>
    #[inline]
    pub unsafe fn cmd_next_subpass2(
        &self,
        command_buffer: vk::CommandBuffer,
        subpass_begin_info: &vk::SubpassBeginInfo<'_>,
        subpass_end_info: &vk::SubpassEndInfo<'_>,
    ) {
        (self.device_fn_1_2.cmd_next_subpass2)(
            command_buffer,
            subpass_begin_info,
            subpass_end_info,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass2.html>
    #[inline]
    pub unsafe fn cmd_end_render_pass2(
        &self,
        command_buffer: vk::CommandBuffer,
        subpass_end_info: &vk::SubpassEndInfo<'_>,
    ) {
        (self.device_fn_1_2.cmd_end_render_pass2)(command_buffer, subpass_end_info);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetQueryPool.html>
    #[inline]
    pub unsafe fn reset_query_pool(
        &self,
        query_pool: vk::QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        (self.device_fn_1_2.reset_query_pool)(self.handle(), query_pool, first_query, query_count);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreCounterValue.html>
    #[inline]
    pub unsafe fn get_semaphore_counter_value(&self, semaphore: vk::Semaphore) -> VkResult<u64> {
        let mut value = mem::MaybeUninit::uninit();
        (self.device_fn_1_2.get_semaphore_counter_value)(
            self.handle(),
            semaphore,
            value.as_mut_ptr(),
        )
        .assume_init_on_success(value)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWaitSemaphores.html>
    #[inline]
    pub unsafe fn wait_semaphores(
        &self,
        wait_info: &vk::SemaphoreWaitInfo<'_>,
        timeout: u64,
    ) -> VkResult<()> {
        (self.device_fn_1_2.wait_semaphores)(self.handle(), wait_info, timeout).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSignalSemaphore.html>
    #[inline]
    pub unsafe fn signal_semaphore(
        &self,
        signal_info: &vk::SemaphoreSignalInfo<'_>,
    ) -> VkResult<()> {
        (self.device_fn_1_2.signal_semaphore)(self.handle(), signal_info).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddress.html>
    #[inline]
    pub unsafe fn get_buffer_device_address(
        &self,
        info: &vk::BufferDeviceAddressInfo<'_>,
    ) -> vk::DeviceAddress {
        (self.device_fn_1_2.get_buffer_device_address)(self.handle(), info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferOpaqueCaptureAddress.html>
    #[inline]
    pub unsafe fn get_buffer_opaque_capture_address(
        &self,
        info: &vk::BufferDeviceAddressInfo<'_>,
    ) -> u64 {
        (self.device_fn_1_2.get_buffer_opaque_capture_address)(self.handle(), info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddress.html>
    #[inline]
    pub unsafe fn get_device_memory_opaque_capture_address(
        &self,
        info: &vk::DeviceMemoryOpaqueCaptureAddressInfo<'_>,
    ) -> u64 {
        (self.device_fn_1_2.get_device_memory_opaque_capture_address)(self.handle(), info)
    }
}

/// Vulkan core 1.1
impl Device {
    #[inline]
    pub fn fp_v1_1(&self) -> &crate::DeviceFnV1_1 {
        &self.device_fn_1_1
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory2.html>
    #[inline]
    pub unsafe fn bind_buffer_memory2(
        &self,
        bind_infos: &[vk::BindBufferMemoryInfo<'_>],
    ) -> VkResult<()> {
        (self.device_fn_1_1.bind_buffer_memory2)(
            self.handle(),
            bind_infos.len() as _,
            bind_infos.as_ptr(),
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory2.html>
    #[inline]
    pub unsafe fn bind_image_memory2(
        &self,
        bind_infos: &[vk::BindImageMemoryInfo<'_>],
    ) -> VkResult<()> {
        (self.device_fn_1_1.bind_image_memory2)(
            self.handle(),
            bind_infos.len() as _,
            bind_infos.as_ptr(),
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPeerMemoryFeatures.html>
    #[inline]
    pub unsafe fn get_device_group_peer_memory_features(
        &self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
    ) -> vk::PeerMemoryFeatureFlags {
        let mut peer_memory_features = mem::MaybeUninit::uninit();
        (self.device_fn_1_1.get_device_group_peer_memory_features)(
            self.handle(),
            heap_index,
            local_device_index,
            remote_device_index,
            peer_memory_features.as_mut_ptr(),
        );
        peer_memory_features.assume_init()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDeviceMask.html>
    #[inline]
    pub unsafe fn cmd_set_device_mask(&self, command_buffer: vk::CommandBuffer, device_mask: u32) {
        (self.device_fn_1_1.cmd_set_device_mask)(command_buffer, device_mask);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchBase.html>
    #[inline]
    pub unsafe fn cmd_dispatch_base(
        &self,
        command_buffer: vk::CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        (self.device_fn_1_1.cmd_dispatch_base)(
            command_buffer,
            base_group_x,
            base_group_y,
            base_group_z,
            group_count_x,
            group_count_y,
            group_count_z,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements2.html>
    #[inline]
    pub unsafe fn get_image_memory_requirements2(
        &self,
        info: &vk::ImageMemoryRequirementsInfo2<'_>,
        out: &mut vk::MemoryRequirements2<'_>,
    ) {
        (self.device_fn_1_1.get_image_memory_requirements2)(self.handle(), info, out);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements2.html>
    #[inline]
    pub unsafe fn get_buffer_memory_requirements2(
        &self,
        info: &vk::BufferMemoryRequirementsInfo2<'_>,
        out: &mut vk::MemoryRequirements2<'_>,
    ) {
        (self.device_fn_1_1.get_buffer_memory_requirements2)(self.handle(), info, out);
    }

    /// Retrieve the number of elements to pass to [`get_image_sparse_memory_requirements2()`][Self::get_image_sparse_memory_requirements2()]
    #[inline]
    pub unsafe fn get_image_sparse_memory_requirements2_len(
        &self,
        info: &vk::ImageSparseMemoryRequirementsInfo2<'_>,
    ) -> usize {
        let mut count = mem::MaybeUninit::uninit();
        (self.device_fn_1_1.get_image_sparse_memory_requirements2)(
            self.handle(),
            info,
            count.as_mut_ptr(),
            ptr::null_mut(),
        );
        count.assume_init() as usize
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements2.html>
    ///
    /// Call [`get_image_sparse_memory_requirements2_len()`][Self::get_image_sparse_memory_requirements2_len()] to query the number of elements to pass to `out`.
    /// Be sure to [`Default::default()`]-initialize these elements and optionally set their `p_next` pointer.
    #[inline]
    pub unsafe fn get_image_sparse_memory_requirements2(
        &self,
        info: &vk::ImageSparseMemoryRequirementsInfo2<'_>,
        out: &mut [vk::SparseImageMemoryRequirements2<'_>],
    ) {
        let mut count = out.len() as u32;
        (self.device_fn_1_1.get_image_sparse_memory_requirements2)(
            self.handle(),
            info,
            &mut count,
            out.as_mut_ptr(),
        );
        assert_eq!(count as usize, out.len());
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkTrimCommandPool.html>
    #[inline]
    pub unsafe fn trim_command_pool(
        &self,
        command_pool: vk::CommandPool,
        flags: vk::CommandPoolTrimFlags,
    ) {
        (self.device_fn_1_1.trim_command_pool)(self.handle(), command_pool, flags);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue2.html>
    #[inline]
    pub unsafe fn get_device_queue2(&self, queue_info: &vk::DeviceQueueInfo2<'_>) -> vk::Queue {
        let mut queue = mem::MaybeUninit::uninit();
        (self.device_fn_1_1.get_device_queue2)(self.handle(), queue_info, queue.as_mut_ptr());
        queue.assume_init()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSamplerYcbcrConversion.html>
    #[inline]
    pub unsafe fn create_sampler_ycbcr_conversion(
        &self,
        create_info: &vk::SamplerYcbcrConversionCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::SamplerYcbcrConversion> {
        let mut ycbcr_conversion = mem::MaybeUninit::uninit();
        (self.device_fn_1_1.create_sampler_ycbcr_conversion)(
            self.handle(),
            create_info,
            allocation_callbacks.as_raw_ptr(),
            ycbcr_conversion.as_mut_ptr(),
        )
        .assume_init_on_success(ycbcr_conversion)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySamplerYcbcrConversion.html>
    #[inline]
    pub unsafe fn destroy_sampler_ycbcr_conversion(
        &self,
        ycbcr_conversion: vk::SamplerYcbcrConversion,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.device_fn_1_1.destroy_sampler_ycbcr_conversion)(
            self.handle(),
            ycbcr_conversion,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorUpdateTemplate.html>
    #[inline]
    pub unsafe fn create_descriptor_update_template(
        &self,
        create_info: &vk::DescriptorUpdateTemplateCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::DescriptorUpdateTemplate> {
        let mut descriptor_update_template = mem::MaybeUninit::uninit();
        (self.device_fn_1_1.create_descriptor_update_template)(
            self.handle(),
            create_info,
            allocation_callbacks.as_raw_ptr(),
            descriptor_update_template.as_mut_ptr(),
        )
        .assume_init_on_success(descriptor_update_template)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorUpdateTemplate.html>
    #[inline]
    pub unsafe fn destroy_descriptor_update_template(
        &self,
        descriptor_update_template: vk::DescriptorUpdateTemplate,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.device_fn_1_1.destroy_descriptor_update_template)(
            self.handle(),
            descriptor_update_template,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSetWithTemplate.html>
    #[inline]
    pub unsafe fn update_descriptor_set_with_template(
        &self,
        descriptor_set: vk::DescriptorSet,
        descriptor_update_template: vk::DescriptorUpdateTemplate,
        data: *const ffi::c_void,
    ) {
        (self.device_fn_1_1.update_descriptor_set_with_template)(
            self.handle(),
            descriptor_set,
            descriptor_update_template,
            data,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSupport.html>
    #[inline]
    pub unsafe fn get_descriptor_set_layout_support(
        &self,
        create_info: &vk::DescriptorSetLayoutCreateInfo<'_>,
        out: &mut vk::DescriptorSetLayoutSupport<'_>,
    ) {
        (self.device_fn_1_1.get_descriptor_set_layout_support)(self.handle(), create_info, out);
    }
}

/// Vulkan core 1.0
impl Device {
    #[inline]
    pub fn fp_v1_0(&self) -> &crate::DeviceFnV1_0 {
        &self.device_fn_1_0
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDevice.html>
    #[inline]
    pub unsafe fn destroy_device(
        &self,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.device_fn_1_0.destroy_device)(self.handle(), allocation_callbacks.as_raw_ptr());
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySampler.html>
    #[inline]
    pub unsafe fn destroy_sampler(
        &self,
        sampler: vk::Sampler,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.device_fn_1_0.destroy_sampler)(
            self.handle(),
            sampler,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFreeMemory.html>
    #[inline]
    pub unsafe fn free_memory(
        &self,
        memory: vk::DeviceMemory,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.device_fn_1_0.free_memory)(self.handle(), memory, allocation_callbacks.as_raw_ptr());
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFreeCommandBuffers.html>
    #[inline]
    pub unsafe fn free_command_buffers(
        &self,
        command_pool: vk::CommandPool,
        command_buffers: &[vk::CommandBuffer],
    ) {
        (self.device_fn_1_0.free_command_buffers)(
            self.handle(),
            command_pool,
            command_buffers.len() as u32,
            command_buffers.as_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateEvent.html>
    #[inline]
    pub unsafe fn create_event(
        &self,
        create_info: &vk::EventCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::Event> {
        let mut event = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.create_event)(
            self.handle(),
            create_info,
            allocation_callbacks.as_raw_ptr(),
            event.as_mut_ptr(),
        )
        .assume_init_on_success(event)
    }

    /// Returns [`true`] if the event was set, and [`false`] if the event was reset, otherwise it will
    /// return the error code.
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetEventStatus.html>
    #[inline]
    pub unsafe fn get_event_status(&self, event: vk::Event) -> VkResult<bool> {
        let err_code = (self.device_fn_1_0.get_event_status)(self.handle(), event);
        match err_code {
            vk::Result::EVENT_SET => Ok(true),
            vk::Result::EVENT_RESET => Ok(false),
            _ => Err(err_code),
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetEvent.html>
    #[inline]
    pub unsafe fn set_event(&self, event: vk::Event) -> VkResult<()> {
        (self.device_fn_1_0.set_event)(self.handle(), event).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetEvent.html>
    #[inline]
    pub unsafe fn reset_event(&self, event: vk::Event) -> VkResult<()> {
        (self.device_fn_1_0.reset_event)(self.handle(), event).result()
    }
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent.html>
    #[inline]
    pub unsafe fn cmd_set_event(
        &self,
        command_buffer: vk::CommandBuffer,
        event: vk::Event,
        stage_mask: vk::PipelineStageFlags,
    ) {
        (self.device_fn_1_0.cmd_set_event)(command_buffer, event, stage_mask);
    }
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent.html>
    #[inline]
    pub unsafe fn cmd_reset_event(
        &self,
        command_buffer: vk::CommandBuffer,
        event: vk::Event,
        stage_mask: vk::PipelineStageFlags,
    ) {
        (self.device_fn_1_0.cmd_reset_event)(command_buffer, event, stage_mask);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents.html>
    #[inline]
    pub unsafe fn cmd_wait_events(
        &self,
        command_buffer: vk::CommandBuffer,
        events: &[vk::Event],
        src_stage_mask: vk::PipelineStageFlags,
        dst_stage_mask: vk::PipelineStageFlags,
        memory_barriers: &[vk::MemoryBarrier<'_>],
        buffer_memory_barriers: &[vk::BufferMemoryBarrier<'_>],
        image_memory_barriers: &[vk::ImageMemoryBarrier<'_>],
    ) {
        (self.device_fn_1_0.cmd_wait_events)(
            command_buffer,
            events.len() as _,
            events.as_ptr(),
            src_stage_mask,
            dst_stage_mask,
            memory_barriers.len() as _,
            memory_barriers.as_ptr(),
            buffer_memory_barriers.len() as _,
            buffer_memory_barriers.as_ptr(),
            image_memory_barriers.len() as _,
            image_memory_barriers.as_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyFence.html>
    #[inline]
    pub unsafe fn destroy_fence(
        &self,
        fence: vk::Fence,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.device_fn_1_0.destroy_fence)(self.handle(), fence, allocation_callbacks.as_raw_ptr());
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyEvent.html>
    #[inline]
    pub unsafe fn destroy_event(
        &self,
        event: vk::Event,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.device_fn_1_0.destroy_event)(self.handle(), event, allocation_callbacks.as_raw_ptr());
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyImage.html>
    #[inline]
    pub unsafe fn destroy_image(
        &self,
        image: vk::Image,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.device_fn_1_0.destroy_image)(self.handle(), image, allocation_callbacks.as_raw_ptr());
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyCommandPool.html>
    #[inline]
    pub unsafe fn destroy_command_pool(
        &self,
        pool: vk::CommandPool,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.device_fn_1_0.destroy_command_pool)(
            self.handle(),
            pool,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyImageView.html>
    #[inline]
    pub unsafe fn destroy_image_view(
        &self,
        image_view: vk::ImageView,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.device_fn_1_0.destroy_image_view)(
            self.handle(),
            image_view,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyRenderPass.html>
    #[inline]
    pub unsafe fn destroy_render_pass(
        &self,
        renderpass: vk::RenderPass,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.device_fn_1_0.destroy_render_pass)(
            self.handle(),
            renderpass,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyFramebuffer.html>
    #[inline]
    pub unsafe fn destroy_framebuffer(
        &self,
        framebuffer: vk::Framebuffer,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.device_fn_1_0.destroy_framebuffer)(
            self.handle(),
            framebuffer,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineLayout.html>
    #[inline]
    pub unsafe fn destroy_pipeline_layout(
        &self,
        pipeline_layout: vk::PipelineLayout,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.device_fn_1_0.destroy_pipeline_layout)(
            self.handle(),
            pipeline_layout,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineCache.html>
    #[inline]
    pub unsafe fn destroy_pipeline_cache(
        &self,
        pipeline_cache: vk::PipelineCache,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.device_fn_1_0.destroy_pipeline_cache)(
            self.handle(),
            pipeline_cache,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyBuffer.html>
    #[inline]
    pub unsafe fn destroy_buffer(
        &self,
        buffer: vk::Buffer,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.device_fn_1_0.destroy_buffer)(
            self.handle(),
            buffer,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyShaderModule.html>
    #[inline]
    pub unsafe fn destroy_shader_module(
        &self,
        shader: vk::ShaderModule,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.device_fn_1_0.destroy_shader_module)(
            self.handle(),
            shader,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyPipeline.html>
    #[inline]
    pub unsafe fn destroy_pipeline(
        &self,
        pipeline: vk::Pipeline,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.device_fn_1_0.destroy_pipeline)(
            self.handle(),
            pipeline,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySemaphore.html>
    #[inline]
    pub unsafe fn destroy_semaphore(
        &self,
        semaphore: vk::Semaphore,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.device_fn_1_0.destroy_semaphore)(
            self.handle(),
            semaphore,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorPool.html>
    #[inline]
    pub unsafe fn destroy_descriptor_pool(
        &self,
        pool: vk::DescriptorPool,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.device_fn_1_0.destroy_descriptor_pool)(
            self.handle(),
            pool,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyQueryPool.html>
    #[inline]
    pub unsafe fn destroy_query_pool(
        &self,
        pool: vk::QueryPool,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.device_fn_1_0.destroy_query_pool)(
            self.handle(),
            pool,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorSetLayout.html>
    #[inline]
    pub unsafe fn destroy_descriptor_set_layout(
        &self,
        layout: vk::DescriptorSetLayout,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.device_fn_1_0.destroy_descriptor_set_layout)(
            self.handle(),
            layout,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFreeDescriptorSets.html>
    #[inline]
    pub unsafe fn free_descriptor_sets(
        &self,
        pool: vk::DescriptorPool,
        descriptor_sets: &[vk::DescriptorSet],
    ) -> VkResult<()> {
        (self.device_fn_1_0.free_descriptor_sets)(
            self.handle(),
            pool,
            descriptor_sets.len() as u32,
            descriptor_sets.as_ptr(),
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSets.html>
    #[inline]
    pub unsafe fn update_descriptor_sets(
        &self,
        descriptor_writes: &[vk::WriteDescriptorSet<'_>],
        descriptor_copies: &[vk::CopyDescriptorSet<'_>],
    ) {
        (self.device_fn_1_0.update_descriptor_sets)(
            self.handle(),
            descriptor_writes.len() as u32,
            descriptor_writes.as_ptr(),
            descriptor_copies.len() as u32,
            descriptor_copies.as_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSampler.html>
    #[inline]
    pub unsafe fn create_sampler(
        &self,
        create_info: &vk::SamplerCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::Sampler> {
        let mut sampler = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.create_sampler)(
            self.handle(),
            create_info,
            allocation_callbacks.as_raw_ptr(),
            sampler.as_mut_ptr(),
        )
        .assume_init_on_success(sampler)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage.html>
    #[inline]
    pub unsafe fn cmd_blit_image(
        &self,
        command_buffer: vk::CommandBuffer,
        src_image: vk::Image,
        src_image_layout: vk::ImageLayout,
        dst_image: vk::Image,
        dst_image_layout: vk::ImageLayout,
        regions: &[vk::ImageBlit],
        filter: vk::Filter,
    ) {
        (self.device_fn_1_0.cmd_blit_image)(
            command_buffer,
            src_image,
            src_image_layout,
            dst_image,
            dst_image_layout,
            regions.len() as _,
            regions.as_ptr(),
            filter,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage.html>
    #[inline]
    pub unsafe fn cmd_resolve_image(
        &self,
        command_buffer: vk::CommandBuffer,
        src_image: vk::Image,
        src_image_layout: vk::ImageLayout,
        dst_image: vk::Image,
        dst_image_layout: vk::ImageLayout,
        regions: &[vk::ImageResolve],
    ) {
        (self.device_fn_1_0.cmd_resolve_image)(
            command_buffer,
            src_image,
            src_image_layout,
            dst_image,
            dst_image_layout,
            regions.len() as u32,
            regions.as_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdFillBuffer.html>
    #[inline]
    pub unsafe fn cmd_fill_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
        data: u32,
    ) {
        (self.device_fn_1_0.cmd_fill_buffer)(command_buffer, buffer, offset, size, data);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdUpdateBuffer.html>
    #[inline]
    pub unsafe fn cmd_update_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        data: &[u8],
    ) {
        (self.device_fn_1_0.cmd_update_buffer)(
            command_buffer,
            buffer,
            offset,
            data.len() as u64,
            data.as_ptr() as _,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer.html>
    #[inline]
    pub unsafe fn cmd_copy_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        src_buffer: vk::Buffer,
        dst_buffer: vk::Buffer,
        regions: &[vk::BufferCopy],
    ) {
        (self.device_fn_1_0.cmd_copy_buffer)(
            command_buffer,
            src_buffer,
            dst_buffer,
            regions.len() as u32,
            regions.as_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer.html>
    #[inline]
    pub unsafe fn cmd_copy_image_to_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        src_image: vk::Image,
        src_image_layout: vk::ImageLayout,
        dst_buffer: vk::Buffer,
        regions: &[vk::BufferImageCopy],
    ) {
        (self.device_fn_1_0.cmd_copy_image_to_buffer)(
            command_buffer,
            src_image,
            src_image_layout,
            dst_buffer,
            regions.len() as u32,
            regions.as_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage.html>
    #[inline]
    pub unsafe fn cmd_copy_buffer_to_image(
        &self,
        command_buffer: vk::CommandBuffer,
        src_buffer: vk::Buffer,
        dst_image: vk::Image,
        dst_image_layout: vk::ImageLayout,
        regions: &[vk::BufferImageCopy],
    ) {
        (self.device_fn_1_0.cmd_copy_buffer_to_image)(
            command_buffer,
            src_buffer,
            dst_image,
            dst_image_layout,
            regions.len() as u32,
            regions.as_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage.html>
    #[inline]
    pub unsafe fn cmd_copy_image(
        &self,
        command_buffer: vk::CommandBuffer,
        src_image: vk::Image,
        src_image_layout: vk::ImageLayout,
        dst_image: vk::Image,
        dst_image_layout: vk::ImageLayout,
        regions: &[vk::ImageCopy],
    ) {
        (self.device_fn_1_0.cmd_copy_image)(
            command_buffer,
            src_image,
            src_image_layout,
            dst_image,
            dst_image_layout,
            regions.len() as u32,
            regions.as_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAllocateDescriptorSets.html>
    #[inline]
    pub unsafe fn allocate_descriptor_sets(
        &self,
        allocate_info: &vk::DescriptorSetAllocateInfo<'_>,
    ) -> VkResult<Vec<vk::DescriptorSet>> {
        let mut desc_set = Vec::with_capacity(allocate_info.descriptor_set_count as usize);
        (self.device_fn_1_0.allocate_descriptor_sets)(
            self.handle(),
            allocate_info,
            desc_set.as_mut_ptr(),
        )
        .set_vec_len_on_success(desc_set, allocate_info.descriptor_set_count as usize)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorSetLayout.html>
    #[inline]
    pub unsafe fn create_descriptor_set_layout(
        &self,
        create_info: &vk::DescriptorSetLayoutCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::DescriptorSetLayout> {
        let mut layout = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.create_descriptor_set_layout)(
            self.handle(),
            create_info,
            allocation_callbacks.as_raw_ptr(),
            layout.as_mut_ptr(),
        )
        .assume_init_on_success(layout)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDeviceWaitIdle.html>
    #[inline]
    pub unsafe fn device_wait_idle(&self) -> VkResult<()> {
        (self.device_fn_1_0.device_wait_idle)(self.handle()).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorPool.html>
    #[inline]
    pub unsafe fn create_descriptor_pool(
        &self,
        create_info: &vk::DescriptorPoolCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::DescriptorPool> {
        let mut pool = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.create_descriptor_pool)(
            self.handle(),
            create_info,
            allocation_callbacks.as_raw_ptr(),
            pool.as_mut_ptr(),
        )
        .assume_init_on_success(pool)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetDescriptorPool.html>
    #[inline]
    pub unsafe fn reset_descriptor_pool(
        &self,
        pool: vk::DescriptorPool,
        flags: vk::DescriptorPoolResetFlags,
    ) -> VkResult<()> {
        (self.device_fn_1_0.reset_descriptor_pool)(self.handle(), pool, flags).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetCommandPool.html>
    #[inline]
    pub unsafe fn reset_command_pool(
        &self,
        command_pool: vk::CommandPool,
        flags: vk::CommandPoolResetFlags,
    ) -> VkResult<()> {
        (self.device_fn_1_0.reset_command_pool)(self.handle(), command_pool, flags).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetCommandBuffer.html>
    #[inline]
    pub unsafe fn reset_command_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        flags: vk::CommandBufferResetFlags,
    ) -> VkResult<()> {
        (self.device_fn_1_0.reset_command_buffer)(command_buffer, flags).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetFences.html>
    #[inline]
    pub unsafe fn reset_fences(&self, fences: &[vk::Fence]) -> VkResult<()> {
        (self.device_fn_1_0.reset_fences)(self.handle(), fences.len() as u32, fences.as_ptr())
            .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindIndexBuffer.html>
    #[inline]
    pub unsafe fn cmd_bind_index_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        index_type: vk::IndexType,
    ) {
        (self.device_fn_1_0.cmd_bind_index_buffer)(command_buffer, buffer, offset, index_type);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdClearColorImage.html>
    #[inline]
    pub unsafe fn cmd_clear_color_image(
        &self,
        command_buffer: vk::CommandBuffer,
        image: vk::Image,
        image_layout: vk::ImageLayout,
        clear_color_value: &vk::ClearColorValue,
        ranges: &[vk::ImageSubresourceRange],
    ) {
        (self.device_fn_1_0.cmd_clear_color_image)(
            command_buffer,
            image,
            image_layout,
            clear_color_value,
            ranges.len() as u32,
            ranges.as_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdClearDepthStencilImage.html>
    #[inline]
    pub unsafe fn cmd_clear_depth_stencil_image(
        &self,
        command_buffer: vk::CommandBuffer,
        image: vk::Image,
        image_layout: vk::ImageLayout,
        clear_depth_stencil_value: &vk::ClearDepthStencilValue,
        ranges: &[vk::ImageSubresourceRange],
    ) {
        (self.device_fn_1_0.cmd_clear_depth_stencil_image)(
            command_buffer,
            image,
            image_layout,
            clear_depth_stencil_value,
            ranges.len() as u32,
            ranges.as_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdClearAttachments.html>
    #[inline]
    pub unsafe fn cmd_clear_attachments(
        &self,
        command_buffer: vk::CommandBuffer,
        attachments: &[vk::ClearAttachment],
        rects: &[vk::ClearRect],
    ) {
        (self.device_fn_1_0.cmd_clear_attachments)(
            command_buffer,
            attachments.len() as u32,
            attachments.as_ptr(),
            rects.len() as u32,
            rects.as_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexed.html>
    #[inline]
    pub unsafe fn cmd_draw_indexed(
        &self,
        command_buffer: vk::CommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        (self.device_fn_1_0.cmd_draw_indexed)(
            command_buffer,
            index_count,
            instance_count,
            first_index,
            vertex_offset,
            first_instance,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirect.html>
    #[inline]
    pub unsafe fn cmd_draw_indexed_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        (self.device_fn_1_0.cmd_draw_indexed_indirect)(
            command_buffer,
            buffer,
            offset,
            draw_count,
            stride,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteCommands.html>
    #[inline]
    pub unsafe fn cmd_execute_commands(
        &self,
        primary_command_buffer: vk::CommandBuffer,
        secondary_command_buffers: &[vk::CommandBuffer],
    ) {
        (self.device_fn_1_0.cmd_execute_commands)(
            primary_command_buffer,
            secondary_command_buffers.len() as u32,
            secondary_command_buffers.as_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorSets.html>
    #[inline]
    pub unsafe fn cmd_bind_descriptor_sets(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        layout: vk::PipelineLayout,
        first_set: u32,
        descriptor_sets: &[vk::DescriptorSet],
        dynamic_offsets: &[u32],
    ) {
        (self.device_fn_1_0.cmd_bind_descriptor_sets)(
            command_buffer,
            pipeline_bind_point,
            layout,
            first_set,
            descriptor_sets.len() as u32,
            descriptor_sets.as_ptr(),
            dynamic_offsets.len() as u32,
            dynamic_offsets.as_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyQueryPoolResults.html>
    #[inline]
    pub unsafe fn cmd_copy_query_pool_results(
        &self,
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: vk::Buffer,
        dst_offset: vk::DeviceSize,
        stride: vk::DeviceSize,
        flags: vk::QueryResultFlags,
    ) {
        (self.device_fn_1_0.cmd_copy_query_pool_results)(
            command_buffer,
            query_pool,
            first_query,
            query_count,
            dst_buffer,
            dst_offset,
            stride,
            flags,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPushConstants.html>
    #[inline]
    pub unsafe fn cmd_push_constants(
        &self,
        command_buffer: vk::CommandBuffer,
        layout: vk::PipelineLayout,
        stage_flags: vk::ShaderStageFlags,
        offset: u32,
        constants: &[u8],
    ) {
        (self.device_fn_1_0.cmd_push_constants)(
            command_buffer,
            layout,
            stage_flags,
            offset,
            constants.len() as _,
            constants.as_ptr() as _,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass.html>
    #[inline]
    pub unsafe fn cmd_begin_render_pass(
        &self,
        command_buffer: vk::CommandBuffer,
        render_pass_begin: &vk::RenderPassBeginInfo<'_>,
        contents: vk::SubpassContents,
    ) {
        (self.device_fn_1_0.cmd_begin_render_pass)(command_buffer, render_pass_begin, contents);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass.html>
    #[inline]
    pub unsafe fn cmd_next_subpass(
        &self,
        command_buffer: vk::CommandBuffer,
        contents: vk::SubpassContents,
    ) {
        (self.device_fn_1_0.cmd_next_subpass)(command_buffer, contents);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipeline.html>
    #[inline]
    pub unsafe fn cmd_bind_pipeline(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        pipeline: vk::Pipeline,
    ) {
        (self.device_fn_1_0.cmd_bind_pipeline)(command_buffer, pipeline_bind_point, pipeline);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissor.html>
    #[inline]
    pub unsafe fn cmd_set_scissor(
        &self,
        command_buffer: vk::CommandBuffer,
        first_scissor: u32,
        scissors: &[vk::Rect2D],
    ) {
        (self.device_fn_1_0.cmd_set_scissor)(
            command_buffer,
            first_scissor,
            scissors.len() as u32,
            scissors.as_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineWidth.html>
    #[inline]
    pub unsafe fn cmd_set_line_width(&self, command_buffer: vk::CommandBuffer, line_width: f32) {
        (self.device_fn_1_0.cmd_set_line_width)(command_buffer, line_width);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers.html>
    #[inline]
    pub unsafe fn cmd_bind_vertex_buffers(
        &self,
        command_buffer: vk::CommandBuffer,
        first_binding: u32,
        buffers: &[vk::Buffer],
        offsets: &[vk::DeviceSize],
    ) {
        debug_assert_eq!(buffers.len(), offsets.len());
        (self.device_fn_1_0.cmd_bind_vertex_buffers)(
            command_buffer,
            first_binding,
            buffers.len() as u32,
            buffers.as_ptr(),
            offsets.as_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass.html>
    #[inline]
    pub unsafe fn cmd_end_render_pass(&self, command_buffer: vk::CommandBuffer) {
        (self.device_fn_1_0.cmd_end_render_pass)(command_buffer);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDraw.html>
    #[inline]
    pub unsafe fn cmd_draw(
        &self,
        command_buffer: vk::CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        (self.device_fn_1_0.cmd_draw)(
            command_buffer,
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirect.html>
    #[inline]
    pub unsafe fn cmd_draw_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        (self.device_fn_1_0.cmd_draw_indirect)(command_buffer, buffer, offset, draw_count, stride);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDispatch.html>
    #[inline]
    pub unsafe fn cmd_dispatch(
        &self,
        command_buffer: vk::CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        (self.device_fn_1_0.cmd_dispatch)(
            command_buffer,
            group_count_x,
            group_count_y,
            group_count_z,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchIndirect.html>
    #[inline]
    pub unsafe fn cmd_dispatch_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
    ) {
        (self.device_fn_1_0.cmd_dispatch_indirect)(command_buffer, buffer, offset);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewport.html>
    #[inline]
    pub unsafe fn cmd_set_viewport(
        &self,
        command_buffer: vk::CommandBuffer,
        first_viewport: u32,
        viewports: &[vk::Viewport],
    ) {
        (self.device_fn_1_0.cmd_set_viewport)(
            command_buffer,
            first_viewport,
            viewports.len() as u32,
            viewports.as_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBias.html>
    #[inline]
    pub unsafe fn cmd_set_depth_bias(
        &self,
        command_buffer: vk::CommandBuffer,
        constant_factor: f32,
        clamp: f32,
        slope_factor: f32,
    ) {
        (self.device_fn_1_0.cmd_set_depth_bias)(
            command_buffer,
            constant_factor,
            clamp,
            slope_factor,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetBlendConstants.html>
    #[inline]
    pub unsafe fn cmd_set_blend_constants(
        &self,
        command_buffer: vk::CommandBuffer,
        blend_constants: &[f32; 4],
    ) {
        (self.device_fn_1_0.cmd_set_blend_constants)(command_buffer, blend_constants);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBounds.html>
    #[inline]
    pub unsafe fn cmd_set_depth_bounds(
        &self,
        command_buffer: vk::CommandBuffer,
        min_depth_bounds: f32,
        max_depth_bounds: f32,
    ) {
        (self.device_fn_1_0.cmd_set_depth_bounds)(
            command_buffer,
            min_depth_bounds,
            max_depth_bounds,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilCompareMask.html>
    #[inline]
    pub unsafe fn cmd_set_stencil_compare_mask(
        &self,
        command_buffer: vk::CommandBuffer,
        face_mask: vk::StencilFaceFlags,
        compare_mask: u32,
    ) {
        (self.device_fn_1_0.cmd_set_stencil_compare_mask)(command_buffer, face_mask, compare_mask);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilWriteMask.html>
    #[inline]
    pub unsafe fn cmd_set_stencil_write_mask(
        &self,
        command_buffer: vk::CommandBuffer,
        face_mask: vk::StencilFaceFlags,
        write_mask: u32,
    ) {
        (self.device_fn_1_0.cmd_set_stencil_write_mask)(command_buffer, face_mask, write_mask);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilReference.html>
    #[inline]
    pub unsafe fn cmd_set_stencil_reference(
        &self,
        command_buffer: vk::CommandBuffer,
        face_mask: vk::StencilFaceFlags,
        reference: u32,
    ) {
        (self.device_fn_1_0.cmd_set_stencil_reference)(command_buffer, face_mask, reference);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetQueryPoolResults.html>
    #[inline]
    pub unsafe fn get_query_pool_results<T>(
        &self,
        query_pool: vk::QueryPool,
        first_query: u32,
        data: &mut [T],
        flags: vk::QueryResultFlags,
    ) -> VkResult<()> {
        let data_size = mem::size_of_val(data);
        (self.device_fn_1_0.get_query_pool_results)(
            self.handle(),
            query_pool,
            first_query,
            data.len() as u32,
            data_size,
            data.as_mut_ptr().cast(),
            mem::size_of::<T>() as _,
            flags,
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQuery.html>
    #[inline]
    pub unsafe fn cmd_begin_query(
        &self,
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        query: u32,
        flags: vk::QueryControlFlags,
    ) {
        (self.device_fn_1_0.cmd_begin_query)(command_buffer, query_pool, query, flags);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndQuery.html>
    #[inline]
    pub unsafe fn cmd_end_query(
        &self,
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        query: u32,
    ) {
        (self.device_fn_1_0.cmd_end_query)(command_buffer, query_pool, query);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResetQueryPool.html>
    #[inline]
    pub unsafe fn cmd_reset_query_pool(
        &self,
        command_buffer: vk::CommandBuffer,
        pool: vk::QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        (self.device_fn_1_0.cmd_reset_query_pool)(command_buffer, pool, first_query, query_count);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp.html>
    #[inline]
    pub unsafe fn cmd_write_timestamp(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_stage: vk::PipelineStageFlags,
        query_pool: vk::QueryPool,
        query: u32,
    ) {
        (self.device_fn_1_0.cmd_write_timestamp)(command_buffer, pipeline_stage, query_pool, query);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSemaphore.html>
    #[inline]
    pub unsafe fn create_semaphore(
        &self,
        create_info: &vk::SemaphoreCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::Semaphore> {
        let mut semaphore = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.create_semaphore)(
            self.handle(),
            create_info,
            allocation_callbacks.as_raw_ptr(),
            semaphore.as_mut_ptr(),
        )
        .assume_init_on_success(semaphore)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateGraphicsPipelines.html>
    ///
    /// Pipelines are created and returned as described for [Multiple Pipeline Creation].
    ///
    /// [Multiple Pipeline Creation]: https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-multiple
    #[inline]
    pub unsafe fn create_graphics_pipelines(
        &self,
        pipeline_cache: vk::PipelineCache,
        create_infos: &[vk::GraphicsPipelineCreateInfo<'_>],
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> Result<Vec<vk::Pipeline>, (Vec<vk::Pipeline>, vk::Result)> {
        let mut pipelines = Vec::with_capacity(create_infos.len());
        let err_code = (self.device_fn_1_0.create_graphics_pipelines)(
            self.handle(),
            pipeline_cache,
            create_infos.len() as u32,
            create_infos.as_ptr(),
            allocation_callbacks.as_raw_ptr(),
            pipelines.as_mut_ptr(),
        );
        pipelines.set_len(create_infos.len());
        match err_code {
            vk::Result::SUCCESS => Ok(pipelines),
            _ => Err((pipelines, err_code)),
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateComputePipelines.html>
    ///
    /// Pipelines are created and returned as described for [Multiple Pipeline Creation].
    ///
    /// [Multiple Pipeline Creation]: https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-multiple
    #[inline]
    pub unsafe fn create_compute_pipelines(
        &self,
        pipeline_cache: vk::PipelineCache,
        create_infos: &[vk::ComputePipelineCreateInfo<'_>],
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> Result<Vec<vk::Pipeline>, (Vec<vk::Pipeline>, vk::Result)> {
        let mut pipelines = Vec::with_capacity(create_infos.len());
        let err_code = (self.device_fn_1_0.create_compute_pipelines)(
            self.handle(),
            pipeline_cache,
            create_infos.len() as u32,
            create_infos.as_ptr(),
            allocation_callbacks.as_raw_ptr(),
            pipelines.as_mut_ptr(),
        );
        pipelines.set_len(create_infos.len());
        match err_code {
            vk::Result::SUCCESS => Ok(pipelines),
            _ => Err((pipelines, err_code)),
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateBuffer.html>
    #[inline]
    pub unsafe fn create_buffer(
        &self,
        create_info: &vk::BufferCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::Buffer> {
        let mut buffer = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.create_buffer)(
            self.handle(),
            create_info,
            allocation_callbacks.as_raw_ptr(),
            buffer.as_mut_ptr(),
        )
        .assume_init_on_success(buffer)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineLayout.html>
    #[inline]
    pub unsafe fn create_pipeline_layout(
        &self,
        create_info: &vk::PipelineLayoutCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::PipelineLayout> {
        let mut pipeline_layout = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.create_pipeline_layout)(
            self.handle(),
            create_info,
            allocation_callbacks.as_raw_ptr(),
            pipeline_layout.as_mut_ptr(),
        )
        .assume_init_on_success(pipeline_layout)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineCache.html>
    #[inline]
    pub unsafe fn create_pipeline_cache(
        &self,
        create_info: &vk::PipelineCacheCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::PipelineCache> {
        let mut pipeline_cache = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.create_pipeline_cache)(
            self.handle(),
            create_info,
            allocation_callbacks.as_raw_ptr(),
            pipeline_cache.as_mut_ptr(),
        )
        .assume_init_on_success(pipeline_cache)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineCacheData.html>
    #[inline]
    pub unsafe fn get_pipeline_cache_data(
        &self,
        pipeline_cache: vk::PipelineCache,
    ) -> VkResult<Vec<u8>> {
        read_into_uninitialized_vector(|count, data: *mut u8| {
            (self.device_fn_1_0.get_pipeline_cache_data)(
                self.handle(),
                pipeline_cache,
                count,
                data.cast(),
            )
        })
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkMergePipelineCaches.html>
    #[inline]
    pub unsafe fn merge_pipeline_caches(
        &self,
        dst_cache: vk::PipelineCache,
        src_caches: &[vk::PipelineCache],
    ) -> VkResult<()> {
        (self.device_fn_1_0.merge_pipeline_caches)(
            self.handle(),
            dst_cache,
            src_caches.len() as u32,
            src_caches.as_ptr(),
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkMapMemory.html>
    #[inline]
    pub unsafe fn map_memory(
        &self,
        memory: vk::DeviceMemory,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
        flags: vk::MemoryMapFlags,
    ) -> VkResult<*mut ffi::c_void> {
        let mut data = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.map_memory)(
            self.handle(),
            memory,
            offset,
            size,
            flags,
            data.as_mut_ptr(),
        )
        .assume_init_on_success(data)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUnmapMemory.html>
    #[inline]
    pub unsafe fn unmap_memory(&self, memory: vk::DeviceMemory) {
        (self.device_fn_1_0.unmap_memory)(self.handle(), memory);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkInvalidateMappedMemoryRanges.html>
    #[inline]
    pub unsafe fn invalidate_mapped_memory_ranges(
        &self,
        ranges: &[vk::MappedMemoryRange<'_>],
    ) -> VkResult<()> {
        (self.device_fn_1_0.invalidate_mapped_memory_ranges)(
            self.handle(),
            ranges.len() as u32,
            ranges.as_ptr(),
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFlushMappedMemoryRanges.html>
    #[inline]
    pub unsafe fn flush_mapped_memory_ranges(
        &self,
        ranges: &[vk::MappedMemoryRange<'_>],
    ) -> VkResult<()> {
        (self.device_fn_1_0.flush_mapped_memory_ranges)(
            self.handle(),
            ranges.len() as u32,
            ranges.as_ptr(),
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateFramebuffer.html>
    #[inline]
    pub unsafe fn create_framebuffer(
        &self,
        create_info: &vk::FramebufferCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::Framebuffer> {
        let mut framebuffer = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.create_framebuffer)(
            self.handle(),
            create_info,
            allocation_callbacks.as_raw_ptr(),
            framebuffer.as_mut_ptr(),
        )
        .assume_init_on_success(framebuffer)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue.html>
    #[inline]
    pub unsafe fn get_device_queue(&self, queue_family_index: u32, queue_index: u32) -> vk::Queue {
        let mut queue = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.get_device_queue)(
            self.handle(),
            queue_family_index,
            queue_index,
            queue.as_mut_ptr(),
        );
        queue.assume_init()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier.html>
    #[inline]
    pub unsafe fn cmd_pipeline_barrier(
        &self,
        command_buffer: vk::CommandBuffer,
        src_stage_mask: vk::PipelineStageFlags,
        dst_stage_mask: vk::PipelineStageFlags,
        dependency_flags: vk::DependencyFlags,
        memory_barriers: &[vk::MemoryBarrier<'_>],
        buffer_memory_barriers: &[vk::BufferMemoryBarrier<'_>],
        image_memory_barriers: &[vk::ImageMemoryBarrier<'_>],
    ) {
        (self.device_fn_1_0.cmd_pipeline_barrier)(
            command_buffer,
            src_stage_mask,
            dst_stage_mask,
            dependency_flags,
            memory_barriers.len() as u32,
            memory_barriers.as_ptr(),
            buffer_memory_barriers.len() as u32,
            buffer_memory_barriers.as_ptr(),
            image_memory_barriers.len() as u32,
            image_memory_barriers.as_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass.html>
    #[inline]
    pub unsafe fn create_render_pass(
        &self,
        create_info: &vk::RenderPassCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::RenderPass> {
        let mut renderpass = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.create_render_pass)(
            self.handle(),
            create_info,
            allocation_callbacks.as_raw_ptr(),
            renderpass.as_mut_ptr(),
        )
        .assume_init_on_success(renderpass)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBeginCommandBuffer.html>
    #[inline]
    pub unsafe fn begin_command_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        begin_info: &vk::CommandBufferBeginInfo<'_>,
    ) -> VkResult<()> {
        (self.device_fn_1_0.begin_command_buffer)(command_buffer, begin_info).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEndCommandBuffer.html>
    #[inline]
    pub unsafe fn end_command_buffer(&self, command_buffer: vk::CommandBuffer) -> VkResult<()> {
        (self.device_fn_1_0.end_command_buffer)(command_buffer).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWaitForFences.html>
    #[inline]
    pub unsafe fn wait_for_fences(
        &self,
        fences: &[vk::Fence],
        wait_all: bool,
        timeout: u64,
    ) -> VkResult<()> {
        (self.device_fn_1_0.wait_for_fences)(
            self.handle(),
            fences.len() as u32,
            fences.as_ptr(),
            wait_all as u32,
            timeout,
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFenceStatus.html>
    #[inline]
    pub unsafe fn get_fence_status(&self, fence: vk::Fence) -> VkResult<bool> {
        let err_code = (self.device_fn_1_0.get_fence_status)(self.handle(), fence);
        match err_code {
            vk::Result::SUCCESS => Ok(true),
            vk::Result::NOT_READY => Ok(false),
            _ => Err(err_code),
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueWaitIdle.html>
    #[inline]
    pub unsafe fn queue_wait_idle(&self, queue: vk::Queue) -> VkResult<()> {
        (self.device_fn_1_0.queue_wait_idle)(queue).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit.html>
    #[inline]
    pub unsafe fn queue_submit(
        &self,
        queue: vk::Queue,
        submits: &[vk::SubmitInfo<'_>],
        fence: vk::Fence,
    ) -> VkResult<()> {
        (self.device_fn_1_0.queue_submit)(queue, submits.len() as u32, submits.as_ptr(), fence)
            .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueBindSparse.html>
    #[inline]
    pub unsafe fn queue_bind_sparse(
        &self,
        queue: vk::Queue,
        bind_info: &[vk::BindSparseInfo<'_>],
        fence: vk::Fence,
    ) -> VkResult<()> {
        (self.device_fn_1_0.queue_bind_sparse)(
            queue,
            bind_info.len() as u32,
            bind_info.as_ptr(),
            fence,
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateBufferView.html>
    #[inline]
    pub unsafe fn create_buffer_view(
        &self,
        create_info: &vk::BufferViewCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::BufferView> {
        let mut buffer_view = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.create_buffer_view)(
            self.handle(),
            create_info,
            allocation_callbacks.as_raw_ptr(),
            buffer_view.as_mut_ptr(),
        )
        .assume_init_on_success(buffer_view)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyBufferView.html>
    #[inline]
    pub unsafe fn destroy_buffer_view(
        &self,
        buffer_view: vk::BufferView,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.device_fn_1_0.destroy_buffer_view)(
            self.handle(),
            buffer_view,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateImageView.html>
    #[inline]
    pub unsafe fn create_image_view(
        &self,
        create_info: &vk::ImageViewCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::ImageView> {
        let mut image_view = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.create_image_view)(
            self.handle(),
            create_info,
            allocation_callbacks.as_raw_ptr(),
            image_view.as_mut_ptr(),
        )
        .assume_init_on_success(image_view)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAllocateCommandBuffers.html>
    #[inline]
    pub unsafe fn allocate_command_buffers(
        &self,
        allocate_info: &vk::CommandBufferAllocateInfo<'_>,
    ) -> VkResult<Vec<vk::CommandBuffer>> {
        let mut buffers = Vec::with_capacity(allocate_info.command_buffer_count as usize);
        (self.device_fn_1_0.allocate_command_buffers)(
            self.handle(),
            allocate_info,
            buffers.as_mut_ptr(),
        )
        .set_vec_len_on_success(buffers, allocate_info.command_buffer_count as usize)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateCommandPool.html>
    #[inline]
    pub unsafe fn create_command_pool(
        &self,
        create_info: &vk::CommandPoolCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::CommandPool> {
        let mut pool = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.create_command_pool)(
            self.handle(),
            create_info,
            allocation_callbacks.as_raw_ptr(),
            pool.as_mut_ptr(),
        )
        .assume_init_on_success(pool)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateQueryPool.html>
    #[inline]
    pub unsafe fn create_query_pool(
        &self,
        create_info: &vk::QueryPoolCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::QueryPool> {
        let mut pool = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.create_query_pool)(
            self.handle(),
            create_info,
            allocation_callbacks.as_raw_ptr(),
            pool.as_mut_ptr(),
        )
        .assume_init_on_success(pool)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateImage.html>
    #[inline]
    pub unsafe fn create_image(
        &self,
        create_info: &vk::ImageCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::Image> {
        let mut image = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.create_image)(
            self.handle(),
            create_info,
            allocation_callbacks.as_raw_ptr(),
            image.as_mut_ptr(),
        )
        .assume_init_on_success(image)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout.html>
    #[inline]
    pub unsafe fn get_image_subresource_layout(
        &self,
        image: vk::Image,
        subresource: vk::ImageSubresource,
    ) -> vk::SubresourceLayout {
        let mut layout = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.get_image_subresource_layout)(
            self.handle(),
            image,
            &subresource,
            layout.as_mut_ptr(),
        );
        layout.assume_init()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements.html>
    #[inline]
    pub unsafe fn get_image_memory_requirements(&self, image: vk::Image) -> vk::MemoryRequirements {
        let mut mem_req = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.get_image_memory_requirements)(
            self.handle(),
            image,
            mem_req.as_mut_ptr(),
        );
        mem_req.assume_init()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements.html>
    #[inline]
    pub unsafe fn get_buffer_memory_requirements(
        &self,
        buffer: vk::Buffer,
    ) -> vk::MemoryRequirements {
        let mut mem_req = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.get_buffer_memory_requirements)(
            self.handle(),
            buffer,
            mem_req.as_mut_ptr(),
        );
        mem_req.assume_init()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAllocateMemory.html>
    #[inline]
    pub unsafe fn allocate_memory(
        &self,
        allocate_info: &vk::MemoryAllocateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::DeviceMemory> {
        let mut memory = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.allocate_memory)(
            self.handle(),
            allocate_info,
            allocation_callbacks.as_raw_ptr(),
            memory.as_mut_ptr(),
        )
        .assume_init_on_success(memory)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateShaderModule.html>
    #[inline]
    pub unsafe fn create_shader_module(
        &self,
        create_info: &vk::ShaderModuleCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::ShaderModule> {
        let mut shader = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.create_shader_module)(
            self.handle(),
            create_info,
            allocation_callbacks.as_raw_ptr(),
            shader.as_mut_ptr(),
        )
        .assume_init_on_success(shader)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateFence.html>
    #[inline]
    pub unsafe fn create_fence(
        &self,
        create_info: &vk::FenceCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::Fence> {
        let mut fence = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.create_fence)(
            self.handle(),
            create_info,
            allocation_callbacks.as_raw_ptr(),
            fence.as_mut_ptr(),
        )
        .assume_init_on_success(fence)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory.html>
    #[inline]
    pub unsafe fn bind_buffer_memory(
        &self,
        buffer: vk::Buffer,
        device_memory: vk::DeviceMemory,
        offset: vk::DeviceSize,
    ) -> VkResult<()> {
        (self.device_fn_1_0.bind_buffer_memory)(self.handle(), buffer, device_memory, offset)
            .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory.html>
    #[inline]
    pub unsafe fn bind_image_memory(
        &self,
        image: vk::Image,
        device_memory: vk::DeviceMemory,
        offset: vk::DeviceSize,
    ) -> VkResult<()> {
        (self.device_fn_1_0.bind_image_memory)(self.handle(), image, device_memory, offset).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRenderAreaGranularity.html>
    #[inline]
    pub unsafe fn get_render_area_granularity(&self, render_pass: vk::RenderPass) -> vk::Extent2D {
        let mut granularity = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.get_render_area_granularity)(
            self.handle(),
            render_pass,
            granularity.as_mut_ptr(),
        );
        granularity.assume_init()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryCommitment.html>
    #[inline]
    pub unsafe fn get_device_memory_commitment(&self, memory: vk::DeviceMemory) -> vk::DeviceSize {
        let mut committed_memory_in_bytes = mem::MaybeUninit::uninit();
        (self.device_fn_1_0.get_device_memory_commitment)(
            self.handle(),
            memory,
            committed_memory_in_bytes.as_mut_ptr(),
        );
        committed_memory_in_bytes.assume_init()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements.html>
    #[inline]
    pub unsafe fn get_image_sparse_memory_requirements(
        &self,
        image: vk::Image,
    ) -> Vec<vk::SparseImageMemoryRequirements> {
        read_into_uninitialized_vector(|count, data| {
            (self.device_fn_1_0.get_image_sparse_memory_requirements)(
                self.handle(),
                image,
                count,
                data,
            );
            vk::Result::SUCCESS
        })
        // The closure always returns SUCCESS
        .unwrap()
    }
}
