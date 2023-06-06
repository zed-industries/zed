#![allow(unused)]
// TODO kb

use std::ops::{Add, AddAssign, Range, Sub};

use crate::MultiBufferSnapshot;

use super::{
    suggestion_map::{
        SuggestionBufferRows, SuggestionChunks, SuggestionEdit, SuggestionOffset, SuggestionPoint,
        SuggestionSnapshot,
    },
    TextHighlights,
};
use gpui::fonts::HighlightStyle;
use language::{Chunk, Edit, Point, Rope, TextSummary};
use parking_lot::Mutex;
use project::InlayHint;
use rand::Rng;
use sum_tree::Bias;

pub struct EditorAdditionMap(Mutex<EditorAdditionSnapshot>);

#[derive(Clone)]
pub struct EditorAdditionSnapshot {
    // TODO kb merge these two together
    pub suggestion_snapshot: SuggestionSnapshot,
    pub version: usize,
    hints: Vec<InlayHintToRender>,
}

pub type EditorAdditionEdit = Edit<EditorAdditionOffset>;

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct EditorAdditionOffset(pub usize);

impl Add for EditorAdditionOffset {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for EditorAdditionOffset {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl AddAssign for EditorAdditionOffset {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct EditorAdditionPoint(pub Point);

#[derive(Clone)]
pub struct EditorAdditionBufferRows<'a> {
    suggestion_rows: SuggestionBufferRows<'a>,
}

pub struct EditorAdditionChunks<'a> {
    suggestion_chunks: SuggestionChunks<'a>,
}

#[derive(Clone)]
pub struct InlayHintToRender {
    pub(super) position: EditorAdditionPoint,
    pub(super) text: Rope,
}

impl<'a> Iterator for EditorAdditionChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.suggestion_chunks.next()
    }
}

impl<'a> Iterator for EditorAdditionBufferRows<'a> {
    type Item = Option<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        self.suggestion_rows.next()
    }
}

impl EditorAdditionPoint {
    pub fn new(row: u32, column: u32) -> Self {
        Self(Point::new(row, column))
    }

    pub fn row(self) -> u32 {
        self.0.row
    }

    pub fn column(self) -> u32 {
        self.0.column
    }
}

impl EditorAdditionMap {
    pub fn new(suggestion_snapshot: SuggestionSnapshot) -> (Self, EditorAdditionSnapshot) {
        let snapshot = EditorAdditionSnapshot {
            suggestion_snapshot: suggestion_snapshot.clone(),
            version: 0,
            hints: Vec::new(),
        };
        (Self(Mutex::new(snapshot.clone())), snapshot)
    }

    pub fn sync(
        &self,
        suggestion_snapshot: SuggestionSnapshot,
        suggestion_edits: Vec<SuggestionEdit>,
    ) -> (EditorAdditionSnapshot, Vec<EditorAdditionEdit>) {
        let mut snapshot = self.0.lock();

        if snapshot.suggestion_snapshot.version != suggestion_snapshot.version {
            snapshot.version += 1;
        }

        let mut editor_addition_edits = Vec::new();
        for suggestion_edit in suggestion_edits {
            let old = suggestion_edit.old;
            let new = suggestion_edit.new;
            // TODO kb copied from suggestion_map
            editor_addition_edits.push(EditorAdditionEdit {
                old: EditorAdditionOffset(old.start.0)..EditorAdditionOffset(old.end.0),
                new: EditorAdditionOffset(old.start.0)..EditorAdditionOffset(new.end.0),
            })
        }

        snapshot.suggestion_snapshot = suggestion_snapshot;

        (snapshot.clone(), editor_addition_edits)
    }

    pub fn set_inlay_hints(&self, new_hints: Vec<InlayHintToRender>) {
        self.0.lock().hints = new_hints;
    }
}

impl EditorAdditionSnapshot {
    pub fn buffer_snapshot(&self) -> &MultiBufferSnapshot {
        // TODO kb copied from suggestion_map
        self.suggestion_snapshot.buffer_snapshot()
    }

    pub fn to_point(&self, offset: EditorAdditionOffset) -> EditorAdditionPoint {
        // TODO kb copied from suggestion_map
        self.to_editor_addition_point(
            self.suggestion_snapshot
                .to_point(super::suggestion_map::SuggestionOffset(offset.0)),
        )
    }

    pub fn max_point(&self) -> EditorAdditionPoint {
        // TODO kb copied from suggestion_map
        self.to_editor_addition_point(self.suggestion_snapshot.max_point())
    }

    pub fn to_offset(&self, point: EditorAdditionPoint) -> EditorAdditionOffset {
        // TODO kb copied from suggestion_map
        EditorAdditionOffset(
            self.suggestion_snapshot
                .to_offset(self.to_suggestion_point(point, Bias::Left))
                .0,
        )
    }

    pub fn chars_at(&self, start: EditorAdditionPoint) -> impl '_ + Iterator<Item = char> {
        self.suggestion_snapshot
            .chars_at(self.to_suggestion_point(start, Bias::Left))
    }

    // TODO kb what to do with bias?
    pub fn to_suggestion_point(&self, point: EditorAdditionPoint, _: Bias) -> SuggestionPoint {
        SuggestionPoint(point.0)
    }

    pub fn to_editor_addition_point(&self, point: SuggestionPoint) -> EditorAdditionPoint {
        EditorAdditionPoint(point.0)
    }

    pub fn clip_point(&self, point: EditorAdditionPoint, bias: Bias) -> EditorAdditionPoint {
        // TODO kb copied from suggestion_map
        self.to_editor_addition_point(
            self.suggestion_snapshot
                .clip_point(self.to_suggestion_point(point, bias), bias),
        )
    }

    pub fn text_summary_for_range(&self, range: Range<EditorAdditionPoint>) -> TextSummary {
        // TODO kb copied from suggestion_map
        self.suggestion_snapshot.text_summary_for_range(
            self.to_suggestion_point(range.start, Bias::Left)
                ..self.to_suggestion_point(range.end, Bias::Left),
        )
    }

    pub fn buffer_rows<'a>(&'a self, row: u32) -> EditorAdditionBufferRows<'a> {
        EditorAdditionBufferRows {
            suggestion_rows: self.suggestion_snapshot.buffer_rows(row),
        }
    }

    pub fn line_len(&self, row: u32) -> u32 {
        // TODO kb copied from suggestion_map
        self.suggestion_snapshot.line_len(row)
    }

    pub fn chunks<'a>(
        &'a self,
        range: Range<EditorAdditionOffset>,
        language_aware: bool,
        text_highlights: Option<&'a TextHighlights>,
        suggestion_highlight: Option<HighlightStyle>,
    ) -> EditorAdditionChunks<'a> {
        // TODO kb copied from suggestion_map
        EditorAdditionChunks {
            suggestion_chunks: self.suggestion_snapshot.chunks(
                SuggestionOffset(range.start.0)..SuggestionOffset(range.end.0),
                language_aware,
                text_highlights,
                suggestion_highlight,
            ),
        }
    }

    #[cfg(test)]
    pub fn text(&self) -> String {
        // TODO kb copied from suggestion_map
        self.suggestion_snapshot.text()
    }
}
