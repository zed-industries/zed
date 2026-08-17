//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_report.html>

use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use core::mem;

impl crate::ext::debug_report::Instance {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugReportCallbackEXT.html>
    #[inline]
    pub unsafe fn destroy_debug_report_callback(
        &self,
        debug: vk::DebugReportCallbackEXT,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.fp.destroy_debug_report_callback_ext)(
            self.handle,
            debug,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDebugReportCallbackEXT.html>
    #[inline]
    pub unsafe fn create_debug_report_callback(
        &self,
        create_info: &vk::DebugReportCallbackCreateInfoEXT<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::DebugReportCallbackEXT> {
        let mut debug_cb = mem::MaybeUninit::uninit();
        (self.fp.create_debug_report_callback_ext)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            debug_cb.as_mut_ptr(),
        )
        .assume_init_on_success(debug_cb)
    }
}
