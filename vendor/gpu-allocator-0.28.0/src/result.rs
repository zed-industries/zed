use alloc::string::String;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AllocationError {
    #[error("Out of memory")]
    OutOfMemory,
    #[error("Failed to map memory: {0}")]
    FailedToMap(String),
    #[error("No compatible memory type available")]
    NoCompatibleMemoryTypeFound,
    #[error("Invalid AllocationCreateDesc")]
    InvalidAllocationCreateDesc,
    #[error("Invalid AllocatorCreateDesc {0}")]
    InvalidAllocatorCreateDesc(String),
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("Initial `BARRIER_LAYOUT` needs at least `Device10`")]
    BarrierLayoutNeedsDevice10,
    #[error("Castable formats require enhanced barriers")]
    CastableFormatsRequiresEnhancedBarriers,
    #[error("Castable formats require at least `Device12`")]
    CastableFormatsRequiresAtLeastDevice12,
}

pub type Result<V, E = AllocationError> = ::core::result::Result<V, E>;
