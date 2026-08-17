//! The raw descriptor format is not very easy to work with, a fact not aided
//! by prost making almost all members of proto2 syntax message optional
//!
//! This module therefore extracts a slightly less obtuse representation of a
//! message that can be used by the code generation logic

use prost_types::{
    field_descriptor_proto::{Label, Type},
    FieldDescriptorProto,
};

use crate::descriptor::{Descriptor, DescriptorSet, MessageDescriptor, Syntax, TypeName, TypePath};
use crate::escape::{escape_ident, escape_type};

#[derive(Debug, Clone, Copy)]
pub enum ScalarType {
    F64,
    F32,
    I32,
    I64,
    U32,
    U64,
    Bool,
    String,
    Bytes,
}

impl ScalarType {
    pub fn rust_type(&self) -> &'static str {
        match self {
            Self::F64 => "f64",
            Self::F32 => "f32",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::Bool => "bool",
            Self::String => "String",
            Self::Bytes => "Vec<u8>",
        }
    }

    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            Self::F64 | Self::F32 | Self::I32 | Self::I64 | Self::U32 | Self::U64
        )
    }
}

#[derive(Debug, Clone)]
pub enum FieldType {
    Scalar(ScalarType),
    Enum(TypePath),
    Message(TypePath),
    Map(ScalarType, Box<FieldType>),
}

#[derive(Debug, Clone, Copy)]
pub enum FieldModifier {
    Required,
    Optional,
    UseDefault,
    Repeated,
}

impl FieldModifier {
    pub fn is_required(&self) -> bool {
        matches!(self, Self::Required)
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub json_name: Option<String>,
    pub field_modifier: FieldModifier,
    pub field_type: FieldType,
}

impl Field {
    pub fn rust_type_name(&self) -> String {
        use heck::ToUpperCamelCase;
        escape_type(self.name.to_upper_camel_case())
    }

    pub fn rust_field_name(&self) -> String {
        use heck::ToSnakeCase;
        escape_ident(self.name.to_snake_case())
    }

    pub fn json_name(&self) -> String {
        use heck::ToLowerCamelCase;
        self.json_name
            .clone()
            .unwrap_or_else(|| self.name.to_lower_camel_case())
    }
}

#[derive(Debug, Clone)]
pub struct OneOf {
    pub name: String,
    pub path: TypePath,
    pub fields: Vec<Field>,
}

impl OneOf {
    pub fn rust_field_name(&self) -> String {
        use heck::ToSnakeCase;
        escape_ident(self.name.to_snake_case())
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub path: TypePath,
    pub fields: Vec<Field>,
    pub one_ofs: Vec<OneOf>,
}

impl Message {
    pub fn all_fields(&self) -> impl Iterator<Item = &Field> + '_ {
        self.fields
            .iter()
            .chain(self.one_ofs.iter().flat_map(|one_of| one_of.fields.iter()))
    }
}

/// Resolve the provided message descriptor into a slightly less obtuse representation
///
/// Returns None if the provided provided message is auto-generated
pub fn resolve_message(
    descriptors: &DescriptorSet,
    message: &MessageDescriptor,
) -> Option<Message> {
    if message.is_map() {
        return None;
    }

    let mut fields = Vec::new();
    let mut one_of_fields = vec![Vec::new(); message.one_of.len()];

    for field in &message.fields {
        let field_type = field_type(descriptors, field);
        let field_modifier = field_modifier(message, field, &field_type);

        let resolved = Field {
            name: field.name.clone().expect("expected field to have name"),
            json_name: field.json_name.clone(),
            field_type,
            field_modifier,
        };

        // Treat synthetic one-of as normal
        let proto3_optional = field.proto3_optional.unwrap_or(false);
        match (field.oneof_index, proto3_optional) {
            (Some(idx), false) => one_of_fields[idx as usize].push(resolved),
            _ => fields.push(resolved),
        }
    }

    let mut one_ofs = Vec::new();

    for (fields, descriptor) in one_of_fields.into_iter().zip(&message.one_of) {
        // Might be empty in the event of a synthetic one-of
        if !fields.is_empty() {
            let name = descriptor.name.clone().expect("oneof with no name");
            let path = message.path.child(TypeName::new(&name));

            one_ofs.push(OneOf { name, path, fields })
        }
    }

    Some(Message {
        path: message.path.clone(),
        fields,
        one_ofs,
    })
}

fn field_modifier(
    message: &MessageDescriptor,
    field: &FieldDescriptorProto,
    field_type: &FieldType,
) -> FieldModifier {
    let label = Label::try_from(field.label.expect("expected label")).expect("valid label");
    if field.proto3_optional.unwrap_or(false) {
        assert_eq!(label, Label::Optional);
        return FieldModifier::Optional;
    }

    if field.oneof_index.is_some() {
        assert_eq!(label, Label::Optional);
        return FieldModifier::Optional;
    }

    if matches!(field_type, FieldType::Map(_, _)) {
        assert_eq!(label, Label::Repeated);
        return FieldModifier::Repeated;
    }

    match label {
        Label::Optional => match message.syntax {
            Syntax::Proto2 => FieldModifier::Optional,
            Syntax::Proto3 => match field_type {
                FieldType::Message(_) => FieldModifier::Optional,
                _ => FieldModifier::UseDefault,
            },
        },
        Label::Required => FieldModifier::Required,
        Label::Repeated => FieldModifier::Repeated,
    }
}

fn field_type(descriptors: &DescriptorSet, field: &FieldDescriptorProto) -> FieldType {
    match field.type_name.as_ref() {
        Some(type_name) => resolve_type(descriptors, type_name.as_str()),
        None => {
            let scalar =
                match Type::try_from(field.r#type.expect("expected type")).expect("valid type") {
                    Type::Double => ScalarType::F64,
                    Type::Float => ScalarType::F32,
                    Type::Int64 | Type::Sfixed64 | Type::Sint64 => ScalarType::I64,
                    Type::Int32 | Type::Sfixed32 | Type::Sint32 => ScalarType::I32,
                    Type::Uint64 | Type::Fixed64 => ScalarType::U64,
                    Type::Uint32 | Type::Fixed32 => ScalarType::U32,
                    Type::Bool => ScalarType::Bool,
                    Type::String => ScalarType::String,
                    Type::Bytes => ScalarType::Bytes,
                    Type::Message | Type::Enum | Type::Group => panic!("no type name specified"),
                };
            FieldType::Scalar(scalar)
        }
    }
}

fn resolve_type(descriptors: &DescriptorSet, type_name: &str) -> FieldType {
    assert!(
        type_name.starts_with('.'),
        "pbjson does not currently support resolving relative types"
    );
    let maybe_descriptor = descriptors
        .iter()
        .find(|(path, _)| path.prefix_match(type_name).is_some());

    match maybe_descriptor {
        Some((path, Descriptor::Enum(_))) => FieldType::Enum(path.clone()),
        Some((path, Descriptor::Message(descriptor))) => match descriptor.is_map() {
            true => {
                assert_eq!(descriptor.fields.len(), 2, "expected map to have 2 fields");
                let key = &descriptor.fields[0];
                let value = &descriptor.fields[1];

                assert_eq!("key", key.name());
                assert_eq!("value", value.name());

                let key_type = match field_type(descriptors, key) {
                    FieldType::Scalar(scalar) => scalar,
                    _ => panic!("non scalar map key"),
                };
                let value_type = field_type(descriptors, value);
                FieldType::Map(key_type, Box::new(value_type))
            }
            // Note: This may actually be a group but it is non-trivial to detect this,
            // they're deprecated, and pbjson doesn't need to be able to distinguish
            false => FieldType::Message(path.clone()),
        },
        None => panic!("failed to resolve type: {}", type_name),
    }
}
