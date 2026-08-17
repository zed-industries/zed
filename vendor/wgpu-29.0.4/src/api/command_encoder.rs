use alloc::sync::Arc;
use core::ops::Range;

use crate::{
    api::{
        blas::BlasBuildEntry, impl_deferred_command_buffer_actions, tlas::Tlas,
        SharedDeferredCommandBufferActions,
    },
    *,
};

/// Encodes a series of GPU operations.
///
/// A command encoder can record [`RenderPass`]es, [`ComputePass`]es,
/// and transfer operations between driver-managed resources like [`Buffer`]s and [`Texture`]s.
///
/// When finished recording, call [`CommandEncoder::finish`] to obtain a [`CommandBuffer`] which may
/// be submitted for execution.
///
/// Corresponds to [WebGPU `GPUCommandEncoder`](https://gpuweb.github.io/gpuweb/#command-encoder).
#[derive(Debug)]
pub struct CommandEncoder {
    pub(crate) inner: dispatch::DispatchCommandEncoder,
    pub(crate) actions: SharedDeferredCommandBufferActions,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(CommandEncoder: Send, Sync);

crate::cmp::impl_eq_ord_hash_proxy!(CommandEncoder => .inner);

/// Describes a [`CommandEncoder`].
///
/// For use with [`Device::create_command_encoder`].
///
/// Corresponds to [WebGPU `GPUCommandEncoderDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpucommandencoderdescriptor).
pub type CommandEncoderDescriptor<'a> = wgt::CommandEncoderDescriptor<Label<'a>>;
static_assertions::assert_impl_all!(CommandEncoderDescriptor<'_>: Send, Sync);

pub use wgt::TexelCopyBufferInfo as TexelCopyBufferInfoBase;
/// View of a buffer which can be used to copy to/from a texture.
///
/// Corresponds to [WebGPU `GPUTexelCopyBufferInfo`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpuimagecopybuffer).
pub type TexelCopyBufferInfo<'a> = TexelCopyBufferInfoBase<&'a Buffer>;
#[cfg(send_sync)]
static_assertions::assert_impl_all!(TexelCopyBufferInfo<'_>: Send, Sync);

pub use wgt::TexelCopyTextureInfo as TexelCopyTextureInfoBase;
/// View of a texture which can be used to copy to/from a buffer/texture.
///
/// Corresponds to [WebGPU `GPUTexelCopyTextureInfo`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpuimagecopytexture).
pub type TexelCopyTextureInfo<'a> = TexelCopyTextureInfoBase<&'a Texture>;
#[cfg(send_sync)]
static_assertions::assert_impl_all!(TexelCopyTextureInfo<'_>: Send, Sync);

impl CommandEncoder {
    /// Finishes recording and returns a [`CommandBuffer`] that can be submitted for execution.
    pub fn finish(self) -> CommandBuffer {
        let Self { mut inner, actions } = self;
        let buffer = inner.finish();
        CommandBuffer { buffer, actions }
    }

    /// Begins recording of a render pass.
    ///
    /// This function returns a [`RenderPass`] object which records a single render pass.
    ///
    /// As long as the returned  [`RenderPass`] has not ended,
    /// any mutating operation on this command encoder causes an error and invalidates it.
    /// Note that the `'encoder` lifetime relationship protects against this,
    /// but it is possible to opt out of it by calling [`RenderPass::forget_lifetime`].
    /// This can be useful for runtime handling of the encoder->pass
    /// dependency e.g. when pass and encoder are stored in the same data structure.
    pub fn begin_render_pass<'encoder>(
        &'encoder mut self,
        desc: &RenderPassDescriptor<'_>,
    ) -> RenderPass<'encoder> {
        let rpass = self.inner.begin_render_pass(desc);
        RenderPass {
            inner: rpass,
            actions: Arc::clone(&self.actions),
            _encoder_guard: api::PhantomDrop::default(),
        }
    }

    /// Begins recording of a compute pass.
    ///
    /// This function returns a [`ComputePass`] object which records a single compute pass.
    ///
    /// As long as the returned  [`ComputePass`] has not ended,
    /// any mutating operation on this command encoder causes an error and invalidates it.
    /// Note that the `'encoder` lifetime relationship protects against this,
    /// but it is possible to opt out of it by calling [`ComputePass::forget_lifetime`].
    /// This can be useful for runtime handling of the encoder->pass
    /// dependency e.g. when pass and encoder are stored in the same data structure.
    pub fn begin_compute_pass<'encoder>(
        &'encoder mut self,
        desc: &ComputePassDescriptor<'_>,
    ) -> ComputePass<'encoder> {
        let cpass = self.inner.begin_compute_pass(desc);
        ComputePass {
            inner: cpass,
            actions: Arc::clone(&self.actions),
            _encoder_guard: api::PhantomDrop::default(),
        }
    }

    /// Copy data from one buffer to another.
    ///
    /// # Panics
    ///
    /// - Buffer offsets or copy size not a multiple of [`COPY_BUFFER_ALIGNMENT`].
    /// - Copy would overrun buffer.
    /// - Copy within the same buffer.
    pub fn copy_buffer_to_buffer(
        &mut self,
        source: &Buffer,
        source_offset: BufferAddress,
        destination: &Buffer,
        destination_offset: BufferAddress,
        copy_size: impl Into<Option<BufferAddress>>,
    ) {
        self.inner.copy_buffer_to_buffer(
            &source.inner,
            source_offset,
            &destination.inner,
            destination_offset,
            copy_size.into(),
        );
    }

    /// Copy data from a buffer to a texture.
    pub fn copy_buffer_to_texture(
        &mut self,
        source: TexelCopyBufferInfo<'_>,
        destination: TexelCopyTextureInfo<'_>,
        copy_size: Extent3d,
    ) {
        self.inner
            .copy_buffer_to_texture(source, destination, copy_size);
    }

    /// Copy data from a texture to a buffer.
    pub fn copy_texture_to_buffer(
        &mut self,
        source: TexelCopyTextureInfo<'_>,
        destination: TexelCopyBufferInfo<'_>,
        copy_size: Extent3d,
    ) {
        self.inner
            .copy_texture_to_buffer(source, destination, copy_size);
    }

    /// Copy data from one texture to another.
    ///
    /// # Panics
    ///
    /// - Textures are not the same type
    /// - If a depth texture, or a multisampled texture, the entire texture must be copied
    /// - Copy would overrun either texture
    pub fn copy_texture_to_texture(
        &mut self,
        source: TexelCopyTextureInfo<'_>,
        destination: TexelCopyTextureInfo<'_>,
        copy_size: Extent3d,
    ) {
        self.inner
            .copy_texture_to_texture(source, destination, copy_size);
    }

    /// Clears texture to zero.
    ///
    /// Note that unlike with clear_buffer, `COPY_DST` usage is not required.
    ///
    /// # Implementation notes
    ///
    /// - implemented either via buffer copies and render/depth target clear, path depends on texture usages
    /// - behaves like texture zero init, but is performed immediately (clearing is *not* delayed via marking it as uninitialized)
    ///
    /// # Panics
    ///
    /// - `CLEAR_TEXTURE` extension not enabled
    /// - Range is out of bounds
    pub fn clear_texture(&mut self, texture: &Texture, subresource_range: &ImageSubresourceRange) {
        self.inner.clear_texture(&texture.inner, subresource_range);
    }

    /// Clears buffer to zero.
    ///
    /// # Panics
    ///
    /// - Buffer does not have `COPY_DST` usage.
    /// - Range is out of bounds
    pub fn clear_buffer(
        &mut self,
        buffer: &Buffer,
        offset: BufferAddress,
        size: Option<BufferAddress>,
    ) {
        self.inner.clear_buffer(&buffer.inner, offset, size);
    }

    /// Inserts debug marker.
    pub fn insert_debug_marker(&mut self, label: &str) {
        self.inner.insert_debug_marker(label);
    }

    /// Start record commands and group it into debug marker group.
    pub fn push_debug_group(&mut self, label: &str) {
        self.inner.push_debug_group(label);
    }

    /// Stops command recording and creates debug group.
    pub fn pop_debug_group(&mut self) {
        self.inner.pop_debug_group();
    }

    /// Copies query results stored in `query_set` into `destination` so that they can be read
    /// by compute shaders or buffer operations.
    ///
    /// * `query_range` is the range of query result indices to copy from `query_set`.
    ///   Occlusion and timestamp queries occupy 1 result index each;
    ///   for pipeline statistics queries, see [`PipelineStatisticsTypes`].
    /// * `destination_offset` is the offset within `destination` to start writing at.
    ///   It must be a multiple of [`QUERY_RESOLVE_BUFFER_ALIGNMENT`].
    ///
    /// The length of the data written to `destination` will be 8 bytes ([`QUERY_SIZE`])
    /// times the number of elements in `query_range`.
    ///
    /// For further information about using queries, see [`QuerySet`].
    pub fn resolve_query_set(
        &mut self,
        query_set: &QuerySet,
        query_range: Range<u32>,
        destination: &Buffer,
        destination_offset: BufferAddress,
    ) {
        self.inner.resolve_query_set(
            &query_set.inner,
            query_range.start,
            query_range.end - query_range.start,
            &destination.inner,
            destination_offset,
        );
    }

    impl_deferred_command_buffer_actions!();

    /// Get the [`wgpu_hal`] command encoder from this `CommandEncoder`.
    ///
    /// The returned command encoder will be ready to record onto.
    ///
    /// # Errors
    ///
    /// This method will pass in [`None`] if:
    /// - The encoder is not from the backend specified by `A`.
    /// - The encoder is from the `webgpu` or `custom` backend.
    ///
    /// # Types
    ///
    /// The callback argument depends on the backend:
    ///
    #[doc = crate::macros::hal_type_vulkan!("CommandEncoder")]
    #[doc = crate::macros::hal_type_metal!("CommandEncoder")]
    #[doc = crate::macros::hal_type_dx12!("CommandEncoder")]
    #[doc = crate::macros::hal_type_gles!("CommandEncoder")]
    ///
    /// # Safety
    ///
    /// - The raw handle obtained from the `A::CommandEncoder` must not be manually destroyed.
    /// - You must not end the command buffer; wgpu will do it when you call finish.
    /// - The wgpu command encoder must not be interacted with in any way while recording is
    ///   happening to the wgpu_hal or backend command encoder.
    #[cfg(wgpu_core)]
    pub unsafe fn as_hal_mut<A: hal::Api, F: FnOnce(Option<&mut A::CommandEncoder>) -> R, R>(
        &mut self,
        hal_command_encoder_callback: F,
    ) -> R {
        if let Some(encoder) = self.inner.as_core_mut_opt() {
            unsafe {
                encoder
                    .context
                    .command_encoder_as_hal_mut::<A, F, R>(encoder, hal_command_encoder_callback)
            }
        } else {
            hal_command_encoder_callback(None)
        }
    }

    #[cfg(custom)]
    /// Returns custom implementation of CommandEncoder (if custom backend and is internally T)
    pub fn as_custom<T: custom::CommandEncoderInterface>(&self) -> Option<&T> {
        self.inner.as_custom()
    }
}

/// [`Features::TIMESTAMP_QUERY_INSIDE_ENCODERS`] must be enabled on the device in order to call these functions.
impl CommandEncoder {
    /// Issue a timestamp command at this point in the queue.
    /// The timestamp will be written to the specified query set, at the specified index.
    ///
    /// Must be multiplied by [`Queue::get_timestamp_period`] to get
    /// the value in nanoseconds. Absolute values have no meaning,
    /// but timestamps can be subtracted to get the time it takes
    /// for a string of operations to complete.
    ///
    /// Attention: Since commands within a command recorder may be reordered,
    /// there is no strict guarantee that timestamps are taken after all commands
    /// recorded so far and all before all commands recorded after.
    /// This may depend both on the backend and the driver.
    pub fn write_timestamp(&mut self, query_set: &QuerySet, query_index: u32) {
        self.inner.write_timestamp(&query_set.inner, query_index);
    }
}

/// [`Features::EXPERIMENTAL_RAY_QUERY`] must be enabled on the device in order to call these functions.
impl CommandEncoder {
    /// When encoding the acceleration structure build with the raw Hal encoder
    /// (obtained from [`CommandEncoder::as_hal_mut`]), this function marks the
    /// acceleration structures as having been built.
    ///
    /// This function must only be used with the raw encoder API. When using the
    /// wgpu encoding API, acceleration structure build is tracked automatically.
    ///
    /// # Panics
    ///
    /// - If the encoder is being used with the wgpu encoding API.
    ///
    /// # Safety
    ///
    /// - All acceleration structures must have been build in this command encoder.
    /// - All BLASes inputted must have been built before all TLASes that were inputted here and
    ///   which use them.
    pub unsafe fn mark_acceleration_structures_built<'a>(
        &self,
        blas: impl IntoIterator<Item = &'a Blas>,
        tlas: impl IntoIterator<Item = &'a Tlas>,
    ) {
        self.inner
            .mark_acceleration_structures_built(&mut blas.into_iter(), &mut tlas.into_iter())
    }
    /// Build bottom and top level acceleration structures.
    ///
    /// Builds the BLASes then the TLASes, but does ***not*** build the BLASes into the TLASes,
    /// that must be done by setting a TLAS instance in the TLAS package to one that contains the BLAS (and with an appropriate transform)
    ///
    /// # Validation
    ///
    /// - blas: Iterator of bottom level acceleration structure entries to build.
    ///   For each entry, the provided size descriptor must be strictly smaller or equal to the descriptor given at BLAS creation, this means:
    ///   - Less or equal number of geometries
    ///   - Same kind of geometry (with index buffer or without) (same vertex/index format)
    ///   - Same flags
    ///   - Less or equal number of vertices
    ///   - Less or equal number of indices (if applicable)
    /// - tlas: iterator of top level acceleration structure packages to build
    ///   For each entry:
    ///   - Each BLAS in each TLAS instance must have been being built in the current call or in a previous call to `build_acceleration_structures` or `build_acceleration_structures_unsafe_tlas`
    ///   - The number of TLAS instances must be less than or equal to the max number of tlas instances when creating (if creating a package with `TlasPackage::new()` this is already satisfied)
    ///
    /// If the device the command encoder is created from does not have [Features::EXPERIMENTAL_RAY_QUERY] enabled then a validation error is generated
    ///
    /// A bottom level acceleration structure may be build and used as a reference in a top level acceleration structure in the same invocation of this function.
    ///
    /// # Bind group usage
    ///
    /// When a top level acceleration structure is used in a bind group, some validation takes place:
    ///    - The top level acceleration structure is valid and has been built.
    ///    - All the bottom level acceleration structures referenced by the top level acceleration structure are valid and have been built prior,
    ///      or at same time as the containing top level acceleration structure.
    ///
    /// [Features::EXPERIMENTAL_RAY_QUERY]: wgt::Features::EXPERIMENTAL_RAY_QUERY
    pub fn build_acceleration_structures<'a>(
        &mut self,
        blas: impl IntoIterator<Item = &'a BlasBuildEntry<'a>>,
        tlas: impl IntoIterator<Item = &'a Tlas>,
    ) {
        self.inner
            .build_acceleration_structures(&mut blas.into_iter(), &mut tlas.into_iter());
    }

    /// Transition resources to an underlying hal resource state.
    ///
    /// This is an advanced, native-only API (no-op on web) that has two main use cases:
    ///
    /// # Batching Barriers
    ///
    /// Wgpu does not have a global view of the frame when recording command buffers. When you submit multiple command buffers in a single queue submission, wgpu may need to record and
    /// insert new command buffers (holding 1 or more barrier commands) in between the user-supplied command buffers in order to ensure that resources are transitioned to the correct state
    /// for the start of the next user-supplied command buffer.
    ///
    /// Wgpu does not currently attempt to batch multiple of these generated command buffers/barriers together, which may lead to suboptimal barrier placement.
    ///
    /// Consider the following scenario, where the user does `queue.submit(&[a, b, c])`:
    /// * CommandBuffer A: Use resource X as a render pass attachment
    /// * CommandBuffer B: Use resource Y as a render pass attachment
    /// * CommandBuffer C: Use resources X and Y in a bind group
    ///
    /// At submission time, wgpu will record and insert some new command buffers, resulting in a submission that looks like `queue.submit(&[0, a, 1, b, 2, c])`:
    /// * CommandBuffer 0: Barrier to transition resource X from TextureUses::RESOURCE (from last frame) to TextureUses::COLOR_TARGET
    /// * CommandBuffer A: Use resource X as a render pass attachment
    /// * CommandBuffer 1: Barrier to transition resource Y from TextureUses::RESOURCE (from last frame) to TextureUses::COLOR_TARGET
    /// * CommandBuffer B: Use resource Y as a render pass attachment
    /// * CommandBuffer 2: Barrier to transition resources X and Y from TextureUses::COLOR_TARGET to TextureUses::RESOURCE
    /// * CommandBuffer C: Use resources X and Y in a bind group
    ///
    /// To prevent this, after profiling their app, an advanced user might choose to instead do `queue.submit(&[a, b, c])`:
    /// * CommandBuffer A:
    ///     * Use [`CommandEncoder::transition_resources`] to transition resources X and Y from TextureUses::RESOURCE (from last frame) to TextureUses::COLOR_TARGET
    ///     * Use resource X as a render pass attachment
    /// * CommandBuffer B: Use resource Y as a render pass attachment
    /// * CommandBuffer C:
    ///     * Use [`CommandEncoder::transition_resources`] to transition resources X and Y from TextureUses::COLOR_TARGET to TextureUses::RESOURCE
    ///     * Use resources X and Y in a bind group
    ///
    /// At submission time, wgpu will record and insert some new command buffers, resulting in a submission that looks like `queue.submit(&[0, a, b, 1, c])`:
    /// * CommandBuffer 0: Barrier to transition resources X and Y from TextureUses::RESOURCE (from last frame) to TextureUses::COLOR_TARGET
    /// * CommandBuffer A: Use resource X as a render pass attachment
    /// * CommandBuffer B: Use resource Y as a render pass attachment
    /// * CommandBuffer 1: Barrier to transition resources X and Y from TextureUses::COLOR_TARGET to TextureUses::RESOURCE
    /// * CommandBuffer C: Use resources X and Y in a bind group
    ///
    /// Which eliminates the extra command buffer and barrier between command buffers A and B.
    ///
    /// # Native Interoperability
    ///
    /// A user wanting to interoperate with the underlying native graphics APIs (Vulkan, DirectX12, Metal, etc) can use this API to generate barriers between wgpu commands and
    /// the native API commands, for synchronization and resource state transition purposes.
    pub fn transition_resources<'a>(
        &mut self,
        buffer_transitions: impl Iterator<Item = wgt::BufferTransition<&'a Buffer>>,
        texture_transitions: impl Iterator<Item = wgt::TextureTransition<&'a Texture>>,
    ) {
        self.inner.transition_resources(
            &mut buffer_transitions.map(|t| wgt::BufferTransition {
                buffer: &t.buffer.inner,
                state: t.state,
            }),
            &mut texture_transitions.map(|t| wgt::TextureTransition {
                texture: &t.texture.inner,
                selector: t.selector,
                state: t.state,
            }),
        );
    }
}
