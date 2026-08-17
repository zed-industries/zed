//! This module contains the actual code generation logic

use std::fmt::{Display, Formatter};
use std::io::{Result, Write};

mod enumeration;
mod message;

pub use enumeration::generate_enum;
pub use message::generate_message;

#[derive(Debug, Clone, Copy)]
struct Indent(usize);

impl Display for Indent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.0 {
            write!(f, "    ")?;
        }
        Ok(())
    }
}

fn write_fields_array<'a, W: Write, I: Iterator<Item = &'a str>>(
    writer: &mut W,
    indent: usize,
    variants: I,
) -> Result<()> {
    writeln!(writer, "{}const FIELDS: &[&str] = &[", Indent(indent))?;
    for name in variants {
        writeln!(writer, "{}\"{}\",", Indent(indent + 1), name)?;
    }
    writeln!(writer, "{}];", Indent(indent))?;
    writeln!(writer)
}

fn write_serialize_start<W: Write>(indent: usize, rust_type: &str, writer: &mut W) -> Result<()> {
    writeln!(
        writer,
        r#"{indent}impl serde::Serialize for {rust_type} {{
{indent}    #[allow(deprecated)]
{indent}    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
{indent}    where
{indent}        S: serde::Serializer,
{indent}    {{"#,
        indent = Indent(indent),
        rust_type = rust_type
    )
}

fn write_serialize_end<W: Write>(indent: usize, writer: &mut W) -> Result<()> {
    writeln!(
        writer,
        r#"{indent}    }}
{indent}}}"#,
        indent = Indent(indent),
    )
}

fn write_deserialize_start<W: Write>(indent: usize, rust_type: &str, writer: &mut W) -> Result<()> {
    writeln!(
        writer,
        r#"{indent}impl<'de> serde::Deserialize<'de> for {rust_type} {{
{indent}    #[allow(deprecated)]
{indent}    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
{indent}    where
{indent}        D: serde::Deserializer<'de>,
{indent}    {{"#,
        indent = Indent(indent),
        rust_type = rust_type
    )
}

fn write_deserialize_end<W: Write>(indent: usize, writer: &mut W) -> Result<()> {
    writeln!(
        writer,
        r#"{indent}    }}
{indent}}}"#,
        indent = Indent(indent),
    )
}
