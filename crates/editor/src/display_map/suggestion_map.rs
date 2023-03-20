use super::{
    fold_map::{FoldChunks, FoldEdit, FoldOffset, FoldSnapshot},
    TextHighlights,
};
use crate::ToPoint;
use gpui::fonts::HighlightStyle;
use language::{Bias, Chunk, Edit, Patch, Point, Rope};
use parking_lot::Mutex;
use std::{
    cmp,
    ops::{Add, AddAssign, Range, Sub},
};

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

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct SuggestionPoint(pub Point);

#[derive(Clone)]
pub struct Suggestion<T> {
    position: T,
    text: Rope,
    highlight_style: HighlightStyle,
}

pub struct SuggestionMap(Mutex<SuggestionSnapshot>);

impl SuggestionMap {
    pub fn new(fold_snapshot: FoldSnapshot) -> (Self, SuggestionSnapshot) {
        let snapshot = SuggestionSnapshot {
            fold_snapshot,
            suggestion: None,
        };
        (Self(Mutex::new(snapshot.clone())), snapshot)
    }

    pub fn replace<T>(
        &self,
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
                highlight_style: new_suggestion.highlight_style,
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
        snapshot.fold_snapshot = fold_snapshot;

        (snapshot.clone(), suggestion_edits)
    }
}

#[derive(Clone)]
pub struct SuggestionSnapshot {
    fold_snapshot: FoldSnapshot,
    suggestion: Option<Suggestion<FoldOffset>>,
}

impl SuggestionSnapshot {
    pub fn max_point(&self) -> SuggestionPoint {
        if let Some(suggestion) = self.suggestion.as_ref() {
            let suggestion_point = suggestion.position.to_point(&self.fold_snapshot);
            let mut max_point = suggestion_point.0;
            max_point += suggestion.text.max_point();
            max_point += self.fold_snapshot.max_point().0 - suggestion_point.0;
            SuggestionPoint(max_point)
        } else {
            SuggestionPoint(self.fold_snapshot.max_point().0)
        }
    }

    pub fn len(&self) -> SuggestionOffset {
        if let Some(suggestion) = self.suggestion.as_ref() {
            let mut len = suggestion.position.0;
            len += suggestion.text.len();
            len += self.fold_snapshot.len().0 - suggestion.position.0;
            SuggestionOffset(len)
        } else {
            SuggestionOffset(self.fold_snapshot.len().0)
        }
    }

    pub fn chunks<'a>(
        &'a self,
        range: Range<SuggestionOffset>,
        language_aware: bool,
        text_highlights: Option<&'a TextHighlights>,
    ) -> Chunks<'a> {
        if let Some(suggestion) = self.suggestion.as_ref() {
            let suggestion_range =
                suggestion.position.0..suggestion.position.0 + suggestion.text.len();

            let prefix_chunks = if range.start.0 < suggestion_range.start {
                Some(self.fold_snapshot.chunks(
                    FoldOffset(range.start.0)
                        ..cmp::min(FoldOffset(suggestion_range.start), FoldOffset(range.end.0)),
                    language_aware,
                    text_highlights,
                ))
            } else {
                None
            };

            let clipped_suggestion_range = cmp::max(range.start.0, suggestion_range.start)
                ..cmp::min(range.end.0, suggestion_range.end);
            let suggestion_chunks = if clipped_suggestion_range.start < clipped_suggestion_range.end
            {
                let start = clipped_suggestion_range.start - suggestion_range.start;
                let end = clipped_suggestion_range.end - suggestion_range.start;
                Some(suggestion.text.chunks_in_range(start..end))
            } else {
                None
            };

            let suffix_chunks = if range.end.0 > suggestion_range.end {
                let start = cmp::max(suggestion_range.end, range.start.0) - suggestion_range.len();
                let end = range.end.0 - suggestion_range.len();
                Some(self.fold_snapshot.chunks(
                    FoldOffset(start)..FoldOffset(end),
                    language_aware,
                    text_highlights,
                ))
            } else {
                None
            };

            Chunks {
                prefix_chunks,
                suggestion_chunks,
                suffix_chunks,
                highlight_style: suggestion.highlight_style,
            }
        } else {
            Chunks {
                prefix_chunks: Some(self.fold_snapshot.chunks(
                    FoldOffset(range.start.0)..FoldOffset(range.end.0),
                    language_aware,
                    text_highlights,
                )),
                suggestion_chunks: None,
                suffix_chunks: None,
                highlight_style: Default::default(),
            }
        }
    }

    #[cfg(test)]
    pub fn text(&self) -> String {
        self.chunks(Default::default()..self.len(), false, None)
            .map(|chunk| chunk.text)
            .collect()
    }
}

pub struct Chunks<'a> {
    prefix_chunks: Option<FoldChunks<'a>>,
    suggestion_chunks: Option<text::Chunks<'a>>,
    suffix_chunks: Option<FoldChunks<'a>>,
    highlight_style: HighlightStyle,
}

impl<'a> Iterator for Chunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(chunks) = self.prefix_chunks.as_mut() {
            if let Some(chunk) = chunks.next() {
                return Some(chunk);
            } else {
                self.prefix_chunks = None;
            }
        }

        if let Some(chunks) = self.suggestion_chunks.as_mut() {
            if let Some(chunk) = chunks.next() {
                return Some(Chunk {
                    text: chunk,
                    syntax_highlight_id: None,
                    highlight_style: Some(self.highlight_style),
                    diagnostic_severity: None,
                    is_unnecessary: false,
                });
            } else {
                self.suggestion_chunks = None;
            }
        }

        if let Some(chunks) = self.suffix_chunks.as_mut() {
            if let Some(chunk) = chunks.next() {
                return Some(chunk);
            } else {
                self.suffix_chunks = None;
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{display_map::fold_map::FoldMap, MultiBuffer};
    use gpui::MutableAppContext;
    use rand::{prelude::StdRng, Rng};
    use settings::Settings;
    use std::env;

    #[gpui::test(iterations = 100)]
    fn test_random_suggestions(cx: &mut MutableAppContext, mut rng: StdRng) {
        cx.set_global(Settings::test(cx));
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let len = rng.gen_range(0..30);
        let buffer = if rng.gen() {
            let text = util::RandomCharIter::new(&mut rng)
                .take(len)
                .collect::<String>();
            MultiBuffer::build_simple(&text, cx)
        } else {
            MultiBuffer::build_random(&mut rng, cx)
        };
        let mut buffer_snapshot = buffer.read(cx).snapshot(cx);
        log::info!("buffer text: {:?}", buffer_snapshot.text());

        let (mut fold_map, mut fold_snapshot) = FoldMap::new(buffer_snapshot.clone());
        let (suggestion_map, mut suggestion_snapshot) = SuggestionMap::new(fold_snapshot.clone());
        let mut suggestion_edits = Vec::new();

        for _ in 0..operations {
            let mut buffer_edits = Vec::new();
            match rng.gen_range(0..=100) {
                0..=29 => {
                    let new_suggestion = if rng.gen_bool(0.3) {
                        None
                    } else {
                        let index = rng.gen_range(0..=buffer_snapshot.len());
                        let len = rng.gen_range(0..30);
                        Some(Suggestion {
                            position: index,
                            text: util::RandomCharIter::new(&mut rng)
                                .take(len)
                                .collect::<String>()
                                .as_str()
                                .into(),
                            highlight_style: Default::default(),
                        })
                    };
                    let (new_suggestion_snapshot, edits) =
                        suggestion_map.replace(new_suggestion, fold_snapshot, Default::default());
                    suggestion_snapshot = new_suggestion_snapshot;
                    suggestion_edits.push(edits);
                }
                30..=59 => {
                    for (new_fold_snapshot, fold_edits) in fold_map.randomly_mutate(&mut rng) {
                        fold_snapshot = new_fold_snapshot;
                        let (new_suggestion_snapshot, edits) =
                            suggestion_map.sync(fold_snapshot.clone(), fold_edits);
                        suggestion_snapshot = new_suggestion_snapshot;
                        suggestion_edits.push(edits);
                    }
                }
                _ => buffer.update(cx, |buffer, cx| {
                    let subscription = buffer.subscribe();
                    let edit_count = rng.gen_range(1..=5);
                    buffer.randomly_mutate(&mut rng, edit_count, cx);
                    buffer_snapshot = buffer.snapshot(cx);
                    let edits = subscription.consume().into_inner();
                    log::info!("editing {:?}", edits);
                    buffer_edits.extend(edits);
                }),
            };

            let (new_fold_snapshot, fold_edits) =
                fold_map.read(buffer_snapshot.clone(), buffer_edits);
            fold_snapshot = new_fold_snapshot;
            let (new_suggestion_snapshot, edits) =
                suggestion_map.sync(fold_snapshot.clone(), fold_edits);
            suggestion_snapshot = new_suggestion_snapshot;
            suggestion_edits.push(edits);

            log::info!("buffer text: {:?}", buffer_snapshot.text());
            log::info!("folds text: {:?}", fold_snapshot.text());
            log::info!("suggestions text: {:?}", suggestion_snapshot.text());
        }
    }
}
