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
pub struct Suggestion<T> {
    position: T,
    text: Rope,
}

pub struct SuggestionMap(Mutex<SuggestionSnapshot>);

impl SuggestionMap {
    pub fn replace<T>(
        &mut self,
        new_suggestion: Option<Suggestion<T>>,
        fold_snapshot: FoldSnapshot,
        fold_edits: Vec<FoldEdit>,
    ) -> (SuggestionSnapshot, Vec<SuggestionEdit>)
    where
        T: ToPoint,
    {
        let new_suggestion = new_suggestion.map(|new_suggestion| {
            let buffer_point = new_suggestion
                .position
                .to_point(fold_snapshot.buffer_snapshot());
            let fold_point = fold_snapshot.to_fold_point(buffer_point, Bias::Left);
            let fold_offset = fold_point.to_offset(&fold_snapshot);
            Suggestion {
                position: fold_offset,
                text: new_suggestion.text,
            }
        });

        let (_, edits) = self.sync(fold_snapshot, fold_edits);
        let mut snapshot = self.0.lock();

        let old = if let Some(suggestion) = snapshot.suggestion.take() {
            SuggestionOffset(suggestion.position.0)
                ..SuggestionOffset(suggestion.position.0 + suggestion.text.len())
        } else if let Some(new_suggestion) = new_suggestion.as_ref() {
            SuggestionOffset(new_suggestion.position.0)..SuggestionOffset(new_suggestion.position.0)
        } else {
            return (snapshot.clone(), edits);
        };

        let new = if let Some(suggestion) = new_suggestion.as_ref() {
            SuggestionOffset(suggestion.position.0)
                ..SuggestionOffset(suggestion.position.0 + suggestion.text.len())
        } else {
            old.start..old.start
        };

        let patch = Patch::new(edits).compose([SuggestionEdit { old, new }]);
        snapshot.suggestion = new_suggestion;
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
                if end < suggestion.position {
                    suggestion.position.0 += fold_edit.new_len().0;
                    suggestion.position.0 -= fold_edit.old_len().0;
                } else if start > suggestion.position {
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
    suggestion: Option<Suggestion<FoldOffset>>,
}
