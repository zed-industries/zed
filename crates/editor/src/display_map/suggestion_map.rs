use super::fold_map::{FoldEdit, FoldOffset, FoldSnapshot};
use gpui::fonts::HighlightStyle;
use language::{Edit, Rope};
use parking_lot::Mutex;

pub type SuggestionEdit = Edit<SuggestionOffset>;

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct SuggestionOffset(pub usize);

#[derive(Clone)]
pub struct Suggestion {
    position: FoldOffset,
    text: Rope,
    highlight_style: HighlightStyle,
}

pub struct SuggestionMap(Mutex<SuggestionSnapshot>);

impl SuggestionMap {
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
    suggestion: Option<Suggestion>,
}
