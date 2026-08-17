mod miette;

#[cfg(feature = "miette")]
use ::miette::LabeledSpan;
use std::cmp;
use std::fmt::{self, Display, Formatter};

/// Location and length of a token within a glob expression.
///
/// Spans are encoded as a tuple of `usize`s, where the first element is the location or position
/// and the second element is the length. Both position and length are measured in bytes and
/// **not** code points, graphemes, etc.
///
/// # Examples
///
/// Spans can be used to isolate sub-expressions.
///
/// ```rust
/// use wax::Glob;
///
/// let expression = "**/*.txt";
/// let glob = Glob::new(expression).unwrap();
/// for token in glob.captures() {
///     let (start, n) = token.span();
///     println!("capturing sub-expression: {}", &expression[start..][..n]);
/// }
/// ```
pub type Span = (usize, usize);

pub trait SpanExt {
    fn union(self, other: Self) -> Self;
}

impl SpanExt for Span {
    fn union(self, other: Self) -> Self {
        let start = cmp::min(self.0, other.0);
        let end = cmp::max(self.0 + self.1, other.0 + other.1);
        (start, end - start)
    }
}

pub trait Spanned {
    fn span(&self) -> &Span;

    fn span_mut(&mut self) -> &mut Span;

    fn map_span<F>(mut self, f: F) -> Self
    where
        Self: Sized,
        F: FnOnce(Span) -> Span,
    {
        let span = *self.span();
        *self.span_mut() = f(span);
        self
    }
}

impl Spanned for Span {
    fn span(&self) -> &Span {
        self
    }

    fn span_mut(&mut self) -> &mut Span {
        self
    }
}

/// Error associated with a [`Span`] within a glob expression.
///
/// Located errors describe specific instances of an error within a glob expression. Types that
/// implement this trait provide a location within a glob expression via the [`LocatedError::span`]
/// function as well as a description via the [`Display`] trait. See [`BuildError::locations`].
///
/// [`BuildError::locations`]: crate::BuildError::locations
/// [`Display`]: std::fmt::Display
/// [`LocatedError::span`]: crate::query::LocatedError::span
/// [`Span`]: crate::query::Span
pub trait LocatedError: Display {
    /// Gets the span within the glob expression with which the error is associated.
    fn span(&self) -> Span;
}

#[derive(Clone, Copy, Debug)]
pub struct CompositeSpan {
    label: &'static str,
    kind: CompositeSpanKind,
}

impl CompositeSpan {
    pub fn spanned(label: &'static str, span: Span) -> Self {
        CompositeSpan {
            label,
            kind: CompositeSpanKind::Span(span),
        }
    }

    pub fn correlated(label: &'static str, span: Span, correlated: CorrelatedSpan) -> Self {
        CompositeSpan {
            label,
            kind: CompositeSpanKind::Correlated { span, correlated },
        }
    }

    #[cfg(feature = "miette")]
    pub fn labels(&self) -> Vec<LabeledSpan> {
        let label = Some(self.label.to_string());
        match &self.kind {
            CompositeSpanKind::Span(span) => vec![LabeledSpan::new_with_span(label, *span)],
            CompositeSpanKind::Correlated { span, correlated } => {
                Some(LabeledSpan::new_with_span(label, *span))
                    .into_iter()
                    .chain(correlated.labels())
                    .collect()
            },
        }
    }
}

impl Display for CompositeSpan {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl LocatedError for CompositeSpan {
    fn span(&self) -> Span {
        match &self.kind {
            CompositeSpanKind::Span(span) | CompositeSpanKind::Correlated { span, .. } => *span,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum CompositeSpanKind {
    Span(Span),
    Correlated {
        span: Span,
        #[cfg_attr(not(feature = "miette"), allow(dead_code))]
        correlated: CorrelatedSpan,
    },
}

#[derive(Clone, Copy, Debug)]
pub enum CorrelatedSpan {
    Contiguous(Span),
    Split(Span, Span),
}

impl CorrelatedSpan {
    pub fn split_some(left: Option<Span>, right: Span) -> Self {
        if let Some(left) = left {
            CorrelatedSpan::Split(left, right)
        }
        else {
            CorrelatedSpan::Contiguous(right)
        }
    }

    #[cfg(feature = "miette")]
    pub fn labels(&self) -> Vec<LabeledSpan> {
        let label = Some("here".to_string());
        match self {
            CorrelatedSpan::Contiguous(span) => {
                vec![LabeledSpan::new_with_span(label, *span)]
            },
            CorrelatedSpan::Split(left, right) => vec![
                LabeledSpan::new_with_span(label.clone(), *left),
                LabeledSpan::new_with_span(label, *right),
            ],
        }
    }
}

impl From<Span> for CorrelatedSpan {
    fn from(span: Span) -> Self {
        CorrelatedSpan::Contiguous(span)
    }
}
