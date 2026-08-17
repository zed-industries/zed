use std::collections::HashMap;
use std::iter;

use crate::descriptor::field_descriptor_proto;
use crate::descriptor::DescriptorProto;
use crate::descriptor::EnumDescriptorProto;
use crate::descriptor::FieldDescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::reflect::error::ReflectError;
use crate::reflect::field::index::ForwardProtobufFieldType;
use crate::reflect::field::index::ForwardProtobufTypeBox;
use crate::reflect::file::index::MessageIndices;
use crate::reflect::find_message_or_enum::find_message_or_enum;
use crate::reflect::find_message_or_enum::MessageOrEnum;
use crate::reflect::name::protobuf_name_starts_with_package;
use crate::reflect::runtime_type_box::RuntimeType;
use crate::reflect::FileDescriptor;

pub(crate) struct FileDescriptorBuilding<'a> {
    pub(crate) current_file_descriptor: &'a FileDescriptorProto,
    pub(crate) deps_with_public: &'a [FileDescriptor],
    pub(crate) message_by_name_to_package: &'a HashMap<String, usize>,
    pub(crate) messages: &'a [MessageIndices],
    pub(crate) enums_by_name_to_package: &'a HashMap<String, usize>,
}

impl<'a> FileDescriptorBuilding<'a> {
    fn all_descriptors(&self) -> impl Iterator<Item = &'a FileDescriptorProto> {
        iter::once(self.current_file_descriptor)
            .chain(self.deps_with_public.iter().map(|d| d.proto()))
    }

    pub fn find_enum(&self, full_name: &str) -> &'a EnumDescriptorProto {
        assert!(full_name.starts_with("."));

        for file in self.all_descriptors() {
            if let Some(name_to_package) =
                protobuf_name_starts_with_package(full_name, file.package())
            {
                if let Some((_, me)) = find_message_or_enum(file, name_to_package) {
                    match me {
                        MessageOrEnum::Enum(e) => return e,
                        MessageOrEnum::Message(_) => panic!("not an enum: {}", full_name),
                    }
                }
            }
        }

        panic!(
            "enum not found: {}, in files: {}",
            full_name,
            self.all_files_str()
        );
    }

    fn all_files_str(&self) -> String {
        self.all_descriptors()
            .map(|d| d.name())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub(crate) fn resolve_field_type(
        &self,
        field: &FieldDescriptorProto,
    ) -> crate::Result<ForwardProtobufFieldType> {
        Ok(match field.label() {
            field_descriptor_proto::Label::LABEL_OPTIONAL
            | field_descriptor_proto::Label::LABEL_REQUIRED => {
                ForwardProtobufFieldType::Singular(self.resolve_field_element_type(field)?)
            }
            field_descriptor_proto::Label::LABEL_REPEATED => {
                let element = self.resolve_field_element_type(field)?;
                let type_proto = match &element {
                    ForwardProtobufTypeBox::CurrentFileMessage(m) => {
                        Some(&*self.messages[*m].proto)
                    }
                    ForwardProtobufTypeBox::ProtobufTypeBox(t) => match t.runtime() {
                        RuntimeType::Message(m) => Some(m.proto()),
                        _ => None,
                    },
                    _ => None,
                };
                match type_proto {
                    Some(m) if m.options.get_or_default().map_entry() => self.map_field(m)?,
                    _ => ForwardProtobufFieldType::Repeated(element),
                }
            }
        })
    }

    fn resolve_field_element_type(
        &self,
        field: &FieldDescriptorProto,
    ) -> crate::Result<ForwardProtobufTypeBox> {
        Ok(match field.type_() {
            field_descriptor_proto::Type::TYPE_MESSAGE
            | field_descriptor_proto::Type::TYPE_GROUP => {
                self.resolve_message(field.type_name())?
            }
            field_descriptor_proto::Type::TYPE_ENUM => {
                if let Some(name_to_package) = protobuf_name_starts_with_package(
                    field.type_name(),
                    self.current_file_descriptor.package(),
                ) {
                    if let Some(index) = self.enums_by_name_to_package.get(name_to_package) {
                        return Ok(ForwardProtobufTypeBox::CurrentFileEnum(*index));
                    }
                }
                for dep in self.deps_with_public {
                    if let Some(m) = dep.enum_by_full_name(field.type_name()) {
                        return Ok(ForwardProtobufTypeBox::enumeration(m));
                    }
                }
                panic!(
                    "enum not found: {}; files: {}",
                    field.type_name(),
                    self.all_files_str()
                );
            }
            t => ForwardProtobufTypeBox::from_proto_type(t),
        })
    }

    pub(crate) fn resolve_message(&self, type_name: &str) -> crate::Result<ForwardProtobufTypeBox> {
        if let Some(name_to_package) =
            protobuf_name_starts_with_package(type_name, self.current_file_descriptor.package())
        {
            if let Some(index) = self.message_by_name_to_package.get(name_to_package) {
                return Ok(ForwardProtobufTypeBox::CurrentFileMessage(*index));
            }
        }
        for dep in self.deps_with_public {
            if let Some(m) = dep.message_by_full_name(type_name) {
                return Ok(ForwardProtobufTypeBox::message(m));
            }
        }
        Err(ReflectError::MessageNotFoundInFiles(type_name.to_owned(), self.all_files_str()).into())
    }

    fn map_field(&self, type_proto: &DescriptorProto) -> crate::Result<ForwardProtobufFieldType> {
        assert!(type_proto.name().ends_with("Entry"));

        assert_eq!(0, type_proto.extension.len());
        assert_eq!(0, type_proto.extension_range.len());
        assert_eq!(0, type_proto.nested_type.len());
        assert_eq!(0, type_proto.enum_type.len());

        assert_eq!(2, type_proto.field.len());
        let key = &type_proto.field[0];
        let value = &type_proto.field[1];

        assert_eq!("key", key.name());
        assert_eq!("value", value.name());

        assert_eq!(1, key.number());
        assert_eq!(2, value.number());

        assert_eq!(field_descriptor_proto::Label::LABEL_OPTIONAL, key.label());
        assert_eq!(field_descriptor_proto::Label::LABEL_OPTIONAL, value.label());

        // It is OK to resolve using current descriptor because map field
        // should always point to the same file.
        let key = self.resolve_field_element_type(key)?;
        let value = self.resolve_field_element_type(value)?;
        Ok(ForwardProtobufFieldType::Map(key, value))
    }
}
