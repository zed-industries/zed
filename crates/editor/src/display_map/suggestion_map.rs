use std::ops::{Add, AddAssign, Sub};

use crate::{ToOffset, ToPoint};

use super::fold_map::{FoldEdit, FoldOffset, FoldSnapshot};
use gpui::fonts::HighlightStyle;
use language::{Bias, Edit, Patch, Rope};
use parking_lot::Mutex;

pub type SuggestionEdit = Edit<SuggestionOffset>;

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct SuggestionOffset(pub usize);

impl Add for SuggestionOffset {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for SuggestionOffset {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl AddAssign for SuggestionOffset {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

#[derive(Clone)]
pub struct Suggestion {
    offset: FoldOffset,
    text: Rope,
}

pub struct SuggestionMap(Mutex<SuggestionSnapshot>);

impl SuggestionMap {
    pub fn replace<P, T>(
        &mut self,
        position: P,
        text: T,
        fold_snapshot: FoldSnapshot,
        fold_edits: Vec<FoldEdit>,
    ) -> (SuggestionSnapshot, Vec<SuggestionEdit>)
    where
        P: ToPoint,
        T: Into<Rope>,
    {
        let buffer_point = position.to_point(fold_snapshot.buffer_snapshot());
        let fold_point = fold_snapshot.to_fold_point(buffer_point, Bias::Left);
        let fold_offset = fold_point.to_offset(&fold_snapshot);
        let new_suggestion = Suggestion {
            offset: fold_offset,
            text: text.into(),
        };

        let (_, edits) = self.sync(fold_snapshot, fold_edits);
        let mut snapshot = self.0.lock();
        let old = if let Some(suggestion) = snapshot.suggestion.take() {
            SuggestionOffset(suggestion.offset.0)
                ..SuggestionOffset(suggestion.offset.0 + suggestion.text.len())
        } else {
            SuggestionOffset(new_suggestion.offset.0)..SuggestionOffset(new_suggestion.offset.0)
        };
        let new = SuggestionOffset(new_suggestion.offset.0)
            ..SuggestionOffset(new_suggestion.offset.0 + new_suggestion.text.len());
        let patch = Patch::new(edits).compose([SuggestionEdit { old, new }]);
        snapshot.suggestion = Some(new_suggestion);
        (snapshot.clone(), patch.into_inner())
    }

    pub fn sync(
        &self,
        fold_snapshot: FoldSnapshot,
        fold_edits: Vec<FoldEdit>,
    ) -> (SuggestionSnapshot, Vec<SuggestionEdit>) {
        let mut snapshot = self.0.lock();
        let mut suggestion_edits = Vec::new();

        let mut suggestion_old_len = 0;
        let mut suggestion_new_len = 0;
        for fold_edit in fold_edits {
            let start = fold_edit.new.start;
            let end = FoldOffset(start.0 + fold_edit.old_len().0);
            if let Some(suggestion) = snapshot.suggestion.as_mut() {
                if end < suggestion.offset {
                    suggestion.offset.0 += fold_edit.new_len().0;
                    suggestion.offset.0 -= fold_edit.old_len().0;
                } else if start > suggestion.offset {
                    suggestion_old_len = suggestion.text.len();
                    suggestion_new_len = suggestion_old_len;
                } else {
                    suggestion_old_len = suggestion.text.len();
                    snapshot.suggestion.take();
                    suggestion_edits.push(SuggestionEdit {
                        old: SuggestionOffset(fold_edit.old.start.0)
                            ..SuggestionOffset(fold_edit.old.end.0 + suggestion_old_len),
                        new: SuggestionOffset(fold_edit.new.start.0)
                            ..SuggestionOffset(fold_edit.new.end.0),
                    });
                    continue;
                }
            }

            suggestion_edits.push(SuggestionEdit {
                old: SuggestionOffset(fold_edit.old.start.0 + suggestion_old_len)
                    ..SuggestionOffset(fold_edit.old.end.0 + suggestion_old_len),
                new: SuggestionOffset(fold_edit.new.start.0 + suggestion_new_len)
                    ..SuggestionOffset(fold_edit.new.end.0 + suggestion_new_len),
            });
        }
        snapshot.folds_snapshot = fold_snapshot;

        (snapshot.clone(), suggestion_edits)
    }
}

#[derive(Clone)]
pub struct SuggestionSnapshot {
    folds_snapshot: FoldSnapshot,
    suggestion: Option<Suggestion>,
}
