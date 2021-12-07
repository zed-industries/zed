use crate::buffer::{
    rope::TextDimension, Chunk, Diagnostic, Event, Language, Point, Selection, SelectionSetId,
    Subscription, TextSummary,
};
use anyhow::Result;
use clock::ReplicaId;
use gpui::{Entity, ModelContext};
use std::{cmp::Ordering, fmt::Debug, io, ops::Range, sync::Arc};
use sum_tree::Bias;
use theme::SyntaxTheme;

pub trait Buffer: 'static + Entity<Event = Event> {
    type Snapshot: Snapshot;
    type SelectionSet: DocumentSelectionSet<Document = Self>;

    fn replica_id(&self) -> ReplicaId;
    fn language(&self) -> Option<&Arc<Language>>;
    fn snapshot(&self) -> Self::Snapshot;
    fn subscribe(&mut self) -> Subscription;
    fn start_transaction(&mut self, set_id: Option<SelectionSetId>) -> Result<()>;
    fn end_transaction(
        &mut self,
        set_id: Option<SelectionSetId>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()>;
    fn edit<I, S, T>(&mut self, ranges_iter: I, new_text: T, cx: &mut ModelContext<Self>)
    where
        I: IntoIterator<Item = Range<S>>,
        S: ToOffset<Self::Snapshot>,
        T: Into<String>;
    fn edit_with_autoindent<I, S, T>(
        &mut self,
        ranges_iter: I,
        new_text: T,
        cx: &mut ModelContext<Self>,
    ) where
        I: IntoIterator<Item = Range<S>>,
        S: ToOffset<Self::Snapshot>,
        T: Into<String>;
    fn undo(&mut self, cx: &mut ModelContext<Self>);
    fn redo(&mut self, cx: &mut ModelContext<Self>);
    fn add_selection_set<T: ToOffset<Self::Snapshot>>(
        &mut self,
        selections: &[Selection<T>],
        cx: &mut ModelContext<Self>,
    ) -> SelectionSetId;
    fn update_selection_set<T: ToOffset<Self::Snapshot>>(
        &mut self,
        set_id: SelectionSetId,
        selections: &[Selection<T>],
        cx: &mut ModelContext<Self>,
    ) -> Result<()>;
    fn remove_selection_set(
        &mut self,
        set_id: SelectionSetId,
        cx: &mut ModelContext<Self>,
    ) -> Result<()>;
    fn set_active_selection_set(
        &mut self,
        set_id: Option<SelectionSetId>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()>;
    fn selection_set(&self, set_id: SelectionSetId) -> Option<&Self::SelectionSet>;
    fn selection_sets<'a>(
        &'a self,
    ) -> Box<dyn 'a + Iterator<Item = (&'a SelectionSetId, &'a Self::SelectionSet)>>;
}

pub trait Snapshot: 'static + Clone + Send + Unpin {
    type Anchor: Anchor<Snapshot = Self>;
    type AnchorRangeSet: AnchorRangeSet<Snapshot = Self>;

    fn text(&self) -> String;
    fn text_for_range<'a, T: ToOffset<Self>>(
        &'a self,
        range: Range<T>,
    ) -> Box<dyn 'a + Iterator<Item = &'a str>>;
    fn chunks<'a, T: ToOffset<Self>>(
        &'a self,
        range: Range<T>,
        theme: Option<&'a SyntaxTheme>,
    ) -> Box<dyn 'a + Chunks<'a>>;
    fn chars_at<'a, T: ToOffset<Self>>(
        &'a self,
        position: T,
    ) -> Box<dyn 'a + Iterator<Item = char>>;
    fn chars_for_range<'a, T: ToOffset<Self>>(
        &'a self,
        range: Range<T>,
    ) -> Box<dyn 'a + Iterator<Item = char>>;
    fn reversed_chars_at<'a, T: ToOffset<Self>>(
        &'a self,
        position: T,
    ) -> Box<dyn 'a + Iterator<Item = char>>;
    fn bytes_in_range<'a, T: ToOffset<Self>>(&'a self, range: Range<T>) -> Box<dyn 'a + Bytes<'a>>;
    fn contains_str_at<T: ToOffset<Self>>(&self, position: T, needle: &str) -> bool;
    fn is_line_blank(&self, row: u32) -> bool;
    fn indent_column_for_line(&self, row: u32) -> u32;
    fn range_for_syntax_ancestor<T: ToOffset<Self>>(&self, range: Range<T>)
        -> Option<Range<usize>>;
    fn enclosing_bracket_ranges<T: ToOffset<Self>>(
        &self,
        range: Range<T>,
    ) -> Option<(Range<usize>, Range<usize>)>;
    fn text_summary(&self) -> TextSummary;
    fn text_summary_for_range<'a, D, O>(&'a self, range: Range<O>) -> D
    where
        D: TextDimension,
        O: ToOffset<Self>;
    fn max_point(&self) -> Point;
    fn len(&self) -> usize;
    fn line_len(&self, row: u32) -> u32;
    fn anchor_before<T: ToOffset<Self>>(&self, position: T) -> Self::Anchor;
    fn anchor_at<T: ToOffset<Self>>(&self, position: T, bias: Bias) -> Self::Anchor;
    fn anchor_after<T: ToOffset<Self>>(&self, position: T) -> Self::Anchor;
    fn anchor_range_set<E>(
        &self,
        start_bias: Bias,
        end_bias: Bias,
        entries: E,
    ) -> Self::AnchorRangeSet
    where
        E: IntoIterator<Item = Range<usize>>;
    fn clip_offset(&self, offset: usize, bias: Bias) -> usize;
    fn clip_point(&self, point: Point, bias: Bias) -> Point;
    fn to_offset(&self, point: Point) -> usize;
    fn to_point(&self, offset: usize) -> Point;
    fn parse_count(&self) -> usize;
    fn diagnostics_update_count(&self) -> usize;
    fn diagnostics_in_range<'a, T, O>(
        &'a self,
        search_range: Range<T>,
    ) -> Box<dyn 'a + Iterator<Item = (Range<O>, &Diagnostic)>>
    where
        T: 'a + ToOffset<Self>,
        O: 'a + FromAnchor<Self>;
    fn diagnostic_group<'a, O>(
        &'a self,
        group_id: usize,
    ) -> Box<dyn 'a + Iterator<Item = (Range<O>, &Diagnostic)>>
    where
        O: 'a + FromAnchor<Self>;
}

pub trait ToOffset<S: Snapshot> {
    fn to_offset(&self, snapshot: &S) -> usize;
}

pub trait ToPoint<S: Snapshot> {
    fn to_point(&self, snapshot: &S) -> Point;
}

pub trait Anchor:
    Clone + Debug + Send + Sync + ToOffset<Self::Snapshot> + ToPoint<Self::Snapshot>
{
    type Snapshot: Snapshot;

    fn min() -> Self;
    fn max() -> Self;
    fn cmp(&self, other: &Self, snapshot: &Self::Snapshot) -> Ordering;
    fn summary<'a, D: TextDimension>(&self, snapshot: &'a Self::Snapshot) -> D;
}

pub trait FromAnchor<S: Snapshot> {
    fn from_anchor(anchor: &S::Anchor, content: &S) -> Self;
}

pub trait AnchorRangeSet {
    type Snapshot: Snapshot;

    fn len(&self) -> usize;
    fn version(&self) -> &clock::Global;
    fn ranges<'a, D>(
        &'a self,
        snapshot: &'a Self::Snapshot,
    ) -> Box<dyn 'a + Iterator<Item = Range<Point>>>
    where
        D: TextDimension;
}

pub trait DocumentSelectionSet {
    type Document: Buffer;

    fn len(&self) -> usize;
    fn is_active(&self) -> bool;
    fn intersecting_selections<'a, D, I>(
        &'a self,
        range: Range<(I, Bias)>,
        document: &'a Self::Document,
    ) -> Box<dyn 'a + Iterator<Item = Selection<D>>>
    where
        D: TextDimension,
        I: 'a + ToOffset<<Self::Document as Buffer>::Snapshot>;
    fn selections<'a, D>(
        &'a self,
        document: &'a Self::Document,
    ) -> Box<dyn 'a + Iterator<Item = Selection<D>>>
    where
        D: TextDimension;
    fn oldest_selection<'a, D>(&'a self, document: &'a Self::Document) -> Option<Selection<D>>
    where
        D: TextDimension;
    fn newest_selection<'a, D>(&'a self, document: &'a Self::Document) -> Option<Selection<D>>
    where
        D: TextDimension;
}

pub trait Chunks<'a>: Send + Iterator<Item = Chunk<'a>> {
    fn seek(&mut self, offset: usize);
    fn offset(&self) -> usize;
}

pub trait Bytes<'a>: Send + Iterator<Item = &'a [u8]> + io::Read {}

pub trait AnchorRangeExt<T: Anchor> {
    fn cmp(&self, b: &Range<T>, buffer: &T::Snapshot) -> Ordering;
    fn to_offset(&self, content: &T::Snapshot) -> Range<usize>;
}

impl<S: Snapshot> ToOffset<S> for usize {
    fn to_offset(&self, _: &S) -> usize {
        *self
    }
}

impl<S: Snapshot> ToOffset<S> for Point {
    fn to_offset(&self, snapshot: &S) -> usize {
        snapshot.to_offset(*self)
    }
}

impl<S: Snapshot> ToPoint<S> for Point {
    fn to_point(&self, _: &S) -> Point {
        *self
    }
}

impl<S: Snapshot> ToPoint<S> for usize {
    fn to_point(&self, snapshot: &S) -> Point {
        snapshot.to_point(*self)
    }
}

impl<S: Snapshot> FromAnchor<S> for usize
where
    S::Anchor: ToOffset<S>,
{
    fn from_anchor(anchor: &S::Anchor, snapshot: &S) -> Self {
        anchor.to_offset(snapshot)
    }
}

impl<S: Snapshot> FromAnchor<S> for Point
where
    S::Anchor: ToPoint<S>,
{
    fn from_anchor(anchor: &S::Anchor, snapshot: &S) -> Self {
        anchor.to_point(snapshot)
    }
}

impl<T: Anchor> AnchorRangeExt<T> for Range<T> {
    fn cmp(&self, other: &Range<T>, buffer: &T::Snapshot) -> Ordering {
        match self.start.cmp(&other.start, buffer) {
            Ordering::Equal => other.end.cmp(&self.end, buffer),
            ord @ _ => ord,
        }
    }

    fn to_offset(&self, content: &T::Snapshot) -> Range<usize> {
        self.start.to_offset(&content)..self.end.to_offset(&content)
    }
}
