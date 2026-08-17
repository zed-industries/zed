use super::Mutations;
use crate::base::SourceLocation;
use crate::errors::RewritingError;
use crate::html::TextType;
use crate::html_content::{ContentType, StreamingHandler, StreamingHandlerSink};
use crate::rewritable_units::StringChunk;
use encoding_rs::Encoding;
use std::any::Any;
use std::borrow::Cow;
use std::fmt::{self, Debug};

/// A fragment of an HTML text node. Beware: this is tricky to use.
///
/// ## The text is fragmented
///
/// The text chunks are **not** complete text DOM nodes. They are fragments of text nodes, split at arbitrary points.
///
/// Since the rewriter operates on a streaming input with minimal internal buffering, HTML
/// text node can be represented by multiple text chunks. The size of a chunk depends on multiple
/// parameters, such as decoding buffer size and input chunk size.
///
/// It is up to a user of the rewriter to buffer content of chunks to get the whole text node
/// content where desired. The last chunk in a text node can be determined by calling
/// [`last_in_text_node`] method of the chunk.
///
/// Note that in the sequence `"<span>red-<b>or</b>-blue</span>"` the `span` element contains three text
/// nodes: `"red-"`, `"or"`, and `"-blue"`. Each of these can produce multiple text chunks and each will
/// produce one text chunk where [`last_in_text_node`] returns `true`. The last chunk in a text
/// node can have empty textual content. To perform an action once on the text contents of an
/// element, see [`Element::end_tag_handlers`][crate::rewritable_units::Element::end_tag_handlers].
///
/// ## The text may contain entities
///
/// Text chunks are passed through as-is, without unescaping. The text may contain HTML entities, potentially
/// split across chunk boundaries. Text chunks may also appear inside `<style>` or `<script>`,
/// which do not support HTML entities.
///
/// It is important to observe [`TextChunk::text_type`] and unescape/escape the chunks accordingly.
///
/// # Example
/// ```
/// use lol_html::{HtmlRewriter, Settings, text};
///
/// let mut greeting = String::new();
///
/// {
///     let mut rewriter = HtmlRewriter::new(
///         Settings {
///            element_content_handlers: vec![
///                text!("div", |t| {
///                  greeting += t.as_str();
///
///                  if t.last_in_text_node() {
///                         greeting += "!";
///                     }
///
///                     Ok(())
///                 })
///             ],
///             ..Settings::new()
///         },
///         |_:&[u8]| {}
///     );
///
///     rewriter.write(b"<div>He").unwrap();
///     rewriter.write(b"llo w").unwrap();
///     rewriter.write(b"orld</div>").unwrap();
///     rewriter.end().unwrap();
/// }
///
/// assert_eq!(greeting, "Hello world!");
/// ```
///
/// [`last_in_text_node`]: #method.last_in_text_node
pub struct TextChunk<'i> {
    text: Cow<'i, str>,
    text_type: TextType,
    last_in_text_node: bool,
    encoding: &'static Encoding,
    mutations: Mutations,
    user_data: Box<dyn Any>,
    source_location: SourceLocation,
}

impl<'i> TextChunk<'i> {
    #[inline]
    #[must_use]
    pub(crate) fn new(
        text: &'i str,
        text_type: TextType,
        last_in_text_node: bool,
        encoding: &'static Encoding,
        source_location: SourceLocation,
    ) -> Self {
        TextChunk {
            text: text.into(),
            text_type,
            last_in_text_node,
            encoding,
            mutations: Mutations::new(),
            user_data: Box::new(()),
            source_location,
        }
    }

    #[inline(always)]
    pub(crate) fn encoding(&self) -> &'static Encoding {
        self.encoding
    }

    /// Returns the content of the chunk, which [`may not be a complete text node`](TextChunk).
    ///
    /// It may contain markup, such as HTML/XML entities. See [`TextChunk::text_type`].
    ///
    /// Because the text may be fragmented and contain markup, operating on the slice directly is tricky.
    /// It may be necessary to buffer the text. See [`TextChunk::last_in_text_node`].
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// Returns the content of the chunk, which [`may not be a complete text node`](TextChunk).
    ///
    /// It may contain markup, such as HTML/XML entities. See [`TextChunk::text_type`].
    ///
    /// The string may be mutated, but needs to have markup escaped appropriately for the context its in.
    ///
    /// Because the text may be fragmented and contain markup, operating on the slice directly is tricky.
    /// It may be necessary to buffer the text. See [`TextChunk::last_in_text_node`].
    #[inline]
    pub fn as_mut_str(&mut self) -> &mut String {
        self.text.to_mut()
    }

    /// Sets the content of this chunk only, without affecting text chunks around it.
    ///
    /// The markup must contain escaping appropriate for the context it's in, e.g. use `&amp` instead of `&` in HTML body.
    /// See [`TextChunk::text_type`].
    #[inline]
    pub fn set_str(&mut self, text: String) {
        self.text = Cow::Owned(text);
    }

    /// Returns the type of the text in the chunk.
    ///
    /// The type of the text depends on the surrounding context of the text. E.g. regular visible
    /// text and text inside a `<script>` element will have different types. Refer to [`TextType`]
    /// for more information about possible text types.
    ///
    /// [`TextType`]: enum.TextType.html
    ///
    /// # Example
    ///
    /// ```
    /// use lol_html::{rewrite_str, text, RewriteStrSettings};
    /// use lol_html::html_content::TextType;
    ///
    /// let html = rewrite_str(
    ///     r#"<div>Hello</div><script>"use strict";</script>"#,
    ///     RewriteStrSettings {
    ///         element_content_handlers: vec![
    ///             text!("div", |t| {
    ///                 assert_eq!(t.text_type(), TextType::Data);
    ///
    ///                 Ok(())
    ///             }),
    ///             text!("script", |t| {
    ///                 assert_eq!(t.text_type(), TextType::ScriptData);
    ///
    ///                 Ok(())
    ///             })
    ///         ],
    ///         ..RewriteStrSettings::new()
    ///     }
    /// ).unwrap();
    /// ```
    #[inline]
    #[must_use]
    pub fn text_type(&self) -> TextType {
        self.text_type
    }

    /// Returns `true` if the chunk is last in a HTML text node.
    ///
    /// Note that last chunk can have empty textual content.
    ///
    /// In the current implementation, text nodes are interrupted by comments and CDATA.
    /// Use [`Element::end_tag_handlers`][crate::rewritable_units::Element::end_tag_handlers] to add content after all text in an element.
    #[inline]
    #[must_use]
    #[doc(alias = "is_last")]
    pub fn last_in_text_node(&self) -> bool {
        self.last_in_text_node
    }

    /// Inserts `content` before the text chunk.
    ///
    /// Consequent calls to the method append `content` to the previously inserted content.
    ///
    /// # Example
    ///
    /// ```
    /// use lol_html::{rewrite_str, text, RewriteStrSettings};
    /// use lol_html::html_content::ContentType;
    ///
    /// let html = rewrite_str(
    ///     r#"<div>world</div>"#,
    ///     RewriteStrSettings {
    ///         element_content_handlers: vec![
    ///             text!("div", |t| {
    ///                 if !t.last_in_text_node(){
    ///                     t.before("<!-- 42 -->", ContentType::Html);
    ///                     t.before("Hello ", ContentType::Text);
    ///                 }
    ///
    ///                 Ok(())
    ///             })
    ///         ],
    ///         ..RewriteStrSettings::new()
    ///     }
    /// ).unwrap();
    ///
    /// assert_eq!(html, r#"<div><!-- 42 -->Hello world</div>"#);
    /// ```
    #[inline]
    pub fn before(&mut self, content: &str, content_type: ContentType) {
        self.mutations
            .mutate()
            .content_before
            .push_back(StringChunk::from_str(content, content_type));
    }

    /// Inserts `content` after the text chunk.
    ///
    /// Consequent calls to the method prepend `content` to the previously inserted content.
    ///
    /// # Example
    ///
    /// ```
    /// use lol_html::{rewrite_str, text, RewriteStrSettings};
    /// use lol_html::html_content::ContentType;
    ///
    /// let html = rewrite_str(
    ///     r#"<div>Foo</div>"#,
    ///     RewriteStrSettings {
    ///         element_content_handlers: vec![
    ///             text!("div", |t| {
    ///                 if t.last_in_text_node(){
    ///                     t.after("Bar", ContentType::Text);
    ///                     t.after("Qux", ContentType::Text);
    ///                 }
    ///
    ///                 Ok(())
    ///             })
    ///         ],
    ///         ..RewriteStrSettings::new()
    ///     }
    /// ).unwrap();
    ///
    /// assert_eq!(html, r#"<div>FooQuxBar</div>"#);
    /// ```
    #[inline]
    pub fn after(&mut self, content: &str, content_type: ContentType) {
        self.mutations
            .mutate()
            .content_after
            .push_front(StringChunk::from_str(content, content_type));
    }

    /// Replaces the text chunk with the `content`.
    ///
    /// Consequent calls to the method overwrite previous replacement content.
    ///
    /// # Example
    ///
    /// ```
    /// use lol_html::{rewrite_str, text, RewriteStrSettings};
    /// use lol_html::html_content::ContentType;
    ///
    /// let html = rewrite_str(
    ///     r#"<div>Foo</div>"#,
    ///     RewriteStrSettings {
    ///         element_content_handlers: vec![
    ///             text!("div", |t| {
    ///                 if !t.last_in_text_node(){
    ///                     t.replace("Bar", ContentType::Text);
    ///                     t.replace("Qux", ContentType::Text);
    ///                 }
    ///
    ///                 Ok(())
    ///             })
    ///         ],
    ///         ..RewriteStrSettings::new()
    ///     }
    /// ).unwrap();
    ///
    /// assert_eq!(html, r#"<div>Qux</div>"#);
    /// ```
    #[inline]
    pub fn replace(&mut self, content: &str, content_type: ContentType) {
        self.mutations
            .mutate()
            .replace(StringChunk::from_str(content, content_type));
    }

    /// Inserts content from a [`StreamingHandler`] before the text chunk.
    ///
    /// Consequent calls to the method append `content` to the previously inserted content.
    ///
    /// Use the [`streaming!`] macro to make a `StreamingHandler` from a closure.
    pub fn streaming_before(&mut self, string_writer: Box<dyn StreamingHandler + Send + 'static>) {
        self.mutations
            .mutate()
            .content_before
            .push_back(StringChunk::stream(string_writer));
    }

    /// Inserts content from a [`StreamingHandler`] after the text chunk.
    ///
    /// Consequent calls to the method prepend to the previously inserted content.
    ///
    /// Use the [`streaming!`] macro to make a `StreamingHandler` from a closure.
    pub fn streaming_after(&mut self, string_writer: Box<dyn StreamingHandler + Send + 'static>) {
        self.mutations
            .mutate()
            .content_after
            .push_front(StringChunk::stream(string_writer));
    }

    /// Replaces the text chunk with the content from a [`StreamingHandler`].
    ///
    /// Consequent calls to the method overwrite previous replacement content.
    ///
    /// Use the [`streaming!`] macro to make a `StreamingHandler` from a closure.
    pub fn streaming_replace(&mut self, string_writer: Box<dyn StreamingHandler + Send + 'static>) {
        self.mutations
            .mutate()
            .replace(StringChunk::stream(string_writer));
    }

    /// Removes the text chunk.
    #[inline]
    pub fn remove(&mut self) {
        self.mutations.mutate().remove();
    }

    /// Returns `true` if the text chunk has been replaced or removed.
    #[inline]
    #[must_use]
    pub fn removed(&self) -> bool {
        self.mutations.removed()
    }

    #[inline]
    fn serialize_self(&self, sink: &mut StreamingHandlerSink<'_>) -> Result<(), RewritingError> {
        if !self.text.is_empty() {
            // The "text" here is actually markup
            sink.write_str(&self.text, ContentType::Html);
        }
        Ok(())
    }

    /// Position of this text fragment in the source document, before any rewriting
    #[must_use]
    pub fn source_location(&self) -> SourceLocation {
        self.source_location.clone()
    }
}

impl_serialize!(TextChunk);
impl_user_data!(TextChunk<'_>);

impl Debug for TextChunk<'_> {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TextChunk")
            .field("text", &self.as_str())
            .field("last_in_text_node", &self.last_in_text_node())
            .field("at", &self.source_location())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::html_content::*;
    use crate::rewritable_units::test_utils::*;
    use encoding_rs::{Encoding, UTF_8};

    fn rewrite_text_chunk(
        html: &[u8],
        encoding: &'static Encoding,
        mut handler: impl FnMut(&mut TextChunk<'_>),
    ) -> String {
        let mut handler_called = false;

        let output = rewrite_html(
            html,
            encoding,
            vec![],
            vec![doc_text!(|c| {
                handler_called = true;
                handler(c);
                Ok(())
            })],
        );

        assert!(handler_called);

        output
    }

    #[test]
    fn user_data() {
        rewrite_text_chunk(b"foo", UTF_8, |c| {
            c.set_user_data(42usize);

            assert_eq!(*c.user_data().downcast_ref::<usize>().unwrap(), 42usize);

            *c.user_data_mut().downcast_mut::<usize>().unwrap() = 1337usize;

            assert_eq!(*c.user_data().downcast_ref::<usize>().unwrap(), 1337usize);
        });
    }

    #[test]
    fn in_place_text_modifications() {
        let encoding = Encoding::for_label_no_replacement(b"utf-8").unwrap();
        let mut chunk = TextChunk::new(
            "original text",
            TextType::PlainText,
            true,
            encoding,
            SourceLocation::from_start_len(0, 0),
        );

        assert_eq!(chunk.as_str(), "original text");
        chunk.set_str("hello".to_owned());
        assert_eq!(chunk.as_str(), "hello");
        chunk.as_mut_str().push_str(" world!");
        assert_eq!(chunk.as_str(), "hello world!");
    }

    mod serialization {
        use super::*;

        const HTML: &str = "Lorem ipsum dolor sit amet, cÔnsectetur adipiscing elit, sed do eiusmod tempor \
             incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud \
             exercitation & ullamco laboris nisi ut aliquip ex ea commodo > consequat.";

        macro_rules! test {
            ($handler:expr, $expected:expr) => {
                for (html, enc) in encoded(HTML) {
                    assert_eq!(rewrite_text_chunk(&html, enc, $handler), $expected);
                }
            };
        }

        #[test]
        fn parsed() {
            test!(|_| {}, HTML);
        }

        #[test]
        fn with_prepends_and_appends() {
            let mut first = true;
            test!(
                |c| {
                    let is_first = std::mem::replace(&mut first, c.last_in_text_node());
                    if is_first {
                        c.before("<span>", ContentType::Text);
                        c.before("<div>Hey</div>", ContentType::Html);
                        c.before("<foo>", ContentType::Html);
                    }
                    if c.last_in_text_node() {
                        c.after("</foo>", ContentType::Html);
                        c.after("<!-- 42 -->", ContentType::Html);
                        c.after("<foo & bar>", ContentType::Text);
                    }
                },
                concat!(
                    "&lt;span&gt;<div>Hey</div><foo>",
                    "Lorem ipsum dolor sit amet, cÔnsectetur adipiscing elit, sed do eiusmod \
                     tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim \
                     veniam, quis nostrud exercitation & ullamco laboris nisi ut aliquip \
                     ex ea commodo > consequat.",
                    "&lt;foo &amp; bar&gt;<!-- 42 --></foo>"
                )
            );
        }

        #[test]
        fn removed() {
            let mut first = true;
            test!(
                |c| {
                    let is_first = std::mem::replace(&mut first, c.last_in_text_node());
                    assert!(!c.removed());

                    c.remove();

                    assert!(c.removed());

                    if is_first {
                        c.before("<before>", ContentType::Html);
                    }
                    if c.last_in_text_node() {
                        c.after("<after>", ContentType::Html);
                    }
                },
                "<before><after>"
            );
        }

        #[test]
        fn replaced_with_text() {
            test!(
                |c| {
                    if c.last_in_text_node() {
                        c.before("<before>", ContentType::Html);
                        c.after("<after>", ContentType::Html);

                        assert!(!c.removed());

                        c.replace("<div></div>", ContentType::Html);
                        c.replace("<!--42-->", ContentType::Html);
                        c.replace("<foo & bar>", ContentType::Text);

                        assert!(c.removed());
                    } else {
                        c.remove();
                    }
                },
                "<before>&lt;foo &amp; bar&gt;<after>"
            );
        }

        #[test]
        fn replaced_with_html() {
            test!(
                |c| {
                    if c.last_in_text_node() {
                        c.before("<before>", ContentType::Html);
                        c.after("<after>", ContentType::Html);

                        assert!(!c.removed());

                        c.replace("<div></div>", ContentType::Html);
                        c.replace("<!--42-->", ContentType::Html);
                        c.replace("<foo & bar>", ContentType::Html);

                        assert!(c.removed());
                    } else {
                        c.remove();
                    }
                },
                "<before><foo & bar><after>"
            );
        }

        #[test]
        fn last_flush_text_decoder() {
            let rewritten = rewrite_text_chunk(b"<p>\xF0\xF0\x9F\xF0\x9F\x98</p>", UTF_8, |c| {
                if c.last_in_text_node() {
                    c.after(" last", ContentType::Text);
                }
            });
            assert_eq!("<p>\u{fffd}\u{fffd}\u{fffd} last</p>", rewritten);
        }
    }
}
