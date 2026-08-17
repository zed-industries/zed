//! This module contains the code to generate Serialize and Deserialize
//! implementations for message types
//!
//! The implementation follows the proto3 [JSON mapping][1] with the default options
//!
//! Importantly:
//! - numeric types can be decoded from either a string or number
//! - 32-bit integers and floats are encoded as numbers
//! - 64-bit integers are encoded as strings
//! - repeated fields are encoded as arrays
//! - bytes are base64 encoded
//! - messages and maps are encoded as objects
//! - fields are lowerCamelCase except where overridden by the proto definition
//! - default values are not emitted on encode
//! - unrecognised fields error on decode
//!
//! Note: This will not generate code to correctly serialize/deserialize well-known-types
//! such as google.protobuf.Any, google.protobuf.Duration, etc... conversions for these
//! special-cased messages will need to be manually implemented. Once done so, however,
//! any messages containing these types will serialize/deserialize correctly
//!
//! [1]: https://developers.google.com/protocol-buffers/docs/proto3#json

use std::io::{Result, Write};

use crate::message::{Field, FieldModifier, FieldType, Message, OneOf, ScalarType};

use super::{
    write_deserialize_end, write_deserialize_start, write_serialize_end, write_serialize_start,
    Indent,
};
use crate::descriptor::TypePath;
use crate::escape::escape_type;
use crate::generator::write_fields_array;
use crate::resolver::Resolver;

pub fn generate_message<W: Write>(
    resolver: &Resolver<'_>,
    message: &Message,
    writer: &mut W,
    ignore_unknown_fields: bool,
    btree_map_paths: &[String],
    emit_fields: bool,
    preserve_proto_field_names: bool,
) -> Result<()> {
    let rust_type = resolver.rust_type(&message.path);

    // Generate Serialize
    write_serialize_start(0, &rust_type, writer)?;
    write_message_serialize(
        resolver,
        2,
        message,
        writer,
        emit_fields,
        preserve_proto_field_names,
    )?;
    write_serialize_end(0, writer)?;

    // Generate Deserialize
    write_deserialize_start(0, &rust_type, writer)?;
    write_deserialize_message(
        resolver,
        2,
        message,
        &rust_type,
        writer,
        ignore_unknown_fields,
        btree_map_paths,
    )?;
    write_deserialize_end(0, writer)?;
    Ok(())
}

fn write_field_empty_predicate<W: Write>(
    member: &Field,
    writer: &mut W,
    emit_fields: bool,
) -> Result<()> {
    if emit_fields {
        return write!(writer, "true");
    }

    match (&member.field_type, &member.field_modifier) {
        (_, FieldModifier::Required) => unreachable!(),
        (_, FieldModifier::Repeated)
        | (FieldType::Map(_, _), _)
        | (FieldType::Scalar(ScalarType::String), FieldModifier::UseDefault)
        | (FieldType::Scalar(ScalarType::Bytes), FieldModifier::UseDefault) => {
            write!(writer, "!self.{}.is_empty()", member.rust_field_name())
        }
        (_, FieldModifier::Optional) | (FieldType::Message(_), _) => {
            write!(writer, "self.{}.is_some()", member.rust_field_name())
        }
        (FieldType::Scalar(ScalarType::F64), FieldModifier::UseDefault)
        | (FieldType::Scalar(ScalarType::F32), FieldModifier::UseDefault) => {
            write!(writer, "self.{} != 0.", member.rust_field_name())
        }
        (FieldType::Scalar(ScalarType::Bool), FieldModifier::UseDefault) => {
            write!(writer, "self.{}", member.rust_field_name())
        }
        (FieldType::Enum(_), FieldModifier::UseDefault)
        | (FieldType::Scalar(ScalarType::I64), FieldModifier::UseDefault)
        | (FieldType::Scalar(ScalarType::I32), FieldModifier::UseDefault)
        | (FieldType::Scalar(ScalarType::U32), FieldModifier::UseDefault)
        | (FieldType::Scalar(ScalarType::U64), FieldModifier::UseDefault) => {
            write!(writer, "self.{} != 0", member.rust_field_name())
        }
    }
}

fn write_message_serialize<W: Write>(
    resolver: &Resolver<'_>,
    indent: usize,
    message: &Message,
    writer: &mut W,
    emit_fields: bool,
    preserve_proto_field_names: bool,
) -> Result<()> {
    write_struct_serialize_start(indent, message, writer, emit_fields)?;

    for field in &message.fields {
        write_serialize_field(
            resolver,
            indent,
            field,
            writer,
            emit_fields,
            preserve_proto_field_names,
        )?;
    }

    for one_of in &message.one_ofs {
        write_serialize_one_of(indent, resolver, one_of, writer, preserve_proto_field_names)?;
    }

    write_struct_serialize_end(indent, writer)
}

fn write_struct_serialize_start<W: Write>(
    indent: usize,
    message: &Message,
    writer: &mut W,
    emit_fields: bool,
) -> Result<()> {
    writeln!(writer, "{}use serde::ser::SerializeStruct;", Indent(indent))?;

    let required_len = message
        .fields
        .iter()
        .filter(|member| member.field_modifier.is_required())
        .count();

    if required_len != message.fields.len() || !message.one_ofs.is_empty() {
        writeln!(writer, "{}let mut len = {};", Indent(indent), required_len)?;
    } else {
        writeln!(writer, "{}let len = {};", Indent(indent), required_len)?;
    }

    for field in &message.fields {
        if field.field_modifier.is_required() {
            continue;
        }
        write!(writer, "{}if ", Indent(indent))?;
        write_field_empty_predicate(field, writer, emit_fields)?;
        writeln!(writer, " {{")?;
        writeln!(writer, "{}len += 1;", Indent(indent + 1))?;
        writeln!(writer, "{}}}", Indent(indent))?;
    }

    for one_of in &message.one_ofs {
        writeln!(
            writer,
            "{}if self.{}.is_some() {{",
            Indent(indent),
            one_of.rust_field_name()
        )?;
        writeln!(writer, "{}len += 1;", Indent(indent + 1))?;
        writeln!(writer, "{}}}", Indent(indent))?;
    }

    if !message.fields.is_empty() || !message.one_ofs.is_empty() {
        writeln!(
            writer,
            "{}let mut struct_ser = serializer.serialize_struct(\"{}\", len)?;",
            Indent(indent),
            message.path
        )?;
    } else {
        writeln!(
            writer,
            "{}let struct_ser = serializer.serialize_struct(\"{}\", len)?;",
            Indent(indent),
            message.path
        )?;
    }
    Ok(())
}

fn write_struct_serialize_end<W: Write>(indent: usize, writer: &mut W) -> Result<()> {
    writeln!(writer, "{}struct_ser.end()", Indent(indent))
}

fn write_decode_variant<W: Write>(
    resolver: &Resolver<'_>,
    indent: usize,
    value: &str,
    path: &TypePath,
    writer: &mut W,
) -> Result<()> {
    writeln!(writer, "{}::try_from({})", resolver.rust_type(path), value)?;
    write!(
        writer,
        "{}.map_err(|_| serde::ser::Error::custom(format!(\"Invalid variant {{}}\", {})))",
        Indent(indent),
        value
    )
}

/// Depending on the type of the field different ways of accessing field's value
/// are needed - this allows decoupling the type serialization logic from the logic
/// that manipulates its container e.g. Vec, Option, HashMap
struct Variable<'a> {
    /// A reference to the field's value
    as_ref: &'a str,
    /// The field's value
    as_unref: &'a str,
    /// The field without any leading "&" or "*"
    raw: &'a str,
}

fn write_serialize_variable<W: Write>(
    resolver: &Resolver<'_>,
    indent: usize,
    field: &Field,
    variable: Variable<'_>,
    writer: &mut W,
    preserve_proto_field_names: bool,
) -> Result<()> {
    let json_name = field.json_name();
    let field_name = if preserve_proto_field_names {
        field.name.as_str()
    } else {
        json_name.as_str()
    };
    match &field.field_type {
        FieldType::Scalar(scalar) => write_serialize_scalar_variable(
            indent,
            *scalar,
            field.field_modifier,
            variable,
            field_name,
            writer,
        ),
        FieldType::Enum(path) => {
            write!(writer, "{}let v = ", Indent(indent))?;
            match field.field_modifier {
                FieldModifier::Repeated => {
                    writeln!(writer, "{}.iter().cloned().map(|v| {{", variable.raw)?;
                    write!(writer, "{}", Indent(indent + 1))?;
                    write_decode_variant(resolver, indent + 2, "v", path, writer)?;
                    writeln!(writer)?;
                    write!(
                        writer,
                        "{}}}).collect::<Result<Vec<_>, _>>()",
                        Indent(indent + 1)
                    )
                }
                _ => write_decode_variant(resolver, indent + 1, variable.as_unref, path, writer),
            }?;

            writeln!(writer, "?;")?;
            writeln!(
                writer,
                "{}struct_ser.serialize_field(\"{}\", &v)?;",
                Indent(indent),
                field_name,
            )
        }
        FieldType::Map(_, value_type)
            if matches!(
                value_type.as_ref(),
                FieldType::Scalar(ScalarType::I64)
                    | FieldType::Scalar(ScalarType::U64)
                    | FieldType::Scalar(ScalarType::Bytes)
                    | FieldType::Enum(_)
            ) =>
        {
            writeln!(
                writer,
                "{}let v: std::collections::HashMap<_, _> = {}.iter()",
                Indent(indent),
                variable.raw
            )?;

            match value_type.as_ref() {
                FieldType::Scalar(ScalarType::I64) | FieldType::Scalar(ScalarType::U64) => {
                    writeln!(
                        writer,
                        "{}.map(|(k, v)| (k, v.to_string())).collect();",
                        Indent(indent + 1)
                    )?;
                }
                FieldType::Scalar(ScalarType::Bytes) => {
                    writeln!(
                        writer,
                        "{}.map(|(k, v)| (k, pbjson::private::base64::encode(v))).collect();",
                        Indent(indent + 1)
                    )?;
                }
                FieldType::Enum(path) => {
                    writeln!(writer, "{}.map(|(k, v)| {{", Indent(indent + 1))?;
                    write!(writer, "{}let v = ", Indent(indent + 2))?;
                    write_decode_variant(resolver, indent + 3, "*v", path, writer)?;
                    writeln!(writer, "?;")?;
                    writeln!(writer, "{}Ok((k, v))", Indent(indent + 2))?;
                    writeln!(
                        writer,
                        "{}}}).collect::<Result<_,_>>()?;",
                        Indent(indent + 1)
                    )?;
                }
                _ => unreachable!(),
            }
            writeln!(
                writer,
                "{}struct_ser.serialize_field(\"{}\", &v)?;",
                Indent(indent),
                field_name,
            )
        }
        _ => {
            writeln!(
                writer,
                "{}struct_ser.serialize_field(\"{}\", {})?;",
                Indent(indent),
                field_name,
                variable.as_ref
            )
        }
    }
}

fn write_serialize_scalar_variable<W: Write>(
    indent: usize,
    scalar: ScalarType,
    field_modifier: FieldModifier,
    variable: Variable<'_>,
    field_name: &str,
    writer: &mut W,
) -> Result<()> {
    let conversion = match scalar {
        ScalarType::I64 | ScalarType::U64 => "ToString::to_string",
        ScalarType::Bytes => "pbjson::private::base64::encode",
        _ => {
            return writeln!(
                writer,
                "{}struct_ser.serialize_field(\"{}\", {})?;",
                Indent(indent),
                field_name,
                variable.as_ref
            );
        }
    };

    match field_modifier {
        FieldModifier::Repeated => {
            writeln!(
                writer,
                "{}struct_ser.serialize_field(\"{}\", &{}.iter().map({}).collect::<Vec<_>>())?;",
                Indent(indent),
                field_name,
                variable.raw,
                conversion
            )
        }
        _ => {
            writeln!(
                writer,
                "{}#[allow(clippy::needless_borrow)]",
                Indent(indent)
            )?;
            writeln!(
                writer,
                "{}struct_ser.serialize_field(\"{}\", {}(&{}).as_str())?;",
                Indent(indent),
                field_name,
                conversion,
                variable.raw,
            )
        }
    }
}

fn write_serialize_field<W: Write>(
    resolver: &Resolver<'_>,
    indent: usize,
    field: &Field,
    writer: &mut W,
    emit_fields: bool,
    preserve_proto_field_names: bool,
) -> Result<()> {
    let as_ref = format!("&self.{}", field.rust_field_name());
    let variable = Variable {
        as_ref: as_ref.as_str(),
        as_unref: &as_ref.as_str()[1..],
        raw: &as_ref.as_str()[1..],
    };

    match &field.field_modifier {
        FieldModifier::Required => {
            write_serialize_variable(
                resolver,
                indent,
                field,
                variable,
                writer,
                preserve_proto_field_names,
            )?;
        }
        FieldModifier::Optional => {
            writeln!(
                writer,
                "{}if let Some(v) = {}.as_ref() {{",
                Indent(indent),
                variable.as_unref
            )?;
            let variable = Variable {
                as_ref: "v",
                as_unref: "*v",
                raw: "v",
            };
            write_serialize_variable(
                resolver,
                indent + 1,
                field,
                variable,
                writer,
                preserve_proto_field_names,
            )?;
            writeln!(writer, "{}}}", Indent(indent))?;
        }
        FieldModifier::Repeated | FieldModifier::UseDefault => {
            write!(writer, "{}if ", Indent(indent))?;
            write_field_empty_predicate(field, writer, emit_fields)?;
            writeln!(writer, " {{")?;
            write_serialize_variable(
                resolver,
                indent + 1,
                field,
                variable,
                writer,
                preserve_proto_field_names,
            )?;
            writeln!(writer, "{}}}", Indent(indent))?;
        }
    }
    Ok(())
}

fn write_serialize_one_of<W: Write>(
    indent: usize,
    resolver: &Resolver<'_>,
    one_of: &OneOf,
    writer: &mut W,
    preserve_proto_field_names: bool,
) -> Result<()> {
    writeln!(
        writer,
        "{}if let Some(v) = self.{}.as_ref() {{",
        Indent(indent),
        one_of.rust_field_name()
    )?;

    writeln!(writer, "{}match v {{", Indent(indent + 1))?;
    for field in &one_of.fields {
        writeln!(
            writer,
            "{}{}::{}(v) => {{",
            Indent(indent + 2),
            resolver.rust_type(&one_of.path),
            field.rust_type_name(),
        )?;
        let variable = Variable {
            as_ref: "v",
            as_unref: "*v",
            raw: "v",
        };
        write_serialize_variable(
            resolver,
            indent + 3,
            field,
            variable,
            writer,
            preserve_proto_field_names,
        )?;
        writeln!(writer, "{}}}", Indent(indent + 2))?;
    }

    writeln!(writer, "{}}}", Indent(indent + 1),)?;
    writeln!(writer, "{}}}", Indent(indent))
}

fn write_deserialize_message<W: Write>(
    resolver: &Resolver<'_>,
    indent: usize,
    message: &Message,
    rust_type: &str,
    writer: &mut W,
    ignore_unknown_fields: bool,
    btree_map_paths: &[String],
) -> Result<()> {
    write_deserialize_field_name(2, message, writer, ignore_unknown_fields)?;

    writeln!(writer, "{}struct GeneratedVisitor;", Indent(indent))?;

    writeln!(
        writer,
        r#"{indent}impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {{
{indent}    type Value = {rust_type};

{indent}    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
{indent}        formatter.write_str("struct {name}")
{indent}    }}

{indent}    fn visit_map<V>(self, mut map_: V) -> std::result::Result<{rust_type}, V::Error>
{indent}        where
{indent}            V: serde::de::MapAccess<'de>,
{indent}    {{"#,
        indent = Indent(indent),
        name = message.path,
        rust_type = rust_type,
    )?;

    for field in &message.fields {
        writeln!(
            writer,
            "{}let mut {}__ = None;",
            Indent(indent + 2),
            field.rust_field_name(),
        )?;
    }

    for one_of in &message.one_ofs {
        writeln!(
            writer,
            "{}let mut {}__ = None;",
            Indent(indent + 2),
            one_of.rust_field_name(),
        )?;
    }

    if !message.fields.is_empty() || !message.one_ofs.is_empty() {
        writeln!(
            writer,
            "{}while let Some(k) = map_.next_key()? {{",
            Indent(indent + 2)
        )?;

        writeln!(writer, "{}match k {{", Indent(indent + 3))?;

        let btree_map = btree_map_paths
            .iter()
            .any(|prefix| message.path.prefix_match(prefix.as_ref()).is_some());

        for field in &message.fields {
            write_deserialize_field(resolver, indent + 4, field, None, btree_map, writer)?;
        }

        for one_of in &message.one_ofs {
            for field in &one_of.fields {
                write_deserialize_field(
                    resolver,
                    indent + 4,
                    field,
                    Some(one_of),
                    btree_map,
                    writer,
                )?;
            }
        }

        if ignore_unknown_fields {
            writeln!(
                writer,
                "{}GeneratedField::__SkipField__ => {{",
                Indent(indent + 4),
            )?;
            writeln!(
                writer,
                "{}let _ = map_.next_value::<serde::de::IgnoredAny>()?;",
                Indent(indent + 5),
            )?;
            writeln!(writer, "{}}}", Indent(indent + 4))?;
        }

        writeln!(writer, "{}}}", Indent(indent + 3))?;
        writeln!(writer, "{}}}", Indent(indent + 2))?;
    } else {
        writeln!(
            writer,
            "{}while map_.next_key::<GeneratedField>()?.is_some() {{",
            Indent(indent + 2)
        )?;
        writeln!(
            writer,
            "{}let _ = map_.next_value::<serde::de::IgnoredAny>()?;",
            Indent(indent + 3)
        )?;
        writeln!(writer, "{}}}", Indent(indent + 2))?;
    }

    writeln!(writer, "{}Ok({} {{", Indent(indent + 2), rust_type)?;
    for field in &message.fields {
        match field.field_modifier {
            FieldModifier::Required => {
                writeln!(
                    writer,
                    "{indent}{field}: {field}__.ok_or_else(|| serde::de::Error::missing_field(\"{json_name}\"))?,",
                    indent = Indent(indent + 3),
                    field = field.rust_field_name(),
                    json_name = field.json_name()
                )?;
            }
            FieldModifier::UseDefault | FieldModifier::Repeated => {
                // Note: this currently does not hydrate optional proto2 fields with defaults
                writeln!(
                    writer,
                    "{indent}{field}: {field}__.unwrap_or_default(),",
                    indent = Indent(indent + 3),
                    field = field.rust_field_name()
                )?;
            }
            _ => {
                writeln!(
                    writer,
                    "{indent}{field}: {field}__,",
                    indent = Indent(indent + 3),
                    field = field.rust_field_name()
                )?;
            }
        }
    }
    for one_of in &message.one_ofs {
        writeln!(
            writer,
            "{indent}{field}: {field}__,",
            indent = Indent(indent + 3),
            field = one_of.rust_field_name(),
        )?;
    }

    writeln!(writer, "{}}})", Indent(indent + 2))?;
    writeln!(writer, "{}}}", Indent(indent + 1))?;
    writeln!(writer, "{}}}", Indent(indent))?;
    writeln!(
        writer,
        "{}deserializer.deserialize_struct(\"{}\", FIELDS, GeneratedVisitor)",
        Indent(indent),
        message.path
    )
}

fn write_deserialize_field_name<W: Write>(
    indent: usize,
    message: &Message,
    writer: &mut W,
    ignore_unknown_fields: bool,
) -> Result<()> {
    let fields: Vec<_> = message
        .all_fields()
        .map(|field| {
            let json_name = field.json_name();
            // only carry the original proto name if it's different from the provided json name
            let proto_name =
                Some(field.name.as_str()).filter(|proto_name| proto_name != &json_name);
            (json_name, field.rust_type_name(), proto_name)
        })
        .collect();

    write_fields_array(
        writer,
        indent,
        fields.iter().flat_map(|(json_name, _, proto_name)| {
            proto_name.iter().copied().chain([json_name.as_str()])
        }),
    )?;
    write_fields_enum(
        writer,
        indent,
        fields.iter().map(|(_, type_name, _)| type_name.as_str()),
        ignore_unknown_fields,
    )?;

    writeln!(
        writer,
        r#"{indent}impl<'de> serde::Deserialize<'de> for GeneratedField {{
{indent}    fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
{indent}    where
{indent}        D: serde::Deserializer<'de>,
{indent}    {{
{indent}        struct GeneratedVisitor;

{indent}        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {{
{indent}            type Value = GeneratedField;

{indent}            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
{indent}                write!(formatter, "expected one of: {{:?}}", &FIELDS)
{indent}            }}

{indent}            #[allow(unused_variables)]
{indent}            fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
{indent}            where
{indent}                E: serde::de::Error,
{indent}            {{"#,
        indent = Indent(indent)
    )?;

    if !fields.is_empty() {
        writeln!(writer, "{}match value {{", Indent(indent + 4))?;
        for (json_name, type_name, proto_name) in &fields {
            if let Some(proto_name) = proto_name {
                writeln!(
                    writer,
                    "{}\"{}\" | \"{}\" => Ok(GeneratedField::{}),",
                    Indent(indent + 5),
                    json_name,
                    proto_name,
                    escape_type(type_name.to_string())
                )?;
            } else {
                writeln!(
                    writer,
                    "{}\"{}\" => Ok(GeneratedField::{}),",
                    Indent(indent + 5),
                    json_name,
                    escape_type(type_name.to_string())
                )?;
            }
        }
        if ignore_unknown_fields {
            writeln!(
                writer,
                "{}_ => Ok(GeneratedField::__SkipField__),",
                Indent(indent + 5)
            )?;
        } else {
            writeln!(
                writer,
                "{}_ => Err(serde::de::Error::unknown_field(value, FIELDS)),",
                Indent(indent + 5)
            )?;
        }
        writeln!(writer, "{}}}", Indent(indent + 4))?;
    } else if ignore_unknown_fields {
        writeln!(
            writer,
            "{}Ok(GeneratedField::__SkipField__)",
            Indent(indent + 5)
        )?;
    } else {
        writeln!(
            writer,
            "{}Err(serde::de::Error::unknown_field(value, FIELDS))",
            Indent(indent + 5)
        )?;
    }

    writeln!(
        writer,
        r#"{indent}            }}
{indent}        }}
{indent}        deserializer.deserialize_identifier(GeneratedVisitor)
{indent}    }}
{indent}}}"#,
        indent = Indent(indent)
    )
}

fn write_fields_enum<'a, W: Write, I: Iterator<Item = &'a str>>(
    writer: &mut W,
    indent: usize,
    fields: I,
    ignore_unknown_fields: bool,
) -> Result<()> {
    writeln!(
        writer,
        "{}#[allow(clippy::enum_variant_names)]",
        Indent(indent)
    )?;
    writeln!(writer, "{}enum GeneratedField {{", Indent(indent))?;
    for type_name in fields {
        writeln!(
            writer,
            "{}{},",
            Indent(indent + 1),
            escape_type(type_name.to_string())
        )?;
    }

    if ignore_unknown_fields {
        writeln!(writer, "{}__SkipField__,", Indent(indent + 1))?;
    }

    writeln!(writer, "{}}}", Indent(indent))
}

fn write_deserialize_field<W: Write>(
    resolver: &Resolver<'_>,
    indent: usize,
    field: &Field,
    one_of: Option<&OneOf>,
    btree_map: bool,
    writer: &mut W,
) -> Result<()> {
    let field_name = match one_of {
        Some(one_of) => one_of.rust_field_name(),
        None => field.rust_field_name(),
    };

    let json_name = field.json_name();
    writeln!(
        writer,
        "{}GeneratedField::{} => {{",
        Indent(indent),
        field.rust_type_name()
    )?;
    writeln!(
        writer,
        "{}if {}__.is_some() {{",
        Indent(indent + 1),
        field_name
    )?;

    // Note: this will report duplicate field if multiple value are specified for a one of
    writeln!(
        writer,
        "{}return Err(serde::de::Error::duplicate_field(\"{}\"));",
        Indent(indent + 2),
        json_name
    )?;
    writeln!(writer, "{}}}", Indent(indent + 1))?;
    write!(writer, "{}{}__ = ", Indent(indent + 1), field_name)?;

    match one_of {
        Some(one_of) => match &field.field_type {
            FieldType::Scalar(s) => match override_deserializer(*s) {
                Some(deserializer) => {
                    write!(
                        writer,
                        "map_.next_value::<::std::option::Option<{}>>()?.map(|x| {}::{}(x.0))",
                        deserializer,
                        resolver.rust_type(&one_of.path),
                        field.rust_type_name()
                    )?;
                }
                None => {
                    write!(
                        writer,
                        "map_.next_value::<::std::option::Option<_>>()?.map({}::{})",
                        resolver.rust_type(&one_of.path),
                        field.rust_type_name()
                    )?;
                }
            },
            FieldType::Enum(path) => {
                write!(
                    writer,
                    "map_.next_value::<::std::option::Option<{}>>()?.map(|x| {}::{}(x as i32))",
                    resolver.rust_type(path),
                    resolver.rust_type(&one_of.path),
                    field.rust_type_name()
                )?;
            }
            FieldType::Message(_) => writeln!(
                writer,
                "map_.next_value::<::std::option::Option<_>>()?.map({}::{})",
                resolver.rust_type(&one_of.path),
                field.rust_type_name()
            )?,
            FieldType::Map(_, _) => unreachable!("one of cannot contain map fields"),
        },

        None => match &field.field_type {
            FieldType::Scalar(scalar) => {
                write_encode_scalar_field(indent + 1, *scalar, field.field_modifier, writer)?;
            }
            FieldType::Enum(path) => match field.field_modifier {
                FieldModifier::Optional => {
                    write!(
                        writer,
                        "map_.next_value::<::std::option::Option<{}>>()?.map(|x| x as i32)",
                        resolver.rust_type(path)
                    )?;
                }
                FieldModifier::Repeated => {
                    write!(
                        writer,
                        "Some(map_.next_value::<Vec<{}>>()?.into_iter().map(|x| x as i32).collect())",
                        resolver.rust_type(path)
                    )?;
                }
                _ => {
                    write!(
                        writer,
                        "Some(map_.next_value::<{}>()? as i32)",
                        resolver.rust_type(path)
                    )?;
                }
            },
            FieldType::Map(key, value) => {
                write!(writer, "Some(")?;
                writeln!(writer)?;
                match btree_map {
                    true => write!(
                        writer,
                        "{}map_.next_value::<std::collections::BTreeMap<",
                        Indent(indent + 2),
                    )?,
                    false => write!(
                        writer,
                        "{}map_.next_value::<std::collections::HashMap<",
                        Indent(indent + 2),
                    )?,
                }

                let map_k = match key {
                    ScalarType::Bytes | ScalarType::F32 | ScalarType::F64 => {
                        panic!("protobuf disallows maps with floating point or bytes keys")
                    }
                    _ if key.is_numeric() => {
                        write!(
                            writer,
                            "::pbjson::private::NumberDeserialize<{}>",
                            key.rust_type()
                        )?;
                        "k.0"
                    }
                    _ => {
                        write!(writer, "_")?;
                        "k"
                    }
                };
                write!(writer, ", ")?;
                let map_v = match value.as_ref() {
                    FieldType::Scalar(scalar) if scalar.is_numeric() => {
                        write!(
                            writer,
                            "::pbjson::private::NumberDeserialize<{}>",
                            scalar.rust_type()
                        )?;
                        "v.0"
                    }
                    FieldType::Scalar(ScalarType::Bytes) => {
                        write!(writer, "::pbjson::private::BytesDeserialize<_>",)?;
                        "v.0"
                    }
                    FieldType::Enum(path) => {
                        write!(writer, "{}", resolver.rust_type(path))?;
                        "v as i32"
                    }
                    FieldType::Map(_, _) => panic!("protobuf disallows nested maps"),
                    _ => {
                        write!(writer, "_")?;
                        "v"
                    }
                };

                writeln!(writer, ">>()?")?;
                if map_k != "k" || map_v != "v" {
                    writeln!(
                        writer,
                        "{}.into_iter().map(|(k,v)| ({}, {})).collect()",
                        Indent(indent + 3),
                        map_k,
                        map_v,
                    )?;
                }
                write!(writer, "{})", Indent(indent + 1))?;
            }
            FieldType::Message(_) => match field.field_modifier {
                FieldModifier::Repeated => {
                    // No explicit presence for repeated fields
                    write!(writer, "Some(map_.next_value()?)")?;
                }
                _ => write!(writer, "map_.next_value()?")?,
            },
        },
    }
    writeln!(writer, ";")?;
    writeln!(writer, "{}}}", Indent(indent))
}

fn override_deserializer(scalar: ScalarType) -> Option<&'static str> {
    match scalar {
        ScalarType::Bytes => Some("::pbjson::private::BytesDeserialize<_>"),
        _ if scalar.is_numeric() => Some("::pbjson::private::NumberDeserialize<_>"),
        _ => None,
    }
}

fn write_encode_scalar_field<W: Write>(
    indent: usize,
    scalar: ScalarType,
    field_modifier: FieldModifier,
    writer: &mut W,
) -> Result<()> {
    let deserializer = match override_deserializer(scalar) {
        Some(deserializer) => deserializer,
        None => {
            return match field_modifier {
                FieldModifier::Optional => {
                    write!(writer, "map_.next_value()?")
                }
                _ => write!(writer, "Some(map_.next_value()?)"),
            };
        }
    };

    writeln!(writer)?;

    match field_modifier {
        FieldModifier::Optional => {
            writeln!(
                writer,
                "{}map_.next_value::<::std::option::Option<{}>>()?.map(|x| x.0)",
                Indent(indent + 1),
                deserializer
            )?;
        }
        FieldModifier::Repeated => {
            writeln!(
                writer,
                "{}Some(map_.next_value::<Vec<{}>>()?",
                Indent(indent + 1),
                deserializer
            )?;
            writeln!(
                writer,
                "{}.into_iter().map(|x| x.0).collect())",
                Indent(indent + 2)
            )?;
        }
        _ => {
            writeln!(
                writer,
                "{}Some(map_.next_value::<{}>()?.0)",
                Indent(indent + 1),
                deserializer
            )?;
        }
    }
    write!(writer, "{}", Indent(indent))
}
