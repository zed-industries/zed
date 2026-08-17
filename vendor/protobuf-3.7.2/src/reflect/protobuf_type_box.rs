use crate::descriptor::field_descriptor_proto::Type;
use crate::error::ProtobufError;
use crate::error::WireError;
use crate::reflect::EnumDescriptor;
use crate::reflect::MessageDescriptor;
use crate::reflect::ProtobufValue;
use crate::reflect::ReflectRepeatedMut;
use crate::reflect::ReflectValueBox;
use crate::reflect::RuntimeType;
use crate::wire_format::WireType;
use crate::CodedInputStream;

/// Runtime type and protobuf type.
#[derive(Debug, Clone)]
pub(crate) struct ProtobufType {
    /// Runtime type.
    runtime: RuntimeType,
    /// Wire type.
    t: Type,
}

impl ProtobufType {
    pub(crate) fn runtime(&self) -> &RuntimeType {
        &self.runtime
    }

    pub(crate) fn t(&self) -> Type {
        self.t
    }

    pub(crate) fn _into_runtime(self) -> RuntimeType {
        self.runtime
    }

    pub(crate) fn message(message: MessageDescriptor) -> ProtobufType {
        ProtobufType::new(RuntimeType::Message(message), Type::TYPE_MESSAGE).unwrap()
    }

    pub(crate) fn enumeration(enumeration: EnumDescriptor) -> ProtobufType {
        ProtobufType::new(RuntimeType::Enum(enumeration), Type::TYPE_ENUM).unwrap()
    }

    pub(crate) fn from_proto_type(t: Type) -> ProtobufType {
        ProtobufType::new(RuntimeType::from_proto_type(t), t).unwrap()
    }

    pub(crate) fn new(runtime: RuntimeType, t: Type) -> crate::Result<ProtobufType> {
        match (t, &runtime) {
            (Type::TYPE_INT32, RuntimeType::I32) => {}
            (Type::TYPE_INT64, RuntimeType::I64) => {}
            (Type::TYPE_UINT32, RuntimeType::U32) => {}
            (Type::TYPE_UINT64, RuntimeType::U64) => {}
            (Type::TYPE_SINT32, RuntimeType::I32) => {}
            (Type::TYPE_SINT64, RuntimeType::I64) => {}
            (Type::TYPE_FIXED32, RuntimeType::U32) => {}
            (Type::TYPE_FIXED64, RuntimeType::U64) => {}
            (Type::TYPE_SFIXED32, RuntimeType::I32) => {}
            (Type::TYPE_SFIXED64, RuntimeType::I64) => {}
            (Type::TYPE_FLOAT, RuntimeType::F32) => {}
            (Type::TYPE_DOUBLE, RuntimeType::F64) => {}
            (Type::TYPE_BOOL, RuntimeType::Bool) => {}
            (Type::TYPE_STRING, RuntimeType::String) => {}
            (Type::TYPE_BYTES, RuntimeType::VecU8) => {}
            (Type::TYPE_MESSAGE, RuntimeType::Message(..)) => {}
            (Type::TYPE_ENUM, RuntimeType::Enum(..)) => {}
            (Type::TYPE_GROUP, ..) => return Err(ProtobufError::GroupIsNotImplemented.into()),
            _ => return Err(ProtobufError::IncompatibleProtobufTypeAndRuntimeType.into()),
        }
        Ok(ProtobufType { runtime, t })
    }

    pub(crate) fn read(
        &self,
        is: &mut CodedInputStream,
        wire_type: WireType,
    ) -> crate::Result<ReflectValueBox> {
        if wire_type != WireType::for_type(self.t) {
            return Err(WireError::UnexpectedWireType(wire_type).into());
        }
        Ok(match self.t {
            Type::TYPE_DOUBLE => ReflectValueBox::F64(is.read_double()?),
            Type::TYPE_FLOAT => ReflectValueBox::F32(is.read_float()?),
            Type::TYPE_INT64 => ReflectValueBox::I64(is.read_int64()?),
            Type::TYPE_UINT64 => ReflectValueBox::U64(is.read_uint64()?),
            Type::TYPE_INT32 => ReflectValueBox::I32(is.read_int32()?),
            Type::TYPE_FIXED64 => ReflectValueBox::U64(is.read_fixed64()?),
            Type::TYPE_FIXED32 => ReflectValueBox::U32(is.read_fixed32()?),
            Type::TYPE_BOOL => ReflectValueBox::Bool(is.read_bool()?),
            Type::TYPE_UINT32 => ReflectValueBox::U32(is.read_uint32()?),
            Type::TYPE_SFIXED32 => ReflectValueBox::I32(is.read_sfixed32()?),
            Type::TYPE_SFIXED64 => ReflectValueBox::I64(is.read_sfixed64()?),
            Type::TYPE_SINT32 => ReflectValueBox::I32(is.read_sint32()?),
            Type::TYPE_SINT64 => ReflectValueBox::I64(is.read_sint64()?),
            Type::TYPE_STRING => ReflectValueBox::String(is.read_string()?),
            Type::TYPE_BYTES => ReflectValueBox::Bytes(is.read_bytes()?),
            Type::TYPE_ENUM => match &self.runtime {
                RuntimeType::Enum(e) => {
                    let v = is.read_enum_value()?;
                    ReflectValueBox::Enum(e.clone(), v)
                }
                _ => unreachable!(),
            },
            Type::TYPE_GROUP => return Err(ProtobufError::GroupIsNotImplemented.into()),
            Type::TYPE_MESSAGE => match &self.runtime {
                RuntimeType::Message(m) => ReflectValueBox::Message(is.read_message_dyn(m)?),
                _ => unreachable!(),
            },
        })
    }

    pub(crate) fn read_repeated_into(
        &self,
        is: &mut CodedInputStream,
        wire_type: WireType,
        repeated: &mut ReflectRepeatedMut,
    ) -> crate::Result<()> {
        if wire_type == WireType::for_type(self.t) {
            let value = self.read(is, wire_type)?;
            repeated.push(value);
            Ok(())
        } else if wire_type == WireType::LengthDelimited {
            fn extend<V: ProtobufValue>(repeated: &mut ReflectRepeatedMut, mut v: Vec<V>) {
                repeated.extend(ReflectRepeatedMut::new(&mut v));
            }

            match self.t {
                Type::TYPE_INT32 => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_int32_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_INT64 => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_int64_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_UINT32 => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_uint32_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_UINT64 => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_uint64_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_SINT32 => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_sint32_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_SINT64 => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_sint64_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_FIXED32 => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_fixed32_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_FIXED64 => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_fixed64_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_SFIXED32 => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_sfixed32_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_SFIXED64 => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_sfixed64_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_FLOAT => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_float_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_DOUBLE => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_double_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_BOOL => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_bool_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_ENUM => match &self.runtime {
                    RuntimeType::Enum(e) => {
                        let mut v = Vec::new();
                        is.read_repeated_packed_enum_values_into(&mut v)?;
                        for e_v in v {
                            repeated.push(ReflectValueBox::Enum(e.clone(), e_v));
                        }
                        Ok(())
                    }
                    _ => unreachable!(),
                },
                Type::TYPE_GROUP => Err(ProtobufError::GroupIsNotImplemented.into()),
                Type::TYPE_MESSAGE | Type::TYPE_STRING | Type::TYPE_BYTES => {
                    Err(WireError::UnexpectedWireType(wire_type).into())
                }
            }
        } else {
            Err(WireError::UnexpectedWireType(wire_type).into())
        }
    }
}
