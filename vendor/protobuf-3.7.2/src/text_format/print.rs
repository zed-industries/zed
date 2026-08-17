use std::fmt;
use std::fmt::Write;

use protobuf_support::text_format::quote_bytes_to;

use crate::message_dyn::MessageDyn;
use crate::reflect::MessageRef;
use crate::reflect::ReflectFieldRef;
use crate::reflect::ReflectValueRef;
use crate::UnknownValueRef;

fn print_str_to(s: &str, buf: &mut String) {
    // TODO: keep printable Unicode
    quote_bytes_to(s.as_bytes(), buf);
}

fn do_indent(buf: &mut String, pretty: bool, indent: usize) {
    if pretty && indent > 0 {
        for _ in 0..indent {
            buf.push_str("  ");
        }
    }
}

trait FieldName: fmt::Display {}
impl<'a> FieldName for &'a str {}
impl FieldName for u32 {}

fn print_start_field<F: FieldName>(
    buf: &mut String,
    pretty: bool,
    indent: usize,
    first: &mut bool,
    field_name: F,
) {
    if !*first && !pretty {
        buf.push_str(" ");
    }
    do_indent(buf, pretty, indent);
    *first = false;
    write!(buf, "{}", field_name).unwrap();
}

fn print_end_field(buf: &mut String, pretty: bool) {
    if pretty {
        buf.push_str("\n");
    }
}

fn print_field<F: FieldName>(
    buf: &mut String,
    pretty: bool,
    indent: usize,
    first: &mut bool,
    field_name: F,
    value: ReflectValueRef,
) {
    print_start_field(buf, pretty, indent, first, field_name);

    match value {
        ReflectValueRef::Message(m) => {
            buf.push_str(" {");
            if pretty {
                buf.push_str("\n");
            }
            print_to_internal(&m, buf, pretty, indent + 1);
            do_indent(buf, pretty, indent);
            buf.push_str("}");
        }
        ReflectValueRef::Enum(d, v) => {
            buf.push_str(": ");
            match d.value_by_number(v) {
                Some(e) => buf.push_str(e.name()),
                None => write!(buf, ": {}", v).unwrap(),
            }
        }
        ReflectValueRef::String(s) => {
            buf.push_str(": ");
            print_str_to(s, buf);
        }
        ReflectValueRef::Bytes(b) => {
            buf.push_str(": ");
            quote_bytes_to(b, buf);
        }
        ReflectValueRef::I32(v) => {
            write!(buf, ": {}", v).unwrap();
        }
        ReflectValueRef::I64(v) => {
            write!(buf, ": {}", v).unwrap();
        }
        ReflectValueRef::U32(v) => {
            write!(buf, ": {}", v).unwrap();
        }
        ReflectValueRef::U64(v) => {
            write!(buf, ": {}", v).unwrap();
        }
        ReflectValueRef::Bool(v) => {
            write!(buf, ": {}", v).unwrap();
        }
        ReflectValueRef::F32(v) => {
            write!(buf, ": {}", v).unwrap();
        }
        ReflectValueRef::F64(v) => {
            write!(buf, ": {}", v).unwrap();
        }
    }

    print_end_field(buf, pretty);
}

fn print_to_internal(m: &MessageRef, buf: &mut String, pretty: bool, indent: usize) {
    let d = m.descriptor_dyn();
    let mut first = true;
    for f in d.fields() {
        match f.get_reflect(&**m) {
            ReflectFieldRef::Map(map) => {
                for (k, v) in &map {
                    print_start_field(buf, pretty, indent, &mut first, f.name());
                    buf.push_str(" {");
                    if pretty {
                        buf.push_str("\n");
                    }

                    let mut entry_first = true;

                    print_field(buf, pretty, indent + 1, &mut entry_first, "key", k);
                    print_field(buf, pretty, indent + 1, &mut entry_first, "value", v);
                    do_indent(buf, pretty, indent);
                    buf.push_str("}");
                    print_end_field(buf, pretty);
                }
            }
            ReflectFieldRef::Repeated(repeated) => {
                for v in repeated {
                    print_field(buf, pretty, indent, &mut first, f.name(), v);
                }
            }
            ReflectFieldRef::Optional(optional) => {
                if let Some(v) = optional.value() {
                    print_field(buf, pretty, indent, &mut first, f.name(), v);
                }
            }
        }
    }

    let mut fields: Vec<(u32, UnknownValueRef)> = m.unknown_fields_dyn().iter().collect();
    // Sort for stable output
    fields.sort_by_key(|(field_number, _)| *field_number);
    for (field_number, value) in fields {
        // TODO: try decode nested message for length-delimited
        print_field(
            buf,
            pretty,
            indent,
            &mut first,
            field_number,
            value.to_reflect_value_ref(),
        );
    }
}

/// Text-format
pub fn print_to(m: &dyn MessageDyn, buf: &mut String) {
    print_to_internal(&MessageRef::from(m), buf, false, 0)
}

fn print_to_string_internal(m: &dyn MessageDyn, pretty: bool) -> String {
    let mut r = String::new();
    print_to_internal(&MessageRef::from(m), &mut r, pretty, 0);
    r
}

/// Text-format
pub fn print_to_string(m: &dyn MessageDyn) -> String {
    print_to_string_internal(m, false)
}

/// Text-format
pub fn print_to_string_pretty(m: &dyn MessageDyn) -> String {
    print_to_string_internal(m, true)
}

/// Text-format to `fmt::Formatter`.
pub fn fmt(m: &dyn MessageDyn, f: &mut fmt::Formatter) -> fmt::Result {
    let pretty = f.alternate();
    f.write_str(&print_to_string_internal(m, pretty))
}
