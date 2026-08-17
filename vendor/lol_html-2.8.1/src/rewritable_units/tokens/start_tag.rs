use super::{Attribute, AttributeNameError, Attributes};
use super::{Mutations, Serialize, Token};
use crate::base::SourceLocation;
use crate::base::SpannedRawBytes;
use crate::base::{Bytes, BytesCow};
use crate::errors::RewritingError;
use crate::html::Namespace;
use crate::html_content::{ContentType, StreamingHandler, StreamingHandlerSink};
use crate::rewritable_units::StringChunk;
use encoding_rs::Encoding;
use std::fmt::{self, Debug};

/// An HTML start tag rewritable unit.
///
/// Exposes API for examination and modification of a parsed HTML start tag.
pub struct StartTag<'input_token> {
    name: BytesCow<'input_token>,
    attributes: Attributes<'input_token>,
    ns: Namespace,
    self_closing: bool,
    raw: SpannedRawBytes<'input_token>,
    pub(crate) mutations: Mutations,
}

impl<'input_token> StartTag<'input_token> {
    /// Reuses encoding from `attributes`
    #[inline]
    #[must_use]
    pub(super) fn new_token(
        name: Bytes<'input_token>,
        attributes: Attributes<'input_token>,
        ns: Namespace,
        self_closing: bool,
        raw: SpannedRawBytes<'input_token>,
    ) -> Token<'input_token> {
        Token::StartTag(StartTag {
            name: name.into(),
            attributes,
            ns,
            self_closing,
            raw,
            mutations: Mutations::new(),
        })
    }

    #[inline(always)]
    #[doc(hidden)]
    pub const fn encoding(&self) -> &'static Encoding {
        self.attributes.encoding
    }

    /// Returns the name of the tag, always ASCII lowercased.
    #[inline]
    pub fn name(&self) -> String {
        self.name.as_lowercase_string(self.attributes.encoding)
    }

    /// Returns the name of the tag, preserving its case.
    #[inline]
    pub fn name_preserve_case(&self) -> String {
        self.name.as_string(self.attributes.encoding)
    }

    /// Sets the name of the tag.
    #[inline]
    pub(crate) fn set_name_raw(&mut self, name: BytesCow<'static>) {
        self.name = name;
        self.raw.set_modified();
    }

    /// Sets the name of the start tag only. To rename the element, prefer [`Element::set_tag_name()`][crate::html_content::Element::set_tag_name].
    ///
    /// The tag name must have a valid syntax for its context.
    ///
    /// The new tag name must be in the same namespace, have the same content model, and be valid in its location.
    /// Otherwise change of the tag name may cause the resulting document to be parsed in an unexpected way,
    /// out of sync with this library.
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.set_name_raw(BytesCow::owned_from_str(name.into(), self.encoding()));
    }

    /// Returns the [namespace URI] of the tag's element.
    ///
    /// [namespace URI]: https://developer.mozilla.org/en-US/docs/Web/API/Element/namespaceURI
    #[inline]
    pub fn namespace_uri(&self) -> &'static str {
        self.ns.uri()
    }

    /// Returns the value of an attribute with the `name`. The value may have HTML/XML entities.
    ///
    /// Returns `None` if the element doesn't have an attribute with the `name`.
    #[inline]
    #[must_use]
    pub fn get_attribute(&self, name: &str) -> Option<String> {
        self.attributes.get_attribute(name)
    }

    /// Returns `true` if the element has an attribute with `name`.
    #[inline]
    #[must_use]
    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes.has_attribute(name)
    }

    /// Returns an immutable collection of tag's attributes.
    ///
    /// `get_attribute` is faster if you only need to read few attributes.
    #[inline]
    pub fn attributes(&self) -> &[Attribute<'input_token>] {
        self.attributes.to_slice()
    }

    /// Sets `value` of tag's attribute with `name`. The value may have HTML/XML entities.
    ///
    /// `"` will be entity-escaped if needed. `&` won't be escaped.
    ///
    /// If tag doesn't have an attribute with the `name`, method adds a new attribute
    /// to the tag with `name` and `value`.
    #[inline]
    pub fn set_attribute(&mut self, name: &str, value: &str) -> Result<(), AttributeNameError> {
        self.attributes
            .set_attribute(name, value, self.attributes.encoding)?;
        self.raw.set_modified();

        Ok(())
    }

    /// Removes an attribute with the `name` if it is present.
    #[inline]
    pub fn remove_attribute(&mut self, name: &str) {
        if self.attributes.remove_attribute(name) {
            self.raw.set_modified();
        }
    }

    /// Whether the tag syntactically ends with `/>`. In HTML content this is purely a decorative, unnecessary, and has no effect of any kind.
    ///
    /// The `/>` syntax only affects parsing of elements in foreign content (SVG and MathML).
    /// It will never close any HTML tags that aren't already defined as [void][spec] in HTML.
    ///
    /// This function only reports the parsed syntax, and will not report which elements are actually void in HTML.
    ///
    /// [spec]: https://html.spec.whatwg.org/multipage/syntax.html#start-tags
    ///
    /// If the `/` is part of an unquoted attribute, it's not parsed as the self-closing syntax.
    #[inline]
    pub fn self_closing(&self) -> bool {
        self.self_closing
    }

    /// If false, the tag won't be seiralized with `/>`
    ///
    /// This doesn't affect content model
    pub(crate) fn set_self_closing_syntax(&mut self, has_slash: bool) {
        self.self_closing = has_slash;
    }

    /// Inserts `content` before the start tag.
    ///
    /// Consequent calls to the method append `content` to the previously inserted content.
    #[inline]
    pub fn before(&mut self, content: &str, content_type: ContentType) {
        self.mutations
            .mutate()
            .content_before
            .push_back(StringChunk::from_str(content, content_type));
    }

    /// Inserts `content` after the start tag.
    ///
    /// Consequent calls to the method prepend `content` to the previously inserted content.
    #[inline]
    pub fn after(&mut self, content: &str, content_type: ContentType) {
        self.mutations
            .mutate()
            .content_after
            .push_front(StringChunk::from_str(content, content_type));
    }

    /// Replaces the start tag with `content`.
    ///
    /// Consequent calls to the method overwrite previous replacement content.
    #[inline]
    pub fn replace(&mut self, content: &str, content_type: ContentType) {
        self.mutations
            .mutate()
            .replace(StringChunk::from_str(content, content_type));
    }

    /// Inserts content from a [`StreamingHandler`] before the start tag.
    ///
    /// Consequent calls to the method append to the previously inserted content.
    ///
    /// Use the [`streaming!`] macro to make a `StreamingHandler` from a closure.
    pub fn streaming_before(&mut self, string_writer: Box<dyn StreamingHandler + Send + 'static>) {
        self.mutations
            .mutate()
            .content_before
            .push_back(StringChunk::stream(string_writer));
    }

    /// Inserts content from a [`StreamingHandler`] after the start tag.
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

    /// Replaces the start tag with the content from a [`StreamingHandler`].
    ///
    /// Consequent calls to the method overwrite previous replacement content.
    ///
    /// Use the [`streaming!`] macro to make a `StreamingHandler` from a closure.
    pub fn streaming_replace(&mut self, string_writer: Box<dyn StreamingHandler + Send + 'static>) {
        self.mutations
            .mutate()
            .replace(StringChunk::stream(string_writer));
    }

    /// Removes the start tag.
    #[inline]
    pub fn remove(&mut self) {
        self.mutations.mutate().remove();
    }

    fn serialize_self(
        &mut self,
        sink: &mut StreamingHandlerSink<'_>,
    ) -> Result<(), RewritingError> {
        let output_handler = sink.output_handler();

        if let Some(raw) = self.raw.original() {
            output_handler(raw);
            return Ok(());
        }

        output_handler(b"<");
        output_handler(&self.name);

        if !self.attributes.is_empty() {
            self.attributes.into_bytes(output_handler)?;

            // NOTE: attributes can be modified the way that
            // last attribute has an unquoted value. We always
            // add extra space before the `/`, because otherwise
            // it will be treated as a part of such an unquoted
            // attribute value.
            if self.self_closing {
                output_handler(b" ");
            }
        }

        if self.self_closing {
            output_handler(b"/>");
        } else {
            output_handler(b">");
        }
        Ok(())
    }

    #[cfg(test)]
    pub(crate) const fn raw_attributes(
        &self,
    ) -> (
        &'input_token Bytes<'input_token>,
        &'input_token crate::parser::AttributeBuffer,
    ) {
        self.attributes.raw_attributes()
    }

    /// Position of this tag in the source document, before any rewriting
    pub fn source_location(&self) -> SourceLocation {
        self.raw.source_location()
    }

    /// [Self::namespace_uri], but as a `CStr`
    #[inline]
    #[must_use]
    pub fn namespace_uri_c_str(&self) -> &'static std::ffi::CStr {
        self.ns.uri_c_str()
    }
}

impl_serialize!(StartTag);

impl Debug for StartTag<'_> {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StartTag")
            .field("name", &self.name())
            .field("attributes", &self.attributes())
            .field("self_closing", &self.self_closing)
            .field("at", &self.source_location())
            .finish()
    }
}
