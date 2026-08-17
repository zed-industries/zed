use crate::*;

/// Handle to a binding group.
///
/// A `BindGroup` represents the set of resources bound to the bindings described by a
/// [`BindGroupLayout`]. It can be created with [`Device::create_bind_group`]. A `BindGroup` can
/// be bound to a particular [`RenderPass`] with [`RenderPass::set_bind_group`], or to a
/// [`ComputePass`] with [`ComputePass::set_bind_group`].
///
/// Corresponds to [WebGPU `GPUBindGroup`](https://gpuweb.github.io/gpuweb/#gpubindgroup).
#[derive(Debug, Clone)]
pub struct BindGroup {
    pub(crate) inner: dispatch::DispatchBindGroup,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(BindGroup: Send, Sync);

crate::cmp::impl_eq_ord_hash_proxy!(BindGroup => .inner);

impl BindGroup {
    #[cfg(custom)]
    /// Returns custom implementation of BindGroup (if custom backend and is internally T)
    pub fn as_custom<T: custom::BindGroupInterface>(&self) -> Option<&T> {
        self.inner.as_custom()
    }
}

/// Resource to be bound by a [`BindGroup`] for use with a pipeline.
///
/// The pipeline’s [`BindGroupLayout`] must contain a matching [`BindingType`].
///
/// Corresponds to [WebGPU `GPUBindingResource`](
/// https://gpuweb.github.io/gpuweb/#typedefdef-gpubindingresource).
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum BindingResource<'a> {
    /// Binding is backed by a buffer.
    ///
    /// Corresponds to [`wgt::BufferBindingType::Uniform`] and [`wgt::BufferBindingType::Storage`]
    /// with [`BindGroupLayoutEntry::count`] set to None.
    Buffer(BufferBinding<'a>),
    /// Binding is backed by an array of buffers.
    ///
    /// [`Features::BUFFER_BINDING_ARRAY`] must be supported to use this feature.
    ///
    /// Corresponds to [`wgt::BufferBindingType::Uniform`] and [`wgt::BufferBindingType::Storage`]
    /// with [`BindGroupLayoutEntry::count`] set to Some.
    BufferArray(&'a [BufferBinding<'a>]),
    /// Binding is a sampler.
    ///
    /// Corresponds to [`wgt::BindingType::Sampler`] with [`BindGroupLayoutEntry::count`] set to None.
    Sampler(&'a Sampler),
    /// Binding is backed by an array of samplers.
    ///
    /// [`Features::TEXTURE_BINDING_ARRAY`] must be supported to use this feature.
    ///
    /// Corresponds to [`wgt::BindingType::Sampler`] with [`BindGroupLayoutEntry::count`] set
    /// to Some.
    SamplerArray(&'a [&'a Sampler]),
    /// Binding is backed by a texture.
    ///
    /// Corresponds to [`wgt::BindingType::Texture`] and [`wgt::BindingType::StorageTexture`] with
    /// [`BindGroupLayoutEntry::count`] set to None.
    TextureView(&'a TextureView),
    /// Binding is backed by an array of textures.
    ///
    /// [`Features::TEXTURE_BINDING_ARRAY`] must be supported to use this feature.
    ///
    /// Corresponds to [`wgt::BindingType::Texture`] and [`wgt::BindingType::StorageTexture`] with
    /// [`BindGroupLayoutEntry::count`] set to Some.
    TextureViewArray(&'a [&'a TextureView]),
    /// Binding is backed by a top level acceleration structure
    ///
    /// Corresponds to [`wgt::BindingType::AccelerationStructure`] with [`BindGroupLayoutEntry::count`] set to None.
    ///
    /// # Validation
    /// When using (e.g. with `set_bind_group`) a bind group that has been created with one or more of this binding
    /// resource certain checks take place.
    /// - TLAS must have been built, if not a validation error is generated
    /// - All BLASes that were built into the TLAS must be built before the TLAS, if this was not satisfied and TLAS was
    ///   built using `build_acceleration_structures` a validation error is generated otherwise this is a part of the
    ///   safety section of `build_acceleration_structures_unsafe_tlas` and so undefined behavior occurs.
    AccelerationStructure(&'a Tlas),
    /// Binding is backed by an array of top level acceleration structures.
    ///
    /// Corresponds to [`wgt::BindingType::AccelerationStructure`] with [`BindGroupLayoutEntry::count`] set to Some.
    ///
    /// # Validation
    /// The same validation rules apply as for [`BindingResource::AccelerationStructure`], for each element.
    ///
    /// Note: backend support may vary; this is primarily intended for native backends.
    AccelerationStructureArray(&'a [&'a Tlas]),
    /// Binding is backed by an external texture.
    ///
    /// [`Features::EXTERNAL_TEXTURE`] must be supported to use this feature.
    ///
    /// Corresponds to [`wgt::BindingType::ExternalTexture`].
    ExternalTexture(&'a ExternalTexture),
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(BindingResource<'_>: Send, Sync);

/// Describes the segment of a buffer to bind.
///
/// Corresponds to [WebGPU `GPUBufferBinding`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpubufferbinding).
#[derive(Clone, Debug)]
pub struct BufferBinding<'a> {
    /// The buffer to bind.
    pub buffer: &'a Buffer,

    /// Base offset of the buffer, in bytes.
    ///
    /// If the [`has_dynamic_offset`] field of this buffer's layout entry is
    /// `true`, the offset here will be added to the dynamic offset passed to
    /// [`RenderPass::set_bind_group`] or [`ComputePass::set_bind_group`].
    ///
    /// If the buffer was created with [`BufferUsages::UNIFORM`], then this
    /// offset must be a multiple of
    /// [`Limits::min_uniform_buffer_offset_alignment`].
    ///
    /// If the buffer was created with [`BufferUsages::STORAGE`], then this
    /// offset must be a multiple of
    /// [`Limits::min_storage_buffer_offset_alignment`].
    ///
    /// [`has_dynamic_offset`]: BindingType::Buffer::has_dynamic_offset
    pub offset: BufferAddress,

    /// Size of the binding in bytes, or `None` for using the rest of the buffer.
    pub size: Option<BufferSize>,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(BufferBinding<'_>: Send, Sync);

/// An element of a [`BindGroupDescriptor`], consisting of a bindable resource
/// and the slot to bind it to.
///
/// Corresponds to [WebGPU `GPUBindGroupEntry`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpubindgroupentry).
#[derive(Clone, Debug)]
pub struct BindGroupEntry<'a> {
    /// Slot for which binding provides resource. Corresponds to an entry of the same
    /// binding index in the [`BindGroupLayoutDescriptor`].
    pub binding: u32,
    /// Resource to attach to the binding
    pub resource: BindingResource<'a>,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(BindGroupEntry<'_>: Send, Sync);

/// Describes a group of bindings and the resources to be bound.
///
/// For use with [`Device::create_bind_group`].
///
/// Corresponds to [WebGPU `GPUBindGroupDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpubindgroupdescriptor).
#[derive(Clone, Debug)]
pub struct BindGroupDescriptor<'a> {
    /// Debug label of the bind group. This will show up in graphics debuggers for easy identification.
    pub label: Label<'a>,
    /// The [`BindGroupLayout`] that corresponds to this bind group.
    pub layout: &'a BindGroupLayout,
    /// The resources to bind to this bind group.
    pub entries: &'a [BindGroupEntry<'a>],
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(BindGroupDescriptor<'_>: Send, Sync);
