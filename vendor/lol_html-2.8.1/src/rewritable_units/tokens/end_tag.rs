use super::{Mutations, Token};
use crate::base::{Bytes, BytesCow};
use crate::base::{SourceLocation, SpannedRawBytes};
use crate::errors::RewritingError;
use crate::html_content::{ContentType, StreamingHandler, StreamingHandlerSink};
use crate::rewritable_units::StringChunk;
use encoding_rs::Encoding;
use std::fmt::{self, Debug};

/// An HTML end tag rewritable unit.
///
/// Exposes API for examination and modification of a parsed HTML end tag.
pub struct EndTag<'i> {
    name: BytesCow<'i>,
    raw: SpannedRawBytes<'i>,
    encoding: &'static Encoding,
    pub(crate) mutations: Mutations,
}

impl<'i> EndTag<'i> {
    #[inline]
    #[must_use]
    pub(super) fn new_token(
        name: Bytes<'i>,
        raw: SpannedRawBytes<'i>,
        encoding: &'static Encoding,
    ) -> Token<'i> {
        Token::EndTag(EndTag {
            name: name.into(),
            raw,
            encoding,
            mutations: Mutations::new(),
        })
    }

    #[inline(always)]
    pub(crate) fn encoding(&self) -> &'static Encoding {
        self.encoding
    }

    /// Returns the name of the tag, always ASCII lowercase.
    #[inline]
    #[must_use]
    pub fn name(&self) -> String {
        self.name.as_lowercase_string(self.encoding)
    }

    /// Returns the name of the tag, preserving its case.
    #[inline]
    #[must_use]
    pub fn name_preserve_case(&self) -> String {
        self.name.as_string(self.encoding)
    }

    /// Sets the name of the tag.
    pub(crate) fn set_name_raw(&mut self, name: BytesCow<'static>) {
        self.name = name;
        self.raw.set_modified();
    }

    /// Sets the name of the tag only. To rename the element, prefer [`Element::set_tag_name()`][crate::html_content::Element::set_tag_name].
    ///
    /// The name will be converted to the document's encoding.
    ///
    /// The name must have a valid syntax, and the closing tag must be valid in its context.
    /// The parser will not take the new name into account, so if the new tag alters the structure of the document,
    /// the rest of the generated document will be parsed differently than during rewriting.
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.set_name_str(name.into());
    }

    /// Sets the name of the tag only. To rename the element, prefer [`Element::set_tag_name()`][crate::html_content::Element::set_tag_name].
    ///
    /// The name will be converted to the document's encoding.
    ///
    /// The name must have a valid syntax, and the closing tag must be valid in its context.
    /// The parser will not take the new name into account, so if the new tag alters the structure of the document,
    /// the rest of the generated document will be parsed differently than during rewriting.
    #[doc(hidden)]
    pub fn set_name_str(&mut self, name: String) {
        self.set_name_raw(BytesCow::owned_from_str(name, self.encoding));
    }

    /// Inserts `content` before the end tag.
    ///
    /// Consequent calls to the method append `content` to the previously inserted content.
    #[inline]
    pub fn before(&mut self, content: &str, content_type: ContentType) {
        self.mutations
            .mutate()
            .content_before
            .push_back(StringChunk::from_str(content, content_type));
    }

    /// Inserts `content` after the end tag.
    ///
    /// Consequent calls to the method prepend `content` to the previously inserted content.
    #[inline]
    pub fn after(&mut self, content: &str, content_type: ContentType) {
        self.mutations
            .mutate()
            .content_after
            .push_front(StringChunk::from_str(content, content_type));
    }

    /// Replaces the end tag with `content`.
    ///
    /// Consequent calls to the method overwrite previous replacement content.
    #[inline]
    pub fn replace(&mut self, content: &str, content_type: ContentType) {
        self.mutations
            .mutate()
            .replace(StringChunk::from_str(content, content_type));
    }

    /// Inserts content from a [`StreamingHandler`] before the end tag.
    ///
    /// Consequent calls to the method append to the previously inserted content.
    ///
    /// Use the [`streaming!`] macro to make a `StreamingHandler` from a closure.
    #[inline]
    pub fn streaming_before(&mut self, string_writer: Box<dyn StreamingHandler + Send + 'static>) {
        self.mutations
            .mutate()
            .content_before
            .push_back(StringChunk::stream(string_writer));
    }

    /// Inserts content from a [`StreamingHandler`] after the end tag.
    ///
    /// Consequent calls to the method prepend to the previously inserted content.
    ///
    /// Use the [`streaming!`] macro to make a `StreamingHandler` from a closure.
    #[inline]
    pub fn streaming_after(&mut self, string_writer: Box<dyn StreamingHandler + Send + 'static>) {
        self.mutations
            .mutate()
            .content_after
            .push_front(StringChunk::stream(string_writer));
    }

    /// Replaces the end tag with content from a [`StreamingHandler`].
    ///
    /// Consequent calls to the method overwrite previous replacement content.
    ///
    /// Use the [`streaming!`] macro to make a `StreamingHandler` from a closure.
    #[inline]
    pub fn streaming_replace(&mut self, string_writer: Box<dyn StreamingHandler + Send + 'static>) {
        self.mutations
            .mutate()
            .replace(StringChunk::stream(string_writer));
    }

    /// Removes the end tag.
    #[inline]
    pub fn remove(&mut self) {
        self.mutations.mutate().remove();
    }

    /// Returns `true` if the end tag has been replaced or removed.
    #[inline]
    #[must_use]
    pub fn removed(&self) -> bool {
        self.mutations.removed()
    }

    #[inline]
    fn serialize_self(&self, sink: &mut StreamingHandlerSink<'_>) -> Result<(), RewritingError> {
        let output_handler = sink.output_handler();

        if let Some(raw) = self.raw.original() {
            output_handler(raw);
        } else {
            output_handler(b"</");
            output_handler(&self.name);
            output_handler(b">");
        }
        Ok(())
    }

    /// Position of this tag in the source document, before any rewriting
    #[must_use]
    pub fn source_location(&self) -> SourceLocation {
        self.raw.source_location()
    }
}

impl_serialize!(EndTag);

impl Debug for EndTag<'_> {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EndTag")
            .field("name", &self.name())
            .field("at", &self.source_location())
            .finish()
    }
}
