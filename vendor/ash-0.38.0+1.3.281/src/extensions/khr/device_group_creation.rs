//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_device_group_creation.html>

use crate::prelude::*;
use crate::vk;
use core::mem;
use core::ptr;

impl crate::khr::device_group_creation::Instance {
    /// Retrieve the number of elements to pass to [`enumerate_physical_device_groups()`][Self::enumerate_physical_device_groups()]
    #[inline]
    pub unsafe fn enumerate_physical_device_groups_len(&self) -> VkResult<usize> {
        let mut group_count = mem::MaybeUninit::uninit();
        (self.fp.enumerate_physical_device_groups_khr)(
            self.handle,
            group_count.as_mut_ptr(),
            ptr::null_mut(),
        )
        .assume_init_on_success(group_count)
        .map(|c| c as usize)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceGroupsKHR.html>
    ///
    /// Call [`enumerate_physical_device_groups_len()`][Self::enumerate_physical_device_groups_len()] to query the number of elements to pass to `out`.
    /// Be sure to [`Default::default()`]-initialize these elements and optionally set their `p_next` pointer.
    #[inline]
    pub unsafe fn enumerate_physical_device_groups(
        &self,
        out: &mut [vk::PhysicalDeviceGroupProperties<'_>],
    ) -> VkResult<()> {
        let mut count = out.len() as u32;
        (self.fp.enumerate_physical_device_groups_khr)(self.handle, &mut count, out.as_mut_ptr())
            .result()?;
        assert_eq!(count as usize, out.len());
        Ok(())
    }
}
