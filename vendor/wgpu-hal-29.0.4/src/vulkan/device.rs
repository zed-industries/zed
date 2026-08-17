use alloc::{borrow::ToOwned as _, collections::BTreeMap, ffi::CString, sync::Arc, vec::Vec};
use core::{
    ffi::CStr,
    mem::{self, MaybeUninit},
    num::NonZeroU32,
    ptr,
    time::Duration,
};

use arrayvec::ArrayVec;
use ash::{ext, vk};
use hashbrown::hash_map::Entry;
use parking_lot::Mutex;

use super::{conv, RawTlasInstance};
use crate::TlasInstance;

impl super::DeviceShared {
    /// Set the name of `object` to `name`.
    ///
    /// If `name` contains an interior null byte, then the name set will be truncated to that byte.
    ///
    /// # Safety
    ///
    /// This method inherits the safety contract from [`vkSetDebugUtilsObjectName`]. In particular:
    ///
    /// - `object` must be a valid handle for one of the following:
    ///   - An instance-level object from the same instance as this device.
    ///   - A physical-device-level object that descends from the same physical device as this
    ///     device.
    ///   - A device-level object that descends from this device.
    /// - `object` must be externally synchronized—only the calling thread should access it during
    ///   this call.
    ///
    /// [`vkSetDebugUtilsObjectName`]: https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetDebugUtilsObjectNameEXT.html
    pub(super) unsafe fn set_object_name(&self, object: impl vk::Handle, name: &str) {
        let Some(extension) = self.extension_fns.debug_utils.as_ref() else {
            return;
        };

        // Keep variables outside the if-else block to ensure they do not
        // go out of scope while we hold a pointer to them
        let mut buffer: [u8; 64] = [0u8; 64];
        let buffer_vec: Vec<u8>;

        // Append a null terminator to the string
        let name_bytes = if name.len() < buffer.len() {
            // Common case, string is very small. Allocate a copy on the stack.
            buffer[..name.len()].copy_from_slice(name.as_bytes());
            // Add null terminator
            buffer[name.len()] = 0;
            &buffer[..name.len() + 1]
        } else {
            // Less common case, the string is large.
            // This requires a heap allocation.
            buffer_vec = name
                .as_bytes()
                .iter()
                .cloned()
                .chain(core::iter::once(0))
                .collect();
            &buffer_vec
        };

        let name = CStr::from_bytes_until_nul(name_bytes).expect("We have added a null byte");

        let _result = unsafe {
            extension.set_debug_utils_object_name(
                &vk::DebugUtilsObjectNameInfoEXT::default()
                    .object_handle(object)
                    .object_name(name),
            )
        };
    }

    pub fn make_render_pass(
        &self,
        key: super::RenderPassKey,
    ) -> Result<vk::RenderPass, crate::DeviceError> {
        Ok(match self.render_passes.lock().entry(key) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
                let super::RenderPassKey {
                    ref colors,
                    ref depth_stencil,
                    sample_count,
                    multiview_mask,
                } = *e.key();

                let mut vk_attachments = Vec::new();
                let mut color_refs = Vec::with_capacity(colors.len());
                let mut resolve_refs = Vec::with_capacity(color_refs.capacity());
                let mut ds_ref = None;
                let samples = vk::SampleCountFlags::from_raw(sample_count);
                let unused = vk::AttachmentReference {
                    attachment: vk::ATTACHMENT_UNUSED,
                    layout: vk::ImageLayout::UNDEFINED,
                };
                for cat in colors.iter() {
                    let (color_ref, resolve_ref) =
                        if let Some(super::ColorAttachmentKey { base, resolve }) = cat {
                            let super::AttachmentKey {
                                format,
                                layout,
                                ops,
                            } = *base;

                            let color_ref = vk::AttachmentReference {
                                attachment: vk_attachments.len() as u32,
                                layout,
                            };
                            vk_attachments.push({
                                let (load_op, store_op) = conv::map_attachment_ops(ops);
                                vk::AttachmentDescription::default()
                                    .format(format)
                                    .samples(samples)
                                    .load_op(load_op)
                                    .store_op(store_op)
                                    .initial_layout(layout)
                                    .final_layout(layout)
                            });
                            let resolve_ref = if let Some(rat) = resolve {
                                let super::AttachmentKey {
                                    format,
                                    layout,
                                    ops,
                                } = *rat;

                                let (load_op, store_op) = conv::map_attachment_ops(ops);
                                let vk_attachment = vk::AttachmentDescription::default()
                                    .format(format)
                                    .samples(vk::SampleCountFlags::TYPE_1)
                                    .load_op(load_op)
                                    .store_op(store_op)
                                    .initial_layout(layout)
                                    .final_layout(layout);
                                vk_attachments.push(vk_attachment);

                                vk::AttachmentReference {
                                    attachment: vk_attachments.len() as u32 - 1,
                                    layout,
                                }
                            } else {
                                unused
                            };

                            (color_ref, resolve_ref)
                        } else {
                            (unused, unused)
                        };

                    color_refs.push(color_ref);
                    resolve_refs.push(resolve_ref);
                }

                if let Some(ds) = depth_stencil {
                    let super::DepthStencilAttachmentKey {
                        ref base,
                        stencil_ops,
                    } = *ds;

                    let super::AttachmentKey {
                        format,
                        layout,
                        ops,
                    } = *base;

                    ds_ref = Some(vk::AttachmentReference {
                        attachment: vk_attachments.len() as u32,
                        layout,
                    });
                    let (load_op, store_op) = conv::map_attachment_ops(ops);
                    let (stencil_load_op, stencil_store_op) = conv::map_attachment_ops(stencil_ops);
                    let vk_attachment = vk::AttachmentDescription::default()
                        .format(format)
                        .samples(samples)
                        .load_op(load_op)
                        .store_op(store_op)
                        .stencil_load_op(stencil_load_op)
                        .stencil_store_op(stencil_store_op)
                        .initial_layout(layout)
                        .final_layout(layout);
                    vk_attachments.push(vk_attachment);
                }

                let vk_subpasses = [{
                    let mut vk_subpass = vk::SubpassDescription::default()
                        .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                        .color_attachments(&color_refs)
                        .resolve_attachments(&resolve_refs);

                    if self
                        .workarounds
                        .contains(super::Workarounds::EMPTY_RESOLVE_ATTACHMENT_LISTS)
                        && resolve_refs.is_empty()
                    {
                        vk_subpass.p_resolve_attachments = ptr::null();
                    }

                    if let Some(ref reference) = ds_ref {
                        vk_subpass = vk_subpass.depth_stencil_attachment(reference)
                    }
                    vk_subpass
                }];

                let mut vk_info = vk::RenderPassCreateInfo::default()
                    .attachments(&vk_attachments)
                    .subpasses(&vk_subpasses);

                let mut multiview_info;
                let mask;
                if let Some(multiview_mask) = multiview_mask {
                    mask = [multiview_mask.get()];

                    // On Vulkan 1.1 or later, this is an alias for core functionality
                    multiview_info = vk::RenderPassMultiviewCreateInfoKHR::default()
                        .view_masks(&mask)
                        .correlation_masks(&mask);
                    vk_info = vk_info.push_next(&mut multiview_info);
                }

                let raw = unsafe {
                    self.raw
                        .create_render_pass(&vk_info, None)
                        .map_err(super::map_host_device_oom_err)?
                };

                *e.insert(raw)
            }
        })
    }

    fn make_memory_ranges<'a, I: 'a + Iterator<Item = crate::MemoryRange>>(
        &self,
        buffer: &'a super::Buffer,
        ranges: I,
    ) -> Option<impl 'a + Iterator<Item = vk::MappedMemoryRange<'a>>> {
        let allocation = buffer.allocation.as_ref()?.lock();
        let mask = self.private_caps.non_coherent_map_mask;
        Some(ranges.map(move |range| {
            vk::MappedMemoryRange::default()
                .memory(allocation.memory())
                .offset((allocation.offset() + range.start) & !mask)
                .size((range.end - range.start + mask) & !mask)
        }))
    }
}

impl
    gpu_descriptor::DescriptorDevice<vk::DescriptorSetLayout, vk::DescriptorPool, vk::DescriptorSet>
    for super::DeviceShared
{
    unsafe fn create_descriptor_pool(
        &self,
        descriptor_count: &gpu_descriptor::DescriptorTotalCount,
        max_sets: u32,
        flags: gpu_descriptor::DescriptorPoolCreateFlags,
    ) -> Result<vk::DescriptorPool, gpu_descriptor::CreatePoolError> {
        //Note: ignoring other types, since they can't appear here
        let unfiltered_counts = [
            (vk::DescriptorType::SAMPLER, descriptor_count.sampler),
            (
                vk::DescriptorType::SAMPLED_IMAGE,
                descriptor_count.sampled_image,
            ),
            (
                vk::DescriptorType::STORAGE_IMAGE,
                descriptor_count.storage_image,
            ),
            (
                vk::DescriptorType::UNIFORM_BUFFER,
                descriptor_count.uniform_buffer,
            ),
            (
                vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
                descriptor_count.uniform_buffer_dynamic,
            ),
            (
                vk::DescriptorType::STORAGE_BUFFER,
                descriptor_count.storage_buffer,
            ),
            (
                vk::DescriptorType::STORAGE_BUFFER_DYNAMIC,
                descriptor_count.storage_buffer_dynamic,
            ),
            (
                vk::DescriptorType::ACCELERATION_STRUCTURE_KHR,
                descriptor_count.acceleration_structure,
            ),
        ];

        let filtered_counts = unfiltered_counts
            .iter()
            .cloned()
            .filter(|&(_, count)| count != 0)
            .map(|(ty, count)| vk::DescriptorPoolSize {
                ty,
                descriptor_count: count,
            })
            .collect::<ArrayVec<_, 8>>();

        let mut vk_flags =
            if flags.contains(gpu_descriptor::DescriptorPoolCreateFlags::UPDATE_AFTER_BIND) {
                vk::DescriptorPoolCreateFlags::UPDATE_AFTER_BIND
            } else {
                vk::DescriptorPoolCreateFlags::empty()
            };
        if flags.contains(gpu_descriptor::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET) {
            vk_flags |= vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET;
        }
        let vk_info = vk::DescriptorPoolCreateInfo::default()
            .max_sets(max_sets)
            .flags(vk_flags)
            .pool_sizes(&filtered_counts);

        match unsafe { self.raw.create_descriptor_pool(&vk_info, None) } {
            Ok(pool) => Ok(pool),
            Err(vk::Result::ERROR_OUT_OF_HOST_MEMORY) => {
                Err(gpu_descriptor::CreatePoolError::OutOfHostMemory)
            }
            Err(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY) => {
                Err(gpu_descriptor::CreatePoolError::OutOfDeviceMemory)
            }
            Err(vk::Result::ERROR_FRAGMENTATION) => {
                Err(gpu_descriptor::CreatePoolError::Fragmentation)
            }
            Err(err) => handle_unexpected(err),
        }
    }

    unsafe fn destroy_descriptor_pool(&self, pool: vk::DescriptorPool) {
        unsafe { self.raw.destroy_descriptor_pool(pool, None) }
    }

    unsafe fn alloc_descriptor_sets<'a>(
        &self,
        pool: &mut vk::DescriptorPool,
        layouts: impl ExactSizeIterator<Item = &'a vk::DescriptorSetLayout>,
        sets: &mut impl Extend<vk::DescriptorSet>,
    ) -> Result<(), gpu_descriptor::DeviceAllocationError> {
        let result = unsafe {
            self.raw.allocate_descriptor_sets(
                &vk::DescriptorSetAllocateInfo::default()
                    .descriptor_pool(*pool)
                    .set_layouts(
                        &smallvec::SmallVec::<[vk::DescriptorSetLayout; 32]>::from_iter(
                            layouts.cloned(),
                        ),
                    ),
            )
        };

        match result {
            Ok(vk_sets) => {
                sets.extend(vk_sets);
                Ok(())
            }
            Err(vk::Result::ERROR_OUT_OF_HOST_MEMORY)
            | Err(vk::Result::ERROR_OUT_OF_POOL_MEMORY) => {
                Err(gpu_descriptor::DeviceAllocationError::OutOfHostMemory)
            }
            Err(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY) => {
                Err(gpu_descriptor::DeviceAllocationError::OutOfDeviceMemory)
            }
            Err(vk::Result::ERROR_FRAGMENTED_POOL) => {
                Err(gpu_descriptor::DeviceAllocationError::FragmentedPool)
            }
            Err(err) => handle_unexpected(err),
        }
    }

    unsafe fn dealloc_descriptor_sets<'a>(
        &self,
        pool: &mut vk::DescriptorPool,
        sets: impl Iterator<Item = vk::DescriptorSet>,
    ) {
        let result = unsafe {
            self.raw.free_descriptor_sets(
                *pool,
                &smallvec::SmallVec::<[vk::DescriptorSet; 32]>::from_iter(sets),
            )
        };
        match result {
            Ok(()) => {}
            Err(err) => handle_unexpected(err),
        }
    }
}

struct CompiledStage {
    create_info: vk::PipelineShaderStageCreateInfo<'static>,
    _entry_point: CString,
    temp_raw_module: Option<vk::ShaderModule>,
}

impl super::Device {
    /// # Safety
    ///
    /// - `vk_image` must be created respecting `desc`
    /// - If `drop_callback` is [`None`], wgpu-hal will take ownership of `vk_image`. If
    ///   `drop_callback` is [`Some`], `vk_image` must be valid until the callback is called.
    /// - If the `ImageCreateFlags` does not contain `MUTABLE_FORMAT`, the `view_formats` of `desc` must be empty.
    /// - If `memory` is not [`super::TextureMemory::External`], wgpu-hal will take ownership of the
    ///   memory (which is presumed to back `vk_image`). Otherwise, the memory must remain valid until
    ///   `drop_callback` is called.
    pub unsafe fn texture_from_raw(
        &self,
        vk_image: vk::Image,
        desc: &crate::TextureDescriptor,
        drop_callback: Option<crate::DropCallback>,
        memory: super::TextureMemory,
    ) -> super::Texture {
        let identity = self.shared.texture_identity_factory.next();
        let drop_guard = crate::DropGuard::from_option(drop_callback);

        if let Some(label) = desc.label {
            unsafe { self.shared.set_object_name(vk_image, label) };
        }

        super::Texture {
            raw: vk_image,
            drop_guard,
            memory,
            format: desc.format,
            copy_size: desc.copy_extent(),
            identity,
        }
    }

    fn find_memory_type_index(
        &self,
        type_bits_req: u32,
        flags_req: vk::MemoryPropertyFlags,
    ) -> Option<usize> {
        let mem_properties = unsafe {
            self.shared
                .instance
                .raw
                .get_physical_device_memory_properties(self.shared.physical_device)
        };

        // https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryProperties.html
        for (i, mem_ty) in mem_properties.memory_types_as_slice().iter().enumerate() {
            let types_bits = 1 << i;
            let is_required_memory_type = type_bits_req & types_bits != 0;
            let has_required_properties = mem_ty.property_flags & flags_req == flags_req;
            if is_required_memory_type && has_required_properties {
                return Some(i);
            }
        }

        None
    }

    fn create_image_without_memory(
        &self,
        desc: &crate::TextureDescriptor,
        external_memory_image_create_info: Option<&mut vk::ExternalMemoryImageCreateInfo>,
    ) -> Result<ImageWithoutMemory, crate::DeviceError> {
        let copy_size = desc.copy_extent();

        let mut raw_flags = vk::ImageCreateFlags::empty();
        if desc.dimension == wgt::TextureDimension::D3
            && desc.usage.contains(wgt::TextureUses::COLOR_TARGET)
        {
            raw_flags |= vk::ImageCreateFlags::TYPE_2D_ARRAY_COMPATIBLE;
        }
        if desc.is_cube_compatible() {
            raw_flags |= vk::ImageCreateFlags::CUBE_COMPATIBLE;
        }

        let original_format = self.shared.private_caps.map_texture_format(desc.format);
        let mut vk_view_formats = vec![];
        if !desc.view_formats.is_empty() {
            raw_flags |= vk::ImageCreateFlags::MUTABLE_FORMAT;

            if self.shared.private_caps.image_format_list {
                vk_view_formats = desc
                    .view_formats
                    .iter()
                    .map(|f| self.shared.private_caps.map_texture_format(*f))
                    .collect();
                vk_view_formats.push(original_format)
            }
        }
        if desc.format.is_multi_planar_format() {
            raw_flags |=
                vk::ImageCreateFlags::MUTABLE_FORMAT | vk::ImageCreateFlags::EXTENDED_USAGE;
        }

        let mut vk_info = vk::ImageCreateInfo::default()
            .flags(raw_flags)
            .image_type(conv::map_texture_dimension(desc.dimension))
            .format(original_format)
            .extent(conv::map_copy_extent(&copy_size))
            .mip_levels(desc.mip_level_count)
            .array_layers(desc.array_layer_count())
            .samples(vk::SampleCountFlags::from_raw(desc.sample_count))
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(conv::map_texture_usage(desc.usage))
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .initial_layout(vk::ImageLayout::UNDEFINED);

        let mut format_list_info = vk::ImageFormatListCreateInfo::default();
        if !vk_view_formats.is_empty() {
            format_list_info = format_list_info.view_formats(&vk_view_formats);
            vk_info = vk_info.push_next(&mut format_list_info);
        }

        if let Some(ext_info) = external_memory_image_create_info {
            vk_info = vk_info.push_next(ext_info);
        }

        let raw = unsafe { self.shared.raw.create_image(&vk_info, None) }.map_err(map_err)?;
        fn map_err(err: vk::Result) -> crate::DeviceError {
            // We don't use VK_EXT_image_compression_control
            // VK_ERROR_COMPRESSION_EXHAUSTED_EXT
            super::map_host_device_oom_and_ioca_err(err)
        }
        let mut req = unsafe { self.shared.raw.get_image_memory_requirements(raw) };

        if desc.usage.contains(wgt::TextureUses::TRANSIENT) {
            let mem_type_index = self.find_memory_type_index(
                req.memory_type_bits,
                vk::MemoryPropertyFlags::LAZILY_ALLOCATED,
            );
            if let Some(mem_type_index) = mem_type_index {
                req.memory_type_bits = 1 << mem_type_index;
            }
        }

        Ok(ImageWithoutMemory {
            raw,
            requirements: req,
        })
    }

    /// # Safety
    ///
    /// - Vulkan (with VK_KHR_external_memory_win32)
    /// - The `d3d11_shared_handle` must be valid and respecting `desc`
    /// - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT` flag is used because we need to hold a reference to the handle
    #[cfg(windows)]
    pub unsafe fn texture_from_d3d11_shared_handle(
        &self,
        d3d11_shared_handle: windows::Win32::Foundation::HANDLE,
        desc: &crate::TextureDescriptor,
    ) -> Result<super::Texture, crate::DeviceError> {
        if !self
            .shared
            .features
            .contains(wgt::Features::VULKAN_EXTERNAL_MEMORY_WIN32)
        {
            log::error!("Vulkan driver does not support VK_KHR_external_memory_win32");
            return Err(crate::DeviceError::Unexpected);
        }

        let mut external_memory_image_info = vk::ExternalMemoryImageCreateInfo::default()
            .handle_types(vk::ExternalMemoryHandleTypeFlags::D3D11_TEXTURE);

        let image =
            self.create_image_without_memory(desc, Some(&mut external_memory_image_info))?;

        // Some external memory types require dedicated allocation
        // https://docs.vulkan.org/guide/latest/extensions/external.html#_importing_memory
        let mut dedicated_allocate_info =
            vk::MemoryDedicatedAllocateInfo::default().image(image.raw);

        let mut import_memory_info = vk::ImportMemoryWin32HandleInfoKHR::default()
            .handle_type(vk::ExternalMemoryHandleTypeFlags::D3D11_TEXTURE)
            .handle(d3d11_shared_handle.0 as _);
        // TODO: We should use `push_next` instead, but currently ash does not provide this method for the `ImportMemoryWin32HandleInfoKHR` type.
        #[allow(clippy::unnecessary_mut_passed)]
        {
            import_memory_info.p_next = <*const _>::cast(&mut dedicated_allocate_info);
        }

        let mem_type_index = self
            .find_memory_type_index(
                image.requirements.memory_type_bits,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
            )
            .ok_or(crate::DeviceError::Unexpected)?;

        let memory_allocate_info = vk::MemoryAllocateInfo::default()
            .allocation_size(image.requirements.size)
            .memory_type_index(mem_type_index as _)
            .push_next(&mut import_memory_info);
        let memory = unsafe { self.shared.raw.allocate_memory(&memory_allocate_info, None) }
            .map_err(super::map_host_device_oom_err)?;

        unsafe { self.shared.raw.bind_image_memory(image.raw, memory, 0) }
            .map_err(super::map_host_device_oom_err)?;

        Ok(unsafe {
            self.texture_from_raw(
                image.raw,
                desc,
                None,
                super::TextureMemory::Dedicated(memory),
            )
        })
    }

    fn create_shader_module_impl(
        &self,
        spv: &[u32],
        label: &crate::Label<'_>,
    ) -> Result<vk::ShaderModule, crate::DeviceError> {
        let vk_info = vk::ShaderModuleCreateInfo::default()
            .flags(vk::ShaderModuleCreateFlags::empty())
            .code(spv);

        let raw = unsafe {
            profiling::scope!("vkCreateShaderModule");
            self.shared
                .raw
                .create_shader_module(&vk_info, None)
                .map_err(map_err)?
        };
        fn map_err(err: vk::Result) -> crate::DeviceError {
            // We don't use VK_NV_glsl_shader
            // VK_ERROR_INVALID_SHADER_NV
            super::map_host_device_oom_err(err)
        }

        if let Some(label) = label {
            unsafe { self.shared.set_object_name(raw, label) };
        }

        Ok(raw)
    }

    fn compile_stage(
        &self,
        stage: &crate::ProgrammableStage<super::ShaderModule>,
        naga_stage: naga::ShaderStage,
        binding_map: &naga::back::spv::BindingMap,
    ) -> Result<CompiledStage, crate::PipelineError> {
        let stage_flags = crate::auxil::map_naga_stage(naga_stage);
        let vk_module = match *stage.module {
            super::ShaderModule::Raw(raw) => raw,
            super::ShaderModule::Intermediate {
                ref naga_shader,
                runtime_checks,
            } => {
                let pipeline_options = naga::back::spv::PipelineOptions {
                    entry_point: stage.entry_point.to_owned(),
                    shader_stage: naga_stage,
                };
                let needs_temp_options = !runtime_checks.bounds_checks
                    || !runtime_checks.force_loop_bounding
                    || !runtime_checks.ray_query_initialization_tracking
                    || !binding_map.is_empty()
                    || naga_shader.debug_source.is_some()
                    || !stage.zero_initialize_workgroup_memory
                    || !runtime_checks.task_shader_dispatch_tracking
                    || !runtime_checks.mesh_shader_primitive_indices_clamp;

                let mut temp_options;
                let options = if needs_temp_options {
                    temp_options = self.naga_options.clone();
                    if !runtime_checks.bounds_checks {
                        temp_options.bounds_check_policies = naga::proc::BoundsCheckPolicies {
                            index: naga::proc::BoundsCheckPolicy::Unchecked,
                            buffer: naga::proc::BoundsCheckPolicy::Unchecked,
                            image_load: naga::proc::BoundsCheckPolicy::Unchecked,
                            binding_array: naga::proc::BoundsCheckPolicy::Unchecked,
                        };
                    }
                    if !runtime_checks.force_loop_bounding {
                        temp_options.force_loop_bounding = false;
                    }
                    if !runtime_checks.ray_query_initialization_tracking {
                        temp_options.ray_query_initialization_tracking = false;
                    }
                    if !binding_map.is_empty() {
                        temp_options.binding_map = binding_map.clone();
                    }

                    if let Some(ref debug) = naga_shader.debug_source {
                        temp_options.debug_info = Some(naga::back::spv::DebugInfo {
                            source_code: &debug.source_code,
                            file_name: debug.file_name.as_ref(),
                            language: naga::back::spv::SourceLanguage::WGSL,
                        })
                    }
                    if !stage.zero_initialize_workgroup_memory {
                        temp_options.zero_initialize_workgroup_memory =
                            naga::back::spv::ZeroInitializeWorkgroupMemoryMode::None;
                    }
                    if !runtime_checks.task_shader_dispatch_tracking {
                        temp_options.task_dispatch_limits = None;
                    }
                    temp_options.mesh_shader_primitive_indices_clamp =
                        runtime_checks.mesh_shader_primitive_indices_clamp;

                    &temp_options
                } else {
                    &self.naga_options
                };

                let (module, info) = naga::back::pipeline_constants::process_overrides(
                    &naga_shader.module,
                    &naga_shader.info,
                    Some((naga_stage, stage.entry_point)),
                    stage.constants,
                )
                .map_err(|e| {
                    crate::PipelineError::PipelineConstants(stage_flags, format!("{e}"))
                })?;

                let spv = {
                    profiling::scope!("naga::spv::write_vec");
                    naga::back::spv::write_vec(&module, &info, options, Some(&pipeline_options))
                }
                .map_err(|e| crate::PipelineError::Linkage(stage_flags, format!("{e}")))?;
                self.create_shader_module_impl(&spv, &None)?
            }
        };

        let mut flags = vk::PipelineShaderStageCreateFlags::empty();
        if self.shared.features.contains(wgt::Features::SUBGROUP) {
            flags |= vk::PipelineShaderStageCreateFlags::ALLOW_VARYING_SUBGROUP_SIZE
        }

        let entry_point = CString::new(stage.entry_point).unwrap();
        let mut create_info = vk::PipelineShaderStageCreateInfo::default()
            .flags(flags)
            .stage(conv::map_shader_stage(stage_flags))
            .module(vk_module);

        // Circumvent struct lifetime check because of a self-reference inside CompiledStage
        create_info.p_name = entry_point.as_ptr();

        Ok(CompiledStage {
            create_info,
            _entry_point: entry_point,
            temp_raw_module: match *stage.module {
                super::ShaderModule::Raw(_) => None,
                super::ShaderModule::Intermediate { .. } => Some(vk_module),
            },
        })
    }

    /// Returns the queue family index of the device's internal queue.
    ///
    /// This is useful for constructing memory barriers needed for queue family ownership transfer when
    /// external memory is involved (from/to `VK_QUEUE_FAMILY_EXTERNAL_KHR` and `VK_QUEUE_FAMILY_FOREIGN_EXT`
    /// for example).
    pub fn queue_family_index(&self) -> u32 {
        self.shared.family_index
    }

    pub fn queue_index(&self) -> u32 {
        self.shared.queue_index
    }

    pub fn raw_device(&self) -> &ash::Device {
        &self.shared.raw
    }

    pub fn raw_physical_device(&self) -> vk::PhysicalDevice {
        self.shared.physical_device
    }

    pub fn raw_queue(&self) -> vk::Queue {
        self.shared.raw_queue
    }

    pub fn enabled_device_extensions(&self) -> &[&'static CStr] {
        &self.shared.enabled_extensions
    }

    pub fn shared_instance(&self) -> &super::InstanceShared {
        &self.shared.instance
    }

    fn error_if_would_oom_on_resource_allocation(
        &self,
        needs_host_access: bool,
        size: u64,
    ) -> Result<(), crate::DeviceError> {
        let Some(threshold) = self
            .shared
            .instance
            .memory_budget_thresholds
            .for_resource_creation
        else {
            return Ok(());
        };

        if !self
            .shared
            .enabled_extensions
            .contains(&ext::memory_budget::NAME)
        {
            return Ok(());
        }

        let get_physical_device_properties = self
            .shared
            .instance
            .get_physical_device_properties
            .as_ref()
            .unwrap();

        let mut memory_budget_properties = vk::PhysicalDeviceMemoryBudgetPropertiesEXT::default();

        let mut memory_properties =
            vk::PhysicalDeviceMemoryProperties2::default().push_next(&mut memory_budget_properties);

        unsafe {
            get_physical_device_properties.get_physical_device_memory_properties2(
                self.shared.physical_device,
                &mut memory_properties,
            );
        }

        let mut host_visible_heaps = [false; vk::MAX_MEMORY_HEAPS];
        let mut device_local_heaps = [false; vk::MAX_MEMORY_HEAPS];

        let memory_properties = memory_properties.memory_properties;

        for i in 0..memory_properties.memory_type_count {
            let memory_type = memory_properties.memory_types[i as usize];
            let flags = memory_type.property_flags;

            if flags.intersects(
                vk::MemoryPropertyFlags::LAZILY_ALLOCATED | vk::MemoryPropertyFlags::PROTECTED,
            ) {
                continue; // not used by gpu-alloc
            }

            if flags.contains(vk::MemoryPropertyFlags::HOST_VISIBLE) {
                host_visible_heaps[memory_type.heap_index as usize] = true;
            }

            if flags.contains(vk::MemoryPropertyFlags::DEVICE_LOCAL) {
                device_local_heaps[memory_type.heap_index as usize] = true;
            }
        }

        let heaps = if needs_host_access {
            host_visible_heaps
        } else {
            device_local_heaps
        };

        // NOTE: We might end up checking multiple heaps since gpu-alloc doesn't have a way
        // for us to query the heap the resource will end up on. But this is unlikely,
        // there is usually only one heap on integrated GPUs and two on dedicated GPUs.

        for (i, check) in heaps.iter().enumerate() {
            if !check {
                continue;
            }

            let heap_usage = memory_budget_properties.heap_usage[i];
            let heap_budget = memory_budget_properties.heap_budget[i];

            if heap_usage + size >= heap_budget / 100 * threshold as u64 {
                return Err(crate::DeviceError::OutOfMemory);
            }
        }

        Ok(())
    }
}

impl crate::Device for super::Device {
    type A = super::Api;

    unsafe fn create_buffer(
        &self,
        desc: &crate::BufferDescriptor,
    ) -> Result<super::Buffer, crate::DeviceError> {
        let vk_info = vk::BufferCreateInfo::default()
            .size(desc.size)
            .usage(conv::map_buffer_usage(desc.usage))
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let raw = unsafe {
            self.shared
                .raw
                .create_buffer(&vk_info, None)
                .map_err(super::map_host_device_oom_and_ioca_err)?
        };

        let mut requirements = unsafe { self.shared.raw.get_buffer_memory_requirements(raw) };

        let is_cpu_read = desc.usage.contains(wgt::BufferUses::MAP_READ);
        let is_cpu_write = desc.usage.contains(wgt::BufferUses::MAP_WRITE);

        let location = match (is_cpu_read, is_cpu_write) {
            (true, true) => gpu_allocator::MemoryLocation::CpuToGpu,
            (true, false) => gpu_allocator::MemoryLocation::GpuToCpu,
            (false, true) => gpu_allocator::MemoryLocation::CpuToGpu,
            (false, false) => gpu_allocator::MemoryLocation::GpuOnly,
        };

        let needs_host_access = is_cpu_read || is_cpu_write;

        self.error_if_would_oom_on_resource_allocation(needs_host_access, requirements.size)
            .inspect_err(|_| {
                unsafe { self.shared.raw.destroy_buffer(raw, None) };
            })?;

        let name = desc.label.unwrap_or("Unlabeled buffer");

        if desc
            .usage
            .contains(wgt::BufferUses::ACCELERATION_STRUCTURE_SCRATCH)
        {
            // There is no way to specify this usage to Vulkan so we must make sure the alignment requirement is large enough.
            requirements.alignment = requirements
                .alignment
                .max(self.shared.private_caps.scratch_buffer_alignment as u64);
        }

        let allocation = self
            .mem_allocator
            .lock()
            .allocate(&gpu_allocator::vulkan::AllocationCreateDesc {
                name,
                requirements: vk::MemoryRequirements {
                    memory_type_bits: requirements.memory_type_bits & self.valid_ash_memory_types,
                    ..requirements
                },
                location,
                linear: true, // Buffers are always linear
                allocation_scheme: gpu_allocator::vulkan::AllocationScheme::GpuAllocatorManaged,
            })
            .inspect_err(|_| {
                unsafe { self.shared.raw.destroy_buffer(raw, None) };
            })?;

        unsafe {
            self.shared
                .raw
                .bind_buffer_memory(raw, allocation.memory(), allocation.offset())
        }
        .map_err(super::map_host_device_oom_and_ioca_err)
        .inspect_err(|_| {
            unsafe { self.shared.raw.destroy_buffer(raw, None) };
        })?;

        if let Some(label) = desc.label {
            unsafe { self.shared.set_object_name(raw, label) };
        }

        self.counters.buffer_memory.add(allocation.size() as isize);
        self.counters.buffers.add(1);

        Ok(super::Buffer {
            raw,
            allocation: Some(Mutex::new(super::BufferMemoryBacking::Managed(allocation))),
        })
    }
    unsafe fn destroy_buffer(&self, buffer: super::Buffer) {
        unsafe { self.shared.raw.destroy_buffer(buffer.raw, None) };
        if let Some(allocation) = buffer.allocation {
            let allocation = allocation.into_inner();
            self.counters.buffer_memory.sub(allocation.size() as isize);
            match allocation {
                super::BufferMemoryBacking::Managed(allocation) => {
                    let result = self.mem_allocator.lock().free(allocation);
                    if let Err(err) = result {
                        log::warn!("Failed to free buffer allocation: {err}");
                    }
                }
                super::BufferMemoryBacking::VulkanMemory { memory, .. } => unsafe {
                    self.shared.raw.free_memory(memory, None);
                },
            }
        }

        self.counters.buffers.sub(1);
    }

    unsafe fn add_raw_buffer(&self, _buffer: &super::Buffer) {
        self.counters.buffers.add(1);
    }

    unsafe fn map_buffer(
        &self,
        buffer: &super::Buffer,
        range: crate::MemoryRange,
    ) -> Result<crate::BufferMapping, crate::DeviceError> {
        if let Some(ref allocation) = buffer.allocation {
            let mut allocation = allocation.lock();
            if let super::BufferMemoryBacking::Managed(ref mut allocation) = *allocation {
                let is_coherent = allocation
                    .memory_properties()
                    .contains(vk::MemoryPropertyFlags::HOST_COHERENT);
                Ok(crate::BufferMapping {
                    ptr: unsafe {
                        allocation
                            .mapped_ptr()
                            .unwrap()
                            .cast()
                            .offset(range.start as isize)
                    },
                    is_coherent,
                })
            } else {
                crate::hal_usage_error("tried to map externally created buffer")
            }
        } else {
            crate::hal_usage_error("tried to map external buffer")
        }
    }

    unsafe fn unmap_buffer(&self, buffer: &super::Buffer) {
        if buffer.allocation.is_some() {
            // gpu-allocator maps the buffer when allocated and unmap it when free'd
        } else {
            crate::hal_usage_error("tried to unmap external buffer")
        }
    }

    unsafe fn flush_mapped_ranges<I>(&self, buffer: &super::Buffer, ranges: I)
    where
        I: Iterator<Item = crate::MemoryRange>,
    {
        if let Some(vk_ranges) = self.shared.make_memory_ranges(buffer, ranges) {
            unsafe {
                self.shared
                    .raw
                    .flush_mapped_memory_ranges(
                        &smallvec::SmallVec::<[vk::MappedMemoryRange; 32]>::from_iter(vk_ranges),
                    )
            }
            .unwrap();
        }
    }
    unsafe fn invalidate_mapped_ranges<I>(&self, buffer: &super::Buffer, ranges: I)
    where
        I: Iterator<Item = crate::MemoryRange>,
    {
        if let Some(vk_ranges) = self.shared.make_memory_ranges(buffer, ranges) {
            unsafe {
                self.shared
                    .raw
                    .invalidate_mapped_memory_ranges(&smallvec::SmallVec::<
                        [vk::MappedMemoryRange; 32],
                    >::from_iter(vk_ranges))
            }
            .unwrap();
        }
    }

    unsafe fn create_texture(
        &self,
        desc: &crate::TextureDescriptor,
    ) -> Result<super::Texture, crate::DeviceError> {
        let image = self.create_image_without_memory(desc, None)?;

        self.error_if_would_oom_on_resource_allocation(false, image.requirements.size)
            .inspect_err(|_| {
                unsafe { self.shared.raw.destroy_image(image.raw, None) };
            })?;

        let name = desc.label.unwrap_or("Unlabeled texture");

        let allocation = self
            .mem_allocator
            .lock()
            .allocate(&gpu_allocator::vulkan::AllocationCreateDesc {
                name,
                requirements: vk::MemoryRequirements {
                    memory_type_bits: image.requirements.memory_type_bits
                        & self.valid_ash_memory_types,
                    ..image.requirements
                },
                location: gpu_allocator::MemoryLocation::GpuOnly,
                linear: false,
                allocation_scheme: gpu_allocator::vulkan::AllocationScheme::GpuAllocatorManaged,
            })
            .inspect_err(|_| {
                unsafe { self.shared.raw.destroy_image(image.raw, None) };
            })?;

        self.counters.texture_memory.add(allocation.size() as isize);

        unsafe {
            self.shared
                .raw
                .bind_image_memory(image.raw, allocation.memory(), allocation.offset())
        }
        .map_err(super::map_host_device_oom_err)
        .inspect_err(|_| {
            unsafe { self.shared.raw.destroy_image(image.raw, None) };
        })?;

        Ok(unsafe {
            self.texture_from_raw(
                image.raw,
                desc,
                None,
                super::TextureMemory::Allocation(allocation),
            )
        })
    }

    unsafe fn destroy_texture(&self, texture: super::Texture) {
        if texture.drop_guard.is_none() {
            unsafe { self.shared.raw.destroy_image(texture.raw, None) };
        }

        match texture.memory {
            super::TextureMemory::Allocation(allocation) => {
                self.counters.texture_memory.sub(allocation.size() as isize);
                let result = self.mem_allocator.lock().free(allocation);
                if let Err(err) = result {
                    log::warn!("Failed to free texture allocation: {err}");
                }
            }
            super::TextureMemory::Dedicated(memory) => unsafe {
                self.shared.raw.free_memory(memory, None);
            },
            super::TextureMemory::External => {}
        }

        self.counters.textures.sub(1);
    }

    unsafe fn add_raw_texture(&self, _texture: &super::Texture) {
        self.counters.textures.add(1);
    }

    unsafe fn create_texture_view(
        &self,
        texture: &super::Texture,
        desc: &crate::TextureViewDescriptor,
    ) -> Result<super::TextureView, crate::DeviceError> {
        let subresource_range = conv::map_subresource_range(&desc.range, texture.format);
        let raw_format = self.shared.private_caps.map_texture_format(desc.format);
        let mut vk_info = vk::ImageViewCreateInfo::default()
            .flags(vk::ImageViewCreateFlags::empty())
            .image(texture.raw)
            .view_type(conv::map_view_dimension(desc.dimension))
            .format(raw_format)
            .subresource_range(subresource_range);
        let layers =
            NonZeroU32::new(subresource_range.layer_count).expect("Unexpected zero layer count");

        let mut image_view_info;
        if self.shared.private_caps.image_view_usage && !desc.usage.is_empty() {
            image_view_info =
                vk::ImageViewUsageCreateInfo::default().usage(conv::map_texture_usage(desc.usage));
            vk_info = vk_info.push_next(&mut image_view_info);
        }

        let raw = unsafe { self.shared.raw.create_image_view(&vk_info, None) }
            .map_err(super::map_host_device_oom_and_ioca_err)?;

        if let Some(label) = desc.label {
            unsafe { self.shared.set_object_name(raw, label) };
        }

        let identity = self.shared.texture_view_identity_factory.next();

        self.counters.texture_views.add(1);

        Ok(super::TextureView {
            raw_texture: texture.raw,
            raw,
            _layers: layers,
            format: desc.format,
            raw_format,
            base_mip_level: desc.range.base_mip_level,
            dimension: desc.dimension,
            texture_identity: texture.identity,
            view_identity: identity,
        })
    }
    unsafe fn destroy_texture_view(&self, view: super::TextureView) {
        unsafe { self.shared.raw.destroy_image_view(view.raw, None) };

        self.counters.texture_views.sub(1);
    }

    unsafe fn create_sampler(
        &self,
        desc: &crate::SamplerDescriptor,
    ) -> Result<super::Sampler, crate::DeviceError> {
        let mut create_info = vk::SamplerCreateInfo::default()
            .flags(vk::SamplerCreateFlags::empty())
            .mag_filter(conv::map_filter_mode(desc.mag_filter))
            .min_filter(conv::map_filter_mode(desc.min_filter))
            .mipmap_mode(conv::map_mip_filter_mode(desc.mipmap_filter))
            .address_mode_u(conv::map_address_mode(desc.address_modes[0]))
            .address_mode_v(conv::map_address_mode(desc.address_modes[1]))
            .address_mode_w(conv::map_address_mode(desc.address_modes[2]))
            .min_lod(desc.lod_clamp.start)
            .max_lod(desc.lod_clamp.end);

        if let Some(fun) = desc.compare {
            create_info = create_info
                .compare_enable(true)
                .compare_op(conv::map_comparison(fun));
        }

        if desc.anisotropy_clamp != 1 {
            // We only enable anisotropy if it is supported, and wgpu-hal interface guarantees
            // the clamp is in the range [1, 16] which is always supported if anisotropy is.
            create_info = create_info
                .anisotropy_enable(true)
                .max_anisotropy(desc.anisotropy_clamp as f32);
        }

        if let Some(color) = desc.border_color {
            create_info = create_info.border_color(conv::map_border_color(color));
        }

        let mut sampler_cache_guard = self.shared.sampler_cache.lock();

        let raw = sampler_cache_guard.create_sampler(&self.shared.raw, create_info)?;

        // Note: Cached samplers will just continually overwrite the label
        //
        // https://github.com/gfx-rs/wgpu/issues/6867
        if let Some(label) = desc.label {
            // SAFETY: we are holding a lock on the sampler cache,
            // so we can only be setting the name from one thread.
            unsafe { self.shared.set_object_name(raw, label) };
        }

        drop(sampler_cache_guard);

        self.counters.samplers.add(1);

        Ok(super::Sampler { raw, create_info })
    }
    unsafe fn destroy_sampler(&self, sampler: super::Sampler) {
        self.shared.sampler_cache.lock().destroy_sampler(
            &self.shared.raw,
            sampler.create_info,
            sampler.raw,
        );

        self.counters.samplers.sub(1);
    }

    unsafe fn create_command_encoder(
        &self,
        desc: &crate::CommandEncoderDescriptor<super::Queue>,
    ) -> Result<super::CommandEncoder, crate::DeviceError> {
        let vk_info = vk::CommandPoolCreateInfo::default()
            .queue_family_index(desc.queue.family_index)
            .flags(vk::CommandPoolCreateFlags::TRANSIENT);

        let raw = unsafe {
            self.shared
                .raw
                .create_command_pool(&vk_info, None)
                .map_err(super::map_host_device_oom_err)?
        };

        self.counters.command_encoders.add(1);

        Ok(super::CommandEncoder {
            raw,
            device: Arc::clone(&self.shared),
            active: vk::CommandBuffer::null(),
            bind_point: vk::PipelineBindPoint::default(),
            temp: super::Temp::default(),
            free: Vec::new(),
            discarded: Vec::new(),
            rpass_debug_marker_active: false,
            end_of_pass_timer_query: None,
            framebuffers: Default::default(),
            temp_texture_views: Default::default(),
            counters: Arc::clone(&self.counters),
            current_pipeline_is_multiview: false,
        })
    }

    unsafe fn create_bind_group_layout(
        &self,
        desc: &crate::BindGroupLayoutDescriptor,
    ) -> Result<super::BindGroupLayout, crate::DeviceError> {
        // Iterate through the entries and accumulate our Vulkan
        // DescriptorSetLayoutBindings and DescriptorBindingFlags, as well as
        // our binding map and our descriptor counts.
        // Note: not bothering with on stack arrays here as it's low frequency
        let mut vk_bindings = Vec::new();
        let mut binding_flags = Vec::new();
        let mut binding_map = Vec::new();
        let mut next_binding = 0;
        let mut contains_binding_arrays = false;
        let mut desc_count = gpu_descriptor::DescriptorTotalCount::default();
        for entry in desc.entries {
            if entry.count.is_some() {
                contains_binding_arrays = true;
            }

            let partially_bound = desc
                .flags
                .contains(crate::BindGroupLayoutFlags::PARTIALLY_BOUND);
            let mut flags = vk::DescriptorBindingFlags::empty();
            if partially_bound && entry.count.is_some() {
                flags |= vk::DescriptorBindingFlags::PARTIALLY_BOUND;
            }
            if entry.count.is_some() {
                flags |= vk::DescriptorBindingFlags::UPDATE_AFTER_BIND;
            }

            let count = entry.count.map_or(1, |c| c.get());
            match entry.ty {
                wgt::BindingType::ExternalTexture => unimplemented!(),
                _ => {
                    vk_bindings.push(vk::DescriptorSetLayoutBinding {
                        binding: next_binding,
                        descriptor_type: conv::map_binding_type(entry.ty),
                        descriptor_count: count,
                        stage_flags: conv::map_shader_stage(entry.visibility),
                        p_immutable_samplers: ptr::null(),
                        _marker: Default::default(),
                    });
                    binding_flags.push(flags);
                    binding_map.push((
                        entry.binding,
                        super::BindingInfo {
                            binding: next_binding,
                            binding_array_size: entry.count,
                        },
                    ));
                    next_binding += 1;
                }
            }

            match entry.ty {
                wgt::BindingType::Buffer {
                    ty,
                    has_dynamic_offset,
                    ..
                } => match ty {
                    wgt::BufferBindingType::Uniform => {
                        if has_dynamic_offset {
                            desc_count.uniform_buffer_dynamic += count;
                        } else {
                            desc_count.uniform_buffer += count;
                        }
                    }
                    wgt::BufferBindingType::Storage { .. } => {
                        if has_dynamic_offset {
                            desc_count.storage_buffer_dynamic += count;
                        } else {
                            desc_count.storage_buffer += count;
                        }
                    }
                },
                wgt::BindingType::Sampler { .. } => {
                    desc_count.sampler += count;
                }
                wgt::BindingType::Texture { .. } => {
                    desc_count.sampled_image += count;
                }
                wgt::BindingType::StorageTexture { .. } => {
                    desc_count.storage_image += count;
                }
                wgt::BindingType::AccelerationStructure { .. } => {
                    desc_count.acceleration_structure += count;
                }
                wgt::BindingType::ExternalTexture => unimplemented!(),
            }
        }

        let vk_info = vk::DescriptorSetLayoutCreateInfo::default()
            .bindings(&vk_bindings)
            .flags(if contains_binding_arrays {
                vk::DescriptorSetLayoutCreateFlags::UPDATE_AFTER_BIND_POOL
            } else {
                vk::DescriptorSetLayoutCreateFlags::empty()
            });

        let mut binding_flag_info =
            vk::DescriptorSetLayoutBindingFlagsCreateInfo::default().binding_flags(&binding_flags);

        let vk_info = vk_info.push_next(&mut binding_flag_info);

        let raw = unsafe {
            self.shared
                .raw
                .create_descriptor_set_layout(&vk_info, None)
                .map_err(super::map_host_device_oom_err)?
        };

        if let Some(label) = desc.label {
            unsafe { self.shared.set_object_name(raw, label) };
        }

        self.counters.bind_group_layouts.add(1);

        Ok(super::BindGroupLayout {
            raw,
            desc_count,
            entries: desc.entries.into(),
            binding_map,
            contains_binding_arrays,
        })
    }
    unsafe fn destroy_bind_group_layout(&self, bg_layout: super::BindGroupLayout) {
        unsafe {
            self.shared
                .raw
                .destroy_descriptor_set_layout(bg_layout.raw, None)
        };

        self.counters.bind_group_layouts.sub(1);
    }

    unsafe fn create_pipeline_layout(
        &self,
        desc: &crate::PipelineLayoutDescriptor<super::BindGroupLayout>,
    ) -> Result<super::PipelineLayout, crate::DeviceError> {
        //Note: not bothering with on stack array here as it's low frequency
        let vk_set_layouts = desc
            .bind_group_layouts
            .iter()
            .map(|bgl| match bgl {
                Some(bgl) => bgl.raw,
                None => {
                    // `VUID-VkPipelineLayoutCreateInfo-pSetLayouts-parameter`
                    // says `VK_NULL_HANDLE` is allowed but
                    // `VUID-VkPipelineLayoutCreateInfo-graphicsPipelineLibrary-06753`
                    // says it's not, unless the `graphicsPipelineLibrary`
                    // feature is enabled.
                    //
                    // We use an empty descriptor set layout to work around this.
                    self.shared.empty_descriptor_set_layout
                }
            })
            .collect::<Vec<_>>();
        let vk_immediates_ranges: Option<vk::PushConstantRange> = if desc.immediate_size != 0 {
            Some(vk::PushConstantRange {
                stage_flags: vk::ShaderStageFlags::ALL,
                offset: 0,
                size: desc.immediate_size,
            })
        } else {
            None
        };

        let vk_info = vk::PipelineLayoutCreateInfo::default()
            .flags(vk::PipelineLayoutCreateFlags::empty())
            .set_layouts(&vk_set_layouts)
            .push_constant_ranges(vk_immediates_ranges.as_slice());

        let raw = {
            profiling::scope!("vkCreatePipelineLayout");
            unsafe {
                self.shared
                    .raw
                    .create_pipeline_layout(&vk_info, None)
                    .map_err(super::map_host_device_oom_err)?
            }
        };

        if let Some(label) = desc.label {
            unsafe { self.shared.set_object_name(raw, label) };
        }

        let mut binding_map = BTreeMap::new();
        for (group, layout) in desc.bind_group_layouts.iter().enumerate() {
            let Some(layout) = layout else {
                continue;
            };

            for &(binding, binding_info) in &layout.binding_map {
                binding_map.insert(
                    naga::ResourceBinding {
                        group: group as u32,
                        binding,
                    },
                    naga::back::spv::BindingInfo {
                        descriptor_set: group as u32,
                        binding: binding_info.binding,
                        binding_array_size: binding_info.binding_array_size.map(NonZeroU32::get),
                    },
                );
            }
        }

        self.counters.pipeline_layouts.add(1);
        Ok(super::PipelineLayout { raw, binding_map })
    }
    unsafe fn destroy_pipeline_layout(&self, pipeline_layout: super::PipelineLayout) {
        unsafe {
            self.shared
                .raw
                .destroy_pipeline_layout(pipeline_layout.raw, None)
        };

        self.counters.pipeline_layouts.sub(1);
    }

    unsafe fn create_bind_group(
        &self,
        desc: &crate::BindGroupDescriptor<
            super::BindGroupLayout,
            super::Buffer,
            super::Sampler,
            super::TextureView,
            super::AccelerationStructure,
        >,
    ) -> Result<super::BindGroup, crate::DeviceError> {
        let desc_set_layout_flags = if desc.layout.contains_binding_arrays {
            gpu_descriptor::DescriptorSetLayoutCreateFlags::UPDATE_AFTER_BIND
        } else {
            gpu_descriptor::DescriptorSetLayoutCreateFlags::empty()
        };

        let mut vk_sets = unsafe {
            self.desc_allocator.lock().allocate(
                &*self.shared,
                &desc.layout.raw,
                desc_set_layout_flags,
                &desc.layout.desc_count,
                1,
            )?
        };

        let set = vk_sets.pop().unwrap();
        if let Some(label) = desc.label {
            unsafe { self.shared.set_object_name(*set.raw(), label) };
        }

        /// Helper for splitting off and initializing a given number of elements on a pre-allocated
        /// stack, based on items returned from an [`ExactSizeIterator`].  Typically created from a
        /// [`MaybeUninit`] slice (see [`Vec::spare_capacity_mut()`]).
        /// The updated [`ExtensionStack`] of remaining uninitialized elements is returned, safely
        /// representing that the initialized and remaining elements are two independent mutable
        /// borrows.
        struct ExtendStack<'a, T> {
            remainder: &'a mut [MaybeUninit<T>],
        }

        impl<'a, T> ExtendStack<'a, T> {
            fn from_vec_capacity(vec: &'a mut Vec<T>) -> Self {
                Self {
                    remainder: vec.spare_capacity_mut(),
                }
            }

            fn extend_one(self, value: T) -> (Self, &'a mut T) {
                let (to_init, remainder) = self.remainder.split_first_mut().unwrap();
                let init = to_init.write(value);
                (Self { remainder }, init)
            }

            fn extend(
                self,
                iter: impl IntoIterator<Item = T> + ExactSizeIterator,
            ) -> (Self, &'a mut [T]) {
                let (to_init, remainder) = self.remainder.split_at_mut(iter.len());

                for (value, to_init) in iter.into_iter().zip(to_init.iter_mut()) {
                    to_init.write(value);
                }

                // we can't use the safe (yet unstable) MaybeUninit::write_slice() here because of having an iterator to write

                let init = {
                    // SAFETY: The loop above has initialized exactly as many items as to_init is
                    // long, so it is safe to cast away the MaybeUninit<T> wrapper into T.

                    // Additional safety docs from unstable slice_assume_init_mut
                    // SAFETY: similar to safety notes for `slice_get_ref`, but we have a
                    // mutable reference which is also guaranteed to be valid for writes.
                    unsafe { mem::transmute::<&mut [MaybeUninit<T>], &mut [T]>(to_init) }
                };
                (Self { remainder }, init)
            }
        }

        let mut writes = Vec::with_capacity(desc.entries.len());
        let mut buffer_infos = Vec::with_capacity(desc.buffers.len());
        let mut buffer_infos = ExtendStack::from_vec_capacity(&mut buffer_infos);
        let mut image_infos = Vec::with_capacity(desc.samplers.len() + desc.textures.len());
        let mut image_infos = ExtendStack::from_vec_capacity(&mut image_infos);
        // TODO: This length could be reduced to just the number of top-level acceleration
        // structure bindings, where multiple consecutive TLAS bindings that are set via
        // one `WriteDescriptorSet` count towards one "info" struct, not the total number of
        // acceleration structure bindings to write:
        let mut acceleration_structure_infos =
            Vec::with_capacity(desc.acceleration_structures.len());
        let mut acceleration_structure_infos =
            ExtendStack::from_vec_capacity(&mut acceleration_structure_infos);
        let mut raw_acceleration_structures =
            Vec::with_capacity(desc.acceleration_structures.len());
        let mut raw_acceleration_structures =
            ExtendStack::from_vec_capacity(&mut raw_acceleration_structures);

        let layout_and_entry_iter = desc.entries.iter().map(|entry| {
            let layout = desc
                .layout
                .entries
                .iter()
                .find(|layout_entry| layout_entry.binding == entry.binding)
                .expect("internal error: no layout entry found with binding slot");
            (layout, entry)
        });
        let mut next_binding = 0;
        for (layout, entry) in layout_and_entry_iter {
            let write = vk::WriteDescriptorSet::default().dst_set(*set.raw());

            match layout.ty {
                wgt::BindingType::Sampler(_) => {
                    let start = entry.resource_index;
                    let end = start + entry.count;
                    let local_image_infos;
                    (image_infos, local_image_infos) =
                        image_infos.extend(desc.samplers[start as usize..end as usize].iter().map(
                            |sampler| vk::DescriptorImageInfo::default().sampler(sampler.raw),
                        ));
                    writes.push(
                        write
                            .dst_binding(next_binding)
                            .descriptor_type(conv::map_binding_type(layout.ty))
                            .image_info(local_image_infos),
                    );
                    next_binding += 1;
                }
                wgt::BindingType::Texture { .. } | wgt::BindingType::StorageTexture { .. } => {
                    let start = entry.resource_index;
                    let end = start + entry.count;
                    let local_image_infos;
                    (image_infos, local_image_infos) =
                        image_infos.extend(desc.textures[start as usize..end as usize].iter().map(
                            |binding| {
                                let layout =
                                    conv::derive_image_layout(binding.usage, binding.view.format);
                                vk::DescriptorImageInfo::default()
                                    .image_view(binding.view.raw)
                                    .image_layout(layout)
                            },
                        ));
                    writes.push(
                        write
                            .dst_binding(next_binding)
                            .descriptor_type(conv::map_binding_type(layout.ty))
                            .image_info(local_image_infos),
                    );
                    next_binding += 1;
                }
                wgt::BindingType::Buffer { .. } => {
                    let start = entry.resource_index;
                    let end = start + entry.count;
                    let local_buffer_infos;
                    (buffer_infos, local_buffer_infos) =
                        buffer_infos.extend(desc.buffers[start as usize..end as usize].iter().map(
                            |binding| {
                                vk::DescriptorBufferInfo::default()
                                    .buffer(binding.buffer.raw)
                                    .offset(binding.offset)
                                    .range(
                                        binding.size.map_or(vk::WHOLE_SIZE, wgt::BufferSize::get),
                                    )
                            },
                        ));
                    writes.push(
                        write
                            .dst_binding(next_binding)
                            .descriptor_type(conv::map_binding_type(layout.ty))
                            .buffer_info(local_buffer_infos),
                    );
                    next_binding += 1;
                }
                wgt::BindingType::AccelerationStructure { .. } => {
                    let start = entry.resource_index;
                    let end = start + entry.count;

                    let local_raw_acceleration_structures;
                    (
                        raw_acceleration_structures,
                        local_raw_acceleration_structures,
                    ) = raw_acceleration_structures.extend(
                        desc.acceleration_structures[start as usize..end as usize]
                            .iter()
                            .map(|acceleration_structure| acceleration_structure.raw),
                    );

                    let local_acceleration_structure_infos;
                    (
                        acceleration_structure_infos,
                        local_acceleration_structure_infos,
                    ) = acceleration_structure_infos.extend_one(
                        vk::WriteDescriptorSetAccelerationStructureKHR::default()
                            .acceleration_structures(local_raw_acceleration_structures),
                    );

                    writes.push(
                        write
                            .dst_binding(next_binding)
                            .descriptor_type(conv::map_binding_type(layout.ty))
                            .descriptor_count(entry.count)
                            .push_next(local_acceleration_structure_infos),
                    );
                    next_binding += 1;
                }
                wgt::BindingType::ExternalTexture => unimplemented!(),
            }
        }

        unsafe { self.shared.raw.update_descriptor_sets(&writes, &[]) };

        self.counters.bind_groups.add(1);

        Ok(super::BindGroup { set })
    }

    unsafe fn destroy_bind_group(&self, group: super::BindGroup) {
        unsafe {
            self.desc_allocator
                .lock()
                .free(&*self.shared, Some(group.set))
        };

        self.counters.bind_groups.sub(1);
    }

    unsafe fn create_shader_module(
        &self,
        desc: &crate::ShaderModuleDescriptor,
        shader: crate::ShaderInput,
    ) -> Result<super::ShaderModule, crate::ShaderError> {
        let shader_module = match shader {
            crate::ShaderInput::Naga(naga_shader)
                if self
                    .shared
                    .workarounds
                    .contains(super::Workarounds::SEPARATE_ENTRY_POINTS)
                    || !naga_shader.module.overrides.is_empty() =>
            {
                super::ShaderModule::Intermediate {
                    naga_shader,
                    runtime_checks: desc.runtime_checks,
                }
            }
            crate::ShaderInput::Naga(naga_shader) => {
                let mut naga_options = self.naga_options.clone();
                naga_options.debug_info =
                    naga_shader
                        .debug_source
                        .as_ref()
                        .map(|d| naga::back::spv::DebugInfo {
                            source_code: d.source_code.as_ref(),
                            file_name: d.file_name.as_ref(),
                            language: naga::back::spv::SourceLanguage::WGSL,
                        });
                if !desc.runtime_checks.bounds_checks {
                    naga_options.bounds_check_policies = naga::proc::BoundsCheckPolicies {
                        index: naga::proc::BoundsCheckPolicy::Unchecked,
                        buffer: naga::proc::BoundsCheckPolicy::Unchecked,
                        image_load: naga::proc::BoundsCheckPolicy::Unchecked,
                        binding_array: naga::proc::BoundsCheckPolicy::Unchecked,
                    };
                }
                let spv = naga::back::spv::write_vec(
                    &naga_shader.module,
                    &naga_shader.info,
                    &naga_options,
                    None,
                )
                .map_err(|e| crate::ShaderError::Compilation(format!("{e}")))?;
                super::ShaderModule::Raw(self.create_shader_module_impl(&spv, &desc.label)?)
            }
            crate::ShaderInput::SpirV(data) => {
                super::ShaderModule::Raw(self.create_shader_module_impl(data, &desc.label)?)
            }
            crate::ShaderInput::MetalLib { .. }
            | crate::ShaderInput::Msl { .. }
            | crate::ShaderInput::Dxil { .. }
            | crate::ShaderInput::Hlsl { .. }
            | crate::ShaderInput::Glsl { .. } => unreachable!(),
        };

        self.counters.shader_modules.add(1);

        Ok(shader_module)
    }

    unsafe fn destroy_shader_module(&self, module: super::ShaderModule) {
        match module {
            super::ShaderModule::Raw(raw) => {
                unsafe { self.shared.raw.destroy_shader_module(raw, None) };
            }
            super::ShaderModule::Intermediate { .. } => {}
        }

        self.counters.shader_modules.sub(1);
    }

    unsafe fn create_render_pipeline(
        &self,
        desc: &crate::RenderPipelineDescriptor<
            super::PipelineLayout,
            super::ShaderModule,
            super::PipelineCache,
        >,
    ) -> Result<super::RenderPipeline, crate::PipelineError> {
        let dynamic_states = [
            vk::DynamicState::VIEWPORT,
            vk::DynamicState::SCISSOR,
            vk::DynamicState::BLEND_CONSTANTS,
            vk::DynamicState::STENCIL_REFERENCE,
        ];
        let mut compatible_rp_key = super::RenderPassKey {
            sample_count: desc.multisample.count,
            multiview_mask: desc.multiview_mask,
            ..Default::default()
        };
        let mut stages = ArrayVec::<_, { crate::MAX_CONCURRENT_SHADER_STAGES }>::new();
        let mut vertex_buffers = Vec::new();
        let mut vertex_attributes = Vec::new();

        if let crate::VertexProcessor::Standard {
            vertex_buffers: desc_vertex_buffers,
            vertex_stage: _,
        } = &desc.vertex_processor
        {
            vertex_buffers = Vec::with_capacity(desc_vertex_buffers.len());
            for (i, vb) in desc_vertex_buffers.iter().enumerate() {
                vertex_buffers.push(vk::VertexInputBindingDescription {
                    binding: i as u32,
                    stride: vb.array_stride as u32,
                    input_rate: match vb.step_mode {
                        wgt::VertexStepMode::Vertex => vk::VertexInputRate::VERTEX,
                        wgt::VertexStepMode::Instance => vk::VertexInputRate::INSTANCE,
                    },
                });
                for at in vb.attributes {
                    vertex_attributes.push(vk::VertexInputAttributeDescription {
                        location: at.shader_location,
                        binding: i as u32,
                        format: conv::map_vertex_format(at.format),
                        offset: at.offset as u32,
                    });
                }
            }
        }

        let vk_vertex_input = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&vertex_buffers)
            .vertex_attribute_descriptions(&vertex_attributes);

        let vk_input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(conv::map_topology(desc.primitive.topology))
            .primitive_restart_enable(desc.primitive.strip_index_format.is_some());

        let mut compiled_vs = None;
        let mut compiled_ms = None;
        let mut compiled_ts = None;
        match &desc.vertex_processor {
            crate::VertexProcessor::Standard {
                vertex_buffers: _,
                vertex_stage,
            } => {
                compiled_vs = Some(self.compile_stage(
                    vertex_stage,
                    naga::ShaderStage::Vertex,
                    &desc.layout.binding_map,
                )?);
                stages.push(compiled_vs.as_ref().unwrap().create_info);
            }
            crate::VertexProcessor::Mesh {
                task_stage,
                mesh_stage,
            } => {
                if let Some(t) = task_stage.as_ref() {
                    compiled_ts = Some(self.compile_stage(
                        t,
                        naga::ShaderStage::Task,
                        &desc.layout.binding_map,
                    )?);
                    stages.push(compiled_ts.as_ref().unwrap().create_info);
                }
                compiled_ms = Some(self.compile_stage(
                    mesh_stage,
                    naga::ShaderStage::Mesh,
                    &desc.layout.binding_map,
                )?);
                stages.push(compiled_ms.as_ref().unwrap().create_info);
            }
        }
        let compiled_fs = match desc.fragment_stage {
            Some(ref stage) => {
                let compiled = self.compile_stage(
                    stage,
                    naga::ShaderStage::Fragment,
                    &desc.layout.binding_map,
                )?;
                stages.push(compiled.create_info);
                Some(compiled)
            }
            None => None,
        };

        let mut vk_rasterization = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(conv::map_polygon_mode(desc.primitive.polygon_mode))
            .front_face(conv::map_front_face(desc.primitive.front_face))
            .line_width(1.0)
            .depth_clamp_enable(desc.primitive.unclipped_depth);
        if let Some(face) = desc.primitive.cull_mode {
            vk_rasterization = vk_rasterization.cull_mode(conv::map_cull_face(face))
        }
        let mut vk_rasterization_conservative_state =
            vk::PipelineRasterizationConservativeStateCreateInfoEXT::default()
                .conservative_rasterization_mode(
                    vk::ConservativeRasterizationModeEXT::OVERESTIMATE,
                );
        if desc.primitive.conservative {
            vk_rasterization = vk_rasterization.push_next(&mut vk_rasterization_conservative_state);
        }

        let mut vk_depth_stencil = vk::PipelineDepthStencilStateCreateInfo::default();
        if let Some(ref ds) = desc.depth_stencil {
            let vk_format = self.shared.private_caps.map_texture_format(ds.format);
            let vk_layout = if ds.is_read_only(desc.primitive.cull_mode) {
                vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL
            } else {
                vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL
            };
            compatible_rp_key.depth_stencil = Some(super::DepthStencilAttachmentKey {
                base: super::AttachmentKey::compatible(vk_format, vk_layout),
                stencil_ops: crate::AttachmentOps::all(),
            });

            if ds.is_depth_enabled() {
                vk_depth_stencil = vk_depth_stencil
                    .depth_test_enable(true)
                    .depth_write_enable(ds.depth_write_enabled.unwrap_or_default())
                    .depth_compare_op(conv::map_comparison(ds.depth_compare.unwrap_or_default()));
            }
            if ds.stencil.is_enabled() {
                let s = &ds.stencil;
                let front = conv::map_stencil_face(&s.front, s.read_mask, s.write_mask);
                let back = conv::map_stencil_face(&s.back, s.read_mask, s.write_mask);
                vk_depth_stencil = vk_depth_stencil
                    .stencil_test_enable(true)
                    .front(front)
                    .back(back);
            }

            if ds.bias.is_enabled() {
                vk_rasterization = vk_rasterization
                    .depth_bias_enable(true)
                    .depth_bias_constant_factor(ds.bias.constant as f32)
                    .depth_bias_clamp(ds.bias.clamp)
                    .depth_bias_slope_factor(ds.bias.slope_scale);
            }
        }

        let vk_viewport = vk::PipelineViewportStateCreateInfo::default()
            .flags(vk::PipelineViewportStateCreateFlags::empty())
            .scissor_count(1)
            .viewport_count(1);

        let vk_sample_mask = [
            desc.multisample.mask as u32,
            (desc.multisample.mask >> 32) as u32,
        ];
        let vk_multisample = vk::PipelineMultisampleStateCreateInfo::default()
            .rasterization_samples(vk::SampleCountFlags::from_raw(desc.multisample.count))
            .alpha_to_coverage_enable(desc.multisample.alpha_to_coverage_enabled)
            .sample_mask(&vk_sample_mask);

        let mut vk_attachments = Vec::with_capacity(desc.color_targets.len());
        for cat in desc.color_targets {
            let (key, attarchment) = if let Some(cat) = cat.as_ref() {
                let mut vk_attachment = vk::PipelineColorBlendAttachmentState::default()
                    .color_write_mask(vk::ColorComponentFlags::from_raw(cat.write_mask.bits()));
                if let Some(ref blend) = cat.blend {
                    let (color_op, color_src, color_dst) = conv::map_blend_component(&blend.color);
                    let (alpha_op, alpha_src, alpha_dst) = conv::map_blend_component(&blend.alpha);
                    vk_attachment = vk_attachment
                        .blend_enable(true)
                        .color_blend_op(color_op)
                        .src_color_blend_factor(color_src)
                        .dst_color_blend_factor(color_dst)
                        .alpha_blend_op(alpha_op)
                        .src_alpha_blend_factor(alpha_src)
                        .dst_alpha_blend_factor(alpha_dst);
                }

                let vk_format = self.shared.private_caps.map_texture_format(cat.format);
                (
                    Some(super::ColorAttachmentKey {
                        base: super::AttachmentKey::compatible(
                            vk_format,
                            vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                        ),
                        resolve: None,
                    }),
                    vk_attachment,
                )
            } else {
                (None, vk::PipelineColorBlendAttachmentState::default())
            };

            compatible_rp_key.colors.push(key);
            vk_attachments.push(attarchment);
        }

        let vk_color_blend =
            vk::PipelineColorBlendStateCreateInfo::default().attachments(&vk_attachments);

        let vk_dynamic_state =
            vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_states);

        let raw_pass = self.shared.make_render_pass(compatible_rp_key)?;

        let vk_infos = [{
            vk::GraphicsPipelineCreateInfo::default()
                .layout(desc.layout.raw)
                .stages(&stages)
                .vertex_input_state(&vk_vertex_input)
                .input_assembly_state(&vk_input_assembly)
                .rasterization_state(&vk_rasterization)
                .viewport_state(&vk_viewport)
                .multisample_state(&vk_multisample)
                .depth_stencil_state(&vk_depth_stencil)
                .color_blend_state(&vk_color_blend)
                .dynamic_state(&vk_dynamic_state)
                .render_pass(raw_pass)
        }];

        let pipeline_cache = desc
            .cache
            .map(|it| it.raw)
            .unwrap_or(vk::PipelineCache::null());

        let mut raw_vec = {
            profiling::scope!("vkCreateGraphicsPipelines");
            unsafe {
                self.shared
                    .raw
                    .create_graphics_pipelines(pipeline_cache, &vk_infos, None)
                    .map_err(|(_, e)| super::map_pipeline_err(e))
            }?
        };

        let raw = raw_vec.pop().unwrap();
        if let Some(label) = desc.label {
            unsafe { self.shared.set_object_name(raw, label) };
        }

        if let Some(CompiledStage {
            temp_raw_module: Some(raw_module),
            ..
        }) = compiled_vs
        {
            unsafe { self.shared.raw.destroy_shader_module(raw_module, None) };
        }
        if let Some(CompiledStage {
            temp_raw_module: Some(raw_module),
            ..
        }) = compiled_ts
        {
            unsafe { self.shared.raw.destroy_shader_module(raw_module, None) };
        }
        if let Some(CompiledStage {
            temp_raw_module: Some(raw_module),
            ..
        }) = compiled_ms
        {
            unsafe { self.shared.raw.destroy_shader_module(raw_module, None) };
        }
        if let Some(CompiledStage {
            temp_raw_module: Some(raw_module),
            ..
        }) = compiled_fs
        {
            unsafe { self.shared.raw.destroy_shader_module(raw_module, None) };
        }

        self.counters.render_pipelines.add(1);

        Ok(super::RenderPipeline {
            raw,
            is_multiview: desc.multiview_mask.is_some(),
        })
    }

    unsafe fn destroy_render_pipeline(&self, pipeline: super::RenderPipeline) {
        unsafe { self.shared.raw.destroy_pipeline(pipeline.raw, None) };

        self.counters.render_pipelines.sub(1);
    }

    unsafe fn create_compute_pipeline(
        &self,
        desc: &crate::ComputePipelineDescriptor<
            super::PipelineLayout,
            super::ShaderModule,
            super::PipelineCache,
        >,
    ) -> Result<super::ComputePipeline, crate::PipelineError> {
        let compiled = self.compile_stage(
            &desc.stage,
            naga::ShaderStage::Compute,
            &desc.layout.binding_map,
        )?;

        let vk_infos = [{
            vk::ComputePipelineCreateInfo::default()
                .layout(desc.layout.raw)
                .stage(compiled.create_info)
        }];

        let pipeline_cache = desc
            .cache
            .map(|it| it.raw)
            .unwrap_or(vk::PipelineCache::null());

        let mut raw_vec = {
            profiling::scope!("vkCreateComputePipelines");
            unsafe {
                self.shared
                    .raw
                    .create_compute_pipelines(pipeline_cache, &vk_infos, None)
                    .map_err(|(_, e)| super::map_pipeline_err(e))
            }?
        };

        let raw = raw_vec.pop().unwrap();
        if let Some(label) = desc.label {
            unsafe { self.shared.set_object_name(raw, label) };
        }

        if let Some(raw_module) = compiled.temp_raw_module {
            unsafe { self.shared.raw.destroy_shader_module(raw_module, None) };
        }

        self.counters.compute_pipelines.add(1);

        Ok(super::ComputePipeline { raw })
    }

    unsafe fn destroy_compute_pipeline(&self, pipeline: super::ComputePipeline) {
        unsafe { self.shared.raw.destroy_pipeline(pipeline.raw, None) };

        self.counters.compute_pipelines.sub(1);
    }

    unsafe fn create_pipeline_cache(
        &self,
        desc: &crate::PipelineCacheDescriptor<'_>,
    ) -> Result<super::PipelineCache, crate::PipelineCacheError> {
        let mut info = vk::PipelineCacheCreateInfo::default();
        if let Some(data) = desc.data {
            info = info.initial_data(data)
        }
        profiling::scope!("vkCreatePipelineCache");
        let raw = unsafe { self.shared.raw.create_pipeline_cache(&info, None) }
            .map_err(super::map_host_device_oom_err)?;

        Ok(super::PipelineCache { raw })
    }
    fn pipeline_cache_validation_key(&self) -> Option<[u8; 16]> {
        Some(self.shared.pipeline_cache_validation_key)
    }
    unsafe fn destroy_pipeline_cache(&self, cache: super::PipelineCache) {
        unsafe { self.shared.raw.destroy_pipeline_cache(cache.raw, None) }
    }
    unsafe fn create_query_set(
        &self,
        desc: &wgt::QuerySetDescriptor<crate::Label>,
    ) -> Result<super::QuerySet, crate::DeviceError> {
        // Assume each query is 256 bytes.
        // On an AMD W6800 with driver version 32.0.12030.9, occlusion queries are 256.
        self.error_if_would_oom_on_resource_allocation(true, desc.count as u64 * 256)?;

        let (vk_type, pipeline_statistics) = match desc.ty {
            wgt::QueryType::Occlusion => (
                vk::QueryType::OCCLUSION,
                vk::QueryPipelineStatisticFlags::empty(),
            ),
            wgt::QueryType::PipelineStatistics(statistics) => (
                vk::QueryType::PIPELINE_STATISTICS,
                conv::map_pipeline_statistics(statistics),
            ),
            wgt::QueryType::Timestamp => (
                vk::QueryType::TIMESTAMP,
                vk::QueryPipelineStatisticFlags::empty(),
            ),
        };

        let vk_info = vk::QueryPoolCreateInfo::default()
            .query_type(vk_type)
            .query_count(desc.count)
            .pipeline_statistics(pipeline_statistics);

        let raw = unsafe { self.shared.raw.create_query_pool(&vk_info, None) }
            .map_err(super::map_host_device_oom_err)?;
        if let Some(label) = desc.label {
            unsafe { self.shared.set_object_name(raw, label) };
        }

        self.counters.query_sets.add(1);

        Ok(super::QuerySet { raw })
    }

    unsafe fn destroy_query_set(&self, set: super::QuerySet) {
        unsafe { self.shared.raw.destroy_query_pool(set.raw, None) };

        self.counters.query_sets.sub(1);
    }

    unsafe fn create_fence(&self) -> Result<super::Fence, crate::DeviceError> {
        self.counters.fences.add(1);

        Ok(if self.shared.private_caps.timeline_semaphores {
            let mut sem_type_info =
                vk::SemaphoreTypeCreateInfo::default().semaphore_type(vk::SemaphoreType::TIMELINE);
            let vk_info = vk::SemaphoreCreateInfo::default().push_next(&mut sem_type_info);
            let raw = unsafe { self.shared.raw.create_semaphore(&vk_info, None) }
                .map_err(super::map_host_device_oom_err)?;

            super::Fence::TimelineSemaphore(raw)
        } else {
            super::Fence::FencePool {
                last_completed: 0,
                active: Vec::new(),
                free: Vec::new(),
            }
        })
    }
    unsafe fn destroy_fence(&self, fence: super::Fence) {
        match fence {
            super::Fence::TimelineSemaphore(raw) => {
                unsafe { self.shared.raw.destroy_semaphore(raw, None) };
            }
            super::Fence::FencePool {
                active,
                free,
                last_completed: _,
            } => {
                for (_, raw) in active {
                    unsafe { self.shared.raw.destroy_fence(raw, None) };
                }
                for raw in free {
                    unsafe { self.shared.raw.destroy_fence(raw, None) };
                }
            }
        }

        self.counters.fences.sub(1);
    }
    unsafe fn get_fence_value(
        &self,
        fence: &super::Fence,
    ) -> Result<crate::FenceValue, crate::DeviceError> {
        fence.get_latest(
            &self.shared.raw,
            self.shared.extension_fns.timeline_semaphore.as_ref(),
        )
    }
    unsafe fn wait(
        &self,
        fence: &super::Fence,
        wait_value: crate::FenceValue,
        timeout: Option<Duration>,
    ) -> Result<bool, crate::DeviceError> {
        let timeout_ns = timeout
            .unwrap_or(Duration::MAX)
            .as_nanos()
            .min(u64::MAX as _) as u64;
        self.shared.wait_for_fence(fence, wait_value, timeout_ns)
    }

    unsafe fn start_graphics_debugger_capture(&self) -> bool {
        #[cfg(feature = "renderdoc")]
        {
            // Renderdoc requires us to give us the pointer that vkInstance _points to_.
            let raw_vk_instance =
                vk::Handle::as_raw(self.shared.instance.raw.handle()) as *mut *mut _;
            let raw_vk_instance_dispatch_table = unsafe { *raw_vk_instance };
            unsafe {
                self.render_doc
                    .start_frame_capture(raw_vk_instance_dispatch_table, ptr::null_mut())
            }
        }
        #[cfg(not(feature = "renderdoc"))]
        false
    }
    unsafe fn stop_graphics_debugger_capture(&self) {
        #[cfg(feature = "renderdoc")]
        {
            // Renderdoc requires us to give us the pointer that vkInstance _points to_.
            let raw_vk_instance =
                vk::Handle::as_raw(self.shared.instance.raw.handle()) as *mut *mut _;
            let raw_vk_instance_dispatch_table = unsafe { *raw_vk_instance };

            unsafe {
                self.render_doc
                    .end_frame_capture(raw_vk_instance_dispatch_table, ptr::null_mut())
            }
        }
    }

    unsafe fn pipeline_cache_get_data(&self, cache: &super::PipelineCache) -> Option<Vec<u8>> {
        let data = unsafe { self.raw_device().get_pipeline_cache_data(cache.raw) };
        data.ok()
    }

    unsafe fn get_acceleration_structure_build_sizes<'a>(
        &self,
        desc: &crate::GetAccelerationStructureBuildSizesDescriptor<'a, super::Buffer>,
    ) -> crate::AccelerationStructureBuildSizes {
        const CAPACITY: usize = 8;

        let ray_tracing_functions = self
            .shared
            .extension_fns
            .ray_tracing
            .as_ref()
            .expect("Feature `RAY_TRACING` not enabled");

        let (geometries, primitive_counts) = match *desc.entries {
            crate::AccelerationStructureEntries::Instances(ref instances) => {
                let instance_data = vk::AccelerationStructureGeometryInstancesDataKHR::default();

                let geometry = vk::AccelerationStructureGeometryKHR::default()
                    .geometry_type(vk::GeometryTypeKHR::INSTANCES)
                    .geometry(vk::AccelerationStructureGeometryDataKHR {
                        instances: instance_data,
                    });

                (
                    smallvec::smallvec![geometry],
                    smallvec::smallvec![instances.count],
                )
            }
            crate::AccelerationStructureEntries::Triangles(ref in_geometries) => {
                let mut primitive_counts =
                    smallvec::SmallVec::<[u32; CAPACITY]>::with_capacity(in_geometries.len());
                let mut geometries = smallvec::SmallVec::<
                    [vk::AccelerationStructureGeometryKHR; CAPACITY],
                >::with_capacity(in_geometries.len());

                for triangles in in_geometries {
                    let mut triangle_data =
                        vk::AccelerationStructureGeometryTrianglesDataKHR::default()
                            .index_type(vk::IndexType::NONE_KHR)
                            .vertex_format(conv::map_vertex_format(triangles.vertex_format))
                            .max_vertex(triangles.vertex_count)
                            .vertex_stride(triangles.vertex_stride)
                            // The vulkan spec suggests we could pass a non-zero invalid address here if fetching
                            // the real address has significant overhead, but we pass the real one to be on the
                            // safe side for now.
                            // from https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetAccelerationStructureBuildSizesKHR.html
                            // > The srcAccelerationStructure, dstAccelerationStructure, and mode members
                            // > of pBuildInfo are ignored. Any VkDeviceOrHostAddressKHR or VkDeviceOrHostAddressConstKHR
                            // > members of pBuildInfo are ignored by this command, except that the hostAddress
                            // > member of VkAccelerationStructureGeometryTrianglesDataKHR::transformData will
                            // > be examined to check if it is NULL.
                            .transform_data(vk::DeviceOrHostAddressConstKHR {
                                device_address: if desc
                                    .flags
                                    .contains(wgt::AccelerationStructureFlags::USE_TRANSFORM)
                                {
                                    unsafe {
                                        ray_tracing_functions
                                            .buffer_device_address
                                            .get_buffer_device_address(
                                                &vk::BufferDeviceAddressInfo::default().buffer(
                                                    triangles
                                                        .transform
                                                        .as_ref()
                                                        .unwrap()
                                                        .buffer
                                                        .raw,
                                                ),
                                            )
                                    }
                                } else {
                                    0
                                },
                            });

                    let pritive_count = if let Some(ref indices) = triangles.indices {
                        triangle_data =
                            triangle_data.index_type(conv::map_index_format(indices.format));
                        indices.count / 3
                    } else {
                        triangles.vertex_count / 3
                    };

                    let geometry = vk::AccelerationStructureGeometryKHR::default()
                        .geometry_type(vk::GeometryTypeKHR::TRIANGLES)
                        .geometry(vk::AccelerationStructureGeometryDataKHR {
                            triangles: triangle_data,
                        })
                        .flags(conv::map_acceleration_structure_geometry_flags(
                            triangles.flags,
                        ));

                    geometries.push(geometry);
                    primitive_counts.push(pritive_count);
                }
                (geometries, primitive_counts)
            }
            crate::AccelerationStructureEntries::AABBs(ref in_geometries) => {
                let mut primitive_counts =
                    smallvec::SmallVec::<[u32; CAPACITY]>::with_capacity(in_geometries.len());
                let mut geometries = smallvec::SmallVec::<
                    [vk::AccelerationStructureGeometryKHR; CAPACITY],
                >::with_capacity(in_geometries.len());
                for aabb in in_geometries {
                    let aabbs_data = vk::AccelerationStructureGeometryAabbsDataKHR::default()
                        .stride(aabb.stride);

                    let geometry = vk::AccelerationStructureGeometryKHR::default()
                        .geometry_type(vk::GeometryTypeKHR::AABBS)
                        .geometry(vk::AccelerationStructureGeometryDataKHR { aabbs: aabbs_data })
                        .flags(conv::map_acceleration_structure_geometry_flags(aabb.flags));

                    geometries.push(geometry);
                    primitive_counts.push(aabb.count);
                }
                (geometries, primitive_counts)
            }
        };

        let ty = match *desc.entries {
            crate::AccelerationStructureEntries::Instances(_) => {
                vk::AccelerationStructureTypeKHR::TOP_LEVEL
            }
            _ => vk::AccelerationStructureTypeKHR::BOTTOM_LEVEL,
        };

        let geometry_info = vk::AccelerationStructureBuildGeometryInfoKHR::default()
            .ty(ty)
            .flags(conv::map_acceleration_structure_flags(desc.flags))
            .geometries(&geometries);

        let mut raw = Default::default();
        unsafe {
            ray_tracing_functions
                .acceleration_structure
                .get_acceleration_structure_build_sizes(
                    vk::AccelerationStructureBuildTypeKHR::DEVICE,
                    &geometry_info,
                    &primitive_counts,
                    &mut raw,
                )
        }

        crate::AccelerationStructureBuildSizes {
            acceleration_structure_size: raw.acceleration_structure_size,
            update_scratch_size: raw.update_scratch_size,
            build_scratch_size: raw.build_scratch_size,
        }
    }

    unsafe fn get_acceleration_structure_device_address(
        &self,
        acceleration_structure: &super::AccelerationStructure,
    ) -> wgt::BufferAddress {
        let ray_tracing_functions = self
            .shared
            .extension_fns
            .ray_tracing
            .as_ref()
            .expect("Feature `RAY_TRACING` not enabled");

        unsafe {
            ray_tracing_functions
                .acceleration_structure
                .get_acceleration_structure_device_address(
                    &vk::AccelerationStructureDeviceAddressInfoKHR::default()
                        .acceleration_structure(acceleration_structure.raw),
                )
        }
    }

    unsafe fn create_acceleration_structure(
        &self,
        desc: &crate::AccelerationStructureDescriptor,
    ) -> Result<super::AccelerationStructure, crate::DeviceError> {
        let ray_tracing_functions = self
            .shared
            .extension_fns
            .ray_tracing
            .as_ref()
            .expect("Feature `RAY_TRACING` not enabled");

        let vk_buffer_info = vk::BufferCreateInfo::default()
            .size(desc.size)
            .usage(
                vk::BufferUsageFlags::ACCELERATION_STRUCTURE_STORAGE_KHR
                    | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            )
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        unsafe {
            let raw_buffer = self
                .shared
                .raw
                .create_buffer(&vk_buffer_info, None)
                .map_err(super::map_host_device_oom_and_ioca_err)?;

            let requirements = self.shared.raw.get_buffer_memory_requirements(raw_buffer);

            self.error_if_would_oom_on_resource_allocation(false, requirements.size)
                .inspect_err(|_| {
                    self.shared.raw.destroy_buffer(raw_buffer, None);
                })?;

            let name = desc
                .label
                .unwrap_or("Unlabeled acceleration structure buffer");

            let allocation = self
                .mem_allocator
                .lock()
                .allocate(&gpu_allocator::vulkan::AllocationCreateDesc {
                    name,
                    requirements,
                    location: gpu_allocator::MemoryLocation::GpuOnly,
                    linear: true, // Buffers are always linear
                    allocation_scheme: gpu_allocator::vulkan::AllocationScheme::GpuAllocatorManaged,
                })
                .inspect_err(|_| {
                    self.shared.raw.destroy_buffer(raw_buffer, None);
                })?;

            self.shared
                .raw
                .bind_buffer_memory(raw_buffer, allocation.memory(), allocation.offset())
                .map_err(super::map_host_device_oom_and_ioca_err)
                .inspect_err(|_| {
                    self.shared.raw.destroy_buffer(raw_buffer, None);
                })?;

            if let Some(label) = desc.label {
                self.shared.set_object_name(raw_buffer, label);
            }

            let vk_info = vk::AccelerationStructureCreateInfoKHR::default()
                .buffer(raw_buffer)
                .offset(0)
                .size(desc.size)
                .ty(conv::map_acceleration_structure_format(desc.format));

            let raw_acceleration_structure = ray_tracing_functions
                .acceleration_structure
                .create_acceleration_structure(&vk_info, None)
                .map_err(super::map_host_oom_and_ioca_err)
                .inspect_err(|_| {
                    self.shared.raw.destroy_buffer(raw_buffer, None);
                })?;

            if let Some(label) = desc.label {
                self.shared
                    .set_object_name(raw_acceleration_structure, label);
            }

            let pool = if desc.allow_compaction {
                let vk_info = vk::QueryPoolCreateInfo::default()
                    .query_type(vk::QueryType::ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR)
                    .query_count(1);

                let raw = self
                    .shared
                    .raw
                    .create_query_pool(&vk_info, None)
                    .map_err(super::map_host_device_oom_err)
                    .inspect_err(|_| {
                        ray_tracing_functions
                            .acceleration_structure
                            .destroy_acceleration_structure(raw_acceleration_structure, None);
                        self.shared.raw.destroy_buffer(raw_buffer, None);
                    })?;
                Some(raw)
            } else {
                None
            };

            Ok(super::AccelerationStructure {
                raw: raw_acceleration_structure,
                buffer: raw_buffer,
                allocation,
                compacted_size_query: pool,
            })
        }
    }

    unsafe fn destroy_acceleration_structure(
        &self,
        acceleration_structure: super::AccelerationStructure,
    ) {
        let ray_tracing_functions = self
            .shared
            .extension_fns
            .ray_tracing
            .as_ref()
            .expect("Feature `RAY_TRACING` not enabled");

        unsafe {
            ray_tracing_functions
                .acceleration_structure
                .destroy_acceleration_structure(acceleration_structure.raw, None);
            self.shared
                .raw
                .destroy_buffer(acceleration_structure.buffer, None);
            let result = self
                .mem_allocator
                .lock()
                .free(acceleration_structure.allocation);
            if let Err(err) = result {
                log::warn!("Failed to free buffer acceleration structure: {err}");
            }
            if let Some(query) = acceleration_structure.compacted_size_query {
                self.shared.raw.destroy_query_pool(query, None)
            }
        }
    }

    fn get_internal_counters(&self) -> wgt::HalCounters {
        self.counters
            .memory_allocations
            .set(self.shared.memory_allocations_counter.read());

        self.counters.as_ref().clone()
    }

    fn generate_allocator_report(&self) -> Option<wgt::AllocatorReport> {
        let gpu_allocator::AllocatorReport {
            allocations,
            blocks,
            total_allocated_bytes,
            total_capacity_bytes,
        } = self.mem_allocator.lock().generate_report();

        let allocations = allocations
            .into_iter()
            .map(|alloc| wgt::AllocationReport {
                name: alloc.name,
                offset: alloc.offset,
                size: alloc.size,
            })
            .collect();

        let blocks = blocks
            .into_iter()
            .map(|block| wgt::MemoryBlockReport {
                size: block.size,
                allocations: block.allocations.clone(),
            })
            .collect();

        Some(wgt::AllocatorReport {
            allocations,
            blocks,
            total_allocated_bytes,
            total_reserved_bytes: total_capacity_bytes,
        })
    }

    fn tlas_instance_to_bytes(&self, instance: TlasInstance) -> Vec<u8> {
        const MAX_U24: u32 = (1u32 << 24u32) - 1u32;
        let temp = RawTlasInstance {
            transform: instance.transform,
            custom_data_and_mask: (instance.custom_data & MAX_U24)
                | (u32::from(instance.mask) << 24),
            shader_binding_table_record_offset_and_flags: 0,
            acceleration_structure_reference: instance.blas_address,
        };
        bytemuck::bytes_of(&temp).to_vec()
    }

    fn check_if_oom(&self) -> Result<(), crate::DeviceError> {
        let Some(threshold) = self
            .shared
            .instance
            .memory_budget_thresholds
            .for_device_loss
        else {
            return Ok(());
        };

        if !self
            .shared
            .enabled_extensions
            .contains(&ext::memory_budget::NAME)
        {
            return Ok(());
        }

        let get_physical_device_properties = self
            .shared
            .instance
            .get_physical_device_properties
            .as_ref()
            .unwrap();

        let mut memory_budget_properties = vk::PhysicalDeviceMemoryBudgetPropertiesEXT::default();

        let mut memory_properties =
            vk::PhysicalDeviceMemoryProperties2::default().push_next(&mut memory_budget_properties);

        unsafe {
            get_physical_device_properties.get_physical_device_memory_properties2(
                self.shared.physical_device,
                &mut memory_properties,
            );
        }

        let memory_properties = memory_properties.memory_properties;

        for i in 0..memory_properties.memory_heap_count {
            let heap_usage = memory_budget_properties.heap_usage[i as usize];
            let heap_budget = memory_budget_properties.heap_budget[i as usize];

            if heap_usage >= heap_budget / 100 * threshold as u64 {
                return Err(crate::DeviceError::OutOfMemory);
            }
        }

        Ok(())
    }
}

impl super::DeviceShared {
    pub(super) fn new_binary_semaphore(
        &self,
        name: &str,
    ) -> Result<vk::Semaphore, crate::DeviceError> {
        unsafe {
            let semaphore = self
                .raw
                .create_semaphore(&vk::SemaphoreCreateInfo::default(), None)
                .map_err(super::map_host_device_oom_err)?;

            self.set_object_name(semaphore, name);

            Ok(semaphore)
        }
    }

    pub(super) fn wait_for_fence(
        &self,
        fence: &super::Fence,
        wait_value: crate::FenceValue,
        timeout_ns: u64,
    ) -> Result<bool, crate::DeviceError> {
        profiling::scope!("Device::wait");
        match *fence {
            super::Fence::TimelineSemaphore(raw) => {
                let semaphores = [raw];
                let values = [wait_value];
                let vk_info = vk::SemaphoreWaitInfo::default()
                    .semaphores(&semaphores)
                    .values(&values);
                let result = match self.extension_fns.timeline_semaphore {
                    Some(super::ExtensionFn::Extension(ref ext)) => unsafe {
                        ext.wait_semaphores(&vk_info, timeout_ns)
                    },
                    Some(super::ExtensionFn::Promoted) => unsafe {
                        self.raw.wait_semaphores(&vk_info, timeout_ns)
                    },
                    None => unreachable!(),
                };
                match result {
                    Ok(()) => Ok(true),
                    Err(vk::Result::TIMEOUT) => Ok(false),
                    Err(other) => Err(super::map_host_device_oom_and_lost_err(other)),
                }
            }
            super::Fence::FencePool {
                last_completed,
                ref active,
                free: _,
            } => {
                if wait_value <= last_completed {
                    Ok(true)
                } else {
                    match active.iter().find(|&&(value, _)| value >= wait_value) {
                        Some(&(_, raw)) => {
                            match unsafe { self.raw.wait_for_fences(&[raw], true, timeout_ns) } {
                                Ok(()) => Ok(true),
                                Err(vk::Result::TIMEOUT) => Ok(false),
                                Err(other) => Err(super::map_host_device_oom_and_lost_err(other)),
                            }
                        }
                        None => {
                            crate::hal_usage_error(format!(
                                "no signals reached value {wait_value}"
                            ));
                        }
                    }
                }
            }
        }
    }
}

impl From<gpu_descriptor::AllocationError> for crate::DeviceError {
    fn from(error: gpu_descriptor::AllocationError) -> Self {
        use gpu_descriptor::AllocationError as Ae;
        match error {
            Ae::OutOfDeviceMemory | Ae::OutOfHostMemory | Ae::Fragmentation => Self::OutOfMemory,
        }
    }
}

/// We usually map unexpected vulkan errors to the [`crate::DeviceError::Unexpected`]
/// variant to be more robust even in cases where the driver is not
/// complying with the spec.
///
/// However, we implement a few Trait methods that don't have an equivalent
/// error variant. In those cases we use this function.
fn handle_unexpected(err: vk::Result) -> ! {
    panic!("Unexpected Vulkan error: `{err}`")
}

struct ImageWithoutMemory {
    raw: vk::Image,
    requirements: vk::MemoryRequirements,
}
