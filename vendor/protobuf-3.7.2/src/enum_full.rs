use crate::reflect::EnumDescriptor;
use crate::reflect::EnumValueDescriptor;
use crate::Enum;

/// Trait is implemented for all enum types if lite runtime is not requested.
///
/// This trait provides access to runtime reflection.
pub trait EnumFull: Enum {
    /// Get enum value descriptor.
    fn descriptor(&self) -> EnumValueDescriptor;

    /// Get enum descriptor by type.
    fn enum_descriptor() -> EnumDescriptor;
}
