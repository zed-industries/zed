use std::fmt;

#[cfg(feature = "bytes")]
use ::bytes::Bytes;

#[cfg(feature = "bytes")]
use crate::chars::Chars;
use crate::reflect::runtime_types::RuntimeTypeBool;
use crate::reflect::runtime_types::RuntimeTypeF32;
use crate::reflect::runtime_types::RuntimeTypeF64;
use crate::reflect::runtime_types::RuntimeTypeI32;
use crate::reflect::runtime_types::RuntimeTypeI64;
use crate::reflect::runtime_types::RuntimeTypeString;
#[cfg(feature = "bytes")]
use crate::reflect::runtime_types::RuntimeTypeTokioBytes;
#[cfg(feature = "bytes")]
use crate::reflect::runtime_types::RuntimeTypeTokioChars;
use crate::reflect::runtime_types::RuntimeTypeTrait;
use crate::reflect::runtime_types::RuntimeTypeU32;
use crate::reflect::runtime_types::RuntimeTypeU64;
use crate::reflect::runtime_types::RuntimeTypeVecU8;

pub(crate) mod value_box;
pub(crate) mod value_ref;

/// Type implemented by all protobuf singular types
/// (primitives, string, messages, enums).
///
/// Used in reflection.
pub trait ProtobufValue: Clone + Default + fmt::Debug + Send + Sync + Sized + 'static {
    /// Actual implementation of type properties.
    type RuntimeType: RuntimeTypeTrait<Value = Self>;
}

impl ProtobufValue for u32 {
    type RuntimeType = RuntimeTypeU32;
}

impl ProtobufValue for u64 {
    type RuntimeType = RuntimeTypeU64;
}

impl ProtobufValue for i32 {
    type RuntimeType = RuntimeTypeI32;
}

impl ProtobufValue for i64 {
    type RuntimeType = RuntimeTypeI64;
}

impl ProtobufValue for f32 {
    type RuntimeType = RuntimeTypeF32;
}

impl ProtobufValue for f64 {
    type RuntimeType = RuntimeTypeF64;
}

impl ProtobufValue for bool {
    type RuntimeType = RuntimeTypeBool;
}

impl ProtobufValue for String {
    type RuntimeType = RuntimeTypeString;
}

impl ProtobufValue for Vec<u8> {
    type RuntimeType = RuntimeTypeVecU8;
}

#[cfg(feature = "bytes")]
impl ProtobufValue for Bytes {
    type RuntimeType = RuntimeTypeTokioBytes;
}

#[cfg(feature = "bytes")]
impl ProtobufValue for Chars {
    type RuntimeType = RuntimeTypeTokioChars;
}

// conflicting implementations, so generated code is used instead
/*
impl<E : ProtobufEnum> ProtobufValue for E {
}

impl<M : Message> ProtobufValue for M {
}
*/
