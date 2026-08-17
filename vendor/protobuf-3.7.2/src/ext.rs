//! Utilities to support "extension" fields.
//!
//! This is a stopgap implementation, it only allows to fetch basic singular values,
//! and that's it. Anything similar to extension registry is not implemented yet.
//!
//! Extensions are [described in the official protobuf documentation][exts].
//!
//! [exts]: https://developers.google.com/protocol-buffers/docs/proto#extensions

use std::marker::PhantomData;

use crate::descriptor::field_descriptor_proto::Type;
use crate::reflect::runtime_types::RuntimeTypeTrait;
use crate::reflect::ProtobufValue;
use crate::Message;

/// Optional ext field
///
/// This is initialized from generated code, do not instantiate directly.
pub struct ExtFieldOptional<M, T> {
    /// Extension field number.
    field_number: u32,
    /// Extension field type.
    field_type: Type,
    /// Marker
    phantom: PhantomData<(M, T)>,
}

/// Repeated ext field
///
/// This is initialized from generated code, do not instantiate directly.
pub struct ExtFieldRepeated<M, V> {
    /// Extension field number
    #[allow(dead_code)]
    field_number: u32,
    /// Field type.
    #[allow(dead_code)]
    field_type: Type,
    /// Extension field number
    phantom: PhantomData<(M, V)>,
}

impl<M, V> ExtFieldOptional<M, V> {
    /// Constructor. Called from generated code.
    pub const fn new(field_number: u32, field_type: Type) -> Self {
        ExtFieldOptional {
            field_number,
            field_type,
            phantom: PhantomData,
        }
    }
}

impl<M: Message, V: ProtobufValue> ExtFieldOptional<M, V> {
    /// Get a copy of value from a message.
    ///
    /// Extension data is stored in [`UnknownFields`](crate::UnknownFields).
    pub fn get(&self, m: &M) -> Option<V> {
        m.unknown_fields()
            .get(self.field_number)
            .and_then(|u| V::RuntimeType::get_from_unknown(u, self.field_type))
    }
}

impl<M, V> ExtFieldRepeated<M, V> {
    /// Constructor. Called from generated code.
    pub const fn new(field_number: u32, field_type: Type) -> Self {
        ExtFieldRepeated {
            field_number,
            field_type,
            phantom: PhantomData,
        }
    }
}

impl<M: Message, V: ProtobufValue> ExtFieldRepeated<M, V> {
    /// Get a copy of value from a message (**not implemented**).
    pub fn get(&self, _m: &M) -> Vec<V> {
        unimplemented!("extension fields implementation in rust-protobuf is stopgap")
    }
}
