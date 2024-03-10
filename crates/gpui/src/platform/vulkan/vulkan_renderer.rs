use std::ffi::CStr;
use std::os::raw::c_void;
use std::sync::Arc;

use ash::extensions::ext::DebugUtils;
use ash::vk;
use itertools::Itertools;

use crate::Pixels;
use crate::Scene;
use crate::Size;

use super::VulkanAtlas;
use super::VulkanSurface;

unsafe extern "system" fn validation_layer_callback(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut c_void,
) -> vk::Bool32 {
    match severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => {
            log::trace!(
                "Vulkan Validation (verbose, {:?}): {}",
                message_type,
                CStr::from_ptr((*callback_data).p_message).to_string_lossy()
            )
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO => {
            log::info!(
                "Vulkan Validation (info, {:?}): {}",
                message_type,
                CStr::from_ptr((*callback_data).p_message).to_string_lossy()
            )
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => {
            log::warn!(
                "Vulkan Validation (warning, {:?}): {}",
                message_type,
                CStr::from_ptr((*callback_data).p_message).to_string_lossy()
            )
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => {
            log::error!(
                "Vulkan Validation (error, {:?}): {}",
                message_type,
                CStr::from_ptr((*callback_data).p_message).to_string_lossy()
            )
        }
        _ => unreachable!(),
    }

    vk::FALSE
}

pub(crate) struct VulkanRenderer {
    instance: ash::Instance,
    surface: vk::SurfaceKHR,
    debug_callback: vk::DebugUtilsMessengerEXT,
    device: ash::Device,
    swapchain: vk::SwapchainKHR,
    swapchain_image_views: Vec<vk::ImageView>,
    atlas: Arc<VulkanAtlas>,

    debug_utils_loader: DebugUtils,
    surface_loader: ash::extensions::khr::Surface,
    swapchain_loader: ash::extensions::khr::Swapchain,

    viewport_size: Size<Pixels>,
}

impl VulkanRenderer {
    pub fn new(surface_data: Box<dyn VulkanSurface>, viewport_size: Size<Pixels>) -> Self {
        // Entry point
        let entry = ash::Entry::linked();

        // Check instance extension support
        let required_instance_extensions = [
            DebugUtils::name(),
            surface_data.extension_name(),
            ash::extensions::khr::Surface::name(),
        ];
        let available_instance_extensions =
            entry.enumerate_instance_extension_properties(None).unwrap();
        for required in required_instance_extensions {
            if !available_instance_extensions
                .iter()
                .any(|available| unsafe {CStr::from_ptr(available.extension_name.as_ptr())} == required)
            {
                panic!("Missing vulkan extension: {:?}", required);
            }
        }

        // Check validation layer support
        let required_validation_layers =
            [CStr::from_bytes_with_nul(b"VK_LAYER_KHRONOS_validation\0").unwrap()];
        let available_validation_layers = entry.enumerate_instance_layer_properties().unwrap();
        for required in required_validation_layers {
            if !available_validation_layers.iter().any(
                |available| unsafe { CStr::from_ptr(available.layer_name.as_ptr()) } == required,
            ) {
                panic!("Missing vulkan layer: {:?}", required);
            }
        }

        // Create instance
        let app_info = vk::ApplicationInfo::builder()
            .engine_name(CStr::from_bytes_with_nul(b"GPUI\0").unwrap())
            .engine_version(vk::make_api_version(
                0,
                env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap(),
                env!("CARGO_PKG_VERSION_MINOR").parse::<u32>().unwrap(),
                env!("CARGO_PKG_VERSION_PATCH").parse::<u32>().unwrap(),
            ))
            .api_version(vk::API_VERSION_1_3)
            .build();

        let instance_create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&required_instance_extensions.map(|name| name.as_ptr()))
            .enabled_layer_names(&required_validation_layers.map(|name| name.as_ptr()))
            .build();

        let instance = unsafe { entry.create_instance(&instance_create_info, None) }.unwrap();

        // Load extensions
        let platform_surface_loader = surface_data.extension_loader(&entry, &instance);
        let surface_loader = ash::extensions::khr::Surface::new(&entry, &instance);
        let debug_utils_loader = DebugUtils::new(&entry, &instance);

        // Validation callback
        let debug_messenger_create_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                    | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
            )
            .pfn_user_callback(Some(validation_layer_callback));

        let debug_callback = unsafe {
            debug_utils_loader.create_debug_utils_messenger(&debug_messenger_create_info, None)
        }
        .unwrap();

        // Physical device
        let required_extensions = [ash::extensions::khr::Swapchain::name().as_ptr()];

        let physical_device = unsafe { instance.enumerate_physical_devices() }
            .unwrap()
            .into_iter()
            .filter(|device| {
                let queue_family_props =
                    unsafe { instance.get_physical_device_queue_family_properties(*device) };

                queue_family_props.iter().enumerate().any(|(idx, props)| {
                    props.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                        && surface_data.presentation_support(
                            &platform_surface_loader,
                            *device,
                            idx as u32,
                        )
                })
            })
            .find(|device| {
                let extensions =
                    unsafe { instance.enumerate_device_extension_properties(*device) }.unwrap();
                required_extensions.iter().all(|required| {
                    extensions.iter().any(|extension| unsafe {
                        CStr::from_ptr(extension.extension_name.as_ptr())
                            == CStr::from_ptr(*required)
                    })
                })
            })
            .expect("No physical device with vulkan support found");
        let memory_properties =
            unsafe { instance.get_physical_device_memory_properties(physical_device) };

        // Get queue family index
        let graphics_family =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) }
                .iter()
                .enumerate()
                .filter(|(idx, queue_family)| {
                    queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                        && surface_data.presentation_support(
                            &platform_surface_loader,
                            physical_device,
                            *idx as u32,
                        )
                })
                .max_by_key(|(_, queue_family)| queue_family.queue_count)
                .unwrap()
                .0 as u32;

        // Device
        let queue_create_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(graphics_family)
            .queue_priorities(&[1.0])
            .build();

        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&[queue_create_info])
            .enabled_extension_names(&required_extensions)
            .build();

        let device =
            unsafe { instance.create_device(physical_device, &device_create_info, None) }.unwrap();

        // Surface
        let surface = surface_data.create_surface(&platform_surface_loader);

        // Swapchain
        let surface_capabilities = unsafe {
            surface_loader.get_physical_device_surface_capabilities(physical_device, surface)
        }
        .unwrap();
        let surface_format =
            unsafe { surface_loader.get_physical_device_surface_formats(physical_device, surface) }
                .unwrap()
                .into_iter()
                .find(|format| {
                    format.format == vk::Format::B8G8R8A8_UNORM
                        && [vk::ColorSpaceKHR::SRGB_NONLINEAR].contains(&format.color_space)
                })
                .unwrap_or_else(|| todo!("vulkan: support more surface formats"));
        let present_mode = unsafe {
            surface_loader.get_physical_device_surface_present_modes(physical_device, surface)
        }
        .unwrap()
        .into_iter()
        .sorted_by_key(|present_mode| {
            [
                vk::PresentModeKHR::MAILBOX,
                vk::PresentModeKHR::FIFO,
                vk::PresentModeKHR::FIFO_RELAXED,
            ]
            .iter()
            .find_position(|target| &present_mode == target)
            .map(|(idx, _)| idx)
            .unwrap_or(100)
        })
        .next()
        .unwrap();
        let viewport_size: Size<Pixels> = Size {
            width: viewport_size.width.clamp(
                surface_capabilities.min_image_extent.width.into(),
                surface_capabilities.max_image_extent.width.into(),
            ),
            height: viewport_size.height.clamp(
                surface_capabilities.min_image_extent.height.into(),
                surface_capabilities.max_image_extent.height.into(),
            ),
        };

        let swapchain_create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(surface)
            .min_image_count(
                (surface_capabilities.min_image_count + 1)
                    .min(surface_capabilities.max_image_count),
            )
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(
                vk::Extent2D::builder()
                    .width(viewport_size.width.into())
                    .height(viewport_size.height.into())
                    .build(),
            )
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .queue_family_indices(&[graphics_family])
            .pre_transform(surface_capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .build();
        let swapchain_loader = ash::extensions::khr::Swapchain::new(&instance, &device);
        let swapchain =
            unsafe { swapchain_loader.create_swapchain(&swapchain_create_info, None) }.unwrap();
        let swapchain_images = unsafe { swapchain_loader.get_swapchain_images(swapchain) }.unwrap();
        let swapchain_image_views = swapchain_images
            .iter()
            .map(|swapchain_image| {
                let create_info = vk::ImageViewCreateInfo::builder()
                    .image(*swapchain_image)
                    .view_type(vk::ImageViewType::TYPE_2D)
                    .format(surface_format.format)
                    .components(
                        vk::ComponentMapping::builder()
                            .r(vk::ComponentSwizzle::IDENTITY)
                            .g(vk::ComponentSwizzle::IDENTITY)
                            .b(vk::ComponentSwizzle::IDENTITY)
                            .a(vk::ComponentSwizzle::A)
                            .build(),
                    )
                    .subresource_range(
                        vk::ImageSubresourceRange::builder()
                            .aspect_mask(vk::ImageAspectFlags::COLOR)
                            .base_mip_level(0)
                            .level_count(1)
                            .base_array_layer(0)
                            .layer_count(1)
                            .build(),
                    )
                    .build();
                unsafe { device.create_image_view(&create_info, None) }.unwrap()
            })
            .collect_vec();

        // Atlas
        let atlas = Arc::new(VulkanAtlas::new(device.clone(), memory_properties));

        Self {
            instance,
            surface,
            debug_callback,
            device,
            swapchain,
            swapchain_image_views,
            atlas,
            debug_utils_loader,
            surface_loader,
            swapchain_loader,
            viewport_size,
        }
    }

    pub fn destroy(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();
            self.atlas.destroy();
            self.swapchain_image_views
                .drain(..)
                .for_each(|image_view| self.device.destroy_image_view(image_view, None));
            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);
            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);
            self.debug_utils_loader
                .destroy_debug_utils_messenger(self.debug_callback, None);
            self.instance.destroy_instance(None);
        }
    }

    pub fn update_drawable_size(&self, size: Size<Pixels>) {
        if size != self.viewport_size {
            // todo!("vulkan")
        }
    }

    pub fn viewport_size(&self) -> Size<Pixels> {
        self.viewport_size
    }

    pub fn sprite_atlas(&self) -> &Arc<VulkanAtlas> {
        &self.atlas
    }

    pub fn draw(&mut self, _scene: &Scene) {}
}
