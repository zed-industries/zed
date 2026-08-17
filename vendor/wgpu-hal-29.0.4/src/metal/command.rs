use objc2::{
    rc::{autoreleasepool, Retained},
    runtime::ProtocolObject,
};
use objc2_foundation::{NSRange, NSString, NSUInteger};
use objc2_metal::{
    MTLAccelerationStructure, MTLAccelerationStructureCommandEncoder, MTLBlitCommandEncoder,
    MTLBlitPassDescriptor, MTLBuffer, MTLCommandBuffer, MTLCommandBufferStatus, MTLCommandEncoder,
    MTLCommandQueue, MTLComputeCommandEncoder, MTLComputePassDescriptor, MTLCounterDontSample,
    MTLDevice, MTLLoadAction, MTLPrimitiveType, MTLRenderCommandEncoder, MTLRenderPassDescriptor,
    MTLResidencySet, MTLResidencySetDescriptor, MTLSamplerState, MTLScissorRect, MTLSize,
    MTLStoreAction, MTLTexture, MTLVertexAmplificationViewMapping, MTLViewport,
    MTLVisibilityResultMode,
};

use super::{adapter, conv, TimestampQuerySupport};
use crate::CommandEncoder as _;
use alloc::{
    borrow::{Cow, ToOwned as _},
    sync::Arc,
    vec::Vec,
};
use core::{ops::Range, ptr::NonNull, sync::atomic};
use smallvec::SmallVec;

// has to match `Temp::binding_sizes`
const WORD_SIZE: usize = 4;

impl Default for super::CommandState {
    fn default() -> Self {
        Self {
            blit: None,
            acceleration_structure_builder: None,
            render: None,
            compute: None,
            raw_primitive_type: MTLPrimitiveType::Point,
            index: None,
            stage_infos: Default::default(),
            storage_buffer_length_map: Default::default(),
            vertex_buffer_size_map: Default::default(),
            immediates: Vec::new(),
            pending_timer_queries: Vec::new(),
        }
    }
}

/// Helper for passing encoders to `update_bind_group_state`.
///
/// Combines [`naga::ShaderStage`] and an encoder of the appropriate type for
/// that stage.
enum Encoder<'e> {
    Vertex(&'e ProtocolObject<dyn MTLRenderCommandEncoder>),
    Fragment(&'e ProtocolObject<dyn MTLRenderCommandEncoder>),
    Task(&'e ProtocolObject<dyn MTLRenderCommandEncoder>),
    Mesh(&'e ProtocolObject<dyn MTLRenderCommandEncoder>),
    Compute(&'e ProtocolObject<dyn MTLComputeCommandEncoder>),
}

impl Encoder<'_> {
    fn stage(&self) -> naga::ShaderStage {
        match self {
            Self::Vertex(_) => naga::ShaderStage::Vertex,
            Self::Fragment(_) => naga::ShaderStage::Fragment,
            Self::Task(_) => naga::ShaderStage::Task,
            Self::Mesh(_) => naga::ShaderStage::Mesh,
            Self::Compute(_) => naga::ShaderStage::Compute,
        }
    }

    fn set_buffer(
        &self,
        buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
        offset: NSUInteger,
        index: NSUInteger,
    ) {
        unsafe {
            match *self {
                Self::Vertex(enc) => enc.setVertexBuffer_offset_atIndex(buffer, offset, index),
                Self::Fragment(enc) => enc.setFragmentBuffer_offset_atIndex(buffer, offset, index),
                Self::Task(enc) => enc.setObjectBuffer_offset_atIndex(buffer, offset, index),
                Self::Mesh(enc) => enc.setMeshBuffer_offset_atIndex(buffer, offset, index),
                Self::Compute(enc) => enc.setBuffer_offset_atIndex(buffer, offset, index),
            }
        }
    }

    fn set_acceleration_structure(
        &self,
        buffer: Option<&ProtocolObject<dyn MTLAccelerationStructure>>,
        index: NSUInteger,
    ) {
        unsafe {
            match *self {
                Self::Vertex(enc) => {
                    enc.setVertexAccelerationStructure_atBufferIndex(buffer, index)
                }
                Self::Fragment(enc) => {
                    enc.setFragmentAccelerationStructure_atBufferIndex(buffer, index)
                }
                Self::Task(_) => {
                    unreachable!("Acceleration structures are not allowed in task shaders")
                }
                Self::Mesh(_) => {
                    unreachable!("Acceleration structures are not allowed in mesh shaders")
                }
                Self::Compute(enc) => enc.setAccelerationStructure_atBufferIndex(buffer, index),
            }
        }
    }

    fn set_bytes(&self, bytes: NonNull<core::ffi::c_void>, length: NSUInteger, index: NSUInteger) {
        unsafe {
            match *self {
                Self::Vertex(enc) => enc.setVertexBytes_length_atIndex(bytes, length, index),
                Self::Fragment(enc) => enc.setFragmentBytes_length_atIndex(bytes, length, index),
                Self::Task(enc) => enc.setObjectBytes_length_atIndex(bytes, length, index),
                Self::Mesh(enc) => enc.setMeshBytes_length_atIndex(bytes, length, index),
                Self::Compute(enc) => enc.setBytes_length_atIndex(bytes, length, index),
            }
        }
    }

    fn set_sampler_state(
        &self,
        state: Option<&ProtocolObject<dyn MTLSamplerState>>,
        index: NSUInteger,
    ) {
        unsafe {
            match *self {
                Self::Vertex(enc) => enc.setVertexSamplerState_atIndex(state, index),
                Self::Fragment(enc) => enc.setFragmentSamplerState_atIndex(state, index),
                Self::Task(enc) => enc.setObjectSamplerState_atIndex(state, index),
                Self::Mesh(enc) => enc.setMeshSamplerState_atIndex(state, index),
                Self::Compute(enc) => enc.setSamplerState_atIndex(state, index),
            }
        }
    }

    fn set_texture(&self, texture: Option<&ProtocolObject<dyn MTLTexture>>, index: NSUInteger) {
        unsafe {
            match *self {
                Self::Vertex(enc) => enc.setVertexTexture_atIndex(texture, index),
                Self::Fragment(enc) => enc.setFragmentTexture_atIndex(texture, index),
                Self::Task(enc) => enc.setObjectTexture_atIndex(texture, index),
                Self::Mesh(enc) => enc.setMeshTexture_atIndex(texture, index),
                Self::Compute(enc) => enc.setTexture_atIndex(texture, index),
            }
        }
    }
}

impl super::CommandEncoder {
    pub fn raw_command_buffer(&self) -> Option<&ProtocolObject<dyn MTLCommandBuffer>> {
        self.raw_cmd_buf.as_deref()
    }

    fn enter_blit(&mut self) -> Retained<ProtocolObject<dyn MTLBlitCommandEncoder>> {
        if self.state.blit.is_none() {
            self.leave_acceleration_structure_builder();
            debug_assert!(self.state.render.is_none() && self.state.compute.is_none());
            let cmd_buf = self.raw_cmd_buf.as_ref().unwrap();

            // Take care of pending timer queries.
            // If we can't use `sample_counters_in_buffer` we have to create a dummy blit encoder!
            //
            // There is a known bug in Metal where blit encoders won't write timestamps if they don't have a blit operation.
            // See https://github.com/gpuweb/gpuweb/issues/2046#issuecomment-1205793680 & https://source.chromium.org/chromium/chromium/src/+/006c4eb70c96229834bbaf271290f40418144cd3:third_party/dawn/src/dawn/native/metal/BackendMTL.mm;l=350
            //
            // To make things worse:
            // * what counts as a blit operation is a bit unclear, experimenting seemed to indicate that resolve_counters doesn't count.
            // * in some cases (when?) using `set_start_of_encoder_sample_index` doesn't work, so we have to use `set_end_of_encoder_sample_index` instead
            //
            // All this means that pretty much the only *reliable* thing as of writing is to:
            // * create a dummy blit encoder using set_end_of_encoder_sample_index
            // * do a dummy write that is known to be not optimized out.
            // * close the encoder since we used set_end_of_encoder_sample_index and don't want to get any extra stuff in there.
            // * create another encoder for whatever we actually had in mind.
            let supports_sample_counters_in_buffer = self
                .shared
                .private_caps
                .timestamp_query_support
                .contains(TimestampQuerySupport::ON_BLIT_ENCODER);

            if !self.state.pending_timer_queries.is_empty() && !supports_sample_counters_in_buffer {
                autoreleasepool(|_| {
                    let descriptor = MTLBlitPassDescriptor::new();
                    let mut last_query = None;
                    for (i, (set, index)) in self.state.pending_timer_queries.drain(..).enumerate()
                    {
                        let sba_descriptor = unsafe {
                            descriptor
                                .sampleBufferAttachments()
                                .objectAtIndexedSubscript(i)
                        };
                        sba_descriptor
                            .setSampleBuffer(Some(set.counter_sample_buffer.as_ref().unwrap()));

                        // Here be dragons:
                        // As mentioned above, for some reasons using the start of the encoder won't yield any results sometimes!
                        unsafe {
                            sba_descriptor.setStartOfEncoderSampleIndex(MTLCounterDontSample)
                        };
                        unsafe { sba_descriptor.setEndOfEncoderSampleIndex(index as _) };

                        last_query = Some((set, index));
                    }
                    let encoder = cmd_buf
                        .blitCommandEncoderWithDescriptor(&descriptor)
                        .unwrap();

                    // As explained above, we need to do some write:
                    // Conveniently, we have a buffer with every query set, that we can use for this for a dummy write,
                    // since we know that it is going to be overwritten again on timer resolve and HAL doesn't define its state before that.
                    let raw_range = NSRange {
                        location: last_query.as_ref().unwrap().1 as usize
                            * crate::QUERY_SIZE as usize,
                        length: 1,
                    };
                    encoder.fillBuffer_range_value(
                        &last_query.as_ref().unwrap().0.raw_buffer,
                        raw_range,
                        255, // Don't write 0, so it's easier to identify if something went wrong.
                    );

                    encoder.endEncoding();
                });
            }

            autoreleasepool(|_| {
                self.state.blit = Some(cmd_buf.blitCommandEncoder().unwrap());
            });

            // Clippy 1.93 hates this (it was patched in 1.93.1)
            #[allow(clippy::panicking_unwrap, reason = "false positive")]
            let encoder = self.state.blit.as_ref().unwrap();

            // UNTESTED:
            // If the above described issue with empty blit encoder applies to `sample_counters_in_buffer` as well, we should use the same workaround instead!
            for (set, index) in self.state.pending_timer_queries.drain(..) {
                debug_assert!(supports_sample_counters_in_buffer);
                unsafe {
                    encoder.sampleCountersInBuffer_atSampleIndex_withBarrier(
                        set.counter_sample_buffer.as_ref().unwrap(),
                        index as _,
                        true,
                    )
                };
            }
        }
        self.state.blit.as_ref().unwrap().clone()
    }

    pub(super) fn leave_blit(&mut self) {
        if let Some(encoder) = self.state.blit.take() {
            encoder.endEncoding();
        }
    }

    fn enter_acceleration_structure_builder(
        &mut self,
    ) -> Retained<ProtocolObject<dyn MTLAccelerationStructureCommandEncoder>> {
        if self.state.acceleration_structure_builder.is_none() {
            self.leave_blit();
            debug_assert!(
                self.state.render.is_none()
                    && self.state.compute.is_none()
                    && self.state.blit.is_none()
            );
            let cmd_buf = self.raw_cmd_buf.as_ref().unwrap();
            autoreleasepool(|_| {
                self.state.acceleration_structure_builder =
                    cmd_buf.accelerationStructureCommandEncoder().to_owned();
            });
        }
        self.state.acceleration_structure_builder.clone().unwrap()
    }

    pub(super) fn leave_acceleration_structure_builder(&mut self) {
        if let Some(encoder) = self.state.acceleration_structure_builder.take() {
            encoder.endEncoding();
        }
    }

    fn active_encoder(&mut self) -> Option<&ProtocolObject<dyn MTLCommandEncoder>> {
        if let Some(ref encoder) = self.state.render {
            Some(ProtocolObject::from_ref(&**encoder))
        } else if let Some(ref encoder) = self.state.acceleration_structure_builder {
            Some(ProtocolObject::from_ref(&**encoder))
        } else if let Some(ref encoder) = self.state.compute {
            Some(ProtocolObject::from_ref(&**encoder))
        } else if let Some(ref encoder) = self.state.blit {
            Some(ProtocolObject::from_ref(&**encoder))
        } else {
            None
        }
    }

    fn begin_pass(&mut self) {
        self.state.reset();
        self.leave_blit();
        self.leave_acceleration_structure_builder();
    }

    /// Updates the bindings for a single shader stage, called in `set_bind_group`.
    fn update_bind_group_state(
        &mut self,
        encoder: Encoder<'_>,
        index_base: super::ResourceData<u32>,
        bg_info: &super::BindGroupLayoutInfo,
        dynamic_offsets: &[wgt::DynamicOffset],
        group_index: u32,
        group: &super::BindGroup,
    ) {
        use naga::ShaderStage as S;
        let resource_indices = match encoder.stage() {
            S::Vertex => &bg_info.base_resource_indices.vs,
            S::Fragment => &bg_info.base_resource_indices.fs,
            S::Task => &bg_info.base_resource_indices.ts,
            S::Mesh => &bg_info.base_resource_indices.ms,
            S::Compute => &bg_info.base_resource_indices.cs,
            S::RayGeneration | S::AnyHit | S::ClosestHit | S::Miss => unimplemented!(),
        };
        let buffers = match encoder.stage() {
            S::Vertex => group.counters.vs.buffers,
            S::Fragment => group.counters.fs.buffers,
            S::Task => group.counters.ts.buffers,
            S::Mesh => group.counters.ms.buffers,
            S::Compute => group.counters.cs.buffers,
            S::RayGeneration | S::AnyHit | S::ClosestHit | S::Miss => unimplemented!(),
        };
        let mut changes_sizes_buffer = false;
        for index in 0..buffers {
            let res = &group.buffers[(index_base.buffers + index) as usize];
            match res {
                super::BufferLikeResource::Buffer {
                    ptr,
                    mut offset,
                    dynamic_index,
                    binding_size,
                    binding_location,
                } => {
                    let buffer = Some(unsafe { ptr.as_ref() });
                    if let Some(dyn_index) = dynamic_index {
                        offset += dynamic_offsets[*dyn_index as usize] as wgt::BufferAddress;
                    }
                    let index = (resource_indices.buffers + index) as usize;
                    encoder.set_buffer(buffer, offset as usize, index);
                    if let Some(size) = binding_size {
                        let br = naga::ResourceBinding {
                            group: group_index,
                            binding: *binding_location,
                        };
                        self.state.storage_buffer_length_map.insert(br, *size);
                        changes_sizes_buffer = true;
                    }
                }
                super::BufferLikeResource::AccelerationStructure(ptr) => {
                    let buffer = Some(unsafe { ptr.as_ref() });
                    let index = (resource_indices.buffers + index) as usize;
                    encoder.set_acceleration_structure(buffer, index);
                }
            }
        }
        if changes_sizes_buffer {
            if let Some((index, sizes)) = self
                .state
                .make_sizes_buffer_update(encoder.stage(), &mut self.temp.binding_sizes)
            {
                let bytes_ptr = NonNull::new(sizes.as_ptr().cast_mut().cast()).unwrap();
                let length = sizes.len() * WORD_SIZE;
                let index = index as _;
                encoder.set_bytes(bytes_ptr, length, index);
            }
        }
        let samplers = match encoder.stage() {
            S::Vertex => group.counters.vs.samplers,
            S::Fragment => group.counters.fs.samplers,
            S::Task => group.counters.ts.samplers,
            S::Mesh => group.counters.ms.samplers,
            S::Compute => group.counters.cs.samplers,
            S::RayGeneration | S::AnyHit | S::ClosestHit | S::Miss => unimplemented!(),
        };
        for index in 0..samplers {
            let res = group.samplers[(index_base.samplers + index) as usize];
            let index = (resource_indices.samplers + index) as usize;
            let state = Some(unsafe { res.as_ref() });
            encoder.set_sampler_state(state, index);
        }

        let textures = match encoder.stage() {
            S::Vertex => group.counters.vs.textures,
            S::Fragment => group.counters.fs.textures,
            S::Task => group.counters.ts.textures,
            S::Mesh => group.counters.ms.textures,
            S::Compute => group.counters.cs.textures,
            S::RayGeneration | S::AnyHit | S::ClosestHit | S::Miss => unimplemented!(),
        };
        for index in 0..textures {
            let res = group.textures[(index_base.textures + index) as usize];
            let index = (resource_indices.textures + index) as usize;
            let texture = Some(unsafe { res.as_ref() });
            encoder.set_texture(texture, index);
        }
    }
}

impl super::CommandState {
    fn reset(&mut self) {
        self.storage_buffer_length_map.clear();
        self.vertex_buffer_size_map.clear();
        self.stage_infos.vs.clear();
        self.stage_infos.fs.clear();
        self.stage_infos.cs.clear();
        self.stage_infos.ts.clear();
        self.stage_infos.ms.clear();
        self.immediates.clear();
    }

    fn make_sizes_buffer_update<'a>(
        &self,
        stage: naga::ShaderStage,
        result_sizes: &'a mut Vec<u32>,
    ) -> Option<(u32, &'a [u32])> {
        let stage_info = &self.stage_infos[stage];
        let slot = stage_info.sizes_slot?;

        result_sizes.clear();
        result_sizes.extend(stage_info.sized_bindings.iter().map(|br| {
            self.storage_buffer_length_map
                .get(br)
                .map(|size| u32::try_from(size.get()).unwrap_or(u32::MAX))
                .unwrap_or_default()
        }));

        // Extend with the sizes of the mapped vertex buffers, in the order
        // they were added to the map.
        result_sizes.extend(stage_info.vertex_buffer_mappings.iter().map(|vbm| {
            self.vertex_buffer_size_map
                .get(&(vbm.id as u64))
                .map(|size| u32::try_from(size.get()).unwrap_or(u32::MAX))
                .unwrap_or_default()
        }));

        if !result_sizes.is_empty() {
            Some((slot as _, result_sizes))
        } else {
            None
        }
    }
}

impl crate::CommandEncoder for super::CommandEncoder {
    type A = super::Api;

    unsafe fn begin_encoding(&mut self, label: crate::Label) -> Result<(), crate::DeviceError> {
        let queue = &self.queue_shared.raw;
        let retain_references = self.shared.settings.retain_command_buffer_references;

        // Guard against exhausting Metal's command buffer budget. Use the hard
        // limit (`MAX_COMMAND_BUFFERS`) so we fail before Metal can hang inside
        // `new_command_buffer`.
        let previous = self
            .queue_shared
            .command_buffer_created_not_submitted
            .fetch_add(1, atomic::Ordering::AcqRel);
        if previous >= adapter::MAX_COMMAND_BUFFERS {
            let current = previous + 1;
            log::warn!(
                "metal: refusing to create new command buffer; {current} outstanding command \
                 buffers exceeds the limit of {}. Treating this as device lost. \
                 Ensure command encoders are submitted or dropped rather than kept alive \
                 to avoid exhausting Metal's command buffer budget.",
                adapter::MAX_COMMAND_BUFFERS
            );
            return Err(crate::DeviceError::Lost);
        }

        let raw = autoreleasepool(move |_| {
            let cmd_buf_ref = if retain_references {
                queue.commandBuffer()
            } else {
                queue.commandBufferWithUnretainedReferences()
            }
            .unwrap();
            if let Some(label) = label {
                cmd_buf_ref.setLabel(Some(&NSString::from_str(label)));
            }
            cmd_buf_ref.to_owned()
        });

        self.raw_cmd_buf = Some(raw);

        Ok(())
    }

    unsafe fn discard_encoding(&mut self) {
        self.leave_blit();
        self.leave_acceleration_structure_builder();
        // when discarding, we don't have a guarantee that
        // everything is in a good state, so check carefully
        if let Some(encoder) = self.state.render.take() {
            encoder.endEncoding();
        }
        if let Some(encoder) = self.state.compute.take() {
            encoder.endEncoding();
        }
        let had_command_buffer = self.raw_cmd_buf.is_some();
        // Clear the Option first so the underlying `metal::CommandBuffer` is
        // dropped before we update the counter.
        self.raw_cmd_buf = None;
        if had_command_buffer {
            self.queue_shared
                .command_buffer_created_not_submitted
                .fetch_sub(1, atomic::Ordering::AcqRel);
        }
    }

    unsafe fn end_encoding(&mut self) -> Result<super::CommandBuffer, crate::DeviceError> {
        // Handle pending timer query if any.
        if !self.state.pending_timer_queries.is_empty() {
            self.leave_blit();
            self.enter_blit();
        }

        self.leave_blit();
        self.leave_acceleration_structure_builder();
        debug_assert!(self.state.render.is_none());
        debug_assert!(self.state.compute.is_none());
        debug_assert!(self.state.pending_timer_queries.is_empty());

        Ok(super::CommandBuffer {
            raw: self.raw_cmd_buf.take().unwrap(),
            queue_shared: Arc::clone(&self.queue_shared),
        })
    }

    unsafe fn reset_all<I>(&mut self, _cmd_bufs: I)
    where
        I: Iterator<Item = super::CommandBuffer>,
    {
        //do nothing
    }

    unsafe fn transition_buffers<'a, T>(&mut self, _barriers: T)
    where
        T: Iterator<Item = crate::BufferBarrier<'a, super::Buffer>>,
    {
    }

    unsafe fn transition_textures<'a, T>(&mut self, _barriers: T)
    where
        T: Iterator<Item = crate::TextureBarrier<'a, super::Texture>>,
    {
    }

    unsafe fn clear_buffer(&mut self, buffer: &super::Buffer, range: crate::MemoryRange) {
        let encoder = self.enter_blit();
        encoder.fillBuffer_range_value(&buffer.raw, conv::map_range(&range), 0);
    }

    unsafe fn copy_buffer_to_buffer<T>(
        &mut self,
        src: &super::Buffer,
        dst: &super::Buffer,
        regions: T,
    ) where
        T: Iterator<Item = crate::BufferCopy>,
    {
        let encoder = self.enter_blit();
        for copy in regions {
            unsafe {
                encoder.copyFromBuffer_sourceOffset_toBuffer_destinationOffset_size(
                    &src.raw,
                    copy.src_offset as usize,
                    &dst.raw,
                    copy.dst_offset as usize,
                    copy.size.get() as usize,
                )
            };
        }
    }

    unsafe fn copy_texture_to_texture<T>(
        &mut self,
        src: &super::Texture,
        _src_usage: wgt::TextureUses,
        dst: &super::Texture,
        regions: T,
    ) where
        T: Iterator<Item = crate::TextureCopy>,
    {
        let dst_texture = if src.format != dst.format {
            let raw_format = self
                .shared
                .private_texture_format_caps
                .map_format(src.format);
            Cow::Owned(autoreleasepool(|_| {
                dst.raw.newTextureViewWithPixelFormat(raw_format).unwrap()
            }))
        } else {
            Cow::Borrowed(&dst.raw)
        };
        let encoder = self.enter_blit();
        for copy in regions {
            let src_origin = conv::map_origin(&copy.src_base.origin);
            let dst_origin = conv::map_origin(&copy.dst_base.origin);
            // no clamping is done: Metal expects physical sizes here
            let extent = conv::map_copy_extent(&copy.size);
            unsafe {
                encoder.copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin(
                    &src.raw,
                    copy.src_base.array_layer as usize,
                    copy.src_base.mip_level as usize,
                    src_origin,
                    extent,
                    &dst_texture,
                    copy.dst_base.array_layer as usize,
                    copy.dst_base.mip_level as usize,
                    dst_origin,
                )
            };
        }
    }

    unsafe fn copy_buffer_to_texture<T>(
        &mut self,
        src: &super::Buffer,
        dst: &super::Texture,
        regions: T,
    ) where
        T: Iterator<Item = crate::BufferTextureCopy>,
    {
        let encoder = self.enter_blit();
        for copy in regions {
            let dst_origin = conv::map_origin(&copy.texture_base.origin);
            // Metal expects buffer-texture copies in virtual sizes
            let extent = copy
                .texture_base
                .max_copy_size(&dst.copy_size)
                .min(&copy.size);
            let bytes_per_row = copy.buffer_layout.bytes_per_row.unwrap_or(0) as u64;
            let image_byte_stride = if extent.depth > 1 {
                copy.buffer_layout
                    .rows_per_image
                    .map_or(0, |v| v as u64 * bytes_per_row)
            } else {
                // Don't pass a stride when updating a single layer, otherwise metal validation
                // fails when updating a subset of the image due to the stride being larger than
                // the amount of data to copy.
                0
            };
            unsafe {
                encoder.copyFromBuffer_sourceOffset_sourceBytesPerRow_sourceBytesPerImage_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin_options(
                    &src.raw,
                    copy.buffer_layout.offset as usize,
                    bytes_per_row as usize,
                    image_byte_stride as usize,
                    conv::map_copy_extent(&extent),
                    &dst.raw,
                    copy.texture_base.array_layer as usize,
                    copy.texture_base.mip_level as usize,
                    dst_origin,
                    conv::get_blit_option(dst.format, copy.texture_base.aspect),
                )
            };
        }
    }

    unsafe fn copy_texture_to_buffer<T>(
        &mut self,
        src: &super::Texture,
        _src_usage: wgt::TextureUses,
        dst: &super::Buffer,
        regions: T,
    ) where
        T: Iterator<Item = crate::BufferTextureCopy>,
    {
        let encoder = self.enter_blit();
        for copy in regions {
            let src_origin = conv::map_origin(&copy.texture_base.origin);
            // Metal expects texture-buffer copies in virtual sizes
            let extent = copy
                .texture_base
                .max_copy_size(&src.copy_size)
                .min(&copy.size);
            let bytes_per_row = copy.buffer_layout.bytes_per_row.unwrap_or(0) as u64;
            let bytes_per_image = copy
                .buffer_layout
                .rows_per_image
                .map_or(0, |v| v as u64 * bytes_per_row);
            unsafe {
                encoder.copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toBuffer_destinationOffset_destinationBytesPerRow_destinationBytesPerImage_options(
                    &src.raw,
                    copy.texture_base.array_layer as usize,
                    copy.texture_base.mip_level as usize,
                    src_origin,
                    conv::map_copy_extent(&extent),
                    &dst.raw,
                    copy.buffer_layout.offset as usize,
                    bytes_per_row as usize,
                    bytes_per_image as usize,
                    conv::get_blit_option(src.format, copy.texture_base.aspect),
                )
            };
        }
    }

    unsafe fn copy_acceleration_structure_to_acceleration_structure(
        &mut self,
        src: &super::AccelerationStructure,
        dst: &super::AccelerationStructure,
        copy: wgt::AccelerationStructureCopy,
    ) {
        let command_encoder = self.enter_acceleration_structure_builder();
        match copy {
            wgt::AccelerationStructureCopy::Clone => unsafe {
                command_encoder
                    .copyAccelerationStructure_toAccelerationStructure(&src.raw, &dst.raw);
            },
            wgt::AccelerationStructureCopy::Compact => {
                command_encoder.copyAndCompactAccelerationStructure_toAccelerationStructure(
                    &src.raw, &dst.raw,
                );
            }
        };
    }

    unsafe fn begin_query(&mut self, set: &super::QuerySet, index: u32) {
        match set.ty {
            wgt::QueryType::Occlusion => {
                self.state
                    .render
                    .as_ref()
                    .unwrap()
                    .setVisibilityResultMode_offset(
                        MTLVisibilityResultMode::Boolean,
                        index as usize * crate::QUERY_SIZE as usize,
                    );
            }
            _ => {}
        }
    }
    unsafe fn end_query(&mut self, set: &super::QuerySet, _index: u32) {
        match set.ty {
            wgt::QueryType::Occlusion => {
                self.state
                    .render
                    .as_ref()
                    .unwrap()
                    .setVisibilityResultMode_offset(MTLVisibilityResultMode::Disabled, 0);
            }
            _ => {}
        }
    }
    unsafe fn write_timestamp(&mut self, set: &super::QuerySet, index: u32) {
        let support = self.shared.private_caps.timestamp_query_support;
        debug_assert!(
            support.contains(TimestampQuerySupport::STAGE_BOUNDARIES),
            "Timestamp queries are not supported"
        );
        let sample_buffer = set.counter_sample_buffer.as_ref().unwrap();
        let with_barrier = true;

        // Try to use an existing encoder for timestamp query if possible.
        // This works only if it's supported for the active encoder.
        if let (true, Some(encoder)) = (
            support.contains(TimestampQuerySupport::ON_BLIT_ENCODER),
            self.state.blit.as_ref(),
        ) {
            unsafe {
                encoder.sampleCountersInBuffer_atSampleIndex_withBarrier(
                    sample_buffer,
                    index as _,
                    with_barrier,
                )
            };
        } else if let (true, Some(encoder)) = (
            support.contains(TimestampQuerySupport::ON_RENDER_ENCODER),
            self.state.render.as_ref(),
        ) {
            unsafe {
                encoder.sampleCountersInBuffer_atSampleIndex_withBarrier(
                    sample_buffer,
                    index as _,
                    with_barrier,
                )
            };
        } else if let (true, Some(encoder)) = (
            support.contains(TimestampQuerySupport::ON_COMPUTE_ENCODER),
            self.state.compute.as_ref(),
        ) {
            unsafe {
                encoder.sampleCountersInBuffer_atSampleIndex_withBarrier(
                    sample_buffer,
                    index as _,
                    with_barrier,
                )
            };
        } else {
            // If we're here it means we either have no encoder open, or it's not supported to sample within them.
            // If this happens with render/compute open, this is an invalid usage!
            debug_assert!(self.state.render.is_none() && self.state.compute.is_none());

            // But otherwise it means we'll put defer this to the next created encoder.
            self.state.pending_timer_queries.push((set.clone(), index));

            // Ensure we didn't already have a blit open.
            self.leave_blit();
        };
    }

    unsafe fn reset_queries(&mut self, set: &super::QuerySet, range: Range<u32>) {
        let encoder = self.enter_blit();
        let raw_range = NSRange {
            location: range.start as usize * crate::QUERY_SIZE as usize,
            length: (range.end - range.start) as usize * crate::QUERY_SIZE as usize,
        };
        encoder.fillBuffer_range_value(&set.raw_buffer, raw_range, 0);
    }

    unsafe fn copy_query_results(
        &mut self,
        set: &super::QuerySet,
        range: Range<u32>,
        buffer: &super::Buffer,
        offset: wgt::BufferAddress,
        _: wgt::BufferSize, // Metal doesn't support queries that are bigger than a single element are not supported
    ) {
        let encoder = self.enter_blit();
        match set.ty {
            wgt::QueryType::Occlusion => {
                let size = (range.end - range.start) as u64 * crate::QUERY_SIZE;
                unsafe {
                    encoder.copyFromBuffer_sourceOffset_toBuffer_destinationOffset_size(
                        &set.raw_buffer,
                        range.start as usize * crate::QUERY_SIZE as usize,
                        &buffer.raw,
                        offset as usize,
                        size as usize,
                    )
                };
            }
            wgt::QueryType::Timestamp => {
                unsafe {
                    encoder.resolveCounters_inRange_destinationBuffer_destinationOffset(
                        set.counter_sample_buffer.as_ref().unwrap(),
                        NSRange::new(range.start as usize, (range.end - range.start) as usize),
                        &buffer.raw,
                        offset as usize,
                    )
                };
            }
            wgt::QueryType::PipelineStatistics(_) => todo!(),
        }
    }

    // render

    unsafe fn begin_render_pass(
        &mut self,
        desc: &crate::RenderPassDescriptor<super::QuerySet, super::TextureView>,
    ) -> Result<(), crate::DeviceError> {
        self.begin_pass();
        self.state.index = None;

        assert!(self.state.blit.is_none());
        assert!(self.state.compute.is_none());
        assert!(self.state.render.is_none());

        autoreleasepool(|_| {
            let descriptor = MTLRenderPassDescriptor::new();

            for (i, at) in desc.color_attachments.iter().enumerate() {
                if let Some(at) = at.as_ref() {
                    let at_descriptor =
                        unsafe { descriptor.colorAttachments().objectAtIndexedSubscript(i) };
                    at_descriptor.setTexture(Some(&at.target.view.raw));
                    if let Some(depth_slice) = at.depth_slice {
                        at_descriptor.setDepthPlane(depth_slice as usize);
                    }
                    if let Some(ref resolve) = at.resolve_target {
                        //Note: the selection of levels and slices is already handled by `TextureView`
                        at_descriptor.setResolveTexture(Some(&resolve.view.raw));
                    }
                    let load_action = if at.ops.contains(crate::AttachmentOps::LOAD) {
                        MTLLoadAction::Load
                    } else if at.ops.contains(crate::AttachmentOps::LOAD_DONT_CARE) {
                        MTLLoadAction::DontCare
                    } else if at.ops.contains(crate::AttachmentOps::LOAD_CLEAR) {
                        at_descriptor.setClearColor(conv::map_clear_color(&at.clear_value));
                        MTLLoadAction::Clear
                    } else {
                        unreachable!()
                    };
                    let store_action = conv::map_store_action(
                        at.ops.contains(crate::AttachmentOps::STORE),
                        at.resolve_target.is_some(),
                    );
                    at_descriptor.setLoadAction(load_action);
                    at_descriptor.setStoreAction(store_action);
                }
            }

            if let Some(ref at) = desc.depth_stencil_attachment {
                if at.target.view.aspects.contains(crate::FormatAspects::DEPTH) {
                    let at_descriptor = descriptor.depthAttachment();
                    at_descriptor.setTexture(Some(&at.target.view.raw));

                    let load_action = if at.depth_ops.contains(crate::AttachmentOps::LOAD) {
                        MTLLoadAction::Load
                    } else if at.depth_ops.contains(crate::AttachmentOps::LOAD_DONT_CARE) {
                        MTLLoadAction::DontCare
                    } else if at.depth_ops.contains(crate::AttachmentOps::LOAD_CLEAR) {
                        at_descriptor.setClearDepth(at.clear_value.0 as f64);
                        MTLLoadAction::Clear
                    } else {
                        unreachable!();
                    };
                    let store_action = if at.depth_ops.contains(crate::AttachmentOps::STORE) {
                        MTLStoreAction::Store
                    } else {
                        MTLStoreAction::DontCare
                    };
                    at_descriptor.setLoadAction(load_action);
                    at_descriptor.setStoreAction(store_action);
                }
                if at
                    .target
                    .view
                    .aspects
                    .contains(crate::FormatAspects::STENCIL)
                {
                    let at_descriptor = descriptor.stencilAttachment();
                    at_descriptor.setTexture(Some(&at.target.view.raw));

                    let load_action = if at.stencil_ops.contains(crate::AttachmentOps::LOAD) {
                        MTLLoadAction::Load
                    } else if at
                        .stencil_ops
                        .contains(crate::AttachmentOps::LOAD_DONT_CARE)
                    {
                        MTLLoadAction::DontCare
                    } else if at.stencil_ops.contains(crate::AttachmentOps::LOAD_CLEAR) {
                        at_descriptor.setClearStencil(at.clear_value.1);
                        MTLLoadAction::Clear
                    } else {
                        unreachable!()
                    };
                    let store_action = if at.stencil_ops.contains(crate::AttachmentOps::STORE) {
                        MTLStoreAction::Store
                    } else {
                        MTLStoreAction::DontCare
                    };
                    at_descriptor.setLoadAction(load_action);
                    at_descriptor.setStoreAction(store_action);
                }
            }

            let mut sba_index = 0;
            let mut next_sba_descriptor = || {
                let sba_descriptor = unsafe {
                    descriptor
                        .sampleBufferAttachments()
                        .objectAtIndexedSubscript(sba_index)
                };

                unsafe { sba_descriptor.setEndOfVertexSampleIndex(MTLCounterDontSample) };
                unsafe { sba_descriptor.setStartOfFragmentSampleIndex(MTLCounterDontSample) };

                sba_index += 1;
                sba_descriptor
            };

            for (set, index) in self.state.pending_timer_queries.drain(..) {
                let sba_descriptor = next_sba_descriptor();
                sba_descriptor.setSampleBuffer(Some(set.counter_sample_buffer.as_ref().unwrap()));
                unsafe { sba_descriptor.setStartOfVertexSampleIndex(index as _) };
                unsafe { sba_descriptor.setEndOfFragmentSampleIndex(MTLCounterDontSample) };
            }

            if let Some(ref timestamp_writes) = desc.timestamp_writes {
                let sba_descriptor = next_sba_descriptor();
                sba_descriptor.setSampleBuffer(Some(
                    timestamp_writes
                        .query_set
                        .counter_sample_buffer
                        .as_ref()
                        .unwrap(),
                ));

                unsafe {
                    sba_descriptor.setStartOfVertexSampleIndex(
                        timestamp_writes
                            .beginning_of_pass_write_index
                            .map_or(MTLCounterDontSample, |i| i as _),
                    )
                };
                unsafe {
                    sba_descriptor.setEndOfFragmentSampleIndex(
                        timestamp_writes
                            .end_of_pass_write_index
                            .map_or(MTLCounterDontSample, |i| i as _),
                    )
                };
            }

            if let Some(occlusion_query_set) = desc.occlusion_query_set {
                descriptor.setVisibilityResultBuffer(Some(occlusion_query_set.raw_buffer.as_ref()))
            }
            // This strangely isn't mentioned in https://developer.apple.com/documentation/metal/improving-rendering-performance-with-vertex-amplification.
            // The docs for [`renderTargetArrayLength`](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/rendertargetarraylength)
            // also say "The number of active layers that all attachments must have for layered rendering," implying it is only for layered rendering.
            // However, when I don't set this, I get undefined behavior in nonzero layers, and all non-apple examples of vertex amplification set it.
            // So this is just one of those undocumented requirements.
            if let Some(mv) = desc.multiview_mask {
                descriptor.setRenderTargetArrayLength(32 - mv.leading_zeros() as usize);
            }
            let raw = self.raw_cmd_buf.as_ref().unwrap();
            let encoder = raw.renderCommandEncoderWithDescriptor(&descriptor).unwrap();
            if let Some(mv) = desc.multiview_mask {
                // Most likely the API just wasn't thought about enough. It's not like they ever allow you
                // to use enough views to overflow a 32-bit bitmask.
                let mv = mv.get();
                let msb = 32 - mv.leading_zeros();
                let mut maps: SmallVec<[MTLVertexAmplificationViewMapping; 32]> = SmallVec::new();
                for i in 0..msb {
                    if (mv & (1 << i)) != 0 {
                        maps.push(MTLVertexAmplificationViewMapping {
                            renderTargetArrayIndexOffset: i,
                            viewportArrayIndexOffset: i,
                        });
                    }
                }
                unsafe {
                    encoder.setVertexAmplificationCount_viewMappings(
                        mv.count_ones() as usize,
                        maps.as_ptr(),
                    )
                };
            }
            if let Some(label) = desc.label {
                encoder.setLabel(Some(&NSString::from_str(label)));
            }
            self.state.render = Some(encoder);
        });

        Ok(())
    }

    unsafe fn end_render_pass(&mut self) {
        self.state.render.take().unwrap().endEncoding();
    }

    unsafe fn set_bind_group(
        &mut self,
        layout: &super::PipelineLayout,
        group_index: u32,
        group: &super::BindGroup,
        dynamic_offsets: &[wgt::DynamicOffset],
    ) {
        let bg_info = layout.bind_group_infos[group_index as usize]
            .as_ref()
            .unwrap();
        let render_encoder = self.state.render.clone();
        let compute_encoder = self.state.compute.clone();
        if let Some(encoder) = render_encoder {
            self.update_bind_group_state(
                Encoder::Vertex(&encoder),
                // All zeros, as vs comes first
                super::ResourceData::default(),
                bg_info,
                dynamic_offsets,
                group_index,
                group,
            );
            self.update_bind_group_state(
                Encoder::Task(&encoder),
                // All zeros, as ts comes first
                super::ResourceData::default(),
                bg_info,
                dynamic_offsets,
                group_index,
                group,
            );
            self.update_bind_group_state(
                Encoder::Mesh(&encoder),
                group.counters.ts.clone(),
                bg_info,
                dynamic_offsets,
                group_index,
                group,
            );
            self.update_bind_group_state(
                Encoder::Fragment(&encoder),
                super::ResourceData {
                    buffers: group.counters.vs.buffers
                        + group.counters.ts.buffers
                        + group.counters.ms.buffers,
                    textures: group.counters.vs.textures
                        + group.counters.ts.textures
                        + group.counters.ms.textures,
                    samplers: group.counters.vs.samplers
                        + group.counters.ts.samplers
                        + group.counters.ms.samplers,
                },
                bg_info,
                dynamic_offsets,
                group_index,
                group,
            );
            // Call useResource on all textures and buffers used indirectly so they are alive
            for (resource, use_info) in group.resources_to_use.iter() {
                encoder.useResource_usage_stages(
                    unsafe { resource.as_ref() },
                    use_info.uses,
                    use_info.stages,
                );
            }
        }
        if let Some(encoder) = compute_encoder {
            self.update_bind_group_state(
                Encoder::Compute(&encoder),
                super::ResourceData {
                    buffers: group.counters.vs.buffers
                        + group.counters.ts.buffers
                        + group.counters.ms.buffers
                        + group.counters.fs.buffers,
                    textures: group.counters.vs.textures
                        + group.counters.ts.textures
                        + group.counters.ms.textures
                        + group.counters.fs.textures,
                    samplers: group.counters.vs.samplers
                        + group.counters.ts.samplers
                        + group.counters.ms.samplers
                        + group.counters.fs.samplers,
                },
                bg_info,
                dynamic_offsets,
                group_index,
                group,
            );
            // Call useResource on all textures and buffers used indirectly so they are alive
            for (resource, use_info) in group.resources_to_use.iter() {
                if !use_info.visible_in_compute {
                    continue;
                }
                encoder.useResource_usage(unsafe { resource.as_ref() }, use_info.uses);
            }
        }
    }

    unsafe fn set_immediates(
        &mut self,
        layout: &super::PipelineLayout,
        offset_bytes: u32,
        data: &[u32],
    ) {
        let state_pc = &mut self.state.immediates;
        if state_pc.len() < layout.total_immediates as usize {
            state_pc.resize(layout.total_immediates as usize, 0);
        }
        debug_assert_eq!(offset_bytes as usize % WORD_SIZE, 0);

        let offset_words = offset_bytes as usize / WORD_SIZE;
        state_pc[offset_words..offset_words + data.len()].copy_from_slice(data);

        let bytes = NonNull::new(state_pc.as_ptr().cast_mut().cast()).unwrap();
        if let Some(ref compute) = self.state.compute {
            unsafe {
                compute.setBytes_length_atIndex(
                    bytes,
                    layout.total_immediates as usize * WORD_SIZE,
                    layout.immediates_infos.cs.unwrap().buffer_index as usize,
                )
            };
        }
        if let Some(ref render) = self.state.render {
            if let Some(vs) = layout.immediates_infos.vs {
                unsafe {
                    render.setVertexBytes_length_atIndex(
                        bytes,
                        layout.total_immediates as usize * WORD_SIZE,
                        vs.buffer_index as _,
                    )
                }
            }
            if let Some(fs) = layout.immediates_infos.fs {
                unsafe {
                    render.setFragmentBytes_length_atIndex(
                        bytes,
                        layout.total_immediates as usize * WORD_SIZE,
                        fs.buffer_index as _,
                    )
                }
            }
            if let Some(ts) = layout.immediates_infos.ts {
                if self.shared.private_caps.mesh_shaders {
                    unsafe {
                        render.setObjectBytes_length_atIndex(
                            bytes,
                            layout.total_immediates as usize * WORD_SIZE,
                            ts.buffer_index as _,
                        )
                    }
                }
            }
            if let Some(ms) = layout.immediates_infos.ms {
                if self.shared.private_caps.mesh_shaders {
                    unsafe {
                        render.setMeshBytes_length_atIndex(
                            bytes,
                            layout.total_immediates as usize * WORD_SIZE,
                            ms.buffer_index as _,
                        )
                    }
                }
            }
        }
    }

    unsafe fn insert_debug_marker(&mut self, label: &str) {
        if let Some(encoder) = self.active_encoder() {
            encoder.insertDebugSignpost(&NSString::from_str(label));
        }
    }
    unsafe fn begin_debug_marker(&mut self, group_label: &str) {
        if let Some(encoder) = self.active_encoder() {
            encoder.pushDebugGroup(&NSString::from_str(group_label));
        } else if let Some(ref buf) = self.raw_cmd_buf {
            buf.pushDebugGroup(&NSString::from_str(group_label));
        }
    }
    unsafe fn end_debug_marker(&mut self) {
        if let Some(encoder) = self.active_encoder() {
            encoder.popDebugGroup();
        } else if let Some(ref buf) = self.raw_cmd_buf {
            buf.popDebugGroup();
        }
    }

    unsafe fn set_render_pipeline(&mut self, pipeline: &super::RenderPipeline) {
        self.state.raw_primitive_type = pipeline.raw_primitive_type;
        match pipeline.vs_info {
            Some(ref info) => self.state.stage_infos.vs.assign_from(info),
            None => self.state.stage_infos.vs.clear(),
        }
        match pipeline.fs_info {
            Some(ref info) => self.state.stage_infos.fs.assign_from(info),
            None => self.state.stage_infos.fs.clear(),
        }
        match pipeline.ts_info {
            Some(ref info) => self.state.stage_infos.ts.assign_from(info),
            None => self.state.stage_infos.ts.clear(),
        }
        match pipeline.ms_info {
            Some(ref info) => self.state.stage_infos.ms.assign_from(info),
            None => self.state.stage_infos.ms.clear(),
        }

        let encoder = self.state.render.as_ref().unwrap();
        encoder.setRenderPipelineState(&pipeline.raw);
        encoder.setFrontFacingWinding(pipeline.raw_front_winding);
        encoder.setCullMode(pipeline.raw_cull_mode);
        encoder.setTriangleFillMode(pipeline.raw_triangle_fill_mode);
        if let Some(depth_clip) = pipeline.raw_depth_clip_mode {
            encoder.setDepthClipMode(depth_clip);
        }
        if let Some((ref state, bias)) = pipeline.depth_stencil {
            encoder.setDepthStencilState(Some(state));
            encoder.setDepthBias_slopeScale_clamp(
                bias.constant as f32,
                bias.slope_scale,
                bias.clamp,
            );
        }

        if pipeline.vs_info.is_some() {
            if let Some((index, sizes)) = self
                .state
                .make_sizes_buffer_update(naga::ShaderStage::Vertex, &mut self.temp.binding_sizes)
            {
                unsafe {
                    encoder.setVertexBytes_length_atIndex(
                        NonNull::new(sizes.as_ptr().cast_mut().cast()).unwrap(),
                        sizes.len() * WORD_SIZE,
                        index as _,
                    )
                };
            }
        }
        if pipeline.fs_info.is_some() {
            if let Some((index, sizes)) = self
                .state
                .make_sizes_buffer_update(naga::ShaderStage::Fragment, &mut self.temp.binding_sizes)
            {
                unsafe {
                    encoder.setFragmentBytes_length_atIndex(
                        NonNull::new(sizes.as_ptr().cast_mut().cast()).unwrap(),
                        sizes.len() * WORD_SIZE,
                        index as _,
                    )
                };
            }
        }
        if let Some(ts_info) = &pipeline.ts_info {
            // update the threadgroup memory sizes
            while self.state.stage_infos.ms.work_group_memory_sizes.len()
                < ts_info.work_group_memory_sizes.len()
            {
                self.state.stage_infos.ms.work_group_memory_sizes.push(0);
            }
            for (index, (cur_size, pipeline_size)) in self
                .state
                .stage_infos
                .ms
                .work_group_memory_sizes
                .iter_mut()
                .zip(ts_info.work_group_memory_sizes.iter())
                .enumerate()
            {
                let size = pipeline_size.next_multiple_of(16);
                if *cur_size != size {
                    *cur_size = size;
                    unsafe { encoder.setObjectThreadgroupMemoryLength_atIndex(size as _, index) };
                }
            }
            if let Some((index, sizes)) = self
                .state
                .make_sizes_buffer_update(naga::ShaderStage::Task, &mut self.temp.binding_sizes)
            {
                unsafe {
                    encoder.setObjectBytes_length_atIndex(
                        NonNull::new(sizes.as_ptr().cast_mut().cast()).unwrap(),
                        sizes.len() * WORD_SIZE,
                        index as _,
                    )
                };
            }
        }
        if let Some(_ms_info) = &pipeline.ms_info {
            // So there isn't an equivalent to
            // https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setthreadgroupmemorylength(_:offset:index:)
            // for mesh shaders. This is probably because the CPU has less control over the dispatch sizes and such. Interestingly
            // it also affects mesh shaders without task/object shaders, even though none of compute, task or fragment shaders
            // behave this way.
            if let Some((index, sizes)) = self
                .state
                .make_sizes_buffer_update(naga::ShaderStage::Mesh, &mut self.temp.binding_sizes)
            {
                unsafe {
                    encoder.setMeshBytes_length_atIndex(
                        NonNull::new(sizes.as_ptr().cast_mut().cast()).unwrap(),
                        sizes.len() * WORD_SIZE,
                        index as _,
                    )
                };
            }
        }
    }

    unsafe fn set_index_buffer<'a>(
        &mut self,
        binding: crate::BufferBinding<'a, super::Buffer>,
        format: wgt::IndexFormat,
    ) {
        let (stride, raw_type) = conv::map_index_format(format);
        self.state.index = Some(super::IndexState {
            buffer_ptr: NonNull::from(&*binding.buffer.raw),
            offset: binding.offset,
            stride,
            raw_type,
        });
    }

    unsafe fn set_vertex_buffer<'a>(
        &mut self,
        index: u32,
        binding: crate::BufferBinding<'a, super::Buffer>,
    ) {
        let buffer_index = self.shared.private_caps.max_vertex_buffers as u64 - 1 - index as u64;
        let encoder = self.state.render.as_ref().unwrap();
        unsafe {
            encoder.setVertexBuffer_offset_atIndex(
                Some(&binding.buffer.raw),
                binding.offset as usize,
                buffer_index as usize,
            )
        };

        let buffer_size = binding.resolve_size();
        if buffer_size > 0 {
            self.state.vertex_buffer_size_map.insert(
                buffer_index,
                core::num::NonZeroU64::new(buffer_size).unwrap(),
            );
        } else {
            self.state.vertex_buffer_size_map.remove(&buffer_index);
        }

        if let Some((index, sizes)) = self
            .state
            .make_sizes_buffer_update(naga::ShaderStage::Vertex, &mut self.temp.binding_sizes)
        {
            unsafe {
                encoder.setVertexBytes_length_atIndex(
                    NonNull::new(sizes.as_ptr().cast_mut().cast()).unwrap(),
                    sizes.len() * WORD_SIZE,
                    index as _,
                )
            };
        }
    }

    unsafe fn set_viewport(&mut self, rect: &crate::Rect<f32>, depth_range: Range<f32>) {
        let zfar = if self.shared.disabilities.broken_viewport_near_depth {
            depth_range.end - depth_range.start
        } else {
            depth_range.end
        };
        let encoder = self.state.render.as_ref().unwrap();
        encoder.setViewport(MTLViewport {
            originX: rect.x as _,
            originY: rect.y as _,
            width: rect.w as _,
            height: rect.h as _,
            znear: depth_range.start as _,
            zfar: zfar as _,
        });
    }
    unsafe fn set_scissor_rect(&mut self, rect: &crate::Rect<u32>) {
        //TODO: support empty scissors by modifying the viewport
        let scissor = MTLScissorRect {
            x: rect.x as _,
            y: rect.y as _,
            width: rect.w as _,
            height: rect.h as _,
        };
        let encoder = self.state.render.as_ref().unwrap();
        encoder.setScissorRect(scissor);
    }
    unsafe fn set_stencil_reference(&mut self, value: u32) {
        let encoder = self.state.render.as_ref().unwrap();
        encoder.setStencilFrontReferenceValue_backReferenceValue(value, value);
    }
    unsafe fn set_blend_constants(&mut self, color: &[f32; 4]) {
        let encoder = self.state.render.as_ref().unwrap();
        encoder.setBlendColorRed_green_blue_alpha(color[0], color[1], color[2], color[3]);
    }

    unsafe fn draw(
        &mut self,
        first_vertex: u32,
        vertex_count: u32,
        first_instance: u32,
        instance_count: u32,
    ) {
        let encoder = self.state.render.as_ref().unwrap();
        if first_instance != 0 {
            unsafe {
                encoder.drawPrimitives_vertexStart_vertexCount_instanceCount_baseInstance(
                    self.state.raw_primitive_type,
                    first_vertex as _,
                    vertex_count as _,
                    instance_count as _,
                    first_instance as _,
                )
            };
        } else if instance_count != 1 {
            unsafe {
                encoder.drawPrimitives_vertexStart_vertexCount_instanceCount(
                    self.state.raw_primitive_type,
                    first_vertex as _,
                    vertex_count as _,
                    instance_count as _,
                )
            };
        } else {
            unsafe {
                encoder.drawPrimitives_vertexStart_vertexCount(
                    self.state.raw_primitive_type,
                    first_vertex as _,
                    vertex_count as _,
                )
            };
        }
    }

    unsafe fn draw_indexed(
        &mut self,
        first_index: u32,
        index_count: u32,
        base_vertex: i32,
        first_instance: u32,
        instance_count: u32,
    ) {
        let encoder = self.state.render.as_ref().unwrap();
        let index = self.state.index.as_ref().unwrap();
        let offset = (index.offset + index.stride * first_index as wgt::BufferAddress) as usize;
        if base_vertex != 0 || first_instance != 0 {
            unsafe {
                encoder.drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset_instanceCount_baseVertex_baseInstance(
                    self.state.raw_primitive_type,
                    index_count as _,
                    index.raw_type,
                    index.buffer_ptr.as_ref(),
                    offset,
                    instance_count as _,
                    base_vertex as _,
                    first_instance as _,
                )
            };
        } else if instance_count != 1 {
            unsafe {
                encoder.drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset_instanceCount(
                    self.state.raw_primitive_type,
                    index_count as _,
                    index.raw_type,
                    index.buffer_ptr.as_ref(),
                    offset,
                    instance_count as _,
                )
            };
        } else {
            unsafe {
                encoder.drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset(
                    self.state.raw_primitive_type,
                    index_count as _,
                    index.raw_type,
                    index.buffer_ptr.as_ref(),
                    offset,
                )
            };
        }
    }

    unsafe fn draw_mesh_tasks(
        &mut self,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        let encoder = self.state.render.as_ref().unwrap();
        let size = MTLSize {
            width: group_count_x as usize,
            height: group_count_y as usize,
            depth: group_count_z as usize,
        };
        encoder.drawMeshThreadgroups_threadsPerObjectThreadgroup_threadsPerMeshThreadgroup(
            size,
            self.state.stage_infos.ts.raw_wg_size,
            self.state.stage_infos.ms.raw_wg_size,
        );
    }

    unsafe fn draw_indirect(
        &mut self,
        buffer: &super::Buffer,
        mut offset: wgt::BufferAddress,
        draw_count: u32,
    ) {
        let encoder = self.state.render.as_ref().unwrap();
        for _ in 0..draw_count {
            unsafe {
                encoder.drawPrimitives_indirectBuffer_indirectBufferOffset(
                    self.state.raw_primitive_type,
                    &buffer.raw,
                    offset as usize,
                )
            };
            offset += size_of::<wgt::DrawIndirectArgs>() as wgt::BufferAddress;
        }
    }

    unsafe fn draw_indexed_indirect(
        &mut self,
        buffer: &super::Buffer,
        mut offset: wgt::BufferAddress,
        draw_count: u32,
    ) {
        let encoder = self.state.render.as_ref().unwrap();
        let index = self.state.index.as_ref().unwrap();
        for _ in 0..draw_count {
            unsafe {
                encoder.drawIndexedPrimitives_indexType_indexBuffer_indexBufferOffset_indirectBuffer_indirectBufferOffset(
                    self.state.raw_primitive_type,
                    index.raw_type,
                    index.buffer_ptr.as_ref(),
                    index.offset as usize,
                    &buffer.raw,
                    offset as usize,
                )
            };
            offset += size_of::<wgt::DrawIndexedIndirectArgs>() as wgt::BufferAddress;
        }
    }

    unsafe fn draw_mesh_tasks_indirect(
        &mut self,
        buffer: &<Self::A as crate::Api>::Buffer,
        mut offset: wgt::BufferAddress,
        draw_count: u32,
    ) {
        let encoder = self.state.render.as_ref().unwrap();
        for _ in 0..draw_count {
            unsafe {
                encoder.drawMeshThreadgroupsWithIndirectBuffer_indirectBufferOffset_threadsPerObjectThreadgroup_threadsPerMeshThreadgroup(
                    &buffer.raw,
                    offset as usize,
                    self.state.stage_infos.ts.raw_wg_size,
                    self.state.stage_infos.ms.raw_wg_size,
                )
            };
            offset += size_of::<wgt::DispatchIndirectArgs>() as wgt::BufferAddress;
        }
    }

    unsafe fn draw_indirect_count(
        &mut self,
        _buffer: &super::Buffer,
        _offset: wgt::BufferAddress,
        _count_buffer: &super::Buffer,
        _count_offset: wgt::BufferAddress,
        _max_count: u32,
    ) {
        //TODO
    }
    unsafe fn draw_indexed_indirect_count(
        &mut self,
        _buffer: &super::Buffer,
        _offset: wgt::BufferAddress,
        _count_buffer: &super::Buffer,
        _count_offset: wgt::BufferAddress,
        _max_count: u32,
    ) {
        //TODO
    }

    unsafe fn draw_mesh_tasks_indirect_count(
        &mut self,
        _buffer: &<Self::A as crate::Api>::Buffer,
        _offset: wgt::BufferAddress,
        _count_buffer: &<Self::A as crate::Api>::Buffer,
        _count_offset: wgt::BufferAddress,
        _max_count: u32,
    ) {
        unreachable!()
    }

    // compute

    unsafe fn begin_compute_pass(&mut self, desc: &crate::ComputePassDescriptor<super::QuerySet>) {
        self.begin_pass();

        debug_assert!(self.state.blit.is_none());
        debug_assert!(self.state.compute.is_none());
        debug_assert!(self.state.render.is_none());

        let raw = self.raw_cmd_buf.as_ref().unwrap();

        autoreleasepool(|_| {
            // TimeStamp Queries and ComputePassDescriptor were both introduced in Metal 2.3 (macOS 11, iOS 14)
            // and we currently only need ComputePassDescriptor for timestamp queries
            let encoder = if self.shared.private_caps.timestamp_query_support.is_empty() {
                raw.computeCommandEncoder().unwrap()
            } else {
                let descriptor = MTLComputePassDescriptor::new();

                let mut sba_index = 0;
                let mut next_sba_descriptor = || {
                    let sba_descriptor = unsafe {
                        descriptor
                            .sampleBufferAttachments()
                            .objectAtIndexedSubscript(sba_index)
                    };
                    sba_index += 1;
                    sba_descriptor
                };

                for (set, index) in self.state.pending_timer_queries.drain(..) {
                    let sba_descriptor = next_sba_descriptor();
                    sba_descriptor
                        .setSampleBuffer(Some(set.counter_sample_buffer.as_ref().unwrap()));
                    unsafe { sba_descriptor.setStartOfEncoderSampleIndex(index as _) };
                    unsafe { sba_descriptor.setEndOfEncoderSampleIndex(MTLCounterDontSample) };
                }

                if let Some(timestamp_writes) = desc.timestamp_writes.as_ref() {
                    let sba_descriptor = next_sba_descriptor();
                    sba_descriptor.setSampleBuffer(Some(
                        timestamp_writes
                            .query_set
                            .counter_sample_buffer
                            .as_ref()
                            .unwrap(),
                    ));

                    unsafe {
                        sba_descriptor.setStartOfEncoderSampleIndex(
                            timestamp_writes
                                .beginning_of_pass_write_index
                                .map_or(MTLCounterDontSample, |i| i as _),
                        )
                    };
                    unsafe {
                        sba_descriptor.setEndOfEncoderSampleIndex(
                            timestamp_writes
                                .end_of_pass_write_index
                                .map_or(MTLCounterDontSample, |i| i as _),
                        )
                    };
                }

                raw.computeCommandEncoderWithDescriptor(&descriptor)
                    .unwrap()
            };

            if let Some(label) = desc.label {
                encoder.setLabel(Some(&NSString::from_str(label)));
            }

            self.state.compute = Some(encoder.to_owned());
        });
    }
    unsafe fn end_compute_pass(&mut self) {
        self.state.compute.take().unwrap().endEncoding();
    }

    unsafe fn set_compute_pipeline(&mut self, pipeline: &super::ComputePipeline) {
        let previous_sizes =
            core::mem::take(&mut self.state.stage_infos.cs.work_group_memory_sizes);
        self.state.stage_infos.cs.assign_from(&pipeline.cs_info);

        let encoder = self.state.compute.as_ref().unwrap();
        encoder.setComputePipelineState(&pipeline.raw);

        if let Some((index, sizes)) = self
            .state
            .make_sizes_buffer_update(naga::ShaderStage::Compute, &mut self.temp.binding_sizes)
        {
            unsafe {
                encoder.setBytes_length_atIndex(
                    NonNull::new(sizes.as_ptr().cast_mut().cast()).unwrap(),
                    sizes.len() * WORD_SIZE,
                    index as _,
                )
            };
        }

        // update the threadgroup memory sizes
        for (i, current_size) in self
            .state
            .stage_infos
            .cs
            .work_group_memory_sizes
            .iter_mut()
            .enumerate()
        {
            let prev_size = if i < previous_sizes.len() {
                previous_sizes[i]
            } else {
                u32::MAX
            };
            let size: u32 = current_size.next_multiple_of(16);
            *current_size = size;
            if size != prev_size {
                unsafe { encoder.setThreadgroupMemoryLength_atIndex(size as _, i) };
            }
        }
    }

    unsafe fn dispatch(&mut self, count: [u32; 3]) {
        if count[0] > 0 && count[1] > 0 && count[2] > 0 {
            let encoder = self.state.compute.as_ref().unwrap();
            let raw_count = MTLSize {
                width: count[0] as usize,
                height: count[1] as usize,
                depth: count[2] as usize,
            };
            encoder.dispatchThreadgroups_threadsPerThreadgroup(
                raw_count,
                self.state.stage_infos.cs.raw_wg_size,
            );
        }
    }

    unsafe fn dispatch_indirect(&mut self, buffer: &super::Buffer, offset: wgt::BufferAddress) {
        let encoder = self.state.compute.as_ref().unwrap();
        unsafe {
            encoder
                .dispatchThreadgroupsWithIndirectBuffer_indirectBufferOffset_threadsPerThreadgroup(
                    &buffer.raw,
                    offset as usize,
                    self.state.stage_infos.cs.raw_wg_size,
                )
        };
    }

    unsafe fn build_acceleration_structures<'a, T>(
        &mut self,
        _descriptor_count: u32,
        descriptors: T,
    ) where
        super::Api: 'a,
        T: IntoIterator<
            Item = crate::BuildAccelerationStructureDescriptor<
                'a,
                super::Buffer,
                super::AccelerationStructure,
            >,
        >,
    {
        let command_encoder = self.enter_acceleration_structure_builder();
        for descriptor in descriptors {
            let acceleration_structure_descriptor =
                conv::map_acceleration_structure_descriptor(descriptor.entries, descriptor.flags);
            match descriptor.mode {
                crate::AccelerationStructureBuildMode::Build => {
                    command_encoder
                        .buildAccelerationStructure_descriptor_scratchBuffer_scratchBufferOffset(
                            &descriptor.destination_acceleration_structure.raw,
                            &acceleration_structure_descriptor,
                            &descriptor.scratch_buffer.raw,
                            descriptor.scratch_buffer_offset as usize,
                        );
                }
                crate::AccelerationStructureBuildMode::Update => unsafe {
                    command_encoder.refitAccelerationStructure_descriptor_destination_scratchBuffer_scratchBufferOffset(
                        &descriptor.source_acceleration_structure.unwrap().raw,
                        &acceleration_structure_descriptor,
                        Some(&descriptor.destination_acceleration_structure.raw),
                        Some(&descriptor.scratch_buffer.raw),
                        descriptor.scratch_buffer_offset as usize,
                    );
                },
            }
        }
    }

    unsafe fn place_acceleration_structure_barrier(
        &mut self,
        _barriers: crate::AccelerationStructureBarrier,
    ) {
    }

    unsafe fn read_acceleration_structure_compact_size(
        &mut self,
        acceleration_structure: &super::AccelerationStructure,
        buffer: &super::Buffer,
    ) {
        let command_encoder = self.enter_acceleration_structure_builder();
        command_encoder.writeCompactedAccelerationStructureSize_toBuffer_offset(
            &acceleration_structure.raw,
            &buffer.raw,
            0,
        );
    }

    unsafe fn set_acceleration_structure_dependencies(
        command_buffers: &[&super::CommandBuffer],
        dependencies: &[&super::AccelerationStructure],
    ) {
        let Some(first_command_buffer) = command_buffers.first() else {
            return;
        };
        let desc = MTLResidencySetDescriptor::new();
        desc.setLabel(first_command_buffer.raw.label().as_deref());
        let residency_set = first_command_buffer
            .raw
            .device()
            .newResidencySetWithDescriptor_error(&desc)
            .unwrap();
        for command_buffer in command_buffers {
            command_buffer.raw.useResidencySet(&residency_set);
        }
        for dependency in dependencies {
            residency_set.addAllocation(ProtocolObject::from_ref(&*dependency.raw));
        }
        residency_set.commit();
    }
}

impl Drop for super::CommandEncoder {
    fn drop(&mut self) {
        // Metal raises an assert when a MTLCommandEncoder is deallocated without a call
        // to endEncoding. This isn't documented in the general case at
        // https://developer.apple.com/documentation/metal/mtlcommandencoder, but for the
        // more-specific MTLComputeCommandEncoder it is stated as a requirement at
        // https://developer.apple.com/documentation/metal/mtlcomputecommandencoder. It
        // appears to be a requirement for all MTLCommandEncoder objects. Failing to call
        // endEncoding causes a crash with the message 'Command encoder released without
        // endEncoding'. To prevent this, we explicitiy call discard_encoding, which
        // calls endEncoding on any still-held MTLCommandEncoders.
        unsafe {
            self.discard_encoding();
        }
        self.counters.command_encoders.sub(1);
    }
}

impl Drop for super::CommandBuffer {
    fn drop(&mut self) {
        // `command_buffer_created_not_submitted` is usually decremented when the command
        // buffer is submitted. But if we're dropping a command buffer that was never
        // submitted, we need to decrement the count here.
        let status = self.raw.status();
        if status == MTLCommandBufferStatus::NotEnqueued
            || status == MTLCommandBufferStatus::Enqueued
        {
            let previous = self
                .queue_shared
                .command_buffer_created_not_submitted
                .fetch_sub(1, atomic::Ordering::AcqRel);
            debug_assert!(previous > 0);
        }
    }
}
