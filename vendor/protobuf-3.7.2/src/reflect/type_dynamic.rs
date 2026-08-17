//! Reflection internals.

use std::marker;

use crate::reflect::runtime_types::RuntimeTypeTrait;
use crate::reflect::types::ProtobufTypeTrait;
use crate::reflect::ProtobufValue;
use crate::reflect::RuntimeType;
use crate::wire_format::WireType;

/// Dynamic version of [`ProtobufType`](crate::reflect::types::ProtobufType).
///
/// This is used internally.
pub(crate) trait _ProtobufTypeDynamic: Send + Sync + 'static {
    /// Wire type for this type.
    fn wire_type(&self) -> WireType;

    /// Get runtime type for this protobuf type.
    fn runtime_type(&self) -> RuntimeType;
}

pub(crate) struct _ProtobufTypeDynamicImpl<T: ProtobufTypeTrait>(pub marker::PhantomData<T>);

impl<T> _ProtobufTypeDynamic for _ProtobufTypeDynamicImpl<T>
where
    T: ProtobufTypeTrait,
    <T as ProtobufTypeTrait>::ProtobufValue: ProtobufValue,
{
    fn wire_type(&self) -> WireType {
        T::WIRE_TYPE
    }

    fn runtime_type(&self) -> RuntimeType {
        <T::ProtobufValue as ProtobufValue>::RuntimeType::runtime_type_box()
    }
}
