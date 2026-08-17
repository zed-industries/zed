//! This is an implementation of [`Reader`] for reading from a [`BufRead`] as
//! underlying byte stream.

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use crate::errors::{Error, Result};
use crate::events::{BytesText, Event};
use crate::name::QName;
use crate::parser::Parser;
use crate::reader::{BangType, ReadRefResult, ReadTextResult, Reader, Span, XmlSource};
use crate::utils::is_whitespace;

macro_rules! impl_buffered_source {
    ($($lf:lifetime, $reader:tt, $async:ident, $await:ident)?) => {
        #[cfg(not(feature = "encoding"))]
        #[inline]
        $($async)? fn remove_utf8_bom(&mut self) -> io::Result<()> {
            use crate::encoding::UTF8_BOM;

            loop {
                break match self $(.$reader)? .fill_buf() $(.$await)? {
                    Ok(n) => {
                        if n.starts_with(UTF8_BOM) {
                            self $(.$reader)? .consume(UTF8_BOM.len());
                        }
                        Ok(())
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    Err(e) => Err(e),
                };
            }
        }

        #[cfg(feature = "encoding")]
        #[inline]
        $($async)? fn detect_encoding(&mut self) -> io::Result<Option<&'static encoding_rs::Encoding>> {
            loop {
                break match self $(.$reader)? .fill_buf() $(.$await)? {
                    Ok(n) => if let Some((enc, bom_len)) = crate::encoding::detect_encoding(n) {
                        self $(.$reader)? .consume(bom_len);
                        Ok(Some(enc))
                    } else {
                        Ok(None)
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    Err(e) => Err(e),
                };
            }
        }

        #[inline]
        $($async)? fn read_text $(<$lf>)? (
            &mut self,
            buf: &'b mut Vec<u8>,
            position: &mut u64,
        ) -> ReadTextResult<'b, &'b mut Vec<u8>> {
            let mut read = 0;
            let start = buf.len();
            loop {
                let available = match self $(.$reader)? .fill_buf() $(.$await)? {
                    Ok(n) if n.is_empty() => break,
                    Ok(n) => n,
                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    Err(e) => {
                        *position += read;
                        return ReadTextResult::Err(e);
                    }
                };

                // Search for start of markup or an entity or character reference
                match memchr::memchr2(b'<', b'&', available) {
                    // Special handling is needed only on the first iteration.
                    // On next iterations we already read something and should emit Text event
                    Some(0) if read == 0 && available[0] == b'<' => return ReadTextResult::Markup(buf),
                    // Do not consume `&` because it may be lone and we would be need to
                    // return it as part of Text event
                    Some(0) if read == 0 => return ReadTextResult::Ref(buf),
                    Some(i) if available[i] == b'<' => {
                        buf.extend_from_slice(&available[..i]);

                        self $(.$reader)? .consume(i);
                        read += i as u64;

                        *position += read;
                        return ReadTextResult::UpToMarkup(&buf[start..]);
                    }
                    Some(i) => {
                        buf.extend_from_slice(&available[..i]);

                        self $(.$reader)? .consume(i);
                        read += i as u64;

                        *position += read;
                        return ReadTextResult::UpToRef(&buf[start..]);
                    }
                    None => {
                        buf.extend_from_slice(available);

                        let used = available.len();
                        self $(.$reader)? .consume(used);
                        read += used as u64;
                    }
                }
            }

            *position += read;
            ReadTextResult::UpToEof(&buf[start..])
        }

        #[inline]
        $($async)? fn read_ref $(<$lf>)? (
            &mut self,
            buf: &'b mut Vec<u8>,
            position: &mut u64,
        ) -> ReadRefResult<'b> {
            let mut read = 0;
            let start = buf.len();
            loop {
                let available = match self $(.$reader)? .fill_buf() $(.$await)? {
                    Ok(n) if n.is_empty() => break,
                    Ok(n) => n,
                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    Err(e) => {
                        *position += read;
                        return ReadRefResult::Err(e);
                    }
                };
                // `read_ref` called when the first character is `&`, so we
                // should explicitly skip it at first iteration lest we confuse
                // it with the end
                if read == 0 {
                    debug_assert!(
                        available.starts_with(b"&"),
                        "`read_ref` must be called at `&`:\n{:?}",
                        crate::utils::Bytes(available)
                    );
                    // If that ampersand is lone, then it will be part of text
                    // and we should keep it
                    buf.push(b'&');
                    self $(.$reader)? .consume(1);
                    read += 1;
                    continue;
                }

                match memchr::memchr3(b';', b'&', b'<', available) {
                    Some(i) if available[i] == b';' => {
                        // +1 -- skip the end `;`
                        let used = i + 1;

                        buf.extend_from_slice(&available[..used]);
                        self $(.$reader)? .consume(used);
                        read += used as u64;

                        *position += read;

                        return ReadRefResult::Ref(&buf[start..]);
                    }
                    // Do not consume `&` because it may be lone and we would be need to
                    // return it as part of Text event
                    Some(i) => {
                        let is_amp = available[i] == b'&';
                        buf.extend_from_slice(&available[..i]);

                        self $(.$reader)? .consume(i);
                        read += i as u64;

                        *position += read;

                        return if is_amp {
                            ReadRefResult::UpToRef(&buf[start..])
                        } else {
                            ReadRefResult::UpToMarkup(&buf[start..])
                        };
                    }
                    None => {
                        buf.extend_from_slice(available);

                        let used = available.len();
                        self $(.$reader)? .consume(used);
                        read += used as u64;
                    }
                }
            }

            *position += read;
            ReadRefResult::UpToEof(&buf[start..])
        }

        #[inline]
        $($async)? fn read_with<$($lf,)? P: Parser>(
            &mut self,
            mut parser: P,
            buf: &'b mut Vec<u8>,
            position: &mut u64,
        ) -> Result<&'b [u8]> {
            let mut read = 1;
            let start = buf.len();
            // '<' was consumed in peek_one(), but not placed in buf
            buf.push(b'<');
            loop {
                let available = match self $(.$reader)? .fill_buf() $(.$await)? {
                    Ok(n) if n.is_empty() => break,
                    Ok(n) => n,
                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    Err(e) => {
                        *position += read;
                        return Err(Error::Io(e.into()));
                    }
                };

                if let Some(i) = parser.feed(available) {
                    let used = i + 1; // +1 for `>`
                    buf.extend_from_slice(&available[..used]);

                    self $(.$reader)? .consume(used);
                    read += used as u64;

                    *position += read;
                    return Ok(&buf[start..]);
                }

                // The `>` symbol not yet found, continue reading
                buf.extend_from_slice(available);

                let used = available.len();
                self $(.$reader)? .consume(used);
                read += used as u64;
            }

            *position += read;
            Err(Error::Syntax(parser.eof_error(&buf[start..])))
        }

        #[inline]
        $($async)? fn read_bang_element $(<$lf>)? (
            &mut self,
            buf: &'b mut Vec<u8>,
            position: &mut u64,
        ) -> Result<(BangType, &'b [u8])> {
            // Peeked '<!' before being called, so it's guaranteed to start with it.
            let start = buf.len();
            let mut read = 2;
            // '<' was consumed in peek_one(), but not placed in buf
            buf.push(b'<');
            buf.push(b'!');
            self $(.$reader)? .consume(1);

            let mut bang_type = loop {
                break match self $(.$reader)? .fill_buf() $(.$await)? {
                    Ok(n) => BangType::new(n.first().cloned())?,
                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    Err(e) => return Err(Error::Io(e.into())),
                };
            };

            loop {
                let available = match self $(.$reader)? .fill_buf() $(.$await)? {
                    Ok(n) if n.is_empty() => break,
                    Ok(n) => n,
                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    Err(e) => {
                        *position += read;
                        return Err(Error::Io(e.into()));
                    }
                };
                // We only parse from start because we don't want to consider
                // whatever is in the buffer before the bang element
                if let Some(i) = bang_type.feed(&buf[start..], available) {
                    let consumed = i + 1; // +1 for `>`
                    buf.extend_from_slice(&available[..consumed]);

                    self $(.$reader)? .consume(consumed);
                    read += consumed as u64;

                    *position += read;
                    return Ok((bang_type, &buf[start..]));
                }

                // The `>` symbol not yet found, continue reading
                buf.extend_from_slice(available);

                let used = available.len();
                self $(.$reader)? .consume(used);
                read += used as u64;
            }

            *position += read;
            Err(Error::Syntax(bang_type.to_err()))
        }

        #[inline]
        $($async)? fn skip_whitespace(&mut self, position: &mut u64) -> io::Result<()> {
            loop {
                break match self $(.$reader)? .fill_buf() $(.$await)? {
                    Ok(n) => {
                        let count = n.iter().position(|b| !is_whitespace(*b)).unwrap_or(n.len());
                        if count > 0 {
                            self $(.$reader)? .consume(count);
                            *position += count as u64;
                            continue;
                        } else {
                            Ok(())
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    Err(e) => Err(e),
                };
            }
        }

        #[inline]
        $($async)? fn peek_one(&mut self) -> io::Result<Option<u8>> {
            // That method is called only when available buffer starts from '<'
            // We need to consume it
            self $(.$reader)? .consume(1);
            let available = loop {
                break match self $(.$reader)? .fill_buf() $(.$await)? {
                    Ok(n) => n,
                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    Err(e) => return Err(e),
                };
            };
            Ok(available.first().cloned())
        }
    };
}

// Make it public for use in async implementations.
// New rustc reports
// > warning: the item `impl_buffered_source` is imported redundantly
// so make it public only when async feature is enabled
#[cfg(feature = "async-tokio")]
pub(super) use impl_buffered_source;

/// Implementation of `XmlSource` for any `BufRead` reader using a user-given
/// `Vec<u8>` as buffer that will be borrowed by events.
impl<'b, R: BufRead> XmlSource<'b, &'b mut Vec<u8>> for R {
    impl_buffered_source!();
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// This is an implementation for reading from a [`BufRead`] as underlying byte stream.
impl<R: BufRead> Reader<R> {
    /// Reads the next `Event`.
    ///
    /// This is the main entry point for reading XML `Event`s.
    ///
    /// `Event`s borrow `buf` and can be converted to own their data if needed (uses `Cow`
    /// internally).
    ///
    /// Having the possibility to control the internal buffers gives you some additional benefits
    /// such as:
    ///
    /// - Reduce the number of allocations by reusing the same buffer. For constrained systems,
    ///   you can call `buf.clear()` once you are done with processing the event (typically at the
    ///   end of your loop).
    /// - Reserve the buffer length if you know the file size (using `Vec::with_capacity`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use quick_xml::events::Event;
    /// use quick_xml::reader::Reader;
    ///
    /// let xml = r#"<tag1 att1 = "test">
    ///                 <tag2><!--Test comment-->Test</tag2>
    ///                 <tag2>Test 2</tag2>
    ///              </tag1>"#;
    /// let mut reader = Reader::from_str(xml);
    /// reader.config_mut().trim_text(true);
    /// let mut count = 0;
    /// let mut buf = Vec::new();
    /// let mut txt = Vec::new();
    /// loop {
    ///     match reader.read_event_into(&mut buf) {
    ///         Ok(Event::Start(_)) => count += 1,
    ///         Ok(Event::Text(e)) => txt.push(e.decode().unwrap().into_owned()),
    ///         Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
    ///         Ok(Event::Eof) => break,
    ///         _ => (),
    ///     }
    ///     buf.clear();
    /// }
    /// assert_eq!(count, 3);
    /// assert_eq!(txt, vec!["Test".to_string(), "Test 2".to_string()]);
    /// ```
    #[inline]
    pub fn read_event_into<'b>(&mut self, buf: &'b mut Vec<u8>) -> Result<Event<'b>> {
        self.read_event_impl(buf)
    }

    /// Reads until end element is found using provided buffer as intermediate
    /// storage for events content. This function is supposed to be called after
    /// you already read a [`Start`] event.
    ///
    /// Returns a span that cover content between `>` of an opening tag and `<` of
    /// a closing tag or an empty slice, if [`expand_empty_elements`] is set and
    /// this method was called after reading expanded [`Start`] event.
    ///
    /// Manages nested cases where parent and child elements have the _literally_
    /// same name.
    ///
    /// If a corresponding [`End`] event is not found, an error of type [`Error::IllFormed`]
    /// will be returned. In particularly, that error will be returned if you call
    /// this method without consuming the corresponding [`Start`] event first.
    ///
    /// If your reader created from a string slice or byte array slice, it is
    /// better to use [`read_to_end()`] method, because it will not copy bytes
    /// into intermediate buffer.
    ///
    /// The provided `buf` buffer will be filled only by one event content at time.
    /// Before reading of each event the buffer will be cleared. If you know an
    /// appropriate size of each event, you can preallocate the buffer to reduce
    /// number of reallocations.
    ///
    /// The `end` parameter should contain name of the end element _in the reader
    /// encoding_. It is good practice to always get that parameter using
    /// [`BytesStart::to_end()`] method.
    ///
    /// The correctness of the skipped events does not checked, if you disabled
    /// the [`check_end_names`] option.
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
    /// reader.config_mut().trim_text(true);
    /// let mut buf = Vec::new();
    ///
    /// let start = BytesStart::new("outer");
    /// let end   = start.to_end().into_owned();
    ///
    /// // First, we read a start event...
    /// assert_eq!(reader.read_event_into(&mut buf).unwrap(), Event::Start(start));
    ///
    /// // ...then, we could skip all events to the corresponding end event.
    /// // This call will correctly handle nested <outer> elements.
    /// // Note, however, that this method does not handle namespaces.
    /// reader.read_to_end_into(end.name(), &mut buf).unwrap();
    ///
    /// // At the end we should get an Eof event, because we ate the whole XML
    /// assert_eq!(reader.read_event_into(&mut buf).unwrap(), Event::Eof);
    /// ```
    ///
    /// [`Start`]: Event::Start
    /// [`End`]: Event::End
    /// [`BytesStart::to_end()`]: crate::events::BytesStart::to_end
    /// [`read_to_end()`]: Self::read_to_end
    /// [`expand_empty_elements`]: crate::reader::Config::expand_empty_elements
    /// [`check_end_names`]: crate::reader::Config::check_end_names
    /// [the specification]: https://www.w3.org/TR/xml11/#dt-etag
    pub fn read_to_end_into(&mut self, end: QName, buf: &mut Vec<u8>) -> Result<Span> {
        Ok(read_to_end!(self, end, buf, read_event_impl, {
            buf.clear();
        }))
    }

    /// Reads content between start and end tags, including any markup using
    /// provided buffer as intermediate storage for events content. This function
    /// is supposed to be called after you already read a [`Start`] event.
    ///
    /// Manages nested cases where parent and child elements have the _literally_
    /// same name.
    ///
    /// This method does not unescape read data, instead it returns content
    /// "as is" of the XML document. This is because it has no idea what text
    /// it reads, and if, for example, it contains CDATA section, attempt to
    /// unescape it content will spoil data.
    ///
    /// If your reader created from a string slice or byte array slice, it is
    /// better to use [`read_text()`] method, because it will not copy bytes
    /// into intermediate buffer.
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
    /// let mut reader = Reader::from_reader("
    ///     <html>
    ///         <title>This is a HTML text</title>
    ///         <p>Usual XML rules does not apply inside it
    ///         <p>For example, elements not needed to be &quot;closed&quot;
    ///     </html>
    /// ".as_bytes());
    /// reader.config_mut().trim_text(true);
    ///
    /// let start = BytesStart::new("html");
    /// let end   = start.to_end().into_owned();
    ///
    /// let mut buf = Vec::new();
    ///
    /// // First, we read a start event...
    /// assert_eq!(reader.read_event_into(&mut buf).unwrap(), Event::Start(start));
    /// // ...and disable checking of end names because we expect HTML further...
    /// reader.config_mut().check_end_names = false;
    ///
    /// // ...then, we could read text content until close tag.
    /// // This call will correctly handle nested <html> elements.
    /// let text = reader.read_text_into(end.name(), &mut buf).unwrap();
    /// let text = text.decode().unwrap();
    /// assert_eq!(text, r#"
    ///         <title>This is a HTML text</title>
    ///         <p>Usual XML rules does not apply inside it
    ///         <p>For example, elements not needed to be &quot;closed&quot;
    ///     "#);
    /// assert!(matches!(text, Cow::Borrowed(_)));
    ///
    /// // Now we can enable checks again
    /// reader.config_mut().check_end_names = true;
    ///
    /// // At the end we should get an Eof event, because we ate the whole XML
    /// assert_eq!(reader.read_event_into(&mut buf).unwrap(), Event::Eof);
    /// ```
    ///
    /// [`Start`]: Event::Start
    /// [`read_text()`]: Self::read_text()
    pub fn read_text_into<'b>(
        &mut self,
        end: QName,
        buf: &'b mut Vec<u8>,
    ) -> Result<BytesText<'b>> {
        let start = buf.len();
        let span = read_to_end!(self, end, buf, read_event_impl, {});

        let len = span.end - span.start;
        // SAFETY: `buf` may contain not more than isize::MAX bytes and because it is
        // not cleared when reading event, length of the returned span should fit into
        // usize (because otherwise we panic at appending to the buffer before that point)
        let end = start + len as usize;

        Ok(BytesText::wrap(&buf[start..end], self.decoder()))
    }
}

impl Reader<BufReader<File>> {
    /// Creates an XML reader from a file path.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(Self::from_reader(reader))
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
        1,
        &mut Vec::new()
    );
}
