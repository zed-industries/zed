use alloc::vec::Vec;
use core::ops::{Deref, DerefMut, RangeBounds};

use crate::{Span, Statement};

/// A code block is a vector of statements, with maybe a vector of spans.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", serde(transparent))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Block {
    body: Vec<Statement>,
    #[cfg_attr(feature = "serialize", serde(skip))]
    span_info: Vec<Span>,
}

impl Block {
    pub const fn new() -> Self {
        Self {
            body: Vec::new(),
            span_info: Vec::new(),
        }
    }

    pub fn from_vec(body: Vec<Statement>) -> Self {
        let span_info = core::iter::repeat_n(Span::default(), body.len()).collect();
        Self { body, span_info }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            body: Vec::with_capacity(capacity),
            span_info: Vec::with_capacity(capacity),
        }
    }

    #[allow(unused_variables)]
    pub fn push(&mut self, end: Statement, span: Span) {
        self.body.push(end);
        self.span_info.push(span);
    }

    pub fn extend(&mut self, item: Option<(Statement, Span)>) {
        if let Some((end, span)) = item {
            self.push(end, span)
        }
    }

    pub fn extend_block(&mut self, other: Self) {
        self.span_info.extend(other.span_info);
        self.body.extend(other.body);
    }

    pub fn append(&mut self, other: &mut Self) {
        self.span_info.append(&mut other.span_info);
        self.body.append(&mut other.body);
    }

    pub fn cull<R: RangeBounds<usize> + Clone>(&mut self, range: R) {
        self.span_info.drain(range.clone());
        self.body.drain(range);
    }

    pub fn splice<R: RangeBounds<usize> + Clone>(&mut self, range: R, other: Self) {
        self.span_info.splice(range.clone(), other.span_info);
        self.body.splice(range, other.body);
    }

    pub fn span_into_iter(self) -> impl Iterator<Item = (Statement, Span)> {
        let Block { body, span_info } = self;
        body.into_iter().zip(span_info)
    }

    pub fn span_iter(&self) -> impl Iterator<Item = (&Statement, &Span)> {
        let span_iter = self.span_info.iter();
        self.body.iter().zip(span_iter)
    }

    pub fn span_iter_mut(&mut self) -> impl Iterator<Item = (&mut Statement, Option<&mut Span>)> {
        let span_iter = self.span_info.iter_mut().map(Some);
        self.body.iter_mut().zip(span_iter)
    }

    pub const fn is_empty(&self) -> bool {
        self.body.is_empty()
    }

    pub const fn len(&self) -> usize {
        self.body.len()
    }
}

impl Deref for Block {
    type Target = [Statement];
    fn deref(&self) -> &[Statement] {
        &self.body
    }
}

impl DerefMut for Block {
    fn deref_mut(&mut self) -> &mut [Statement] {
        &mut self.body
    }
}

impl<'a> IntoIterator for &'a Block {
    type Item = &'a Statement;
    type IntoIter = core::slice::Iter<'a, Statement>;

    fn into_iter(self) -> core::slice::Iter<'a, Statement> {
        self.iter()
    }
}

#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for Block {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from_vec(Vec::deserialize(deserializer)?))
    }
}

impl From<Vec<Statement>> for Block {
    fn from(body: Vec<Statement>) -> Self {
        Self::from_vec(body)
    }
}
