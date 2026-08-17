//! Unique IDs for modules in the runtime.

use core::num::NonZeroU64;

/// A unique identifier (within an engine or similar) for a compiled
/// module.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompiledModuleId(NonZeroU64);

impl CompiledModuleId {
    /// Allocates a new ID which will be unique within this process.
    pub fn new() -> Self {
        // As an implementation detail this is implemented on the same
        // allocator as stores. It's ok if there are "holes" in the store id
        // space as it's not required to be compact, it's just used for
        // uniqueness.
        CompiledModuleId(crate::store::StoreId::allocate().as_raw())
    }
}
