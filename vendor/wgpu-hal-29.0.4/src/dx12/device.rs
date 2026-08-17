use alloc::borrow::ToOwned;
use alloc::{
    borrow::Cow,
    string::{String, ToString as _},
    sync::Arc,
    vec::Vec,
};
use arrayvec::ArrayVec;
use core::{ffi, num::NonZeroU32, ptr, time::Duration};
use std::time::Instant;

use bytemuck::TransparentWrapper;
use parking_lot::Mutex;
use windows::{
    core::Interface as _,
    Win32::{
        Foundation,
        Graphics::{Direct3D12, Dxgi},
        System::Threading,
    },
};

use super::{conv, descriptor, D3D12Lib};
use crate::{
    auxil::{
        self,
        dxgi::{name::ObjectExt as _, result::HResult as _},
    },
    dx12::{
        borrow_optional_interface_temporarily, pipeline_desc::RenderPipelineStateStreamDesc,
        shader_compilation, suballocation, DCompLib, DynamicStorageBufferOffsets, Event,
        ShaderCacheKey, ShaderCacheValue,
    },
    AccelerationStructureEntries, TlasInstance,
};

// this has to match Naga's HLSL backend, and also needs to be null-terminated
const NAGA_LOCATION_SEMANTIC: &[u8] = c"LOC".to_bytes();

impl super::Device {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        adapter: auxil::dxgi::factory::DxgiAdapter,
        raw: Direct3D12::ID3D12Device,
        present_queue: Direct3D12::ID3D12CommandQueue,
        features: wgt::Features,
        limits: &wgt::Limits,
        memory_hints: &wgt::MemoryHints,
        private_caps: super::PrivateCapabilities,
        library: &Arc<D3D12Lib>,
        dcomp_lib: &Arc<DCompLib>,
        memory_budget_thresholds: wgt::MemoryBudgetThresholds,
        compiler_container: Arc<shader_compilation::CompilerContainer>,
        backend_options: wgt::Dx12BackendOptions,
    ) -> Result<Self, crate::DeviceError> {
        if private_caps
            .instance_flags
            .contains(wgt::InstanceFlags::VALIDATION)
        {
            auxil::dxgi::exception::register_exception_handler();
        }

        let mem_allocator =
            suballocation::Allocator::new(&raw, memory_hints, memory_budget_thresholds)?;

        let idle_fence: Direct3D12::ID3D12Fence = unsafe {
            profiling::scope!("ID3D12Device::CreateFence");
            raw.CreateFence(0, Direct3D12::D3D12_FENCE_FLAG_NONE)
        }
        .into_device_result("Idle fence creation")?;

        let raw_desc = Direct3D12::D3D12_RESOURCE_DESC {
            Dimension: Direct3D12::D3D12_RESOURCE_DIMENSION_BUFFER,
            Alignment: 0,
            Width: super::ZERO_BUFFER_SIZE,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: Dxgi::Common::DXGI_FORMAT_UNKNOWN,
            SampleDesc: Dxgi::Common::DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Layout: Direct3D12::D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            Flags: Direct3D12::D3D12_RESOURCE_FLAG_NONE,
        };

        let heap_properties = Direct3D12::D3D12_HEAP_PROPERTIES {
            Type: Direct3D12::D3D12_HEAP_TYPE_CUSTOM,
            CPUPageProperty: Direct3D12::D3D12_CPU_PAGE_PROPERTY_NOT_AVAILABLE,
            MemoryPoolPreference: match private_caps.memory_architecture {
                super::MemoryArchitecture::Unified { .. } => Direct3D12::D3D12_MEMORY_POOL_L0,
                super::MemoryArchitecture::NonUnified => Direct3D12::D3D12_MEMORY_POOL_L1,
            },
            CreationNodeMask: 0,
            VisibleNodeMask: 0,
        };

        profiling::scope!("Zero Buffer Allocation");
        let mut zero_buffer = None::<Direct3D12::ID3D12Resource>;
        unsafe {
            raw.CreateCommittedResource(
                &heap_properties,
                Direct3D12::D3D12_HEAP_FLAG_NONE,
                &raw_desc,
                Direct3D12::D3D12_RESOURCE_STATE_COMMON,
                None,
                &mut zero_buffer,
            )
        }
        .into_device_result("Zero buffer creation")?;

        let zero_buffer = zero_buffer.ok_or(crate::DeviceError::Unexpected)?;

        // Note: without `D3D12_HEAP_FLAG_CREATE_NOT_ZEROED`
        // this resource is zeroed by default.

        // maximum number of CBV/SRV/UAV descriptors in heap for Tier 1
        let capacity_views = limits.max_non_sampler_bindings as u64;

        let draw_mesh = if features
            .features_wgpu
            .contains(wgt::FeaturesWGPU::EXPERIMENTAL_MESH_SHADER)
        {
            Some(Self::create_command_signature(
                &raw,
                None,
                size_of::<wgt::DispatchIndirectArgs>(),
                &[Direct3D12::D3D12_INDIRECT_ARGUMENT_DESC {
                    Type: Direct3D12::D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH_MESH,
                    ..Default::default()
                }],
                0,
            )?)
        } else {
            None
        };

        let shared = super::DeviceShared {
            adapter,
            zero_buffer,
            cmd_signatures: super::CommandSignatures {
                draw: Self::create_command_signature(
                    &raw,
                    None,
                    size_of::<wgt::DrawIndirectArgs>(),
                    &[Direct3D12::D3D12_INDIRECT_ARGUMENT_DESC {
                        Type: Direct3D12::D3D12_INDIRECT_ARGUMENT_TYPE_DRAW,
                        ..Default::default()
                    }],
                    0,
                )?,
                draw_indexed: Self::create_command_signature(
                    &raw,
                    None,
                    size_of::<wgt::DrawIndexedIndirectArgs>(),
                    &[Direct3D12::D3D12_INDIRECT_ARGUMENT_DESC {
                        Type: Direct3D12::D3D12_INDIRECT_ARGUMENT_TYPE_DRAW_INDEXED,
                        ..Default::default()
                    }],
                    0,
                )?,
                draw_mesh,
                dispatch: Self::create_command_signature(
                    &raw,
                    None,
                    size_of::<wgt::DispatchIndirectArgs>(),
                    &[Direct3D12::D3D12_INDIRECT_ARGUMENT_DESC {
                        Type: Direct3D12::D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH,
                        ..Default::default()
                    }],
                    0,
                )?,
            },
            heap_views: descriptor::GeneralHeap::new(
                &raw,
                Direct3D12::D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
                capacity_views,
            )?,
            sampler_heap: super::sampler::SamplerHeap::new(&raw, &private_caps)?,
            private_caps,
        };

        let mut rtv_pool =
            descriptor::CpuPool::new(raw.clone(), Direct3D12::D3D12_DESCRIPTOR_HEAP_TYPE_RTV);
        let null_rtv_handle = rtv_pool.alloc_handle()?;
        // A null pResource is used to initialize a null descriptor,
        // which guarantees D3D11-like null binding behavior (reading 0s, writes are discarded)
        unsafe {
            raw.CreateRenderTargetView(
                None,
                Some(&Direct3D12::D3D12_RENDER_TARGET_VIEW_DESC {
                    Format: Dxgi::Common::DXGI_FORMAT_R8G8B8A8_UNORM,
                    ViewDimension: Direct3D12::D3D12_RTV_DIMENSION_TEXTURE2D,
                    Anonymous: Direct3D12::D3D12_RENDER_TARGET_VIEW_DESC_0 {
                        Texture2D: Direct3D12::D3D12_TEX2D_RTV {
                            MipSlice: 0,
                            PlaneSlice: 0,
                        },
                    },
                }),
                null_rtv_handle.raw,
            )
        };

        Ok(super::Device {
            raw: raw.clone(),
            present_queue,
            idler: super::Idler { fence: idle_fence },
            features,
            shared: Arc::new(shared),
            rtv_pool: Arc::new(Mutex::new(rtv_pool)),
            dsv_pool: Mutex::new(descriptor::CpuPool::new(
                raw.clone(),
                Direct3D12::D3D12_DESCRIPTOR_HEAP_TYPE_DSV,
            )),
            srv_uav_pool: Mutex::new(descriptor::CpuPool::new(
                raw.clone(),
                Direct3D12::D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
            )),
            options: backend_options,
            library: Arc::clone(library),
            dcomp_lib: Arc::clone(dcomp_lib),
            #[cfg(feature = "renderdoc")]
            render_doc: Default::default(),
            null_rtv_handle,
            mem_allocator,
            compiler_container,
            shader_cache: Default::default(),
            counters: Default::default(),
        })
    }

    fn create_command_signature(
        raw: &Direct3D12::ID3D12Device,
        root_signature: Option<&Direct3D12::ID3D12RootSignature>,
        byte_stride: usize,
        arguments: &[Direct3D12::D3D12_INDIRECT_ARGUMENT_DESC],
        node_mask: u32,
    ) -> Result<Direct3D12::ID3D12CommandSignature, crate::DeviceError> {
        let mut signature = None;
        unsafe {
            raw.CreateCommandSignature(
                &Direct3D12::D3D12_COMMAND_SIGNATURE_DESC {
                    ByteStride: byte_stride as u32,
                    NumArgumentDescs: arguments.len() as u32,
                    pArgumentDescs: arguments.as_ptr(),
                    NodeMask: node_mask,
                },
                root_signature,
                &mut signature,
            )
        }
        .into_device_result("Command signature creation")?;
        signature.ok_or(crate::DeviceError::Unexpected)
    }

    // Blocks until the dedicated present queue is finished with all of its work.
    //
    // Once this method completes, the surface is able to be resized or deleted.
    pub(super) unsafe fn wait_for_present_queue_idle(&self) -> Result<(), crate::DeviceError> {
        let cur_value = unsafe { self.idler.fence.GetCompletedValue() };
        if cur_value == !0 {
            return Err(crate::DeviceError::Lost);
        }

        let event = Event::create(false, false)?;

        let value = cur_value + 1;
        unsafe { self.present_queue.Signal(&self.idler.fence, value) }
            .into_device_result("Signal")?;
        let hr = unsafe { self.idler.fence.SetEventOnCompletion(value, event.0) };
        hr.into_device_result("Set event")?;
        unsafe { Threading::WaitForSingleObject(event.0, Threading::INFINITE) };
        Ok(())
    }

    /// When generating the vertex shader, the fragment stage must be passed if it exists!
    /// Otherwise, the generated HLSL may be incorrect since the fragment shader inputs are
    /// allowed to be a subset of the vertex outputs.
    fn load_shader(
        &self,
        stage: &crate::ProgrammableStage<super::ShaderModule>,
        layout: &super::PipelineLayout,
        naga_stage: naga::ShaderStage,
        fragment_stage: Option<&crate::ProgrammableStage<super::ShaderModule>>,
    ) -> Result<super::CompiledShader, crate::PipelineError> {
        let stage_bit = auxil::map_naga_stage(naga_stage);

        let needs_temp_options = stage.zero_initialize_workgroup_memory
            != layout.naga_options.zero_initialize_workgroup_memory
            || stage.module.runtime_checks.bounds_checks != layout.naga_options.restrict_indexing
            || stage.module.runtime_checks.force_loop_bounding
                != layout.naga_options.force_loop_bounding
            || stage
                .module
                .runtime_checks
                .ray_query_initialization_tracking
                != layout.naga_options.ray_query_initialization_tracking;
        let mut temp_options;
        let naga_options = if needs_temp_options {
            temp_options = layout.naga_options.clone();
            temp_options.zero_initialize_workgroup_memory = stage.zero_initialize_workgroup_memory;
            temp_options.restrict_indexing = stage.module.runtime_checks.bounds_checks;
            temp_options.force_loop_bounding = stage.module.runtime_checks.force_loop_bounding;
            temp_options.ray_query_initialization_tracking = stage
                .module
                .runtime_checks
                .ray_query_initialization_tracking;
            &temp_options
        } else {
            &layout.naga_options
        };

        let key = match &stage.module.source {
            super::ShaderModuleSource::Naga(naga_shader) => {
                use naga::back::hlsl;

                let frag_ep = match fragment_stage {
                    Some(crate::ProgrammableStage {
                        module:
                            super::ShaderModule {
                                source: super::ShaderModuleSource::Naga(naga_shader),
                                ..
                            },
                        entry_point,
                        ..
                    }) => Some(
                        hlsl::FragmentEntryPoint::new(&naga_shader.module, entry_point).ok_or(
                            crate::PipelineError::EntryPoint(naga::ShaderStage::Fragment),
                        ),
                    ),
                    _ => None,
                }
                .transpose()?;
                let (module, info) = naga::back::pipeline_constants::process_overrides(
                    &naga_shader.module,
                    &naga_shader.info,
                    Some((naga_stage, stage.entry_point)),
                    stage.constants,
                )
                .map_err(|e| {
                    crate::PipelineError::PipelineConstants(stage_bit, format!("HLSL: {e:?}"))
                })?;

                let pipeline_options = hlsl::PipelineOptions {
                    entry_point: Some((naga_stage, stage.entry_point.to_string())),
                };

                //TODO: reuse the writer
                let (source, entry_point) = {
                    let mut source = String::new();
                    let mut writer =
                        hlsl::Writer::new(&mut source, naga_options, &pipeline_options);

                    profiling::scope!("naga::back::hlsl::write");
                    let mut reflection_info = writer
                        .write(&module, &info, frag_ep.as_ref())
                        .map_err(|e| {
                            crate::PipelineError::Linkage(stage_bit, format!("HLSL: {e:?}"))
                        })?;

                    assert_eq!(reflection_info.entry_point_names.len(), 1);

                    let entry_point = reflection_info
                        .entry_point_names
                        .pop()
                        .unwrap()
                        .map_err(|e| crate::PipelineError::Linkage(stage_bit, format!("{e}")))?;

                    (source, entry_point)
                };
                log::debug!(
                    "Naga generated shader for {entry_point:?} at {naga_stage:?}:\n{source}"
                );

                ShaderCacheKey {
                    source,
                    entry_point,
                    stage: naga_stage,
                    shader_model: naga_options.shader_model,
                }
            }
            super::ShaderModuleSource::HlslPassthrough(passthrough) => ShaderCacheKey {
                source: passthrough.shader.clone(),
                entry_point: stage.entry_point.to_string(),
                stage: naga_stage,
                shader_model: naga_options.shader_model,
            },
            super::ShaderModuleSource::DxilPassthrough(passthrough) => {
                return Ok(super::CompiledShader::Precompiled(
                    passthrough.shader.clone(),
                ))
            }
        };

        {
            let mut shader_cache = self.shader_cache.lock();
            let nr_of_shaders_compiled = shader_cache.nr_of_shaders_compiled;
            if let Some(value) = shader_cache.entries.get_mut(&key) {
                value.last_used = nr_of_shaders_compiled;
                return Ok(value.shader.clone());
            }
        }

        let source_name = stage.module.raw_name.as_deref();

        let full_stage = format!("{}_{}", naga_stage.to_hlsl_str(), key.shader_model.to_str());

        let compiled_shader = self.compiler_container.compile(
            self,
            &key.source,
            source_name,
            &key.entry_point,
            stage_bit,
            &full_stage,
        )?;

        {
            let mut shader_cache = self.shader_cache.lock();
            shader_cache.nr_of_shaders_compiled += 1;
            let nr_of_shaders_compiled = shader_cache.nr_of_shaders_compiled;
            let value = ShaderCacheValue {
                last_used: nr_of_shaders_compiled,
                shader: compiled_shader.clone(),
            };
            shader_cache.entries.insert(key, value);

            // Retain all entries that have been used since we compiled the last 100 shaders.
            if shader_cache.entries.len() > 200 {
                shader_cache
                    .entries
                    .retain(|_, v| v.last_used >= nr_of_shaders_compiled - 100);
            }
        }

        Ok(compiled_shader)
    }

    pub fn raw_device(&self) -> &Direct3D12::ID3D12Device {
        &self.raw
    }

    pub fn raw_queue(&self) -> &Direct3D12::ID3D12CommandQueue {
        &self.present_queue
    }

    pub unsafe fn texture_from_raw(
        resource: Direct3D12::ID3D12Resource,
        format: wgt::TextureFormat,
        dimension: wgt::TextureDimension,
        size: wgt::Extent3d,
        mip_level_count: u32,
        sample_count: u32,
    ) -> super::Texture {
        super::Texture {
            resource,
            format,
            dimension,
            size,
            mip_level_count,
            sample_count,
            allocation: suballocation::Allocation::none(
                suballocation::AllocationType::Texture,
                format.theoretical_memory_footprint(size),
            ),
        }
    }

    pub unsafe fn buffer_from_raw(
        resource: Direct3D12::ID3D12Resource,
        size: wgt::BufferAddress,
    ) -> super::Buffer {
        super::Buffer {
            resource,
            size,
            allocation: suballocation::Allocation::none(
                suballocation::AllocationType::Buffer,
                size,
            ),
        }
    }
}

impl crate::Device for super::Device {
    type A = super::Api;

    unsafe fn create_buffer(
        &self,
        desc: &crate::BufferDescriptor,
    ) -> Result<super::Buffer, crate::DeviceError> {
        let mut desc = desc.clone();

        if desc.usage.contains(wgt::BufferUses::UNIFORM) {
            desc.size = desc
                .size
                .next_multiple_of(Direct3D12::D3D12_CONSTANT_BUFFER_DATA_PLACEMENT_ALIGNMENT.into())
        }

        let (resource, allocation) =
            suballocation::DeviceAllocationContext::from(self).create_buffer(&desc)?;

        self.counters.buffers.add(1);

        Ok(super::Buffer {
            resource,
            size: desc.size,
            allocation,
        })
    }

    unsafe fn destroy_buffer(&self, buffer: super::Buffer) {
        suballocation::DeviceAllocationContext::from(self)
            .free_resource(buffer.resource, buffer.allocation);

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
        let mut ptr = ptr::null_mut();
        // TODO: 0 for subresource should be fine here until map and unmap buffer is subresource aware?
        unsafe { buffer.resource.Map(0, None, Some(&mut ptr)) }.into_device_result("Map buffer")?;

        Ok(crate::BufferMapping {
            ptr: ptr::NonNull::new(unsafe { ptr.offset(range.start as isize).cast::<u8>() })
                .unwrap(),
            //TODO: double-check this. Documentation is a bit misleading -
            // it implies that Map/Unmap is needed to invalidate/flush memory.
            is_coherent: true,
        })
    }

    unsafe fn unmap_buffer(&self, buffer: &super::Buffer) {
        unsafe { buffer.resource.Unmap(0, None) };
    }

    unsafe fn flush_mapped_ranges<I>(&self, _buffer: &super::Buffer, _ranges: I) {}
    unsafe fn invalidate_mapped_ranges<I>(&self, _buffer: &super::Buffer, _ranges: I) {}

    unsafe fn create_texture(
        &self,
        desc: &crate::TextureDescriptor,
    ) -> Result<super::Texture, crate::DeviceError> {
        let raw_desc = Direct3D12::D3D12_RESOURCE_DESC {
            Dimension: conv::map_texture_dimension(desc.dimension),
            Alignment: 0,
            Width: desc.size.width as u64,
            Height: desc.size.height,
            DepthOrArraySize: desc.size.depth_or_array_layers as u16,
            MipLevels: desc.mip_level_count as u16,
            Format: auxil::dxgi::conv::map_texture_format_for_resource(
                desc.format,
                desc.usage,
                !desc.view_formats.is_empty(),
                self.shared
                    .private_caps
                    .casting_fully_typed_format_supported,
            ),
            SampleDesc: Dxgi::Common::DXGI_SAMPLE_DESC {
                Count: desc.sample_count,
                Quality: 0,
            },
            Layout: Direct3D12::D3D12_TEXTURE_LAYOUT_UNKNOWN,
            Flags: conv::map_texture_usage_to_resource_flags(desc.usage),
        };

        let (resource, allocation) =
            suballocation::DeviceAllocationContext::from(self).create_texture(desc, raw_desc)?;

        self.counters.textures.add(1);

        Ok(super::Texture {
            resource,
            format: desc.format,
            dimension: desc.dimension,
            size: desc.size,
            mip_level_count: desc.mip_level_count,
            sample_count: desc.sample_count,
            allocation,
        })
    }

    unsafe fn destroy_texture(&self, texture: super::Texture) {
        suballocation::DeviceAllocationContext::from(self)
            .free_resource(texture.resource, texture.allocation);

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
        let view_desc = desc.to_internal(texture);

        self.counters.texture_views.add(1);

        Ok(super::TextureView {
            raw_format: view_desc.rtv_dsv_format,
            aspects: view_desc.aspects,
            dimension: desc.dimension,
            texture: texture.resource.clone(),
            subresource_index: texture.calc_subresource(
                desc.range.base_mip_level,
                desc.range.base_array_layer,
                0,
            ),
            mip_slice: desc.range.base_mip_level,
            handle_srv: if desc.usage.intersects(wgt::TextureUses::RESOURCE) {
                match unsafe { view_desc.to_srv() } {
                    Some(raw_desc) => {
                        let handle = self.srv_uav_pool.lock().alloc_handle()?;
                        unsafe {
                            self.raw.CreateShaderResourceView(
                                &texture.resource,
                                Some(&raw_desc),
                                handle.raw,
                            )
                        };
                        Some(handle)
                    }
                    None => None,
                }
            } else {
                None
            },
            handle_uav: if desc.usage.intersects(
                wgt::TextureUses::STORAGE_READ_ONLY
                    | wgt::TextureUses::STORAGE_WRITE_ONLY
                    | wgt::TextureUses::STORAGE_READ_WRITE,
            ) {
                match unsafe { view_desc.to_uav() } {
                    Some(raw_desc) => {
                        let handle = self.srv_uav_pool.lock().alloc_handle()?;
                        unsafe {
                            self.raw.CreateUnorderedAccessView(
                                &texture.resource,
                                None,
                                Some(&raw_desc),
                                handle.raw,
                            );
                        }
                        Some(handle)
                    }
                    None => None,
                }
            } else {
                None
            },
            handle_rtv: if desc.usage.intersects(wgt::TextureUses::COLOR_TARGET)
                && desc.dimension != wgt::TextureViewDimension::D3
            // 3D RTVs must be created in the render pass
            {
                let raw_desc = unsafe { view_desc.to_rtv() };
                let handle = self.rtv_pool.lock().alloc_handle()?;
                unsafe {
                    self.raw
                        .CreateRenderTargetView(&texture.resource, Some(&raw_desc), handle.raw)
                };
                Some(handle)
            } else {
                None
            },
            handle_dsv_ro: if desc.usage.intersects(wgt::TextureUses::DEPTH_STENCIL_READ) {
                let raw_desc = unsafe { view_desc.to_dsv(true) };
                let handle = self.dsv_pool.lock().alloc_handle()?;
                unsafe {
                    self.raw
                        .CreateDepthStencilView(&texture.resource, Some(&raw_desc), handle.raw)
                };
                Some(handle)
            } else {
                None
            },
            handle_dsv_rw: if desc.usage.intersects(wgt::TextureUses::DEPTH_STENCIL_WRITE) {
                let raw_desc = unsafe { view_desc.to_dsv(false) };
                let handle = self.dsv_pool.lock().alloc_handle()?;
                unsafe {
                    self.raw
                        .CreateDepthStencilView(&texture.resource, Some(&raw_desc), handle.raw)
                };
                Some(handle)
            } else {
                None
            },
        })
    }

    unsafe fn destroy_texture_view(&self, view: super::TextureView) {
        if view.handle_srv.is_some() || view.handle_uav.is_some() {
            let mut pool = self.srv_uav_pool.lock();
            if let Some(handle) = view.handle_srv {
                pool.free_handle(handle);
            }
            if let Some(handle) = view.handle_uav {
                pool.free_handle(handle);
            }
        }
        if let Some(handle) = view.handle_rtv {
            self.rtv_pool.lock().free_handle(handle);
        }
        if view.handle_dsv_ro.is_some() || view.handle_dsv_rw.is_some() {
            let mut pool = self.dsv_pool.lock();
            if let Some(handle) = view.handle_dsv_ro {
                pool.free_handle(handle);
            }
            if let Some(handle) = view.handle_dsv_rw {
                pool.free_handle(handle);
            }
        }

        self.counters.texture_views.sub(1);
    }

    unsafe fn create_sampler(
        &self,
        desc: &crate::SamplerDescriptor,
    ) -> Result<super::Sampler, crate::DeviceError> {
        let reduction = match desc.compare {
            Some(_) => Direct3D12::D3D12_FILTER_REDUCTION_TYPE_COMPARISON,
            None => Direct3D12::D3D12_FILTER_REDUCTION_TYPE_STANDARD,
        };
        let mut filter = Direct3D12::D3D12_FILTER(
            (conv::map_filter_mode(desc.min_filter).0 << Direct3D12::D3D12_MIN_FILTER_SHIFT)
                | (conv::map_filter_mode(desc.mag_filter).0 << Direct3D12::D3D12_MAG_FILTER_SHIFT)
                | (conv::map_mipmap_filter_mode(desc.mipmap_filter).0
                    << Direct3D12::D3D12_MIP_FILTER_SHIFT)
                | (reduction.0 << Direct3D12::D3D12_FILTER_REDUCTION_TYPE_SHIFT),
        );

        if desc.anisotropy_clamp != 1 {
            filter.0 |= Direct3D12::D3D12_FILTER_ANISOTROPIC.0;
        };

        let border_color = conv::map_border_color(desc.border_color);

        let raw_desc = Direct3D12::D3D12_SAMPLER_DESC {
            Filter: filter,
            AddressU: conv::map_address_mode(desc.address_modes[0]),
            AddressV: conv::map_address_mode(desc.address_modes[1]),
            AddressW: conv::map_address_mode(desc.address_modes[2]),
            MipLODBias: 0f32,
            MaxAnisotropy: desc.anisotropy_clamp as u32,

            ComparisonFunc: conv::map_comparison(desc.compare.unwrap_or_default()),
            BorderColor: border_color,
            MinLOD: desc.lod_clamp.start,
            MaxLOD: desc.lod_clamp.end,
        };

        let index = self
            .shared
            .sampler_heap
            .create_sampler(&self.raw, raw_desc)?;

        self.counters.samplers.add(1);

        Ok(super::Sampler {
            index,
            desc: raw_desc,
        })
    }

    unsafe fn destroy_sampler(&self, sampler: super::Sampler) {
        self.shared
            .sampler_heap
            .destroy_sampler(sampler.desc, sampler.index);
        self.counters.samplers.sub(1);
    }

    unsafe fn create_command_encoder(
        &self,
        desc: &crate::CommandEncoderDescriptor<super::Queue>,
    ) -> Result<super::CommandEncoder, crate::DeviceError> {
        let allocator: Direct3D12::ID3D12CommandAllocator = unsafe {
            self.raw
                .CreateCommandAllocator(Direct3D12::D3D12_COMMAND_LIST_TYPE_DIRECT)
        }
        .into_device_result("Command allocator creation")?;

        if let Some(label) = desc.label {
            allocator.set_name(label)?;
        }

        self.counters.command_encoders.add(1);

        Ok(super::CommandEncoder {
            allocator,
            device: self.raw.clone(),
            shared: Arc::clone(&self.shared),
            mem_allocator: self.mem_allocator.clone(),
            rtv_pool: Arc::clone(&self.rtv_pool),
            temp_rtv_handles: Vec::new(),
            intermediate_copy_bufs: Vec::new(),
            null_rtv_handle: self.null_rtv_handle,
            list: None,
            free_lists: Vec::new(),
            pass: super::PassState::new(),
            temp: super::Temp::default(),
            end_of_pass_timer_query: None,
            counters: Arc::clone(&self.counters),
        })
    }

    unsafe fn create_bind_group_layout(
        &self,
        desc: &crate::BindGroupLayoutDescriptor,
    ) -> Result<super::BindGroupLayout, crate::DeviceError> {
        let mut num_views = 0;
        let mut has_sampler_in_group = false;
        for entry in desc.entries.iter() {
            let count = entry.count.map_or(1, NonZeroU32::get);
            match entry.ty {
                wgt::BindingType::Buffer {
                    ty: wgt::BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    ..
                } => {}
                wgt::BindingType::Buffer { .. }
                | wgt::BindingType::Texture { .. }
                | wgt::BindingType::StorageTexture { .. }
                | wgt::BindingType::AccelerationStructure { .. } => num_views += count,
                wgt::BindingType::Sampler { .. } => has_sampler_in_group = true,
                // Three texture planes and one params buffer
                wgt::BindingType::ExternalTexture => num_views += 4 * count,
            }
        }

        if has_sampler_in_group {
            num_views += 1;
        }

        self.counters.bind_group_layouts.add(1);

        Ok(super::BindGroupLayout {
            entries: desc.entries.to_vec(),
            cpu_heap_views: if num_views != 0 {
                let heap = descriptor::CpuHeap::new(
                    &self.raw,
                    Direct3D12::D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
                    num_views,
                )?;
                Some(heap)
            } else {
                None
            },
            copy_counts: vec![1; num_views as usize],
        })
    }

    unsafe fn destroy_bind_group_layout(&self, _bg_layout: super::BindGroupLayout) {
        self.counters.bind_group_layouts.sub(1);
    }

    unsafe fn create_pipeline_layout(
        &self,
        desc: &crate::PipelineLayoutDescriptor<super::BindGroupLayout>,
    ) -> Result<super::PipelineLayout, crate::DeviceError> {
        use naga::back::hlsl;
        // Pipeline layouts are implemented as RootSignature for D3D12.
        //
        // Immediates are implemented as root constants.
        //
        // Each bind group layout might use one SRV/CBV/UAV descriptor table.
        // With resources in the bind group layout using:
        //  - 1 CBV per non-dynamic uniform buffer
        //  - 1 SRV per acceleration structure
        //  - 1 SRV for all samplers in a bind group
        //  - 1 SRV per texture
        //  - 1 SRV per read-only storage buffer
        //  - 1 UAV per storage texture
        //  - 1 UAV per read-write storage buffer
        //  - 3 SRVs & 1 CBV per external texture
        //
        // Each dynamic uniform buffer takes up a CBV root descriptor.
        // This is easier than trying to patch up the offset on the shader side.
        //
        // Each dynamic storage buffer is an SRV or UAV in the descriptor table
        // and its dynamic offsets are passed via root constants.
        //
        // All samplers go into a single sampler descriptor table.
        //
        // 3 additional root constants are used to populate built-in (shader) inputs.
        //
        // Root signature layout:
        // Root Constants: Parameter=0, Space=0
        //     ...
        // (bind group [0]) - Space=0
        //   View descriptor table, if any
        //   Sampler buffer descriptor table, if any
        //   Root descriptors (for dynamic offset buffers)
        // (bind group [1]) - Space=0
        // ...
        // (bind group [2]) - Space=0
        // Special constant buffer: Space=0
        // Sampler descriptor tables: Space=0
        //   SamplerState Array: Space=0, Register=0-2047
        //   SamplerComparisonState Array: Space=0, Register=2048-4095

        //TODO: put lower bind group indices further down the root signature. See:
        // https://microsoft.github.io/DirectX-Specs/d3d/ResourceBinding.html#binding-model
        // Currently impossible because wgpu-core only re-binds the descriptor sets based
        // on Vulkan-like layout compatibility rules.

        let mut binding_map = hlsl::BindingMap::default();
        let mut sampler_buffer_binding_map = hlsl::SamplerIndexBufferBindingMap::default();
        let mut external_texture_binding_map = hlsl::ExternalTextureBindingMap::default();
        let mut bind_cbv = hlsl::BindTarget::default();
        let mut bind_srv = hlsl::BindTarget::default();
        let mut bind_uav = hlsl::BindTarget::default();
        let mut parameters = Vec::new();
        let mut immediates_target = None;
        let mut root_constant_info = None;

        if desc.immediate_size != 0 {
            let parameter_index = parameters.len();
            let size = desc.immediate_size / 4;
            parameters.push(Direct3D12::D3D12_ROOT_PARAMETER {
                ParameterType: Direct3D12::D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS,
                Anonymous: Direct3D12::D3D12_ROOT_PARAMETER_0 {
                    Constants: Direct3D12::D3D12_ROOT_CONSTANTS {
                        ShaderRegister: bind_cbv.register,
                        RegisterSpace: bind_cbv.space as u32,
                        Num32BitValues: size,
                    },
                },
                ShaderVisibility: Direct3D12::D3D12_SHADER_VISIBILITY_ALL,
            });
            let binding = bind_cbv;
            bind_cbv.register += 1;
            root_constant_info = Some(super::RootConstantInfo {
                root_index: parameter_index as u32,
                range: 0..size,
            });
            immediates_target = Some(binding);

            bind_cbv.space += 1;
        }

        let mut dynamic_storage_buffer_offsets_targets = alloc::collections::BTreeMap::new();
        let mut total_dynamic_storage_buffers = 0;

        // Collect the whole number of bindings we will create upfront.
        // It allows us to preallocate enough storage to avoid reallocation,
        // which could cause invalid pointers.
        let mut total_non_dynamic_entries = 0_usize;
        let mut sampler_in_any_bind_group = false;
        for bgl in desc.bind_group_layouts {
            let Some(bgl) = bgl else {
                continue;
            };

            let mut sampler_in_bind_group = false;

            for entry in &bgl.entries {
                match entry.ty {
                    wgt::BindingType::Buffer {
                        ty: wgt::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        ..
                    } => {}
                    wgt::BindingType::Sampler(_) => sampler_in_bind_group = true,
                    // Three texture planes and one params buffer
                    wgt::BindingType::ExternalTexture => total_non_dynamic_entries += 4,
                    _ => total_non_dynamic_entries += 1,
                }
            }

            if sampler_in_bind_group {
                // One for the sampler buffer
                total_non_dynamic_entries += 1;
                sampler_in_any_bind_group = true;
            }
        }

        if sampler_in_any_bind_group {
            // Two for the sampler arrays themselves
            total_non_dynamic_entries += 2;
        }

        let mut ranges = Vec::with_capacity(total_non_dynamic_entries);

        let mut bind_group_infos = [const { None }; crate::MAX_BIND_GROUPS];
        for (index, bgl) in desc.bind_group_layouts.iter().enumerate() {
            let Some(bgl) = bgl else {
                continue;
            };

            let mut info = super::BindGroupInfo {
                tables: super::TableTypes::empty(),
                base_root_index: parameters.len() as u32,
                dynamic_storage_buffer_offsets: None,
            };

            let mut visibility_view_static = wgt::ShaderStages::empty();
            let mut visibility_view_dynamic_uniform = wgt::ShaderStages::empty();
            let mut visibility_view_dynamic_storage = wgt::ShaderStages::empty();
            for entry in bgl.entries.iter() {
                match entry.ty {
                    wgt::BindingType::Sampler { .. } => {
                        visibility_view_static |= wgt::ShaderStages::all()
                    }
                    wgt::BindingType::Buffer {
                        ty: wgt::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        ..
                    } => visibility_view_dynamic_uniform |= entry.visibility,
                    wgt::BindingType::Buffer {
                        ty: wgt::BufferBindingType::Storage { .. },
                        has_dynamic_offset: true,
                        ..
                    } => visibility_view_dynamic_storage |= entry.visibility,
                    _ => visibility_view_static |= entry.visibility,
                }
            }

            let mut dynamic_storage_buffers = 0;

            // SRV/CBV/UAV descriptor tables
            let range_base = ranges.len();
            for entry in bgl.entries.iter() {
                let count = entry.count.map_or(1, NonZeroU32::get);
                if let wgt::BindingType::ExternalTexture = entry.ty {
                    // External textures need 3 SRVs (a texture for each plane)
                    // and 1 CBV for the parameters buffer.
                    let bind_target = hlsl::ExternalTextureBindTarget {
                        planes: core::array::from_fn(|_| hlsl::BindTarget {
                            register: {
                                let register = bind_srv.register;
                                bind_srv.register += count;
                                register
                            },
                            ..bind_srv
                        }),
                        params: hlsl::BindTarget {
                            register: {
                                let register = bind_cbv.register;
                                bind_cbv.register += count;
                                register
                            },
                            ..bind_cbv
                        },
                    };
                    external_texture_binding_map.insert(
                        naga::ResourceBinding {
                            group: index as u32,
                            binding: entry.binding,
                        },
                        bind_target,
                    );
                    for bt in bind_target.planes {
                        ranges.push(Direct3D12::D3D12_DESCRIPTOR_RANGE {
                            RangeType: Direct3D12::D3D12_DESCRIPTOR_RANGE_TYPE_SRV,
                            NumDescriptors: count,
                            BaseShaderRegister: bt.register,
                            RegisterSpace: bt.space as u32,
                            OffsetInDescriptorsFromTableStart:
                                Direct3D12::D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND,
                        });
                    }
                    ranges.push(Direct3D12::D3D12_DESCRIPTOR_RANGE {
                        RangeType: Direct3D12::D3D12_DESCRIPTOR_RANGE_TYPE_CBV,
                        NumDescriptors: count,
                        BaseShaderRegister: bind_target.params.register,
                        RegisterSpace: bind_target.params.space as u32,
                        OffsetInDescriptorsFromTableStart:
                            Direct3D12::D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND,
                    });
                } else {
                    let (range_ty, has_dynamic_offset) = match entry.ty {
                        wgt::BindingType::Buffer {
                            ty,
                            has_dynamic_offset: true,
                            ..
                        } => match ty {
                            wgt::BufferBindingType::Uniform => continue,
                            wgt::BufferBindingType::Storage { .. } => {
                                (conv::map_binding_type(&entry.ty), true)
                            }
                        },
                        ref other => (conv::map_binding_type(other), false),
                    };
                    let bt = match range_ty {
                        Direct3D12::D3D12_DESCRIPTOR_RANGE_TYPE_CBV => &mut bind_cbv,
                        Direct3D12::D3D12_DESCRIPTOR_RANGE_TYPE_SRV => &mut bind_srv,
                        Direct3D12::D3D12_DESCRIPTOR_RANGE_TYPE_UAV => &mut bind_uav,
                        Direct3D12::D3D12_DESCRIPTOR_RANGE_TYPE_SAMPLER => continue,
                        _ => todo!(),
                    };

                    let binding_array_size = entry.count.map(NonZeroU32::get);

                    let dynamic_storage_buffer_offsets_index = if has_dynamic_offset {
                        debug_assert!(
                            binding_array_size.is_none(),
                            "binding arrays and dynamic buffers are mutually exclusive"
                        );
                        let ret = Some(dynamic_storage_buffers);
                        dynamic_storage_buffers += 1;
                        ret
                    } else {
                        None
                    };

                    binding_map.insert(
                        naga::ResourceBinding {
                            group: index as u32,
                            binding: entry.binding,
                        },
                        hlsl::BindTarget {
                            binding_array_size,
                            dynamic_storage_buffer_offsets_index,
                            ..*bt
                        },
                    );
                    ranges.push(Direct3D12::D3D12_DESCRIPTOR_RANGE {
                        RangeType: range_ty,
                        NumDescriptors: count,
                        BaseShaderRegister: bt.register,
                        RegisterSpace: bt.space as u32,
                        OffsetInDescriptorsFromTableStart:
                            Direct3D12::D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND,
                    });
                    bt.register += count;
                }
            }

            let mut sampler_index_within_bind_group = 0;
            for entry in bgl.entries.iter() {
                if let wgt::BindingType::Sampler(_) = entry.ty {
                    binding_map.insert(
                        naga::ResourceBinding {
                            group: index as u32,
                            binding: entry.binding,
                        },
                        hlsl::BindTarget {
                            // Naga does not use the space field for samplers
                            space: 255,
                            register: sampler_index_within_bind_group,
                            binding_array_size: None,
                            dynamic_storage_buffer_offsets_index: None,
                            restrict_indexing: false,
                        },
                    );
                    sampler_index_within_bind_group += 1;
                }
            }

            if sampler_index_within_bind_group != 0 {
                sampler_buffer_binding_map.insert(
                    hlsl::SamplerIndexBufferKey {
                        group: index as u32,
                    },
                    bind_srv,
                );
                ranges.push(Direct3D12::D3D12_DESCRIPTOR_RANGE {
                    RangeType: Direct3D12::D3D12_DESCRIPTOR_RANGE_TYPE_SRV,
                    NumDescriptors: 1,
                    BaseShaderRegister: bind_srv.register,
                    RegisterSpace: bind_srv.space as u32,
                    OffsetInDescriptorsFromTableStart:
                        Direct3D12::D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND,
                });
                bind_srv.register += 1;
            }

            if ranges.len() > range_base {
                let range = &ranges[range_base..];
                parameters.push(Direct3D12::D3D12_ROOT_PARAMETER {
                    ParameterType: Direct3D12::D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE,
                    Anonymous: Direct3D12::D3D12_ROOT_PARAMETER_0 {
                        DescriptorTable: Direct3D12::D3D12_ROOT_DESCRIPTOR_TABLE {
                            NumDescriptorRanges: range.len() as u32,
                            pDescriptorRanges: range.as_ptr(),
                        },
                    },
                    ShaderVisibility: conv::map_visibility(visibility_view_static),
                });
                info.tables |= super::TableTypes::SRV_CBV_UAV;
            }

            // Root descriptors for dynamic uniform buffers
            let dynamic_buffers_visibility = conv::map_visibility(visibility_view_dynamic_uniform);
            for entry in bgl.entries.iter() {
                match entry.ty {
                    wgt::BindingType::Buffer {
                        ty: wgt::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        ..
                    } => {}
                    _ => continue,
                };

                binding_map.insert(
                    naga::ResourceBinding {
                        group: index as u32,
                        binding: entry.binding,
                    },
                    hlsl::BindTarget {
                        binding_array_size: entry.count.map(NonZeroU32::get),
                        restrict_indexing: true,
                        ..bind_cbv
                    },
                );

                parameters.push(Direct3D12::D3D12_ROOT_PARAMETER {
                    ParameterType: Direct3D12::D3D12_ROOT_PARAMETER_TYPE_CBV,
                    Anonymous: Direct3D12::D3D12_ROOT_PARAMETER_0 {
                        Descriptor: Direct3D12::D3D12_ROOT_DESCRIPTOR {
                            ShaderRegister: bind_cbv.register,
                            RegisterSpace: bind_cbv.space as u32,
                        },
                    },
                    ShaderVisibility: dynamic_buffers_visibility,
                });

                bind_cbv.register += entry.count.map_or(1, NonZeroU32::get);
            }

            // Root constants for (offsets of) dynamic storage buffers
            if dynamic_storage_buffers > 0 {
                let parameter_index = parameters.len();

                parameters.push(Direct3D12::D3D12_ROOT_PARAMETER {
                    ParameterType: Direct3D12::D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS,
                    Anonymous: Direct3D12::D3D12_ROOT_PARAMETER_0 {
                        Constants: Direct3D12::D3D12_ROOT_CONSTANTS {
                            ShaderRegister: bind_cbv.register,
                            RegisterSpace: bind_cbv.space as u32,
                            Num32BitValues: dynamic_storage_buffers,
                        },
                    },
                    ShaderVisibility: conv::map_visibility(visibility_view_dynamic_storage),
                });

                let binding = hlsl::OffsetsBindTarget {
                    space: bind_cbv.space,
                    register: bind_cbv.register,
                    size: dynamic_storage_buffers,
                };

                bind_cbv.register += 1;

                dynamic_storage_buffer_offsets_targets.insert(index as u32, binding);
                info.dynamic_storage_buffer_offsets = Some(DynamicStorageBufferOffsets {
                    root_index: parameter_index as u32,
                    range: total_dynamic_storage_buffers as usize
                        ..total_dynamic_storage_buffers as usize + dynamic_storage_buffers as usize,
                });
                total_dynamic_storage_buffers += dynamic_storage_buffers;
            }

            bind_group_infos[index] = Some(info);
        }

        let sampler_heap_target = hlsl::SamplerHeapBindTargets {
            standard_samplers: hlsl::BindTarget {
                space: 0,
                register: 0,
                binding_array_size: None,
                dynamic_storage_buffer_offsets_index: None,
                restrict_indexing: false,
            },
            comparison_samplers: hlsl::BindTarget {
                space: 0,
                register: 2048,
                binding_array_size: None,
                dynamic_storage_buffer_offsets_index: None,
                restrict_indexing: false,
            },
        };

        let mut sampler_heap_root_index = None;
        if sampler_in_any_bind_group {
            // Sampler descriptor tables
            //
            // We bind two sampler ranges pointing to the same descriptor heap, using two different register ranges.
            //
            // We bind them as normal samplers in registers 0-2047 and comparison samplers in registers 2048-4095.
            // Tier 2 hardware guarantees that the type of sampler only needs to match if the sampler is actually
            // accessed in the shader. As such, we can bind the same array of samplers to both registers.
            //
            // We do this because HLSL does not allow you to alias registers at all.
            let range_base = ranges.len();
            // Standard samplers, registers 0-2047
            ranges.push(Direct3D12::D3D12_DESCRIPTOR_RANGE {
                RangeType: Direct3D12::D3D12_DESCRIPTOR_RANGE_TYPE_SAMPLER,
                NumDescriptors: 2048,
                BaseShaderRegister: 0,
                RegisterSpace: 0,
                OffsetInDescriptorsFromTableStart: 0,
            });
            // Comparison samplers, registers 2048-4095
            ranges.push(Direct3D12::D3D12_DESCRIPTOR_RANGE {
                RangeType: Direct3D12::D3D12_DESCRIPTOR_RANGE_TYPE_SAMPLER,
                NumDescriptors: 2048,
                BaseShaderRegister: 2048,
                RegisterSpace: 0,
                OffsetInDescriptorsFromTableStart: 0,
            });

            let range = &ranges[range_base..];
            sampler_heap_root_index = Some(parameters.len() as super::RootIndex);
            parameters.push(Direct3D12::D3D12_ROOT_PARAMETER {
                ParameterType: Direct3D12::D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE,
                Anonymous: Direct3D12::D3D12_ROOT_PARAMETER_0 {
                    DescriptorTable: Direct3D12::D3D12_ROOT_DESCRIPTOR_TABLE {
                        NumDescriptorRanges: range.len() as u32,
                        pDescriptorRanges: range.as_ptr(),
                    },
                },
                ShaderVisibility: Direct3D12::D3D12_SHADER_VISIBILITY_ALL,
            });
        }

        // Ensure that we didn't reallocate!
        debug_assert_eq!(ranges.len(), total_non_dynamic_entries);

        let (special_constants_root_index, special_constants_binding) = if desc.flags.intersects(
            crate::PipelineLayoutFlags::FIRST_VERTEX_INSTANCE
                | crate::PipelineLayoutFlags::NUM_WORK_GROUPS,
        ) {
            let parameter_index = parameters.len();
            parameters.push(Direct3D12::D3D12_ROOT_PARAMETER {
                ParameterType: Direct3D12::D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS,
                Anonymous: Direct3D12::D3D12_ROOT_PARAMETER_0 {
                    Constants: Direct3D12::D3D12_ROOT_CONSTANTS {
                        ShaderRegister: bind_cbv.register,
                        RegisterSpace: bind_cbv.space as u32,
                        Num32BitValues: 3, // 0 = first_vertex, 1 = first_instance, 2 = other
                    },
                },
                ShaderVisibility: Direct3D12::D3D12_SHADER_VISIBILITY_ALL, // really needed for VS and CS only,
            });
            let binding = bind_cbv;
            // This is the last time we use this, but lets increment
            // it so if we add more later, the value behaves correctly.

            // This is an allow as it doesn't trigger on 1.90, hal's MSRV.
            #[allow(unused_assignments)]
            {
                bind_cbv.register += 1;
            }
            (Some(parameter_index as u32), Some(binding))
        } else {
            (None, None)
        };

        let blob = self.library.serialize_root_signature(
            Direct3D12::D3D_ROOT_SIGNATURE_VERSION_1_0,
            &parameters,
            &[],
            Direct3D12::D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
        )?;

        let raw = unsafe {
            self.raw
                .CreateRootSignature::<Direct3D12::ID3D12RootSignature>(0, blob.as_slice())
        }
        .into_device_result("Root signature creation")?;

        let special_constants = if let Some(root_index) = special_constants_root_index {
            let cmd_signatures = if desc
                .flags
                .contains(crate::PipelineLayoutFlags::INDIRECT_BUILTIN_UPDATE)
            {
                let constant_indirect_argument_desc = Direct3D12::D3D12_INDIRECT_ARGUMENT_DESC {
                    Type: Direct3D12::D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT,
                    Anonymous: Direct3D12::D3D12_INDIRECT_ARGUMENT_DESC_0 {
                        Constant: Direct3D12::D3D12_INDIRECT_ARGUMENT_DESC_0_1 {
                            RootParameterIndex: root_index,
                            DestOffsetIn32BitValues: 0,
                            Num32BitValuesToSet: 3,
                        },
                    },
                };
                let special_constant_buffer_args_len = {
                    // Hack: construct a dummy value of the special constants buffer value we need to
                    // fill, and calculate the size of each member.
                    let super::RootElement::SpecialConstantBuffer {
                        first_vertex,
                        first_instance,
                        other,
                    } = (super::RootElement::SpecialConstantBuffer {
                        first_vertex: 0,
                        first_instance: 0,
                        other: 0,
                    })
                    else {
                        unreachable!();
                    };
                    size_of_val(&first_vertex) + size_of_val(&first_instance) + size_of_val(&other)
                };

                let draw_mesh = if self
                    .features
                    .features_wgpu
                    .contains(wgt::FeaturesWGPU::EXPERIMENTAL_MESH_SHADER)
                {
                    Some(Self::create_command_signature(
                        &self.raw,
                        Some(&raw),
                        special_constant_buffer_args_len + size_of::<wgt::DispatchIndirectArgs>(),
                        &[
                            constant_indirect_argument_desc,
                            Direct3D12::D3D12_INDIRECT_ARGUMENT_DESC {
                                Type: Direct3D12::D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH_MESH,
                                ..Default::default()
                            },
                        ],
                        0,
                    )?)
                } else {
                    None
                };

                Some(super::CommandSignatures {
                    draw: Self::create_command_signature(
                        &self.raw,
                        Some(&raw),
                        special_constant_buffer_args_len + size_of::<wgt::DrawIndirectArgs>(),
                        &[
                            constant_indirect_argument_desc,
                            Direct3D12::D3D12_INDIRECT_ARGUMENT_DESC {
                                Type: Direct3D12::D3D12_INDIRECT_ARGUMENT_TYPE_DRAW,
                                ..Default::default()
                            },
                        ],
                        0,
                    )?,
                    draw_indexed: Self::create_command_signature(
                        &self.raw,
                        Some(&raw),
                        special_constant_buffer_args_len
                            + size_of::<wgt::DrawIndexedIndirectArgs>(),
                        &[
                            constant_indirect_argument_desc,
                            Direct3D12::D3D12_INDIRECT_ARGUMENT_DESC {
                                Type: Direct3D12::D3D12_INDIRECT_ARGUMENT_TYPE_DRAW_INDEXED,
                                ..Default::default()
                            },
                        ],
                        0,
                    )?,
                    draw_mesh,
                    dispatch: Self::create_command_signature(
                        &self.raw,
                        Some(&raw),
                        special_constant_buffer_args_len + size_of::<wgt::DispatchIndirectArgs>(),
                        &[
                            constant_indirect_argument_desc,
                            Direct3D12::D3D12_INDIRECT_ARGUMENT_DESC {
                                Type: Direct3D12::D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH,
                                ..Default::default()
                            },
                        ],
                        0,
                    )?,
                })
            } else {
                None
            };
            Some(super::PipelineLayoutSpecialConstants {
                root_index,
                indirect_cmd_signatures: cmd_signatures,
            })
        } else {
            None
        };

        if let Some(label) = desc.label {
            raw.set_name(label)?;
        }

        self.counters.pipeline_layouts.add(1);

        Ok(super::PipelineLayout {
            shared: super::PipelineLayoutShared {
                signature: Some(raw),
                total_root_elements: parameters.len() as super::RootIndex,
                special_constants,
                root_constant_info,
                sampler_heap_root_index,
            },
            bind_group_infos,
            naga_options: hlsl::Options {
                shader_model: self.shared.private_caps.shader_model,
                binding_map,
                fake_missing_bindings: false,
                special_constants_binding,
                immediates_target,
                dynamic_storage_buffer_offsets_targets,
                zero_initialize_workgroup_memory: true,
                restrict_indexing: true,
                sampler_heap_target,
                sampler_buffer_binding_map,
                external_texture_binding_map,
                force_loop_bounding: true,
                ray_query_initialization_tracking: true,
            },
        })
    }

    unsafe fn destroy_pipeline_layout(&self, _pipeline_layout: super::PipelineLayout) {
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
        let mut cpu_views = desc
            .layout
            .cpu_heap_views
            .as_ref()
            .map(|cpu_heap| cpu_heap.inner.lock());
        if let Some(ref mut inner) = cpu_views {
            inner.stage.clear();
        }
        let mut dynamic_buffers = Vec::new();

        let layout_and_entry_iter = desc.entries.iter().map(|entry| {
            let layout = desc
                .layout
                .entries
                .iter()
                .find(|layout_entry| layout_entry.binding == entry.binding)
                .expect("internal error: no layout entry found with binding slot");
            (layout, entry)
        });
        let mut sampler_indexes: Vec<super::sampler::SamplerIndex> = Vec::new();

        for (layout, entry) in layout_and_entry_iter {
            match layout.ty {
                wgt::BindingType::Buffer {
                    ty,
                    has_dynamic_offset,
                    ..
                } => {
                    let start = entry.resource_index as usize;
                    let end = start + entry.count as usize;
                    for data in &desc.buffers[start..end] {
                        let gpu_address = data.resolve_address();
                        let mut size = data.resolve_size().try_into().unwrap();

                        if has_dynamic_offset {
                            match ty {
                                wgt::BufferBindingType::Uniform => {
                                    dynamic_buffers.push(super::DynamicBuffer::Uniform(
                                        Direct3D12::D3D12_GPU_DESCRIPTOR_HANDLE {
                                            ptr: data.resolve_address(),
                                        },
                                    ));
                                    continue;
                                }
                                wgt::BufferBindingType::Storage { .. } => {
                                    size = (data.buffer.size - data.offset) as u32;
                                    dynamic_buffers.push(super::DynamicBuffer::Storage);
                                }
                            }
                        }

                        let inner = cpu_views.as_mut().unwrap();
                        let cpu_index = inner.stage.len() as u32;
                        let handle = desc.layout.cpu_heap_views.as_ref().unwrap().at(cpu_index);
                        match ty {
                            wgt::BufferBindingType::Uniform => {
                                let size_mask =
                                    Direct3D12::D3D12_CONSTANT_BUFFER_DATA_PLACEMENT_ALIGNMENT - 1;
                                let raw_desc = Direct3D12::D3D12_CONSTANT_BUFFER_VIEW_DESC {
                                    BufferLocation: gpu_address,
                                    SizeInBytes: ((size - 1) | size_mask) + 1,
                                };
                                unsafe {
                                    self.raw.CreateConstantBufferView(Some(&raw_desc), handle)
                                };
                            }
                            wgt::BufferBindingType::Storage { read_only: true } => {
                                let raw_desc = Direct3D12::D3D12_SHADER_RESOURCE_VIEW_DESC {
                                    Format: Dxgi::Common::DXGI_FORMAT_R32_TYPELESS,
                                    Shader4ComponentMapping:
                                        Direct3D12::D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING,
                                    ViewDimension: Direct3D12::D3D12_SRV_DIMENSION_BUFFER,
                                    Anonymous: Direct3D12::D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                                        Buffer: Direct3D12::D3D12_BUFFER_SRV {
                                            FirstElement: data.offset / 4,
                                            NumElements: size / 4,
                                            StructureByteStride: 0,
                                            Flags: Direct3D12::D3D12_BUFFER_SRV_FLAG_RAW,
                                        },
                                    },
                                };
                                unsafe {
                                    self.raw.CreateShaderResourceView(
                                        &data.buffer.resource,
                                        Some(&raw_desc),
                                        handle,
                                    )
                                };
                            }
                            wgt::BufferBindingType::Storage { read_only: false } => {
                                let raw_desc = Direct3D12::D3D12_UNORDERED_ACCESS_VIEW_DESC {
                                    Format: Dxgi::Common::DXGI_FORMAT_R32_TYPELESS,
                                    ViewDimension: Direct3D12::D3D12_UAV_DIMENSION_BUFFER,
                                    Anonymous: Direct3D12::D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                                        Buffer: Direct3D12::D3D12_BUFFER_UAV {
                                            FirstElement: data.offset / 4,
                                            NumElements: size / 4,
                                            StructureByteStride: 0,
                                            CounterOffsetInBytes: 0,
                                            Flags: Direct3D12::D3D12_BUFFER_UAV_FLAG_RAW,
                                        },
                                    },
                                };
                                unsafe {
                                    self.raw.CreateUnorderedAccessView(
                                        &data.buffer.resource,
                                        None,
                                        Some(&raw_desc),
                                        handle,
                                    )
                                };
                            }
                        }
                        inner.stage.push(handle);
                    }
                }
                wgt::BindingType::Texture { .. } => {
                    let start = entry.resource_index as usize;
                    let end = start + entry.count as usize;
                    for data in &desc.textures[start..end] {
                        let handle = data.view.handle_srv.unwrap();
                        cpu_views.as_mut().unwrap().stage.push(handle.raw);
                    }
                }
                wgt::BindingType::StorageTexture { .. } => {
                    let start = entry.resource_index as usize;
                    let end = start + entry.count as usize;
                    for data in &desc.textures[start..end] {
                        let handle = data.view.handle_uav.unwrap();
                        cpu_views.as_mut().unwrap().stage.push(handle.raw);
                    }
                }
                wgt::BindingType::Sampler { .. } => {
                    let start = entry.resource_index as usize;
                    let end = start + entry.count as usize;
                    for &data in &desc.samplers[start..end] {
                        sampler_indexes.push(data.index);
                    }
                }
                wgt::BindingType::AccelerationStructure { .. } => {
                    let start = entry.resource_index as usize;
                    let end = start + entry.count as usize;
                    for data in &desc.acceleration_structures[start..end] {
                        let inner = cpu_views.as_mut().unwrap();
                        let cpu_index = inner.stage.len() as u32;
                        let handle = desc.layout.cpu_heap_views.as_ref().unwrap().at(cpu_index);
                        let raw_desc = Direct3D12::D3D12_SHADER_RESOURCE_VIEW_DESC {
                            Format: Dxgi::Common::DXGI_FORMAT_UNKNOWN,
                            Shader4ComponentMapping:
                                Direct3D12::D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING,
                            ViewDimension:
                                Direct3D12::D3D12_SRV_DIMENSION_RAYTRACING_ACCELERATION_STRUCTURE,
                            Anonymous: Direct3D12::D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                                RaytracingAccelerationStructure:
                                    Direct3D12::D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV {
                                        Location: unsafe { data.resource.GetGPUVirtualAddress() },
                                    },
                            },
                        };
                        unsafe {
                            self.raw
                                .CreateShaderResourceView(None, Some(&raw_desc), handle)
                        };
                        inner.stage.push(handle);
                    }
                }
                wgt::BindingType::ExternalTexture => {
                    // We don't yet support binding arrays of external textures.
                    // https://github.com/gfx-rs/wgpu/issues/8027
                    assert_eq!(entry.count, 1);
                    let external_texture = &desc.external_textures[entry.resource_index as usize];
                    for plane in &external_texture.planes {
                        let plane_handle = plane.view.handle_srv.unwrap();
                        cpu_views.as_mut().unwrap().stage.push(plane_handle.raw);
                    }
                    let gpu_address = external_texture.params.resolve_address();
                    let size = external_texture.params.resolve_size() as u32;
                    let inner = cpu_views.as_mut().unwrap();
                    let cpu_index = inner.stage.len() as u32;
                    let params_handle = desc.layout.cpu_heap_views.as_ref().unwrap().at(cpu_index);
                    let size_mask = Direct3D12::D3D12_CONSTANT_BUFFER_DATA_PLACEMENT_ALIGNMENT - 1;
                    let raw_desc = Direct3D12::D3D12_CONSTANT_BUFFER_VIEW_DESC {
                        BufferLocation: gpu_address,
                        SizeInBytes: ((size - 1) | size_mask) + 1,
                    };
                    unsafe {
                        self.raw
                            .CreateConstantBufferView(Some(&raw_desc), params_handle)
                    };
                    inner.stage.push(params_handle);
                }
            }
        }

        let sampler_index_buffer = if !sampler_indexes.is_empty() {
            let buffer_size = (sampler_indexes.len() * size_of::<u32>()) as u64;

            let label = if let Some(label) = desc.label {
                Cow::Owned(format!("{label} (Internal Sampler Index Buffer)"))
            } else {
                Cow::Borrowed("Internal Sampler Index Buffer")
            };

            let buffer_desc = crate::BufferDescriptor {
                label: Some(&label),
                size: buffer_size,
                usage: wgt::BufferUses::STORAGE_READ_ONLY | wgt::BufferUses::MAP_WRITE,
                // D3D12 backend doesn't care about the memory flags
                memory_flags: crate::MemoryFlags::empty(),
            };

            let (buffer, allocation) =
                suballocation::DeviceAllocationContext::from(self).create_buffer(&buffer_desc)?;

            let mut mapping = ptr::null_mut::<ffi::c_void>();
            unsafe { buffer.Map(0, None, Some(&mut mapping)) }.into_device_result("Map")?;

            assert!(!mapping.is_null());
            assert_eq!(mapping as usize % 4, 0);

            unsafe {
                ptr::copy_nonoverlapping(
                    sampler_indexes.as_ptr(),
                    mapping.cast(),
                    sampler_indexes.len(),
                )
            };

            // The unmapping is not needed, as all memory is coherent in d3d12, but lets be nice to our address space.
            unsafe { buffer.Unmap(0, None) };

            let srv_desc = Direct3D12::D3D12_SHADER_RESOURCE_VIEW_DESC {
                Format: Dxgi::Common::DXGI_FORMAT_UNKNOWN,
                ViewDimension: Direct3D12::D3D12_SRV_DIMENSION_BUFFER,
                Anonymous: Direct3D12::D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                    Buffer: Direct3D12::D3D12_BUFFER_SRV {
                        FirstElement: 0,
                        NumElements: sampler_indexes.len() as u32,
                        StructureByteStride: 4,
                        Flags: Direct3D12::D3D12_BUFFER_SRV_FLAG_NONE,
                    },
                },
                Shader4ComponentMapping: Direct3D12::D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING,
            };

            let inner = cpu_views.as_mut().unwrap();
            let cpu_index = inner.stage.len() as u32;
            let srv = desc.layout.cpu_heap_views.as_ref().unwrap().at(cpu_index);

            unsafe {
                self.raw
                    .CreateShaderResourceView(&buffer, Some(&srv_desc), srv)
            };

            cpu_views.as_mut().unwrap().stage.push(srv);

            Some(super::SamplerIndexBuffer { buffer, allocation })
        } else {
            None
        };

        let handle_views = match cpu_views {
            Some(inner) => {
                let dual = unsafe {
                    descriptor::upload(
                        &self.raw,
                        &inner,
                        &self.shared.heap_views,
                        &desc.layout.copy_counts,
                    )
                }?;
                Some(dual)
            }
            None => None,
        };

        self.counters.bind_groups.add(1);

        Ok(super::BindGroup {
            handle_views,
            sampler_index_buffer,
            dynamic_buffers,
        })
    }

    unsafe fn destroy_bind_group(&self, group: super::BindGroup) {
        if let Some(dual) = group.handle_views {
            self.shared.heap_views.free_slice(dual);
        }

        if let Some(sampler_buffer) = group.sampler_index_buffer {
            suballocation::DeviceAllocationContext::from(self)
                .free_resource(sampler_buffer.buffer, sampler_buffer.allocation);
        }

        self.counters.bind_groups.sub(1);
    }

    unsafe fn create_shader_module(
        &self,
        desc: &crate::ShaderModuleDescriptor,
        shader: crate::ShaderInput,
    ) -> Result<super::ShaderModule, crate::ShaderError> {
        self.counters.shader_modules.add(1);

        let raw_name = desc
            .label
            .and_then(|label| alloc::ffi::CString::new(label).ok());
        match shader {
            crate::ShaderInput::Naga(naga) => Ok(super::ShaderModule {
                source: super::ShaderModuleSource::Naga(naga),
                raw_name,
                runtime_checks: desc.runtime_checks,
            }),
            crate::ShaderInput::Dxil {
                shader,
                num_workgroups,
            } => Ok(super::ShaderModule {
                source: super::ShaderModuleSource::DxilPassthrough(super::DxilPassthroughShader {
                    shader: shader.to_vec(),
                    num_workgroups,
                }),
                raw_name,
                runtime_checks: desc.runtime_checks,
            }),
            crate::ShaderInput::Hlsl {
                shader,
                num_workgroups,
            } => Ok(super::ShaderModule {
                source: super::ShaderModuleSource::HlslPassthrough(super::HlslPassthroughShader {
                    shader: shader.to_owned(),
                    num_workgroups,
                }),
                raw_name,
                runtime_checks: desc.runtime_checks,
            }),
            crate::ShaderInput::SpirV(_)
            | crate::ShaderInput::MetalLib { .. }
            | crate::ShaderInput::Msl { .. }
            | crate::ShaderInput::Glsl { .. } => {
                unreachable!()
            }
        }
    }
    unsafe fn destroy_shader_module(&self, _module: super::ShaderModule) {
        self.counters.shader_modules.sub(1);
        // just drop
    }

    unsafe fn create_render_pipeline(
        &self,
        desc: &crate::RenderPipelineDescriptor<
            super::PipelineLayout,
            super::ShaderModule,
            super::PipelineCache,
        >,
    ) -> Result<super::RenderPipeline, crate::PipelineError> {
        let mut shader_stages = wgt::ShaderStages::empty();
        let (topology_class, topology) = conv::map_topology(desc.primitive.topology);
        let mut rtv_formats = [Dxgi::Common::DXGI_FORMAT_UNKNOWN;
            Direct3D12::D3D12_SIMULTANEOUS_RENDER_TARGET_COUNT as usize];
        for (rtv_format, ct) in rtv_formats.iter_mut().zip(desc.color_targets) {
            if let Some(ct) = ct.as_ref() {
                *rtv_format = auxil::dxgi::conv::map_texture_format(ct.format);
            }
        }

        let bias = desc
            .depth_stencil
            .as_ref()
            .map(|ds| ds.bias)
            .unwrap_or_default();

        let rasterizer_state = Direct3D12::D3D12_RASTERIZER_DESC {
            FillMode: conv::map_polygon_mode(desc.primitive.polygon_mode),
            CullMode: match desc.primitive.cull_mode {
                None => Direct3D12::D3D12_CULL_MODE_NONE,
                Some(wgt::Face::Front) => Direct3D12::D3D12_CULL_MODE_FRONT,
                Some(wgt::Face::Back) => Direct3D12::D3D12_CULL_MODE_BACK,
            },
            FrontCounterClockwise: match desc.primitive.front_face {
                wgt::FrontFace::Cw => Foundation::FALSE,
                wgt::FrontFace::Ccw => Foundation::TRUE,
            },
            DepthBias: bias.constant,
            DepthBiasClamp: bias.clamp,
            SlopeScaledDepthBias: bias.slope_scale,
            DepthClipEnable: windows_core::BOOL::from(!desc.primitive.unclipped_depth),
            MultisampleEnable: windows_core::BOOL::from(desc.multisample.count > 1),
            ForcedSampleCount: 0,
            AntialiasedLineEnable: false.into(),
            ConservativeRaster: if desc.primitive.conservative {
                Direct3D12::D3D12_CONSERVATIVE_RASTERIZATION_MODE_ON
            } else {
                Direct3D12::D3D12_CONSERVATIVE_RASTERIZATION_MODE_OFF
            },
        };

        let blob_fs = match desc.fragment_stage {
            Some(ref stage) => {
                shader_stages |= wgt::ShaderStages::FRAGMENT;
                Some(self.load_shader(stage, desc.layout, naga::ShaderStage::Fragment, None)?)
            }
            None => None,
        };
        let pixel_shader = match &blob_fs {
            Some(shader) => shader.create_native_shader(),
            None => Direct3D12::D3D12_SHADER_BYTECODE::default(),
        };
        let stream_output = Direct3D12::D3D12_STREAM_OUTPUT_DESC {
            pSODeclaration: ptr::null(),
            NumEntries: 0,
            pBufferStrides: ptr::null(),
            NumStrides: 0,
            RasterizedStream: 0,
        };
        let blend_state = Direct3D12::D3D12_BLEND_DESC {
            AlphaToCoverageEnable: windows_core::BOOL::from(
                desc.multisample.alpha_to_coverage_enabled,
            ),
            IndependentBlendEnable: true.into(),
            RenderTarget: conv::map_render_targets(desc.color_targets),
        };
        let depth_stencil_state = match desc.depth_stencil {
            Some(ref ds) => conv::map_depth_stencil(ds),
            None => Default::default(),
        };
        let dsv_format = desc
            .depth_stencil
            .as_ref()
            .map_or(Dxgi::Common::DXGI_FORMAT_UNKNOWN, |ds| {
                auxil::dxgi::conv::map_texture_format(ds.format)
            });
        let sample_desc = Dxgi::Common::DXGI_SAMPLE_DESC {
            Count: desc.multisample.count,
            Quality: 0,
        };
        let cached_pso = Direct3D12::D3D12_CACHED_PIPELINE_STATE {
            pCachedBlob: ptr::null(),
            CachedBlobSizeInBytes: 0,
        };
        let flags = Direct3D12::D3D12_PIPELINE_STATE_FLAG_NONE;

        let mut view_instancing = ArrayVec::<Direct3D12::D3D12_VIEW_INSTANCE_LOCATION, 32>::new();
        if let Some(mask) = desc.multiview_mask {
            let mask = mask.get();
            // This array is just what _could_ be rendered to. We actually apply the mask at
            // renderpass creation time. The `view_index` passed to the shader depends on the
            // view's index in this array, so if we include every view in this array, `view_index`
            // actually the texture array layer, like in vulkan.
            for i in 0..32 - mask.leading_zeros() {
                view_instancing.push(Direct3D12::D3D12_VIEW_INSTANCE_LOCATION {
                    ViewportArrayIndex: 0,
                    RenderTargetArrayIndex: i,
                });
            }
        }

        // Borrow view instancing slice, so we can be sure that it won't be moved while we have pointers into this buffer.
        let view_instancing_slice = view_instancing.as_slice();

        let mut stream_desc = RenderPipelineStateStreamDesc {
            // Shared by vertex and mesh pipelines
            root_signature: desc.layout.shared.signature.as_ref(),
            pixel_shader,
            blend_state,
            sample_mask: desc.multisample.mask as u32,
            rasterizer_state,
            depth_stencil_state,
            primitive_topology_type: topology_class,
            rtv_formats: Direct3D12::D3D12_RT_FORMAT_ARRAY {
                RTFormats: rtv_formats,
                NumRenderTargets: desc.color_targets.len() as u32,
            },
            dsv_format,
            sample_desc,
            node_mask: 0,
            cached_pso,
            flags,
            view_instancing: if !view_instancing_slice.is_empty() {
                Some(Direct3D12::D3D12_VIEW_INSTANCING_DESC {
                    ViewInstanceCount: view_instancing_slice.len() as u32,
                    pViewInstanceLocations: view_instancing_slice.as_ptr(),
                    // This lets us hide/mask certain values later, at renderpass creation time.
                    Flags: Direct3D12::D3D12_VIEW_INSTANCING_FLAG_ENABLE_VIEW_INSTANCE_MASKING,
                })
            } else {
                None
            },

            // Optional data that depends on the pipeline type (vertex vs mesh).
            vertex_shader: Default::default(),
            input_layout: Default::default(),
            index_buffer_strip_cut_value: Default::default(),
            stream_output,
            task_shader: Default::default(),
            mesh_shader: Default::default(),
        };
        let mut input_element_descs = Vec::new();
        let blob_vs;
        let blob_ts;
        let blob_ms;
        let mut vertex_strides = [None; crate::MAX_VERTEX_BUFFERS];
        match &desc.vertex_processor {
            &crate::VertexProcessor::Standard {
                vertex_buffers,
                ref vertex_stage,
            } => {
                shader_stages |= wgt::ShaderStages::VERTEX;
                blob_vs = Some(self.load_shader(
                    vertex_stage,
                    desc.layout,
                    naga::ShaderStage::Vertex,
                    desc.fragment_stage.as_ref(),
                )?);

                for (i, (stride, vbuf)) in vertex_strides.iter_mut().zip(vertex_buffers).enumerate()
                {
                    *stride = Some(vbuf.array_stride as u32);
                    let (slot_class, step_rate) = match vbuf.step_mode {
                        wgt::VertexStepMode::Vertex => {
                            (Direct3D12::D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA, 0)
                        }
                        wgt::VertexStepMode::Instance => {
                            (Direct3D12::D3D12_INPUT_CLASSIFICATION_PER_INSTANCE_DATA, 1)
                        }
                    };
                    for attribute in vbuf.attributes {
                        input_element_descs.push(Direct3D12::D3D12_INPUT_ELEMENT_DESC {
                            SemanticName: windows::core::PCSTR(NAGA_LOCATION_SEMANTIC.as_ptr()),
                            SemanticIndex: attribute.shader_location,
                            Format: auxil::dxgi::conv::map_vertex_format(attribute.format),
                            InputSlot: i as u32,
                            AlignedByteOffset: attribute.offset as u32,
                            InputSlotClass: slot_class,
                            InstanceDataStepRate: step_rate,
                        });
                    }
                }
                stream_desc.vertex_shader = blob_vs.as_ref().unwrap().create_native_shader();
                stream_desc.input_layout = Direct3D12::D3D12_INPUT_LAYOUT_DESC {
                    pInputElementDescs: if input_element_descs.is_empty() {
                        ptr::null()
                    } else {
                        input_element_descs.as_ptr()
                    },
                    NumElements: input_element_descs.len() as u32,
                };
                stream_desc.index_buffer_strip_cut_value = match desc.primitive.strip_index_format {
                    Some(wgt::IndexFormat::Uint16) => {
                        Direct3D12::D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_0xFFFF
                    }
                    Some(wgt::IndexFormat::Uint32) => {
                        Direct3D12::D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_0xFFFFFFFF
                    }
                    None => Direct3D12::D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_DISABLED,
                };
                stream_desc.stream_output = Direct3D12::D3D12_STREAM_OUTPUT_DESC {
                    pSODeclaration: ptr::null(),
                    NumEntries: 0,
                    pBufferStrides: ptr::null(),
                    NumStrides: 0,
                    RasterizedStream: 0,
                };
            }
            crate::VertexProcessor::Mesh {
                task_stage,
                mesh_stage,
            } => {
                blob_ts = if let Some(ts) = task_stage {
                    shader_stages |= wgt::ShaderStages::TASK;
                    Some(self.load_shader(
                        ts,
                        desc.layout,
                        naga::ShaderStage::Task,
                        desc.fragment_stage.as_ref(),
                    )?)
                } else {
                    None
                };
                let task_shader = if let Some(ts) = &blob_ts {
                    ts.create_native_shader()
                } else {
                    Default::default()
                };
                shader_stages |= wgt::ShaderStages::MESH;
                blob_ms = Some(self.load_shader(
                    mesh_stage,
                    desc.layout,
                    naga::ShaderStage::Mesh,
                    desc.fragment_stage.as_ref(),
                )?);
                stream_desc.task_shader = task_shader;
                stream_desc.mesh_shader = blob_ms.as_ref().unwrap().create_native_shader();
            }
        };
        let raw: Direct3D12::ID3D12PipelineState =
            // If stream descriptors are available, use them as they are more flexible.
            if let Ok(device) = self.raw.cast::<Direct3D12::ID3D12Device2>() {
                // Prefer stream descs where possible
                let mut stream = stream_desc.to_stream();
                unsafe {
                    profiling::scope!("ID3D12Device2::CreatePipelineState");
                    stream.create_pipeline_state(&device).map_err(|err| {
                        crate::PipelineError::Linkage(shader_stages, err.to_string())
                    })?
                }
            } else {
                unsafe {
                    // Safety: `stream_desc` entirely outlives the `desc`.
                    let desc = stream_desc.to_graphics_pipeline_descriptor();
                    self.raw.CreateGraphicsPipelineState(&desc).map_err(|err| {
                        crate::PipelineError::Linkage(shader_stages, err.to_string())
                    })?
                }
            };

        if let Some(label) = desc.label {
            raw.set_name(label)?;
        }

        self.counters.render_pipelines.add(1);

        Ok(super::RenderPipeline {
            raw,
            layout: desc.layout.shared.clone(),
            topology,
            vertex_strides,
        })
    }

    unsafe fn destroy_render_pipeline(&self, _pipeline: super::RenderPipeline) {
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
        let blob_cs =
            self.load_shader(&desc.stage, desc.layout, naga::ShaderStage::Compute, None)?;

        let pair = {
            profiling::scope!("ID3D12Device::CreateComputePipelineState");
            unsafe {
                self.raw.CreateComputePipelineState(
                    &Direct3D12::D3D12_COMPUTE_PIPELINE_STATE_DESC {
                        pRootSignature: borrow_optional_interface_temporarily(
                            &desc.layout.shared.signature,
                        ),
                        CS: blob_cs.create_native_shader(),
                        NodeMask: 0,
                        CachedPSO: Direct3D12::D3D12_CACHED_PIPELINE_STATE::default(),
                        Flags: Direct3D12::D3D12_PIPELINE_STATE_FLAG_NONE,
                    },
                )
            }
        };

        let raw: Direct3D12::ID3D12PipelineState = pair.map_err(|err| {
            crate::PipelineError::Linkage(wgt::ShaderStages::COMPUTE, err.to_string())
        })?;

        if let Some(label) = desc.label {
            raw.set_name(label)?;
        }

        self.counters.compute_pipelines.add(1);

        Ok(super::ComputePipeline {
            raw,
            layout: desc.layout.shared.clone(),
        })
    }

    unsafe fn destroy_compute_pipeline(&self, _pipeline: super::ComputePipeline) {
        self.counters.compute_pipelines.sub(1);
    }

    unsafe fn create_pipeline_cache(
        &self,
        _desc: &crate::PipelineCacheDescriptor<'_>,
    ) -> Result<super::PipelineCache, crate::PipelineCacheError> {
        Ok(super::PipelineCache)
    }
    unsafe fn destroy_pipeline_cache(&self, _: super::PipelineCache) {}

    unsafe fn create_query_set(
        &self,
        desc: &wgt::QuerySetDescriptor<crate::Label>,
    ) -> Result<super::QuerySet, crate::DeviceError> {
        let (heap_ty, raw_ty) = match desc.ty {
            wgt::QueryType::Occlusion => (
                Direct3D12::D3D12_QUERY_HEAP_TYPE_OCCLUSION,
                Direct3D12::D3D12_QUERY_TYPE_BINARY_OCCLUSION,
            ),
            wgt::QueryType::PipelineStatistics(_) => (
                Direct3D12::D3D12_QUERY_HEAP_TYPE_PIPELINE_STATISTICS,
                Direct3D12::D3D12_QUERY_TYPE_PIPELINE_STATISTICS,
            ),
            wgt::QueryType::Timestamp => (
                Direct3D12::D3D12_QUERY_HEAP_TYPE_TIMESTAMP,
                Direct3D12::D3D12_QUERY_TYPE_TIMESTAMP,
            ),
        };

        if let Some(threshold) = self
            .mem_allocator
            .memory_budget_thresholds
            .for_resource_creation
        {
            let info = self
                .shared
                .adapter
                .query_video_memory_info(Dxgi::DXGI_MEMORY_SEGMENT_GROUP_LOCAL)?;

            // Assume each query is 256 bytes.
            // On an AMD W6800 with driver version 32.0.12030.9, occlusion and pipeline statistics are 256, timestamp is 8.

            if info.CurrentUsage + desc.count as u64 * 256 >= info.Budget / 100 * threshold as u64 {
                return Err(crate::DeviceError::OutOfMemory);
            }
        }

        let mut raw = None::<Direct3D12::ID3D12QueryHeap>;
        unsafe {
            self.raw.CreateQueryHeap(
                &Direct3D12::D3D12_QUERY_HEAP_DESC {
                    Type: heap_ty,
                    Count: desc.count,
                    NodeMask: 0,
                },
                &mut raw,
            )
        }
        .into_device_result("Query heap creation")?;

        let raw = raw.ok_or(crate::DeviceError::Unexpected)?;

        if let Some(label) = desc.label {
            raw.set_name(label)?;
        }

        self.counters.query_sets.add(1);

        Ok(super::QuerySet { raw, raw_ty })
    }

    unsafe fn destroy_query_set(&self, _set: super::QuerySet) {
        self.counters.query_sets.sub(1);
    }

    unsafe fn create_fence(&self) -> Result<super::Fence, crate::DeviceError> {
        let raw: Direct3D12::ID3D12Fence =
            unsafe { self.raw.CreateFence(0, Direct3D12::D3D12_FENCE_FLAG_SHARED) }
                .into_device_result("Fence creation")?;

        self.counters.fences.add(1);

        Ok(super::Fence { raw })
    }
    unsafe fn destroy_fence(&self, _fence: super::Fence) {
        self.counters.fences.sub(1);
    }

    unsafe fn get_fence_value(
        &self,
        fence: &super::Fence,
    ) -> Result<crate::FenceValue, crate::DeviceError> {
        Ok(unsafe { fence.raw.GetCompletedValue() })
    }
    unsafe fn wait(
        &self,
        fence: &super::Fence,
        value: crate::FenceValue,
        timeout: Option<Duration>,
    ) -> Result<bool, crate::DeviceError> {
        let timeout = timeout.unwrap_or(Duration::MAX);

        // We first check if the fence has already reached the value we're waiting for.
        let mut fence_value = unsafe { fence.raw.GetCompletedValue() };
        if fence_value >= value {
            return Ok(true);
        }

        let event = Event::create(false, false)?;

        unsafe { fence.raw.SetEventOnCompletion(value, event.0) }
            .into_device_result("Set event")?;

        let start_time = Instant::now();

        // We need to loop to get correct behavior when timeouts are involved.
        //
        // wait(0):
        //   - We set the event from the fence value 0.
        //   - WaitForSingleObject times out, we return false.
        //
        // wait(1):
        //   - We set the event from the fence value 1.
        //   - WaitForSingleObject returns. However we do not know if the fence value is 0 or 1,
        //     just that _something_ triggered the event. We check the fence value, and if it is
        //     1, we return true. Otherwise, we loop and wait again.
        loop {
            let elapsed = start_time.elapsed();

            // We need to explicitly use checked_sub. Overflow with duration panics, and if the
            // timing works out just right, we can get a negative remaining wait duration.
            //
            // This happens when a previous iteration WaitForSingleObject succeeded with a previous fence value,
            // right before the timeout would have been hit.
            let remaining_wait_duration = match timeout.checked_sub(elapsed) {
                Some(remaining) => remaining,
                None => {
                    log::trace!("Timeout elapsed in between waits!");
                    break Ok(false);
                }
            };

            log::trace!("Waiting for fence value {value} for {remaining_wait_duration:?}");

            match unsafe {
                Threading::WaitForSingleObject(
                    event.0,
                    remaining_wait_duration.as_millis().min(u32::MAX as u128) as u32,
                )
            } {
                Foundation::WAIT_OBJECT_0 => {}
                Foundation::WAIT_ABANDONED | Foundation::WAIT_FAILED => {
                    log::error!("Wait failed!");
                    break Err(crate::DeviceError::Lost);
                }
                Foundation::WAIT_TIMEOUT => {
                    log::trace!("Wait timed out!");
                    break Ok(false);
                }
                other => {
                    log::error!("Unexpected wait status: 0x{other:?}");
                    break Err(crate::DeviceError::Lost);
                }
            };

            fence_value = unsafe { fence.raw.GetCompletedValue() };
            log::trace!("Wait complete! Fence actual value: {fence_value}");

            if fence_value >= value {
                break Ok(true);
            }
        }
    }

    unsafe fn start_graphics_debugger_capture(&self) -> bool {
        #[cfg(feature = "renderdoc")]
        {
            unsafe {
                self.render_doc
                    .start_frame_capture(self.raw.as_raw(), ptr::null_mut())
            }
        }
        #[cfg(not(feature = "renderdoc"))]
        false
    }

    unsafe fn stop_graphics_debugger_capture(&self) {
        #[cfg(feature = "renderdoc")]
        unsafe {
            self.render_doc
                .end_frame_capture(self.raw.as_raw(), ptr::null_mut())
        }
    }

    unsafe fn get_acceleration_structure_build_sizes<'a>(
        &self,
        desc: &crate::GetAccelerationStructureBuildSizesDescriptor<'a, super::Buffer>,
    ) -> crate::AccelerationStructureBuildSizes {
        let mut geometry_desc;
        let device5 = self.raw.cast::<Direct3D12::ID3D12Device5>().unwrap();
        let ty;
        let inputs0;
        let num_desc;
        match desc.entries {
            AccelerationStructureEntries::Instances(instances) => {
                ty = Direct3D12::D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL;
                inputs0 = Direct3D12::D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS_0 {
                    InstanceDescs: 0,
                };
                num_desc = instances.count;
            }
            AccelerationStructureEntries::Triangles(triangles) => {
                geometry_desc = Vec::with_capacity(triangles.len());
                for triangle in triangles {
                    let index_format = triangle
                        .indices
                        .as_ref()
                        .map_or(Dxgi::Common::DXGI_FORMAT_UNKNOWN, |indices| {
                            auxil::dxgi::conv::map_index_format(indices.format)
                        });
                    let index_count = triangle.indices.as_ref().map_or(0, |indices| indices.count);

                    let triangle_desc = Direct3D12::D3D12_RAYTRACING_GEOMETRY_TRIANGLES_DESC {
                        // https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device5-getraytracingaccelerationstructureprebuildinfo
                        // It may not inspect/dereference any GPU virtual addresses, other than
                        // to check to see if a pointer is NULL or not, such as the optional
                        // transform in D3D12_RAYTRACING_GEOMETRY_TRIANGLES_DESC, without
                        // dereferencing it.
                        //
                        // This suggests we could pass a non-zero invalid address here if fetching the
                        // real address has significant overhead, but we pass the real one to be on the
                        // safe side for now.
                        Transform3x4: if desc
                            .flags
                            .contains(wgt::AccelerationStructureFlags::USE_TRANSFORM)
                        {
                            unsafe {
                                triangle
                                    .transform
                                    .as_ref()
                                    .unwrap()
                                    .buffer
                                    .resource
                                    .GetGPUVirtualAddress()
                            }
                        } else {
                            0
                        },
                        IndexFormat: index_format,
                        VertexFormat: auxil::dxgi::conv::map_vertex_format(triangle.vertex_format),
                        IndexCount: index_count,
                        VertexCount: triangle.vertex_count,
                        IndexBuffer: 0,
                        VertexBuffer: Direct3D12::D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE {
                            StartAddress: 0,
                            StrideInBytes: triangle.vertex_stride,
                        },
                    };

                    geometry_desc.push(Direct3D12::D3D12_RAYTRACING_GEOMETRY_DESC {
                        Type: Direct3D12::D3D12_RAYTRACING_GEOMETRY_TYPE_TRIANGLES,
                        Flags: conv::map_acceleration_structure_geometry_flags(triangle.flags),
                        Anonymous: Direct3D12::D3D12_RAYTRACING_GEOMETRY_DESC_0 {
                            Triangles: triangle_desc,
                        },
                    })
                }
                ty = Direct3D12::D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL;
                inputs0 = Direct3D12::D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS_0 {
                    pGeometryDescs: geometry_desc.as_ptr(),
                };
                num_desc = geometry_desc.len() as u32;
            }
            AccelerationStructureEntries::AABBs(aabbs) => {
                geometry_desc = Vec::with_capacity(aabbs.len());
                for aabb in aabbs {
                    let aabb_desc = Direct3D12::D3D12_RAYTRACING_GEOMETRY_AABBS_DESC {
                        AABBCount: aabb.count as u64,
                        AABBs: Direct3D12::D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE {
                            StartAddress: 0,
                            StrideInBytes: aabb.stride,
                        },
                    };
                    geometry_desc.push(Direct3D12::D3D12_RAYTRACING_GEOMETRY_DESC {
                        Type: Direct3D12::D3D12_RAYTRACING_GEOMETRY_TYPE_PROCEDURAL_PRIMITIVE_AABBS,
                        Flags: conv::map_acceleration_structure_geometry_flags(aabb.flags),
                        Anonymous: Direct3D12::D3D12_RAYTRACING_GEOMETRY_DESC_0 {
                            AABBs: aabb_desc,
                        },
                    })
                }
                ty = Direct3D12::D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL;
                inputs0 = Direct3D12::D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS_0 {
                    pGeometryDescs: geometry_desc.as_ptr(),
                };
                num_desc = geometry_desc.len() as u32;
            }
        };
        let acceleration_structure_inputs =
            Direct3D12::D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS {
                Type: ty,
                Flags: conv::map_acceleration_structure_build_flags(desc.flags, None),
                NumDescs: num_desc,
                DescsLayout: Direct3D12::D3D12_ELEMENTS_LAYOUT_ARRAY,
                Anonymous: inputs0,
            };
        let mut info = Direct3D12::D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO::default();
        unsafe {
            device5.GetRaytracingAccelerationStructurePrebuildInfo(
                &acceleration_structure_inputs,
                &mut info,
            )
        };
        crate::AccelerationStructureBuildSizes {
            acceleration_structure_size: info.ResultDataMaxSizeInBytes,
            update_scratch_size: info.UpdateScratchDataSizeInBytes,
            build_scratch_size: info.ScratchDataSizeInBytes,
        }
    }

    unsafe fn get_acceleration_structure_device_address(
        &self,
        acceleration_structure: &super::AccelerationStructure,
    ) -> wgt::BufferAddress {
        unsafe { acceleration_structure.resource.GetGPUVirtualAddress() }
    }

    unsafe fn create_acceleration_structure(
        &self,
        desc: &crate::AccelerationStructureDescriptor,
    ) -> Result<super::AccelerationStructure, crate::DeviceError> {
        // Create a D3D12 resource as per-usual.
        let size = desc.size;

        let raw_desc = Direct3D12::D3D12_RESOURCE_DESC {
            Dimension: Direct3D12::D3D12_RESOURCE_DIMENSION_BUFFER,
            Alignment: 0,
            Width: size,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: Dxgi::Common::DXGI_FORMAT_UNKNOWN,
            SampleDesc: Dxgi::Common::DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Layout: Direct3D12::D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            // TODO: when moving to enhanced barriers use Direct3D12::D3D12_RESOURCE_FLAG_RAYTRACING_ACCELERATION_STRUCTURE
            Flags: Direct3D12::D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS,
        };

        let (resource, allocation) = suballocation::DeviceAllocationContext::from(self)
            .create_acceleration_structure(desc, raw_desc)?;

        // for some reason there is no counter for acceleration structures

        Ok(super::AccelerationStructure {
            resource,
            allocation,
        })
    }

    unsafe fn destroy_acceleration_structure(
        &self,
        acceleration_structure: super::AccelerationStructure,
    ) {
        suballocation::DeviceAllocationContext::from(self).free_resource(
            acceleration_structure.resource,
            acceleration_structure.allocation,
        );
    }

    fn get_internal_counters(&self) -> wgt::HalCounters {
        self.counters.as_ref().clone()
    }

    fn generate_allocator_report(&self) -> Option<wgt::AllocatorReport> {
        Some(self.mem_allocator.generate_report())
    }

    fn tlas_instance_to_bytes(&self, instance: TlasInstance) -> Vec<u8> {
        const MAX_U24: u32 = (1u32 << 24u32) - 1u32;
        let temp = Direct3D12::D3D12_RAYTRACING_INSTANCE_DESC {
            Transform: instance.transform,
            _bitfield1: (instance.custom_data & MAX_U24) | (u32::from(instance.mask) << 24),
            _bitfield2: 0,
            AccelerationStructure: instance.blas_address,
        };

        wgt::bytemuck_wrapper!(unsafe struct Desc(Direct3D12::D3D12_RAYTRACING_INSTANCE_DESC));

        bytemuck::bytes_of(&Desc::wrap(temp)).to_vec()
    }

    fn check_if_oom(&self) -> Result<(), crate::DeviceError> {
        let Some(threshold) = self.mem_allocator.memory_budget_thresholds.for_device_loss else {
            return Ok(());
        };

        let info = self
            .shared
            .adapter
            .query_video_memory_info(Dxgi::DXGI_MEMORY_SEGMENT_GROUP_LOCAL)?;

        if info.CurrentUsage >= info.Budget / 100 * threshold as u64 {
            return Err(crate::DeviceError::OutOfMemory);
        }

        if matches!(
            self.shared.private_caps.memory_architecture,
            super::MemoryArchitecture::NonUnified
        ) {
            let info = self
                .shared
                .adapter
                .query_video_memory_info(Dxgi::DXGI_MEMORY_SEGMENT_GROUP_NON_LOCAL)?;

            if info.CurrentUsage >= info.Budget / 100 * threshold as u64 {
                return Err(crate::DeviceError::OutOfMemory);
            }
        }

        Ok(())
    }
}
