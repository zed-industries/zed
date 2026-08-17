//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html>

use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use core::mem;

impl crate::ext::debug_utils::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectNameEXT.html>
    #[inline]
    pub unsafe fn set_debug_utils_object_name(
        &self,
        name_info: &vk::DebugUtilsObjectNameInfoEXT<'_>,
    ) -> VkResult<()> {
        (self.fp.set_debug_utils_object_name_ext)(self.handle, name_info).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectTagEXT.html>
    #[inline]
    pub unsafe fn set_debug_utils_object_tag(
        &self,
        tag_info: &vk::DebugUtilsObjectTagInfoEXT<'_>,
    ) -> VkResult<()> {
        (self.fp.set_debug_utils_object_tag_ext)(self.handle, tag_info).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginDebugUtilsLabelEXT.html>
    #[inline]
    pub unsafe fn cmd_begin_debug_utils_label(
        &self,
        command_buffer: vk::CommandBuffer,
        label: &vk::DebugUtilsLabelEXT<'_>,
    ) {
        (self.fp.cmd_begin_debug_utils_label_ext)(command_buffer, label);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndDebugUtilsLabelEXT.html>
    #[inline]
    pub unsafe fn cmd_end_debug_utils_label(&self, command_buffer: vk::CommandBuffer) {
        (self.fp.cmd_end_debug_utils_label_ext)(command_buffer);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdInsertDebugUtilsLabelEXT.html>
    #[inline]
    pub unsafe fn cmd_insert_debug_utils_label(
        &self,
        command_buffer: vk::CommandBuffer,
        label: &vk::DebugUtilsLabelEXT<'_>,
    ) {
        (self.fp.cmd_insert_debug_utils_label_ext)(command_buffer, label);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueBeginDebugUtilsLabelEXT.html>
    #[inline]
    pub unsafe fn queue_begin_debug_utils_label(
        &self,
        queue: vk::Queue,
        label: &vk::DebugUtilsLabelEXT<'_>,
    ) {
        (self.fp.queue_begin_debug_utils_label_ext)(queue, label);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueEndDebugUtilsLabelEXT.html>
    #[inline]
    pub unsafe fn queue_end_debug_utils_label(&self, queue: vk::Queue) {
        (self.fp.queue_end_debug_utils_label_ext)(queue);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueInsertDebugUtilsLabelEXT.html>
    #[inline]
    pub unsafe fn queue_insert_debug_utils_label(
        &self,
        queue: vk::Queue,
        label: &vk::DebugUtilsLabelEXT<'_>,
    ) {
        (self.fp.queue_insert_debug_utils_label_ext)(queue, label);
    }
}

impl crate::ext::debug_utils::Instance {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html>
    #[inline]
    pub unsafe fn create_debug_utils_messenger(
        &self,
        create_info: &vk::DebugUtilsMessengerCreateInfoEXT<'_>,
        allocator: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::DebugUtilsMessengerEXT> {
        let mut messenger = mem::MaybeUninit::uninit();
        (self.fp.create_debug_utils_messenger_ext)(
            self.handle,
            create_info,
            allocator.as_raw_ptr(),
            messenger.as_mut_ptr(),
        )
        .assume_init_on_success(messenger)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html>
    #[inline]
    pub unsafe fn destroy_debug_utils_messenger(
        &self,
        messenger: vk::DebugUtilsMessengerEXT,
        allocator: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.fp.destroy_debug_utils_messenger_ext)(self.handle, messenger, allocator.as_raw_ptr());
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSubmitDebugUtilsMessageEXT.html>
    #[inline]
    pub unsafe fn submit_debug_utils_message(
        &self,
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
        message_types: vk::DebugUtilsMessageTypeFlagsEXT,
        callback_data: &vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    ) {
        (self.fp.submit_debug_utils_message_ext)(
            self.handle,
            message_severity,
            message_types,
            callback_data,
        );
    }
}
