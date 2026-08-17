use alloc::{
    borrow::ToOwned,
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::{error::Error, fmt, ops::Range};

use crate::{error::replace_control_chars, Arena, Handle, UniqueArena};

/// A source code span, used for error reporting.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Span {
    start: u32,
    end: u32,
}

impl Span {
    pub const UNDEFINED: Self = Self { start: 0, end: 0 };

    /// Creates a new `Span` from a range of byte indices
    ///
    /// Note: end is exclusive, it doesn't belong to the `Span`
    pub const fn new(start: u32, end: u32) -> Self {
        Span { start, end }
    }

    /// Returns a new `Span` starting at `self` and ending at `other`
    pub const fn until(&self, other: &Self) -> Self {
        Span {
            start: self.start,
            end: other.end,
        }
    }

    /// Modifies `self` to contain the smallest `Span` possible that
    /// contains both `self` and `other`
    pub fn subsume(&mut self, other: Self) {
        *self = if !self.is_defined() {
            // self isn't defined so use other
            other
        } else if !other.is_defined() {
            // other isn't defined so don't try to subsume
            *self
        } else {
            // Both self and other are defined so calculate the span that contains them both
            Span {
                start: self.start.min(other.start),
                end: self.end.max(other.end),
            }
        }
    }

    /// Returns the smallest `Span` possible that contains all the `Span`s
    /// defined in the `from` iterator
    pub fn total_span<T: Iterator<Item = Self>>(from: T) -> Self {
        let mut span: Self = Default::default();
        for other in from {
            span.subsume(other);
        }
        span
    }

    /// Converts `self` to a range if the span is not unknown
    pub fn to_range(self) -> Option<Range<usize>> {
        if self.is_defined() {
            Some(self.start as usize..self.end as usize)
        } else {
            None
        }
    }

    /// Check whether `self` was defined or is a default/unknown span
    pub fn is_defined(&self) -> bool {
        *self != Self::default()
    }

    /// Return a [`SourceLocation`] for this span in the provided source.
    pub fn location(&self, source: &str) -> SourceLocation {
        let prefix = &source[..self.start as usize];
        let line_number = prefix.matches('\n').count() as u32 + 1;
        let line_start = prefix.rfind('\n').map(|pos| pos + 1).unwrap_or(0) as u32;
        let line_position = self.start - line_start + 1;

        SourceLocation {
            line_number,
            line_position,
            offset: self.start,
            length: self.end - self.start,
        }
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Span {
            start: range.start as u32,
            end: range.end as u32,
        }
    }
}

impl core::ops::Index<Span> for str {
    type Output = str;

    #[inline]
    fn index(&self, span: Span) -> &str {
        &self[span.start as usize..span.end as usize]
    }
}

/// A human-readable representation for a span, tailored for text source.
///
/// Roughly corresponds to the positional members of [`GPUCompilationMessage`][gcm] from
/// the WebGPU specification, except
/// - `offset` and `length` are in bytes (UTF-8 code units), instead of UTF-16 code units.
/// - `line_position` is in bytes (UTF-8 code units), instead of UTF-16 code units.
///
/// [gcm]: https://www.w3.org/TR/webgpu/#gpucompilationmessage
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SourceLocation {
    /// 1-based line number.
    pub line_number: u32,
    /// 1-based column in code units (in bytes) of the start of the span.
    pub line_position: u32,
    /// 0-based Offset in code units (in bytes) of the start of the span.
    pub offset: u32,
    /// Length in code units (in bytes) of the span.
    pub length: u32,
}

/// A source code span together with "context", a user-readable description of what part of the error it refers to.
pub type SpanContext = (Span, String);

/// Wrapper class for [`Error`], augmenting it with a list of [`SpanContext`]s.
#[derive(Debug, Clone)]
pub struct WithSpan<E> {
    inner: E,
    spans: Vec<SpanContext>,
}

impl<E> fmt::Display for WithSpan<E>
where
    E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

#[cfg(test)]
impl<E> PartialEq for WithSpan<E>
where
    E: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<E> Error for WithSpan<E>
where
    E: Error,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.inner.source()
    }
}

impl<E> WithSpan<E> {
    /// Create a new [`WithSpan`] from an [`Error`], containing no spans.
    pub const fn new(inner: E) -> Self {
        Self {
            inner,
            spans: Vec::new(),
        }
    }

    /// Reverse of [`Self::new`], discards span information and returns an inner error.
    pub fn into_inner(self) -> E {
        self.inner
    }

    pub const fn as_inner(&self) -> &E {
        &self.inner
    }

    /// Iterator over stored [`SpanContext`]s.
    pub fn spans(&self) -> impl ExactSizeIterator<Item = &SpanContext> {
        self.spans.iter()
    }

    /// Add a new span with description.
    pub fn with_span<S>(mut self, span: Span, description: S) -> Self
    where
        S: ToString,
    {
        if span.is_defined() {
            self.spans.push((span, description.to_string()));
        }
        self
    }

    /// Add a [`SpanContext`].
    pub fn with_context(self, span_context: SpanContext) -> Self {
        let (span, description) = span_context;
        self.with_span(span, description)
    }

    /// Add a [`Handle`] from either [`Arena`] or [`UniqueArena`], borrowing its span information from there
    /// and annotating with a type and the handle representation.
    pub(crate) fn with_handle<T, A: SpanProvider<T>>(self, handle: Handle<T>, arena: &A) -> Self {
        self.with_context(arena.get_span_context(handle))
    }

    /// Convert inner error using [`From`].
    pub fn into_other<E2>(self) -> WithSpan<E2>
    where
        E2: From<E>,
    {
        WithSpan {
            inner: self.inner.into(),
            spans: self.spans,
        }
    }

    /// Convert inner error into another type. Joins span information contained in `self`
    /// with what is returned from `func`.
    pub fn and_then<F, E2>(self, func: F) -> WithSpan<E2>
    where
        F: FnOnce(E) -> WithSpan<E2>,
    {
        let mut res = func(self.inner);
        res.spans.extend(self.spans);
        res
    }

    /// Return a [`SourceLocation`] for our first span, if we have one.
    pub fn location(&self, source: &str) -> Option<SourceLocation> {
        if self.spans.is_empty() || source.is_empty() {
            return None;
        }

        Some(self.spans[0].0.location(source))
    }

    pub(crate) fn diagnostic(&self) -> codespan_reporting::diagnostic::Diagnostic<()>
    where
        E: Error,
    {
        use codespan_reporting::diagnostic::{Diagnostic, Label};
        let diagnostic = Diagnostic::error()
            .with_message(self.inner.to_string())
            .with_labels(
                self.spans()
                    .map(|&(span, ref desc)| {
                        Label::primary((), span.to_range().unwrap()).with_message(desc.to_owned())
                    })
                    .collect(),
            )
            .with_notes({
                let mut notes = Vec::new();
                let mut source: &dyn Error = &self.inner;
                while let Some(next) = Error::source(source) {
                    notes.push(next.to_string());
                    source = next;
                }
                notes
            });
        diagnostic
    }

    /// Emits a summary of the error to standard error stream.
    #[cfg(feature = "stderr")]
    pub fn emit_to_stderr(&self, source: &str)
    where
        E: Error,
    {
        self.emit_to_stderr_with_path(source, "wgsl")
    }

    /// Emits a summary of the error to standard error stream.
    #[cfg(feature = "stderr")]
    pub fn emit_to_stderr_with_path(&self, source: &str, path: &str)
    where
        E: Error,
    {
        use codespan_reporting::{files, term};

        let files = files::SimpleFile::new(path, replace_control_chars(source));
        let config = term::Config::default();

        cfg_if::cfg_if! {
            if #[cfg(feature = "termcolor")] {
                let writer = term::termcolor::StandardStream::stderr(term::termcolor::ColorChoice::Auto);
                term::emit_to_write_style(&mut writer.lock(), &config, &files, &self.diagnostic())
                    .expect("cannot write error");
            } else {
                let writer = std::io::stderr();
                term::emit_to_io_write(&mut writer.lock(), &config, &files, &self.diagnostic())
                    .expect("cannot write error");
            }
        }
    }

    /// Emits a summary of the error to a string.
    pub fn emit_to_string(&self, source: &str) -> String
    where
        E: Error,
    {
        self.emit_to_string_with_path(source, "wgsl")
    }

    /// Emits a summary of the error to a string.
    pub fn emit_to_string_with_path(&self, source: &str, path: &str) -> String
    where
        E: Error,
    {
        use codespan_reporting::{files, term};

        let files = files::SimpleFile::new(path, replace_control_chars(source));
        let config = term::Config::default();

        let mut writer = crate::error::DiagnosticBuffer::new();
        writer
            .emit_to_self(&config, &files, &self.diagnostic())
            .expect("cannot write error");
        writer.into_string()
    }
}

/// Convenience trait for [`Error`] to be able to apply spans to anything.
pub(crate) trait AddSpan: Sized {
    /// The returned output type.
    type Output;

    /// See [`WithSpan::new`].
    fn with_span(self) -> Self::Output;
    /// See [`WithSpan::with_span`].
    fn with_span_static(self, span: Span, description: &'static str) -> Self::Output;
    /// See [`WithSpan::with_context`].
    fn with_span_context(self, span_context: SpanContext) -> Self::Output;
    /// See [`WithSpan::with_handle`].
    fn with_span_handle<T, A: SpanProvider<T>>(self, handle: Handle<T>, arena: &A) -> Self::Output;
}

impl<E> AddSpan for E {
    type Output = WithSpan<Self>;

    fn with_span(self) -> WithSpan<Self> {
        WithSpan::new(self)
    }

    fn with_span_static(self, span: Span, description: &'static str) -> WithSpan<Self> {
        WithSpan::new(self).with_span(span, description)
    }

    fn with_span_context(self, span_context: SpanContext) -> WithSpan<Self> {
        WithSpan::new(self).with_context(span_context)
    }

    fn with_span_handle<T, A: SpanProvider<T>>(
        self,
        handle: Handle<T>,
        arena: &A,
    ) -> WithSpan<Self> {
        WithSpan::new(self).with_handle(handle, arena)
    }
}

/// Trait abstracting over getting a span from an [`Arena`] or a [`UniqueArena`].
pub(crate) trait SpanProvider<T> {
    fn get_span(&self, handle: Handle<T>) -> Span;
    fn get_span_context(&self, handle: Handle<T>) -> SpanContext {
        match self.get_span(handle) {
            x if !x.is_defined() => (Default::default(), "".to_string()),
            known => (
                known,
                format!("{} {:?}", core::any::type_name::<T>(), handle),
            ),
        }
    }
}

impl<T> SpanProvider<T> for Arena<T> {
    fn get_span(&self, handle: Handle<T>) -> Span {
        self.get_span(handle)
    }
}

impl<T> SpanProvider<T> for UniqueArena<T> {
    fn get_span(&self, handle: Handle<T>) -> Span {
        self.get_span(handle)
    }
}

/// Convenience trait for [`Result`], adding a [`MapErrWithSpan::map_err_inner`]
/// mapping to [`WithSpan::and_then`].
pub(crate) trait MapErrWithSpan<E, E2>: Sized {
    /// The returned output type.
    type Output: Sized;

    fn map_err_inner<F, E3>(self, func: F) -> Self::Output
    where
        F: FnOnce(E) -> WithSpan<E3>,
        E2: From<E3>;
}

impl<T, E, E2> MapErrWithSpan<E, E2> for Result<T, WithSpan<E>> {
    type Output = Result<T, WithSpan<E2>>;

    fn map_err_inner<F, E3>(self, func: F) -> Result<T, WithSpan<E2>>
    where
        F: FnOnce(E) -> WithSpan<E3>,
        E2: From<E3>,
    {
        self.map_err(|e| e.and_then(func).into_other::<E2>())
    }
}

#[test]
fn span_location() {
    let source = "12\n45\n\n89\n";
    assert_eq!(
        Span { start: 0, end: 1 }.location(source),
        SourceLocation {
            line_number: 1,
            line_position: 1,
            offset: 0,
            length: 1
        }
    );
    assert_eq!(
        Span { start: 1, end: 2 }.location(source),
        SourceLocation {
            line_number: 1,
            line_position: 2,
            offset: 1,
            length: 1
        }
    );
    assert_eq!(
        Span { start: 2, end: 3 }.location(source),
        SourceLocation {
            line_number: 1,
            line_position: 3,
            offset: 2,
            length: 1
        }
    );
    assert_eq!(
        Span { start: 3, end: 5 }.location(source),
        SourceLocation {
            line_number: 2,
            line_position: 1,
            offset: 3,
            length: 2
        }
    );
    assert_eq!(
        Span { start: 4, end: 6 }.location(source),
        SourceLocation {
            line_number: 2,
            line_position: 2,
            offset: 4,
            length: 2
        }
    );
    assert_eq!(
        Span { start: 5, end: 6 }.location(source),
        SourceLocation {
            line_number: 2,
            line_position: 3,
            offset: 5,
            length: 1
        }
    );
    assert_eq!(
        Span { start: 6, end: 7 }.location(source),
        SourceLocation {
            line_number: 3,
            line_position: 1,
            offset: 6,
            length: 1
        }
    );
    assert_eq!(
        Span { start: 7, end: 8 }.location(source),
        SourceLocation {
            line_number: 4,
            line_position: 1,
            offset: 7,
            length: 1
        }
    );
    assert_eq!(
        Span { start: 8, end: 9 }.location(source),
        SourceLocation {
            line_number: 4,
            line_position: 2,
            offset: 8,
            length: 1
        }
    );
    assert_eq!(
        Span { start: 9, end: 10 }.location(source),
        SourceLocation {
            line_number: 4,
            line_position: 3,
            offset: 9,
            length: 1
        }
    );
    assert_eq!(
        Span { start: 10, end: 11 }.location(source),
        SourceLocation {
            line_number: 5,
            line_position: 1,
            offset: 10,
            length: 1
        }
    );
}
