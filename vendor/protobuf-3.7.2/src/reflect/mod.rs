//! # Reflection implementation for protobuf data
//!
//! ## Generated vs dynamic
//!
//! rust-protobuf supports reflection for both:
//! * generated messages (generated rust code)
//! * dynamic messages (created from arbitrary `FileDescriptorProto` without code generation)
//!
//! The API to work with these types of messages is the same.
//!
//! ## API
//!
//! The API roughly follows Google protobuf C++ and Java API.
//! Some minor adjustements are made to make code more idiomatic to rust.

mod acc;
mod dynamic;
mod enums;
pub(crate) mod error;
mod field;
mod file;
mod find_message_or_enum;
mod map;
pub(crate) mod message;
mod oneof;
mod optional;
mod protobuf_type_box;
mod repeated;
mod runtime_type_box;
mod service;
mod type_dynamic;
pub(crate) mod value;

// Runtime type types are public, but not visible in public API.
pub(crate) mod runtime_types;

pub(crate) mod types;

pub(crate) mod reflect_eq;

pub mod rt;

pub(crate) mod name;

#[doc(hidden)]
pub use self::enums::generated::GeneratedEnumDescriptorData;
pub use self::enums::EnumDescriptor;
pub use self::enums::EnumValueDescriptor;
pub use self::field::runtime_field_type::RuntimeFieldType;
pub use self::field::FieldDescriptor;
pub use self::field::ReflectFieldRef;
#[doc(hidden)]
pub use self::file::generated::GeneratedFileDescriptor;
pub use self::file::syntax::Syntax;
pub use self::file::FileDescriptor;
pub use self::map::ReflectMapMut;
pub use self::map::ReflectMapRef;
#[doc(hidden)]
pub use self::message::generated::GeneratedMessageDescriptorData;
pub use self::message::message_ref::MessageRef;
pub use self::message::MessageDescriptor;
#[doc(hidden)]
pub use self::oneof::generated::GeneratedOneofDescriptorData;
pub use self::oneof::OneofDescriptor;
pub use self::optional::ReflectOptionalRef;
pub use self::reflect_eq::ReflectEq;
pub use self::reflect_eq::ReflectEqMode;
pub use self::repeated::ReflectRepeatedMut;
pub use self::repeated::ReflectRepeatedRef;
pub use self::runtime_type_box::RuntimeType;
pub use self::service::MethodDescriptor;
pub use self::service::ServiceDescriptor;
pub use self::value::value_box::ReflectValueBox;
pub use self::value::value_ref::ReflectValueRef;
pub use self::value::ProtobufValue;
