use std::ascii;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::iter;

use itertools::{Either, Itertools};
use log::debug;
use multimap::MultiMap;
use prost_types::field_descriptor_proto::{Label, Type};
use prost_types::source_code_info::Location;
use prost_types::{
    DescriptorProto, EnumDescriptorProto, EnumValueDescriptorProto, FieldDescriptorProto,
    FieldOptions, FileDescriptorProto, OneofDescriptorProto, ServiceDescriptorProto,
    SourceCodeInfo,
};

use crate::ast::{Comments, Method, Service};
use crate::extern_paths::ExternPaths;
use crate::ident::{to_snake, to_upper_camel};
use crate::message_graph::MessageGraph;
use crate::{BytesType, Config, MapType};

#[derive(PartialEq)]
enum Syntax {
    Proto2,
    Proto3,
}

pub struct CodeGenerator<'a> {
    config: &'a mut Config,
    package: String,
    source_info: SourceCodeInfo,
    syntax: Syntax,
    message_graph: &'a MessageGraph,
    extern_paths: &'a ExternPaths,
    depth: u8,
    path: Vec<i32>,
    buf: &'a mut String,
}

fn push_indent(buf: &mut String, depth: u8) {
    for _ in 0..depth {
        buf.push_str("    ");
    }
}

impl<'a> CodeGenerator<'a> {
    pub fn generate(
        config: &mut Config,
        message_graph: &MessageGraph,
        extern_paths: &ExternPaths,
        file: FileDescriptorProto,
        buf: &mut String,
    ) {
        let mut source_info = file
            .source_code_info
            .expect("no source code info in request");
        source_info.location.retain(|location| {
            let len = location.path.len();
            len > 0 && len % 2 == 0
        });
        source_info
            .location
            .sort_by_key(|location| location.path.clone());

        let syntax = match file.syntax.as_ref().map(String::as_str) {
            None | Some("proto2") => Syntax::Proto2,
            Some("proto3") => Syntax::Proto3,
            Some(s) => panic!("unknown syntax: {}", s),
        };

        let mut code_gen = CodeGenerator {
            config,
            package: file.package.unwrap_or_else(String::new),
            source_info,
            syntax,
            message_graph,
            extern_paths,
            depth: 0,
            path: Vec::new(),
            buf,
        };

        debug!(
            "file: {:?}, package: {:?}",
            file.name.as_ref().unwrap(),
            code_gen.package
        );

        code_gen.path.push(4);
        for (idx, message) in file.message_type.into_iter().enumerate() {
            code_gen.path.push(idx as i32);
            code_gen.append_message(message);
            code_gen.path.pop();
        }
        code_gen.path.pop();

        code_gen.path.push(5);
        for (idx, desc) in file.enum_type.into_iter().enumerate() {
            code_gen.path.push(idx as i32);
            code_gen.append_enum(desc);
            code_gen.path.pop();
        }
        code_gen.path.pop();

        if code_gen.config.service_generator.is_some() {
            code_gen.path.push(6);
            for (idx, service) in file.service.into_iter().enumerate() {
                code_gen.path.push(idx as i32);
                code_gen.push_service(service);
                code_gen.path.pop();
            }

            if let Some(service_generator) = code_gen.config.service_generator.as_mut() {
                service_generator.finalize(code_gen.buf);
            }

            code_gen.path.pop();
        }
    }

    fn append_message(&mut self, message: DescriptorProto) {
        debug!("  message: {:?}", message.name());

        let message_name = message.name().to_string();
        let fq_message_name = format!(
            "{}{}.{}",
            if self.package.is_empty() { "" } else { "." },
            self.package,
            message.name()
        );

        // Skip external types.
        if self.extern_paths.resolve_ident(&fq_message_name).is_some() {
            return;
        }

        // Split the nested message types into a vector of normal nested message types, and a map
        // of the map field entry types. The path index of the nested message types is preserved so
        // that comments can be retrieved.
        type NestedTypes = Vec<(DescriptorProto, usize)>;
        type MapTypes = HashMap<String, (FieldDescriptorProto, FieldDescriptorProto)>;
        let (nested_types, map_types): (NestedTypes, MapTypes) = message
            .nested_type
            .into_iter()
            .enumerate()
            .partition_map(|(idx, nested_type)| {
                if nested_type
                    .options
                    .as_ref()
                    .and_then(|options| options.map_entry)
                    .unwrap_or(false)
                {
                    let key = nested_type.field[0].clone();
                    let value = nested_type.field[1].clone();
                    assert_eq!("key", key.name());
                    assert_eq!("value", value.name());

                    let name = format!("{}.{}", &fq_message_name, nested_type.name());
                    Either::Right((name, (key, value)))
                } else {
                    Either::Left((nested_type, idx))
                }
            });

        // Split the fields into a vector of the normal fields, and oneof fields.
        // Path indexes are preserved so that comments can be retrieved.
        type Fields = Vec<(FieldDescriptorProto, usize)>;
        type OneofFields = MultiMap<i32, (FieldDescriptorProto, usize)>;
        let (fields, mut oneof_fields): (Fields, OneofFields) = message
            .field
            .into_iter()
            .enumerate()
            .partition_map(|(idx, field)| {
                if field.proto3_optional.unwrap_or(false) {
                    Either::Left((field, idx))
                } else if let Some(oneof_index) = field.oneof_index {
                    Either::Right((oneof_index, (field, idx)))
                } else {
                    Either::Left((field, idx))
                }
            });

        self.append_doc(&fq_message_name, None);
        self.append_type_attributes(&fq_message_name);
        self.push_indent();
        self.buf
            .push_str("#[derive(Clone, PartialEq, ::prost::Message)]\n");
        self.push_indent();
        self.buf.push_str("pub struct ");
        self.buf.push_str(&to_upper_camel(&message_name));
        self.buf.push_str(" {\n");

        self.depth += 1;
        self.path.push(2);
        for (field, idx) in fields {
            self.path.push(idx as i32);
            match field
                .type_name
                .as_ref()
                .and_then(|type_name| map_types.get(type_name))
            {
                Some(&(ref key, ref value)) => {
                    self.append_map_field(&fq_message_name, field, key, value)
                }
                None => self.append_field(&fq_message_name, field),
            }
            self.path.pop();
        }
        self.path.pop();

        self.path.push(8);
        for (idx, oneof) in message.oneof_decl.iter().enumerate() {
            let idx = idx as i32;

            let fields = match oneof_fields.get_vec(&idx) {
                Some(fields) => fields,
                None => continue,
            };

            self.path.push(idx);
            self.append_oneof_field(&message_name, &fq_message_name, oneof, fields);
            self.path.pop();
        }
        self.path.pop();

        self.depth -= 1;
        self.push_indent();
        self.buf.push_str("}\n");

        if !message.enum_type.is_empty() || !nested_types.is_empty() || !oneof_fields.is_empty() {
            self.push_mod(&message_name);
            self.path.push(3);
            for (nested_type, idx) in nested_types {
                self.path.push(idx as i32);
                self.append_message(nested_type);
                self.path.pop();
            }
            self.path.pop();

            self.path.push(4);
            for (idx, nested_enum) in message.enum_type.into_iter().enumerate() {
                self.path.push(idx as i32);
                self.append_enum(nested_enum);
                self.path.pop();
            }
            self.path.pop();

            for (idx, oneof) in message.oneof_decl.into_iter().enumerate() {
                let idx = idx as i32;
                // optional fields create a synthetic oneof that we want to skip
                let fields = match oneof_fields.remove(&idx) {
                    Some(fields) => fields,
                    None => continue,
                };
                self.append_oneof(&fq_message_name, oneof, idx, fields);
            }

            self.pop_mod();
        }
    }

    fn append_type_attributes(&mut self, fq_message_name: &str) {
        assert_eq!(b'.', fq_message_name.as_bytes()[0]);
        for attribute in self.config.type_attributes.get(fq_message_name) {
            push_indent(&mut self.buf, self.depth);
            self.buf.push_str(&attribute);
            self.buf.push('\n');
        }
    }

    fn append_field_attributes(&mut self, fq_message_name: &str, field_name: &str) {
        assert_eq!(b'.', fq_message_name.as_bytes()[0]);
        for attribute in self
            .config
            .field_attributes
            .get_field(fq_message_name, field_name)
        {
            push_indent(&mut self.buf, self.depth);
            self.buf.push_str(&attribute);
            self.buf.push('\n');
        }
    }

    fn append_field(&mut self, fq_message_name: &str, field: FieldDescriptorProto) {
        let type_ = field.r#type();
        let repeated = field.label == Some(Label::Repeated as i32);
        let deprecated = self.deprecated(&field);
        let optional = self.optional(&field);
        let ty = self.resolve_type(&field, fq_message_name);

        let boxed = !repeated
            && (type_ == Type::Message || type_ == Type::Group)
            && self
                .message_graph
                .is_nested(field.type_name(), fq_message_name);

        debug!(
            "    field: {:?}, type: {:?}, boxed: {}",
            field.name(),
            ty,
            boxed
        );

        self.append_doc(fq_message_name, Some(field.name()));

        if deprecated {
            self.push_indent();
            self.buf.push_str("#[deprecated]\n");
        }

        self.push_indent();
        self.buf.push_str("#[prost(");
        let type_tag = self.field_type_tag(&field);
        self.buf.push_str(&type_tag);

        if type_ == Type::Bytes {
            let bytes_type = self
                .config
                .bytes_type
                .get_first_field(fq_message_name, field.name())
                .copied()
                .unwrap_or_default();
            self.buf
                .push_str(&format!("={:?}", bytes_type.annotation()));
        }

        match field.label() {
            Label::Optional => {
                if optional {
                    self.buf.push_str(", optional");
                }
            }
            Label::Required => self.buf.push_str(", required"),
            Label::Repeated => {
                self.buf.push_str(", repeated");
                if can_pack(&field)
                    && !field
                        .options
                        .as_ref()
                        .map_or(self.syntax == Syntax::Proto3, |options| options.packed())
                {
                    self.buf.push_str(", packed=\"false\"");
                }
            }
        }

        if boxed {
            self.buf.push_str(", boxed");
        }
        self.buf.push_str(", tag=\"");
        self.buf.push_str(&field.number().to_string());

        if let Some(ref default) = field.default_value {
            self.buf.push_str("\", default=\"");
            if type_ == Type::Bytes {
                self.buf.push_str("b\\\"");
                for b in unescape_c_escape_string(default) {
                    self.buf.extend(
                        ascii::escape_default(b).flat_map(|c| (c as char).escape_default()),
                    );
                }
                self.buf.push_str("\\\"");
            } else if type_ == Type::Enum {
                let enum_value = to_upper_camel(default);
                let stripped_prefix = if self.config.strip_enum_prefix {
                    // Field types are fully qualified, so we extract
                    // the last segment and strip it from the left
                    // side of the default value.
                    let enum_type = field
                        .type_name
                        .as_ref()
                        .and_then(|ty| ty.split('.').last())
                        .unwrap();

                    strip_enum_prefix(&to_upper_camel(&enum_type), &enum_value)
                } else {
                    &enum_value
                };
                self.buf.push_str(stripped_prefix);
            } else {
                self.buf.push_str(&default.escape_default().to_string());
            }
        }

        self.buf.push_str("\")]\n");
        self.append_field_attributes(fq_message_name, field.name());
        self.push_indent();
        self.buf.push_str("pub ");
        self.buf.push_str(&to_snake(field.name()));
        self.buf.push_str(": ");
        if repeated {
            self.buf.push_str("::prost::alloc::vec::Vec<");
        } else if optional {
            self.buf.push_str("::core::option::Option<");
        }
        if boxed {
            self.buf.push_str("::prost::alloc::boxed::Box<");
        }
        self.buf.push_str(&ty);
        if boxed {
            self.buf.push('>');
        }
        if repeated || optional {
            self.buf.push('>');
        }
        self.buf.push_str(",\n");
    }

    fn append_map_field(
        &mut self,
        fq_message_name: &str,
        field: FieldDescriptorProto,
        key: &FieldDescriptorProto,
        value: &FieldDescriptorProto,
    ) {
        let key_ty = self.resolve_type(key, fq_message_name);
        let value_ty = self.resolve_type(value, fq_message_name);

        debug!(
            "    map field: {:?}, key type: {:?}, value type: {:?}",
            field.name(),
            key_ty,
            value_ty
        );

        self.append_doc(fq_message_name, Some(field.name()));
        self.push_indent();

        let map_type = self
            .config
            .map_type
            .get_first_field(fq_message_name, field.name())
            .copied()
            .unwrap_or_default();
        let key_tag = self.field_type_tag(key);
        let value_tag = self.map_value_type_tag(value);

        self.buf.push_str(&format!(
            "#[prost({}=\"{}, {}\", tag=\"{}\")]\n",
            map_type.annotation(),
            key_tag,
            value_tag,
            field.number()
        ));
        self.append_field_attributes(fq_message_name, field.name());
        self.push_indent();
        self.buf.push_str(&format!(
            "pub {}: {}<{}, {}>,\n",
            to_snake(field.name()),
            map_type.rust_type(),
            key_ty,
            value_ty
        ));
    }

    fn append_oneof_field(
        &mut self,
        message_name: &str,
        fq_message_name: &str,
        oneof: &OneofDescriptorProto,
        fields: &[(FieldDescriptorProto, usize)],
    ) {
        let name = format!(
            "{}::{}",
            to_snake(message_name),
            to_upper_camel(oneof.name())
        );
        self.append_doc(fq_message_name, None);
        self.push_indent();
        self.buf.push_str(&format!(
            "#[prost(oneof=\"{}\", tags=\"{}\")]\n",
            name,
            fields
                .iter()
                .map(|&(ref field, _)| field.number())
                .join(", ")
        ));
        self.append_field_attributes(fq_message_name, oneof.name());
        self.push_indent();
        self.buf.push_str(&format!(
            "pub {}: ::core::option::Option<{}>,\n",
            to_snake(oneof.name()),
            name
        ));
    }

    fn append_oneof(
        &mut self,
        fq_message_name: &str,
        oneof: OneofDescriptorProto,
        idx: i32,
        fields: Vec<(FieldDescriptorProto, usize)>,
    ) {
        self.path.push(8);
        self.path.push(idx);
        self.append_doc(fq_message_name, None);
        self.path.pop();
        self.path.pop();

        let oneof_name = format!("{}.{}", fq_message_name, oneof.name());
        self.append_type_attributes(&oneof_name);
        self.push_indent();
        self.buf
            .push_str("#[derive(Clone, PartialEq, ::prost::Oneof)]\n");
        self.push_indent();
        self.buf.push_str("pub enum ");
        self.buf.push_str(&to_upper_camel(oneof.name()));
        self.buf.push_str(" {\n");

        self.path.push(2);
        self.depth += 1;
        for (field, idx) in fields {
            let type_ = field.r#type();

            self.path.push(idx as i32);
            self.append_doc(fq_message_name, Some(field.name()));
            self.path.pop();

            self.push_indent();
            let ty_tag = self.field_type_tag(&field);
            self.buf.push_str(&format!(
                "#[prost({}, tag=\"{}\")]\n",
                ty_tag,
                field.number()
            ));
            self.append_field_attributes(&oneof_name, field.name());

            self.push_indent();
            let ty = self.resolve_type(&field, fq_message_name);

            let boxed = (type_ == Type::Message || type_ == Type::Group)
                && self
                    .message_graph
                    .is_nested(field.type_name(), fq_message_name);

            debug!(
                "    oneof: {:?}, type: {:?}, boxed: {}",
                field.name(),
                ty,
                boxed
            );

            if boxed {
                self.buf.push_str(&format!(
                    "{}(::prost::alloc::boxed::Box<{}>),\n",
                    to_upper_camel(field.name()),
                    ty
                ));
            } else {
                self.buf
                    .push_str(&format!("{}({}),\n", to_upper_camel(field.name()), ty));
            }
        }
        self.depth -= 1;
        self.path.pop();

        self.push_indent();
        self.buf.push_str("}\n");
    }

    fn location(&self) -> &Location {
        let idx = self
            .source_info
            .location
            .binary_search_by_key(&&self.path[..], |location| &location.path[..])
            .unwrap();

        &self.source_info.location[idx]
    }

    fn append_doc(&mut self, fq_name: &str, field_name: Option<&str>) {
        let append_doc = if let Some(field_name) = field_name {
            self.config
                .disable_comments
                .get_first_field(fq_name, field_name)
                .is_none()
        } else {
            self.config.disable_comments.get(fq_name).next().is_none()
        };
        if append_doc {
            Comments::from_location(self.location()).append_with_indent(self.depth, &mut self.buf)
        }
    }

    fn append_enum(&mut self, desc: EnumDescriptorProto) {
        debug!("  enum: {:?}", desc.name());

        // Skip external types.
        let enum_name = &desc.name();
        let enum_values = &desc.value;
        let fq_enum_name = format!(
            "{}{}.{}",
            if self.package.is_empty() { "" } else { "." },
            self.package,
            enum_name
        );
        if self.extern_paths.resolve_ident(&fq_enum_name).is_some() {
            return;
        }

        self.append_doc(&fq_enum_name, None);
        self.append_type_attributes(&fq_enum_name);
        self.push_indent();
        self.buf.push_str(
            "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]\n",
        );
        self.push_indent();
        self.buf.push_str("#[repr(i32)]\n");
        self.push_indent();
        self.buf.push_str("pub enum ");
        self.buf.push_str(&to_upper_camel(desc.name()));
        self.buf.push_str(" {\n");

        let mut numbers = HashSet::new();

        self.depth += 1;
        self.path.push(2);
        for (idx, value) in enum_values.iter().enumerate() {
            // Skip duplicate enum values. Protobuf allows this when the
            // 'allow_alias' option is set.
            if !numbers.insert(value.number()) {
                continue;
            }

            self.path.push(idx as i32);
            let stripped_prefix = if self.config.strip_enum_prefix {
                Some(to_upper_camel(&enum_name))
            } else {
                None
            };
            self.append_enum_value(&fq_enum_name, value, stripped_prefix);
            self.path.pop();
        }
        self.path.pop();
        self.depth -= 1;

        self.push_indent();
        self.buf.push_str("}\n");
    }

    fn append_enum_value(
        &mut self,
        fq_enum_name: &str,
        value: &EnumValueDescriptorProto,
        prefix_to_strip: Option<String>,
    ) {
        self.append_doc(fq_enum_name, Some(value.name()));
        self.append_field_attributes(fq_enum_name, &value.name());
        self.push_indent();
        let name = to_upper_camel(value.name());
        let name_unprefixed = match prefix_to_strip {
            Some(prefix) => strip_enum_prefix(&prefix, &name),
            None => &name,
        };
        self.buf.push_str(name_unprefixed);
        self.buf.push_str(" = ");
        self.buf.push_str(&value.number().to_string());
        self.buf.push_str(",\n");
    }

    fn push_service(&mut self, service: ServiceDescriptorProto) {
        let name = service.name().to_owned();
        debug!("  service: {:?}", name);

        let comments = Comments::from_location(self.location());

        self.path.push(2);
        let methods = service
            .method
            .into_iter()
            .enumerate()
            .map(|(idx, mut method)| {
                debug!("  method: {:?}", method.name());
                self.path.push(idx as i32);
                let comments = Comments::from_location(self.location());
                self.path.pop();

                let name = method.name.take().unwrap();
                let input_proto_type = method.input_type.take().unwrap();
                let output_proto_type = method.output_type.take().unwrap();
                let input_type = self.resolve_ident(&input_proto_type);
                let output_type = self.resolve_ident(&output_proto_type);
                let client_streaming = method.client_streaming();
                let server_streaming = method.server_streaming();

                Method {
                    name: to_snake(&name),
                    proto_name: name,
                    comments,
                    input_type,
                    output_type,
                    input_proto_type,
                    output_proto_type,
                    options: method.options.unwrap_or_default(),
                    client_streaming,
                    server_streaming,
                }
            })
            .collect();
        self.path.pop();

        let service = Service {
            name: to_upper_camel(&name),
            proto_name: name,
            package: self.package.clone(),
            comments,
            methods,
            options: service.options.unwrap_or_default(),
        };

        if let Some(service_generator) = self.config.service_generator.as_mut() {
            service_generator.generate(service, &mut self.buf)
        }
    }

    fn push_indent(&mut self) {
        push_indent(&mut self.buf, self.depth);
    }

    fn push_mod(&mut self, module: &str) {
        self.push_indent();
        self.buf.push_str("/// Nested message and enum types in `");
        self.buf.push_str(module);
        self.buf.push_str("`.\n");

        self.push_indent();
        self.buf.push_str("pub mod ");
        self.buf.push_str(&to_snake(module));
        self.buf.push_str(" {\n");

        self.package.push('.');
        self.package.push_str(module);

        self.depth += 1;
    }

    fn pop_mod(&mut self) {
        self.depth -= 1;

        let idx = self.package.rfind('.').unwrap();
        self.package.truncate(idx);

        self.push_indent();
        self.buf.push_str("}\n");
    }

    fn resolve_type(&self, field: &FieldDescriptorProto, fq_message_name: &str) -> String {
        match field.r#type() {
            Type::Float => String::from("f32"),
            Type::Double => String::from("f64"),
            Type::Uint32 | Type::Fixed32 => String::from("u32"),
            Type::Uint64 | Type::Fixed64 => String::from("u64"),
            Type::Int32 | Type::Sfixed32 | Type::Sint32 | Type::Enum => String::from("i32"),
            Type::Int64 | Type::Sfixed64 | Type::Sint64 => String::from("i64"),
            Type::Bool => String::from("bool"),
            Type::String => String::from("::prost::alloc::string::String"),
            Type::Bytes => self
                .config
                .bytes_type
                .get_first_field(fq_message_name, field.name())
                .copied()
                .unwrap_or_default()
                .rust_type()
                .to_owned(),
            Type::Group | Type::Message => self.resolve_ident(field.type_name()),
        }
    }

    fn resolve_ident(&self, pb_ident: &str) -> String {
        // protoc should always give fully qualified identifiers.
        assert_eq!(".", &pb_ident[..1]);

        if let Some(proto_ident) = self.extern_paths.resolve_ident(pb_ident) {
            return proto_ident;
        }

        let mut local_path = self.package.split('.').peekable();

        // If no package is specified the start of the package name will be '.'
        // and split will return an empty string ("") which breaks resolution
        // The fix to this is to ignore the first item if it is empty.
        if local_path.peek().map_or(false, |s| s.is_empty()) {
            local_path.next();
        }

        let mut ident_path = pb_ident[1..].split('.');
        let ident_type = ident_path.next_back().unwrap();
        let mut ident_path = ident_path.peekable();

        // Skip path elements in common.
        while local_path.peek().is_some() && local_path.peek() == ident_path.peek() {
            local_path.next();
            ident_path.next();
        }

        local_path
            .map(|_| "super".to_string())
            .chain(ident_path.map(to_snake))
            .chain(iter::once(to_upper_camel(ident_type)))
            .join("::")
    }

    fn field_type_tag(&self, field: &FieldDescriptorProto) -> Cow<'static, str> {
        match field.r#type() {
            Type::Float => Cow::Borrowed("float"),
            Type::Double => Cow::Borrowed("double"),
            Type::Int32 => Cow::Borrowed("int32"),
            Type::Int64 => Cow::Borrowed("int64"),
            Type::Uint32 => Cow::Borrowed("uint32"),
            Type::Uint64 => Cow::Borrowed("uint64"),
            Type::Sint32 => Cow::Borrowed("sint32"),
            Type::Sint64 => Cow::Borrowed("sint64"),
            Type::Fixed32 => Cow::Borrowed("fixed32"),
            Type::Fixed64 => Cow::Borrowed("fixed64"),
            Type::Sfixed32 => Cow::Borrowed("sfixed32"),
            Type::Sfixed64 => Cow::Borrowed("sfixed64"),
            Type::Bool => Cow::Borrowed("bool"),
            Type::String => Cow::Borrowed("string"),
            Type::Bytes => Cow::Borrowed("bytes"),
            Type::Group => Cow::Borrowed("group"),
            Type::Message => Cow::Borrowed("message"),
            Type::Enum => Cow::Owned(format!(
                "enumeration={:?}",
                self.resolve_ident(field.type_name())
            )),
        }
    }

    fn map_value_type_tag(&self, field: &FieldDescriptorProto) -> Cow<'static, str> {
        match field.r#type() {
            Type::Enum => Cow::Owned(format!(
                "enumeration({})",
                self.resolve_ident(field.type_name())
            )),
            _ => self.field_type_tag(field),
        }
    }

    fn optional(&self, field: &FieldDescriptorProto) -> bool {
        if field.proto3_optional.unwrap_or(false) {
            return true;
        }

        if field.label() != Label::Optional {
            return false;
        }

        match field.r#type() {
            Type::Message => true,
            _ => self.syntax == Syntax::Proto2,
        }
    }

    /// Returns `true` if the field options includes the `deprecated` option.
    fn deprecated(&self, field: &FieldDescriptorProto) -> bool {
        field
            .options
            .as_ref()
            .map_or(false, FieldOptions::deprecated)
    }
}

/// Returns `true` if the repeated field type can be packed.
fn can_pack(field: &FieldDescriptorProto) -> bool {
    matches!(
        field.r#type(),
        Type::Float
            | Type::Double
            | Type::Int32
            | Type::Int64
            | Type::Uint32
            | Type::Uint64
            | Type::Sint32
            | Type::Sint64
            | Type::Fixed32
            | Type::Fixed64
            | Type::Sfixed32
            | Type::Sfixed64
            | Type::Bool
            | Type::Enum
    )
}

/// Based on [`google::protobuf::UnescapeCEscapeString`][1]
/// [1]: https://github.com/google/protobuf/blob/3.3.x/src/google/protobuf/stubs/strutil.cc#L312-L322
fn unescape_c_escape_string(s: &str) -> Vec<u8> {
    let src = s.as_bytes();
    let len = src.len();
    let mut dst = Vec::new();

    let mut p = 0;

    while p < len {
        if src[p] != b'\\' {
            dst.push(src[p]);
            p += 1;
        } else {
            p += 1;
            if p == len {
                panic!(
                    "invalid c-escaped default binary value ({}): ends with '\'",
                    s
                )
            }
            match src[p] {
                b'a' => {
                    dst.push(0x07);
                    p += 1;
                }
                b'b' => {
                    dst.push(0x08);
                    p += 1;
                }
                b'f' => {
                    dst.push(0x0C);
                    p += 1;
                }
                b'n' => {
                    dst.push(0x0A);
                    p += 1;
                }
                b'r' => {
                    dst.push(0x0D);
                    p += 1;
                }
                b't' => {
                    dst.push(0x09);
                    p += 1;
                }
                b'v' => {
                    dst.push(0x0B);
                    p += 1;
                }
                b'\\' => {
                    dst.push(0x5C);
                    p += 1;
                }
                b'?' => {
                    dst.push(0x3F);
                    p += 1;
                }
                b'\'' => {
                    dst.push(0x27);
                    p += 1;
                }
                b'"' => {
                    dst.push(0x22);
                    p += 1;
                }
                b'0'..=b'7' => {
                    eprintln!("another octal: {}, offset: {}", s, &s[p..]);
                    let mut octal = 0;
                    for _ in 0..3 {
                        if p < len && src[p] >= b'0' && src[p] <= b'7' {
                            eprintln!("\toctal: {}", octal);
                            octal = octal * 8 + (src[p] - b'0');
                            p += 1;
                        } else {
                            break;
                        }
                    }
                    dst.push(octal);
                }
                b'x' | b'X' => {
                    if p + 2 > len {
                        panic!(
                            "invalid c-escaped default binary value ({}): incomplete hex value",
                            s
                        )
                    }
                    match u8::from_str_radix(&s[p + 1..p + 3], 16) {
                        Ok(b) => dst.push(b),
                        _ => panic!(
                            "invalid c-escaped default binary value ({}): invalid hex value",
                            &s[p..p + 2]
                        ),
                    }
                    p += 3;
                }
                _ => panic!(
                    "invalid c-escaped default binary value ({}): invalid escape",
                    s
                ),
            }
        }
    }
    dst
}

/// Strip an enum's type name from the prefix of an enum value.
///
/// This function assumes that both have been formatted to Rust's
/// upper camel case naming conventions.
///
/// It also tries to handle cases where the stripped name would be
/// invalid - for example, if it were to begin with a number.
fn strip_enum_prefix<'a>(prefix: &str, name: &'a str) -> &'a str {
    let stripped = name.strip_prefix(prefix).unwrap_or(name);

    // If the next character after the stripped prefix is not
    // uppercase, then it means that we didn't have a true prefix -
    // for example, "Foo" should not be stripped from "Foobar".
    if stripped
        .chars()
        .next()
        .map(char::is_uppercase)
        .unwrap_or(false)
    {
        stripped
    } else {
        name
    }
}

impl MapType {
    /// The `prost-derive` annotation type corresponding to the map type.
    fn annotation(&self) -> &'static str {
        match self {
            MapType::HashMap => "map",
            MapType::BTreeMap => "btree_map",
        }
    }

    /// The fully-qualified Rust type corresponding to the map type.
    fn rust_type(&self) -> &'static str {
        match self {
            MapType::HashMap => "::std::collections::HashMap",
            MapType::BTreeMap => "::prost::alloc::collections::BTreeMap",
        }
    }
}

impl BytesType {
    /// The `prost-derive` annotation type corresponding to the bytes type.
    fn annotation(&self) -> &'static str {
        match self {
            BytesType::Vec => "vec",
            BytesType::Bytes => "bytes",
        }
    }

    /// The fully-qualified Rust type corresponding to the bytes type.
    fn rust_type(&self) -> &'static str {
        match self {
            BytesType::Vec => "::prost::alloc::vec::Vec<u8>",
            BytesType::Bytes => "::prost::bytes::Bytes",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unescape_c_escape_string() {
        assert_eq!(
            &b"hello world"[..],
            &unescape_c_escape_string("hello world")[..]
        );

        assert_eq!(&b"\0"[..], &unescape_c_escape_string(r#"\0"#)[..]);

        assert_eq!(
            &[0o012, 0o156],
            &unescape_c_escape_string(r#"\012\156"#)[..]
        );
        assert_eq!(&[0x01, 0x02], &unescape_c_escape_string(r#"\x01\x02"#)[..]);

        assert_eq!(
            &b"\0\x01\x07\x08\x0C\n\r\t\x0B\\\'\"\xFE"[..],
            &unescape_c_escape_string(r#"\0\001\a\b\f\n\r\t\v\\\'\"\xfe"#)[..]
        );
    }

    #[test]
    fn test_strip_enum_prefix() {
        assert_eq!(strip_enum_prefix("Foo", "FooBar"), "Bar");
        assert_eq!(strip_enum_prefix("Foo", "Foobar"), "Foobar");
        assert_eq!(strip_enum_prefix("Foo", "Foo"), "Foo");
        assert_eq!(strip_enum_prefix("Foo", "Bar"), "Bar");
        assert_eq!(strip_enum_prefix("Foo", "Foo1"), "Foo1");
    }
}
