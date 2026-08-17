use crate::MTLResourceID;

impl MTLResourceID {
    /// Construct a `MTLResourceID` from an ID previously gotten via `to_raw`.
    ///
    /// # Safety
    ///
    /// The documentation for `MTLResourceID` says:
    ///
    /// > A MTLResourceID represents a specific GPU resource, mutating this
    /// > handle is undefined unless the mutation results in the value
    /// > equalling an already existing handle of the same resource type.
    ///
    /// So we've tentatively marked this method as `unsafe`, with the safety
    /// requirement that the ID must be valid, i.e. have previously come from
    /// [`to_raw`][Self::to_raw] or similar.
    ///
    /// If you disagree with this assessment, feel free to open an issue!
    pub const unsafe fn from_raw(id: u64) -> Self {
        Self { _impl: id }
    }

    /// Get the underlying data of the ID.
    ///
    /// May be useful for FFI purposes.
    pub const fn to_raw(self) -> u64 {
        self._impl
    }
}
