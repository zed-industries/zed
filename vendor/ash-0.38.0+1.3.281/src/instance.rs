#[cfg(doc)]
use super::Entry;
use crate::device::Device;
use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use alloc::vec::Vec;
use core::ffi;
use core::mem;
use core::ptr;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstance.html>
#[derive(Clone)]
pub struct Instance {
    pub(crate) handle: vk::Instance,

    pub(crate) instance_fn_1_0: crate::InstanceFnV1_0,
    pub(crate) instance_fn_1_1: crate::InstanceFnV1_1,
    pub(crate) instance_fn_1_3: crate::InstanceFnV1_3,
}

impl Instance {
    pub unsafe fn load(static_fn: &crate::StaticFn, instance: vk::Instance) -> Self {
        Self::load_with(
            |name| mem::transmute((static_fn.get_instance_proc_addr)(instance, name.as_ptr())),
            instance,
        )
    }

    pub unsafe fn load_with(
        mut load_fn: impl FnMut(&ffi::CStr) -> *const ffi::c_void,
        instance: vk::Instance,
    ) -> Self {
        Self::from_parts_1_3(
            instance,
            crate::InstanceFnV1_0::load(&mut load_fn),
            crate::InstanceFnV1_1::load(&mut load_fn),
            crate::InstanceFnV1_3::load(&mut load_fn),
        )
    }

    #[inline]
    pub fn from_parts_1_3(
        handle: vk::Instance,
        instance_fn_1_0: crate::InstanceFnV1_0,
        instance_fn_1_1: crate::InstanceFnV1_1,
        instance_fn_1_3: crate::InstanceFnV1_3,
    ) -> Self {
        Self {
            handle,

            instance_fn_1_0,
            instance_fn_1_1,
            instance_fn_1_3,
        }
    }

    #[inline]
    pub fn handle(&self) -> vk::Instance {
        self.handle
    }
}

/// Vulkan core 1.3
impl Instance {
    #[inline]
    pub fn fp_v1_3(&self) -> &crate::InstanceFnV1_3 {
        &self.instance_fn_1_3
    }

    /// Retrieve the number of elements to pass to [`get_physical_device_tool_properties()`][Self::get_physical_device_tool_properties()]
    #[inline]
    pub unsafe fn get_physical_device_tool_properties_len(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> VkResult<usize> {
        let mut count = mem::MaybeUninit::uninit();
        (self.instance_fn_1_3.get_physical_device_tool_properties)(
            physical_device,
            count.as_mut_ptr(),
            ptr::null_mut(),
        )
        .assume_init_on_success(count)
        .map(|c| c as usize)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceToolProperties.html>
    ///
    /// Call [`get_physical_device_tool_properties_len()`][Self::get_physical_device_tool_properties_len()] to query the number of elements to pass to `out`.
    /// Be sure to [`Default::default()`]-initialize these elements and optionally set their `p_next` pointer.
    #[inline]
    pub unsafe fn get_physical_device_tool_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        out: &mut [vk::PhysicalDeviceToolProperties<'_>],
    ) -> VkResult<()> {
        let mut count = out.len() as u32;
        (self.instance_fn_1_3.get_physical_device_tool_properties)(
            physical_device,
            &mut count,
            out.as_mut_ptr(),
        )
        .result()?;
        assert_eq!(count as usize, out.len());
        Ok(())
    }
}

/// Vulkan core 1.1
impl Instance {
    #[inline]
    pub fn fp_v1_1(&self) -> &crate::InstanceFnV1_1 {
        &self.instance_fn_1_1
    }

    /// Retrieve the number of elements to pass to [`enumerate_physical_device_groups()`][Self::enumerate_physical_device_groups()]
    #[inline]
    pub unsafe fn enumerate_physical_device_groups_len(&self) -> VkResult<usize> {
        let mut group_count = mem::MaybeUninit::uninit();
        (self.instance_fn_1_1.enumerate_physical_device_groups)(
            self.handle(),
            group_count.as_mut_ptr(),
            ptr::null_mut(),
        )
        .assume_init_on_success(group_count)
        .map(|c| c as usize)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceGroups.html>
    ///
    /// Call [`enumerate_physical_device_groups_len()`][Self::enumerate_physical_device_groups_len()] to query the number of elements to pass to `out`.
    /// Be sure to [`Default::default()`]-initialize these elements and optionally set their `p_next` pointer.
    #[inline]
    pub unsafe fn enumerate_physical_device_groups(
        &self,
        out: &mut [vk::PhysicalDeviceGroupProperties<'_>],
    ) -> VkResult<()> {
        let mut count = out.len() as u32;
        (self.instance_fn_1_1.enumerate_physical_device_groups)(
            self.handle(),
            &mut count,
            out.as_mut_ptr(),
        )
        .result()?;
        assert_eq!(count as usize, out.len());
        Ok(())
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures2.html>
    #[inline]
    pub unsafe fn get_physical_device_features2(
        &self,
        physical_device: vk::PhysicalDevice,
        features: &mut vk::PhysicalDeviceFeatures2<'_>,
    ) {
        (self.instance_fn_1_1.get_physical_device_features2)(physical_device, features);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties2.html>
    #[inline]
    pub unsafe fn get_physical_device_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        prop: &mut vk::PhysicalDeviceProperties2<'_>,
    ) {
        (self.instance_fn_1_1.get_physical_device_properties2)(physical_device, prop);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties2.html>
    #[inline]
    pub unsafe fn get_physical_device_format_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        out: &mut vk::FormatProperties2<'_>,
    ) {
        (self.instance_fn_1_1.get_physical_device_format_properties2)(physical_device, format, out);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2.html>
    #[inline]
    pub unsafe fn get_physical_device_image_format_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        format_info: &vk::PhysicalDeviceImageFormatInfo2<'_>,
        image_format_prop: &mut vk::ImageFormatProperties2<'_>,
    ) -> VkResult<()> {
        (self
            .instance_fn_1_1
            .get_physical_device_image_format_properties2)(
            physical_device,
            format_info,
            image_format_prop,
        )
        .result()
    }

    /// Retrieve the number of elements to pass to [`get_physical_device_queue_family_properties2()`][Self::get_physical_device_queue_family_properties2()]
    #[inline]
    pub unsafe fn get_physical_device_queue_family_properties2_len(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> usize {
        let mut queue_count = mem::MaybeUninit::uninit();
        (self
            .instance_fn_1_1
            .get_physical_device_queue_family_properties2)(
            physical_device,
            queue_count.as_mut_ptr(),
            ptr::null_mut(),
        );
        queue_count.assume_init() as usize
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html>
    ///
    /// Call [`get_physical_device_queue_family_properties2_len()`][Self::get_physical_device_queue_family_properties2_len()] to query the number of elements to pass to `out`.
    /// Be sure to [`Default::default()`]-initialize these elements and optionally set their `p_next` pointer.
    #[inline]
    pub unsafe fn get_physical_device_queue_family_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        out: &mut [vk::QueueFamilyProperties2<'_>],
    ) {
        let mut count = out.len() as u32;
        (self
            .instance_fn_1_1
            .get_physical_device_queue_family_properties2)(
            physical_device,
            &mut count,
            out.as_mut_ptr(),
        );
        assert_eq!(count as usize, out.len());
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2.html>
    #[inline]
    pub unsafe fn get_physical_device_memory_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        out: &mut vk::PhysicalDeviceMemoryProperties2<'_>,
    ) {
        (self.instance_fn_1_1.get_physical_device_memory_properties2)(physical_device, out);
    }

    /// Retrieve the number of elements to pass to [`get_physical_device_sparse_image_format_properties2()`][Self::get_physical_device_sparse_image_format_properties2()]
    #[inline]
    pub unsafe fn get_physical_device_sparse_image_format_properties2_len(
        &self,
        physical_device: vk::PhysicalDevice,
        format_info: &vk::PhysicalDeviceSparseImageFormatInfo2<'_>,
    ) -> usize {
        let mut format_count = mem::MaybeUninit::uninit();
        (self
            .instance_fn_1_1
            .get_physical_device_sparse_image_format_properties2)(
            physical_device,
            format_info,
            format_count.as_mut_ptr(),
            ptr::null_mut(),
        );
        format_count.assume_init() as usize
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2.html>
    ///
    /// Call [`get_physical_device_sparse_image_format_properties2_len()`][Self::get_physical_device_sparse_image_format_properties2_len()] to query the number of elements to pass to `out`.
    /// Be sure to [`Default::default()`]-initialize these elements and optionally set their `p_next` pointer.
    #[inline]
    pub unsafe fn get_physical_device_sparse_image_format_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        format_info: &vk::PhysicalDeviceSparseImageFormatInfo2<'_>,
        out: &mut [vk::SparseImageFormatProperties2<'_>],
    ) {
        let mut count = out.len() as u32;
        (self
            .instance_fn_1_1
            .get_physical_device_sparse_image_format_properties2)(
            physical_device,
            format_info,
            &mut count,
            out.as_mut_ptr(),
        );
        assert_eq!(count as usize, out.len());
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalBufferProperties.html>
    #[inline]
    pub unsafe fn get_physical_device_external_buffer_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        external_buffer_info: &vk::PhysicalDeviceExternalBufferInfo<'_>,
        out: &mut vk::ExternalBufferProperties<'_>,
    ) {
        (self
            .instance_fn_1_1
            .get_physical_device_external_buffer_properties)(
            physical_device,
            external_buffer_info,
            out,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalFenceProperties.html>
    #[inline]
    pub unsafe fn get_physical_device_external_fence_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        external_fence_info: &vk::PhysicalDeviceExternalFenceInfo<'_>,
        out: &mut vk::ExternalFenceProperties<'_>,
    ) {
        (self
            .instance_fn_1_1
            .get_physical_device_external_fence_properties)(
            physical_device,
            external_fence_info,
            out,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalSemaphoreProperties.html>
    #[inline]
    pub unsafe fn get_physical_device_external_semaphore_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        external_semaphore_info: &vk::PhysicalDeviceExternalSemaphoreInfo<'_>,
        out: &mut vk::ExternalSemaphoreProperties<'_>,
    ) {
        (self
            .instance_fn_1_1
            .get_physical_device_external_semaphore_properties)(
            physical_device,
            external_semaphore_info,
            out,
        );
    }
}

/// Vulkan core 1.0
impl Instance {
    #[inline]
    pub fn fp_v1_0(&self) -> &crate::InstanceFnV1_0 {
        &self.instance_fn_1_0
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDevice.html>
    ///
    /// # Safety
    ///
    /// There is a [parent/child relation] between [`Instance`] and the resulting [`Device`].  The
    /// application must not [destroy][Instance::destroy_instance()] the parent [`Instance`] object
    /// before first [destroying][Device::destroy_device()] the returned [`Device`] child object.
    /// [`Device`] does _not_ implement [drop][drop()] semantics and can only be destroyed via
    /// [`destroy_device()`][Device::destroy_device()].
    ///
    /// See the [`Entry::create_instance()`] documentation for more destruction ordering rules on
    /// [`Instance`].
    ///
    /// [parent/child relation]: https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#fundamentals-objectmodel-lifetime
    #[inline]
    pub unsafe fn create_device(
        &self,
        physical_device: vk::PhysicalDevice,
        create_info: &vk::DeviceCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<Device> {
        let mut device = mem::MaybeUninit::uninit();
        let device = (self.instance_fn_1_0.create_device)(
            physical_device,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            device.as_mut_ptr(),
        )
        .assume_init_on_success(device)?;
        Ok(Device::load(&self.instance_fn_1_0, device))
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceProcAddr.html>
    #[inline]
    pub unsafe fn get_device_proc_addr(
        &self,
        device: vk::Device,
        p_name: *const ffi::c_char,
    ) -> vk::PFN_vkVoidFunction {
        (self.instance_fn_1_0.get_device_proc_addr)(device, p_name)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyInstance.html>
    #[inline]
    pub unsafe fn destroy_instance(
        &self,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.instance_fn_1_0.destroy_instance)(self.handle(), allocation_callbacks.as_raw_ptr());
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties.html>
    #[inline]
    pub unsafe fn get_physical_device_format_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
    ) -> vk::FormatProperties {
        let mut format_prop = mem::MaybeUninit::uninit();
        (self.instance_fn_1_0.get_physical_device_format_properties)(
            physical_device,
            format,
            format_prop.as_mut_ptr(),
        );
        format_prop.assume_init()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties.html>
    #[inline]
    pub unsafe fn get_physical_device_image_format_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        typ: vk::ImageType,
        tiling: vk::ImageTiling,
        usage: vk::ImageUsageFlags,
        flags: vk::ImageCreateFlags,
    ) -> VkResult<vk::ImageFormatProperties> {
        let mut image_format_prop = mem::MaybeUninit::uninit();
        (self
            .instance_fn_1_0
            .get_physical_device_image_format_properties)(
            physical_device,
            format,
            typ,
            tiling,
            usage,
            flags,
            image_format_prop.as_mut_ptr(),
        )
        .assume_init_on_success(image_format_prop)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties.html>
    #[inline]
    pub unsafe fn get_physical_device_memory_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceMemoryProperties {
        let mut memory_prop = mem::MaybeUninit::uninit();
        (self.instance_fn_1_0.get_physical_device_memory_properties)(
            physical_device,
            memory_prop.as_mut_ptr(),
        );
        memory_prop.assume_init()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties.html>
    #[inline]
    pub unsafe fn get_physical_device_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceProperties {
        let mut prop = mem::MaybeUninit::uninit();
        (self.instance_fn_1_0.get_physical_device_properties)(physical_device, prop.as_mut_ptr());
        prop.assume_init()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties.html>
    #[inline]
    pub unsafe fn get_physical_device_queue_family_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Vec<vk::QueueFamilyProperties> {
        read_into_uninitialized_vector(|count, data| {
            (self
                .instance_fn_1_0
                .get_physical_device_queue_family_properties)(
                physical_device, count, data
            );
            vk::Result::SUCCESS
        })
        // The closure always returns SUCCESS
        .unwrap()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures.html>
    #[inline]
    pub unsafe fn get_physical_device_features(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceFeatures {
        let mut prop = mem::MaybeUninit::uninit();
        (self.instance_fn_1_0.get_physical_device_features)(physical_device, prop.as_mut_ptr());
        prop.assume_init()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDevices.html>
    #[inline]
    pub unsafe fn enumerate_physical_devices(&self) -> VkResult<Vec<vk::PhysicalDevice>> {
        read_into_uninitialized_vector(|count, data| {
            (self.instance_fn_1_0.enumerate_physical_devices)(self.handle(), count, data)
        })
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceExtensionProperties.html>
    #[inline]
    pub unsafe fn enumerate_device_extension_properties(
        &self,
        device: vk::PhysicalDevice,
    ) -> VkResult<Vec<vk::ExtensionProperties>> {
        read_into_uninitialized_vector(|count, data| {
            (self.instance_fn_1_0.enumerate_device_extension_properties)(
                device,
                ptr::null(),
                count,
                data,
            )
        })
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceLayerProperties.html>
    #[inline]
    pub unsafe fn enumerate_device_layer_properties(
        &self,
        device: vk::PhysicalDevice,
    ) -> VkResult<Vec<vk::LayerProperties>> {
        read_into_uninitialized_vector(|count, data| {
            (self.instance_fn_1_0.enumerate_device_layer_properties)(device, count, data)
        })
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties.html>
    #[inline]
    pub unsafe fn get_physical_device_sparse_image_format_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        typ: vk::ImageType,
        samples: vk::SampleCountFlags,
        usage: vk::ImageUsageFlags,
        tiling: vk::ImageTiling,
    ) -> Vec<vk::SparseImageFormatProperties> {
        read_into_uninitialized_vector(|count, data| {
            (self
                .instance_fn_1_0
                .get_physical_device_sparse_image_format_properties)(
                physical_device,
                format,
                typ,
                samples,
                usage,
                tiling,
                count,
                data,
            );
            vk::Result::SUCCESS
        })
        // The closure always returns SUCCESS
        .unwrap()
    }
}
