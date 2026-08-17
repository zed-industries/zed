/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! XML Encoding module that uses Rust lifetimes to make
//! generating malformed XML a compile error

use crate::escape::escape;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter, Write};

// currently there's actually no way that encoding can fail but give it time :-)
#[non_exhaustive]
#[derive(Debug)]
pub struct XmlEncodeError {}

impl Display for XmlEncodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "error encoding XML")
    }
}

impl StdError for XmlEncodeError {}

/// XmlWriter Abstraction
///
/// XmlWriter (and friends) make generating an invalid XML document a type error. Nested branches
/// of the Xml document mutable borrow from the root. You cannot continue writing to the root
/// until the nested branch is dropped and dropping the nested branch writes the terminator (e.g.
/// closing element).
///
/// The one exception to this rule is names—it is possible to construct an invalid Xml Name. However,
/// names are always known ahead of time and always static, so this would be obvious from the code.
///
/// Furthermore, once `const panic` stabilizes, we'll be able to make an invalid XmlName a compiler
/// error.
///
/// # Examples
/// ```rust
/// use aws_smithy_xml::encode::XmlWriter;
/// let mut s = String::new();
/// let mut doc = XmlWriter::new(&mut s);
/// let mut start_el = doc.start_el("Root")
///     .write_ns("http://example.com", None);
/// let mut start_tag = start_el.finish();
/// start_tag.data("hello");
/// start_tag.finish();
/// assert_eq!(s, "<Root xmlns=\"http://example.com\">hello</Root>");
/// ```
///
/// See `tests/handwritten_serializers.rs` for more usage examples.
pub struct XmlWriter<'a> {
    doc: &'a mut String,
}

impl<'a> XmlWriter<'a> {
    pub fn new(doc: &'a mut String) -> Self {
        Self { doc }
    }
}

impl XmlWriter<'_> {
    pub fn start_el<'b, 'c>(&'c mut self, tag: &'b str) -> ElWriter<'c, 'b> {
        write!(self.doc, "<{tag}").unwrap();
        ElWriter::new(self.doc, tag)
    }
}

pub struct ElWriter<'a, 'b> {
    start: &'b str,
    doc: Option<&'a mut String>,
}

impl<'a, 'b> ElWriter<'a, 'b> {
    fn new(doc: &'a mut String, start: &'b str) -> ElWriter<'a, 'b> {
        ElWriter {
            start,
            doc: Some(doc),
        }
    }

    pub fn write_attribute(&mut self, key: &str, value: &str) -> &mut Self {
        write!(self.doc(), " {}=\"{}\"", key, escape(value)).unwrap();
        self
    }

    pub fn write_ns(mut self, namespace: &str, prefix: Option<&str>) -> Self {
        match prefix {
            Some(prefix) => {
                write!(self.doc(), " xmlns:{}=\"{}\"", prefix, escape(namespace)).unwrap()
            }
            None => write!(self.doc(), " xmlns=\"{}\"", escape(namespace)).unwrap(),
        }
        self
    }

    fn write_end(doc: &mut String) {
        write!(doc, ">").unwrap();
    }

    fn doc<'c>(&'c mut self) -> &'c mut String
    where
        'a: 'c,
    {
        // The self.doc is an Option in order to signal whether the closing '>' has been emitted
        // already (None) or not (Some). It ensures the following invariants:
        // - If finish() has been called, then self.doc is None and therefore no more writes
        //   to the &mut String are possible.
        // - When drop() is called, if self.doc is Some, then finish() has not (and will not)
        //   be called, and therefore drop() should close the tag represented by this struct.
        //
        // Since this function calls unwrap(), it must not be called from finish() or drop().
        // As finish() consumes self, calls to this method from any other method will not encounter
        // a None value in self.doc.
        self.doc.as_mut().unwrap()
    }

    pub fn finish(mut self) -> ScopeWriter<'a, 'b> {
        let doc = self.doc.take().unwrap();
        Self::write_end(doc);
        ScopeWriter {
            doc,
            start: self.start,
        }
    }
}

impl Drop for ElWriter<'_, '_> {
    fn drop(&mut self) {
        if let Some(doc) = self.doc.take() {
            // Calls to write_end() are always preceded by self.doc.take(). The value in self.doc
            // is set to Some initially, and is never reset to Some after being taken. Since this
            // transition to None happens only once, we will never double-close the XML element.
            Self::write_end(doc);
        }
    }
}

/// Wrap the construction of a tag pair `<a></a>`
pub struct ScopeWriter<'a, 'b> {
    doc: &'a mut String,
    start: &'b str,
}

impl Drop for ScopeWriter<'_, '_> {
    fn drop(&mut self) {
        write!(self.doc, "</{}>", self.start).unwrap();
    }
}

impl ScopeWriter<'_, '_> {
    pub fn data(&mut self, data: &str) {
        self.doc.write_str(escape(data).as_ref()).unwrap();
    }

    pub fn finish(self) {
        // drop will be called which writes the closer to the document
    }

    pub fn start_el<'b, 'c>(&'c mut self, tag: &'b str) -> ElWriter<'c, 'b> {
        write!(self.doc, "<{tag}").unwrap();
        ElWriter::new(self.doc, tag)
    }
}

#[cfg(test)]
mod test {
    use crate::encode::XmlWriter;
    use aws_smithy_protocol_test::{assert_ok, validate_body, MediaType};

    #[test]
    fn forgot_finish() {
        let mut out = String::new();

        fn writer(out: &mut String) {
            let mut doc_writer = XmlWriter::new(out);
            doc_writer.start_el("Hello");
            // We intentionally "forget" to call finish() on the ElWriter:
            // when the XML structs get dropped, the element must get closed automatically.
        }
        writer(&mut out);

        assert_ok(validate_body(out, r#"<Hello></Hello>"#, MediaType::Xml));
    }

    #[test]
    fn forgot_finish_with_attribute() {
        let mut out = String::new();

        fn writer(out: &mut String) {
            let mut doc_writer = XmlWriter::new(out);
            doc_writer.start_el("Hello").write_attribute("key", "foo");
            // We intentionally "forget" to call finish() on the ElWriter:
            // when the XML structs get dropped, the element must get closed automatically.
        }
        writer(&mut out);

        assert_ok(validate_body(
            out,
            r#"<Hello key="foo"></Hello>"#,
            MediaType::Xml,
        ));
    }

    #[test]
    fn basic_document_encoding() {
        let mut out = String::new();
        let mut doc_writer = XmlWriter::new(&mut out);
        let mut start_el = doc_writer
            .start_el("Hello")
            .write_ns("http://example.com", None);
        start_el.write_attribute("key", "foo");
        let mut tag = start_el.finish();
        let mut inner = tag.start_el("inner").finish();
        inner.data("hello world!");
        inner.finish();
        let more_inner = tag.start_el("inner").finish();
        more_inner.finish();
        tag.finish();

        assert_ok(validate_body(
            out,
            r#"<Hello key="foo" xmlns="http://example.com">
                    <inner>hello world!</inner>
                    <inner></inner>
                </Hello>"#,
            MediaType::Xml,
        ));
    }

    #[test]
    fn escape_data() {
        let mut s = String::new();
        {
            let mut doc_writer = XmlWriter::new(&mut s);
            let mut start_el = doc_writer.start_el("Hello");
            start_el.write_attribute("key", "<key=\"value\">");
            let mut tag = start_el.finish();
            tag.data("\n\r&");
        }
        assert_eq!(
            s,
            r#"<Hello key="&lt;key=&quot;value&quot;&gt;">&#xA;&#xD;&amp;</Hello>"#
        )
    }
}
