use crate::rewritable_units::{Comment, Doctype, DocumentEnd, Element, EndTag, TextChunk};
use crate::selectors_vm::Selector;
// N.B. `use crate::` will break this because the constructor is not public, only the struct itself
use super::AsciiCompatibleEncoding;
use std::borrow::Cow;
use std::error::Error;

/// Trait used to parameterize the type of handlers used in the rewriter.
///
/// This trait is meant to be an implementation detail for the [`Send`-compatible type aliases](crate::send).
/// We don't recommend writing code generic over [`HandlerTypes`], because it makes working with closures much more difficult.
///
/// Many types like [`Element`] and [`ElementHandler`] have a hidden generic type that defaults to `LocalHandlerTypes`.
/// If you need to use `Send`-able handlers, remove the default type by replacing it with `_`, e.g. `Element<'_, '_, _>`.
#[diagnostic::on_unimplemented(
    note = "If `{Self}` is a generic type, add `{Self}: HandlerTypes` trait bound, otherwise replace `{Self}` with `LocalHandlerTypes`",
    note = "The concrete type of `{Self}` can only be either `LocalHandlerTypes` to allow non-`Send` closures in content handlers, or `SendHandlerTypes` to require `Send` closures"
)]
pub trait HandlerTypes: Sized {
    /// Handler type for [`Doctype`].
    type DoctypeHandler<'handler>: FnMut(&mut Doctype<'_>) -> HandlerResult + 'handler;
    /// Handler type for [`Comment`].
    ///
    /// The entire content of the comment will be buffered.
    type CommentHandler<'handler>: FnMut(&mut Comment<'_>) -> HandlerResult + 'handler;
    /// Handler type for [`TextChunk`] fragments. Beware: this is tricky to use.
    ///
    /// The text chunks are **not** text DOM nodes. They are fragments of text nodes, split at arbitrary points.
    ///
    /// See [`TextChunk`] documentation for more info. See also [`TextChunk::last_in_text_node()`].
    type TextHandler<'handler>: FnMut(&mut TextChunk<'_>) -> HandlerResult + 'handler;
    /// Handler type for [`Element`].
    type ElementHandler<'handler>: FnMut(&mut Element<'_, '_, Self>) -> HandlerResult + 'handler;
    /// Handler type for [`EndTag`].
    type EndTagHandler<'handler>: FnOnce(&mut EndTag<'_>) -> HandlerResult + 'handler;
    /// Handler type for [`DocumentEnd`].
    type EndHandler<'handler>: FnOnce(&mut DocumentEnd<'_>) -> HandlerResult + 'handler;

    // Inside the HTML rewriter we need to create handlers, and they need to be the most constrained
    // possible version of a handler (i.e. if we have `Send` and non-`Send` handlers we need to
    // create a `Send` handler to make it compatible with both classes of handlers), so that's
    // what we offer below.
    //
    // Note that in the HTML rewriter all we have is an abstract `H` that implements `HandlerTypes`.
    // Therefore, there is no direct way of create a handler that is compatible with *all* possible
    // implementations of `HandlerTypes`, so each implementation of `HandlerTypes` needs to provide
    // a way to create a handler compatible with itself.

    #[doc(hidden)]
    fn new_end_tag_handler<'handler>(
        handler: impl IntoHandler<EndTagHandlerSend<'handler>>,
    ) -> Self::EndTagHandler<'handler>;

    #[doc(hidden)]
    fn new_element_handler<'handler>(
        handler: impl IntoHandler<ElementHandlerSend<'handler, Self>>,
    ) -> Self::ElementHandler<'handler>;

    /// Creates a handler by running multiple handlers in sequence.
    #[doc(hidden)]
    fn combine_handlers(handlers: Vec<Self::EndTagHandler<'_>>) -> Self::EndTagHandler<'_>;
}

/// Handler type for non-[`Send`]able [`HtmlRewriter`](crate::HtmlRewriter)s.
pub struct LocalHandlerTypes {}

impl HandlerTypes for LocalHandlerTypes {
    type DoctypeHandler<'h> = DoctypeHandler<'h>;
    type CommentHandler<'h> = CommentHandler<'h>;
    type TextHandler<'h> = TextHandler<'h>;
    type ElementHandler<'h> = ElementHandler<'h>;
    type EndTagHandler<'h> = EndTagHandler<'h>;
    type EndHandler<'h> = EndHandler<'h>;

    fn new_end_tag_handler<'h>(
        handler: impl IntoHandler<EndTagHandlerSend<'h>>,
    ) -> Self::EndTagHandler<'h> {
        handler.into_handler()
    }

    fn new_element_handler<'h>(
        handler: impl IntoHandler<ElementHandlerSend<'h, Self>>,
    ) -> Self::ElementHandler<'h> {
        handler.into_handler()
    }

    fn combine_handlers(handlers: Vec<Self::EndTagHandler<'_>>) -> Self::EndTagHandler<'_> {
        Box::new(move |end_tag: &mut EndTag<'_>| {
            for handler in handlers {
                handler(end_tag)?;
            }

            Ok(())
        })
    }
}

/// Marker type for sendable handlers. Use aliases from the [`send`](crate::send) module.
#[doc(hidden)]
pub struct SendHandlerTypes {}

impl HandlerTypes for SendHandlerTypes {
    type DoctypeHandler<'h> = DoctypeHandlerSend<'h>;
    type CommentHandler<'h> = CommentHandlerSend<'h>;
    type TextHandler<'h> = TextHandlerSend<'h>;
    type ElementHandler<'h> = ElementHandlerSend<'h, Self>;
    type EndTagHandler<'h> = EndTagHandlerSend<'h>;
    type EndHandler<'h> = EndHandlerSend<'h>;

    fn new_end_tag_handler<'h>(
        handler: impl IntoHandler<Self::EndTagHandler<'h>>,
    ) -> Self::EndTagHandler<'h> {
        handler.into_handler()
    }

    fn new_element_handler<'h>(
        handler: impl IntoHandler<Self::ElementHandler<'h>>,
    ) -> Self::ElementHandler<'h> {
        handler.into_handler()
    }

    fn combine_handlers(handlers: Vec<Self::EndTagHandler<'_>>) -> Self::EndTagHandler<'_> {
        Box::new(move |end_tag: &mut EndTag<'_>| {
            for handler in handlers {
                handler(end_tag)?;
            }

            Ok(())
        })
    }
}

/// The result of a handler.
pub type HandlerResult = Result<(), Box<dyn Error + Send + Sync + 'static>>;

/// Boxed closure for handling the [document type declaration].
///
/// [document type declaration]: https://developer.mozilla.org/en-US/docs/Glossary/Doctype
pub type DoctypeHandler<'h> = Box<dyn FnMut(&mut Doctype<'_>) -> HandlerResult + 'h>;
/// Boxed closure for handling HTML comments.
pub type CommentHandler<'h> = Box<dyn FnMut(&mut Comment<'_>) -> HandlerResult + 'h>;
/// Boxed closure for handling text chunks present the HTML.
pub type TextHandler<'h> = Box<dyn FnMut(&mut TextChunk<'_>) -> HandlerResult + 'h>;
/// Boxed closure for handling elements matched by a selector.
pub type ElementHandler<'h, H = LocalHandlerTypes> =
    Box<dyn FnMut(&mut Element<'_, '_, H>) -> HandlerResult + 'h>;
/// Boxed closure for handling end tags.
pub type EndTagHandler<'h> = Box<dyn FnOnce(&mut EndTag<'_>) -> HandlerResult + 'h>;
/// Boxed closure for handling the document end. This is called after the last chunk is processed.
pub type EndHandler<'h> = Box<dyn FnOnce(&mut DocumentEnd<'_>) -> HandlerResult + 'h>;

/// [Sendable](crate::send) boxed closure for handling the [document type declaration].
///
/// [document type declaration]: https://developer.mozilla.org/en-US/docs/Glossary/Doctype
///
/// See also non-sendable [`DoctypeHandler`](crate::DoctypeHandler).
pub type DoctypeHandlerSend<'h> = Box<dyn FnMut(&mut Doctype<'_>) -> HandlerResult + Send + 'h>;
/// [Sendable](crate::send) boxed closure for handling HTML comments.
///
/// See also non-sendable [`CommentHandler`](crate::CommentHandler).
pub type CommentHandlerSend<'h> = Box<dyn FnMut(&mut Comment<'_>) -> HandlerResult + Send + 'h>;
/// [Sendable](crate::send) boxed closure for handling text chunks](TextChunk) present the HTML.
///
/// See also non-sendable [`TextHandler`](crate::TextHandler).
pub type TextHandlerSend<'h> = Box<dyn FnMut(&mut TextChunk<'_>) -> HandlerResult + Send + 'h>;
/// [Sendable](crate::send) boxed closure for handling elements matched by a selector.
pub type ElementHandlerSend<'h, H = SendHandlerTypes> =
    Box<dyn FnMut(&mut Element<'_, '_, H>) -> HandlerResult + Send + 'h>;
/// [Sendable](crate::send) boxed closure for handling end tags.
///
/// See also non-sendable [`EndTagHandler`](crate::EndTagHandler).
pub type EndTagHandlerSend<'h> = Box<dyn FnOnce(&mut EndTag<'_>) -> HandlerResult + Send + 'h>;
/// [Sendable](crate::send) boxed closure for handling the document end. This is called after the last chunk is processed.
///
/// See also non-sendable [`EndHandler`](crate::EndHandler).
pub type EndHandlerSend<'h> = Box<dyn FnOnce(&mut DocumentEnd<'_>) -> HandlerResult + Send + 'h>;

/// Trait that allows closures to be used as handlers
#[diagnostic::on_unimplemented(
    message = "Handler could not be made from `{Self}`\nThe internal `IntoHandler` trait is implemented for closures like `FnMut(&mut _) -> HandlerResult` and `FnOnce(&mut _) -> HandlerResult`, with `+ Send` if needed",
    note = "Ensure that the closure's arguments are correct (add explicit parameter types if needed) and that it implements `Send` if using `Send`-able handlers"
)]
#[doc(hidden)]
pub trait IntoHandler<T: Sized> {
    fn into_handler(self) -> T;
}

impl<'h, F: FnMut(&mut Doctype<'_>) -> HandlerResult + 'h> IntoHandler<DoctypeHandler<'h>> for F {
    fn into_handler(self) -> DoctypeHandler<'h> {
        Box::new(self)
    }
}

impl<'h, F: FnMut(&mut Comment<'_>) -> HandlerResult + 'h> IntoHandler<CommentHandler<'h>> for F {
    fn into_handler(self) -> CommentHandler<'h> {
        Box::new(self)
    }
}

impl<'h, F: FnMut(&mut TextChunk<'_>) -> HandlerResult + 'h> IntoHandler<TextHandler<'h>> for F {
    fn into_handler(self) -> TextHandler<'h> {
        Box::new(self)
    }
}

impl<'h, F: FnMut(&mut Element<'_, '_, LocalHandlerTypes>) -> HandlerResult + 'h>
    IntoHandler<ElementHandler<'h>> for F
{
    fn into_handler(self) -> ElementHandler<'h> {
        Box::new(self)
    }
}

impl<'h, F: FnOnce(&mut EndTag<'_>) -> HandlerResult + 'h> IntoHandler<EndTagHandler<'h>> for F {
    fn into_handler(self) -> EndTagHandler<'h> {
        Box::new(self)
    }
}

impl<'h, F: FnOnce(&mut DocumentEnd<'_>) -> HandlerResult + 'h> IntoHandler<EndHandler<'h>> for F {
    fn into_handler(self) -> EndHandler<'h> {
        Box::new(self)
    }
}

impl<'h, F: FnMut(&mut Doctype<'_>) -> HandlerResult + Send + 'h>
    IntoHandler<DoctypeHandlerSend<'h>> for F
{
    fn into_handler(self) -> DoctypeHandlerSend<'h> {
        Box::new(self)
    }
}

impl<'h, F: FnMut(&mut Comment<'_>) -> HandlerResult + Send + 'h>
    IntoHandler<CommentHandlerSend<'h>> for F
{
    fn into_handler(self) -> CommentHandlerSend<'h> {
        Box::new(self)
    }
}

impl<'h, F: FnMut(&mut TextChunk<'_>) -> HandlerResult + Send + 'h> IntoHandler<TextHandlerSend<'h>>
    for F
{
    fn into_handler(self) -> TextHandlerSend<'h> {
        Box::new(self)
    }
}

impl<'h, H: HandlerTypes, F: FnMut(&mut Element<'_, '_, H>) -> HandlerResult + Send + 'h>
    IntoHandler<ElementHandlerSend<'h, H>> for F
{
    fn into_handler(self) -> ElementHandlerSend<'h, H> {
        Box::new(self)
    }
}

impl<'h, F: FnOnce(&mut EndTag<'_>) -> HandlerResult + Send + 'h> IntoHandler<EndTagHandlerSend<'h>>
    for F
{
    fn into_handler(self) -> EndTagHandlerSend<'h> {
        Box::new(self)
    }
}

impl<'h, F: FnOnce(&mut DocumentEnd<'_>) -> HandlerResult + Send + 'h>
    IntoHandler<EndHandlerSend<'h>> for F
{
    fn into_handler(self) -> EndHandlerSend<'h> {
        Box::new(self)
    }
}

/// Specifies element content handlers associated with a selector.
pub struct ElementContentHandlers<'h, H: HandlerTypes = LocalHandlerTypes> {
    /// Element handler. See [`element!`](crate::element) and [`HandlerTypes::ElementHandler`].
    pub element: Option<H::ElementHandler<'h>>,
    /// Comment handler. See [`comments!`](crate::comments) and [`HandlerTypes::CommentHandler`].
    pub comments: Option<H::CommentHandler<'h>>,
    /// Text handler that receives fragments of text nodes. See [`TextChunk`], [`text!`](crate::text), and [`HandlerTypes::TextHandler`].
    pub text: Option<H::TextHandler<'h>>,
}

impl<H: HandlerTypes> Default for ElementContentHandlers<'_, H> {
    fn default() -> Self {
        ElementContentHandlers {
            element: None,
            comments: None,
            text: None,
        }
    }
}

impl<'h, H: HandlerTypes> ElementContentHandlers<'h, H> {
    /// Sets a handler for elements matched by a selector.
    #[inline]
    #[must_use]
    pub fn element(mut self, handler: impl IntoHandler<H::ElementHandler<'h>>) -> Self {
        self.element = Some(handler.into_handler());

        self
    }

    /// Sets a handler for HTML comments in the inner content of elements matched by a selector.
    #[inline]
    #[must_use]
    pub fn comments(mut self, handler: impl IntoHandler<H::CommentHandler<'h>>) -> Self {
        self.comments = Some(handler.into_handler());

        self
    }

    /// Sets a handler for text chunks in the inner content of elements matched by a selector.
    #[inline]
    #[must_use]
    pub fn text(mut self, handler: impl IntoHandler<H::TextHandler<'h>>) -> Self {
        self.text = Some(handler.into_handler());

        self
    }
}

/// Specifies document-level content handlers.
///
/// Some content can't be captured by CSS selectors as it lays outside of content of any
/// of the HTML elements. Document-level handlers allow capture such a content:
///
/// ```html
/// <!doctype html>
/// <!--
///     I can't be captured with a selector, but I can be
///     captured with a document-level comment handler
/// -->
/// <html>
/// <!-- I can be captured with a selector -->
/// </html>
/// ```
pub struct DocumentContentHandlers<'h, H: HandlerTypes = LocalHandlerTypes> {
    /// Doctype handler. See [`doctype!`](crate::doctype) and [`HandlerTypes::DoctypeHandler`].
    pub doctype: Option<H::DoctypeHandler<'h>>,
    /// Comment handler. See [`doc_comments!`](crate::doc_comments) and [`HandlerTypes::CommentHandler`].
    pub comments: Option<H::CommentHandler<'h>>,
    /// Text handler that receives fragments of text nodes. See [`TextChunk`], [`doc_text!`](crate::doc_text), and [`HandlerTypes::TextHandler`].
    pub text: Option<H::TextHandler<'h>>,
    /// End handler. See [`HandlerTypes::EndHandler`].
    pub end: Option<H::EndHandler<'h>>,
}

impl<H: HandlerTypes> Default for DocumentContentHandlers<'_, H> {
    fn default() -> Self {
        DocumentContentHandlers {
            doctype: None,
            comments: None,
            text: None,
            end: None,
        }
    }
}

impl<'h, H: HandlerTypes> DocumentContentHandlers<'h, H> {
    /// Sets a handler for the [document type declaration].
    ///
    /// [document type declaration]: https://developer.mozilla.org/en-US/docs/Glossary/Doctype
    #[inline]
    #[must_use]
    pub fn doctype(mut self, handler: impl IntoHandler<H::DoctypeHandler<'h>>) -> Self {
        self.doctype = Some(handler.into_handler());

        self
    }

    /// Sets a handler for all HTML comments present in the input HTML markup.
    #[inline]
    #[must_use]
    pub fn comments(mut self, handler: impl IntoHandler<H::CommentHandler<'h>>) -> Self {
        self.comments = Some(handler.into_handler());

        self
    }

    /// Sets a handler for all text chunks present in the input HTML markup.
    #[inline]
    #[must_use]
    pub fn text(mut self, handler: impl IntoHandler<H::TextHandler<'h>>) -> Self {
        self.text = Some(handler.into_handler());

        self
    }

    /// Sets a handler for the document end, which is called after the last chunk is processed.
    #[inline]
    #[must_use]
    pub fn end(mut self, handler: impl IntoHandler<H::EndHandler<'h>>) -> Self {
        self.end = Some(handler.into_handler());

        self
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __element_content_handler {
    ($selector:expr, $handler_name:ident, $handler:expr) => {
        (
            ::std::borrow::Cow::Owned($selector.parse::<$crate::Selector>().unwrap()),
            $crate::ElementContentHandlers::default().$handler_name($handler),
        )
    };
}

/// A convenience macro to construct a [rewriting handler](ElementContentHandlers) for elements that can be matched by the
/// specified CSS selector.
///
/// # Example
/// ```
/// use lol_html::{rewrite_str, element, RewriteStrSettings};
/// use lol_html::html_content::ContentType;
///
/// let html = rewrite_str(
///     r#"<span id="foo"></span>"#,
///     RewriteStrSettings {
///         element_content_handlers: vec![
///             element!("#foo", |el| {
///                 el.set_inner_content("Hello!", ContentType::Text);
///
///                 Ok(())
///             })
///         ],
///         ..RewriteStrSettings::new()
///     }
/// ).unwrap();
///
/// assert_eq!(html, r#"<span id="foo">Hello!</span>"#);
/// ```
///
/// When using [sendable handlers](crate::send), beware that the [`Element`] type has a generic argument that controls `Send` compatibility.
/// Use [`send::Element`](crate::send::Element) or write the closure's argument's type as `&mut Element<'_, '_, _>`.
///
/// This macro can create either sendable or non-sendable handlers, but not both in a generic context.
/// `H: HandlerTypes` bound won't work with this macro.
#[macro_export(local_inner_macros)]
macro_rules! element {
    ($selector:expr, $handler:expr) => {{
        // Without this rust won't be able to always infer the type of the handler.
        #[inline(always)]
        const fn type_hint<'h, T, H: $crate::HandlerTypes>(h: T) -> T
        where
            T: FnMut(&mut $crate::html_content::Element<'_, '_, H>) -> $crate::HandlerResult + 'h,
        {
            h
        }

        __element_content_handler!($selector, element, type_hint($handler))
    }};
}

/// Handler for [`element.on_end_tag()`](Element::on_end_tag).
#[macro_export(local_inner_macros)]
macro_rules! end_tag {
    ($handler:expr) => {{
        // Without this rust won't be able to always infer the type of the handler.
        #[inline(always)]
        const fn type_hint<'h, T>(h: T) -> T
        where
            T: FnMut(&mut $crate::html_content::EndTag<'_>) -> $crate::HandlerResult + 'h,
        {
            h
        }

        Box::new(type_hint($handler)) as _
    }};
}

/// A convenience macro to construct a [rewriting handler](ElementContentHandlers) for fragments of text in the inner content of an
/// element that can be matched by the specified CSS selector. Beware: this is tricky to use.
///
/// The text chunks may split the text nodes into smaller fragments. See [`TextChunk`] for more info.
///
/// # Example
/// ```
/// use lol_html::{rewrite_str, text, RewriteStrSettings};
/// use lol_html::html_content::ContentType;
///
/// let html = rewrite_str(
///     r#"<span>Hello</span>"#,
///     RewriteStrSettings {
///         element_content_handlers: vec![
///             text!("span", |t| {
///                 if t.last_in_text_node() {
///                     t.after(" world", ContentType::Text);
///                 }
///
///                 Ok(())
///             })
///         ],
///         ..RewriteStrSettings::new()
///     }
/// ).unwrap();
///
/// assert_eq!(html, r#"<span>Hello world</span>"#);
/// ```
///
/// This macro can create either [sendable](crate::send) or non-sendable handlers, but not both in a generic context.
/// `H: HandlerTypes` bound won't work with this macro.
#[macro_export(local_inner_macros)]
macro_rules! text {
    ($selector:expr, $handler:expr) => {{
        // Without this rust won't be able to always infer the type of the handler.
        #[inline(always)]
        fn type_hint<T>(h: T) -> T
        where
            T: FnMut(&mut $crate::html_content::TextChunk) -> $crate::HandlerResult,
        {
            h
        }

        __element_content_handler!($selector, text, type_hint($handler))
    }};
}

/// A convenience macro to construct a [rewriting handler](ElementContentHandlers) for HTML comments in the inner content of
/// an element that can be matched by the specified CSS selector.
///
/// # Example
/// ```
/// use lol_html::{rewrite_str, comments, RewriteStrSettings};
/// use lol_html::html_content::ContentType;
///
/// let html = rewrite_str(
///     r#"<span><!-- 42 --></span>"#,
///     RewriteStrSettings {
///         element_content_handlers: vec![
///             comments!("span", |c| {
///                 c.set_text("Hello!").unwrap();
///
///                 Ok(())
///             })
///         ],
///         ..RewriteStrSettings::new()
///     }
/// ).unwrap();
///
/// assert_eq!(html, r#"<span><!--Hello!--></span>"#);
/// ```
///
/// This macro can create either [sendable](crate::send) or non-sendable handlers, but not both in a generic context.
/// `H: HandlerTypes` bound won't work with this macro.
#[macro_export(local_inner_macros)]
macro_rules! comments {
    ($selector:expr, $handler:expr) => {{
        // Without this rust won't be able to always infer the type of the handler.
        #[inline(always)]
        const fn type_hint<T>(h: T) -> T
        where
            T: FnMut(&mut $crate::html_content::Comment<'_>) -> $crate::HandlerResult,
        {
            h
        }

        __element_content_handler!($selector, comments, type_hint($handler))
    }};
}

/// A convenience macro to construct a [`StreamingHandler`](crate::html_content::StreamingHandler) from a closure.
///
/// For use with [`Element::streaming_replace`], etc.
///
/// The closure must be `'static` (can't capture by a temporary reference), and `Send`, even when using [non-sendable](crate::send) rewriter.
///
/// ```rust
/// use lol_html::{element, streaming, RewriteStrSettings};
/// use lol_html::html_content::ContentType;
///
/// RewriteStrSettings {
///     element_content_handlers: vec![
///         element!("div", |element| {
///             element.streaming_replace(streaming!(|sink| {
///                 sink.write_str("…", ContentType::Html);
///                 sink.write_str("…", ContentType::Html);
///                 Ok(())
///             }));
///             Ok(())
///         })
///     ],
///     ..RewriteStrSettings::default()
/// };
/// ```
///
/// Note: if you get "implementation of `FnOnce` is not general enough" error, add explicit argument
/// `sink: &mut StreamingHandlerSink<'_>` to the closure.
#[macro_export(local_inner_macros)]
macro_rules! streaming {
    ($closure:expr) => {{
        use ::std::error::Error;
        use $crate::html_content::StreamingHandlerSink;
        // Without this rust won't be able to always infer the type of the handler.
        #[inline(always)]
        const fn streaming_macro_type_hint<StreamingHandler>(
            handler_closure: StreamingHandler,
        ) -> StreamingHandler
        where
            StreamingHandler:
                FnOnce(&mut StreamingHandlerSink<'_>) -> Result<(), Box<dyn Error + Send + Sync>> + 'static,
        {
            handler_closure
        }

        Box::new(streaming_macro_type_hint($closure))
            as Box<dyn $crate::html_content::StreamingHandler + Send>
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __document_content_handler {
    ($handler_name:ident, $handler:expr) => {
        $crate::DocumentContentHandlers::default().$handler_name($handler)
    };
}

/// A convenience macro to construct a [handler](DocumentContentHandlers) for [document type declarations] in the HTML document.
///
/// # Example
/// ```
/// use lol_html::{rewrite_str, doctype, RewriteStrSettings};
/// use lol_html::html_content::ContentType;
///
/// rewrite_str(
///     r#"<!doctype html>"#,
///     RewriteStrSettings {
///         document_content_handlers: vec![
///             doctype!(|d| {
///                 assert_eq!(d.name().unwrap(), "html");
///
///                 Ok(())
///             })
///         ],
///         ..RewriteStrSettings::new()
///     }
/// ).unwrap();
/// ```
///
/// [document type declarations]: https://developer.mozilla.org/en-US/docs/Glossary/Doctype
#[macro_export(local_inner_macros)]
macro_rules! doctype {
    ($handler:expr) => {{
        // Without this rust won't be able to always infer the type of the handler.
        #[inline(always)]
        const fn type_hint<T>(h: T) -> T
        where
            T: FnMut(&mut $crate::html_content::Doctype<'_>) -> $crate::HandlerResult,
        {
            h
        }

        __document_content_handler!(doctype, type_hint($handler))
    }};
}

/// A convenience macro to construct a [rewriting handler](DocumentContentHandlers) for all text chunks in the HTML document. Beware: this is tricky to use.
///
/// The text chunks may split the text nodes into smaller fragments. See [`TextChunk`] for more info.
///
/// # Example
/// ```
/// use lol_html::{rewrite_str, doc_text, RewriteStrSettings};
/// use lol_html::html_content::ContentType;
///
/// let html = rewrite_str(
///     r#"Hello<span>Hello</span>Hello"#,
///     RewriteStrSettings {
///         document_content_handlers: vec![
///             doc_text!(|t| {
///                 if t.last_in_text_node() {
///                     t.after(" world", ContentType::Text);
///                 }
///
///                 Ok(())
///             })
///         ],
///         ..RewriteStrSettings::new()
///     }
/// ).unwrap();
///
/// assert_eq!(html, r#"Hello world<span>Hello world</span>Hello world"#);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! doc_text {
    ($handler:expr) => {{
        // Without this rust won't be able to always infer the type of the handler.
        #[inline(always)]
        const fn type_hint<T>(h: T) -> T
        where
            T: FnMut(&mut $crate::html_content::TextChunk<'_>) -> $crate::HandlerResult,
        {
            h
        }

        __document_content_handler!(text, type_hint($handler))
    }};
}

/// A convenience macro to construct a [rewriting handler](DocumentContentHandlers) for all HTML comments in the HTML document.
///
/// # Example
/// ```
/// use lol_html::{rewrite_str, doc_comments, RewriteStrSettings};
/// use lol_html::html_content::ContentType;
///
/// let html = rewrite_str(
///     r#"<!-- 42 --><span><!-- 42 --></span><!-- 42 -->"#,
///     RewriteStrSettings {
///         document_content_handlers: vec![
///             doc_comments!(|c| {
///                 c.set_text("Hello!").unwrap();
///
///                 Ok(())
///             })
///         ],
///         ..RewriteStrSettings::new()
///     }
/// ).unwrap();
///
/// assert_eq!(html, r#"<!--Hello!--><span><!--Hello!--></span><!--Hello!-->"#);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! doc_comments {
    ($handler:expr) => {{
        // Without this rust won't be able to always infer the type of the handler.
        #[inline(always)]
        const fn type_hint<T>(h: T) -> T
        where
            T: FnMut(&mut $crate::html_content::Comment<'_>) -> $crate::HandlerResult,
        {
            h
        }

        __document_content_handler!(comments, type_hint($handler))
    }};
}

/// A convenience macro to construct a [rewriting handler](DocumentContentHandlers) for the end of the document.
///
/// This handler will only be called after the rewriter has finished processing the final chunk.
///
/// # Example
/// ```
/// use lol_html::{rewrite_str, element, end, RewriteStrSettings};
/// use lol_html::html_content::ContentType;
///
/// let html = rewrite_str(
///     r#"<span>foo</span>"#,
///     RewriteStrSettings {
///         element_content_handlers: vec![
///             element!("span", |el| {
///                 el.append("bar", ContentType::Text);
///
///                 Ok(())
///             })
///         ],
///         document_content_handlers: vec![
///             end!(|end| {
///                 end.append("<div>baz</div>", ContentType::Html);
///
///                 Ok(())
///             })
///         ],
///         ..RewriteStrSettings::new()
///     }
/// ).unwrap();
///
/// assert_eq!(html, r#"<span>foobar</span><div>baz</div>"#);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! end {
    ($handler:expr) => {{
        // Without this rust won't be able to always infer the type of the handler.
        #[inline(always)]
        const fn type_hint<T>(h: T) -> T
        where
            T: FnOnce(&mut $crate::html_content::DocumentEnd<'_>) -> $crate::HandlerResult,
        {
            h
        }

        __document_content_handler!(end, type_hint($handler))
    }};
}

/// Specifies the memory settings for [`HtmlRewriter`].
///
/// [`HtmlRewriter`]: struct.HtmlRewriter.html
// NOTE: exposed in C API as well, thus repr(C).
#[repr(C)]
pub struct MemorySettings {
    /// Specifies the number of bytes that should be preallocated on [`HtmlRewriter`] instantiation
    /// for the internal parsing buffer.
    ///
    /// In some cases (e.g. when rewriter encounters a start tag represented by two or more input
    /// chunks) the rewriter needs to buffer input content.
    ///
    /// Internal parsing buffer is used in such cases. Reallocations and, thus, performance
    /// degradation can be avoided by preallocating the buffer ahead of time. As a drawback of
    /// this approach, every instance of the rewriter will consume the preallocated amount of
    /// memory.
    ///
    /// It's up to the user to adjust the limit according to their environment limitations.
    ///
    /// ### Default
    ///
    /// `1024` bytes when constructed with `MemorySettings::new()`.
    ///
    /// [`HtmlRewriter`]: struct.HtmlRewriter.html
    pub preallocated_parsing_buffer_size: usize,

    /// Sets a hard limit in bytes on memory consumption of a [`HtmlRewriter`] instance.
    ///
    /// Rewriter's [`write`] and [`end`] methods will error if this limit is exceeded.
    ///
    /// Note, that value doesn't reflect the exact threshold after which the rewriter will bailout.
    /// It is impossible to account for all the memory consumed without a significant performance
    /// penalty. So, instead, we try to provide the best approximation by measuring the memory
    /// consumed by internal buffers that grow depending on the input.
    ///
    /// ### Default
    ///
    /// [`std::usize::MAX`] when constructed with `MemorySettings::new()`.
    ///
    /// [`HtmlRewriter`]: struct.HtmlRewriter.html
    /// [`std::usize::MAX`]: https://doc.rust-lang.org/std/usize/constant.MAX.html
    /// [`write`]: struct.HtmlRewriter.html#method.write
    /// [`end`]: struct.HtmlRewriter.html#method.end
    pub max_allowed_memory_usage: usize,
}

impl Default for MemorySettings {
    #[inline]
    fn default() -> Self {
        Self {
            preallocated_parsing_buffer_size: 1024,
            max_allowed_memory_usage: usize::MAX,
        }
    }
}

impl MemorySettings {
    /// Create a new [`MemorySettings`] with default values.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Specifies settings for [`HtmlRewriter`].
///
/// [`HtmlRewriter`]: struct.HtmlRewriter.html
pub struct Settings<'handlers, 'selectors, H: HandlerTypes = LocalHandlerTypes> {
    /// Specifies CSS selectors and rewriting handlers for elements and their inner content.
    ///
    /// ### Hint
    ///
    /// [`element`], [`comments`] and [`text`] convenience macros can be used to construct a
    /// `(Selector, ElementContentHandlers)` tuple.
    ///
    /// ### Example
    /// ```
    /// use std::borrow::Cow;
    /// use lol_html::{ElementContentHandlers, Settings};
    /// use lol_html::html_content::{Comment, Element};
    ///
    /// let settings = Settings {
    ///     element_content_handlers: vec! [
    ///         (
    ///             Cow::Owned("div[foo]".parse().unwrap()),
    ///             ElementContentHandlers::default().element(|el: &mut Element| {
    ///                 // ...
    ///
    ///                 Ok(())
    ///             })
    ///         ),
    ///         (
    ///             Cow::Owned("body".parse().unwrap()),
    ///             ElementContentHandlers::default().comments(|c: &mut Comment| {
    ///                 // ...
    ///
    ///                 Ok(())
    ///             })
    ///         )
    ///     ],
    ///     ..Settings::new()
    /// };
    /// ```
    ///
    /// [`element`]: macro.element.html
    /// [`comments`]: macro.comments.html
    /// [`text`]: macro.text.html
    pub element_content_handlers: Vec<(
        Cow<'selectors, Selector>,
        ElementContentHandlers<'handlers, H>,
    )>,

    /// Specifies rewriting handlers for the content without associating it to a particular
    /// CSS selector.
    ///
    /// Refer to [`DocumentContentHandlers`] documentation for more information.
    ///
    /// ### Hint
    /// [`doctype`], [`doc_comments`] and [`doc_text`] convenience macros can be used to construct
    /// items of this vector.
    ///
    /// [`DocumentContentHandlers`]: struct.DocumentContentHandlers.html
    /// [`doctype`]: macro.doctype.html
    /// [`doc_comments`]: macro.doc_comments.html
    /// [`doc_text`]: macro.doc_text.html
    pub document_content_handlers: Vec<DocumentContentHandlers<'handlers, H>>,

    /// Specifies the [character encoding] for the input and the output of the rewriter.
    ///
    /// Can be a [label] for any of the web-compatible encodings with an exception for `UTF-16LE`,
    /// `UTF-16BE`, `ISO-2022-JP` and `replacement` (these non-ASCII-compatible encodings
    /// are not supported).
    ///
    /// [character encoding]: https://developer.mozilla.org/en-US/docs/Glossary/character_encoding
    /// [label]: https://encoding.spec.whatwg.org/#names-and-labels
    ///
    /// ### Default
    ///
    /// `"utf-8"` when constructed with `Settings::new()`.
    pub encoding: AsciiCompatibleEncoding,

    /// Specifies the memory settings.
    pub memory_settings: MemorySettings,

    /// If set to `true` the rewriter bails out if it encounters markup that drives the HTML parser
    /// into ambigious state.
    ///
    /// Since the rewriter operates on a token stream and doesn't have access to a full
    /// DOM-tree, there are certain rare cases of non-conforming HTML markup which can't be
    /// guaranteed to be parsed correctly without an ability to backtrace the tree.
    ///
    /// Therefore, due to security considerations, sometimes it's preferable to abort the
    /// rewriting process in case of such uncertainty.
    ///
    /// One of the simplest examples of such markup is the following:
    ///
    /// ```html
    /// ...
    /// <select><xmp><script>"use strict";</script></select>
    /// ...
    /// ```
    ///
    /// The `<xmp>` element is not allowed inside the `<select>` element, so in a browser the start
    /// tag for `<xmp>` will be ignored and following `<script>` element will be parsed and executed.
    ///
    /// On the other hand, the `<select>` element itself can be also ignored depending on the
    /// context in which it was parsed. In this case, the `<xmp>` element will not be ignored
    /// and the `<script>` element along with its content will be parsed as a simple text inside
    /// it.
    ///
    /// So, in this case the parser needs an ability to backtrace the DOM-tree to figure out the
    /// correct parsing context.
    ///
    /// ### Default
    ///
    /// `true` when constructed with `Settings::new()`.
    pub strict: bool,

    /// If enabled the rewriter enables support for [Edge Side Includes] tags, treating them as
    /// [void elements] and allowing them to be replaced with desired content.
    ///
    /// [Edge Side Includes]: https://www.w3.org/TR/esi-lang/
    /// [void elements]: https://developer.mozilla.org/en-US/docs/Glossary/Void_element
    pub enable_esi_tags: bool,

    /// If enabled the rewriter will dynamically change the charset when it encounters a `meta` tag
    /// that specifies the charset.
    ///
    /// The charset can be modified by the `meta` tag with
    ///
    /// ```html
    /// <meta charset="windows-1251">
    /// ```
    ///
    /// or
    ///
    /// ```html
    /// <meta http-equiv="content-type" content="text/html; charset=windows-1251">
    /// ```
    ///
    /// Note that an explicit `charset` in the `Content-type` header should take precedence over
    /// the `meta` tag, so only enable this if the content type does not explicitly specify a
    /// charset.  For details check [this][html5encoding].
    ///
    /// [html5encoding]: https://blog.whatwg.org/the-road-to-html-5-character-encoding
    ///
    /// ### Default
    ///
    /// `false` when constructed with `Settings::new()`.
    pub adjust_charset_on_meta_tag: bool,
}

impl Default for Settings<'_, '_, LocalHandlerTypes> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Settings<'_, '_, LocalHandlerTypes> {
    /// Creates [`Settings`] for non-[`Send`]able [`HtmlRewriter`](crate::HtmlRewriter)s.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::new_for_handler_types()
    }
}

impl Settings<'_, '_, SendHandlerTypes> {
    /// Creates [`Settings`] for [`Send`]able [`HtmlRewriter`](crate::HtmlRewriter)s.
    #[inline]
    #[must_use]
    pub fn new_send() -> Self {
        Self::new_for_handler_types()
    }
}

impl<H: HandlerTypes> Settings<'_, '_, H> {
    /// Creates [`Settings`].
    #[inline]
    #[must_use]
    pub fn new_for_handler_types() -> Self {
        Settings {
            element_content_handlers: vec![],
            document_content_handlers: vec![],
            encoding: AsciiCompatibleEncoding(encoding_rs::UTF_8),
            memory_settings: MemorySettings::default(),
            strict: true,
            enable_esi_tags: false,
            adjust_charset_on_meta_tag: false,
        }
    }
}

impl<'h, 's, H: HandlerTypes> From<RewriteStrSettings<'h, 's, H>> for Settings<'h, 's, H> {
    #[inline]
    fn from(settings: RewriteStrSettings<'h, 's, H>) -> Self {
        Settings {
            element_content_handlers: settings.element_content_handlers,
            document_content_handlers: settings.document_content_handlers,
            strict: settings.strict,
            enable_esi_tags: settings.enable_esi_tags,
            ..Settings::new_for_handler_types()
        }
    }
}

/// Specifies settings for the [`rewrite_str`] function.
///
/// [`rewrite_str`]: fn.rewrite_str.html
pub struct RewriteStrSettings<'handlers, 'selectors, H: HandlerTypes = LocalHandlerTypes> {
    /// Specifies CSS selectors and rewriting handlers for elements and their inner content.
    ///
    /// ### Hint
    ///
    /// [`element`], [`comments`] and [`text`] convenience macros can be used to construct a
    /// `(Selector, ElementContentHandlers)` tuple.
    ///
    /// ### Example
    /// ```
    /// use std::borrow::Cow;
    /// use lol_html::{ElementContentHandlers, RewriteStrSettings};
    /// use lol_html::html_content::{Comment, Element};
    ///
    /// let settings = RewriteStrSettings {
    ///     element_content_handlers: vec! [
    ///         (
    ///             Cow::Owned("div[foo]".parse().unwrap()),
    ///             ElementContentHandlers::default().element(|el: &mut Element| {
    ///                 // ...
    ///
    ///                 Ok(())
    ///             })
    ///         ),
    ///         (
    ///             Cow::Owned("div[foo]".parse().unwrap()),
    ///             ElementContentHandlers::default().comments(|c: &mut Comment| {
    ///                 // ...
    ///
    ///                 Ok(())
    ///             })
    ///         )
    ///     ],
    ///     ..RewriteStrSettings::new()
    /// };
    /// ```
    ///
    /// [`element`]: macro.element.html
    /// [`comments`]: macro.comments.html
    /// [`text`]: macro.text.html
    pub element_content_handlers: Vec<(
        Cow<'selectors, Selector>,
        ElementContentHandlers<'handlers, H>,
    )>,

    /// Specifies rewriting handlers for the content without associating it to a particular
    /// CSS selector.
    ///
    /// Refer to [`DocumentContentHandlers`] documentation for more information.
    ///
    /// ### Hint
    /// [`doctype`], [`doc_comments`] and [`doc_text`] convenience macros can be used to construct
    /// items of this vector.
    ///
    /// [`DocumentContentHandlers`]: struct.DocumentContentHandlers.html
    /// [`doctype`]: macro.doctype.html
    /// [`doc_comments`]: macro.doc_comments.html
    /// [`doc_text`]: macro.doc_text.html
    pub document_content_handlers: Vec<DocumentContentHandlers<'handlers, H>>,

    /// If set to `true` the rewriter bails out if it encounters markup that drives the HTML parser
    /// into ambigious state.
    ///
    /// Since the rewriter operates on a token stream and doesn't have access to a full
    /// DOM-tree, there are certain rare cases of non-conforming HTML markup which can't be
    /// guaranteed to be parsed correctly without an ability to backtrace the tree.
    ///
    /// Therefore, due to security considerations, sometimes it's preferable to abort the
    /// rewriting process in case of such uncertainty.
    ///
    /// One of the simplest examples of such markup is the following:
    ///
    /// ```html
    /// ...
    /// <select><xmp><script>"use strict";</script></select>
    /// ...
    /// ```
    ///
    /// The `<xmp>` element is not allowed inside the `<select>` element, so in a browser the start
    /// tag for `<xmp>` will be ignored and following `<script>` element will be parsed and executed.
    ///
    /// On the other hand, the `<select>` element itself can be also ignored depending on the
    /// context in which it was parsed. In this case, the `<xmp>` element will not be ignored
    /// and the `<script>` element along with its content will be parsed as a simple text inside
    /// it.
    ///
    /// So, in this case the parser needs an ability to backtrace the DOM-tree to figure out the
    /// correct parsing context.
    ///
    /// ### Default
    ///
    /// `true` when constructed with `Settings::new()`.
    pub strict: bool,

    /// If enabled the rewriter enables support for [Edge Side Includes] tags, treating them as
    /// [void elements] and allowing them to be replaced with desired content.
    ///
    /// [Edge Side Includes]: https://www.w3.org/TR/esi-lang/
    /// [void elements]: https://developer.mozilla.org/en-US/docs/Glossary/Void_element
    pub enable_esi_tags: bool,
}

impl Default for RewriteStrSettings<'_, '_, LocalHandlerTypes> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl RewriteStrSettings<'_, '_, LocalHandlerTypes> {
    /// Creates [`Settings`] for non-[`Send`]able [`HtmlRewriter`](crate::HtmlRewriter)s.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self::new_for_handler_types()
    }
}

impl RewriteStrSettings<'_, '_, SendHandlerTypes> {
    /// Creates [`Settings`] for [`Send`]able [`HtmlRewriter`](crate::HtmlRewriter)s.
    #[inline]
    #[must_use]
    pub const fn new_send() -> Self {
        Self::new_for_handler_types()
    }
}

impl<H: HandlerTypes> RewriteStrSettings<'_, '_, H> {
    /// Creates [`RewriteStrSettings`].
    #[inline]
    #[must_use]
    pub const fn new_for_handler_types() -> Self {
        RewriteStrSettings {
            element_content_handlers: vec![],
            document_content_handlers: vec![],
            strict: true,
            enable_esi_tags: true,
        }
    }
}
