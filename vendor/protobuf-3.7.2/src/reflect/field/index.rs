use protobuf_support::json_name::json_name;

use crate::descriptor::field_descriptor_proto::Type;
use crate::descriptor::FieldDescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::owning_ref::OwningRef;
use crate::reflect::error::ReflectError;
use crate::reflect::field::protobuf_field_type::ProtobufFieldType;
use crate::reflect::file::building::FileDescriptorBuilding;
use crate::reflect::protobuf_type_box::ProtobufType;
use crate::reflect::EnumDescriptor;
use crate::reflect::EnumValueDescriptor;
use crate::reflect::FieldDescriptor;
use crate::reflect::FileDescriptor;
use crate::reflect::MessageDescriptor;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;
use crate::reflect::RuntimeType;

#[derive(Debug)]
pub(crate) enum ForwardProtobufTypeBox {
    ProtobufTypeBox(ProtobufType),
    CurrentFileEnum(usize),
    CurrentFileMessage(usize),
}

impl ForwardProtobufTypeBox {
    pub(crate) fn message(message: MessageDescriptor) -> ForwardProtobufTypeBox {
        ForwardProtobufTypeBox::ProtobufTypeBox(ProtobufType::message(message))
    }

    pub(crate) fn enumeration(enumeration: EnumDescriptor) -> ForwardProtobufTypeBox {
        ForwardProtobufTypeBox::ProtobufTypeBox(ProtobufType::enumeration(enumeration))
    }

    pub(crate) fn from_proto_type(t: Type) -> ForwardProtobufTypeBox {
        ForwardProtobufTypeBox::ProtobufTypeBox(ProtobufType::from_proto_type(t))
    }

    pub(crate) fn resolve(&self, file: &FileDescriptor) -> ProtobufType {
        match self {
            ForwardProtobufTypeBox::ProtobufTypeBox(t) => t.clone(),
            ForwardProtobufTypeBox::CurrentFileMessage(m) => {
                ProtobufType::message(MessageDescriptor::new(file.clone(), *m))
            }
            ForwardProtobufTypeBox::CurrentFileEnum(m) => {
                ProtobufType::enumeration(EnumDescriptor::new(file.clone(), *m))
            }
        }
    }

    pub(crate) fn resolve_message(&self, file: &FileDescriptor) -> MessageDescriptor {
        match self.resolve(file).runtime() {
            RuntimeType::Message(m) => m.clone(),
            _ => panic!("not message"),
        }
    }
}

#[derive(Debug)]
pub(crate) enum ForwardProtobufFieldType {
    Singular(ForwardProtobufTypeBox),
    Repeated(ForwardProtobufTypeBox),
    Map(ForwardProtobufTypeBox, ForwardProtobufTypeBox),
}

impl ForwardProtobufFieldType {
    pub(crate) fn resolve(&self, file: &FileDescriptor) -> ProtobufFieldType {
        match self {
            ForwardProtobufFieldType::Singular(t) => ProtobufFieldType::Singular(t.resolve(file)),
            ForwardProtobufFieldType::Repeated(t) => ProtobufFieldType::Repeated(t.resolve(file)),
            ForwardProtobufFieldType::Map(k, v) => {
                ProtobufFieldType::Map(k.resolve(file), v.resolve(file))
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum FieldDefaultValue {
    ReflectValueBox(ReflectValueBox),
    Enum(usize),
}

#[derive(Debug)]
pub(crate) enum FieldKind {
    MessageField(
        /// Message index.
        usize,
    ),
    Extension(
        /// Message index or `None` for file.
        Option<usize>,
        ForwardProtobufTypeBox,
    ),
}

#[derive(Debug)]
pub(crate) struct FieldIndex {
    pub(crate) proto: OwningRef<FileDescriptorProto, FieldDescriptorProto>,
    pub(crate) kind: FieldKind,
    pub(crate) json_name: String,
    pub(crate) field_type: ForwardProtobufFieldType,
    pub(crate) default_value: Option<FieldDefaultValue>,
}

impl FieldIndex {
    fn enum_default_value(
        field: &FieldDescriptorProto,
        building: &FileDescriptorBuilding,
    ) -> crate::Result<FieldDefaultValue> {
        let en = building.find_enum(field.type_name());
        let (n, _) = match en
            .value
            .iter()
            .enumerate()
            .find(|(_n, v)| v.name() == field.default_value())
        {
            Some(v) => v,
            None => Err(ReflectError::CouldNotParseDefaultValueForField(
                field.name().to_owned(),
            ))?,
        };
        Ok(FieldDefaultValue::Enum(n))
    }

    fn parse_default_value(
        field: &FieldDescriptorProto,
        building: &FileDescriptorBuilding,
    ) -> crate::Result<FieldDefaultValue> {
        Ok(FieldDefaultValue::ReflectValueBox(match field.type_() {
            Type::TYPE_GROUP | Type::TYPE_MESSAGE => {
                return Err(ReflectError::CouldNotParseDefaultValueForField(
                    field.name().to_owned(),
                )
                .into());
            }
            Type::TYPE_ENUM => {
                return Self::enum_default_value(field, building);
            }
            t => RuntimeType::from_proto_type(t)
                .parse_proto_default_value(field.default_value())
                .map_err(|()| {
                    ReflectError::CouldNotParseDefaultValueForField(field.name().to_owned())
                })?,
        }))
    }

    pub(crate) fn index(
        containing_message: Option<usize>,
        field: OwningRef<FileDescriptorProto, FieldDescriptorProto>,
        building: &FileDescriptorBuilding,
    ) -> crate::Result<FieldIndex> {
        let default_value = if field.has_default_value() {
            Some(Self::parse_default_value(&field, building)?)
        } else {
            None
        };

        let json_name = if !field.json_name().is_empty() {
            field.json_name().to_owned()
        } else {
            json_name(field.name())
        };

        let extendee = if field.has_extendee() {
            Some(building.resolve_message(field.extendee())?)
        } else {
            None
        };

        let kind = match (containing_message, extendee) {
            (Some(m), None) => FieldKind::MessageField(m),
            (m, Some(extendee)) => FieldKind::Extension(m, extendee),
            (None, None) => panic!("field must be in a message or an extension"),
        };

        let field_type = building.resolve_field_type(&field)?;
        Ok(FieldIndex {
            proto: field,
            kind,
            default_value,
            json_name,
            field_type,
        })
    }

    pub(crate) fn default_value<'a>(&'a self, field: &FieldDescriptor) -> ReflectValueRef<'a> {
        match &self.default_value {
            Some(FieldDefaultValue::ReflectValueBox(v)) => v.as_value_ref(),
            Some(FieldDefaultValue::Enum(v)) => match field.singular_runtime_type() {
                RuntimeType::Enum(e) => {
                    let ev = EnumValueDescriptor::new(e.clone(), *v);
                    ReflectValueRef::from(ev)
                }
                t => panic!("wrong type {:?} for default value enum", t),
            },
            None => field.singular_runtime_type().default_value_ref(),
        }
    }
}
