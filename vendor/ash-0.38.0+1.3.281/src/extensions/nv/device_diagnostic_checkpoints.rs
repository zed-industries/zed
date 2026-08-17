//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_device_diagnostic_checkpoints.html>

use crate::vk;
use core::ffi;
use core::mem;
use core::ptr;

impl crate::nv::device_diagnostic_checkpoints::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCheckpointNV.html>
    #[inline]
    pub unsafe fn cmd_set_checkpoint(
        &self,
        command_buffer: vk::CommandBuffer,
        p_checkpoint_marker: *const ffi::c_void,
    ) {
        (self.fp.cmd_set_checkpoint_nv)(command_buffer, p_checkpoint_marker);
    }

    /// Retrieve the number of elements to pass to [`get_queue_checkpoint_data()`][Self::get_queue_checkpoint_data()]
    #[inline]
    pub unsafe fn get_queue_checkpoint_data_len(&self, queue: vk::Queue) -> usize {
        let mut count = mem::MaybeUninit::uninit();
        (self.fp.get_queue_checkpoint_data_nv)(queue, count.as_mut_ptr(), ptr::null_mut());
        count.assume_init() as usize
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointDataNV.html>
    ///
    /// Call [`get_queue_checkpoint_data_len()`][Self::get_queue_checkpoint_data_len()] to query the number of elements to pass to `out`.
    /// Be sure to [`Default::default()`]-initialize these elements and optionally set their `p_next` pointer.
    #[inline]
    pub unsafe fn get_queue_checkpoint_data(
        &self,
        queue: vk::Queue,
        out: &mut [vk::CheckpointDataNV<'_>],
    ) {
        let mut count = out.len() as u32;
        (self.fp.get_queue_checkpoint_data_nv)(queue, &mut count, out.as_mut_ptr());
        assert_eq!(count as usize, out.len());
    }
}
