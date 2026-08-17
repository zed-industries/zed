//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_sample_locations.html>

use crate::vk;

impl crate::ext::sample_locations::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEXT.html>
    #[inline]
    pub unsafe fn cmd_set_sample_locations(
        &self,
        command_buffer: vk::CommandBuffer,
        sample_locations_info: &vk::SampleLocationsInfoEXT<'_>,
    ) {
        (self.fp.cmd_set_sample_locations_ext)(command_buffer, sample_locations_info)
    }
}

impl crate::ext::sample_locations::Instance {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html>
    #[inline]
    pub unsafe fn get_physical_device_multisample_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        samples: vk::SampleCountFlags,
        multisample_properties: &mut vk::MultisamplePropertiesEXT<'_>,
    ) {
        (self.fp.get_physical_device_multisample_properties_ext)(
            physical_device,
            samples,
            multisample_properties,
        )
    }
}
