use std::default::Default;

use ash::vk;
use gpu_allocator::{
    vulkan::{AllocationCreateDesc, AllocationScheme, Allocator, AllocatorCreateDesc},
    MemoryLocation,
};
use log::info;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();

    let entry = unsafe { ash::Entry::load() }.unwrap();

    // Create Vulkan instance
    let instance = {
        let app_name = c"Vulkan gpu-allocator test";

        let appinfo = vk::ApplicationInfo::default()
            .application_name(app_name)
            .application_version(0)
            .engine_name(app_name)
            .engine_version(0)
            .api_version(vk::make_api_version(0, 1, 0, 0));

        let layer_names_raw = [c"VK_LAYER_KHRONOS_validation".as_ptr()];

        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&appinfo)
            .enabled_layer_names(&layer_names_raw);

        unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Instance creation error")
        }
    };

    // Look for vulkan physical device
    let (pdevice, queue_family_index) = {
        let pdevices = unsafe {
            instance
                .enumerate_physical_devices()
                .expect("Physical device error")
        };
        pdevices
            .iter()
            .find_map(|pdevice| {
                unsafe { instance.get_physical_device_queue_family_properties(*pdevice) }
                    .iter()
                    .enumerate()
                    .find_map(|(index, &info)| {
                        let supports_graphics = info.queue_flags.contains(vk::QueueFlags::GRAPHICS);
                        if supports_graphics {
                            Some((*pdevice, index))
                        } else {
                            None
                        }
                    })
            })
            .expect("Couldn't find suitable device.")
    };

    // Create vulkan device
    let device = {
        let device_extension_names_raw = vec![];
        let features = vk::PhysicalDeviceFeatures {
            shader_clip_distance: 1,
            ..Default::default()
        };
        let priorities = [1.0];

        let queue_info = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index as u32)
            .queue_priorities(&priorities);

        let create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(std::slice::from_ref(&queue_info))
            .enabled_extension_names(&device_extension_names_raw)
            .enabled_features(&features);

        unsafe { instance.create_device(pdevice, &create_info, None).unwrap() }
    };

    // Setting up the allocator
    let mut allocator = Allocator::new(&AllocatorCreateDesc {
        instance: instance.clone(),
        device: device.clone(),
        physical_device: pdevice,
        debug_settings: Default::default(),
        buffer_device_address: false,
        allocation_sizes: Default::default(),
    })
    .unwrap();

    // Test allocating Gpu Only memory
    {
        let test_buffer_info = vk::BufferCreateInfo::default()
            .size(512)
            .usage(vk::BufferUsageFlags::STORAGE_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let test_buffer = unsafe { device.create_buffer(&test_buffer_info, None) }.unwrap();
        let requirements = unsafe { device.get_buffer_memory_requirements(test_buffer) };
        let location = MemoryLocation::GpuOnly;

        let allocation = allocator
            .allocate(&AllocationCreateDesc {
                requirements,
                location,
                linear: true,
                allocation_scheme: AllocationScheme::GpuAllocatorManaged,
                name: "Test allocation (Gpu Only)",
            })
            .unwrap();

        unsafe {
            device
                .bind_buffer_memory(test_buffer, allocation.memory(), allocation.offset())
                .unwrap()
        };

        allocator.free(allocation).unwrap();

        unsafe { device.destroy_buffer(test_buffer, None) };

        info!("Allocation and deallocation of GpuOnly memory was successful.");
    }

    // Test allocating Cpu to Gpu memory
    {
        let test_buffer_info = vk::BufferCreateInfo::default()
            .size(512)
            .usage(vk::BufferUsageFlags::STORAGE_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let test_buffer = unsafe { device.create_buffer(&test_buffer_info, None) }.unwrap();
        let requirements = unsafe { device.get_buffer_memory_requirements(test_buffer) };
        let location = MemoryLocation::CpuToGpu;

        let allocation = allocator
            .allocate(&AllocationCreateDesc {
                requirements,
                location,
                linear: true,
                allocation_scheme: AllocationScheme::GpuAllocatorManaged,
                name: "Test allocation (Cpu to Gpu)",
            })
            .unwrap();

        unsafe {
            device
                .bind_buffer_memory(test_buffer, allocation.memory(), allocation.offset())
                .unwrap()
        };

        allocator.free(allocation).unwrap();

        unsafe { device.destroy_buffer(test_buffer, None) };

        info!("Allocation and deallocation of CpuToGpu memory was successful.");
    }

    // Test allocating Gpu to Cpu memory
    {
        let test_buffer_info = vk::BufferCreateInfo::default()
            .size(512)
            .usage(vk::BufferUsageFlags::STORAGE_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let test_buffer = unsafe { device.create_buffer(&test_buffer_info, None) }.unwrap();
        let requirements = unsafe { device.get_buffer_memory_requirements(test_buffer) };
        let location = MemoryLocation::GpuToCpu;

        let allocation = allocator
            .allocate(&AllocationCreateDesc {
                requirements,
                location,
                linear: true,
                allocation_scheme: AllocationScheme::GpuAllocatorManaged,
                name: "Test allocation (Gpu to Cpu)",
            })
            .unwrap();

        unsafe {
            device
                .bind_buffer_memory(test_buffer, allocation.memory(), allocation.offset())
                .unwrap()
        };

        allocator.free(allocation).unwrap();

        unsafe { device.destroy_buffer(test_buffer, None) };

        info!("Allocation and deallocation of GpuToCpu memory was successful.");
    }

    drop(allocator); // Explicitly drop before destruction of device and instance.
    unsafe { device.destroy_device(None) };
    unsafe { instance.destroy_instance(None) };
}
