use super::mutations::MutationsInner;
use super::{
    Attribute, AttributeNameError, ContentType, EndTag, Mutations, StartTag, StreamingHandler,
    StringChunk,
};
use crate::HandlerResult;
use crate::base::{BytesCow, SourceLocation};
use crate::rewriter::{HandlerTypes, LocalHandlerTypes};
use encoding_rs::Encoding;
use std::any::Any;
use std::borrow::Cow;
use std::fmt::{self, Debug};
use thiserror::Error;

/// An error that occurs when invalid value is provided for the tag name.
#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
pub enum TagNameError {
    /// The provided value is empty.
    #[error("Tag name can't be empty.")]
    Empty,

    /// The first character of the provided value is not an ASCII alphabetical character.
    #[error("The first character of the tag name should be an ASCII alphabetical character.")]
    InvalidFirstCharacter,

    /// The provided value contains a character that is forbidden by the HTML grammar in tag names
    /// (e.g. `'>'`).
    #[error("`{0}` character is forbidden in the tag name")]
    ForbiddenCharacter(char),

    /// The provided value contains a character that can't be represented in the document's
    /// [`encoding`].
    ///
    /// [`encoding`]: ../struct.Settings.html#structfield.encoding
    #[error(
        "The tag name contains a character that can't be represented in the document's character encoding."
    )]
    UnencodableCharacter,
}

/// An HTML element rewritable unit.
///
/// Exposes API for examination and modification of a parsed HTML element.
pub struct Element<'rewriter, 'input_token, H: HandlerTypes = LocalHandlerTypes> {
    start_tag: &'rewriter mut StartTag<'input_token>,
    end_tag_mutations: Option<Mutations>,
    modified_end_tag_name: Option<Box<[u8]>>,
    end_tag_handlers: Vec<H::EndTagHandler<'static>>,
    can_have_content: bool,
    should_remove_content: bool,
    encoding: &'static Encoding,
    user_data: Box<dyn Any>,
}

impl<'rewriter, 'input_token, H: HandlerTypes> Element<'rewriter, 'input_token, H> {
    #[inline]
    #[must_use]
    pub(crate) fn new(
        start_tag: &'rewriter mut StartTag<'input_token>,
        can_have_content: bool,
    ) -> Self {
        let encoding = start_tag.encoding();

        Element {
            start_tag,
            end_tag_mutations: None,
            modified_end_tag_name: None,
            end_tag_handlers: Vec::new(),
            can_have_content,
            should_remove_content: false,
            encoding,
            user_data: Box::new(()),
        }
    }

    fn tag_name_bytes_from_str(&self, name: &str) -> Result<BytesCow<'static>, TagNameError> {
        match name.as_bytes().first() {
            Some(ch) if !ch.is_ascii_alphabetic() => Err(TagNameError::InvalidFirstCharacter),
            Some(_) => {
                if let Some(ch) =
                    name.as_bytes().iter().copied().find(|&ch| {
                        matches!(ch, b' ' | b'\n' | b'\r' | b'\t' | b'\x0C' | b'/' | b'>')
                    })
                {
                    Err(TagNameError::ForbiddenCharacter(ch as char))
                } else {
                    // NOTE: if character can't be represented in the given
                    // encoding then encoding_rs replaces it with a numeric
                    // character reference. Character references are not
                    // supported in tag names, so we need to bail.
                    BytesCow::owned_from_str_without_replacements(name, self.encoding)
                        .map_err(|_| TagNameError::UnencodableCharacter)
                }
            }
            None => Err(TagNameError::Empty),
        }
    }

    #[inline]
    fn remove_content(&mut self) {
        self.start_tag.mutations.mutate().content_after.clear();
        if let Some(end) = self.end_tag_mutations.as_mut().and_then(|m| m.if_mutated()) {
            end.content_before.clear();
        }
        self.should_remove_content = true;
    }

    #[inline]
    fn end_tag_mutations_mut(&mut self) -> &mut MutationsInner {
        self.end_tag_mutations
            .get_or_insert_with(Mutations::new)
            .mutate()
    }

    /// Returns the tag name of the element.
    #[inline]
    #[must_use]
    pub fn tag_name(&self) -> String {
        self.start_tag.name()
    }

    /// Returns the tag name of the element, preserving its case.
    #[inline]
    #[must_use]
    pub fn tag_name_preserve_case(&self) -> String {
        self.start_tag.name_preserve_case()
    }

    /// Sets the tag name of the element.
    ///
    /// The new tag name must be in the same namespace, have the same content model, and be valid in its location.
    /// Otherwise change of the tag name may cause the resulting document to be parsed in an unexpected way,
    /// out of sync with this library.
    #[inline]
    pub fn set_tag_name(&mut self, name: &str) -> Result<(), TagNameError> {
        let name = self.tag_name_bytes_from_str(name)?;

        if self.can_have_content {
            self.modified_end_tag_name = Some((*name).into());
        }

        self.start_tag.set_name_raw(name);

        Ok(())
    }

    /// Whether the tag syntactically ends with `/>`. In HTML content this is purely a decorative, unnecessary, and has no effect of any kind.
    ///
    /// The `/>` syntax only affects parsing of elements in foreign content (SVG and MathML).
    /// It will never close any HTML tags that aren't already defined as [void][spec] in HTML.
    ///
    /// This function only reports the parsed syntax, and will not report which elements are actually void in HTML.
    /// Use [`can_have_content()`][Self::can_have_content] to check if the element is non-void.
    ///
    /// [spec]: https://html.spec.whatwg.org/multipage/syntax.html#start-tags
    ///
    /// If the `/` is part of an unquoted attribute, it's not parsed as the self-closing syntax.
    #[inline]
    #[must_use]
    pub fn is_self_closing(&self) -> bool {
        self.start_tag.self_closing()
    }

    /// Whether the element can have inner content.
    ///
    /// Returns `true` if the element isn't a [void element in HTML][void],
    /// or is in **foreign content** and doesn't have a self-closing tag (eg, `<svg />`).
    ///
    /// [void]: https://html.spec.whatwg.org/multipage/syntax.html#void-elements
    ///
    /// Note that the self-closing syntax has no effect in HTML content.
    #[inline]
    #[must_use]
    pub fn can_have_content(&self) -> bool {
        self.can_have_content
    }

    /// Returns the [namespace URI] of the element.
    ///
    /// [namespace URI]: https://developer.mozilla.org/en-US/docs/Web/API/Element/namespaceURI
    #[inline]
    #[must_use]
    pub fn namespace_uri(&self) -> &'static str {
        self.start_tag.namespace_uri()
    }

    /// Returns an immutable collection of element's attributes.
    ///
    /// `get_attribute` is faster if you only need to read few attributes.
    #[inline]
    #[must_use]
    pub fn attributes(&self) -> &[Attribute<'input_token>] {
        self.start_tag.attributes()
    }

    /// Returns the value of an attribute with the `name`. The value may have HTML/XML entities.
    ///
    /// Returns `None` if the element doesn't have an attribute with the `name`.
    #[inline]
    #[must_use]
    pub fn get_attribute(&self, name: &str) -> Option<String> {
        self.start_tag.get_attribute(name)
    }

    /// Returns `true` if the element has an attribute with `name`.
    #[inline]
    #[must_use]
    pub fn has_attribute(&self, name: &str) -> bool {
        self.start_tag.has_attribute(name)
    }

    /// Sets `value` of element's attribute with `name`. The value may have HTML/XML entities.
    ///
    /// `"` will be entity-escaped if needed. `&` won't be escaped.
    ///
    /// If element doesn't have an attribute with the `name`, method adds a new attribute
    /// to the element with `name` and `value`.
    #[inline]
    pub fn set_attribute(&mut self, name: &str, value: &str) -> Result<(), AttributeNameError> {
        self.start_tag.set_attribute(name, value)
    }

    /// Removes an attribute with the `name` if it is present.
    #[inline]
    pub fn remove_attribute(&mut self, name: &str) {
        self.start_tag.remove_attribute(name);
    }

    /// Inserts `content` before the element.
    ///
    /// Consequent calls to the method append `content` to the previously inserted content.
    ///
    /// # Example
    ///
    /// ```
    /// use lol_html::{rewrite_str, element, RewriteStrSettings};
    /// use lol_html::html_content::ContentType;
    ///
    /// let html = rewrite_str(
    ///     r#"<div id="foo"></div>"#,
    ///     RewriteStrSettings {
    ///         element_content_handlers: vec![
    ///             element!("#foo", |el| {
    ///                 el.before("<bar>", ContentType::Html);
    ///                 el.before("<qux>", ContentType::Html);
    ///                 el.before("<quz>", ContentType::Text);
    ///
    ///                 Ok(())
    ///             })
    ///         ],
    ///         ..RewriteStrSettings::new()
    ///     }
    /// ).unwrap();
    ///
    /// assert_eq!(html, r#"<bar><qux>&lt;quz&gt;<div id="foo"></div>"#);
    /// ```
    #[inline]
    pub fn before(&mut self, content: &str, content_type: ContentType) {
        self.start_tag
            .mutations
            .mutate()
            .content_before
            .push_back(StringChunk::from_str(content, content_type));
    }

    /// Inserts  content from a [`StreamingHandler`] before the element.
    ///
    /// Consequent calls to the method append to the previously inserted content.
    ///
    /// Use the [`streaming!`] macro to make a `StreamingHandler` from a closure.
    pub fn streaming_before(&mut self, string_writer: Box<dyn StreamingHandler + Send + 'static>) {
        self.start_tag
            .mutations
            .mutate()
            .content_before
            .push_back(StringChunk::stream(string_writer));
    }

    /// Inserts `content` after the element.
    ///
    /// Consequent calls to the method prepend `content` to the previously inserted content.
    ///
    /// # Example
    ///
    /// ```
    /// use lol_html::{rewrite_str, element, RewriteStrSettings};
    /// use lol_html::html_content::ContentType;
    ///
    /// let html = rewrite_str(
    ///     r#"<div id="foo"></div>"#,
    ///     RewriteStrSettings {
    ///         element_content_handlers: vec![
    ///             element!("#foo", |el| {
    ///                 el.after("<bar>", ContentType::Html);
    ///                 el.after("<qux>", ContentType::Html);
    ///                 el.after("<quz>", ContentType::Text);
    ///
    ///                 Ok(())
    ///             })
    ///         ],
    ///         ..RewriteStrSettings::new()
    ///     }
    /// ).unwrap();
    ///
    /// assert_eq!(html, r#"<div id="foo"></div>&lt;quz&gt;<qux><bar>"#);
    /// ```
    #[inline]
    pub fn after(&mut self, content: &str, content_type: ContentType) {
        self.after_chunk(StringChunk::from_str(content, content_type));
    }

    fn after_chunk(&mut self, chunk: StringChunk) {
        if self.can_have_content {
            &mut self.end_tag_mutations_mut().content_after
        } else {
            &mut self.start_tag.mutations.mutate().content_after
        }
        .push_front(chunk);
    }

    /// Inserts content from a [`StreamingHandler`] after the element.
    ///
    /// Consequent calls to the method prepend to the previously inserted content.
    ///
    ///
    /// Use the [`streaming!`] macro to make a `StreamingHandler` from a closure.
    pub fn streaming_after(&mut self, string_writer: Box<dyn StreamingHandler + Send + 'static>) {
        self.after_chunk(StringChunk::stream(string_writer));
    }

    /// Prepends `content` to the element's inner content, i.e. inserts content right after
    /// the element's start tag.
    ///
    /// Consequent calls to the method prepend `content` to the previously inserted content.
    /// A call to the method doesn't make any effect if the element is an [empty element].
    ///
    /// [empty element]: https://developer.mozilla.org/en-US/docs/Glossary/Empty_element
    ///
    /// # Example
    ///
    /// ```
    /// use lol_html::{rewrite_str, element, RewriteStrSettings};
    /// use lol_html::html_content::{ContentType, Element};
    ///
    /// let handler = |el: &mut Element| {
    ///     el.prepend("<bar>", ContentType::Html);
    ///     el.prepend("<qux>", ContentType::Html);
    ///     el.prepend("<quz>", ContentType::Text);
    ///
    ///     Ok(())
    /// };
    ///
    /// let html = rewrite_str(
    ///     r#"<div id="foo"><!-- content --></div><img>"#,
    ///     RewriteStrSettings {
    ///         element_content_handlers: vec![
    ///             element!("#foo", handler),
    ///             element!("img", handler),
    ///         ],
    ///         ..RewriteStrSettings::new()
    ///     }
    /// ).unwrap();
    ///
    /// assert_eq!(html, r#"<div id="foo">&lt;quz&gt;<qux><bar><!-- content --></div><img>"#);
    /// ```
    #[inline]
    pub fn prepend(&mut self, content: &str, content_type: ContentType) {
        self.prepend_chunk(StringChunk::from_str(content, content_type));
    }

    fn prepend_chunk(&mut self, chunk: StringChunk) {
        if self.can_have_content {
            self.start_tag.set_self_closing_syntax(false);
            self.start_tag
                .mutations
                .mutate()
                .content_after
                .push_front(chunk);
        }
    }

    /// Prepends content from a [`StreamingHandler`] to the element's inner content,
    /// i.e. inserts content right after the element's start tag.
    ///
    /// Consequent calls to the method prepend to the previously inserted content.
    /// A call to the method doesn't make any effect if the element is an [empty element].
    ///
    /// [empty element]: https://developer.mozilla.org/en-US/docs/Glossary/Empty_element
    ///
    ///
    /// Use the [`streaming!`] macro to make a `StreamingHandler` from a closure.
    pub fn streaming_prepend(&mut self, string_writer: Box<dyn StreamingHandler + Send + 'static>) {
        self.prepend_chunk(StringChunk::stream(string_writer));
    }

    /// Appends `content` to the element's inner content, i.e. inserts content right before
    /// the element's end tag.
    ///
    /// Consequent calls to the method append `content` to the previously inserted content.
    /// A call to the method doesn't make any effect if the element is an [empty element].
    ///
    /// [empty element]: https://developer.mozilla.org/en-US/docs/Glossary/Empty_element
    ///
    /// # Example
    ///
    /// ```
    /// use lol_html::{rewrite_str, element, RewriteStrSettings};
    /// use lol_html::html_content::{ContentType, Element};
    ///
    /// let handler = |el: &mut Element| {
    ///     el.append("<bar>", ContentType::Html);
    ///     el.append("<qux>", ContentType::Html);
    ///     el.append("<quz>", ContentType::Text);
    ///
    ///     Ok(())
    /// };
    ///
    /// let html = rewrite_str(
    ///     r#"<div id="foo"><!-- content --></div><img>"#,
    ///     RewriteStrSettings {
    ///         element_content_handlers: vec![
    ///             element!("#foo", handler),
    ///             element!("img", handler),
    ///         ],
    ///         ..RewriteStrSettings::new()
    ///     }
    /// ).unwrap();
    ///
    /// assert_eq!(html, r#"<div id="foo"><!-- content --><bar><qux>&lt;quz&gt;</div><img>"#);
    /// ```
    #[inline]
    pub fn append(&mut self, content: &str, content_type: ContentType) {
        self.append_chunk(StringChunk::from_str(content, content_type));
    }

    fn append_chunk(&mut self, chunk: StringChunk) {
        if self.can_have_content {
            self.start_tag.set_self_closing_syntax(false);
            self.end_tag_mutations_mut().content_before.push_back(chunk);
        }
    }

    /// Appends content from a [`StreamingHandler`] to the element's inner content,
    /// i.e. inserts content right before the element's end tag.
    ///
    /// Consequent calls to the method append to the previously inserted content.
    /// A call to the method doesn't make any effect if the element is an [empty element].
    ///
    /// [empty element]: https://developer.mozilla.org/en-US/docs/Glossary/Empty_element
    ///
    /// Use the [`streaming!`] macro to make a `StreamingHandler` from a closure.
    pub fn streaming_append(&mut self, string_writer: Box<dyn StreamingHandler + Send + 'static>) {
        self.append_chunk(StringChunk::stream(string_writer));
    }

    /// Replaces inner content of the element with `content`.
    ///
    /// Consequent calls to the method overwrite previously inserted content.
    /// A call to the method doesn't make any effect if the element is an [empty element].
    ///
    /// [empty element]: https://developer.mozilla.org/en-US/docs/Glossary/Empty_element
    ///
    /// # Example
    ///
    /// ```
    /// use lol_html::{rewrite_str, element, RewriteStrSettings};
    /// use lol_html::html_content::{ContentType, Element};
    ///
    /// let handler = |el: &mut Element| {
    ///     el.append("<!-- only one -->", ContentType::Html);
    ///     el.set_inner_content("<!-- will -->", ContentType::Html);
    ///     el.set_inner_content("<!-- survive -->", ContentType::Html);
    ///
    ///     Ok(())
    /// };
    ///
    /// let html = rewrite_str(
    ///     r#"<div id="foo"><!-- content --></div><img>"#,
    ///     RewriteStrSettings {
    ///         element_content_handlers: vec![
    ///             element!("#foo", handler),
    ///             element!("img", handler),
    ///         ],
    ///         ..RewriteStrSettings::new()
    ///     }
    /// ).unwrap();
    ///
    /// assert_eq!(html, r#"<div id="foo"><!-- survive --></div><img>"#);
    /// ```
    #[inline]
    pub fn set_inner_content(&mut self, content: &str, content_type: ContentType) {
        self.set_inner_content_chunk(StringChunk::from_str(content, content_type));
    }

    fn set_inner_content_chunk(&mut self, chunk: StringChunk) {
        if self.can_have_content {
            self.start_tag.set_self_closing_syntax(false);
            self.remove_content();
            self.start_tag
                .mutations
                .mutate()
                .content_after
                .push_front(chunk);
        }
    }

    /// Replaces inner content of the element with content from a [`StreamingHandler`].
    ///
    /// Consequent calls to the method overwrite previously inserted content.
    /// A call to the method doesn't make any effect if the element is an [empty element].
    ///
    /// [empty element]: https://developer.mozilla.org/en-US/docs/Glossary/Empty_element
    ///
    ///
    /// Use the [`streaming!`] macro to make a `StreamingHandler` from a closure.
    pub fn streaming_set_inner_content(
        &mut self,
        string_writer: Box<dyn StreamingHandler + Send + 'static>,
    ) {
        self.set_inner_content_chunk(StringChunk::stream(string_writer));
    }

    /// Replaces the element and its inner content with `content`.
    ///
    /// Consequent calls to the method overwrite previously inserted content.
    ///
    /// # Example
    ///
    /// ```
    /// use lol_html::{rewrite_str, element, RewriteStrSettings};
    /// use lol_html::html_content::ContentType;
    ///
    /// let html = rewrite_str(
    ///     r#"<div id="foo"></div>"#,
    ///     RewriteStrSettings {
    ///         element_content_handlers: vec![
    ///             element!("#foo", |el| {
    ///                 el.replace("<span></span>", ContentType::Html);
    ///                 el.replace("Hello", ContentType::Text);
    ///
    ///                 Ok(())
    ///             })
    ///         ],
    ///         ..RewriteStrSettings::new()
    ///     }
    /// ).unwrap();
    ///
    /// assert_eq!(html, r#"Hello"#);
    /// ```
    #[inline]
    pub fn replace(&mut self, content: &str, content_type: ContentType) {
        self.replace_chunk(StringChunk::from_str(content, content_type));
    }

    fn replace_chunk(&mut self, chunk: StringChunk) {
        self.start_tag.mutations.mutate().replace(chunk);

        if self.can_have_content {
            self.remove_content();
            self.end_tag_mutations_mut().remove();
        }
    }

    /// Replaces the element and its inner content with content from a [`StreamingHandler`].
    ///
    /// Consequent calls to the method overwrite previously inserted content.
    ///
    ///
    /// Use the [`streaming!`] macro to make a `StreamingHandler` from a closure.
    pub fn streaming_replace(&mut self, string_writer: Box<dyn StreamingHandler + Send + 'static>) {
        self.replace_chunk(StringChunk::stream(string_writer));
    }

    /// Removes the element and its inner content.
    #[inline]
    pub fn remove(&mut self) {
        self.start_tag.mutations.mutate().remove();

        if self.can_have_content {
            self.remove_content();
            self.end_tag_mutations_mut().remove();
        }
    }

    /// Removes the element, but keeps its content. I.e. remove start and end tags of the element.
    ///
    /// # Example
    ///
    /// ```
    /// use lol_html::{rewrite_str, element, RewriteStrSettings};
    ///
    /// let html = rewrite_str(
    ///     r#"<div><span><!-- 42 --></span></div>"#,
    ///     RewriteStrSettings {
    ///         element_content_handlers: vec![
    ///             element!("div", |el| {
    ///                 el.remove_and_keep_content();
    ///
    ///                 Ok(())
    ///             })
    ///         ],
    ///         ..RewriteStrSettings::new()
    ///     }
    /// ).unwrap();
    ///
    /// assert_eq!(html, r#"<span><!-- 42 --></span>"#);
    /// ```
    #[inline]
    pub fn remove_and_keep_content(&mut self) {
        self.start_tag.remove();

        if self.can_have_content {
            self.end_tag_mutations_mut().remove();
        }
    }

    /// Returns `true` if the element has been removed or replaced with some content.
    #[inline]
    #[must_use]
    pub fn removed(&self) -> bool {
        self.start_tag.mutations.removed()
    }

    #[inline]
    pub(crate) fn should_remove_content(&self) -> bool {
        self.should_remove_content
    }

    /// Returns the start tag.
    #[inline]
    pub fn start_tag(&mut self) -> &mut StartTag<'input_token> {
        self.start_tag
    }

    /// Returns the handlers that will run when the end tag is reached.
    ///
    /// The handlers may not run if there is no explicit end tag.
    ///
    /// You can use this to add your "on end tag" handlers.
    ///
    /// This will return `None` if the element can't have an end tag.
    ///
    /// # Example
    ///
    /// ```
    /// use lol_html::html_content::{ContentType, Element};
    /// use lol_html::{element, rewrite_str, text, RewriteStrSettings};
    /// let buffer = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    /// let html = rewrite_str(
    ///     "<span>Short</span><span><b>13</b> characters</span>",
    ///     RewriteStrSettings {
    ///         element_content_handlers: vec![
    ///             element!("span", |el: &mut Element| {
    ///                 // Truncate string for each new span.
    ///                 buffer.borrow_mut().clear();
    ///                 let buffer = buffer.clone();
    ///                 if let Some(handlers) = el.end_tag_handlers() {
    ///                     handlers.push(Box::new(move |end| {
    ///                         let s = buffer.borrow();
    ///                         if s.len() == 13 {
    ///                             // add text before the end tag
    ///                             end.before("!", ContentType::Text);
    ///                         } else {
    ///                             // replace the end tag with an uppercase version
    ///                             end.remove();
    ///                             let name = end.name().to_uppercase();
    ///                             end.after(&format!("</{}>", name), ContentType::Html);
    ///                         }
    ///                         Ok(())
    ///                     }));
    ///                 }
    ///                 Ok(())
    ///             }),
    ///             text!("span", |t| {
    ///                 // Save the text contents for the end tag handler.
    ///                 buffer.borrow_mut().push_str(t.as_str());
    ///                 Ok(())
    ///             }),
    ///         ],
    ///         ..RewriteStrSettings::new()
    ///     },
    /// )
    /// .unwrap();
    ///
    /// assert_eq!(html, "<span>Short</SPAN><span><b>13</b> characters!</span>");
    /// ```
    pub fn end_tag_handlers(&mut self) -> Option<&mut Vec<H::EndTagHandler<'static>>> {
        if self.can_have_content {
            Some(&mut self.end_tag_handlers)
        } else {
            None
        }
    }

    /// Adds a handler to run when the end tag is reached. Returns `Err` when `element.can_have_content()` is `false`.
    ///
    /// Note: currently end tag handlers are not invoked for implicitly-closed elements.
    ///
    /// Use [`end_tag!`](crate::end_tag!) macro to provide type hint for the closure's argument.
    ///
    /// # Example
    ///
    /// ```
    /// use lol_html::html_content::{ContentType, Element};
    /// use lol_html::{element, end_tag, rewrite_str, text, RewriteStrSettings};
    /// let html = rewrite_str(
    ///     "<span>hi</span>",
    ///     RewriteStrSettings {
    ///         element_content_handlers: vec![
    ///             element!("span", |el: &mut Element| {
    ///                 el.on_end_tag(end_tag!(move |end| {
    ///                     end.before("?", ContentType::Text);
    ///                     end.after("!", ContentType::Text);
    ///                     Ok(())
    ///                 }))
    ///             }),
    ///         ],
    ///         ..RewriteStrSettings::new()
    ///     },
    /// )
    /// .unwrap();
    ///
    /// assert_eq!(html, "<span>hi?</span>!");
    /// ```
    #[inline]
    pub fn on_end_tag(&mut self, handler: H::EndTagHandler<'static>) -> HandlerResult {
        if let Some(h) = self.end_tag_handlers() {
            h.push(handler);
            Ok(())
        } else {
            Err(format!("{} can't have content", self.tag_name()).into())
        }
    }

    pub(crate) fn into_end_tag_handler(self) -> Option<H::EndTagHandler<'static>> {
        let end_tag_mutations = self.end_tag_mutations;
        let modified_end_tag_name = self.modified_end_tag_name;
        let mut end_tag_handlers = self.end_tag_handlers;

        if end_tag_mutations.is_some()
            || modified_end_tag_name.is_some()
            || !end_tag_handlers.is_empty()
        {
            end_tag_handlers.insert(
                0,
                H::new_end_tag_handler(|end_tag: &mut EndTag<'_>| {
                    if let Some(name) = modified_end_tag_name {
                        end_tag.set_name_raw(Cow::from(name.into_vec()).into());
                    }

                    if let Some(mutations) = end_tag_mutations {
                        end_tag.mutations = mutations;
                    }

                    Ok(())
                }),
            );

            Some(H::combine_handlers(end_tag_handlers))
        } else {
            None
        }
    }

    /// Position of this element's start tag in the source document, before any rewriting
    ///
    /// The end of this element hasn't been parsed yet. To find it, use [`Element::end_tag_handlers`].
    #[must_use]
    pub fn source_location(&self) -> SourceLocation {
        self.start_tag.source_location()
    }

    /// [Self::namespace_uri], but as a `CStr`
    #[inline]
    #[must_use]
    pub fn namespace_uri_c_str(&self) -> &'static std::ffi::CStr {
        self.start_tag.namespace_uri_c_str()
    }
}

impl_user_data!(Element<'_, '_>);

impl<H: HandlerTypes> Debug for Element<'_, '_, H> {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Element")
            .field("tag_name", &self.tag_name())
            .field("attributes", &self.attributes())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::*;
    use crate::html_content::*;
    use crate::rewritable_units::test_utils::*;
    use crate::*;
    use encoding_rs::{EUC_JP, Encoding, UTF_8};
    use rewritable_units::StreamingHandlerSink;

    fn rewrite_element(
        html: &[u8],
        encoding: &'static Encoding,
        selector: &str,
        mut handler: impl FnMut(&mut Element<'_, '_>),
    ) -> String {
        let mut handler_called = false;

        let output = rewrite_html(
            html,
            encoding,
            vec![
                element!(selector, |el| {
                    handler_called = true;
                    handler(el);
                    Ok(())
                }),
                element!("inner-remove-me", |el| {
                    el.before("[before: should be removed]", ContentType::Text);
                    el.after("[after: should be removed]", ContentType::Text);
                    el.append("[append: should be removed]", ContentType::Text);
                    el.streaming_before(Box::new(|sink: &mut StreamingHandlerSink<'_>| {
                        sink.write_str("[before:", ContentType::Text);
                        sink.write_str(" should be removed]", ContentType::Text);
                        Ok(())
                    }));
                    Ok(())
                }),
            ],
            vec![],
        );

        assert!(handler_called, "Handler not called.");

        output
    }

    #[test]
    fn many_end_tags() {
        let output = rewrite_html(
            "<div><div class=c>…<unclosed class=c></div> <unclosed class=c></div>".as_bytes(),
            UTF_8,
            vec![
                element!("*", |el| {
                    let tag_name = el.tag_name();
                    el.before(&format!("*{tag_name}"), ContentType::Text);
                    el.end_tag_handlers()
                        .unwrap()
                        .push(Box::new(|_: &mut EndTag<'_>| Ok(())) as _);
                    el.end_tag_handlers()
                        .unwrap()
                        .push(Box::new(move |end: &mut EndTag<'_>| {
                            end.before(&format!("*{tag_name}+"), ContentType::Text);
                            end.after(&format!("/*{tag_name}"), ContentType::Text);
                            Ok(())
                        }) as _);
                    Ok(())
                }),
                element!("#notmatch", |_el| { Ok(()) }),
                element!(".c", |el| {
                    let tag_name = el.tag_name();
                    el.before(&format!(".{tag_name}"), ContentType::Text);
                    el.end_tag_handlers()
                        .unwrap()
                        .push(Box::new(move |end: &mut EndTag<'_>| {
                            end.before(&format!(".{tag_name}+"), ContentType::Text);
                            end.after(&format!("/.{tag_name}"), ContentType::Text);
                            Ok(())
                        }) as _);
                    Ok(())
                }),
                element!("div", |el| {
                    el.before("@div", ContentType::Text);
                    el.end_tag_handlers()
                        .unwrap()
                        .push(Box::new(|end: &mut EndTag<'_>| {
                            end.before("div+", ContentType::Text);
                            end.after("/div", ContentType::Text);
                            Ok(())
                        }) as _);
                    el.end_tag_handlers()
                        .unwrap()
                        .push(Box::new(|_: &mut EndTag<'_>| Ok(())) as _);
                    Ok(())
                }),
            ],
            vec![],
        );

        assert_eq!(
            "*div@div<div>*div.div@div<div class=c>…*unclosed.unclosed<unclosed class=c>*unclosed+.unclosed+*div+.div+div+</div>/div/.div/*div/.unclosed/*unclosed *unclosed.unclosed<unclosed class=c>*unclosed+.unclosed+*div+div+</div>/div/*div/.unclosed/*unclosed",
            output
        );
    }

    #[test]
    fn empty_tag_name() {
        rewrite_element(b"<div>", UTF_8, "div", |el| {
            let err = el.set_tag_name("").unwrap_err();

            assert_eq!(err, TagNameError::Empty);
        });
    }

    #[test]
    fn sanitizer_bypass1() {
        let out = rewrite_element(
            b"<math><style><img></style></math>
             <textarea><!--</textarea><img>--></textarea>
             <div><style><img></style></div>",
            UTF_8,
            "img",
            |el| el.set_tag_name("TROUBLE").unwrap(),
        );
        assert_eq!(
            out,
            "<math><style><TROUBLE></style></math>
             <textarea><!--</textarea><TROUBLE>--></textarea>
             <div><style><img></style></div>"
        );
    }

    #[test]
    fn sanitizer_bypass2() {
        let out = rewrite_element(
            b"<svg><p><style><!--</style><img>--></style>
            <math><p></p><style><!--</style><img src/onerror>--></style></math>",
            UTF_8,
            "img",
            |el| el.set_tag_name("BINGO").unwrap(),
        );
        assert_eq!(
            out,
            "<svg><p><style><!--</style><BINGO>--></style>
            <math><p></p><style><!--</style><BINGO src onerror>--></style></math>"
        );
    }

    #[test]
    fn noscript_mode() {
        let out = rewrite_element(
            br#"<noscript><p alt="</noscript><img>">"#,
            UTF_8,
            "img",
            |el| el.set_tag_name("we-have-scripts").unwrap(),
        );
        assert_eq!(out, r#"<noscript><p alt="</noscript><we-have-scripts>">"#);
    }

    #[test]
    fn parse_error_in_foreign_content() {
        let out = rewrite_element(
            br#"<svg></p><style><a id="</style><img>">"#,
            UTF_8,
            "img,a",
            |el| el.set_tag_name("HIT").unwrap(),
        );
        assert_eq!(out, r#"<svg></p><style><a id="</style><HIT>">"#);
    }

    #[test]
    fn parse_error_in_foreign_content2() {
        let out = rewrite_element(
            br#"<math><br><style><a id="</style><img>">
            <math><font kolor/><style><a/></style><body><style><a/></style></math>
            <math><font COLOR/><style><a/></style>"#,
            UTF_8,
            "img,a",
            |el| el.set_tag_name("HIT").unwrap(),
        );
        assert_eq!(
            out,
            r#"<math><br><style><a id="</style><HIT>">
            <math><font kolor/><style><HIT/></style><body><style><a/></style></math>
            <math><font COLOR/><style><a/></style>"#
        );
    }

    #[test]
    fn nested_html_namespace() {
        let out = rewrite_element(
            br#"<math><mtext><br/></mtext><style><a id="</style><img>">"#,
            UTF_8,
            "img,a",
            |el| el.set_tag_name("HIT").unwrap(),
        );
        assert_eq!(
            out,
            r#"<math><mtext><br/></mtext><style><HIT id="</style><img>">"#
        );
    }

    #[test]
    fn self_closing_script() {
        let out = rewrite_element(br"<svG><sCript/><img></script></svg>", UTF_8, "img", |el| {
            el.set_tag_name("HIT").unwrap();
        });
        assert_eq!(out, r"<svG><sCript/><HIT></script></svg>");
    }

    #[test]
    fn surprise_text_integration_point() {
        let out = rewrite_element(
            br#"<math><annotation-xml encoding="nope/html"><style><img></style></annotation-xml></math>
            <math><annotation-xml encoding="text/HTML"><style><img></style></annotation-xml></math>"#,
            UTF_8,
            "img",
            |el| el.set_tag_name("XML").unwrap(),
        );
        assert_eq!(
            out,
            r#"<math><annotation-xml encoding="nope/html"><style><XML></style></annotation-xml></math>
            <math><annotation-xml encoding="text/HTML"><style><img></style></annotation-xml></math>"#
        );
    }

    #[test]
    fn foreignobject() {
        let out = rewrite_element(
            br#"<svg><annotation-xml><foreignobject><style><!--</style><p id="--><img>">"#,
            UTF_8,
            "p,img",
            |el| el.set_tag_name("HIT").unwrap(),
        );
        assert_eq!(
            out,
            r#"<svg><annotation-xml><foreignobject><style><!--</style><HIT id="--><img>">"#
        );
    }

    #[test]
    fn foreignobject2() {
        let out = rewrite_element(
            br#"<svg><a><foreignobject><a><table><a></table><style><!--</style></svg><a id="-><img>">"#,
            UTF_8,
            "a,img",
            |el| el.set_tag_name("A").unwrap(),
        );
        assert_eq!(
            out,
            r#"<svg><A><foreignobject><A><table><A></A><style><!--</style></A><A id="-><img>">"#
        );
    }

    #[test]
    fn math_parse_error() {
        let out = rewrite_element(
            br#"<math><p></p><style><!--</style><img src/onerror>--></style></math>"#,
            UTF_8,
            "a,img",
            |el| el.set_tag_name("A").unwrap(),
        );
        assert_eq!(
            out,
            r#"<math><p></p><style><!--</style><A src onerror>--></style></math>"#
        );
    }

    #[test]
    fn roundtrip_impossible() {
        let out = rewrite_element(
            br#"<form><math><mtext></form><form><mglyph><style></math><img>"#,
            UTF_8,
            "style,img",
            |el| el.set_tag_name("no-img-here").unwrap(),
        );
        assert_eq!(
            out,
            r#"<form><math><mtext></form><form><mglyph><no-img-here></math><img>"#
        );
    }

    #[test]
    fn forbidden_characters_in_tag_name() {
        rewrite_element(b"<div>", UTF_8, "div", |el| {
            for &ch in &[' ', '\n', '\r', '\t', '\x0C', '/', '>'] {
                let err = el.set_tag_name(&format!("foo{ch}bar")).unwrap_err();

                assert_eq!(err, TagNameError::ForbiddenCharacter(ch));
            }
        });
    }

    #[test]
    fn encoding_unmappable_chars_in_tag_name() {
        rewrite_element(b"<div>", EUC_JP, "div", |el| {
            let err = el.set_tag_name("foo\u{00F8}bar").unwrap_err();

            assert_eq!(err, TagNameError::UnencodableCharacter);
        });
    }

    #[test]
    fn invalid_first_char_of_tag_name() {
        rewrite_element(b"<div>", UTF_8, "div", |el| {
            let err = el.set_tag_name("1foo").unwrap_err();

            assert_eq!(err, TagNameError::InvalidFirstCharacter);
        });
    }

    #[test]
    fn namespace_uri() {
        rewrite_element(b"<script></script>", UTF_8, "script", |el| {
            assert_eq!(el.namespace_uri(), "http://www.w3.org/1999/xhtml");
        });

        rewrite_element(b"<svg><script></script></svg>", UTF_8, "script", |el| {
            assert_eq!(el.namespace_uri(), "http://www.w3.org/2000/svg");
        });

        rewrite_element(
            b"<svg><foreignObject><script></script></foreignObject></svg>",
            UTF_8,
            "script",
            |el| {
                assert_eq!(el.namespace_uri(), "http://www.w3.org/1999/xhtml");
            },
        );

        rewrite_element(b"<math><script></script></math>", UTF_8, "script", |el| {
            assert_eq!(el.namespace_uri(), "http://www.w3.org/1998/Math/MathML");
        });
    }

    #[test]
    fn empty_attr_name() {
        rewrite_element(b"<div>", UTF_8, "div", |el| {
            let err = el.set_attribute("", "").unwrap_err();

            assert_eq!(err, AttributeNameError::Empty);
        });
    }

    #[test]
    fn forbidden_characters_in_attr_name() {
        rewrite_element(b"<div>", UTF_8, "div", |el| {
            for &ch in &[' ', '\n', '\r', '\t', '\x0C', '/', '>', '='] {
                let err = el.set_attribute(&format!("foo{ch}bar"), "").unwrap_err();

                assert_eq!(err, AttributeNameError::ForbiddenCharacter(ch));
            }
        });
    }

    #[test]
    fn encoding_unmappable_character_in_attr_name() {
        rewrite_element(b"<div>", EUC_JP, "div", |el| {
            let err = el.set_attribute("foo\u{00F8}bar", "").unwrap_err();

            assert_eq!(err, AttributeNameError::UnencodableCharacter);
        });
    }

    #[test]
    fn tag_name_getter_and_setter() {
        for (html, enc) in encoded("<FooǼ><div><span></span></div></FooǼ>") {
            let output = rewrite_element(&html, enc, "fooǼ", |el| {
                assert_eq!(el.tag_name(), "fooǼ", "Encoding: {}", enc.name());

                el.set_tag_name("BaZǽ").unwrap();

                assert_eq!(el.tag_name(), "bazǽ", "Encoding: {}", enc.name());
            });

            assert_eq!(output, "<BaZǽ><div><span></span></div></BaZǽ>");
        }
    }

    #[test]
    fn attribute_list() {
        for (html, enc) in encoded("<Foo Fooα1=Barβ1 Fooγ2=Barδ2>") {
            rewrite_element(&html, enc, "foo", |el| {
                assert_eq!(el.attributes().len(), 2, "Encoding: {}", enc.name());
                assert_eq!(
                    el.attributes()[0].name(),
                    "fooα1",
                    "Encoding: {}",
                    enc.name()
                );
                assert_eq!(
                    el.attributes()[1].name(),
                    "fooγ2",
                    "Encoding: {}",
                    enc.name()
                );

                assert_eq!(
                    el.attributes()[0].value(),
                    "Barβ1",
                    "Encoding: {}",
                    enc.name()
                );

                assert_eq!(
                    el.attributes()[1].value(),
                    "Barδ2",
                    "Encoding: {}",
                    enc.name()
                );
            });
        }
    }

    #[test]
    fn get_attrs() {
        for (html, enc) in encoded("<Foo Fooα1=Barβ1 Fooγ2=Barδ2>") {
            rewrite_element(&html, enc, "foo", |el| {
                assert_eq!(
                    el.get_attribute("fOoα1").unwrap(),
                    "Barβ1",
                    "Encoding: {}",
                    enc.name()
                );

                assert_eq!(
                    el.get_attribute("Fooα1").unwrap(),
                    "Barβ1",
                    "Encoding: {}",
                    enc.name()
                );

                assert_eq!(
                    el.get_attribute("FOOγ2").unwrap(),
                    "Barδ2",
                    "Encoding: {}",
                    enc.name()
                );

                assert_eq!(
                    el.get_attribute("fooγ2").unwrap(),
                    "Barδ2",
                    "Encoding: {}",
                    enc.name()
                );

                assert_eq!(el.get_attribute("foo3"), None, "Encoding: {}", enc.name());
            });
        }
    }

    #[test]
    fn has_attr() {
        for (html, enc) in encoded("<Foo FooѦ1=Bar1 FooѤ2=Bar2>") {
            rewrite_element(&html, enc, "foo", |el| {
                assert!(el.has_attribute("FOoѦ1"), "Encoding: {}", enc.name());
                assert!(el.has_attribute("fooѦ1"), "Encoding: {}", enc.name());
                assert!(el.has_attribute("FOOѤ2"), "Encoding: {}", enc.name());
                assert!(!el.has_attribute("foo3"), "Encoding: {}", enc.name());
            });
        }
    }

    #[test]
    fn set_attr() {
        for (html, enc) in encoded("<div҈>") {
            rewrite_element(&html, enc, "div҈", |el| {
                el.set_attribute("FooѴ", "҈Bar1҈").unwrap();

                assert_eq!(
                    el.get_attribute("fooѴ").unwrap(),
                    "҈Bar1҈",
                    "Encoding: {}",
                    enc.name()
                );

                el.set_attribute("fOOѴ", "Bar2").unwrap();

                assert_eq!(
                    el.get_attribute("fooѴ").unwrap(),
                    "Bar2",
                    "Encoding: {}",
                    enc.name()
                );
            });
        }
    }

    #[test]
    fn remove_attr() {
        for (html, enc) in encoded("<Foo Foo1இ=Bar1 Foo2இ=Bar2>") {
            rewrite_element(&html, enc, "foo", |el| {
                el.remove_attribute("Unknown");

                assert_eq!(el.attributes().len(), 2, "Encoding: {}", enc.name());

                el.remove_attribute("Foo1இ");

                assert_eq!(el.attributes().len(), 1, "Encoding: {}", enc.name());
                assert_eq!(el.get_attribute("foo1இ"), None, "Encoding: {}", enc.name());

                el.remove_attribute("FoO2இ");

                assert!(el.attributes().is_empty(), "Encoding: {}", enc.name());
                assert_eq!(el.get_attribute("foo2இ"), None, "Encoding: {}", enc.name());
            });
        }
    }

    #[test]
    fn insert_content_before() {
        for (html, enc) in encoded("<div><span>ĥi</span></div>") {
            let output = rewrite_element(&html, enc, "span", |el| {
                el.before("<imgĤ>", ContentType::Html);
                el.before("<imgĤ>", ContentType::Text);
            });

            assert_eq!(output, "<div><imgĤ>&lt;imgĤ&gt;<span>ĥi</span></div>");
        }
    }

    #[test]
    fn prepend_content() {
        for (html, enc) in encoded("<div><span>ĥi</span></div>") {
            let output = rewrite_element(&html, enc, "span", |el| {
                el.prepend("<imgĤ>", ContentType::Html);
                el.prepend("<imgĤ>", ContentType::Text);
            });

            assert_eq!(output, "<div><span>&lt;imgĤ&gt;<imgĤ>ĥi</span></div>");
        }
    }

    #[test]
    fn append_content() {
        for (html, enc) in encoded("<div><span>ĥi</span></div>") {
            let output = rewrite_element(&html, enc, "span", |el| {
                el.append("<imgĤ>", ContentType::Html);
                el.append("<imgĤ>", ContentType::Text);
            });

            assert_eq!(output, "<div><span>ĥi<imgĤ>&lt;imgĤ&gt;</span></div>");
        }
    }

    #[test]
    fn insert_content_after() {
        for (html, enc) in encoded("<div><span>ĥi</span></div>") {
            let output = rewrite_element(&html, enc, "span", |el| {
                el.after("<imgĤ>", ContentType::Html);
                el.after("<imgĤ>", ContentType::Text);
            });

            assert_eq!(output, "<div><span>ĥi</span>&lt;imgĤ&gt;<imgĤ></div>");
        }
    }

    #[test]
    fn set_content_after() {
        for (html, enc) in
            encoded("<div><span>Hi<inner-remove-me>RemoveŴ</inner-remove-me></span></div>")
        {
            let output = rewrite_element(&html, enc, "span", |el| {
                el.streaming_prepend(streaming!(|s| {
                    s.write_utf8_chunk(b"<prepended>", ContentType::Html)?;
                    Ok(())
                }));
                el.append("<appended>", ContentType::Html);
                el.set_inner_content("<imgŵ>", ContentType::Html);
                el.set_inner_content("<imgŵ>", ContentType::Text);
            });

            assert_eq!(output, "<div><span>&lt;imgŵ&gt;</span></div>");

            let output = rewrite_element(&html, enc, "span", |el| {
                el.prepend("<prepended>", ContentType::Html);
                el.append("<appended>", ContentType::Html);
                el.set_inner_content("<imgŵ>", ContentType::Text);
                el.set_inner_content("<imgŵ>", ContentType::Html);
            });

            assert_eq!(output, "<div><span><imgŵ></span></div>");
        }
    }

    #[test]
    fn replace() {
        for (html, enc) in
            encoded("<div><span>Hi<inner-remove-me>Remove㘗</inner-remove-me></span></div>")
        {
            let output = rewrite_element(&html, enc, "span", |el| {
                el.prepend("<prepended>", ContentType::Html);
                el.append("<appended>", ContentType::Html);
                el.replace("<img㘘>", ContentType::Html);
                el.replace("<img㘘>", ContentType::Text);

                assert!(el.removed());
            });

            assert_eq!(output, "<div>&lt;img㘘&gt;</div>");

            let output = rewrite_element(&html, enc, "span", |el| {
                el.prepend("<prepended>", ContentType::Html);
                el.append("<appended>", ContentType::Html);
                el.replace("<img㘘>", ContentType::Text);
                el.replace("<img㘘>", ContentType::Html);

                assert!(el.removed());
            });

            assert_eq!(output, "<div><img㘘></div>");
        }
    }

    #[test]
    fn remove() {
        for (html, enc) in
            encoded("<div><span㗵>Hi<inner-remove-me>Remove</inner-remove-me></span㗵></div>")
        {
            let output = rewrite_element(&html, enc, "span㗵", |el| {
                el.prepend("<prepended>", ContentType::Html);
                el.append("<appended>", ContentType::Html);
                el.remove();

                assert!(el.removed());
            });

            assert_eq!(output, "<div></div>");
        }
    }

    #[test]
    fn remove_with_unfinished_end_tag() {
        for (html, enc) in encoded("<div><span㚴>Heello</span㚴  ") {
            let output = rewrite_element(&html, enc, "span㚴", |el| {
                el.remove();

                assert!(el.removed());
            });

            assert_eq!(output, "<div>");
        }
    }

    #[test]
    fn remove_and_keep_content() {
        for (html, enc) in encoded("<div><spanЫ>Hi</spanЫ></div>") {
            let output = rewrite_element(&html, enc, "spanЫ", |el| {
                el.prepend("<prepended>", ContentType::Html);
                el.append("<appended>", ContentType::Html);
                el.remove_and_keep_content();

                assert!(el.removed());
            });

            assert_eq!(output, "<div><prepended>Hi<appended></div>");
        }
    }

    #[test]
    fn multiple_consequent_removes() {
        let output = rewrite_html(
            b"<div><span>42</span></div><h1>Hello</h1><h2>Hello2</h2>",
            UTF_8,
            vec![
                element!("div", |el| {
                    el.replace("hey & ya", ContentType::Html);
                    Ok(())
                }),
                element!("h1", |el| {
                    el.remove();
                    Ok(())
                }),
                element!("h2", |el| {
                    el.remove_and_keep_content();
                    Ok(())
                }),
            ],
            vec![],
        );

        assert_eq!(output, "hey & yaHello2");
    }

    #[test]
    fn void_element() {
        let output = rewrite_element(b"<img><span>Hi</span></img>", UTF_8, "img", |el| {
            el.after("<!--after-->", ContentType::Html);
            el.append("<!--append-->", ContentType::Html);
            el.prepend("<!--prepend-->", ContentType::Html);
            el.set_inner_content("<!--set_inner_content-->", ContentType::Html);
            el.set_tag_name("img-foo").unwrap();
        });

        assert_eq!(output, "<img-foo><!--after--><span>Hi</span></img>");
    }

    #[test]
    fn self_closing_element() {
        let output = rewrite_element(b"<svg><foo/>Hi</foo></svg>", UTF_8, "foo", |el| {
            el.after("->", ContentType::Html);
            el.streaming_after(streaming!(|sink| {
                sink.write_str("er-", ContentType::Html);
                Ok(())
            }));
            el.after("t", ContentType::Html);
            el.streaming_after(streaming!(|sink| {
                sink.write_str("af", ContentType::Html);
                Ok(())
            }));
            el.after("<!--", ContentType::Html);
            el.set_tag_name("bar").unwrap();
        });

        assert_eq!(output, "<svg><bar/><!--after-->Hi</foo></svg>");
    }

    #[test]
    fn user_data() {
        rewrite_element(b"<div><span>Hi</span></div>", UTF_8, "span", |el| {
            el.set_user_data(42usize);

            assert_eq!(*el.user_data().downcast_ref::<usize>().unwrap(), 42usize);

            *el.user_data_mut().downcast_mut::<usize>().unwrap() = 1337usize;

            assert_eq!(*el.user_data().downcast_ref::<usize>().unwrap(), 1337usize);
        });
    }

    #[test]
    fn on_end_tag_handlers() {
        let handler = |el: &mut Element<'_, '_>| {
            el.end_tag_handlers().unwrap().push(Box::new(move |end| {
                end.before("X", ContentType::Html);
                Ok(())
            }));

            el.on_end_tag(Box::new(move |end| {
                end.before("Y", ContentType::Html);
                Ok(())
            }))
            .unwrap();

            el.on_end_tag(Box::new(move |end| {
                end.before("Z", ContentType::Html);
                Ok(())
            }))
            .unwrap();
        };

        let res = rewrite_element(b"<div>foo</div>", UTF_8, "div", handler);

        assert_eq!(res, "<div>fooXYZ</div>");
    }

    mod serialization {
        use super::*;

        const HTML: &str = r#"<a a1='foo " baré " baz' / a2="foo ' bar ' baz" a3=foo/bar a4></a>"#;
        const SELECTOR: &str = "a";

        macro_rules! test {
            ($handler:expr, $expected:expr) => {
                for (html, enc) in encoded(HTML) {
                    assert_eq!(rewrite_element(&html, enc, SELECTOR, $handler), $expected);
                }
            };
        }

        #[test]
        fn parsed() {
            test!(
                |el| {
                    assert_eq!(el.get_attribute("a1").unwrap(), "foo \" baré \" baz");
                    assert_eq!(el.get_attribute("a3").unwrap(), "foo/bar");
                    assert_eq!(el.get_attribute("a4").unwrap(), "");
                },
                r#"<a a1='foo " baré " baz' / a2="foo ' bar ' baz" a3=foo/bar a4></a>"#
            );
        }

        #[test]
        fn modified_name() {
            test!(
                |el| {
                    el.set_tag_name("div").unwrap();
                },
                r#"<div a1='foo " baré " baz' a2="foo ' bar ' baz" a3=foo/bar a4></div>"#
            );
        }

        #[test]
        fn modified_single_quoted_attr() {
            test!(
                |el| {
                    el.set_attribute("a2", "foo ' bar ' baz42").unwrap();
                },
                r#"<a a1='foo " baré " baz' a2="foo ' bar ' baz42" a3=foo/bar a4></a>"#
            );
        }

        #[test]
        fn modified_double_quoted_attr() {
            test!(
                |el| {
                    el.set_attribute("a2", "foo ' bar ' baz42").unwrap();
                },
                r#"<a a1='foo " baré " baz' a2="foo ' bar ' baz42" a3=foo/bar a4></a>"#
            );
        }

        #[test]
        fn modified_unquoted_attr() {
            test!(
                |el| {
                    el.set_attribute("a3", "foo/bar42").unwrap();
                },
                r#"<a a1='foo " baré " baz' a2="foo ' bar ' baz" a3="foo/bar42" a4></a>"#
            );
        }

        #[test]
        fn set_value_for_attr_without_value() {
            test!(
                |el| {
                    el.set_attribute("a4", "42").unwrap();
                },
                r#"<a a1='foo " baré " baz' a2="foo ' bar ' baz" a3=foo/bar a4="42"></a>"#
            );
        }

        #[test]
        fn add_attr() {
            test!(
                |el| {
                    el.set_attribute("a5", r#"42'"42"#).unwrap();
                },
                r#"<a a1='foo " baré " baz' a2="foo ' bar ' baz" a3=foo/bar a4 a5="42'&quot;42"></a>"#
            );
        }

        #[test]
        fn self_closing_flag() {
            // NOTE: we should add space between valueless attr and self-closing slash
            // during serialization. Otherwise, it will be interpreted as a part of the
            // attribute name.
            let mut output = rewrite_element(b"<img a1=42 a2 />", UTF_8, "img", |el| {
                el.set_attribute("a1", "foo").unwrap();
            });

            assert_eq!(output, r#"<img a1="foo" a2 />"#);

            // NOTE: but we shouldn't add space if there are no attributes.
            output = rewrite_element(b"<img a1 />", UTF_8, "img", |el| {
                el.remove_attribute("a1");
            });

            assert_eq!(output, r"<img/>");
        }

        #[test]
        fn value_trailing_slash() {
            let mut output = rewrite_element(b"<img path=//>", UTF_8, "img", |el| {
                assert_eq!(el.get_attribute("path").unwrap(), "//");
                el.set_attribute("slash", "/").unwrap();

                assert!(!el.can_have_content());
            });

            assert_eq!(output, r#"<img path=// slash="/">"#);

            output = rewrite_element(b"<img path=//>", UTF_8, "img", |el| {
                el.remove_attribute("path");

                assert!(!el.can_have_content());
            });

            assert_eq!(output, r"<img>");
        }

        #[test]
        fn remove_non_existent_attr() {
            test!(
                |el| {
                    el.remove_attribute("a5");
                },
                r#"<a a1='foo " baré " baz' / a2="foo ' bar ' baz" a3=foo/bar a4></a>"#
            );
        }

        #[test]
        fn without_attrs() {
            test!(
                |el| {
                    assert!(el.can_have_content());

                    for name in &["a1", "a2", "a3", "a4"] {
                        el.remove_attribute(name);
                    }
                },
                "<a></a>"
            );
        }

        #[test]
        fn with_before_and_prepend() {
            test!(
                |el| {
                    el.before("<span>", ContentType::Text);
                    el.before("<div>Hey</div>", ContentType::Html);
                    el.before("<foo>", ContentType::Html);
                    el.prepend("</foo>", ContentType::Html);
                    el.prepend("<!-- 42 -->", ContentType::Html);
                    el.prepend("<foo & bar>", ContentType::Text);
                },
                concat!(
                    "&lt;span&gt;<div>Hey</div><foo>",
                    r#"<a a1='foo " baré " baz' / a2="foo ' bar ' baz" a3=foo/bar a4>"#,
                    "&lt;foo &amp; bar&gt;<!-- 42 --></foo>",
                    "</a>"
                )
            );
        }

        #[test]
        fn with_after_and_append() {
            test!(
                |el| {
                    el.append("<span>", ContentType::Text);
                    el.append("<div>Hey</div>", ContentType::Html);
                    el.append("<foo>", ContentType::Html);
                    el.after("</foo>", ContentType::Html);
                    el.after("<!-- 42 -->", ContentType::Html);
                    el.after("<foo & bar>", ContentType::Text);
                },
                concat!(
                    r#"<a a1='foo " baré " baz' / a2="foo ' bar ' baz" a3=foo/bar a4>"#,
                    "&lt;span&gt;<div>Hey</div><foo>",
                    "</a>",
                    "&lt;foo &amp; bar&gt;<!-- 42 --></foo>",
                )
            );
        }

        #[test]
        fn removed() {
            test!(
                |el| {
                    assert!(!el.removed());

                    el.remove();

                    assert!(el.removed());

                    el.before("<before>", ContentType::Html);
                    el.after("<after>", ContentType::Html);
                },
                "<before><after>"
            );
        }

        #[test]
        fn replaced_with_text() {
            test!(
                |el| {
                    el.before("<before>", ContentType::Html);
                    el.after("<after>", ContentType::Html);

                    assert!(!el.removed());

                    el.replace("<div></div>", ContentType::Html);
                    el.replace("<!--42-->", ContentType::Html);
                    el.replace("<foo & bar>", ContentType::Text);

                    assert!(el.removed());
                },
                "<before>&lt;foo &amp; bar&gt;<after>"
            );
        }

        #[test]
        fn replaced_with_html() {
            test!(
                |el| {
                    el.before("<before>", ContentType::Html);
                    el.after("<after>", ContentType::Html);

                    assert!(!el.removed());

                    el.replace("<div></div>", ContentType::Html);
                    el.replace("<!--42-->", ContentType::Html);
                    el.replace("<foo & bar>", ContentType::Html);

                    assert!(el.removed());
                },
                "<before><foo & bar><after>"
            );
        }
    }

    mod location_spans {
        use super::*;
        use encoding_rs::WINDOWS_1252;

        #[test]
        fn tags() {
            let raw_input = r"<html>
                <div line=2>
                    <a line=3><span line=3 />line 3</span></a>
                </div>
                ";
            let output = rewrite_html(
                raw_input.as_bytes(),
                UTF_8,
                vec![element!("*", |el: &mut Element<'_, '_>| {
                    let loc = el.source_location();
                    el.set_attribute("at", &loc.to_string()).unwrap();
                    el.set_attribute("look", &raw_input[loc.bytes()]).unwrap();
                    if el.can_have_content() {
                        el.on_end_tag(Box::new(|end| {
                            let tag = &raw_input[end.source_location().bytes()];
                            assert_eq!("</", &tag[0..2]);
                            assert_eq!(b'>', *tag.as_bytes().last().unwrap());
                            Ok(())
                        }))?;
                    }
                    Ok(())
                })],
                vec![],
            );

            assert_eq!(
                output,
                r#"<html at="0B...6B" look="<html>">
                <div line=2 at="23B...35B" look="<div line=2>">
                    <a line=3 at="56B...66B" look="<a line=3>"><span line=3 at="66B...81B" look="<span line=3 />" />line 3</span></a>
                </div>
                "#
            );
        }

        #[test]
        fn text_and_comments() {
            let mut raw_input = Vec::from(
                br#"
                <!doctype>
                <html>l1
                <meta charset="iso-8859-1">
                l2 </>x
                <p>l3</p><!-- l4 -->
                <svg><![CDATA[
                "#,
            );
            raw_input.extend(127..=255);
            raw_input.extend_from_slice(
                br"
                l5
                ]]></svg>
                ",
            );
            let mut range_start = None;
            let mut prev_range_end = 0;
            let output = rewrite_html(
                &raw_input,
                WINDOWS_1252,
                vec![],
                vec![
                    doc_comments!(|c| {
                        let loc = c.source_location();
                        let raw = &raw_input[loc.bytes()];
                        assert_eq!(&raw[..4], b"<!--");
                        assert_eq!(&raw[raw.len() - 3..], b"-->");
                        c.set_text(&loc.to_string()).unwrap();
                        Ok(())
                    }),
                    doc_text!(|t| {
                        let loc = t.source_location().bytes();
                        let start = *range_start.get_or_insert(loc.start);
                        assert!(loc.start >= start);
                        assert!(loc.end >= start);
                        assert!(loc.start >= prev_range_end);
                        assert!(loc.end >= prev_range_end);
                        prev_range_end = loc.end;

                        assert!(
                            raw_input[start..loc.end]
                                .iter()
                                .all(|&b| b != b'<' && b != b'>')
                        );

                        if t.last_in_text_node() {
                            t.set_str(format!("{start}..{}\n", loc.end));
                            range_start = None;
                        } else {
                            t.remove();
                        }
                        Ok(())
                    }),
                ],
            );

            assert_eq!(
                output,
                "0..17\n<!doctype>27..44\n<html>50..69\n<meta charset=\"iso-8859-1\">96..116\n</>119..137\n<p>140..142\n</p><!--146B...157B-->157..174\n<svg><![CDATA[188..370\n]]></svg>379..396\n",
            );
        }
    }
}
