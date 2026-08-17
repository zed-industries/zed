//! This is an implementation of [`Reader`] for reading from a `&[u8]` as
//! underlying byte stream. This implementation supports not using an
//! intermediate buffer as the byte slice itself can be used to borrow from.

use std::borrow::Cow;

#[cfg(feature = "encoding")]
use crate::reader::EncodingRef;
#[cfg(feature = "encoding")]
use encoding_rs::{Encoding, UTF_8};

use crate::errors::{Error, Result};
use crate::events::Event;
use crate::name::QName;
use crate::reader::{is_whitespace, BangType, ReadElementState, Reader, Span, XmlSource};

use memchr;

/// This is an implementation for reading from a `&[u8]` as underlying byte stream.
/// This implementation supports not using an intermediate buffer as the byte slice
/// itself can be used to borrow from.
impl<'a> Reader<&'a [u8]> {
    /// Creates an XML reader from a string slice.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &'a str) -> Self {
        // Rust strings are guaranteed to be UTF-8, so lock the encoding
        #[cfg(feature = "encoding")]
        {
            let mut reader = Self::from_reader(s.as_bytes());
            reader.parser.encoding = EncodingRef::Explicit(UTF_8);
            reader
        }

        #[cfg(not(feature = "encoding"))]
        Self::from_reader(s.as_bytes())
    }

    /// Read an event that borrows from the input rather than a buffer.
    ///
    /// There is no asynchronous `read_event_async()` version of this function,
    /// because it is not necessary -- the contents are already in memory and no IO
    /// is needed, therefore there is no potential for blocking.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use quick_xml::events::Event;
    /// use quick_xml::reader::Reader;
    ///
    /// let mut reader = Reader::from_str(r#"
    ///     <tag1 att1 = "test">
    ///        <tag2><!--Test comment-->Test</tag2>
    ///        <tag2>Test 2</tag2>
    ///     </tag1>
    /// "#);
    /// reader.trim_text(true);
    ///
    /// let mut count = 0;
    /// let mut txt = Vec::new();
    /// loop {
    ///     match reader.read_event().unwrap() {
    ///         Event::Start(e) => count += 1,
    ///         Event::Text(e) => txt.push(e.unescape().unwrap().into_owned()),
    ///         Event::Eof => break,
    ///         _ => (),
    ///     }
    /// }
    /// assert_eq!(count, 3);
    /// assert_eq!(txt, vec!["Test".to_string(), "Test 2".to_string()]);
    /// ```
    #[inline]
    pub fn read_event(&mut self) -> Result<Event<'a>> {
        self.read_event_impl(())
    }

    /// Reads until end element is found. This function is supposed to be called
    /// after you already read a [`Start`] event.
    ///
    /// Returns a span that cover content between `>` of an opening tag and `<` of
    /// a closing tag or an empty slice, if [`expand_empty_elements`] is set and
    /// this method was called after reading expanded [`Start`] event.
    ///
    /// Manages nested cases where parent and child elements have the _literally_
    /// same name.
    ///
    /// If corresponding [`End`] event will not be found, the [`Error::UnexpectedEof`]
    /// will be returned. In particularly, that error will be returned if you call
    /// this method without consuming the corresponding [`Start`] event first.
    ///
    /// The `end` parameter should contain name of the end element _in the reader
    /// encoding_. It is good practice to always get that parameter using
    /// [`BytesStart::to_end()`] method.
    ///
    /// The correctness of the skipped events does not checked, if you disabled
    /// the [`check_end_names`] option.
    ///
    /// There is no asynchronous `read_to_end_async()` version of this function,
    /// because it is not necessary -- the contents are already in memory and no IO
    /// is needed, therefore there is no potential for blocking.
    ///
    /// # Namespaces
    ///
    /// While the `Reader` does not support namespace resolution, namespaces
    /// does not change the algorithm for comparing names. Although the names
    /// `a:name` and `b:name` where both prefixes `a` and `b` resolves to the
    /// same namespace, are semantically equivalent, `</b:name>` cannot close
    /// `<a:name>`, because according to [the specification]
    ///
    /// > The end of every element that begins with a **start-tag** MUST be marked
    /// > by an **end-tag** containing a name that echoes the element's type as
    /// > given in the **start-tag**
    ///
    /// # Examples
    ///
    /// This example shows, how you can skip XML content after you read the
    /// start event.
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use quick_xml::events::{BytesStart, Event};
    /// use quick_xml::reader::Reader;
    ///
    /// let mut reader = Reader::from_str(r#"
    ///     <outer>
    ///         <inner>
    ///             <inner></inner>
    ///             <inner/>
    ///             <outer></outer>
    ///             <outer/>
    ///         </inner>
    ///     </outer>
    /// "#);
    /// reader.trim_text(true);
    ///
    /// let start = BytesStart::new("outer");
    /// let end   = start.to_end().into_owned();
    ///
    /// // First, we read a start event...
    /// assert_eq!(reader.read_event().unwrap(), Event::Start(start));
    ///
    /// // ...then, we could skip all events to the corresponding end event.
    /// // This call will correctly handle nested <outer> elements.
    /// // Note, however, that this method does not handle namespaces.
    /// reader.read_to_end(end.name()).unwrap();
    ///
    /// // At the end we should get an Eof event, because we ate the whole XML
    /// assert_eq!(reader.read_event().unwrap(), Event::Eof);
    /// ```
    ///
    /// [`Start`]: Event::Start
    /// [`End`]: Event::End
    /// [`BytesStart::to_end()`]: crate::events::BytesStart::to_end
    /// [`expand_empty_elements`]: Self::expand_empty_elements
    /// [`check_end_names`]: Self::check_end_names
    /// [the specification]: https://www.w3.org/TR/xml11/#dt-etag
    pub fn read_to_end(&mut self, end: QName) -> Result<Span> {
        Ok(read_to_end!(self, end, (), read_event_impl, {}))
    }

    /// Reads content between start and end tags, including any markup. This
    /// function is supposed to be called after you already read a [`Start`] event.
    ///
    /// Manages nested cases where parent and child elements have the _literally_
    /// same name.
    ///
    /// This method does not unescape read data, instead it returns content
    /// "as is" of the XML document. This is because it has no idea what text
    /// it reads, and if, for example, it contains CDATA section, attempt to
    /// unescape it content will spoil data.
    ///
    /// Any text will be decoded using the XML current [`decoder()`].
    ///
    /// Actually, this method perform the following code:
    ///
    /// ```ignore
    /// let span = reader.read_to_end(end)?;
    /// let text = reader.decoder().decode(&reader.inner_slice[span]);
    /// ```
    ///
    /// # Examples
    ///
    /// This example shows, how you can read a HTML content from your XML document.
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// # use std::borrow::Cow;
    /// use quick_xml::events::{BytesStart, Event};
    /// use quick_xml::reader::Reader;
    ///
    /// let mut reader = Reader::from_str("
    ///     <html>
    ///         <title>This is a HTML text</title>
    ///         <p>Usual XML rules does not apply inside it
    ///         <p>For example, elements not needed to be &quot;closed&quot;
    ///     </html>
    /// ");
    /// reader.trim_text(true);
    ///
    /// let start = BytesStart::new("html");
    /// let end   = start.to_end().into_owned();
    ///
    /// // First, we read a start event...
    /// assert_eq!(reader.read_event().unwrap(), Event::Start(start));
    /// // ...and disable checking of end names because we expect HTML further...
    /// reader.check_end_names(false);
    ///
    /// // ...then, we could read text content until close tag.
    /// // This call will correctly handle nested <html> elements.
    /// let text = reader.read_text(end.name()).unwrap();
    /// assert_eq!(text, Cow::Borrowed(r#"
    ///         <title>This is a HTML text</title>
    ///         <p>Usual XML rules does not apply inside it
    ///         <p>For example, elements not needed to be &quot;closed&quot;
    ///     "#));
    /// assert!(matches!(text, Cow::Borrowed(_)));
    ///
    /// // Now we can enable checks again
    /// reader.check_end_names(true);
    ///
    /// // At the end we should get an Eof event, because we ate the whole XML
    /// assert_eq!(reader.read_event().unwrap(), Event::Eof);
    /// ```
    ///
    /// [`Start`]: Event::Start
    /// [`decoder()`]: Self::decoder()
    pub fn read_text(&mut self, end: QName) -> Result<Cow<'a, str>> {
        // self.reader will be changed, so store original reference
        let buffer = self.reader;
        let span = self.read_to_end(end)?;

        self.decoder().decode(&buffer[0..span.len()])
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Implementation of `XmlSource` for `&[u8]` reader using a `Self` as buffer
/// that will be borrowed by events. This implementation provides a zero-copy deserialization
impl<'a> XmlSource<'a, ()> for &'a [u8] {
    #[cfg(not(feature = "encoding"))]
    fn remove_utf8_bom(&mut self) -> Result<()> {
        if self.starts_with(crate::encoding::UTF8_BOM) {
            *self = &self[crate::encoding::UTF8_BOM.len()..];
        }
        Ok(())
    }

    #[cfg(feature = "encoding")]
    fn detect_encoding(&mut self) -> Result<Option<&'static Encoding>> {
        if let Some((enc, bom_len)) = crate::encoding::detect_encoding(self) {
            *self = &self[bom_len..];
            return Ok(Some(enc));
        }
        Ok(None)
    }

    fn read_bytes_until(
        &mut self,
        byte: u8,
        _buf: (),
        position: &mut usize,
    ) -> Result<Option<&'a [u8]>> {
        // search byte must be within the ascii range
        debug_assert!(byte.is_ascii());
        if self.is_empty() {
            return Ok(None);
        }

        Ok(Some(if let Some(i) = memchr::memchr(byte, self) {
            *position += i + 1;
            let bytes = &self[..i];
            *self = &self[i + 1..];
            bytes
        } else {
            *position += self.len();
            let bytes = &self[..];
            *self = &[];
            bytes
        }))
    }

    fn read_bang_element(
        &mut self,
        _buf: (),
        position: &mut usize,
    ) -> Result<Option<(BangType, &'a [u8])>> {
        // Peeked one bang ('!') before being called, so it's guaranteed to
        // start with it.
        debug_assert_eq!(self[0], b'!');

        let bang_type = BangType::new(self[1..].first().copied())?;

        if let Some((bytes, i)) = bang_type.parse(&[], self) {
            *position += i;
            *self = &self[i..];
            return Ok(Some((bang_type, bytes)));
        }

        // Note: Do not update position, so the error points to
        // somewhere sane rather than at the EOF
        Err(bang_type.to_err())
    }

    fn read_element(&mut self, _buf: (), position: &mut usize) -> Result<Option<&'a [u8]>> {
        if self.is_empty() {
            return Ok(None);
        }

        let mut state = ReadElementState::Elem;

        if let Some((bytes, i)) = state.change(self) {
            // Position now just after the `>` symbol
            *position += i;
            *self = &self[i..];
            return Ok(Some(bytes));
        }

        // Note: Do not update position, so the error points to a sane place
        // rather than at the EOF.
        Err(Error::UnexpectedEof("Element".to_string()))

        // FIXME: Figure out why the other one works without UnexpectedEof
    }

    fn skip_whitespace(&mut self, position: &mut usize) -> Result<()> {
        let whitespaces = self
            .iter()
            .position(|b| !is_whitespace(*b))
            .unwrap_or(self.len());
        *position += whitespaces;
        *self = &self[whitespaces..];
        Ok(())
    }

    fn skip_one(&mut self, byte: u8, position: &mut usize) -> Result<bool> {
        // search byte must be within the ascii range
        debug_assert!(byte.is_ascii());
        if self.first() == Some(&byte) {
            *self = &self[1..];
            *position += 1;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn peek_one(&mut self) -> Result<Option<u8>> {
        Ok(self.first().copied())
    }
}

#[cfg(test)]
mod test {
    use crate::reader::test::check;
    use crate::reader::XmlSource;

    /// Default buffer constructor just pass the byte array from the test
    fn identity<T>(input: T) -> T {
        input
    }

    check!(
        #[test]
        read_event_impl,
        read_until_close,
        identity,
        ()
    );

    #[cfg(feature = "encoding")]
    mod encoding {
        use crate::events::Event;
        use crate::reader::Reader;
        use encoding_rs::UTF_8;
        use pretty_assertions::assert_eq;

        /// Checks that XML declaration cannot change the encoding from UTF-8 if
        /// a `Reader` was created using `from_str` method
        #[test]
        fn str_always_has_utf8() {
            let mut reader = Reader::from_str("<?xml encoding='UTF-16'?>");

            assert_eq!(reader.decoder().encoding(), UTF_8);
            reader.read_event().unwrap();
            assert_eq!(reader.decoder().encoding(), UTF_8);

            assert_eq!(reader.read_event().unwrap(), Event::Eof);
        }
    }
}
