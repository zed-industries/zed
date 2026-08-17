use super::StreamingHandlerSink;
use std::error::Error as StdError;
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::sync::Mutex;

type HandlerResult = Result<(), Box<dyn StdError + Send + Sync>>;

/// The type of inserted content.
#[derive(Copy, Clone)]
pub enum ContentType {
    /// HTML content type. The rewriter will insert the content as is.
    Html,
    /// Text content type. The rewriter will HTML-escape the content before insertion:
    ///     - `<` will be replaced with `&lt;`
    ///     - `>` will be replaced with `&gt;`
    ///     - `&` will be replaced with `&amp;`
    Text,
}

pub(crate) struct MutationsInner {
    pub content_before: DynamicString,
    pub replacement: DynamicString,
    pub content_after: DynamicString,
    pub removed: bool,
}

impl MutationsInner {
    #[inline]
    pub fn replace(&mut self, chunk: StringChunk) {
        self.remove();
        self.replacement.clear();
        self.replacement.push_back(chunk);
    }

    #[inline]
    pub fn remove(&mut self) {
        self.removed = true;
    }
}

pub(crate) struct Mutations {
    inner: Option<Box<MutationsInner>>,
}

impl Mutations {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { inner: None }
    }

    #[inline]
    pub fn take(&mut self) -> Option<Box<MutationsInner>> {
        self.inner.take()
    }

    #[inline]
    pub fn if_mutated(&mut self) -> Option<&mut MutationsInner> {
        self.inner.as_deref_mut()
    }

    #[inline]
    pub fn mutate(&mut self) -> &mut MutationsInner {
        #[inline(never)]
        fn alloc_content(inner: &mut Option<Box<MutationsInner>>) -> &mut MutationsInner {
            inner.get_or_insert_with(move || {
                Box::new(MutationsInner {
                    content_before: DynamicString::new(),
                    replacement: DynamicString::new(),
                    content_after: DynamicString::new(),
                    removed: false,
                })
            })
        }

        match &mut self.inner {
            Some(inner) => inner,
            uninit => alloc_content(uninit),
        }
    }

    #[inline]
    pub fn removed(&self) -> bool {
        self.inner.as_ref().is_some_and(|inner| inner.removed)
    }
}

/// Part of [`DynamicString`]
pub(crate) enum StringChunk {
    Buffer(Box<str>, ContentType),
    // The mutex is never actually locked, but makes the struct `Sync` without unsafe.
    Stream(Mutex<Box<dyn StreamingHandler + Send + 'static>>),
}

impl StringChunk {
    pub(crate) fn from_str(content: impl Into<Box<str>>, content_type: ContentType) -> Self {
        Self::Buffer(content.into(), content_type)
    }

    #[inline]
    pub(crate) fn stream(handler: Box<dyn StreamingHandler + Send + 'static>) -> Self {
        Self::Stream(Mutex::new(handler))
    }
}

/// String built from fragments or dynamic callbacks
#[derive(Default)]
pub(crate) struct DynamicString {
    chunks: Vec<StringChunk>,
}

impl DynamicString {
    #[inline]
    pub const fn new() -> Self {
        Self { chunks: vec![] }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.chunks.clear();
    }

    #[inline]
    pub fn push_front(&mut self, chunk: StringChunk) {
        self.chunks.insert(0, chunk);
    }

    #[inline]
    pub fn push_back(&mut self, chunk: StringChunk) {
        self.chunks.push(chunk);
    }

    pub fn encode(self, sink: &mut StreamingHandlerSink<'_>) -> HandlerResult {
        for chunk in self.chunks {
            match chunk {
                StringChunk::Buffer(content, content_type) => {
                    sink.write_str(&content, content_type);
                }
                StringChunk::Stream(handler) => {
                    // The mutex will never be locked or poisoned. This is the cheapest way to unwrap it.
                    let (Ok(h) | Err(h)) = handler
                        .into_inner()
                        .map_err(std::sync::PoisonError::into_inner);
                    h.write_all(sink)?;
                }
            }
        }
        Ok(())
    }
}

/// A callback used to write content asynchronously.
///
/// Use the [`streaming!`] macro to construct it.
#[diagnostic::on_unimplemented(
    note = "use `streaming!` macro to create the handler",
    label = "Must be `FnOnce(&mut StreamingHandlerSink<'_>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> + Send + 'static`"
)]
pub trait StreamingHandler {
    /// This method is called only once, and is expected to write content
    /// by calling the [`sink.write_str()`](StreamingHandlerSink::write_str) one or more times.
    ///
    /// Multiple calls to `sink.write_str()` append more content to the output.
    ///
    /// See [`StreamingHandlerSink`].
    ///
    /// Note: if you get "implementation of `FnOnce` is not general enough" error, add explicit argument
    /// `sink: &mut StreamingHandlerSink<'_>` to the closure.
    fn write_all(self: Box<Self>, sink: &mut StreamingHandlerSink<'_>) -> HandlerResult;
}

impl RefUnwindSafe for StringChunk {}
impl UnwindSafe for StringChunk {}

impl<F> From<F> for Box<dyn StreamingHandler + Send + 'static>
where
    F: FnOnce(&mut StreamingHandlerSink<'_>) -> HandlerResult + Send + 'static,
{
    #[inline]
    fn from(f: F) -> Self {
        Box::new(f)
    }
}

impl<F> StreamingHandler for F
where
    F: FnOnce(&mut StreamingHandlerSink<'_>) -> HandlerResult + Send + 'static,
{
    #[inline]
    fn write_all(self: Box<F>, sink: &mut StreamingHandlerSink<'_>) -> HandlerResult {
        (self)(sink)
    }
}
