#[cfg(any(feature = "serde", test))]
use serde::{Deserialize, Serialize};

#[cfg(doc)]
use crate::{DownlevelFlags, COPY_BUFFER_ALIGNMENT};

/// Describes a [`Buffer`](../wgpu/struct.Buffer.html).
///
/// Corresponds to [WebGPU `GPUBufferDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpubufferdescriptor).
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BufferDescriptor<L> {
    /// Debug label of a buffer. This will show up in graphics debuggers for easy identification.
    pub label: L,
    /// Size of a buffer, in bytes.
    pub size: crate::BufferAddress,
    /// Usages of a buffer. If the buffer is used in any way that isn't specified here, the operation
    /// will panic.
    ///
    /// Specifying only usages the application will actually perform may increase performance.
    /// Additionally, on the WebGL backend, there are restrictions on [`BufferUsages::INDEX`];
    /// see [`DownlevelFlags::UNRESTRICTED_INDEX_BUFFER`] for more information.
    pub usage: BufferUsages,
    /// Allows a buffer to be mapped immediately after they are made. It does not have to be [`BufferUsages::MAP_READ`] or
    /// [`BufferUsages::MAP_WRITE`], all buffers are allowed to be mapped at creation.
    ///
    /// If this is `true`, [`size`](#structfield.size) must be a multiple of
    /// [`COPY_BUFFER_ALIGNMENT`].
    pub mapped_at_creation: bool,
}

impl<L> BufferDescriptor<L> {
    /// Takes a closure and maps the label of the buffer descriptor into another.
    #[must_use]
    pub fn map_label<K>(&self, fun: impl FnOnce(&L) -> K) -> BufferDescriptor<K> {
        BufferDescriptor {
            label: fun(&self.label),
            size: self.size,
            usage: self.usage,
            mapped_at_creation: self.mapped_at_creation,
        }
    }
}

bitflags::bitflags! {
    /// Different ways that you can use a buffer.
    ///
    /// The usages determine what kind of memory the buffer is allocated from and what
    /// actions the buffer can partake in.
    ///
    /// Specifying only usages the application will actually perform may increase performance.
    /// Additionally, on the WebGL backend, there are restrictions on [`BufferUsages::INDEX`];
    /// see [`DownlevelFlags::UNRESTRICTED_INDEX_BUFFER`] for more information.
    ///
    /// Corresponds to [WebGPU `GPUBufferUsageFlags`](
    /// https://gpuweb.github.io/gpuweb/#typedefdef-gpubufferusageflags).
    #[repr(transparent)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct BufferUsages: u32 {
        /// Allow a buffer to be mapped for reading using [`Buffer::map_async`] + [`Buffer::get_mapped_range`].
        /// This does not include creating a buffer with [`BufferDescriptor::mapped_at_creation`] set.
        ///
        /// If [`Features::MAPPABLE_PRIMARY_BUFFERS`] isn't enabled, the only other usage a buffer
        /// may have is COPY_DST.
        const MAP_READ = 1 << 0;
        /// Allow a buffer to be mapped for writing using [`Buffer::map_async`] + [`Buffer::get_mapped_range_mut`].
        /// This does not include creating a buffer with [`BufferDescriptor::mapped_at_creation`] set.
        ///
        /// If [`Features::MAPPABLE_PRIMARY_BUFFERS`] feature isn't enabled, the only other usage a buffer
        /// may have is COPY_SRC.
        const MAP_WRITE = 1 << 1;
        /// Allow a buffer to be the source buffer for a [`CommandEncoder::copy_buffer_to_buffer`] or [`CommandEncoder::copy_buffer_to_texture`]
        /// operation.
        const COPY_SRC = 1 << 2;
        /// Allow a buffer to be the destination buffer for a [`CommandEncoder::copy_buffer_to_buffer`], [`CommandEncoder::copy_texture_to_buffer`],
        /// [`CommandEncoder::clear_buffer`] or [`Queue::write_buffer`] operation.
        const COPY_DST = 1 << 3;
        /// Allow a buffer to be the index buffer in a draw operation.
        const INDEX = 1 << 4;
        /// Allow a buffer to be the vertex buffer in a draw operation.
        const VERTEX = 1 << 5;
        /// Allow a buffer to be a [`BufferBindingType::Uniform`] inside a bind group.
        const UNIFORM = 1 << 6;
        /// Allow a buffer to be a [`BufferBindingType::Storage`] inside a bind group.
        const STORAGE = 1 << 7;
        /// Allow a buffer to be the indirect buffer in an indirect draw call.
        const INDIRECT = 1 << 8;
        /// Allow a buffer to be the destination buffer for a [`CommandEncoder::resolve_query_set`] operation.
        const QUERY_RESOLVE = 1 << 9;
        /// Allows a buffer to be used as input for a bottom level acceleration structure build
        const BLAS_INPUT = 1 << 10;
        /// Allows a buffer to be used as input for a top level acceleration structure build
        const TLAS_INPUT = 1 << 11;
    }
}

bitflags::bitflags! {
    /// Similar to `BufferUsages`, but used only for `CommandEncoder::transition_resources`.
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct BufferUses: u16 {
        /// The argument to a read-only mapping.
        const MAP_READ = 1 << 0;
        /// The argument to a write-only mapping.
        const MAP_WRITE = 1 << 1;
        /// The source of a hardware copy.
        /// cbindgen:ignore
        const COPY_SRC = 1 << 2;
        /// The destination of a hardware copy.
        /// cbindgen:ignore
        const COPY_DST = 1 << 3;
        /// The index buffer used for drawing.
        const INDEX = 1 << 4;
        /// A vertex buffer used for drawing.
        const VERTEX = 1 << 5;
        /// A uniform buffer bound in a bind group.
        const UNIFORM = 1 << 6;
        /// A read-only storage buffer used in a bind group.
        /// cbindgen:ignore
        const STORAGE_READ_ONLY = 1 << 7;
        /// A read-write buffer used in a bind group.
        /// cbindgen:ignore
        const STORAGE_READ_WRITE = 1 << 8;
        /// The indirect or count buffer in a indirect draw or dispatch.
        const INDIRECT = 1 << 9;
        /// A buffer used to store query results.
        const QUERY_RESOLVE = 1 << 10;
        /// Buffer used for acceleration structure building.
        const ACCELERATION_STRUCTURE_SCRATCH = 1 << 11;
        /// Buffer used for bottom level acceleration structure building.
        const BOTTOM_LEVEL_ACCELERATION_STRUCTURE_INPUT = 1 << 12;
        /// Buffer used for top level acceleration structure building.
        const TOP_LEVEL_ACCELERATION_STRUCTURE_INPUT = 1 << 13;
        /// A buffer used to store the compacted size of an acceleration structure
        const ACCELERATION_STRUCTURE_QUERY = 1 << 14;
        /// The combination of states that a buffer may be in _at the same time_.
        const INCLUSIVE = Self::MAP_READ.bits() | Self::COPY_SRC.bits() |
            Self::INDEX.bits() | Self::VERTEX.bits() | Self::UNIFORM.bits() |
            Self::STORAGE_READ_ONLY.bits() | Self::INDIRECT.bits() | Self::BOTTOM_LEVEL_ACCELERATION_STRUCTURE_INPUT.bits() | Self::TOP_LEVEL_ACCELERATION_STRUCTURE_INPUT.bits();
        /// The combination of states that a buffer must exclusively be in.
        const EXCLUSIVE = Self::MAP_WRITE.bits() | Self::COPY_DST.bits() | Self::STORAGE_READ_WRITE.bits() | Self::ACCELERATION_STRUCTURE_SCRATCH.bits();
    }
}

/// A buffer transition for use with `CommandEncoder::transition_resources`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BufferTransition<T> {
    /// The buffer to transition.
    pub buffer: T,
    /// The new state to transition to.
    pub state: BufferUses,
}
