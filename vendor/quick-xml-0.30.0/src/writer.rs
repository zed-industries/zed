//! Contains high-level interface for an events-based XML emitter.

use std::io::Write;

use crate::encoding::UTF8_BOM;
use crate::errors::Result;
use crate::events::{attributes::Attribute, BytesCData, BytesStart, BytesText, Event};

#[cfg(feature = "async-tokio")]
mod async_tokio;

/// XML writer. Writes XML [`Event`]s to a [`std::io::Write`] or [`tokio::io::AsyncWrite`] implementor.
#[cfg(feature = "serialize")]
use {crate::de::DeError, serde::Serialize};

/// XML writer. Writes XML [`Event`]s to a [`std::io::Write`] implementor.
///
/// # Examples
///
/// ```
/// # use pretty_assertions::assert_eq;
/// use quick_xml::events::{Event, BytesEnd, BytesStart};
/// use quick_xml::reader::Reader;
/// use quick_xml::writer::Writer;
/// use std::io::Cursor;
///
/// let xml = r#"<this_tag k1="v1" k2="v2"><child>text</child></this_tag>"#;
/// let mut reader = Reader::from_str(xml);
/// reader.trim_text(true);
/// let mut writer = Writer::new(Cursor::new(Vec::new()));
/// loop {
///     match reader.read_event() {
///         Ok(Event::Start(e)) if e.name().as_ref() == b"this_tag" => {
///
///             // crates a new element ... alternatively we could reuse `e` by calling
///             // `e.into_owned()`
///             let mut elem = BytesStart::new("my_elem");
///
///             // collect existing attributes
///             elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()));
///
///             // copy existing attributes, adds a new my-key="some value" attribute
///             elem.push_attribute(("my-key", "some value"));
///
///             // writes the event to the writer
///             assert!(writer.write_event(Event::Start(elem)).is_ok());
///         },
///         Ok(Event::End(e)) if e.name().as_ref() == b"this_tag" => {
///             assert!(writer.write_event(Event::End(BytesEnd::new("my_elem"))).is_ok());
///         },
///         Ok(Event::Eof) => break,
///         // we can either move or borrow the event to write, depending on your use-case
///         Ok(e) => assert!(writer.write_event(e).is_ok()),
///         Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
///     }
/// }
///
/// let result = writer.into_inner().into_inner();
/// let expected = r#"<my_elem k1="v1" k2="v2" my-key="some value"><child>text</child></my_elem>"#;
/// assert_eq!(result, expected.as_bytes());
/// ```
#[derive(Clone)]
pub struct Writer<W> {
    /// underlying writer
    writer: W,
    indent: Option<Indentation>,
}

impl<W> Writer<W> {
    /// Creates a `Writer` from a generic writer.
    pub fn new(inner: W) -> Writer<W> {
        Writer {
            writer: inner,
            indent: None,
        }
    }

    /// Creates a `Writer` with configured indents from a generic writer.
    pub fn new_with_indent(inner: W, indent_char: u8, indent_size: usize) -> Writer<W> {
        Writer {
            writer: inner,
            indent: Some(Indentation::new(indent_char, indent_size)),
        }
    }

    /// Consumes this `Writer`, returning the underlying writer.
    pub fn into_inner(self) -> W {
        self.writer
    }

    /// Get a mutable reference to the underlying writer.
    pub fn get_mut(&mut self) -> &mut W {
        &mut self.writer
    }

    /// Get a reference to the underlying writer.
    pub fn get_ref(&self) -> &W {
        &self.writer
    }
}

impl<W: Write> Writer<W> {
    /// Write a [Byte-Order-Mark] character to the document.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use quick_xml::Result;
    /// # fn main() -> Result<()> {
    /// use quick_xml::events::{BytesStart, BytesText, Event};
    /// use quick_xml::writer::Writer;
    /// use quick_xml::Error;
    /// use std::io::Cursor;
    ///
    /// let mut buffer = Vec::new();
    /// let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);
    ///
    /// writer.write_bom()?;
    /// writer
    ///     .create_element("empty")
    ///     .with_attribute(("attr1", "value1"))
    ///     .write_empty()
    ///     .expect("failure");
    ///
    /// assert_eq!(
    ///     std::str::from_utf8(&buffer).unwrap(),
    ///     "\u{FEFF}<empty attr1=\"value1\"/>"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    /// [Byte-Order-Mark]: https://unicode.org/faq/utf_bom.html#BOM
    pub fn write_bom(&mut self) -> Result<()> {
        self.write(UTF8_BOM)
    }

    /// Writes the given event to the underlying writer.
    pub fn write_event<'a, E: AsRef<Event<'a>>>(&mut self, event: E) -> Result<()> {
        let mut next_should_line_break = true;
        let result = match *event.as_ref() {
            Event::Start(ref e) => {
                let result = self.write_wrapped(b"<", e, b">");
                if let Some(i) = self.indent.as_mut() {
                    i.grow();
                }
                result
            }
            Event::End(ref e) => {
                if let Some(i) = self.indent.as_mut() {
                    i.shrink();
                }
                self.write_wrapped(b"</", e, b">")
            }
            Event::Empty(ref e) => self.write_wrapped(b"<", e, b"/>"),
            Event::Text(ref e) => {
                next_should_line_break = false;
                self.write(e)
            }
            Event::Comment(ref e) => self.write_wrapped(b"<!--", e, b"-->"),
            Event::CData(ref e) => {
                next_should_line_break = false;
                self.write(b"<![CDATA[")?;
                self.write(e)?;
                self.write(b"]]>")
            }
            Event::Decl(ref e) => self.write_wrapped(b"<?", e, b"?>"),
            Event::PI(ref e) => self.write_wrapped(b"<?", e, b"?>"),
            Event::DocType(ref e) => self.write_wrapped(b"<!DOCTYPE ", e, b">"),
            Event::Eof => Ok(()),
        };
        if let Some(i) = self.indent.as_mut() {
            i.should_line_break = next_should_line_break;
        }
        result
    }

    /// Writes bytes
    #[inline]
    pub(crate) fn write(&mut self, value: &[u8]) -> Result<()> {
        self.writer.write_all(value).map_err(Into::into)
    }

    #[inline]
    fn write_wrapped(&mut self, before: &[u8], value: &[u8], after: &[u8]) -> Result<()> {
        if let Some(ref i) = self.indent {
            if i.should_line_break {
                self.writer.write_all(b"\n")?;
                self.writer.write_all(i.current())?;
            }
        }
        self.write(before)?;
        self.write(value)?;
        self.write(after)?;
        Ok(())
    }

    /// Manually write a newline and indentation at the proper level.
    ///
    /// This can be used when the heuristic to line break and indent after any
    /// [`Event`] apart from [`Text`] fails such as when a [`Start`] occurs directly
    /// after [`Text`].
    ///
    /// This method will do nothing if `Writer` was not constructed with [`new_with_indent`].
    ///
    /// [`Text`]: Event::Text
    /// [`Start`]: Event::Start
    /// [`new_with_indent`]: Self::new_with_indent
    pub fn write_indent(&mut self) -> Result<()> {
        if let Some(ref i) = self.indent {
            self.writer.write_all(b"\n")?;
            self.writer.write_all(i.current())?;
        }
        Ok(())
    }

    /// Provides a simple, high-level API for writing XML elements.
    ///
    /// Returns an [`ElementWriter`] that simplifies setting attributes and writing
    /// content inside the element.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use quick_xml::Result;
    /// # fn main() -> Result<()> {
    /// use quick_xml::events::{BytesStart, BytesText, Event};
    /// use quick_xml::writer::Writer;
    /// use quick_xml::Error;
    /// use std::io::Cursor;
    ///
    /// let mut writer = Writer::new(Cursor::new(Vec::new()));
    ///
    /// // writes <tag attr1="value1"/>
    /// writer.create_element("tag")
    ///     .with_attribute(("attr1", "value1"))  // chain `with_attribute()` calls to add many attributes
    ///     .write_empty()?;
    ///
    /// // writes <tag attr1="value1" attr2="value2">with some text inside</tag>
    /// writer.create_element("tag")
    ///     .with_attributes(vec![("attr1", "value1"), ("attr2", "value2")].into_iter())  // or add attributes from an iterator
    ///     .write_text_content(BytesText::new("with some text inside"))?;
    ///
    /// // writes <tag><fruit quantity="0">apple</fruit><fruit quantity="1">orange</fruit></tag>
    /// writer.create_element("tag")
    ///     .write_inner_content(|writer| {
    ///         let fruits = ["apple", "orange"];
    ///         for (quant, item) in fruits.iter().enumerate() {
    ///             writer
    ///                 .create_element("fruit")
    ///                 .with_attribute(("quantity", quant.to_string().as_str()))
    ///                 .write_text_content(BytesText::new(item))?;
    ///         }
    ///         Ok(())
    ///     })?;
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn create_element<'a, N>(&'a mut self, name: &'a N) -> ElementWriter<W>
    where
        N: 'a + AsRef<str> + ?Sized,
    {
        ElementWriter {
            writer: self,
            start_tag: BytesStart::new(name.as_ref()),
        }
    }

    /// Write an arbitrary serializable type
    ///
    /// Note: If you are attempting to write XML in a non-UTF-8 encoding, this may not
    /// be safe to use. Rust basic types assume UTF-8 encodings.
    ///
    /// ```rust
    /// # use pretty_assertions::assert_eq;
    /// # use serde::Serialize;
    /// # use quick_xml::events::{BytesStart, Event};
    /// # use quick_xml::writer::Writer;
    /// # use quick_xml::DeError;
    /// # fn main() -> Result<(), DeError> {
    /// #[derive(Debug, PartialEq, Serialize)]
    /// struct MyData {
    ///     question: String,
    ///     answer: u32,
    /// }
    ///
    /// let data = MyData {
    ///     question: "The Ultimate Question of Life, the Universe, and Everything".into(),
    ///     answer: 42,
    /// };
    ///
    /// let mut buffer = Vec::new();
    /// let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);
    ///
    /// let start = BytesStart::new("root");
    /// let end = start.to_end();
    ///
    /// writer.write_event(Event::Start(start.clone()))?;
    /// writer.write_serializable("my_data", &data)?;
    /// writer.write_event(Event::End(end))?;
    ///
    /// assert_eq!(
    ///     std::str::from_utf8(&buffer)?,
    ///     r#"<root>
    ///     <my_data>
    ///         <question>The Ultimate Question of Life, the Universe, and Everything</question>
    ///         <answer>42</answer>
    ///     </my_data>
    /// </root>"#
    /// );
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "serialize")]
    pub fn write_serializable<T: Serialize>(
        &mut self,
        tag_name: &str,
        content: &T,
    ) -> std::result::Result<(), DeError> {
        use crate::se::{Indent, Serializer};

        self.write_indent()?;
        let mut fmt = ToFmtWrite(&mut self.writer);
        let mut serializer = Serializer::with_root(&mut fmt, Some(tag_name))?;

        if let Some(indent) = &mut self.indent {
            serializer.set_indent(Indent::Borrow(indent));
        }

        content.serialize(serializer)?;

        Ok(())
    }
}

/// A struct to write an element. Contains methods to add attributes and inner
/// elements to the element
pub struct ElementWriter<'a, W: Write> {
    writer: &'a mut Writer<W>,
    start_tag: BytesStart<'a>,
}

impl<'a, W: Write> ElementWriter<'a, W> {
    /// Adds an attribute to this element.
    pub fn with_attribute<'b, I>(mut self, attr: I) -> Self
    where
        I: Into<Attribute<'b>>,
    {
        self.start_tag.push_attribute(attr);
        self
    }

    /// Add additional attributes to this element using an iterator.
    ///
    /// The yielded items must be convertible to [`Attribute`] using `Into`.
    pub fn with_attributes<'b, I>(mut self, attributes: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Attribute<'b>>,
    {
        self.start_tag.extend_attributes(attributes);
        self
    }

    /// Write some text inside the current element.
    pub fn write_text_content(self, text: BytesText) -> Result<&'a mut Writer<W>> {
        self.writer
            .write_event(Event::Start(self.start_tag.borrow()))?;
        self.writer.write_event(Event::Text(text))?;
        self.writer
            .write_event(Event::End(self.start_tag.to_end()))?;
        Ok(self.writer)
    }

    /// Write a CData event `<![CDATA[...]]>` inside the current element.
    pub fn write_cdata_content(self, text: BytesCData) -> Result<&'a mut Writer<W>> {
        self.writer
            .write_event(Event::Start(self.start_tag.borrow()))?;
        self.writer.write_event(Event::CData(text))?;
        self.writer
            .write_event(Event::End(self.start_tag.to_end()))?;
        Ok(self.writer)
    }

    /// Write a processing instruction `<?...?>` inside the current element.
    pub fn write_pi_content(self, text: BytesText) -> Result<&'a mut Writer<W>> {
        self.writer
            .write_event(Event::Start(self.start_tag.borrow()))?;
        self.writer.write_event(Event::PI(text))?;
        self.writer
            .write_event(Event::End(self.start_tag.to_end()))?;
        Ok(self.writer)
    }

    /// Write an empty (self-closing) tag.
    pub fn write_empty(self) -> Result<&'a mut Writer<W>> {
        self.writer.write_event(Event::Empty(self.start_tag))?;
        Ok(self.writer)
    }

    /// Create a new scope for writing XML inside the current element.
    pub fn write_inner_content<F>(self, closure: F) -> Result<&'a mut Writer<W>>
    where
        F: FnOnce(&mut Writer<W>) -> Result<()>,
    {
        self.writer
            .write_event(Event::Start(self.start_tag.borrow()))?;
        closure(self.writer)?;
        self.writer
            .write_event(Event::End(self.start_tag.to_end()))?;
        Ok(self.writer)
    }
}
#[cfg(feature = "serialize")]
struct ToFmtWrite<T>(pub T);

#[cfg(feature = "serialize")]
impl<T> std::fmt::Write for ToFmtWrite<T>
where
    T: std::io::Write,
{
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.write_all(s.as_bytes()).map_err(|_| std::fmt::Error)
    }
}

#[derive(Clone)]
pub(crate) struct Indentation {
    /// todo: this is an awkward fit as it has no impact on indentation logic, but it is
    /// only applicable when an indentation exists. Potentially refactor later
    should_line_break: bool,
    /// The character code to be used for indentations (e.g. ` ` or `\t`)
    indent_char: u8,
    /// How many instances of the indent character ought to be used for each level of indentation
    indent_size: usize,
    /// Used as a cache for the bytes used for indentation
    indents: Vec<u8>,
    /// The current amount of indentation
    current_indent_len: usize,
}

impl Indentation {
    pub fn new(indent_char: u8, indent_size: usize) -> Self {
        Self {
            should_line_break: false,
            indent_char,
            indent_size,
            indents: vec![indent_char; 128],
            current_indent_len: 0, // invariant - needs to remain less than indents.len()
        }
    }

    /// Increase indentation by one level
    pub fn grow(&mut self) {
        self.current_indent_len += self.indent_size;
        if self.current_indent_len > self.indents.len() {
            self.indents
                .resize(self.current_indent_len, self.indent_char);
        }
    }

    /// Decrease indentation by one level. Do nothing, if level already zero
    pub fn shrink(&mut self) {
        self.current_indent_len = self.current_indent_len.saturating_sub(self.indent_size);
    }

    /// Returns indent string for current level
    pub fn current(&self) -> &[u8] {
        &self.indents[..self.current_indent_len]
    }
}

#[cfg(test)]
mod indentation {
    use super::*;
    use crate::events::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn self_closed() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        let tag = BytesStart::new("self-closed")
            .with_attributes(vec![("attr1", "value1"), ("attr2", "value2")].into_iter());
        writer
            .write_event(Event::Empty(tag))
            .expect("write tag failed");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<self-closed attr1="value1" attr2="value2"/>"#
        );
    }

    #[test]
    fn empty_paired() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        let start = BytesStart::new("paired")
            .with_attributes(vec![("attr1", "value1"), ("attr2", "value2")].into_iter());
        let end = start.to_end();
        writer
            .write_event(Event::Start(start.clone()))
            .expect("write start tag failed");
        writer
            .write_event(Event::End(end))
            .expect("write end tag failed");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<paired attr1="value1" attr2="value2">
</paired>"#
        );
    }

    #[test]
    fn paired_with_inner() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        let start = BytesStart::new("paired")
            .with_attributes(vec![("attr1", "value1"), ("attr2", "value2")].into_iter());
        let end = start.to_end();
        let inner = BytesStart::new("inner");

        writer
            .write_event(Event::Start(start.clone()))
            .expect("write start tag failed");
        writer
            .write_event(Event::Empty(inner))
            .expect("write inner tag failed");
        writer
            .write_event(Event::End(end))
            .expect("write end tag failed");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<paired attr1="value1" attr2="value2">
    <inner/>
</paired>"#
        );
    }

    #[test]
    fn paired_with_text() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        let start = BytesStart::new("paired")
            .with_attributes(vec![("attr1", "value1"), ("attr2", "value2")].into_iter());
        let end = start.to_end();
        let text = BytesText::new("text");

        writer
            .write_event(Event::Start(start.clone()))
            .expect("write start tag failed");
        writer
            .write_event(Event::Text(text))
            .expect("write text failed");
        writer
            .write_event(Event::End(end))
            .expect("write end tag failed");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<paired attr1="value1" attr2="value2">text</paired>"#
        );
    }

    #[test]
    fn mixed_content() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        let start = BytesStart::new("paired")
            .with_attributes(vec![("attr1", "value1"), ("attr2", "value2")].into_iter());
        let end = start.to_end();
        let text = BytesText::new("text");
        let inner = BytesStart::new("inner");

        writer
            .write_event(Event::Start(start.clone()))
            .expect("write start tag failed");
        writer
            .write_event(Event::Text(text))
            .expect("write text failed");
        writer
            .write_event(Event::Empty(inner))
            .expect("write inner tag failed");
        writer
            .write_event(Event::End(end))
            .expect("write end tag failed");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<paired attr1="value1" attr2="value2">text<inner/>
</paired>"#
        );
    }

    #[test]
    fn nested() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        let start = BytesStart::new("paired")
            .with_attributes(vec![("attr1", "value1"), ("attr2", "value2")].into_iter());
        let end = start.to_end();
        let inner = BytesStart::new("inner");

        writer
            .write_event(Event::Start(start.clone()))
            .expect("write start 1 tag failed");
        writer
            .write_event(Event::Start(start.clone()))
            .expect("write start 2 tag failed");
        writer
            .write_event(Event::Empty(inner))
            .expect("write inner tag failed");
        writer
            .write_event(Event::End(end.clone()))
            .expect("write end tag 2 failed");
        writer
            .write_event(Event::End(end))
            .expect("write end tag 1 failed");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<paired attr1="value1" attr2="value2">
    <paired attr1="value1" attr2="value2">
        <inner/>
    </paired>
</paired>"#
        );
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn serializable() {
        #[derive(Serialize)]
        struct Foo {
            #[serde(rename = "@attribute")]
            attribute: &'static str,

            element: Bar,
            list: Vec<&'static str>,

            #[serde(rename = "$text")]
            text: &'static str,

            val: String,
        }

        #[derive(Serialize)]
        struct Bar {
            baz: usize,
            bat: usize,
        }

        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        let content = Foo {
            attribute: "attribute",
            element: Bar { baz: 42, bat: 43 },
            list: vec!["first element", "second element"],
            text: "text",
            val: "foo".to_owned(),
        };

        let start = BytesStart::new("paired")
            .with_attributes(vec![("attr1", "value1"), ("attr2", "value2")].into_iter());
        let end = start.to_end();

        writer
            .write_event(Event::Start(start.clone()))
            .expect("write start tag failed");
        writer
            .write_serializable("foo_element", &content)
            .expect("write serializable inner contents failed");
        writer
            .write_event(Event::End(end))
            .expect("write end tag failed");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<paired attr1="value1" attr2="value2">
    <foo_element attribute="attribute">
        <element>
            <baz>42</baz>
            <bat>43</bat>
        </element>
        <list>first element</list>
        <list>second element</list>
        text
        <val>foo</val>
    </foo_element>
</paired>"#
        );
    }

    #[test]
    fn element_writer_empty() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        writer
            .create_element("empty")
            .with_attribute(("attr1", "value1"))
            .with_attribute(("attr2", "value2"))
            .write_empty()
            .expect("failure");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<empty attr1="value1" attr2="value2"/>"#
        );
    }

    #[test]
    fn element_writer_text() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        writer
            .create_element("paired")
            .with_attribute(("attr1", "value1"))
            .with_attribute(("attr2", "value2"))
            .write_text_content(BytesText::new("text"))
            .expect("failure");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<paired attr1="value1" attr2="value2">text</paired>"#
        );
    }

    #[test]
    fn element_writer_nested() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        writer
            .create_element("outer")
            .with_attribute(("attr1", "value1"))
            .with_attribute(("attr2", "value2"))
            .write_inner_content(|writer| {
                let fruits = ["apple", "orange", "banana"];
                for (quant, item) in fruits.iter().enumerate() {
                    writer
                        .create_element("fruit")
                        .with_attribute(("quantity", quant.to_string().as_str()))
                        .write_text_content(BytesText::new(item))?;
                }
                writer
                    .create_element("inner")
                    .write_inner_content(|writer| {
                        writer.create_element("empty").write_empty()?;
                        Ok(())
                    })?;

                Ok(())
            })
            .expect("failure");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<outer attr1="value1" attr2="value2">
    <fruit quantity="0">apple</fruit>
    <fruit quantity="1">orange</fruit>
    <fruit quantity="2">banana</fruit>
    <inner>
        <empty/>
    </inner>
</outer>"#
        );
    }
}
