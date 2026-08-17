// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::tree_builder::NamespaceMap;
use crate::QualName;
pub use markup5ever::serialize::{AttrRef, Serialize, Serializer, TraversalScope};
use std::io::{self, Write};

#[derive(Clone)]
/// Struct for setting serializer options.
pub struct SerializeOpts {
    /// Serialize the root node? Default: ChildrenOnly
    pub traversal_scope: TraversalScope,
}

impl Default for SerializeOpts {
    fn default() -> SerializeOpts {
        SerializeOpts {
            traversal_scope: TraversalScope::ChildrenOnly(None),
        }
    }
}

/// Method for serializing generic node to a given writer.
pub fn serialize<Wr, T>(writer: Wr, node: &T, opts: SerializeOpts) -> io::Result<()>
where
    Wr: Write,
    T: Serialize,
{
    let mut ser = XmlSerializer::new(writer);
    node.serialize(&mut ser, opts.traversal_scope)
}

/// Struct used for serializing nodes into a text that other XML
/// parses can read.
///
/// Serializer contains a set of functions (start_elem, end_elem...)
/// that make parsing nodes easier.
pub struct XmlSerializer<Wr> {
    writer: Wr,
    namespace_stack: NamespaceMapStack,
}

#[derive(Debug)]
struct NamespaceMapStack(Vec<NamespaceMap>);

impl NamespaceMapStack {
    fn new() -> NamespaceMapStack {
        NamespaceMapStack(vec![])
    }

    fn push(&mut self, namespace: NamespaceMap) {
        self.0.push(namespace);
    }

    fn pop(&mut self) {
        self.0.pop();
    }
}

/// Writes given text into the Serializer, escaping it,
/// depending on where the text is written inside the tag or attribute value.
///
/// For example
///```text
///    <tag>'&-quotes'</tag>   becomes      <tag>'&amp;-quotes'</tag>
///    <tag = "'&-quotes'">    becomes      <tag = "&apos;&amp;-quotes&apos;"
///```
fn write_to_buf_escaped<W: Write>(writer: &mut W, text: &str, attr_mode: bool) -> io::Result<()> {
    for c in text.chars() {
        match c {
            '&' => writer.write_all(b"&amp;"),
            '\'' if attr_mode => writer.write_all(b"&apos;"),
            '"' if attr_mode => writer.write_all(b"&quot;"),
            '<' if !attr_mode => writer.write_all(b"&lt;"),
            '>' if !attr_mode => writer.write_all(b"&gt;"),
            c => writer.write_fmt(format_args!("{}", c)),
        }?;
    }
    Ok(())
}

#[inline]
fn write_qual_name<W: Write>(writer: &mut W, name: &QualName) -> io::Result<()> {
    if let Some(ref prefix) = name.prefix {
        writer.write_all(prefix.as_bytes())?;
        writer.write_all(b":")?;
    }

    writer.write_all(name.local.as_bytes())?;
    Ok(())
}

impl<Wr: Write> XmlSerializer<Wr> {
    /// Creates a new Serializier from a writer and given serialization options.
    pub fn new(writer: Wr) -> Self {
        XmlSerializer {
            writer,
            namespace_stack: NamespaceMapStack::new(),
        }
    }

    #[inline(always)]
    fn qual_name(&mut self, name: &QualName) -> io::Result<()> {
        self.find_or_insert_ns(name);
        write_qual_name(&mut self.writer, name)
    }

    #[inline(always)]
    fn qual_attr_name(&mut self, name: &QualName) -> io::Result<()> {
        self.find_or_insert_ns(name);
        write_qual_name(&mut self.writer, name)
    }

    fn find_uri(&self, name: &QualName) -> bool {
        let mut found = false;
        for stack in self.namespace_stack.0.iter().rev() {
            if let Some(Some(el)) = stack.get(&name.prefix) {
                found = *el == name.ns;
                break;
            }
        }
        found
    }

    fn find_or_insert_ns(&mut self, name: &QualName) {
        if (name.prefix.is_some() || !name.ns.is_empty()) && !self.find_uri(name) {
            if let Some(last_ns) = self.namespace_stack.0.last_mut() {
                last_ns.insert(name);
            }
        }
    }
}

impl<Wr: Write> Serializer for XmlSerializer<Wr> {
    /// Serializes given start element into text. Start element contains
    /// qualified name and an attributes iterator.
    fn start_elem<'a, AttrIter>(&mut self, name: QualName, attrs: AttrIter) -> io::Result<()>
    where
        AttrIter: Iterator<Item = AttrRef<'a>>,
    {
        self.namespace_stack.push(NamespaceMap::empty());

        self.writer.write_all(b"<")?;
        self.qual_name(&name)?;
        if let Some(current_namespace) = self.namespace_stack.0.last() {
            for (prefix, url_opt) in current_namespace.get_scope_iter() {
                self.writer.write_all(b" xmlns")?;
                if let Some(ref p) = *prefix {
                    self.writer.write_all(b":")?;
                    self.writer.write_all(p.as_bytes())?;
                }

                self.writer.write_all(b"=\"")?;
                let url = if let Some(ref a) = *url_opt {
                    a.as_bytes()
                } else {
                    b""
                };
                self.writer.write_all(url)?;
                self.writer.write_all(b"\"")?;
            }
        }
        for (name, value) in attrs {
            self.writer.write_all(b" ")?;
            self.qual_attr_name(name)?;
            self.writer.write_all(b"=\"")?;
            write_to_buf_escaped(&mut self.writer, value, true)?;
            self.writer.write_all(b"\"")?;
        }
        self.writer.write_all(b">")?;
        Ok(())
    }

    /// Serializes given end element into text.
    fn end_elem(&mut self, name: QualName) -> io::Result<()> {
        self.namespace_stack.pop();
        self.writer.write_all(b"</")?;
        self.qual_name(&name)?;
        self.writer.write_all(b">")
    }

    /// Serializes comment into text.
    fn write_comment(&mut self, text: &str) -> io::Result<()> {
        self.writer.write_all(b"<!--")?;
        self.writer.write_all(text.as_bytes())?;
        self.writer.write_all(b"-->")
    }

    /// Serializes given doctype
    fn write_doctype(&mut self, name: &str) -> io::Result<()> {
        self.writer.write_all(b"<!DOCTYPE ")?;
        self.writer.write_all(name.as_bytes())?;
        self.writer.write_all(b">")
    }

    /// Serializes text for a node or an attributes.
    fn write_text(&mut self, text: &str) -> io::Result<()> {
        write_to_buf_escaped(&mut self.writer, text, false)
    }

    /// Serializes given processing instruction.
    fn write_processing_instruction(&mut self, target: &str, data: &str) -> io::Result<()> {
        self.writer.write_all(b"<?")?;
        self.writer.write_all(target.as_bytes())?;
        self.writer.write_all(b" ")?;
        self.writer.write_all(data.as_bytes())?;
        self.writer.write_all(b"?>")
    }
}
