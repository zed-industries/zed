use alloc::{
    borrow::ToOwned, format, string::String, string::ToString as _, sync::Arc, vec, vec::Vec,
};
use core::{cmp::max, convert::TryInto, num::NonZeroU32, ptr, sync::atomic::Ordering};

use arrayvec::ArrayVec;
use glow::HasContext;
use naga::FastHashMap;

use super::{conv, lock, MaybeMutex, PrivateCapabilities};
use crate::auxil::map_naga_stage;
use crate::TlasInstance;

type ShaderStage<'a> = (
    naga::ShaderStage,
    &'a crate::ProgrammableStage<'a, super::ShaderModule>,
);
type NameBindingMap = FastHashMap<String, (super::BindingRegister, u8)>;

struct CompilationContext<'a> {
    layout: &'a super::PipelineLayout,
    sampler_map: &'a mut super::SamplerBindMap,
    name_binding_map: &'a mut NameBindingMap,
    immediates_items: &'a mut Vec<naga::back::glsl::ImmediateItem>,
    multiview_mask: Option<NonZeroU32>,
    clip_distance_count: &'a mut u32,
}

impl CompilationContext<'_> {
    fn consume_reflection(
        self,
        gl: &glow::Context,
        module: &naga::Module,
        ep_info: &naga::valid::FunctionInfo,
        reflection_info: naga::back::glsl::ReflectionInfo,
        naga_stage: naga::ShaderStage,
        program: glow::Program,
    ) {
        for (handle, var) in module.global_variables.iter() {
            if ep_info[handle].is_empty() {
                continue;
            }
            let register = match var.space {
                naga::AddressSpace::Uniform => super::BindingRegister::UniformBuffers,
                naga::AddressSpace::Storage { .. } => super::BindingRegister::StorageBuffers,
                _ => continue,
            };

            let br = var.binding.as_ref().unwrap();
            let slot = self.layout.get_slot(br);

            let name = match reflection_info.uniforms.get(&handle) {
                Some(name) => name.clone(),
                None => continue,
            };
            log::trace!(
                "Rebind buffer: {:?} -> {}, register={:?}, slot={}",
                var.name.as_ref(),
                &name,
                register,
                slot
            );
            self.name_binding_map.insert(name, (register, slot));
        }

        for (name, mapping) in reflection_info.texture_mapping {
            let var = &module.global_variables[mapping.texture];
            let register = match module.types[var.ty].inner {
                naga::TypeInner::Image {
                    class: naga::ImageClass::Storage { .. },
                    ..
                } => super::BindingRegister::Images,
                _ => super::BindingRegister::Textures,
            };

            let tex_br = var.binding.as_ref().unwrap();
            let texture_linear_index = self.layout.get_slot(tex_br);

            self.name_binding_map
                .insert(name, (register, texture_linear_index));
            if let Some(sampler_handle) = mapping.sampler {
                let sam_br = module.global_variables[sampler_handle]
                    .binding
                    .as_ref()
                    .unwrap();
                let sampler_linear_index = self.layout.get_slot(sam_br);
                self.sampler_map[texture_linear_index as usize] = Some(sampler_linear_index);
            }
        }

        for (name, location) in reflection_info.varying {
            match naga_stage {
                naga::ShaderStage::Vertex => {
                    assert_eq!(location.index, 0);
                    unsafe { gl.bind_attrib_location(program, location.location, &name) }
                }
                naga::ShaderStage::Fragment => {
                    assert_eq!(location.index, 0);
                    unsafe { gl.bind_frag_data_location(program, location.location, &name) }
                }
                naga::ShaderStage::Compute => {}
                naga::ShaderStage::Task
                | naga::ShaderStage::Mesh
                | naga::ShaderStage::RayGeneration
                | naga::ShaderStage::AnyHit
                | naga::ShaderStage::ClosestHit
                | naga::ShaderStage::Miss => unreachable!(),
            }
        }

        *self.immediates_items = reflection_info.immediates_items;

        if naga_stage == naga::ShaderStage::Vertex {
            *self.clip_distance_count = reflection_info.clip_distance_count;
        }
    }
}

impl super::Device {
    /// # Safety
    ///
    /// - `name` must be created respecting `desc`
    /// - `name` must be a texture
    /// - If `drop_callback` is [`None`], wgpu-hal will take ownership of the texture. If
    ///   `drop_callback` is [`Some`], the texture must be valid until the callback is called.
    #[cfg(any(native, Emscripten))]
    pub unsafe fn texture_from_raw(
        &self,
        name: NonZeroU32,
        desc: &crate::TextureDescriptor,
        drop_callback: Option<crate::DropCallback>,
    ) -> super::Texture {
        super::Texture {
            inner: super::TextureInner::Texture {
                raw: glow::NativeTexture(name),
                target: super::Texture::get_info_from_desc(desc),
            },
            drop_guard: crate::DropGuard::from_option(drop_callback),
            mip_level_count: desc.mip_level_count,
            array_layer_count: desc.array_layer_count(),
            format: desc.format,
            format_desc: self.shared.describe_texture_format(desc.format),
            copy_size: desc.copy_extent(),
        }
    }

    /// # Safety
    ///
    /// - `name` must be created respecting `desc`
    /// - `name` must be a renderbuffer
    /// - If `drop_callback` is [`None`], wgpu-hal will take ownership of the renderbuffer. If
    ///   `drop_callback` is [`Some`], the renderbuffer must be valid until the callback is called.
    #[cfg(any(native, Emscripten))]
    pub unsafe fn texture_from_raw_renderbuffer(
        &self,
        name: NonZeroU32,
        desc: &crate::TextureDescriptor,
        drop_callback: Option<crate::DropCallback>,
    ) -> super::Texture {
        super::Texture {
            inner: super::TextureInner::Renderbuffer {
                raw: glow::NativeRenderbuffer(name),
            },
            drop_guard: crate::DropGuard::from_option(drop_callback),
            mip_level_count: desc.mip_level_count,
            array_layer_count: desc.array_layer_count(),
            format: desc.format,
            format_desc: self.shared.describe_texture_format(desc.format),
            copy_size: desc.copy_extent(),
        }
    }

    unsafe fn compile_shader(
        gl: &glow::Context,
        shader: &str,
        naga_stage: naga::ShaderStage,
        #[cfg_attr(target_arch = "wasm32", allow(unused))] label: Option<&str>,
    ) -> Result<glow::Shader, crate::PipelineError> {
        let target = match naga_stage {
            naga::ShaderStage::Vertex => glow::VERTEX_SHADER,
            naga::ShaderStage::Fragment => glow::FRAGMENT_SHADER,
            naga::ShaderStage::Compute => glow::COMPUTE_SHADER,
            naga::ShaderStage::Task
            | naga::ShaderStage::Mesh
            | naga::ShaderStage::RayGeneration
            | naga::ShaderStage::AnyHit
            | naga::ShaderStage::ClosestHit
            | naga::ShaderStage::Miss => unreachable!(),
        };

        let raw = unsafe { gl.create_shader(target) }.unwrap();
        #[cfg(native)]
        if gl.supports_debug() {
            let name = raw.0.get();
            unsafe { gl.object_label(glow::SHADER, name, label) };
        }

        unsafe { gl.shader_source(raw, shader) };
        unsafe { gl.compile_shader(raw) };

        log::debug!("\tCompiled shader {raw:?}");

        let compiled_ok = unsafe { gl.get_shader_compile_status(raw) };
        let msg = unsafe { gl.get_shader_info_log(raw) };
        if compiled_ok {
            if !msg.is_empty() {
                log::debug!("\tCompile message: {msg}");
            }
            Ok(raw)
        } else {
            log::error!("\tShader compilation failed: {msg}");
            unsafe { gl.delete_shader(raw) };
            Err(crate::PipelineError::Linkage(
                map_naga_stage(naga_stage),
                msg,
            ))
        }
    }

    fn create_shader(
        gl: &glow::Context,
        naga_stage: naga::ShaderStage,
        stage: &crate::ProgrammableStage<super::ShaderModule>,
        context: CompilationContext,
        program: glow::Program,
    ) -> Result<glow::Shader, crate::PipelineError> {
        use naga::back::glsl;
        let pipeline_options = glsl::PipelineOptions {
            shader_stage: naga_stage,
            entry_point: stage.entry_point.to_owned(),
            multiview: context
                .multiview_mask
                .map(|a| NonZeroU32::new(a.get().count_ones()).unwrap()),
        };

        let (module, info) = naga::back::pipeline_constants::process_overrides(
            &stage.module.source.module,
            &stage.module.source.info,
            Some((naga_stage, stage.entry_point)),
            stage.constants,
        )
        .map_err(|e| {
            let msg = format!("{e}");
            crate::PipelineError::PipelineConstants(map_naga_stage(naga_stage), msg)
        })?;

        let entry_point_index = module
            .entry_points
            .iter()
            .position(|ep| ep.name.as_str() == stage.entry_point)
            .ok_or(crate::PipelineError::EntryPoint(naga_stage))?;

        use naga::proc::BoundsCheckPolicy;
        // The image bounds checks require the TEXTURE_LEVELS feature available in GL core 4.3+.
        let version = gl.version();
        let image_check = if !version.is_embedded && (version.major, version.minor) >= (4, 3) {
            BoundsCheckPolicy::ReadZeroSkipWrite
        } else {
            BoundsCheckPolicy::Unchecked
        };

        // Other bounds check are either provided by glsl or not implemented yet.
        let policies = naga::proc::BoundsCheckPolicies {
            index: BoundsCheckPolicy::Unchecked,
            buffer: BoundsCheckPolicy::Unchecked,
            image_load: image_check,
            binding_array: BoundsCheckPolicy::Unchecked,
        };

        let mut output = String::new();
        let needs_temp_options = stage.zero_initialize_workgroup_memory
            != context.layout.naga_options.zero_initialize_workgroup_memory;
        let mut temp_options;
        let naga_options = if needs_temp_options {
            // We use a conditional here, as cloning the naga_options could be expensive
            // That is, we want to avoid doing that unless we cannot avoid it
            temp_options = context.layout.naga_options.clone();
            temp_options.zero_initialize_workgroup_memory = stage.zero_initialize_workgroup_memory;
            &temp_options
        } else {
            &context.layout.naga_options
        };
        let mut writer = glsl::Writer::new(
            &mut output,
            &module,
            &info,
            naga_options,
            &pipeline_options,
            policies,
        )
        .map_err(|e| {
            let msg = format!("{e}");
            crate::PipelineError::Linkage(map_naga_stage(naga_stage), msg)
        })?;

        let reflection_info = writer.write().map_err(|e| {
            let msg = format!("{e}");
            crate::PipelineError::Linkage(map_naga_stage(naga_stage), msg)
        })?;

        log::debug!("Naga generated shader:\n{output}");

        context.consume_reflection(
            gl,
            &module,
            info.get_entry_point(entry_point_index),
            reflection_info,
            naga_stage,
            program,
        );

        unsafe { Self::compile_shader(gl, &output, naga_stage, stage.module.label.as_deref()) }
    }

    unsafe fn create_pipeline<'a>(
        &self,
        gl: &glow::Context,
        shaders: ArrayVec<ShaderStage<'a>, { crate::MAX_CONCURRENT_SHADER_STAGES }>,
        layout: &super::PipelineLayout,
        #[cfg_attr(target_arch = "wasm32", allow(unused))] label: Option<&str>,
        multiview_mask: Option<NonZeroU32>,
    ) -> Result<Arc<super::PipelineInner>, crate::PipelineError> {
        let mut program_stages = ArrayVec::new();
        let group_to_binding_to_slot = layout
            .group_infos
            .iter()
            .map(|group| group.as_ref().map(|group| group.binding_to_slot.clone()))
            .collect::<Vec<_>>();
        for &(naga_stage, stage) in &shaders {
            program_stages.push(super::ProgramStage {
                naga_stage: naga_stage.to_owned(),
                shader_id: stage.module.id,
                entry_point: stage.entry_point.to_owned(),
                zero_initialize_workgroup_memory: stage.zero_initialize_workgroup_memory,
                constant_hash: Self::create_constant_hash(stage),
            });
        }
        let mut guard = self
            .shared
            .program_cache
            .try_lock()
            .expect("Couldn't acquire program_cache lock");
        // This guard ensures that we can't accidentally destroy a program whilst we're about to reuse it
        // The only place that destroys a pipeline is also locking on `program_cache`
        let program = guard
            .entry(super::ProgramCacheKey {
                stages: program_stages,
                group_to_binding_to_slot: group_to_binding_to_slot.into_boxed_slice(),
            })
            .or_insert_with(|| unsafe {
                Self::create_program(
                    gl,
                    shaders,
                    layout,
                    label,
                    multiview_mask,
                    self.shared.shading_language_version,
                    self.shared.private_caps,
                )
            })
            .to_owned()?;
        drop(guard);

        Ok(program)
    }

    fn create_constant_hash(stage: &crate::ProgrammableStage<super::ShaderModule>) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        for (key, value) in stage.constants.iter() {
            buf.extend_from_slice(key.as_bytes());
            buf.extend_from_slice(&value.to_ne_bytes());
        }

        buf
    }

    unsafe fn create_program<'a>(
        gl: &glow::Context,
        shaders: ArrayVec<ShaderStage<'a>, { crate::MAX_CONCURRENT_SHADER_STAGES }>,
        layout: &super::PipelineLayout,
        #[cfg_attr(target_arch = "wasm32", allow(unused))] label: Option<&str>,
        multiview_mask: Option<NonZeroU32>,
        glsl_version: naga::back::glsl::Version,
        private_caps: PrivateCapabilities,
    ) -> Result<Arc<super::PipelineInner>, crate::PipelineError> {
        let glsl_version = match glsl_version {
            naga::back::glsl::Version::Embedded { version, .. } => format!("{version} es"),
            naga::back::glsl::Version::Desktop(version) => format!("{version}"),
        };
        let program = unsafe { gl.create_program() }.unwrap();
        #[cfg(native)]
        if let Some(label) = label {
            if private_caps.contains(PrivateCapabilities::DEBUG_FNS) {
                let name = program.0.get();
                unsafe { gl.object_label(glow::PROGRAM, name, Some(label)) };
            }
        }

        let mut name_binding_map = NameBindingMap::default();
        let mut immediates_items = ArrayVec::<_, { crate::MAX_CONCURRENT_SHADER_STAGES }>::new();
        let mut sampler_map = [None; super::MAX_TEXTURE_SLOTS];
        let mut has_stages = wgt::ShaderStages::empty();
        let mut shaders_to_delete = ArrayVec::<_, { crate::MAX_CONCURRENT_SHADER_STAGES }>::new();
        let mut clip_distance_count = 0;

        for &(naga_stage, stage) in &shaders {
            has_stages |= map_naga_stage(naga_stage);
            let pc_item = {
                immediates_items.push(Vec::new());
                immediates_items.last_mut().unwrap()
            };
            let context = CompilationContext {
                layout,
                sampler_map: &mut sampler_map,
                name_binding_map: &mut name_binding_map,
                immediates_items: pc_item,
                multiview_mask,
                clip_distance_count: &mut clip_distance_count,
            };

            let shader = Self::create_shader(gl, naga_stage, stage, context, program)?;
            shaders_to_delete.push(shader);
        }

        // Create empty fragment shader if only vertex shader is present
        if has_stages == wgt::ShaderStages::VERTEX {
            let shader_src = format!("#version {glsl_version}\n void main(void) {{}}",);
            log::debug!("Only vertex shader is present. Creating an empty fragment shader",);
            let shader = unsafe {
                Self::compile_shader(
                    gl,
                    &shader_src,
                    naga::ShaderStage::Fragment,
                    Some("(wgpu internal) dummy fragment shader"),
                )
            }?;
            shaders_to_delete.push(shader);
        }

        for &shader in shaders_to_delete.iter() {
            unsafe { gl.attach_shader(program, shader) };
        }
        unsafe { gl.link_program(program) };

        for shader in shaders_to_delete {
            unsafe { gl.delete_shader(shader) };
        }

        log::debug!("\tLinked program {program:?}");

        let linked_ok = unsafe { gl.get_program_link_status(program) };
        let msg = unsafe { gl.get_program_info_log(program) };
        if !linked_ok {
            return Err(crate::PipelineError::Linkage(has_stages, msg));
        }
        if !msg.is_empty() {
            log::debug!("\tLink message: {msg}");
        }

        if !private_caps.contains(PrivateCapabilities::SHADER_BINDING_LAYOUT) {
            // This remapping is only needed if we aren't able to put the binding layout
            // in the shader. We can't remap storage buffers this way.
            unsafe { gl.use_program(Some(program)) };
            for (ref name, (register, slot)) in name_binding_map {
                log::trace!("Get binding {name:?} from program {program:?}");
                match register {
                    super::BindingRegister::UniformBuffers => {
                        let index = unsafe { gl.get_uniform_block_index(program, name) }.unwrap();
                        log::trace!("\tBinding slot {slot} to block index {index}");
                        unsafe { gl.uniform_block_binding(program, index, slot as _) };
                    }
                    super::BindingRegister::StorageBuffers => {
                        let index =
                            unsafe { gl.get_shader_storage_block_index(program, name) }.unwrap();
                        log::error!("Unable to re-map shader storage block {name} to {index}");
                        return Err(crate::DeviceError::Lost.into());
                    }
                    super::BindingRegister::Textures | super::BindingRegister::Images => {
                        let location = unsafe { gl.get_uniform_location(program, name) };
                        unsafe { gl.uniform_1_i32(location.as_ref(), slot as _) };
                    }
                }
            }
        }

        let mut uniforms = ArrayVec::new();

        for (stage_idx, stage_items) in immediates_items.into_iter().enumerate() {
            for item in stage_items {
                let naga_module = &shaders[stage_idx].1.module.source.module;
                let type_inner = &naga_module.types[item.ty].inner;

                let location = unsafe { gl.get_uniform_location(program, &item.access_path) };

                log::trace!(
                    "immediate data item: name={}, ty={:?}, offset={}, location={:?}",
                    item.access_path,
                    type_inner,
                    item.offset,
                    location,
                );

                if let Some(location) = location {
                    uniforms.push(super::ImmediateDesc {
                        location,
                        offset: item.offset,
                        size_bytes: type_inner.size(naga_module.to_ctx()),
                        ty: type_inner.clone(),
                    });
                }
            }
        }

        let first_instance_location = if has_stages.contains(wgt::ShaderStages::VERTEX) {
            // If this returns none (the uniform isn't active), that's fine, we just won't set it.
            unsafe { gl.get_uniform_location(program, naga::back::glsl::FIRST_INSTANCE_BINDING) }
        } else {
            None
        };

        Ok(Arc::new(super::PipelineInner {
            program,
            sampler_map,
            first_instance_location,
            immediates_descs: uniforms,
            clip_distance_count,
        }))
    }
}

impl crate::Device for super::Device {
    type A = super::Api;

    unsafe fn create_buffer(
        &self,
        desc: &crate::BufferDescriptor,
    ) -> Result<super::Buffer, crate::DeviceError> {
        let target = if desc.usage.contains(wgt::BufferUses::INDEX) {
            glow::ELEMENT_ARRAY_BUFFER
        } else {
            glow::ARRAY_BUFFER
        };

        let emulate_map = self
            .shared
            .workarounds
            .contains(super::Workarounds::EMULATE_BUFFER_MAP)
            || !self
                .shared
                .private_caps
                .contains(PrivateCapabilities::BUFFER_ALLOCATION);

        if emulate_map && desc.usage.intersects(wgt::BufferUses::MAP_WRITE) {
            return Ok(super::Buffer {
                raw: None,
                target,
                size: desc.size,
                map_flags: 0,
                data: Some(Arc::new(MaybeMutex::new(vec![0; desc.size as usize]))),
                offset_of_current_mapping: Arc::new(MaybeMutex::new(0)),
            });
        }

        let gl = &self.shared.context.lock();

        let target = if desc.usage.contains(wgt::BufferUses::INDEX) {
            glow::ELEMENT_ARRAY_BUFFER
        } else {
            glow::ARRAY_BUFFER
        };

        let is_host_visible = desc
            .usage
            .intersects(wgt::BufferUses::MAP_READ | wgt::BufferUses::MAP_WRITE);
        let is_coherent = desc
            .memory_flags
            .contains(crate::MemoryFlags::PREFER_COHERENT);

        let mut map_flags = 0;
        if desc.usage.contains(wgt::BufferUses::MAP_READ) {
            map_flags |= glow::MAP_READ_BIT;
        }
        if desc.usage.contains(wgt::BufferUses::MAP_WRITE) {
            map_flags |= glow::MAP_WRITE_BIT;
        }

        let raw = Some(unsafe { gl.create_buffer() }.map_err(|_| crate::DeviceError::OutOfMemory)?);
        unsafe { gl.bind_buffer(target, raw) };
        let raw_size = desc
            .size
            .try_into()
            .map_err(|_| crate::DeviceError::OutOfMemory)?;

        if self
            .shared
            .private_caps
            .contains(PrivateCapabilities::BUFFER_ALLOCATION)
        {
            if is_host_visible {
                map_flags |= glow::MAP_PERSISTENT_BIT;
                if is_coherent {
                    map_flags |= glow::MAP_COHERENT_BIT;
                }
            }
            // TODO: may also be required for other calls involving `buffer_sub_data_u8_slice` (e.g. copy buffer to buffer and clear buffer)
            if desc.usage.intersects(wgt::BufferUses::QUERY_RESOLVE) {
                map_flags |= glow::DYNAMIC_STORAGE_BIT;
            }
            unsafe { gl.buffer_storage(target, raw_size, None, map_flags) };
        } else {
            assert!(!is_coherent);
            let usage = if is_host_visible {
                if desc.usage.contains(wgt::BufferUses::MAP_READ) {
                    glow::STREAM_READ
                } else {
                    glow::DYNAMIC_DRAW
                }
            } else {
                // Even if the usage doesn't contain SRC_READ, we update it internally at least once
                // Some vendors take usage very literally and STATIC_DRAW will freeze us with an empty buffer
                // https://github.com/gfx-rs/wgpu/issues/3371
                glow::DYNAMIC_DRAW
            };
            unsafe { gl.buffer_data_size(target, raw_size, usage) };
        }

        unsafe { gl.bind_buffer(target, None) };

        if !is_coherent && desc.usage.contains(wgt::BufferUses::MAP_WRITE) {
            map_flags |= glow::MAP_FLUSH_EXPLICIT_BIT;
        }
        //TODO: do we need `glow::MAP_UNSYNCHRONIZED_BIT`?

        #[cfg(native)]
        if let Some(label) = desc.label {
            if self
                .shared
                .private_caps
                .contains(PrivateCapabilities::DEBUG_FNS)
            {
                let name = raw.map_or(0, |buf| buf.0.get());
                unsafe { gl.object_label(glow::BUFFER, name, Some(label)) };
            }
        }

        let data = if emulate_map && desc.usage.contains(wgt::BufferUses::MAP_READ) {
            Some(Arc::new(MaybeMutex::new(vec![0; desc.size as usize])))
        } else {
            None
        };

        self.counters.buffers.add(1);

        Ok(super::Buffer {
            raw,
            target,
            size: desc.size,
            map_flags,
            data,
            offset_of_current_mapping: Arc::new(MaybeMutex::new(0)),
        })
    }

    unsafe fn destroy_buffer(&self, buffer: super::Buffer) {
        if let Some(raw) = buffer.raw {
            let gl = &self.shared.context.lock();
            unsafe { gl.delete_buffer(raw) };
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
        let is_coherent = buffer.map_flags & glow::MAP_COHERENT_BIT != 0;
        let ptr = match buffer.raw {
            None => {
                let mut vec = lock(buffer.data.as_ref().unwrap());
                let slice = &mut vec.as_mut_slice()[range.start as usize..range.end as usize];
                slice.as_mut_ptr()
            }
            Some(raw) => {
                let gl = &self.shared.context.lock();
                unsafe { gl.bind_buffer(buffer.target, Some(raw)) };
                let ptr = if let Some(ref map_read_allocation) = buffer.data {
                    let mut guard = lock(map_read_allocation);
                    let slice = guard.as_mut_slice();
                    unsafe { self.shared.get_buffer_sub_data(gl, buffer.target, 0, slice) };
                    slice.as_mut_ptr()
                } else {
                    *lock(&buffer.offset_of_current_mapping) = range.start;
                    unsafe {
                        gl.map_buffer_range(
                            buffer.target,
                            range.start as i32,
                            (range.end - range.start) as i32,
                            buffer.map_flags,
                        )
                    }
                };
                unsafe { gl.bind_buffer(buffer.target, None) };
                ptr
            }
        };
        Ok(crate::BufferMapping {
            ptr: ptr::NonNull::new(ptr).ok_or(crate::DeviceError::Lost)?,
            is_coherent,
        })
    }
    unsafe fn unmap_buffer(&self, buffer: &super::Buffer) {
        if let Some(raw) = buffer.raw {
            if buffer.data.is_none() {
                let gl = &self.shared.context.lock();
                unsafe { gl.bind_buffer(buffer.target, Some(raw)) };
                unsafe { gl.unmap_buffer(buffer.target) };
                unsafe { gl.bind_buffer(buffer.target, None) };
                *lock(&buffer.offset_of_current_mapping) = 0;
            }
        }
    }
    unsafe fn flush_mapped_ranges<I>(&self, buffer: &super::Buffer, ranges: I)
    where
        I: Iterator<Item = crate::MemoryRange>,
    {
        if let Some(raw) = buffer.raw {
            if buffer.data.is_none() {
                let gl = &self.shared.context.lock();
                unsafe { gl.bind_buffer(buffer.target, Some(raw)) };
                for range in ranges {
                    let offset_of_current_mapping = *lock(&buffer.offset_of_current_mapping);
                    unsafe {
                        gl.flush_mapped_buffer_range(
                            buffer.target,
                            (range.start - offset_of_current_mapping) as i32,
                            (range.end - range.start) as i32,
                        )
                    };
                }
            }
        }
    }
    unsafe fn invalidate_mapped_ranges<I>(&self, _buffer: &super::Buffer, _ranges: I) {
        //TODO: do we need to do anything?
    }

    unsafe fn create_texture(
        &self,
        desc: &crate::TextureDescriptor,
    ) -> Result<super::Texture, crate::DeviceError> {
        let gl = &self.shared.context.lock();

        let render_usage = wgt::TextureUses::COLOR_TARGET
            | wgt::TextureUses::DEPTH_STENCIL_WRITE
            | wgt::TextureUses::DEPTH_STENCIL_READ
            | wgt::TextureUses::TRANSIENT;
        let format_desc = self.shared.describe_texture_format(desc.format);

        let inner = if render_usage.contains(desc.usage)
            && desc.dimension == wgt::TextureDimension::D2
            && desc.size.depth_or_array_layers == 1
        {
            let raw = unsafe { gl.create_renderbuffer().unwrap() };
            unsafe { gl.bind_renderbuffer(glow::RENDERBUFFER, Some(raw)) };
            if desc.sample_count > 1 {
                unsafe {
                    gl.renderbuffer_storage_multisample(
                        glow::RENDERBUFFER,
                        desc.sample_count as i32,
                        format_desc.internal,
                        desc.size.width as i32,
                        desc.size.height as i32,
                    )
                };
            } else {
                unsafe {
                    gl.renderbuffer_storage(
                        glow::RENDERBUFFER,
                        format_desc.internal,
                        desc.size.width as i32,
                        desc.size.height as i32,
                    )
                };
            }

            #[cfg(native)]
            if let Some(label) = desc.label {
                if self
                    .shared
                    .private_caps
                    .contains(PrivateCapabilities::DEBUG_FNS)
                {
                    let name = raw.0.get();
                    unsafe { gl.object_label(glow::RENDERBUFFER, name, Some(label)) };
                }
            }

            unsafe { gl.bind_renderbuffer(glow::RENDERBUFFER, None) };
            super::TextureInner::Renderbuffer { raw }
        } else {
            let raw = unsafe { gl.create_texture().unwrap() };
            let target = super::Texture::get_info_from_desc(desc);

            unsafe { gl.bind_texture(target, Some(raw)) };
            //Note: this has to be done before defining the storage!
            match desc.format.sample_type(None, Some(self.shared.features)) {
                Some(
                    wgt::TextureSampleType::Float { filterable: false }
                    | wgt::TextureSampleType::Uint
                    | wgt::TextureSampleType::Sint,
                ) => {
                    // reset default filtering mode
                    unsafe {
                        gl.tex_parameter_i32(target, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32)
                    };
                    unsafe {
                        gl.tex_parameter_i32(target, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32)
                    };
                }
                _ => {}
            }

            if conv::is_layered_target(target) {
                unsafe {
                    if self
                        .shared
                        .private_caps
                        .contains(PrivateCapabilities::TEXTURE_STORAGE)
                    {
                        gl.tex_storage_3d(
                            target,
                            desc.mip_level_count as i32,
                            format_desc.internal,
                            desc.size.width as i32,
                            desc.size.height as i32,
                            desc.size.depth_or_array_layers as i32,
                        )
                    } else if target == glow::TEXTURE_3D {
                        let mut width = desc.size.width;
                        let mut height = desc.size.height;
                        let mut depth = desc.size.depth_or_array_layers;
                        for i in 0..desc.mip_level_count {
                            gl.tex_image_3d(
                                target,
                                i as i32,
                                format_desc.internal as i32,
                                width as i32,
                                height as i32,
                                depth as i32,
                                0,
                                format_desc.external,
                                format_desc.data_type,
                                glow::PixelUnpackData::Slice(None),
                            );
                            width = max(1, width / 2);
                            height = max(1, height / 2);
                            depth = max(1, depth / 2);
                        }
                    } else {
                        let mut width = desc.size.width;
                        let mut height = desc.size.height;
                        for i in 0..desc.mip_level_count {
                            gl.tex_image_3d(
                                target,
                                i as i32,
                                format_desc.internal as i32,
                                width as i32,
                                height as i32,
                                desc.size.depth_or_array_layers as i32,
                                0,
                                format_desc.external,
                                format_desc.data_type,
                                glow::PixelUnpackData::Slice(None),
                            );
                            width = max(1, width / 2);
                            height = max(1, height / 2);
                        }
                    }
                };
            } else if desc.sample_count > 1 {
                unsafe {
                    gl.tex_storage_2d_multisample(
                        target,
                        desc.sample_count as i32,
                        format_desc.internal,
                        desc.size.width as i32,
                        desc.size.height as i32,
                        true,
                    )
                };
            } else {
                unsafe {
                    if self
                        .shared
                        .private_caps
                        .contains(PrivateCapabilities::TEXTURE_STORAGE)
                    {
                        gl.tex_storage_2d(
                            target,
                            desc.mip_level_count as i32,
                            format_desc.internal,
                            desc.size.width as i32,
                            desc.size.height as i32,
                        )
                    } else if target == glow::TEXTURE_CUBE_MAP {
                        let mut width = desc.size.width;
                        let mut height = desc.size.height;
                        for i in 0..desc.mip_level_count {
                            for face in [
                                glow::TEXTURE_CUBE_MAP_POSITIVE_X,
                                glow::TEXTURE_CUBE_MAP_NEGATIVE_X,
                                glow::TEXTURE_CUBE_MAP_POSITIVE_Y,
                                glow::TEXTURE_CUBE_MAP_NEGATIVE_Y,
                                glow::TEXTURE_CUBE_MAP_POSITIVE_Z,
                                glow::TEXTURE_CUBE_MAP_NEGATIVE_Z,
                            ] {
                                gl.tex_image_2d(
                                    face,
                                    i as i32,
                                    format_desc.internal as i32,
                                    width as i32,
                                    height as i32,
                                    0,
                                    format_desc.external,
                                    format_desc.data_type,
                                    glow::PixelUnpackData::Slice(None),
                                );
                            }
                            width = max(1, width / 2);
                            height = max(1, height / 2);
                        }
                    } else {
                        let mut width = desc.size.width;
                        let mut height = desc.size.height;
                        for i in 0..desc.mip_level_count {
                            gl.tex_image_2d(
                                target,
                                i as i32,
                                format_desc.internal as i32,
                                width as i32,
                                height as i32,
                                0,
                                format_desc.external,
                                format_desc.data_type,
                                glow::PixelUnpackData::Slice(None),
                            );
                            width = max(1, width / 2);
                            height = max(1, height / 2);
                        }
                    }
                };
            }

            #[cfg(native)]
            if let Some(label) = desc.label {
                if self
                    .shared
                    .private_caps
                    .contains(PrivateCapabilities::DEBUG_FNS)
                {
                    let name = raw.0.get();
                    unsafe { gl.object_label(glow::TEXTURE, name, Some(label)) };
                }
            }

            unsafe { gl.bind_texture(target, None) };
            super::TextureInner::Texture { raw, target }
        };

        self.counters.textures.add(1);

        Ok(super::Texture {
            inner,
            drop_guard: None,
            mip_level_count: desc.mip_level_count,
            array_layer_count: desc.array_layer_count(),
            format: desc.format,
            format_desc,
            copy_size: desc.copy_extent(),
        })
    }

    unsafe fn destroy_texture(&self, texture: super::Texture) {
        if texture.drop_guard.is_none() {
            let gl = &self.shared.context.lock();
            match texture.inner {
                super::TextureInner::Renderbuffer { raw, .. } => {
                    unsafe { gl.delete_renderbuffer(raw) };
                }
                super::TextureInner::DefaultRenderbuffer => {}
                super::TextureInner::Texture { raw, .. } => {
                    unsafe { gl.delete_texture(raw) };
                }
                #[cfg(webgl)]
                super::TextureInner::ExternalFramebuffer { .. } => {}
                #[cfg(native)]
                super::TextureInner::ExternalNativeFramebuffer { .. } => {}
            }
        }

        // For clarity, we explicitly drop the drop guard. Although this has no real semantic effect as the
        // end of the scope will drop the drop guard since this function takes ownership of the texture.
        drop(texture.drop_guard);

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
        self.counters.texture_views.add(1);
        Ok(super::TextureView {
            //TODO: use `conv::map_view_dimension(desc.dimension)`?
            inner: texture.inner.clone(),
            aspects: crate::FormatAspects::new(texture.format, desc.range.aspect),
            mip_levels: desc.range.mip_range(texture.mip_level_count),
            array_layers: desc.range.layer_range(texture.array_layer_count),
            format: texture.format,
        })
    }

    unsafe fn destroy_texture_view(&self, _view: super::TextureView) {
        self.counters.texture_views.sub(1);
    }

    unsafe fn create_sampler(
        &self,
        desc: &crate::SamplerDescriptor,
    ) -> Result<super::Sampler, crate::DeviceError> {
        let gl = &self.shared.context.lock();

        let raw = unsafe { gl.create_sampler().unwrap() };

        let (min, mag) =
            conv::map_filter_modes(desc.min_filter, desc.mag_filter, desc.mipmap_filter);

        unsafe { gl.sampler_parameter_i32(raw, glow::TEXTURE_MIN_FILTER, min as i32) };
        unsafe { gl.sampler_parameter_i32(raw, glow::TEXTURE_MAG_FILTER, mag as i32) };

        unsafe {
            gl.sampler_parameter_i32(
                raw,
                glow::TEXTURE_WRAP_S,
                conv::map_address_mode(desc.address_modes[0]) as i32,
            )
        };
        unsafe {
            gl.sampler_parameter_i32(
                raw,
                glow::TEXTURE_WRAP_T,
                conv::map_address_mode(desc.address_modes[1]) as i32,
            )
        };
        unsafe {
            gl.sampler_parameter_i32(
                raw,
                glow::TEXTURE_WRAP_R,
                conv::map_address_mode(desc.address_modes[2]) as i32,
            )
        };

        if let Some(border_color) = desc.border_color {
            let border = match border_color {
                wgt::SamplerBorderColor::TransparentBlack | wgt::SamplerBorderColor::Zero => {
                    [0.0; 4]
                }
                wgt::SamplerBorderColor::OpaqueBlack => [0.0, 0.0, 0.0, 1.0],
                wgt::SamplerBorderColor::OpaqueWhite => [1.0; 4],
            };
            unsafe { gl.sampler_parameter_f32_slice(raw, glow::TEXTURE_BORDER_COLOR, &border) };
        }

        unsafe { gl.sampler_parameter_f32(raw, glow::TEXTURE_MIN_LOD, desc.lod_clamp.start) };
        unsafe { gl.sampler_parameter_f32(raw, glow::TEXTURE_MAX_LOD, desc.lod_clamp.end) };

        // If clamp is not 1, we know anisotropy is supported up to 16x
        if desc.anisotropy_clamp != 1 {
            unsafe {
                gl.sampler_parameter_i32(
                    raw,
                    glow::TEXTURE_MAX_ANISOTROPY,
                    desc.anisotropy_clamp as i32,
                )
            };
        }

        //set_param_float(glow::TEXTURE_LOD_BIAS, info.lod_bias.0);

        if let Some(compare) = desc.compare {
            unsafe {
                gl.sampler_parameter_i32(
                    raw,
                    glow::TEXTURE_COMPARE_MODE,
                    glow::COMPARE_REF_TO_TEXTURE as i32,
                )
            };
            unsafe {
                gl.sampler_parameter_i32(
                    raw,
                    glow::TEXTURE_COMPARE_FUNC,
                    conv::map_compare_func(compare) as i32,
                )
            };
        }

        #[cfg(native)]
        if let Some(label) = desc.label {
            if self
                .shared
                .private_caps
                .contains(PrivateCapabilities::DEBUG_FNS)
            {
                let name = raw.0.get();
                unsafe { gl.object_label(glow::SAMPLER, name, Some(label)) };
            }
        }

        self.counters.samplers.add(1);

        Ok(super::Sampler { raw })
    }

    unsafe fn destroy_sampler(&self, sampler: super::Sampler) {
        let gl = &self.shared.context.lock();
        unsafe { gl.delete_sampler(sampler.raw) };
        self.counters.samplers.sub(1);
    }

    unsafe fn create_command_encoder(
        &self,
        _desc: &crate::CommandEncoderDescriptor<super::Queue>,
    ) -> Result<super::CommandEncoder, crate::DeviceError> {
        self.counters.command_encoders.add(1);

        Ok(super::CommandEncoder {
            cmd_buffer: super::CommandBuffer::default(),
            state: Default::default(),
            private_caps: self.shared.private_caps,
            counters: Arc::clone(&self.counters),
        })
    }

    unsafe fn create_bind_group_layout(
        &self,
        desc: &crate::BindGroupLayoutDescriptor,
    ) -> Result<super::BindGroupLayout, crate::DeviceError> {
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
    ) -> Result<super::PipelineLayout, crate::DeviceError> {
        use naga::back::glsl;

        let mut group_infos = Vec::with_capacity(desc.bind_group_layouts.len());
        let mut num_samplers = 0u8;
        let mut num_textures = 0u8;
        let mut num_images = 0u8;
        let mut num_uniform_buffers = 0u8;
        let mut num_storage_buffers = 0u8;

        let mut writer_flags = glsl::WriterFlags::ADJUST_COORDINATE_SPACE;
        writer_flags.set(
            glsl::WriterFlags::TEXTURE_SHADOW_LOD,
            self.shared
                .private_caps
                .contains(PrivateCapabilities::SHADER_TEXTURE_SHADOW_LOD),
        );
        writer_flags.set(
            glsl::WriterFlags::DRAW_PARAMETERS,
            self.shared
                .private_caps
                .contains(PrivateCapabilities::FULLY_FEATURED_INSTANCING),
        );
        // We always force point size to be written and it will be ignored by the driver if it's not a point list primitive.
        // https://github.com/gfx-rs/wgpu/pull/3440/files#r1095726950
        writer_flags.set(glsl::WriterFlags::FORCE_POINT_SIZE, true);
        let mut binding_map = glsl::BindingMap::default();

        for (group_index, bg_layout) in desc.bind_group_layouts.iter().enumerate() {
            let Some(bg_layout) = bg_layout else {
                group_infos.push(None);
                continue;
            };

            // create a vector with the size enough to hold all the bindings, filled with `!0`
            let mut binding_to_slot = vec![
                !0;
                bg_layout
                    .entries
                    .iter()
                    .map(|b| b.binding)
                    .max()
                    .map_or(0, |idx| idx as usize + 1)
            ]
            .into_boxed_slice();

            for entry in bg_layout.entries.iter() {
                let counter = match entry.ty {
                    wgt::BindingType::Sampler { .. } => &mut num_samplers,
                    wgt::BindingType::Texture { .. } => &mut num_textures,
                    wgt::BindingType::StorageTexture { .. } => &mut num_images,
                    wgt::BindingType::Buffer {
                        ty: wgt::BufferBindingType::Uniform,
                        ..
                    } => &mut num_uniform_buffers,
                    wgt::BindingType::Buffer {
                        ty: wgt::BufferBindingType::Storage { .. },
                        ..
                    } => &mut num_storage_buffers,
                    wgt::BindingType::AccelerationStructure { .. } => unimplemented!(),
                    wgt::BindingType::ExternalTexture => unimplemented!(),
                };

                binding_to_slot[entry.binding as usize] = *counter;
                let br = naga::ResourceBinding {
                    group: group_index as u32,
                    binding: entry.binding,
                };
                binding_map.insert(br, *counter);
                *counter += entry.count.map_or(1, |c| c.get() as u8);
            }

            group_infos.push(Some(super::BindGroupLayoutInfo {
                entries: Arc::clone(&bg_layout.entries),
                binding_to_slot,
            }));
        }

        self.counters.pipeline_layouts.add(1);

        Ok(super::PipelineLayout {
            group_infos: group_infos.into_boxed_slice(),
            naga_options: glsl::Options {
                version: self.shared.shading_language_version,
                writer_flags,
                binding_map,
                zero_initialize_workgroup_memory: true,
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
        let mut contents = Vec::new();

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
            let binding = match layout.ty {
                wgt::BindingType::Buffer { .. } => {
                    let bb = &desc.buffers[entry.resource_index as usize];
                    super::RawBinding::Buffer {
                        raw: bb.buffer.raw.unwrap(),
                        offset: bb.offset as i32,
                        size: match bb.size {
                            Some(s) => s.get() as i32,
                            None => (bb.buffer.size - bb.offset) as i32,
                        },
                    }
                }
                wgt::BindingType::Sampler { .. } => {
                    let sampler = desc.samplers[entry.resource_index as usize];
                    super::RawBinding::Sampler(sampler.raw)
                }
                wgt::BindingType::Texture { view_dimension, .. } => {
                    let view = desc.textures[entry.resource_index as usize].view;
                    if view.array_layers.start != 0 {
                        log::error!("Unable to create a sampled texture binding for non-zero array layer.\n{}",
                            "This is an implementation problem of wgpu-hal/gles backend.")
                    }
                    let (raw, target) = view.inner.as_native();

                    super::Texture::log_failing_target_heuristics(view_dimension, target);

                    super::RawBinding::Texture {
                        raw,
                        target,
                        aspects: view.aspects,
                        mip_levels: view.mip_levels.clone(),
                    }
                }
                wgt::BindingType::StorageTexture {
                    access,
                    format,
                    view_dimension,
                } => {
                    let view = desc.textures[entry.resource_index as usize].view;
                    let format_desc = self.shared.describe_texture_format(format);
                    let (raw, _target) = view.inner.as_native();
                    super::RawBinding::Image(super::ImageBinding {
                        raw,
                        mip_level: view.mip_levels.start,
                        array_layer: match view_dimension {
                            wgt::TextureViewDimension::D2Array
                            | wgt::TextureViewDimension::CubeArray => None,
                            _ => Some(view.array_layers.start),
                        },
                        access: conv::map_storage_access(access),
                        format: format_desc.internal,
                    })
                }
                wgt::BindingType::AccelerationStructure { .. } => unimplemented!(),
                wgt::BindingType::ExternalTexture => unimplemented!(),
            };
            contents.push(binding);
        }

        self.counters.bind_groups.add(1);

        Ok(super::BindGroup {
            contents: contents.into_boxed_slice(),
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

        Ok(super::ShaderModule {
            source: match shader {
                crate::ShaderInput::Naga(naga) => naga,
                // The backend doesn't yet expose this feature so it should be fine
                crate::ShaderInput::Glsl { .. } => unimplemented!(),
                crate::ShaderInput::SpirV(_)
                | crate::ShaderInput::MetalLib { .. }
                | crate::ShaderInput::Msl { .. }
                | crate::ShaderInput::Dxil { .. }
                | crate::ShaderInput::Hlsl { .. } => {
                    unreachable!()
                }
            },
            label: desc.label.map(|str| str.to_string()),
            id: self.shared.next_shader_id.fetch_add(1, Ordering::Relaxed),
        })
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
        let (vertex_stage, vertex_buffers) = match &desc.vertex_processor {
            crate::VertexProcessor::Standard {
                vertex_buffers,
                ref vertex_stage,
            } => (vertex_stage, vertex_buffers),
            crate::VertexProcessor::Mesh { .. } => unreachable!(),
        };
        let gl = &self.shared.context.lock();
        let mut shaders = ArrayVec::new();
        shaders.push((naga::ShaderStage::Vertex, vertex_stage));
        if let Some(ref fs) = desc.fragment_stage {
            shaders.push((naga::ShaderStage::Fragment, fs));
        }
        let inner = unsafe {
            self.create_pipeline(gl, shaders, desc.layout, desc.label, desc.multiview_mask)
        }?;

        let (vertex_buffers, vertex_attributes) = {
            let mut buffers = Vec::new();
            let mut attributes = Vec::new();
            for (index, vb_layout) in vertex_buffers.iter().enumerate() {
                buffers.push(super::VertexBufferDesc {
                    step: vb_layout.step_mode,
                    stride: vb_layout.array_stride as u32,
                });
                for vat in vb_layout.attributes.iter() {
                    let format_desc = conv::describe_vertex_format(vat.format);
                    attributes.push(super::AttributeDesc {
                        location: vat.shader_location,
                        offset: vat.offset as u32,
                        buffer_index: index as u32,
                        format_desc,
                    });
                }
            }
            (buffers.into_boxed_slice(), attributes.into_boxed_slice())
        };

        let color_targets = {
            let mut targets = Vec::new();
            for ct in desc.color_targets.iter().filter_map(|at| at.as_ref()) {
                targets.push(super::ColorTargetDesc {
                    mask: ct.write_mask,
                    blend: ct.blend.as_ref().map(conv::map_blend),
                });
            }
            //Note: if any of the states are different, and `INDEPENDENT_BLEND` flag
            // is not exposed, then this pipeline will not bind correctly.
            targets.into_boxed_slice()
        };

        self.counters.render_pipelines.add(1);

        Ok(super::RenderPipeline {
            inner,
            primitive: desc.primitive,
            vertex_buffers,
            vertex_attributes,
            color_targets,
            depth: desc.depth_stencil.as_ref().map(|ds| super::DepthState {
                function: conv::map_compare_func(ds.depth_compare.unwrap_or_default()),
                mask: ds.depth_write_enabled.unwrap_or_default(),
            }),
            depth_bias: desc
                .depth_stencil
                .as_ref()
                .map(|ds| ds.bias)
                .unwrap_or_default(),
            stencil: desc
                .depth_stencil
                .as_ref()
                .map(|ds| conv::map_stencil(&ds.stencil)),
            alpha_to_coverage_enabled: desc.multisample.alpha_to_coverage_enabled,
        })
    }

    unsafe fn destroy_render_pipeline(&self, pipeline: super::RenderPipeline) {
        // If the pipeline only has 2 strong references remaining, they're `pipeline` and `program_cache`
        // This is safe to assume as long as:
        // - `RenderPipeline` can't be cloned
        // - The only place that we can get a new reference is during `program_cache.lock()`
        if Arc::strong_count(&pipeline.inner) == 2 {
            let gl = &self.shared.context.lock();
            let mut program_cache = self.shared.program_cache.lock();
            program_cache.retain(|_, v| match *v {
                Ok(ref p) => p.program != pipeline.inner.program,
                Err(_) => false,
            });
            unsafe { gl.delete_program(pipeline.inner.program) };
        }

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
        let gl = &self.shared.context.lock();
        let mut shaders = ArrayVec::new();
        shaders.push((naga::ShaderStage::Compute, &desc.stage));
        let inner = unsafe { self.create_pipeline(gl, shaders, desc.layout, desc.label, None) }?;

        self.counters.compute_pipelines.add(1);

        Ok(super::ComputePipeline { inner })
    }

    unsafe fn destroy_compute_pipeline(&self, pipeline: super::ComputePipeline) {
        // If the pipeline only has 2 strong references remaining, they're `pipeline` and `program_cache``
        // This is safe to assume as long as:
        // - `ComputePipeline` can't be cloned
        // - The only place that we can get a new reference is during `program_cache.lock()`
        if Arc::strong_count(&pipeline.inner) == 2 {
            let gl = &self.shared.context.lock();
            let mut program_cache = self.shared.program_cache.lock();
            program_cache.retain(|_, v| match *v {
                Ok(ref p) => p.program != pipeline.inner.program,
                Err(_) => false,
            });
            unsafe { gl.delete_program(pipeline.inner.program) };
        }

        self.counters.compute_pipelines.sub(1);
    }

    unsafe fn create_pipeline_cache(
        &self,
        _: &crate::PipelineCacheDescriptor<'_>,
    ) -> Result<super::PipelineCache, crate::PipelineCacheError> {
        // Even though the cache doesn't do anything, we still return something here
        // as the least bad option
        Ok(super::PipelineCache)
    }
    unsafe fn destroy_pipeline_cache(&self, _: super::PipelineCache) {}

    #[cfg_attr(target_arch = "wasm32", allow(unused))]
    unsafe fn create_query_set(
        &self,
        desc: &wgt::QuerySetDescriptor<crate::Label>,
    ) -> Result<super::QuerySet, crate::DeviceError> {
        let gl = &self.shared.context.lock();

        let mut queries = Vec::with_capacity(desc.count as usize);
        for _ in 0..desc.count {
            let query =
                unsafe { gl.create_query() }.map_err(|_| crate::DeviceError::OutOfMemory)?;

            // We aren't really able to, in general, label queries.
            //
            // We could take a timestamp here to "initialize" the query,
            // but that's a bit of a hack, and we don't want to insert
            // random timestamps into the command stream of we don't have to.

            queries.push(query);
        }

        self.counters.query_sets.add(1);

        Ok(super::QuerySet {
            queries: queries.into_boxed_slice(),
            target: match desc.ty {
                wgt::QueryType::Occlusion => glow::ANY_SAMPLES_PASSED_CONSERVATIVE,
                wgt::QueryType::Timestamp => glow::TIMESTAMP,
                _ => unimplemented!(),
            },
        })
    }

    unsafe fn destroy_query_set(&self, set: super::QuerySet) {
        let gl = &self.shared.context.lock();
        for &query in set.queries.iter() {
            unsafe { gl.delete_query(query) };
        }
        self.counters.query_sets.sub(1);
    }

    unsafe fn create_fence(&self) -> Result<super::Fence, crate::DeviceError> {
        self.counters.fences.add(1);
        Ok(super::Fence::new(&self.shared.options))
    }

    unsafe fn destroy_fence(&self, fence: super::Fence) {
        let gl = &self.shared.context.lock();
        fence.destroy(gl);
        self.counters.fences.sub(1);
    }

    unsafe fn get_fence_value(
        &self,
        fence: &super::Fence,
    ) -> Result<crate::FenceValue, crate::DeviceError> {
        #[cfg_attr(target_arch = "wasm32", allow(clippy::needless_borrow))]
        Ok(fence.get_latest(&self.shared.context.lock()))
    }
    unsafe fn wait(
        &self,
        fence: &super::Fence,
        wait_value: crate::FenceValue,
        timeout: Option<core::time::Duration>,
    ) -> Result<bool, crate::DeviceError> {
        if fence.satisfied(wait_value) {
            return Ok(true);
        }

        let gl = &self.shared.context.lock();
        // MAX_CLIENT_WAIT_TIMEOUT_WEBGL is:
        // - 1s in Gecko https://searchfox.org/mozilla-central/rev/754074e05178e017ef6c3d8e30428ffa8f1b794d/dom/canvas/WebGLTypes.h#1386
        // - 0 in WebKit https://github.com/WebKit/WebKit/blob/4ef90d4672ca50267c0971b85db403d9684508ea/Source/WebCore/html/canvas/WebGL2RenderingContext.cpp#L110
        // - 0 in Chromium https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/modules/webgl/webgl2_rendering_context_base.cc;l=112;drc=a3cb0ac4c71ec04abfeaed199e5d63230eca2551
        let timeout_ns = if cfg!(any(webgl, Emscripten)) {
            0
        } else {
            timeout
                .map(|t| t.as_nanos().min(u32::MAX as u128) as u32)
                .unwrap_or(u32::MAX)
        };
        fence.wait(gl, wait_value, timeout_ns)
    }

    unsafe fn start_graphics_debugger_capture(&self) -> bool {
        #[cfg(all(native, feature = "renderdoc"))]
        return unsafe {
            self.render_doc
                .start_frame_capture(self.shared.context.raw_context(), ptr::null_mut())
        };
        #[allow(unreachable_code)]
        false
    }
    unsafe fn stop_graphics_debugger_capture(&self) {
        #[cfg(all(native, feature = "renderdoc"))]
        unsafe {
            self.render_doc
                .end_frame_capture(ptr::null_mut(), ptr::null_mut())
        }
    }
    unsafe fn create_acceleration_structure(
        &self,
        _desc: &crate::AccelerationStructureDescriptor,
    ) -> Result<super::AccelerationStructure, crate::DeviceError> {
        unimplemented!()
    }
    unsafe fn get_acceleration_structure_build_sizes<'a>(
        &self,
        _desc: &crate::GetAccelerationStructureBuildSizesDescriptor<'a, super::Buffer>,
    ) -> crate::AccelerationStructureBuildSizes {
        unimplemented!()
    }
    unsafe fn get_acceleration_structure_device_address(
        &self,
        _acceleration_structure: &super::AccelerationStructure,
    ) -> wgt::BufferAddress {
        unimplemented!()
    }
    unsafe fn destroy_acceleration_structure(
        &self,
        _acceleration_structure: super::AccelerationStructure,
    ) {
    }

    fn tlas_instance_to_bytes(&self, _instance: TlasInstance) -> Vec<u8> {
        unimplemented!()
    }

    fn get_internal_counters(&self) -> wgt::HalCounters {
        self.counters.as_ref().clone()
    }

    fn check_if_oom(&self) -> Result<(), crate::DeviceError> {
        Ok(())
    }
}

#[cfg(send_sync)]
unsafe impl Sync for super::Device {}
#[cfg(send_sync)]
unsafe impl Send for super::Device {}
