use crate::reflect::OneofDescriptor;
use crate::Oneof;

/// Implemented by all oneof types when lite runtime is not enabled.
pub trait OneofFull: Oneof {
    /// Descriptor object for this oneof.
    fn descriptor() -> OneofDescriptor;
}
