use alloc::sync::Arc;

use crate::id;

/// Describes the writing of timestamp values in a render or compute pass.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PassTimestampWrites<QS = id::QuerySetId> {
    /// The query set to write the timestamps to.
    pub query_set: QS,
    /// The index of the query set at which a start timestamp of this pass is written, if any.
    pub beginning_of_pass_write_index: Option<u32>,
    /// The index of the query set at which an end timestamp of this pass is written, if any.
    pub end_of_pass_write_index: Option<u32>,
}

/// cbindgen:ignore
pub type ArcPassTimestampWrites = PassTimestampWrites<Arc<crate::resource::QuerySet>>;
