#[cfg(feature = "serde")]
use crate::command::serde_object_reference_struct;
use crate::command::{ArcReferences, ReferenceType};

#[cfg(feature = "serde")]
use macro_rules_attribute::apply;

/// cbindgen:ignore
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", apply(serde_object_reference_struct))]
pub enum ComputeCommand<R: ReferenceType> {
    SetBindGroup {
        index: u32,
        num_dynamic_offsets: usize,
        bind_group: Option<R::BindGroup>,
    },

    SetPipeline(R::ComputePipeline),

    /// Set a range of immediates to values stored in `immediates_data`.
    SetImmediate {
        /// The byte offset within the immediate data storage to write to. This
        /// must be a multiple of four.
        offset: u32,

        /// The number of bytes to write. This must be a multiple of four.
        size_bytes: u32,

        /// Index in `immediates_data` of the start of the data
        /// to be written.
        ///
        /// Note: this is not a byte offset like `offset`. Rather, it is the
        /// index of the first `u32` element in `immediates_data` to read.
        values_offset: u32,
    },

    Dispatch([u32; 3]),

    DispatchIndirect {
        buffer: R::Buffer,
        offset: wgt::BufferAddress,
    },

    PushDebugGroup {
        color: u32,
        len: usize,
    },

    PopDebugGroup,

    InsertDebugMarker {
        color: u32,
        len: usize,
    },

    WriteTimestamp {
        query_set: R::QuerySet,
        query_index: u32,
    },

    BeginPipelineStatisticsQuery {
        query_set: R::QuerySet,
        query_index: u32,
    },

    EndPipelineStatisticsQuery,
}

/// cbindgen:ignore
pub type ArcComputeCommand = ComputeCommand<ArcReferences>;
