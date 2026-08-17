use alloc::{borrow::ToOwned as _, sync::Arc, vec::Vec};
use core::{ptr::NonNull, sync::atomic};
use std::{thread, time};

use bytemuck::TransparentWrapper;
use objc2::{
    available,
    rc::{autoreleasepool, Retained},
    runtime::ProtocolObject,
};
use objc2_foundation::{ns_string, NSError, NSRange, NSString, NSUInteger};
use objc2_metal::{
    MTLAccelerationStructure, MTLAccelerationStructureInstanceOptions, MTLBuffer,
    MTLCaptureManager, MTLCaptureScope, MTLCommandBuffer, MTLCommandBufferStatus,
    MTLCompileOptions, MTLComputePipelineDescriptor, MTLComputePipelineState,
    MTLCounterSampleBufferDescriptor, MTLCounterSet, MTLDepthClipMode, MTLDepthStencilDescriptor,
    MTLDevice, MTLFunction, MTLIndirectAccelerationStructureInstanceDescriptor, MTLLanguageVersion,
    MTLLibrary, MTLMeshRenderPipelineDescriptor, MTLMutability, MTLPackedFloat3, MTLPackedFloat4x3,
    MTLPipelineBufferDescriptorArray, MTLPipelineOption, MTLPixelFormat, MTLPrimitiveTopologyClass,
    MTLRenderPipelineColorAttachmentDescriptorArray, MTLRenderPipelineDescriptor, MTLResource,
    MTLResourceID, MTLResourceOptions, MTLSamplerAddressMode, MTLSamplerDescriptor,
    MTLSamplerMipFilter, MTLSamplerState, MTLSize, MTLStencilDescriptor, MTLStorageMode,
    MTLTexture, MTLTextureDescriptor, MTLTextureType, MTLTriangleFillMode, MTLVertexDescriptor,
    MTLVertexStepFunction,
};

use super::{conv, PassthroughShader, ShaderModuleSource};
use crate::{auxil::map_naga_stage, TlasInstance};

type DeviceResult<T> = Result<T, crate::DeviceError>;

struct CompiledShader {
    library: Retained<ProtocolObject<dyn MTLLibrary>>,
    function: Retained<ProtocolObject<dyn MTLFunction>>,
    wg_size: MTLSize,
    wg_memory_sizes: Vec<u32>,

    /// Bindings of WGSL `storage` globals that contain variable-sized arrays.
    ///
    /// In order to implement bounds checks and the `arrayLength` function for
    /// WGSL runtime-sized arrays, we pass the entry point a struct with a
    /// member for each global variable that contains such an array. That member
    /// is a `u32` holding the variable's total size in bytes---which is simply
    /// the size of the `Buffer` supplying that variable's contents for the
    /// draw call.
    sized_bindings: Vec<naga::ResourceBinding>,

    immutable_buffer_mask: usize,
}

fn create_stencil_desc(
    face: &wgt::StencilFaceState,
    read_mask: u32,
    write_mask: u32,
) -> Retained<MTLStencilDescriptor> {
    let desc = MTLStencilDescriptor::new();
    desc.setStencilCompareFunction(conv::map_compare_function(face.compare));
    desc.setReadMask(read_mask);
    desc.setWriteMask(write_mask);
    desc.setStencilFailureOperation(conv::map_stencil_op(face.fail_op));
    desc.setDepthFailureOperation(conv::map_stencil_op(face.depth_fail_op));
    desc.setDepthStencilPassOperation(conv::map_stencil_op(face.pass_op));
    desc
}

fn create_depth_stencil_desc(
    state: &wgt::DepthStencilState,
) -> Retained<MTLDepthStencilDescriptor> {
    let desc = MTLDepthStencilDescriptor::new();
    desc.setDepthCompareFunction(conv::map_compare_function(
        state.depth_compare.unwrap_or_default(),
    ));
    desc.setDepthWriteEnabled(state.depth_write_enabled.unwrap_or_default());
    let s = &state.stencil;
    if s.is_enabled() {
        let front_desc = create_stencil_desc(&s.front, s.read_mask, s.write_mask);
        desc.setFrontFaceStencil(Some(&front_desc));
        let back_desc = create_stencil_desc(&s.back, s.read_mask, s.write_mask);
        desc.setBackFaceStencil(Some(&back_desc));
    }
    desc
}

const fn convert_vertex_format_to_naga(format: wgt::VertexFormat) -> naga::back::msl::VertexFormat {
    match format {
        wgt::VertexFormat::Uint8 => naga::back::msl::VertexFormat::Uint8,
        wgt::VertexFormat::Uint8x2 => naga::back::msl::VertexFormat::Uint8x2,
        wgt::VertexFormat::Uint8x4 => naga::back::msl::VertexFormat::Uint8x4,
        wgt::VertexFormat::Sint8 => naga::back::msl::VertexFormat::Sint8,
        wgt::VertexFormat::Sint8x2 => naga::back::msl::VertexFormat::Sint8x2,
        wgt::VertexFormat::Sint8x4 => naga::back::msl::VertexFormat::Sint8x4,
        wgt::VertexFormat::Unorm8 => naga::back::msl::VertexFormat::Unorm8,
        wgt::VertexFormat::Unorm8x2 => naga::back::msl::VertexFormat::Unorm8x2,
        wgt::VertexFormat::Unorm8x4 => naga::back::msl::VertexFormat::Unorm8x4,
        wgt::VertexFormat::Snorm8 => naga::back::msl::VertexFormat::Snorm8,
        wgt::VertexFormat::Snorm8x2 => naga::back::msl::VertexFormat::Snorm8x2,
        wgt::VertexFormat::Snorm8x4 => naga::back::msl::VertexFormat::Snorm8x4,
        wgt::VertexFormat::Uint16 => naga::back::msl::VertexFormat::Uint16,
        wgt::VertexFormat::Uint16x2 => naga::back::msl::VertexFormat::Uint16x2,
        wgt::VertexFormat::Uint16x4 => naga::back::msl::VertexFormat::Uint16x4,
        wgt::VertexFormat::Sint16 => naga::back::msl::VertexFormat::Sint16,
        wgt::VertexFormat::Sint16x2 => naga::back::msl::VertexFormat::Sint16x2,
        wgt::VertexFormat::Sint16x4 => naga::back::msl::VertexFormat::Sint16x4,
        wgt::VertexFormat::Unorm16 => naga::back::msl::VertexFormat::Unorm16,
        wgt::VertexFormat::Unorm16x2 => naga::back::msl::VertexFormat::Unorm16x2,
        wgt::VertexFormat::Unorm16x4 => naga::back::msl::VertexFormat::Unorm16x4,
        wgt::VertexFormat::Snorm16 => naga::back::msl::VertexFormat::Snorm16,
        wgt::VertexFormat::Snorm16x2 => naga::back::msl::VertexFormat::Snorm16x2,
        wgt::VertexFormat::Snorm16x4 => naga::back::msl::VertexFormat::Snorm16x4,
        wgt::VertexFormat::Float16 => naga::back::msl::VertexFormat::Float16,
        wgt::VertexFormat::Float16x2 => naga::back::msl::VertexFormat::Float16x2,
        wgt::VertexFormat::Float16x4 => naga::back::msl::VertexFormat::Float16x4,
        wgt::VertexFormat::Float32 => naga::back::msl::VertexFormat::Float32,
        wgt::VertexFormat::Float32x2 => naga::back::msl::VertexFormat::Float32x2,
        wgt::VertexFormat::Float32x3 => naga::back::msl::VertexFormat::Float32x3,
        wgt::VertexFormat::Float32x4 => naga::back::msl::VertexFormat::Float32x4,
        wgt::VertexFormat::Uint32 => naga::back::msl::VertexFormat::Uint32,
        wgt::VertexFormat::Uint32x2 => naga::back::msl::VertexFormat::Uint32x2,
        wgt::VertexFormat::Uint32x3 => naga::back::msl::VertexFormat::Uint32x3,
        wgt::VertexFormat::Uint32x4 => naga::back::msl::VertexFormat::Uint32x4,
        wgt::VertexFormat::Sint32 => naga::back::msl::VertexFormat::Sint32,
        wgt::VertexFormat::Sint32x2 => naga::back::msl::VertexFormat::Sint32x2,
        wgt::VertexFormat::Sint32x3 => naga::back::msl::VertexFormat::Sint32x3,
        wgt::VertexFormat::Sint32x4 => naga::back::msl::VertexFormat::Sint32x4,
        wgt::VertexFormat::Unorm10_10_10_2 => naga::back::msl::VertexFormat::Unorm10_10_10_2,
        wgt::VertexFormat::Unorm8x4Bgra => naga::back::msl::VertexFormat::Unorm8x4Bgra,

        wgt::VertexFormat::Float64
        | wgt::VertexFormat::Float64x2
        | wgt::VertexFormat::Float64x3
        | wgt::VertexFormat::Float64x4 => {
            unimplemented!()
        }
    }
}

impl super::Device {
    fn load_shader(
        &self,
        stage: &crate::ProgrammableStage<super::ShaderModule>,
        vertex_buffer_mappings: &[naga::back::msl::VertexBufferMapping],
        layout: &super::PipelineLayout,
        primitive_class: MTLPrimitiveTopologyClass,
        naga_stage: naga::ShaderStage,
    ) -> Result<CompiledShader, crate::PipelineError> {
        match stage.module.source {
            ShaderModuleSource::Naga(ref naga_shader) => {
                let stage_bit = map_naga_stage(naga_stage);
                let (module, module_info) = naga::back::pipeline_constants::process_overrides(
                    &naga_shader.module,
                    &naga_shader.info,
                    Some((naga_stage, stage.entry_point)),
                    stage.constants,
                )
                .map_err(|e| {
                    crate::PipelineError::PipelineConstants(stage_bit, format!("MSL: {e:?}"))
                })?;

                let ep_resources = &layout.per_stage_map[naga_stage];

                let bounds_check_policy = if stage.module.bounds_checks.bounds_checks {
                    naga::proc::BoundsCheckPolicy::Restrict
                } else {
                    naga::proc::BoundsCheckPolicy::Unchecked
                };

                let options = naga::back::msl::Options {
                    lang_version: match self.shared.private_caps.msl_version {
                        #[allow(deprecated)]
                        MTLLanguageVersion::Version1_0 => (1, 0),
                        MTLLanguageVersion::Version1_1 => (1, 1),
                        MTLLanguageVersion::Version1_2 => (1, 2),
                        MTLLanguageVersion::Version2_0 => (2, 0),
                        MTLLanguageVersion::Version2_1 => (2, 1),
                        MTLLanguageVersion::Version2_2 => (2, 2),
                        MTLLanguageVersion::Version2_3 => (2, 3),
                        MTLLanguageVersion::Version2_4 => (2, 4),
                        MTLLanguageVersion::Version3_0 => (3, 0),
                        MTLLanguageVersion::Version3_1 => (3, 1),
                        MTLLanguageVersion::Version3_2 => (3, 2),
                        // Newer version, fall back to 3.2
                        _ => (3, 2),
                    },
                    inline_samplers: Default::default(),
                    spirv_cross_compatibility: false,
                    fake_missing_bindings: false,
                    per_entry_point_map: naga::back::msl::EntryPointResourceMap::from([(
                        stage.entry_point.to_owned(),
                        ep_resources.clone(),
                    )]),
                    bounds_check_policies: naga::proc::BoundsCheckPolicies {
                        index: bounds_check_policy,
                        buffer: bounds_check_policy,
                        image_load: bounds_check_policy,
                        // TODO: support bounds checks on binding arrays
                        binding_array: naga::proc::BoundsCheckPolicy::Unchecked,
                    },
                    zero_initialize_workgroup_memory: stage.zero_initialize_workgroup_memory,
                    force_loop_bounding: stage.module.bounds_checks.force_loop_bounding,
                };

                let pipeline_options = naga::back::msl::PipelineOptions {
                    entry_point: Some((naga_stage, stage.entry_point.to_owned())),
                    allow_and_force_point_size: match primitive_class {
                        MTLPrimitiveTopologyClass::Point => true,
                        _ => false,
                    },
                    vertex_pulling_transform: true,
                    vertex_buffer_mappings: vertex_buffer_mappings.to_vec(),
                };

                let (source, info) = naga::back::msl::write_string(
                    &module,
                    &module_info,
                    &options,
                    &pipeline_options,
                )
                .map_err(|e| crate::PipelineError::Linkage(stage_bit, format!("MSL: {e:?}")))?;

                log::debug!(
                    "Naga generated shader for entry point '{}' and stage {:?}\n{}",
                    stage.entry_point,
                    naga_stage,
                    &source
                );

                let options = MTLCompileOptions::new();
                options.setLanguageVersion(self.shared.private_caps.msl_version);

                // https://developer.apple.com/documentation/metal/mtlcompileoptions/preserveinvariance
                if available!(macos = 11.0, ios = 13.0, tvos = 14.0, visionos = 1.0) {
                    options.setPreserveInvariance(true);
                }

                let library = self
                    .shared
                    .device
                    .newLibraryWithSource_options_error(
                        &NSString::from_str(&source),
                        Some(&options),
                    )
                    .map_err(|err| {
                        log::debug!("Naga generated shader:\n{source}");
                        crate::PipelineError::Linkage(stage_bit, format!("Metal: {err}"))
                    })?;

                let ep_index = module
                    .entry_points
                    .iter()
                    .position(|ep| ep.stage == naga_stage && ep.name == stage.entry_point)
                    .ok_or(crate::PipelineError::EntryPoint(naga_stage))?;
                let ep = &module.entry_points[ep_index];
                let translated_ep_name = info.entry_point_names[0]
                    .as_ref()
                    .map_err(|e| crate::PipelineError::Linkage(stage_bit, format!("{e}")))?;

                let wg_size = MTLSize {
                    width: ep.workgroup_size[0] as _,
                    height: ep.workgroup_size[1] as _,
                    depth: ep.workgroup_size[2] as _,
                };

                let function = library
                    .newFunctionWithName(&NSString::from_str(translated_ep_name))
                    .ok_or_else(|| {
                        log::error!("Function '{translated_ep_name}' does not exist");
                        crate::PipelineError::EntryPoint(naga_stage)
                    })?;

                // collect sizes indices, immutable buffers, and work group memory sizes
                let ep_info = &module_info.get_entry_point(ep_index);
                let mut wg_memory_sizes = Vec::new();
                let mut sized_bindings = Vec::new();
                let mut immutable_buffer_mask = 0;
                for (var_handle, var) in module.global_variables.iter() {
                    match var.space {
                        naga::AddressSpace::WorkGroup => {
                            if !ep_info[var_handle].is_empty() {
                                let size = module.types[var.ty].inner.size(module.to_ctx());
                                wg_memory_sizes.push(size);
                            }
                        }
                        naga::AddressSpace::Uniform | naga::AddressSpace::Storage { .. } => {
                            let br = match var.binding {
                                Some(br) => br,
                                None => continue,
                            };
                            let storage_access_store = match var.space {
                                naga::AddressSpace::Storage { access } => {
                                    access.contains(naga::StorageAccess::STORE)
                                }
                                _ => false,
                            };

                            // check for an immutable buffer
                            if !ep_info[var_handle].is_empty() && !storage_access_store {
                                let slot = ep_resources.resources[&br].buffer.unwrap();
                                immutable_buffer_mask |= 1 << slot;
                            }

                            let mut dynamic_array_container_ty = var.ty;
                            if let naga::TypeInner::Struct { ref members, .. } =
                                module.types[var.ty].inner
                            {
                                dynamic_array_container_ty = members.last().unwrap().ty;
                            }
                            if let naga::TypeInner::Array {
                                size: naga::ArraySize::Dynamic,
                                ..
                            } = module.types[dynamic_array_container_ty].inner
                            {
                                sized_bindings.push(br);
                            }
                        }
                        _ => {}
                    }
                }

                Ok(CompiledShader {
                    library,
                    function,
                    wg_size,
                    wg_memory_sizes,
                    sized_bindings,
                    immutable_buffer_mask,
                })
            }
            ShaderModuleSource::Passthrough(ref shader) => Ok(CompiledShader {
                library: shader.library.clone(),
                function: shader
                    .library
                    .newFunctionWithName(&NSString::from_str(stage.entry_point))
                    .ok_or(crate::PipelineError::EntryPoint(naga_stage))?,
                wg_size: MTLSize {
                    width: shader.num_workgroups.0 as usize,
                    height: shader.num_workgroups.1 as usize,
                    depth: shader.num_workgroups.2 as usize,
                },
                wg_memory_sizes: vec![],
                sized_bindings: vec![],
                immutable_buffer_mask: 0,
            }),
        }
    }

    fn set_buffers_mutability(
        buffers: &MTLPipelineBufferDescriptorArray,
        mut immutable_mask: usize,
    ) {
        while immutable_mask != 0 {
            let slot = immutable_mask.trailing_zeros();
            immutable_mask ^= 1 << slot;
            unsafe { buffers.objectAtIndexedSubscript(slot as usize) }
                .setMutability(MTLMutability::Immutable);
        }
    }

    pub unsafe fn texture_from_raw(
        raw: Retained<ProtocolObject<dyn MTLTexture>>,
        format: wgt::TextureFormat,
        raw_type: MTLTextureType,
        array_layers: u32,
        mip_levels: u32,
        copy_size: crate::CopyExtent,
    ) -> super::Texture {
        super::Texture {
            raw,
            format,
            raw_type,
            array_layers,
            mip_levels,
            copy_size,
        }
    }

    pub unsafe fn device_from_raw(
        raw: Retained<ProtocolObject<dyn MTLDevice>>,
        features: wgt::Features,
    ) -> super::Device {
        let capabilities_query = super::CapabilitiesQuery::new(&raw);
        let shared = super::AdapterShared::new(raw, &capabilities_query);
        super::Device {
            shared: Arc::new(shared),
            features,
            counters: Default::default(),
        }
    }

    pub unsafe fn buffer_from_raw(
        raw: Retained<ProtocolObject<dyn MTLBuffer>>,
        size: wgt::BufferAddress,
    ) -> super::Buffer {
        super::Buffer { raw, size }
    }

    pub fn raw_device(&self) -> &Retained<ProtocolObject<dyn MTLDevice>> {
        &self.shared.device
    }
}

impl crate::Device for super::Device {
    type A = super::Api;

    unsafe fn create_buffer(&self, desc: &crate::BufferDescriptor) -> DeviceResult<super::Buffer> {
        let map_read = desc.usage.contains(wgt::BufferUses::MAP_READ);
        let map_write = desc.usage.contains(wgt::BufferUses::MAP_WRITE);

        let mut options = MTLResourceOptions::empty();
        options |= if map_read || map_write {
            // `crate::MemoryFlags::PREFER_COHERENT` is ignored here
            MTLResourceOptions::StorageModeShared
        } else {
            MTLResourceOptions::StorageModePrivate
        };
        options.set(MTLResourceOptions::CPUCacheModeWriteCombined, map_write);

        //TODO: HazardTrackingModeUntracked

        autoreleasepool(|_| {
            let raw = self
                .shared
                .device
                .newBufferWithLength_options(desc.size as usize, options)
                .ok_or(crate::DeviceError::OutOfMemory)?;
            if let Some(label) = desc.label {
                raw.setLabel(Some(&NSString::from_str(label)));
            }
            self.counters.buffers.add(1);
            Ok(super::Buffer {
                raw,
                size: desc.size,
            })
        })
    }
    unsafe fn destroy_buffer(&self, _buffer: super::Buffer) {
        self.counters.buffers.sub(1);
    }

    unsafe fn add_raw_buffer(&self, _buffer: &super::Buffer) {
        self.counters.buffers.add(1);
    }

    unsafe fn map_buffer(
        &self,
        buffer: &super::Buffer,
        range: crate::MemoryRange,
    ) -> DeviceResult<crate::BufferMapping> {
        let ptr = buffer.raw.contents().cast::<u8>();
        Ok(crate::BufferMapping {
            ptr: NonNull::new(unsafe { ptr.as_ptr().offset(range.start as isize) }).unwrap(),
            is_coherent: true,
        })
    }

    unsafe fn unmap_buffer(&self, _buffer: &super::Buffer) {}
    unsafe fn flush_mapped_ranges<I>(&self, _buffer: &super::Buffer, _ranges: I) {}
    unsafe fn invalidate_mapped_ranges<I>(&self, _buffer: &super::Buffer, _ranges: I) {}

    unsafe fn create_texture(
        &self,
        desc: &crate::TextureDescriptor,
    ) -> DeviceResult<super::Texture> {
        let mtl_format = self
            .shared
            .private_texture_format_caps
            .map_format(desc.format);

        autoreleasepool(|_| {
            let descriptor = MTLTextureDescriptor::new();

            let mtl_type = match desc.dimension {
                wgt::TextureDimension::D1 => MTLTextureType::Type1D,
                wgt::TextureDimension::D2 => {
                    if desc.sample_count > 1 {
                        unsafe { descriptor.setSampleCount(desc.sample_count as usize) };
                        MTLTextureType::Type2DMultisample
                    } else if desc.size.depth_or_array_layers > 1 {
                        unsafe {
                            descriptor.setArrayLength(desc.size.depth_or_array_layers as usize)
                        };
                        MTLTextureType::Type2DArray
                    } else {
                        MTLTextureType::Type2D
                    }
                }
                wgt::TextureDimension::D3 => {
                    unsafe { descriptor.setDepth(desc.size.depth_or_array_layers as usize) };
                    MTLTextureType::Type3D
                }
            };

            let mtl_storage_mode = if desc.usage.contains(wgt::TextureUses::TRANSIENT)
                && self.shared.private_caps.supports_memoryless_storage
            {
                MTLStorageMode::Memoryless
            } else {
                MTLStorageMode::Private
            };

            descriptor.setTextureType(mtl_type);
            unsafe { descriptor.setWidth(desc.size.width as usize) };
            unsafe { descriptor.setHeight(desc.size.height as usize) };
            unsafe { descriptor.setMipmapLevelCount(desc.mip_level_count as usize) };
            descriptor.setPixelFormat(mtl_format);
            descriptor.setUsage(conv::map_texture_usage(desc.format, desc.usage));
            descriptor.setStorageMode(mtl_storage_mode);

            let raw = self
                .shared
                .device
                .newTextureWithDescriptor(&descriptor)
                .ok_or(crate::DeviceError::OutOfMemory)?;
            if let Some(label) = desc.label {
                raw.setLabel(Some(&NSString::from_str(label)));
            }

            self.counters.textures.add(1);

            Ok(super::Texture {
                raw,
                format: desc.format,
                raw_type: mtl_type,
                mip_levels: desc.mip_level_count,
                array_layers: desc.array_layer_count(),
                copy_size: desc.copy_extent(),
            })
        })
    }

    unsafe fn destroy_texture(&self, _texture: super::Texture) {
        self.counters.textures.sub(1);
    }

    unsafe fn add_raw_texture(&self, _texture: &super::Texture) {
        self.counters.textures.add(1);
    }

    unsafe fn create_texture_view(
        &self,
        texture: &super::Texture,
        desc: &crate::TextureViewDescriptor,
    ) -> DeviceResult<super::TextureView> {
        let raw_type = if texture.raw_type == MTLTextureType::Type2DMultisample {
            texture.raw_type
        } else {
            conv::map_texture_view_dimension(desc.dimension)
        };

        let aspects = crate::FormatAspects::new(texture.format, desc.range.aspect);

        let raw_format = self
            .shared
            .private_texture_format_caps
            .map_view_format(desc.format, aspects);

        let format_equal = raw_format
            == self
                .shared
                .private_texture_format_caps
                .map_format(texture.format);
        let type_equal = raw_type == texture.raw_type;
        let range_full_resource =
            desc.range
                .is_full_resource(desc.format, texture.mip_levels, texture.array_layers);

        let raw = if format_equal && type_equal && range_full_resource {
            // Some images are marked as framebuffer-only, and we can't create aliases of them.
            // Also helps working around Metal bugs with aliased array textures.
            texture.raw.to_owned()
        } else {
            let mip_level_count = desc
                .range
                .mip_level_count
                .unwrap_or(texture.mip_levels - desc.range.base_mip_level);
            let array_layer_count = desc
                .range
                .array_layer_count
                .unwrap_or(texture.array_layers - desc.range.base_array_layer);

            autoreleasepool(|_| {
                let level_range = NSRange {
                    location: desc.range.base_mip_level as _,
                    length: mip_level_count as _,
                };
                let slice_range = NSRange {
                    location: desc.range.base_array_layer as _,
                    length: array_layer_count as _,
                };
                let raw = unsafe {
                    texture
                        .raw
                        .newTextureViewWithPixelFormat_textureType_levels_slices(
                            raw_format,
                            raw_type,
                            level_range,
                            slice_range,
                        )
                        .unwrap()
                };
                if let Some(label) = desc.label {
                    raw.setLabel(Some(&NSString::from_str(label)));
                }
                raw
            })
        };

        self.counters.texture_views.add(1);

        Ok(super::TextureView { raw, aspects })
    }

    unsafe fn destroy_texture_view(&self, _view: super::TextureView) {
        self.counters.texture_views.sub(1);
    }

    unsafe fn create_sampler(
        &self,
        desc: &crate::SamplerDescriptor,
    ) -> DeviceResult<super::Sampler> {
        autoreleasepool(|_| {
            let descriptor = MTLSamplerDescriptor::new();

            descriptor.setMinFilter(conv::map_filter_mode(desc.min_filter));
            descriptor.setMagFilter(conv::map_filter_mode(desc.mag_filter));
            descriptor.setMipFilter(match desc.mipmap_filter {
                wgt::MipmapFilterMode::Nearest if desc.lod_clamp == (0.0..0.0) => {
                    MTLSamplerMipFilter::NotMipmapped
                }
                wgt::MipmapFilterMode::Nearest => MTLSamplerMipFilter::Nearest,
                wgt::MipmapFilterMode::Linear => MTLSamplerMipFilter::Linear,
            });

            let [s, t, r] = desc.address_modes;
            descriptor.setSAddressMode(conv::map_address_mode(s));
            descriptor.setTAddressMode(conv::map_address_mode(t));
            descriptor.setRAddressMode(conv::map_address_mode(r));

            // Anisotropy is always supported on mac up to 16x
            descriptor.setMaxAnisotropy(desc.anisotropy_clamp as _);

            descriptor.setLodMinClamp(desc.lod_clamp.start);
            descriptor.setLodMaxClamp(desc.lod_clamp.end);

            if let Some(fun) = desc.compare {
                descriptor.setCompareFunction(conv::map_compare_function(fun));
            }

            if let Some(border_color) = desc.border_color {
                if let wgt::SamplerBorderColor::Zero = border_color {
                    if s == wgt::AddressMode::ClampToBorder {
                        descriptor.setSAddressMode(MTLSamplerAddressMode::ClampToZero);
                    }

                    if t == wgt::AddressMode::ClampToBorder {
                        descriptor.setTAddressMode(MTLSamplerAddressMode::ClampToZero);
                    }

                    if r == wgt::AddressMode::ClampToBorder {
                        descriptor.setRAddressMode(MTLSamplerAddressMode::ClampToZero);
                    }
                } else {
                    descriptor.setBorderColor(conv::map_border_color(border_color));
                }
            }

            if let Some(label) = desc.label {
                descriptor.setLabel(Some(&NSString::from_str(label)));
            }
            if self.features.contains(wgt::Features::TEXTURE_BINDING_ARRAY) {
                descriptor.setSupportArgumentBuffers(true);
            }
            let raw = self
                .shared
                .device
                .newSamplerStateWithDescriptor(&descriptor)
                .unwrap();

            self.counters.samplers.add(1);

            Ok(super::Sampler { raw })
        })
    }
    unsafe fn destroy_sampler(&self, _sampler: super::Sampler) {
        self.counters.samplers.sub(1);
    }

    unsafe fn create_command_encoder(
        &self,
        desc: &crate::CommandEncoderDescriptor<super::Queue>,
    ) -> Result<super::CommandEncoder, crate::DeviceError> {
        self.counters.command_encoders.add(1);
        Ok(super::CommandEncoder {
            shared: Arc::clone(&self.shared),
            queue_shared: Arc::clone(&desc.queue.shared),
            raw_cmd_buf: None,
            state: super::CommandState::default(),
            temp: super::Temp::default(),
            counters: Arc::clone(&self.counters),
        })
    }

    unsafe fn create_bind_group_layout(
        &self,
        desc: &crate::BindGroupLayoutDescriptor,
    ) -> DeviceResult<super::BindGroupLayout> {
        self.counters.bind_group_layouts.add(1);

        Ok(super::BindGroupLayout {
            entries: Arc::from(desc.entries),
        })
    }

    unsafe fn destroy_bind_group_layout(&self, _bg_layout: super::BindGroupLayout) {
        self.counters.bind_group_layouts.sub(1);
    }

    unsafe fn create_pipeline_layout(
        &self,
        desc: &crate::PipelineLayoutDescriptor<super::BindGroupLayout>,
    ) -> DeviceResult<super::PipelineLayout> {
        #[derive(Debug)]
        struct StageInfo {
            stage: naga::ShaderStage,
            counters: super::ResourceData<super::ResourceIndex>,
            pc_buffer: Option<super::ResourceIndex>,
            pc_limit: u32,
            sizes_buffer: Option<super::ResourceIndex>,
            need_sizes_buffer: bool,
            resources: naga::back::msl::BindingMap,
        }

        let mut stage_data = super::NAGA_STAGES.map(|stage| StageInfo {
            stage,
            counters: super::ResourceData::default(),
            pc_buffer: None,
            pc_limit: 0,
            sizes_buffer: None,
            need_sizes_buffer: false,
            resources: Default::default(),
        });
        let mut bind_group_infos = [const { None }; crate::MAX_BIND_GROUPS];

        // First, place the immediates
        for info in stage_data.iter_mut() {
            info.pc_limit = desc.immediate_size;

            // handle the immediate data buffer assignment and shader overrides
            if info.pc_limit != 0 {
                info.pc_buffer = Some(info.counters.buffers);
                info.counters.buffers += 1;
            }
        }

        // Second, place the described resources
        for (group_index, bgl) in desc.bind_group_layouts.iter().enumerate() {
            let Some(bgl) = bgl else {
                continue;
            };

            // remember where the resources for this set start at each shader stage
            let base_resource_indices = stage_data.map_ref(|info| info.counters.clone());

            for entry in bgl.entries.iter() {
                if let wgt::BindingType::Buffer {
                    ty: wgt::BufferBindingType::Storage { .. },
                    ..
                } = entry.ty
                {
                    for info in stage_data.iter_mut() {
                        if entry.visibility.contains(map_naga_stage(info.stage)) {
                            info.need_sizes_buffer = true;
                        }
                    }
                }

                for info in stage_data.iter_mut() {
                    if !entry.visibility.contains(map_naga_stage(info.stage)) {
                        continue;
                    }

                    let mut target = naga::back::msl::BindTarget::default();
                    // Bindless path
                    if let Some(_) = entry.count {
                        target.buffer = Some(info.counters.buffers as _);
                        info.counters.buffers += 1;
                    } else {
                        match entry.ty {
                            wgt::BindingType::Buffer { ty, .. } => {
                                target.buffer = Some(info.counters.buffers as _);
                                info.counters.buffers += 1;
                                if let wgt::BufferBindingType::Storage { read_only } = ty {
                                    target.mutable = !read_only;
                                }
                            }
                            wgt::BindingType::Sampler { .. } => {
                                target.sampler =
                                    Some(naga::back::msl::BindSamplerTarget::Resource(
                                        info.counters.samplers as _,
                                    ));
                                info.counters.samplers += 1;
                            }
                            wgt::BindingType::Texture { .. } => {
                                target.texture = Some(info.counters.textures as _);
                                info.counters.textures += 1;
                            }
                            wgt::BindingType::StorageTexture { access, .. } => {
                                target.texture = Some(info.counters.textures as _);
                                info.counters.textures += 1;
                                target.mutable = match access {
                                    wgt::StorageTextureAccess::ReadOnly => false,
                                    wgt::StorageTextureAccess::WriteOnly => true,
                                    wgt::StorageTextureAccess::ReadWrite => true,
                                    wgt::StorageTextureAccess::Atomic => true,
                                };
                            }
                            wgt::BindingType::AccelerationStructure { .. } => {
                                target.buffer = Some(info.counters.buffers as _);
                                info.counters.buffers += 1;
                            }
                            wgt::BindingType::ExternalTexture => {
                                target.external_texture =
                                    Some(naga::back::msl::BindExternalTextureTarget {
                                        planes: [
                                            info.counters.textures as _,
                                            (info.counters.textures + 1) as _,
                                            (info.counters.textures + 2) as _,
                                        ],
                                        params: info.counters.buffers as _,
                                    });
                                info.counters.textures += 3;
                                info.counters.buffers += 1;
                            }
                        }
                    }

                    let br = naga::ResourceBinding {
                        group: group_index as u32,
                        binding: entry.binding,
                    };
                    info.resources.insert(br, target);
                }
            }

            bind_group_infos[group_index] = Some(super::BindGroupLayoutInfo {
                base_resource_indices,
            });
        }

        // Finally, make sure we fit the limits
        for info in stage_data.iter_mut() {
            if info.need_sizes_buffer || info.stage == naga::ShaderStage::Vertex {
                // Set aside space for the sizes_buffer, which is required
                // for variable-length buffers, or to support vertex pulling.
                info.sizes_buffer = Some(info.counters.buffers);
                info.counters.buffers += 1;
            }

            if info.counters.buffers > self.shared.private_caps.max_buffers_per_stage
                || info.counters.textures > self.shared.private_caps.max_textures_per_stage
                || info.counters.samplers > self.shared.private_caps.max_samplers_per_stage
            {
                log::error!("Resource limit exceeded: {info:?}");
                return Err(crate::DeviceError::OutOfMemory);
            }
        }

        let immediates_infos = stage_data.map_ref(|info| {
            info.pc_buffer.map(|buffer_index| super::ImmediateDataInfo {
                count: info.pc_limit,
                buffer_index,
            })
        });

        let total_counters = stage_data.map_ref(|info| info.counters.clone());

        let per_stage_map = stage_data.map(|info| naga::back::msl::EntryPointResources {
            immediates_buffer: info
                .pc_buffer
                .map(|buffer_index| buffer_index as naga::back::msl::Slot),
            sizes_buffer: info
                .sizes_buffer
                .map(|buffer_index| buffer_index as naga::back::msl::Slot),
            resources: info.resources,
        });

        self.counters.pipeline_layouts.add(1);

        Ok(super::PipelineLayout {
            bind_group_infos,
            immediates_infos,
            total_counters,
            total_immediates: desc.immediate_size,
            per_stage_map,
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
    ) -> DeviceResult<super::BindGroup> {
        autoreleasepool(|_| {
            let mut bg = super::BindGroup::default();
            for (&stage, counter) in super::NAGA_STAGES.iter().zip(bg.counters.iter_mut()) {
                let stage_bit = map_naga_stage(stage);
                let mut dynamic_offsets_count = 0u32;
                let layout_and_entry_iter = desc.entries.iter().map(|entry| {
                    let layout = desc
                        .layout
                        .entries
                        .iter()
                        .find(|layout_entry| layout_entry.binding == entry.binding)
                        .expect("internal error: no layout entry found with binding slot");
                    (entry, layout)
                });
                for (entry, layout) in layout_and_entry_iter {
                    // Bindless path
                    if layout.count.is_some() {
                        if !layout.visibility.contains(stage_bit) {
                            continue;
                        }

                        let count = entry.count;

                        let stages = conv::map_render_stages(layout.visibility);
                        let uses = conv::map_resource_usage(&layout.ty);

                        // Create argument buffer for this array
                        let buffer = self
                            .shared
                            .device
                            .newBufferWithLength_options(
                                8 * count as usize,
                                MTLResourceOptions::HazardTrackingModeUntracked
                                    | MTLResourceOptions::StorageModeShared,
                            )
                            .unwrap();

                        let contents: &mut [MTLResourceID] = unsafe {
                            core::slice::from_raw_parts_mut(
                                buffer.contents().cast().as_ptr(),
                                count as usize,
                            )
                        };

                        match layout.ty {
                            wgt::BindingType::Texture { .. }
                            | wgt::BindingType::StorageTexture { .. } => {
                                let start = entry.resource_index as usize;
                                let end = start + count as usize;
                                let textures = &desc.textures[start..end];

                                for (idx, tex) in textures.iter().enumerate() {
                                    contents[idx] = tex.view.raw.gpuResourceID();

                                    let use_info = bg
                                        .resources_to_use
                                        .entry(tex.view.as_raw().cast())
                                        .or_default();
                                    use_info.stages |= stages;
                                    use_info.uses |= uses;
                                    use_info.visible_in_compute |=
                                        layout.visibility.contains(wgt::ShaderStages::COMPUTE);
                                }
                            }
                            wgt::BindingType::Sampler { .. } => {
                                let start = entry.resource_index as usize;
                                let end = start + count as usize;
                                let samplers = &desc.samplers[start..end];

                                for (idx, &sampler) in samplers.iter().enumerate() {
                                    contents[idx] = sampler.raw.gpuResourceID();
                                    // Samplers aren't resources like buffers and textures, so don't
                                    // need to be passed to useResource
                                }
                            }
                            wgt::BindingType::AccelerationStructure { .. } => {
                                let start = entry.resource_index as usize;
                                let end = start + count as usize;
                                let acceleration_structures =
                                    &desc.acceleration_structures[start..end];

                                for (idx, &acceleration_structure) in
                                    acceleration_structures.iter().enumerate()
                                {
                                    contents[idx] = acceleration_structure.raw.gpuResourceID();

                                    let use_info = bg
                                        .resources_to_use
                                        .entry(acceleration_structure.as_raw().cast())
                                        .or_default();
                                    use_info.stages |= stages;
                                    use_info.uses |= uses;
                                    use_info.visible_in_compute |=
                                        layout.visibility.contains(wgt::ShaderStages::COMPUTE);
                                }
                            }
                            _ => {
                                unimplemented!();
                            }
                        }

                        bg.buffers.push(super::BufferLikeResource::Buffer {
                            ptr: NonNull::from(&*buffer),
                            offset: 0,
                            dynamic_index: None,
                            binding_size: None,
                            binding_location: layout.binding,
                        });
                        counter.buffers += 1;

                        bg.argument_buffers.push(buffer)
                    }
                    // Bindfull path
                    else {
                        if let wgt::BindingType::Buffer {
                            has_dynamic_offset: true,
                            ..
                        } = layout.ty
                        {
                            dynamic_offsets_count += 1;
                        }
                        if !layout.visibility.contains(stage_bit) {
                            continue;
                        }
                        match layout.ty {
                            wgt::BindingType::Buffer {
                                ty,
                                has_dynamic_offset,
                                ..
                            } => {
                                let start = entry.resource_index as usize;
                                let end = start + 1;
                                bg.buffers
                                    .extend(desc.buffers[start..end].iter().map(|source| {
                                        // Given the restrictions on `BufferBinding::offset`,
                                        // this should never be `None`.
                                        let remaining_size = wgt::BufferSize::new(
                                            source.buffer.size - source.offset,
                                        );
                                        let binding_size = match ty {
                                            wgt::BufferBindingType::Storage { .. } => {
                                                source.size.or(remaining_size)
                                            }
                                            _ => None,
                                        };
                                        super::BufferLikeResource::Buffer {
                                            ptr: source.buffer.as_raw(),
                                            offset: source.offset,
                                            dynamic_index: if has_dynamic_offset {
                                                Some(dynamic_offsets_count - 1)
                                            } else {
                                                None
                                            },
                                            binding_size,
                                            binding_location: layout.binding,
                                        }
                                    }));
                                counter.buffers += 1;
                            }
                            wgt::BindingType::Sampler { .. } => {
                                let start = entry.resource_index as usize;
                                let end = start + 1;
                                bg.samplers.extend(
                                    desc.samplers[start..end].iter().map(|samp| samp.as_raw()),
                                );
                                counter.samplers += 1;
                            }
                            wgt::BindingType::Texture { .. }
                            | wgt::BindingType::StorageTexture { .. } => {
                                let start = entry.resource_index as usize;
                                let end = start + 1;
                                bg.textures.extend(
                                    desc.textures[start..end]
                                        .iter()
                                        .map(|tex| tex.view.as_raw()),
                                );
                                counter.textures += 1;
                            }
                            wgt::BindingType::AccelerationStructure { .. } => {
                                let start = entry.resource_index as usize;
                                let end = start + 1;
                                bg.buffers.extend(
                                    desc.acceleration_structures[start..end].iter().map(
                                        |acceleration_structure| {
                                            super::BufferLikeResource::AccelerationStructure(
                                                acceleration_structure.as_raw(),
                                            )
                                        },
                                    ),
                                );
                                counter.buffers += 1;
                            }
                            wgt::BindingType::ExternalTexture => {
                                // We don't yet support binding arrays of external textures.
                                // https://github.com/gfx-rs/wgpu/issues/8027
                                assert_eq!(entry.count, 1);
                                let external_texture =
                                    &desc.external_textures[entry.resource_index as usize];
                                bg.textures.extend(
                                    external_texture
                                        .planes
                                        .iter()
                                        .map(|plane| plane.view.as_raw()),
                                );
                                bg.buffers.push(super::BufferLikeResource::Buffer {
                                    ptr: external_texture.params.buffer.as_raw(),
                                    offset: external_texture.params.offset,
                                    dynamic_index: None,
                                    binding_size: None,
                                    binding_location: layout.binding,
                                });
                                counter.textures += 3;
                                counter.buffers += 1;
                            }
                        }
                    }
                }
            }

            self.counters.bind_groups.add(1);

            Ok(bg)
        })
    }

    unsafe fn destroy_bind_group(&self, _group: super::BindGroup) {
        self.counters.bind_groups.sub(1);
    }

    unsafe fn create_shader_module(
        &self,
        desc: &crate::ShaderModuleDescriptor,
        shader: crate::ShaderInput,
    ) -> Result<super::ShaderModule, crate::ShaderError> {
        self.counters.shader_modules.add(1);

        match shader {
            crate::ShaderInput::Naga(naga) => Ok(super::ShaderModule {
                source: ShaderModuleSource::Naga(naga),
                bounds_checks: desc.runtime_checks,
            }),
            crate::ShaderInput::MetalLib {
                file,
                num_workgroups,
            } => {
                // SAFETY: this creates a reference to `file` that is dropped before `file` is dropped.
                let library = super::library_from_metallib::new_library_from_metallib_bytes(
                    &self.shared.device,
                    file,
                )
                .map_err(|e| crate::ShaderError::Compilation(format!("Metallib: {e:?}")))?;
                Ok(super::ShaderModule {
                    source: ShaderModuleSource::Passthrough(PassthroughShader {
                        library,
                        num_workgroups,
                    }),
                    // This goes unused for passthrough shaders
                    bounds_checks: wgt::ShaderRuntimeChecks::unchecked(),
                })
            }
            crate::ShaderInput::Msl {
                shader: source,
                num_workgroups,
            } => {
                let options = MTLCompileOptions::new();
                // Obtain the device from shared
                let device = &self.shared.device;
                let library = device
                    .newLibraryWithSource_options_error(&NSString::from_str(source), Some(&options))
                    .map_err(|e| crate::ShaderError::Compilation(format!("MSL: {e:?}")))?;

                Ok(super::ShaderModule {
                    source: ShaderModuleSource::Passthrough(PassthroughShader {
                        library,
                        num_workgroups,
                    }),
                    // This goes unused for passthrough shaders
                    bounds_checks: wgt::ShaderRuntimeChecks::unchecked(),
                })
            }
            crate::ShaderInput::SpirV(_)
            | crate::ShaderInput::Dxil { .. }
            | crate::ShaderInput::Hlsl { .. }
            | crate::ShaderInput::Glsl { .. } => unreachable!(),
        }
    }

    unsafe fn destroy_shader_module(&self, _module: super::ShaderModule) {
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
        autoreleasepool(|_| {
            enum MetalGenericRenderPipelineDescriptor {
                Standard(Retained<MTLRenderPipelineDescriptor>),
                Mesh(Retained<MTLMeshRenderPipelineDescriptor>),
            }
            macro_rules! descriptor_fn {
                ($descriptor:ident . $method:ident $( ( $($args:expr),* ) )? ) => {
                    match $descriptor {
                        MetalGenericRenderPipelineDescriptor::Standard(ref inner) => inner.$method$(($($args),*))?,
                        MetalGenericRenderPipelineDescriptor::Mesh(ref inner) => inner.$method$(($($args),*))?,
                    }
                };
            }
            #[allow(non_snake_case)]
            impl MetalGenericRenderPipelineDescriptor {
                unsafe fn setFragmentFunction(
                    &self,
                    function: Option<&ProtocolObject<dyn MTLFunction>>,
                ) {
                    unsafe { descriptor_fn!(self.setFragmentFunction(function)) };
                }
                fn fragmentBuffers(&self) -> Retained<MTLPipelineBufferDescriptorArray> {
                    descriptor_fn!(self.fragmentBuffers())
                }
                fn setDepthAttachmentPixelFormat(&self, pixel_format: MTLPixelFormat) {
                    descriptor_fn!(self.setDepthAttachmentPixelFormat(pixel_format));
                }
                fn colorAttachments(
                    &self,
                ) -> Retained<MTLRenderPipelineColorAttachmentDescriptorArray> {
                    descriptor_fn!(self.colorAttachments())
                }
                fn setStencilAttachmentPixelFormat(&self, pixel_format: MTLPixelFormat) {
                    descriptor_fn!(self.setStencilAttachmentPixelFormat(pixel_format));
                }
                fn setAlphaToCoverageEnabled(&self, enabled: bool) {
                    descriptor_fn!(self.setAlphaToCoverageEnabled(enabled));
                }
                fn setLabel(&self, label: Option<&NSString>) {
                    descriptor_fn!(self.setLabel(label));
                }
                unsafe fn setMaxVertexAmplificationCount(&self, count: NSUInteger) {
                    unsafe { descriptor_fn!(self.setMaxVertexAmplificationCount(count)) }
                }
            }

            // https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptor/mutability
            let supports_mutability =
                available!(macos = 10.13, ios = 11.0, tvos = 11.0, visionos = 1.0);

            let (primitive_class, raw_primitive_type) =
                conv::map_primitive_topology(desc.primitive.topology);

            let vs_info;
            let ts_info;
            let ms_info;

            // Create the pipeline descriptor and do vertex/mesh pipeline specific setup
            let descriptor = match desc.vertex_processor {
                crate::VertexProcessor::Standard {
                    vertex_buffers,
                    ref vertex_stage,
                } => {
                    // Vertex pipeline specific setup

                    let descriptor = MTLRenderPipelineDescriptor::new();
                    ts_info = None;
                    ms_info = None;

                    // Collect vertex buffer mappings
                    let mut vertex_buffer_mappings =
                        Vec::<naga::back::msl::VertexBufferMapping>::new();
                    for (i, vbl) in vertex_buffers.iter().enumerate() {
                        let mut attributes = Vec::<naga::back::msl::AttributeMapping>::new();
                        for attribute in vbl.attributes.iter() {
                            attributes.push(naga::back::msl::AttributeMapping {
                                shader_location: attribute.shader_location,
                                offset: attribute.offset as u32,
                                format: convert_vertex_format_to_naga(attribute.format),
                            });
                        }

                        let mapping = naga::back::msl::VertexBufferMapping {
                            id: self.shared.private_caps.max_vertex_buffers - 1 - i as u32,
                            stride: if vbl.array_stride > 0 {
                                vbl.array_stride.try_into().unwrap()
                            } else {
                                vbl.attributes
                                    .iter()
                                    .map(|attribute| attribute.offset + attribute.format.size())
                                    .max()
                                    .unwrap_or(0)
                                    .try_into()
                                    .unwrap()
                            },
                            step_mode: match (vbl.array_stride == 0, vbl.step_mode) {
                                (true, _) => naga::back::msl::VertexBufferStepMode::Constant,
                                (false, wgt::VertexStepMode::Vertex) => {
                                    naga::back::msl::VertexBufferStepMode::ByVertex
                                }
                                (false, wgt::VertexStepMode::Instance) => {
                                    naga::back::msl::VertexBufferStepMode::ByInstance
                                }
                            },
                            attributes,
                        };
                        vertex_buffer_mappings.push(mapping);
                    }

                    // Setup vertex shader
                    {
                        let vs = self.load_shader(
                            vertex_stage,
                            &vertex_buffer_mappings,
                            desc.layout,
                            primitive_class,
                            naga::ShaderStage::Vertex,
                        )?;

                        descriptor.setVertexFunction(Some(&vs.function));

                        if supports_mutability {
                            Self::set_buffers_mutability(
                                &descriptor.vertexBuffers(),
                                vs.immutable_buffer_mask,
                            );
                        }

                        vs_info = Some(super::PipelineStageInfo {
                            immediates: desc.layout.immediates_infos.vs,
                            sizes_slot: desc.layout.per_stage_map.vs.sizes_buffer,
                            sized_bindings: vs.sized_bindings,
                            vertex_buffer_mappings,
                            library: Some(vs.library),
                            raw_wg_size: MTLSize {
                                width: 0,
                                height: 0,
                                depth: 0,
                            },
                            work_group_memory_sizes: vec![],
                        });
                    }

                    // Validate vertex buffer count
                    if desc.layout.total_counters.vs.buffers + (vertex_buffers.len() as u32)
                        > self.shared.private_caps.max_vertex_buffers
                    {
                        let msg = format!(
                            "pipeline needs too many buffers in the vertex stage: {} vertex and {} layout",
                            vertex_buffers.len(),
                            desc.layout.total_counters.vs.buffers
                        );
                        return Err(crate::PipelineError::Linkage(
                            wgt::ShaderStages::VERTEX,
                            msg,
                        ));
                    }

                    // Set the pipeline vertex buffer info
                    if !vertex_buffers.is_empty() {
                        let vertex_descriptor = MTLVertexDescriptor::new();
                        for (i, vb) in vertex_buffers.iter().enumerate() {
                            let buffer_index =
                                self.shared.private_caps.max_vertex_buffers as usize - 1 - i;
                            let buffer_desc = unsafe {
                                vertex_descriptor
                                    .layouts()
                                    .objectAtIndexedSubscript(buffer_index)
                            };

                            // Metal expects the stride to be the actual size of the attributes.
                            // The semantics of array_stride == 0 can be achieved by setting
                            // the step function to constant and rate to 0.
                            if vb.array_stride == 0 {
                                let stride = vb
                                    .attributes
                                    .iter()
                                    .map(|attribute| attribute.offset + attribute.format.size())
                                    .max()
                                    .unwrap_or(0);
                                unsafe {
                                    buffer_desc.setStride(wgt::math::align_to(stride as _, 4))
                                };
                                buffer_desc.setStepFunction(MTLVertexStepFunction::Constant);
                                unsafe { buffer_desc.setStepRate(0) };
                            } else {
                                unsafe { buffer_desc.setStride(vb.array_stride as _) };
                                buffer_desc.setStepFunction(conv::map_step_mode(vb.step_mode));
                            }

                            for at in vb.attributes {
                                let attribute_desc = unsafe {
                                    vertex_descriptor
                                        .attributes()
                                        .objectAtIndexedSubscript(at.shader_location as _)
                                };
                                attribute_desc.setFormat(conv::map_vertex_format(at.format));
                                unsafe { attribute_desc.setBufferIndex(buffer_index) };
                                unsafe { attribute_desc.setOffset(at.offset as _) };
                            }
                        }
                        descriptor.setVertexDescriptor(Some(&vertex_descriptor));
                    }

                    MetalGenericRenderPipelineDescriptor::Standard(descriptor)
                }
                crate::VertexProcessor::Mesh {
                    ref task_stage,
                    ref mesh_stage,
                } => {
                    // Mesh pipeline specific setup

                    vs_info = None;
                    let descriptor = MTLMeshRenderPipelineDescriptor::new();

                    // Setup task stage
                    if let Some(ref task_stage) = task_stage {
                        let ts = self.load_shader(
                            task_stage,
                            &[],
                            desc.layout,
                            primitive_class,
                            naga::ShaderStage::Task,
                        )?;
                        unsafe { descriptor.setObjectFunction(Some(&ts.function)) };
                        if supports_mutability {
                            Self::set_buffers_mutability(
                                &descriptor.meshBuffers(),
                                ts.immutable_buffer_mask,
                            );
                        }
                        ts_info = Some(super::PipelineStageInfo {
                            immediates: desc.layout.immediates_infos.ts,
                            sizes_slot: desc.layout.per_stage_map.ts.sizes_buffer,
                            sized_bindings: ts.sized_bindings,
                            vertex_buffer_mappings: vec![],
                            library: Some(ts.library),
                            raw_wg_size: ts.wg_size,
                            work_group_memory_sizes: ts.wg_memory_sizes,
                        });
                    } else {
                        ts_info = None;
                    }

                    // Setup mesh stage
                    {
                        let ms = self.load_shader(
                            mesh_stage,
                            &[],
                            desc.layout,
                            primitive_class,
                            naga::ShaderStage::Mesh,
                        )?;
                        unsafe { descriptor.setMeshFunction(Some(&ms.function)) };
                        if supports_mutability {
                            Self::set_buffers_mutability(
                                &descriptor.meshBuffers(),
                                ms.immutable_buffer_mask,
                            );
                        }
                        ms_info = Some(super::PipelineStageInfo {
                            immediates: desc.layout.immediates_infos.ms,
                            sizes_slot: desc.layout.per_stage_map.ms.sizes_buffer,
                            sized_bindings: ms.sized_bindings,
                            vertex_buffer_mappings: vec![],
                            library: Some(ms.library),
                            raw_wg_size: ms.wg_size,
                            work_group_memory_sizes: ms.wg_memory_sizes,
                        });
                    }

                    MetalGenericRenderPipelineDescriptor::Mesh(descriptor)
                }
            };

            let raw_triangle_fill_mode = match desc.primitive.polygon_mode {
                wgt::PolygonMode::Fill => MTLTriangleFillMode::Fill,
                wgt::PolygonMode::Line => MTLTriangleFillMode::Lines,
                wgt::PolygonMode::Point => panic!(
                    "{:?} is not enabled for this backend",
                    wgt::Features::POLYGON_MODE_POINT
                ),
            };

            // Fragment shader
            let fs_info = match desc.fragment_stage {
                Some(ref stage) => {
                    let fs = self.load_shader(
                        stage,
                        &[],
                        desc.layout,
                        primitive_class,
                        naga::ShaderStage::Fragment,
                    )?;

                    unsafe { descriptor.setFragmentFunction(Some(&fs.function)) };
                    if supports_mutability {
                        Self::set_buffers_mutability(
                            &descriptor.fragmentBuffers(),
                            fs.immutable_buffer_mask,
                        );
                    }

                    Some(super::PipelineStageInfo {
                        immediates: desc.layout.immediates_infos.fs,
                        sizes_slot: desc.layout.per_stage_map.fs.sizes_buffer,
                        sized_bindings: fs.sized_bindings,
                        vertex_buffer_mappings: vec![],
                        library: Some(fs.library),
                        raw_wg_size: MTLSize {
                            width: 0,
                            height: 0,
                            depth: 0,
                        },
                        work_group_memory_sizes: vec![],
                    })
                }
                None => {
                    // TODO: This is a workaround for what appears to be a Metal validation bug
                    // A pixel format is required even though no attachments are provided
                    if desc.color_targets.is_empty() && desc.depth_stencil.is_none() {
                        descriptor.setDepthAttachmentPixelFormat(MTLPixelFormat::Depth32Float);
                    }
                    None
                }
            };

            // Setup pipeline color attachments
            for (i, ct) in desc.color_targets.iter().enumerate() {
                let at_descriptor =
                    unsafe { descriptor.colorAttachments().objectAtIndexedSubscript(i) };
                let ct = if let Some(color_target) = ct.as_ref() {
                    color_target
                } else {
                    at_descriptor.setPixelFormat(MTLPixelFormat::Invalid);
                    continue;
                };

                let raw_format = self
                    .shared
                    .private_texture_format_caps
                    .map_format(ct.format);
                at_descriptor.setPixelFormat(raw_format);
                at_descriptor.setWriteMask(conv::map_color_write(ct.write_mask));

                if let Some(ref blend) = ct.blend {
                    at_descriptor.setBlendingEnabled(true);
                    let (color_op, color_src, color_dst) = conv::map_blend_component(&blend.color);
                    let (alpha_op, alpha_src, alpha_dst) = conv::map_blend_component(&blend.alpha);

                    at_descriptor.setRgbBlendOperation(color_op);
                    at_descriptor.setSourceRGBBlendFactor(color_src);
                    at_descriptor.setDestinationRGBBlendFactor(color_dst);

                    at_descriptor.setAlphaBlendOperation(alpha_op);
                    at_descriptor.setSourceAlphaBlendFactor(alpha_src);
                    at_descriptor.setDestinationAlphaBlendFactor(alpha_dst);
                }
            }

            // Setup depth stencil state
            let depth_stencil = match desc.depth_stencil {
                Some(ref ds) => {
                    let raw_format = self
                        .shared
                        .private_texture_format_caps
                        .map_format(ds.format);
                    let aspects = crate::FormatAspects::from(ds.format);
                    if aspects.contains(crate::FormatAspects::DEPTH) {
                        descriptor.setDepthAttachmentPixelFormat(raw_format);
                    }
                    if aspects.contains(crate::FormatAspects::STENCIL) {
                        descriptor.setStencilAttachmentPixelFormat(raw_format);
                    }

                    let ds_descriptor = create_depth_stencil_desc(ds);
                    let raw = self
                        .shared
                        .device
                        .newDepthStencilStateWithDescriptor(&ds_descriptor)
                        .unwrap();
                    Some((raw, ds.bias))
                }
                None => None,
            };

            // Setup multisample state
            if desc.multisample.count != 1 {
                //TODO: handle sample mask
                match descriptor {
                    MetalGenericRenderPipelineDescriptor::Standard(ref inner) => {
                        #[allow(deprecated)]
                        inner.setSampleCount(desc.multisample.count as _);
                    }
                    MetalGenericRenderPipelineDescriptor::Mesh(ref inner) => {
                        unsafe { inner.setRasterSampleCount(desc.multisample.count as _) };
                    }
                }
                descriptor.setAlphaToCoverageEnabled(desc.multisample.alpha_to_coverage_enabled);
                //descriptor.set_alpha_to_one_enabled(desc.multisample.alpha_to_one_enabled);
            }

            // Set debug label
            if let Some(name) = desc.label {
                descriptor.setLabel(Some(&NSString::from_str(name)));
            }
            if let Some(mv) = desc.multiview_mask {
                unsafe {
                    descriptor.setMaxVertexAmplificationCount(mv.get().count_ones() as usize)
                };
            }

            // Create the pipeline from descriptor
            let raw = match descriptor {
                MetalGenericRenderPipelineDescriptor::Standard(d) => self
                    .shared
                    .device
                    .newRenderPipelineStateWithDescriptor_error(&d),
                MetalGenericRenderPipelineDescriptor::Mesh(d) => self
                    .shared
                    .device
                    .newRenderPipelineStateWithMeshDescriptor_options_reflection_error(
                        &d,
                        MTLPipelineOption::empty(),
                        None,
                    ),
            }
            .map_err(|e| {
                crate::PipelineError::Linkage(
                    wgt::ShaderStages::VERTEX | wgt::ShaderStages::FRAGMENT,
                    format!("new_render_pipeline_state: {e:?}"),
                )
            })?;

            self.counters.render_pipelines.add(1);

            Ok(super::RenderPipeline {
                raw,
                vs_info,
                fs_info,
                ts_info,
                ms_info,
                raw_primitive_type,
                raw_triangle_fill_mode,
                raw_front_winding: conv::map_winding(desc.primitive.front_face),
                raw_cull_mode: conv::map_cull_mode(desc.primitive.cull_mode),
                raw_depth_clip_mode: if self.features.contains(wgt::Features::DEPTH_CLIP_CONTROL) {
                    Some(if desc.primitive.unclipped_depth {
                        MTLDepthClipMode::Clamp
                    } else {
                        MTLDepthClipMode::Clip
                    })
                } else {
                    None
                },
                depth_stencil,
            })
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
        autoreleasepool(|_| {
            let descriptor = MTLComputePipelineDescriptor::new();

            let module = desc.stage.module;
            let cs = if let ShaderModuleSource::Passthrough(passthrough_desc) = &module.source {
                CompiledShader {
                    library: passthrough_desc.library.clone(),
                    function: passthrough_desc
                        .library
                        .newFunctionWithName(&NSString::from_str(desc.stage.entry_point))
                        .ok_or(crate::PipelineError::EntryPoint(naga::ShaderStage::Compute))?,
                    wg_size: MTLSize {
                        width: passthrough_desc.num_workgroups.0 as usize,
                        height: passthrough_desc.num_workgroups.1 as usize,
                        depth: passthrough_desc.num_workgroups.2 as usize,
                    },
                    wg_memory_sizes: vec![],
                    sized_bindings: vec![],
                    immutable_buffer_mask: 0,
                }
            } else {
                self.load_shader(
                    &desc.stage,
                    &[],
                    desc.layout,
                    MTLPrimitiveTopologyClass::Unspecified,
                    naga::ShaderStage::Compute,
                )?
            };

            descriptor.setComputeFunction(Some(&cs.function));

            // https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptor/mutability
            if available!(macos = 10.13, ios = 11.0, tvos = 11.0, visionos = 1.0) {
                Self::set_buffers_mutability(&descriptor.buffers(), cs.immutable_buffer_mask);
            }

            let cs_info = super::PipelineStageInfo {
                library: Some(cs.library),
                immediates: desc.layout.immediates_infos.cs,
                sizes_slot: desc.layout.per_stage_map.cs.sizes_buffer,
                sized_bindings: cs.sized_bindings,
                vertex_buffer_mappings: vec![],
                raw_wg_size: cs.wg_size,
                work_group_memory_sizes: cs.wg_memory_sizes,
            };

            if let Some(name) = desc.label {
                descriptor.setLabel(Some(&NSString::from_str(name)));
            }

            let raw = self
                .shared
                .device
                .newComputePipelineStateWithDescriptor_options_reflection_error(
                    &descriptor,
                    MTLPipelineOption::empty(),
                    None,
                );

            let raw: Retained<ProtocolObject<dyn MTLComputePipelineState>> =
                raw.map_err(|e: Retained<NSError>| {
                    crate::PipelineError::Linkage(
                        wgt::ShaderStages::COMPUTE,
                        format!("new_compute_pipeline_state: {e:?}"),
                    )
                })?;

            self.counters.compute_pipelines.add(1);

            Ok(super::ComputePipeline { raw, cs_info })
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
    ) -> DeviceResult<super::QuerySet> {
        autoreleasepool(|_| {
            match desc.ty {
                wgt::QueryType::Occlusion => {
                    let size = desc.count as u64 * crate::QUERY_SIZE;
                    let options = MTLResourceOptions::empty();
                    //TODO: HazardTrackingModeUntracked
                    let raw_buffer = self
                        .shared
                        .device
                        .newBufferWithLength_options(size as usize, options)
                        .unwrap();
                    if let Some(label) = desc.label {
                        raw_buffer.setLabel(Some(&NSString::from_str(label)));
                    }
                    Ok(super::QuerySet {
                        raw_buffer,
                        counter_sample_buffer: None,
                        ty: desc.ty,
                    })
                }
                wgt::QueryType::Timestamp => {
                    let size = desc.count as u64 * crate::QUERY_SIZE;
                    let device = &self.shared.device;
                    let destination_buffer = device
                        .newBufferWithLength_options(size as usize, MTLResourceOptions::empty())
                        .unwrap();

                    let csb_desc = MTLCounterSampleBufferDescriptor::new();
                    csb_desc.setStorageMode(MTLStorageMode::Shared);
                    unsafe { csb_desc.setSampleCount(desc.count as _) };
                    if let Some(label) = desc.label {
                        csb_desc.setLabel(&NSString::from_str(label));
                    }

                    let counter_sets = device.counterSets().unwrap();
                    let timestamp_counter = match counter_sets
                        .iter()
                        .find(|cs| &*cs.name() == ns_string!("timestamp"))
                    {
                        Some(counter) => counter,
                        None => {
                            log::error!("Failed to obtain timestamp counter set.");
                            return Err(crate::DeviceError::Unexpected);
                        }
                    };
                    csb_desc.setCounterSet(Some(&timestamp_counter));

                    let counter_sample_buffer =
                        match device.newCounterSampleBufferWithDescriptor_error(&csb_desc) {
                            Ok(buffer) => buffer,
                            Err(err) => {
                                log::error!("Failed to create counter sample buffer: {err:?}");
                                return Err(crate::DeviceError::Unexpected);
                            }
                        };

                    self.counters.query_sets.add(1);

                    Ok(super::QuerySet {
                        raw_buffer: destination_buffer,
                        counter_sample_buffer: Some(counter_sample_buffer),
                        ty: desc.ty,
                    })
                }
                _ => {
                    todo!()
                }
            }
        })
    }

    unsafe fn destroy_query_set(&self, _set: super::QuerySet) {
        self.counters.query_sets.sub(1);
    }

    unsafe fn create_fence(&self) -> DeviceResult<super::Fence> {
        self.counters.fences.add(1);
        // https://developer.apple.com/documentation/metal/mtlsharedevent
        let shared_event = if available!(macos = 10.14, ios = 12.0, tvos = 12.0, visionos = 1.0) {
            self.shared.device.newSharedEvent() // This should be supported on said devices, but some sandbox environments may still restrict it, making it return `None`.
        } else {
            None
        };
        Ok(super::Fence {
            completed_value: Arc::new(atomic::AtomicU64::new(0)),
            pending_command_buffers: Vec::new(),
            shared_event,
        })
    }

    unsafe fn destroy_fence(&self, _fence: super::Fence) {
        self.counters.fences.sub(1);
    }

    unsafe fn get_fence_value(&self, fence: &super::Fence) -> DeviceResult<crate::FenceValue> {
        let mut max_value = fence.completed_value.load(atomic::Ordering::Acquire);
        for &(value, ref cmd_buf) in fence.pending_command_buffers.iter() {
            if cmd_buf.status() == MTLCommandBufferStatus::Completed {
                max_value = value;
            }
        }
        Ok(max_value)
    }
    unsafe fn wait(
        &self,
        fence: &super::Fence,
        wait_value: crate::FenceValue,
        timeout: Option<core::time::Duration>,
    ) -> DeviceResult<bool> {
        if wait_value <= fence.completed_value.load(atomic::Ordering::Acquire) {
            return Ok(true);
        }

        let cmd_buf = match fence
            .pending_command_buffers
            .iter()
            .find(|&&(value, _)| value >= wait_value)
        {
            Some((_, cmd_buf)) => cmd_buf,
            None => {
                log::error!("No active command buffers for fence value {wait_value}");
                return Err(crate::DeviceError::Lost);
            }
        };

        let start = time::Instant::now();
        loop {
            if let MTLCommandBufferStatus::Completed = cmd_buf.status() {
                return Ok(true);
            }
            if let Some(timeout) = timeout {
                if start.elapsed() >= timeout {
                    return Ok(false);
                }
            }
            thread::sleep(core::time::Duration::from_millis(1));
        }
    }

    unsafe fn start_graphics_debugger_capture(&self) -> bool {
        // https://developer.apple.com/documentation/metal/mtlcapturemanager
        if !available!(macos = 10.13, ios = 11.0, tvos = 11.0, visionos = 1.0) {
            return false;
        }
        let device = &self.shared.device;
        let shared_capture_manager = unsafe { MTLCaptureManager::sharedCaptureManager() };
        let default_capture_scope = shared_capture_manager.newCaptureScopeWithDevice(device);
        shared_capture_manager.setDefaultCaptureScope(Some(&default_capture_scope));
        #[allow(deprecated)]
        shared_capture_manager.startCaptureWithScope(&default_capture_scope);
        default_capture_scope.beginScope();
        true
    }

    unsafe fn stop_graphics_debugger_capture(&self) {
        let shared_capture_manager = unsafe { MTLCaptureManager::sharedCaptureManager() };
        if let Some(default_capture_scope) = shared_capture_manager.defaultCaptureScope() {
            default_capture_scope.endScope();
        }
        shared_capture_manager.stopCapture();
    }

    unsafe fn get_acceleration_structure_build_sizes(
        &self,
        descriptor: &crate::GetAccelerationStructureBuildSizesDescriptor<super::Buffer>,
    ) -> crate::AccelerationStructureBuildSizes {
        let acceleration_structure_descriptor =
            conv::map_acceleration_structure_descriptor(descriptor.entries, descriptor.flags);
        let info = self
            .shared
            .device
            .accelerationStructureSizesWithDescriptor(&acceleration_structure_descriptor);
        crate::AccelerationStructureBuildSizes {
            acceleration_structure_size: info.accelerationStructureSize as u64,
            update_scratch_size: info.refitScratchBufferSize as u64,
            build_scratch_size: info.buildScratchBufferSize as u64,
        }
    }

    unsafe fn get_acceleration_structure_device_address(
        &self,
        acceleration_structure: &super::AccelerationStructure,
    ) -> wgt::BufferAddress {
        acceleration_structure.raw.gpuResourceID().to_raw()
    }

    unsafe fn create_acceleration_structure(
        &self,
        descriptor: &crate::AccelerationStructureDescriptor,
    ) -> Result<super::AccelerationStructure, crate::DeviceError> {
        // self.counters.acceleration_structures.add(1);
        autoreleasepool(|_| {
            Ok(super::AccelerationStructure {
                raw: self
                    .shared
                    .device
                    .newAccelerationStructureWithSize(descriptor.size as usize)
                    .ok_or(crate::DeviceError::OutOfMemory)?,
            })
        })
    }

    unsafe fn destroy_acceleration_structure(
        &self,
        _acceleration_structure: super::AccelerationStructure,
    ) {
        // self.counters.acceleration_structures.sub(1);
    }

    fn tlas_instance_to_bytes(&self, instance: TlasInstance) -> Vec<u8> {
        let temp = MTLIndirectAccelerationStructureInstanceDescriptor {
            transformationMatrix: MTLPackedFloat4x3 {
                columns: [
                    MTLPackedFloat3 {
                        x: instance.transform[0],
                        y: instance.transform[4],
                        z: instance.transform[8],
                    },
                    MTLPackedFloat3 {
                        x: instance.transform[1],
                        y: instance.transform[5],
                        z: instance.transform[9],
                    },
                    MTLPackedFloat3 {
                        x: instance.transform[2],
                        y: instance.transform[6],
                        z: instance.transform[10],
                    },
                    MTLPackedFloat3 {
                        x: instance.transform[3],
                        y: instance.transform[7],
                        z: instance.transform[11],
                    },
                ],
            },
            options: MTLAccelerationStructureInstanceOptions::None,
            mask: instance.mask as u32,
            intersectionFunctionTableOffset: 0,
            userID: instance.custom_data,
            accelerationStructureID: unsafe { MTLResourceID::from_raw(instance.blas_address) },
        };

        wgt::bytemuck_wrapper!(unsafe struct Desc(MTLIndirectAccelerationStructureInstanceDescriptor));

        bytemuck::bytes_of(&Desc::wrap(temp)).to_vec()
    }

    fn get_internal_counters(&self) -> wgt::HalCounters {
        self.counters.as_ref().clone()
    }

    fn check_if_oom(&self) -> Result<(), crate::DeviceError> {
        // TODO: see https://github.com/gfx-rs/wgpu/issues/7460

        Ok(())
    }
}
