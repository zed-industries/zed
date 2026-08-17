use crate::reflect::RuntimeType;

/// Reflective representation of field type.
pub enum RuntimeFieldType {
    /// Singular field (required, optional for proto2 or singular for proto3)
    Singular(RuntimeType),
    /// Repeated field
    Repeated(RuntimeType),
    /// Map field
    Map(RuntimeType, RuntimeType),
}
