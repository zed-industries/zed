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
use crate::ident::{strip_enum_prefix, to_snake, to_upper_camel};
use crate::message_graph::MessageGraph;
use crate::Config;

mod c_escaping;
use c_escaping::unescape_c_escape_string;

mod syntax;
use syntax::Syntax;

pub struct CodeGenerator<'a> {
    config: &'a mut Config,
    package: String,
    type_path: Vec<String>,
    source_info: Option<SourceCodeInfo>,
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

fn prost_path(config: &Config) -> &str {
    config.prost_path.as_deref().unwrap_or("::prost")
}

struct Field {
    descriptor: FieldDescriptorProto,
    path_index: i32,
}

impl Field {
    fn new(descriptor: FieldDescriptorProto, path_index: i32) -> Self {
        Self {
            descriptor,
            path_index,
        }
    }

    fn rust_name(&self) -> String {
        to_snake(self.descriptor.name())
    }
}

struct OneofField {
    descriptor: OneofDescriptorProto,
    fields: Vec<Field>,
    path_index: i32,
}

impl OneofField {
    fn new(descriptor: OneofDescriptorProto, fields: Vec<Field>, path_index: i32) -> Self {
        Self {
            descriptor,
            fields,
            path_index,
        }
    }

    fn rust_name(&self) -> String {
        to_snake(self.descriptor.name())
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
        let source_info = file.source_code_info.map(|mut s| {
            s.location.retain(|loc| {
                let len = loc.path.len();
                len > 0 && len % 2 == 0
            });
            s.location.sort_by(|a, b| a.path.cmp(&b.path));
            s
        });

        let mut code_gen = CodeGenerator {
            config,
            package: file.package.unwrap_or_default(),
            type_path: Vec::new(),
            source_info,
            syntax: file.syntax.as_deref().into(),
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
        let fq_message_name = self.fq_name(&message_name);

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
        type OneofFieldsByIndex = MultiMap<i32, Field>;
        let (fields, mut oneof_map): (Vec<Field>, OneofFieldsByIndex) = message
            .field
            .into_iter()
            .enumerate()
            .partition_map(|(idx, proto)| {
                let idx = idx as i32;
                if proto.proto3_optional.unwrap_or(false) {
                    Either::Left(Field::new(proto, idx))
                } else if let Some(oneof_index) = proto.oneof_index {
                    Either::Right((oneof_index, Field::new(proto, idx)))
                } else {
                    Either::Left(Field::new(proto, idx))
                }
            });
        // Optional fields create a synthetic oneof that we want to skip
        let oneof_fields: Vec<OneofField> = message
            .oneof_decl
            .into_iter()
            .enumerate()
            .filter_map(move |(idx, proto)| {
                let idx = idx as i32;
                oneof_map
                    .remove(&idx)
                    .map(|fields| OneofField::new(proto, fields, idx))
            })
            .collect();

        self.append_doc(&fq_message_name, None);
        self.append_type_attributes(&fq_message_name);
        self.append_message_attributes(&fq_message_name);
        self.push_indent();
        self.buf
            .push_str("#[allow(clippy::derive_partial_eq_without_eq)]\n");
        self.buf.push_str(&format!(
            "#[derive(Clone, PartialEq, {}::Message)]\n",
            prost_path(self.config)
        ));
        self.append_skip_debug(&fq_message_name);
        self.push_indent();
        self.buf.push_str("pub struct ");
        self.buf.push_str(&to_upper_camel(&message_name));
        self.buf.push_str(" {\n");

        self.depth += 1;
        self.path.push(2);
        for field in &fields {
            self.path.push(field.path_index);
            match field
                .descriptor
                .type_name
                .as_ref()
                .and_then(|type_name| map_types.get(type_name))
            {
                Some((key, value)) => self.append_map_field(&fq_message_name, field, key, value),
                None => self.append_field(&fq_message_name, field),
            }
            self.path.pop();
        }
        self.path.pop();

        self.path.push(8);
        for oneof in &oneof_fields {
            self.path.push(oneof.path_index);
            self.append_oneof_field(&message_name, &fq_message_name, oneof);
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

            for oneof in &oneof_fields {
                self.append_oneof(&fq_message_name, oneof);
            }

            self.pop_mod();
        }

        if self.config.enable_type_names {
            self.append_type_name(&message_name, &fq_message_name);
        }
    }

    fn append_type_name(&mut self, message_name: &str, fq_message_name: &str) {
        self.buf.push_str(&format!(
            "impl {}::Name for {} {{\n",
            self.config.prost_path.as_deref().unwrap_or("::prost"),
            to_upper_camel(message_name)
        ));
        self.depth += 1;

        self.buf.push_str(&format!(
            "const NAME: &'static str = \"{}\";\n",
            message_name,
        ));
        self.buf.push_str(&format!(
            "const PACKAGE: &'static str = \"{}\";\n",
            self.package,
        ));

        let prost_path = self.config.prost_path.as_deref().unwrap_or("::prost");
        let string_path = format!("{prost_path}::alloc::string::String");

        let full_name = format!(
            "{}{}{}{}{message_name}",
            self.package.trim_matches('.'),
            if self.package.is_empty() { "" } else { "." },
            self.type_path.join("."),
            if self.type_path.is_empty() { "" } else { "." },
        );
        let domain_name = self
            .config
            .type_name_domains
            .get_first(fq_message_name)
            .map_or("", |name| name.as_str());

        self.buf.push_str(&format!(
            r#"fn full_name() -> {string_path} {{ "{full_name}".into() }}"#,
        ));

        self.buf.push_str(&format!(
            r#"fn type_url() -> {string_path} {{ "{domain_name}/{full_name}".into() }}"#,
        ));

        self.depth -= 1;
        self.buf.push_str("}\n");
    }

    fn append_type_attributes(&mut self, fq_message_name: &str) {
        assert_eq!(b'.', fq_message_name.as_bytes()[0]);
        for attribute in self.config.type_attributes.get(fq_message_name) {
            push_indent(self.buf, self.depth);
            self.buf.push_str(attribute);
            self.buf.push('\n');
        }
    }

    fn append_message_attributes(&mut self, fq_message_name: &str) {
        assert_eq!(b'.', fq_message_name.as_bytes()[0]);
        for attribute in self.config.message_attributes.get(fq_message_name) {
            push_indent(self.buf, self.depth);
            self.buf.push_str(attribute);
            self.buf.push('\n');
        }
    }

    fn should_skip_debug(&self, fq_message_name: &str) -> bool {
        assert_eq!(b'.', fq_message_name.as_bytes()[0]);
        self.config.skip_debug.get(fq_message_name).next().is_some()
    }

    fn append_skip_debug(&mut self, fq_message_name: &str) {
        if self.should_skip_debug(fq_message_name) {
            push_indent(self.buf, self.depth);
            self.buf.push_str("#[prost(skip_debug)]");
            self.buf.push('\n');
        }
    }

    fn append_enum_attributes(&mut self, fq_message_name: &str) {
        assert_eq!(b'.', fq_message_name.as_bytes()[0]);
        for attribute in self.config.enum_attributes.get(fq_message_name) {
            push_indent(self.buf, self.depth);
            self.buf.push_str(attribute);
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
            push_indent(self.buf, self.depth);
            self.buf.push_str(attribute);
            self.buf.push('\n');
        }
    }

    fn append_field(&mut self, fq_message_name: &str, field: &Field) {
        let type_ = field.descriptor.r#type();
        let repeated = field.descriptor.label == Some(Label::Repeated as i32);
        let deprecated = self.deprecated(&field.descriptor);
        let optional = self.optional(&field.descriptor);
        let boxed = self.boxed(&field.descriptor, fq_message_name, None);
        let ty = self.resolve_type(&field.descriptor, fq_message_name);

        debug!(
            "    field: {:?}, type: {:?}, boxed: {}",
            field.descriptor.name(),
            ty,
            boxed
        );

        self.append_doc(fq_message_name, Some(field.descriptor.name()));

        if deprecated {
            self.push_indent();
            self.buf.push_str("#[deprecated]\n");
        }

        self.push_indent();
        self.buf.push_str("#[prost(");
        let type_tag = self.field_type_tag(&field.descriptor);
        self.buf.push_str(&type_tag);

        if type_ == Type::Bytes {
            let bytes_type = self
                .config
                .bytes_type
                .get_first_field(fq_message_name, field.descriptor.name())
                .copied()
                .unwrap_or_default();
            self.buf
                .push_str(&format!("={:?}", bytes_type.annotation()));
        }

        match field.descriptor.label() {
            Label::Optional => {
                if optional {
                    self.buf.push_str(", optional");
                }
            }
            Label::Required => self.buf.push_str(", required"),
            Label::Repeated => {
                self.buf.push_str(", repeated");
                if can_pack(&field.descriptor)
                    && !field
                        .descriptor
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
        self.buf.push_str(&field.descriptor.number().to_string());

        if let Some(ref default) = field.descriptor.default_value {
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
                let mut enum_value = to_upper_camel(default);
                if self.config.strip_enum_prefix {
                    // Field types are fully qualified, so we extract
                    // the last segment and strip it from the left
                    // side of the default value.
                    let enum_type = field
                        .descriptor
                        .type_name
                        .as_ref()
                        .and_then(|ty| ty.split('.').last())
                        .unwrap();

                    enum_value = strip_enum_prefix(&to_upper_camel(enum_type), &enum_value)
                }
                self.buf.push_str(&enum_value);
            } else {
                self.buf.push_str(&default.escape_default().to_string());
            }
        }

        self.buf.push_str("\")]\n");
        self.append_field_attributes(fq_message_name, field.descriptor.name());
        self.push_indent();
        self.buf.push_str("pub ");
        self.buf.push_str(&field.rust_name());
        self.buf.push_str(": ");

        let prost_path = prost_path(self.config);

        if repeated {
            self.buf
                .push_str(&format!("{}::alloc::vec::Vec<", prost_path));
        } else if optional {
            self.buf.push_str("::core::option::Option<");
        }
        if boxed {
            self.buf
                .push_str(&format!("{}::alloc::boxed::Box<", prost_path));
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
        field: &Field,
        key: &FieldDescriptorProto,
        value: &FieldDescriptorProto,
    ) {
        let key_ty = self.resolve_type(key, fq_message_name);
        let value_ty = self.resolve_type(value, fq_message_name);

        debug!(
            "    map field: {:?}, key type: {:?}, value type: {:?}",
            field.descriptor.name(),
            key_ty,
            value_ty
        );

        self.append_doc(fq_message_name, Some(field.descriptor.name()));
        self.push_indent();

        let map_type = self
            .config
            .map_type
            .get_first_field(fq_message_name, field.descriptor.name())
            .copied()
            .unwrap_or_default();
        let key_tag = self.field_type_tag(key);
        let value_tag = self.map_value_type_tag(value);

        self.buf.push_str(&format!(
            "#[prost({}=\"{}, {}\", tag=\"{}\")]\n",
            map_type.annotation(),
            key_tag,
            value_tag,
            field.descriptor.number()
        ));
        self.append_field_attributes(fq_message_name, field.descriptor.name());
        self.push_indent();
        self.buf.push_str(&format!(
            "pub {}: {}<{}, {}>,\n",
            field.rust_name(),
            map_type.rust_type(),
            key_ty,
            value_ty
        ));
    }

    fn append_oneof_field(
        &mut self,
        message_name: &str,
        fq_message_name: &str,
        oneof: &OneofField,
    ) {
        let type_name = format!(
            "{}::{}",
            to_snake(message_name),
            to_upper_camel(oneof.descriptor.name())
        );
        self.append_doc(fq_message_name, None);
        self.push_indent();
        self.buf.push_str(&format!(
            "#[prost(oneof=\"{}\", tags=\"{}\")]\n",
            type_name,
            oneof
                .fields
                .iter()
                .map(|field| field.descriptor.number())
                .join(", "),
        ));
        self.append_field_attributes(fq_message_name, oneof.descriptor.name());
        self.push_indent();
        self.buf.push_str(&format!(
            "pub {}: ::core::option::Option<{}>,\n",
            oneof.rust_name(),
            type_name
        ));
    }

    fn append_oneof(&mut self, fq_message_name: &str, oneof: &OneofField) {
        self.path.push(8);
        self.path.push(oneof.path_index);
        self.append_doc(fq_message_name, None);
        self.path.pop();
        self.path.pop();

        let oneof_name = format!("{}.{}", fq_message_name, oneof.descriptor.name());
        self.append_type_attributes(&oneof_name);
        self.append_enum_attributes(&oneof_name);
        self.push_indent();
        self.buf
            .push_str("#[allow(clippy::derive_partial_eq_without_eq)]\n");
        self.buf.push_str(&format!(
            "#[derive(Clone, PartialEq, {}::Oneof)]\n",
            prost_path(self.config)
        ));
        self.append_skip_debug(fq_message_name);
        self.push_indent();
        self.buf.push_str("pub enum ");
        self.buf.push_str(&to_upper_camel(oneof.descriptor.name()));
        self.buf.push_str(" {\n");

        self.path.push(2);
        self.depth += 1;
        for field in &oneof.fields {
            self.path.push(field.path_index);
            self.append_doc(fq_message_name, Some(field.descriptor.name()));
            self.path.pop();

            self.push_indent();
            let ty_tag = self.field_type_tag(&field.descriptor);
            self.buf.push_str(&format!(
                "#[prost({}, tag=\"{}\")]\n",
                ty_tag,
                field.descriptor.number()
            ));
            self.append_field_attributes(&oneof_name, field.descriptor.name());

            self.push_indent();
            let ty = self.resolve_type(&field.descriptor, fq_message_name);

            let boxed = self.boxed(
                &field.descriptor,
                fq_message_name,
                Some(oneof.descriptor.name()),
            );

            debug!(
                "    oneof: {:?}, type: {:?}, boxed: {}",
                field.descriptor.name(),
                ty,
                boxed
            );

            if boxed {
                self.buf.push_str(&format!(
                    "{}(::prost::alloc::boxed::Box<{}>),\n",
                    to_upper_camel(field.descriptor.name()),
                    ty
                ));
            } else {
                self.buf.push_str(&format!(
                    "{}({}),\n",
                    to_upper_camel(field.descriptor.name()),
                    ty
                ));
            }
        }
        self.depth -= 1;
        self.path.pop();

        self.push_indent();
        self.buf.push_str("}\n");
    }

    fn location(&self) -> Option<&Location> {
        let source_info = self.source_info.as_ref()?;
        let idx = source_info
            .location
            .binary_search_by_key(&&self.path[..], |location| &location.path[..])
            .unwrap();
        Some(&source_info.location[idx])
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
            if let Some(comments) = self.location().map(Comments::from_location) {
                comments.append_with_indent(self.depth, self.buf);
            }
        }
    }

    fn append_enum(&mut self, desc: EnumDescriptorProto) {
        debug!("  enum: {:?}", desc.name());

        let proto_enum_name = desc.name();
        let enum_name = to_upper_camel(proto_enum_name);

        let enum_values = &desc.value;
        let fq_proto_enum_name = self.fq_name(proto_enum_name);

        if self
            .extern_paths
            .resolve_ident(&fq_proto_enum_name)
            .is_some()
        {
            return;
        }

        self.append_doc(&fq_proto_enum_name, None);
        self.append_type_attributes(&fq_proto_enum_name);
        self.append_enum_attributes(&fq_proto_enum_name);
        self.push_indent();
        let dbg = if self.should_skip_debug(&fq_proto_enum_name) {
            ""
        } else {
            "Debug, "
        };
        self.buf.push_str(&format!(
            "#[derive(Clone, Copy, {}PartialEq, Eq, Hash, PartialOrd, Ord, {}::Enumeration)]\n",
            dbg,
            prost_path(self.config),
        ));
        self.push_indent();
        self.buf.push_str("#[repr(i32)]\n");
        self.push_indent();
        self.buf.push_str("pub enum ");
        self.buf.push_str(&enum_name);
        self.buf.push_str(" {\n");

        let variant_mappings =
            build_enum_value_mappings(&enum_name, self.config.strip_enum_prefix, enum_values);

        self.depth += 1;
        self.path.push(2);
        for variant in variant_mappings.iter() {
            self.path.push(variant.path_idx as i32);

            self.append_doc(&fq_proto_enum_name, Some(variant.proto_name));
            self.append_field_attributes(&fq_proto_enum_name, variant.proto_name);
            self.push_indent();
            self.buf.push_str(&variant.generated_variant_name);
            self.buf.push_str(" = ");
            self.buf.push_str(&variant.proto_number.to_string());
            self.buf.push_str(",\n");

            self.path.pop();
        }

        self.path.pop();
        self.depth -= 1;

        self.push_indent();
        self.buf.push_str("}\n");

        self.push_indent();
        self.buf.push_str("impl ");
        self.buf.push_str(&enum_name);
        self.buf.push_str(" {\n");
        self.depth += 1;
        self.path.push(2);

        self.push_indent();
        self.buf.push_str(
            "/// String value of the enum field names used in the ProtoBuf definition.\n",
        );
        self.push_indent();
        self.buf.push_str("///\n");
        self.push_indent();
        self.buf.push_str(
            "/// The values are not transformed in any way and thus are considered stable\n",
        );
        self.push_indent();
        self.buf.push_str(
            "/// (if the ProtoBuf definition does not change) and safe for programmatic use.\n",
        );
        self.push_indent();
        self.buf
            .push_str("pub fn as_str_name(&self) -> &'static str {\n");
        self.depth += 1;

        self.push_indent();
        self.buf.push_str("match self {\n");
        self.depth += 1;

        for variant in variant_mappings.iter() {
            self.push_indent();
            self.buf.push_str(&enum_name);
            self.buf.push_str("::");
            self.buf.push_str(&variant.generated_variant_name);
            self.buf.push_str(" => \"");
            self.buf.push_str(variant.proto_name);
            self.buf.push_str("\",\n");
        }

        self.depth -= 1;
        self.push_indent();
        self.buf.push_str("}\n"); // End of match

        self.depth -= 1;
        self.push_indent();
        self.buf.push_str("}\n"); // End of as_str_name()

        self.push_indent();
        self.buf
            .push_str("/// Creates an enum from field names used in the ProtoBuf definition.\n");

        self.push_indent();
        self.buf
            .push_str("pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {\n");
        self.depth += 1;

        self.push_indent();
        self.buf.push_str("match value {\n");
        self.depth += 1;

        for variant in variant_mappings.iter() {
            self.push_indent();
            self.buf.push('\"');
            self.buf.push_str(variant.proto_name);
            self.buf.push_str("\" => Some(Self::");
            self.buf.push_str(&variant.generated_variant_name);
            self.buf.push_str("),\n");
        }
        self.push_indent();
        self.buf.push_str("_ => None,\n");

        self.depth -= 1;
        self.push_indent();
        self.buf.push_str("}\n"); // End of match

        self.depth -= 1;
        self.push_indent();
        self.buf.push_str("}\n"); // End of from_str_name()

        self.path.pop();
        self.depth -= 1;
        self.push_indent();
        self.buf.push_str("}\n"); // End of impl
    }

    fn push_service(&mut self, service: ServiceDescriptorProto) {
        let name = service.name().to_owned();
        debug!("  service: {:?}", name);

        let comments = self
            .location()
            .map(Comments::from_location)
            .unwrap_or_default();

        self.path.push(2);
        let methods = service
            .method
            .into_iter()
            .enumerate()
            .map(|(idx, mut method)| {
                debug!("  method: {:?}", method.name());

                self.path.push(idx as i32);
                let comments = self
                    .location()
                    .map(Comments::from_location)
                    .unwrap_or_default();
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
            service_generator.generate(service, self.buf)
        }
    }

    fn push_indent(&mut self) {
        push_indent(self.buf, self.depth);
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

        self.type_path.push(module.into());

        self.depth += 1;
    }

    fn pop_mod(&mut self) {
        self.depth -= 1;

        self.type_path.pop();

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
            Type::String => format!("{}::alloc::string::String", prost_path(self.config)),
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

        let mut local_path = self
            .package
            .split('.')
            .chain(self.type_path.iter().map(String::as_str))
            .peekable();

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

    /// Returns whether the Rust type for this field needs to be `Box<_>`.
    ///
    /// This can be explicitly configured with `Config::boxed`, or necessary
    /// to prevent an infinitely sized type definition in case when the type of
    /// a non-repeated message field transitively contains the message itself.
    fn boxed(
        &self,
        field: &FieldDescriptorProto,
        fq_message_name: &str,
        oneof: Option<&str>,
    ) -> bool {
        let repeated = field.label == Some(Label::Repeated as i32);
        let fd_type = field.r#type();
        if !repeated
            && (fd_type == Type::Message || fd_type == Type::Group)
            && self
                .message_graph
                .is_nested(field.type_name(), fq_message_name)
        {
            return true;
        }
        let config_path = match oneof {
            None => Cow::Borrowed(fq_message_name),
            Some(ooname) => Cow::Owned(format!("{fq_message_name}.{ooname}")),
        };
        if self
            .config
            .boxed
            .get_first_field(&config_path, field.name())
            .is_some()
        {
            if repeated {
                println!(
                    "cargo:warning=\
                    Field X is repeated and manually marked as boxed. \
                    This is deprecated and support will be removed in a later release"
                );
            }
            return true;
        }
        false
    }

    /// Returns `true` if the field options includes the `deprecated` option.
    fn deprecated(&self, field: &FieldDescriptorProto) -> bool {
        field
            .options
            .as_ref()
            .map_or(false, FieldOptions::deprecated)
    }

    /// Returns the fully-qualified name, starting with a dot
    fn fq_name(&self, message_name: &str) -> String {
        format!(
            "{}{}{}{}.{}",
            if self.package.is_empty() { "" } else { "." },
            self.package.trim_matches('.'),
            if self.type_path.is_empty() { "" } else { "." },
            self.type_path.join("."),
            message_name,
        )
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

struct EnumVariantMapping<'a> {
    path_idx: usize,
    proto_name: &'a str,
    proto_number: i32,
    generated_variant_name: String,
}

fn build_enum_value_mappings<'a>(
    generated_enum_name: &str,
    do_strip_enum_prefix: bool,
    enum_values: &'a [EnumValueDescriptorProto],
) -> Vec<EnumVariantMapping<'a>> {
    let mut numbers = HashSet::new();
    let mut generated_names = HashMap::new();
    let mut mappings = Vec::new();

    for (idx, value) in enum_values.iter().enumerate() {
        // Skip duplicate enum values. Protobuf allows this when the
        // 'allow_alias' option is set.
        if !numbers.insert(value.number()) {
            continue;
        }

        let mut generated_variant_name = to_upper_camel(value.name());
        if do_strip_enum_prefix {
            generated_variant_name =
                strip_enum_prefix(generated_enum_name, &generated_variant_name);
        }

        if let Some(old_v) = generated_names.insert(generated_variant_name.to_owned(), value.name())
        {
            panic!("Generated enum variant names overlap: `{}` variant name to be used both by `{}` and `{}` ProtoBuf enum values",
                generated_variant_name, old_v, value.name());
        }

        mappings.push(EnumVariantMapping {
            path_idx: idx,
            proto_name: value.name(),
            proto_number: value.number(),
            generated_variant_name,
        })
    }
    mappings
}
