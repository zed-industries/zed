use std::{ops::Range, sync::Arc};

use gpui::{AsyncApp, Entity, SharedString};
use language::{Anchor, Buffer, BufferSnapshot, EditPreview, OffsetRangeExt, TextBufferSnapshot};

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct EditPredictionId(pub SharedString);

impl From<EditPredictionId> for gpui::ElementId {
    fn from(value: EditPredictionId) -> Self {
        gpui::ElementId::Name(value.0)
    }
}

impl std::fmt::Display for EditPredictionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone)]
pub struct EditPrediction {
    pub id: EditPredictionId,
    pub edits: Arc<[(Range<Anchor>, Arc<str>)]>,
    pub snapshot: BufferSnapshot,
    pub edit_preview: EditPreview,
    // We keep a reference to the buffer so that we do not need to reload it from disk when applying the prediction.
    pub buffer: Entity<Buffer>,
}

impl EditPrediction {
    pub async fn new(
        id: EditPredictionId,
        edited_buffer: &Entity<Buffer>,
        edited_buffer_snapshot: &BufferSnapshot,
        edits: Vec<(Range<Anchor>, Arc<str>)>,
        cx: &mut AsyncApp,
    ) -> Option<Self> {
        let (edits, snapshot, edit_preview_task) = edited_buffer
            .read_with(cx, |buffer, cx| {
                let new_snapshot = buffer.snapshot();
                let edits: Arc<[_]> =
                    interpolate_edits(&edited_buffer_snapshot, &new_snapshot, edits.into())?.into();

                Some((edits.clone(), new_snapshot, buffer.preview_edits(edits, cx)))
            })
            .ok()??;

        let edit_preview = edit_preview_task.await;

        Some(EditPrediction {
            id,
            edits,
            snapshot,
            edit_preview,
            buffer: edited_buffer.clone(),
        })
    }

    pub fn interpolate(
        &self,
        new_snapshot: &TextBufferSnapshot,
    ) -> Option<Vec<(Range<Anchor>, Arc<str>)>> {
        interpolate_edits(&self.snapshot, new_snapshot, self.edits.clone())
    }

    pub fn targets_buffer(&self, buffer: &Buffer) -> bool {
        self.snapshot.remote_id() == buffer.remote_id()
    }
}

impl std::fmt::Debug for EditPrediction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EditPrediction")
            .field("id", &self.id)
            .field("edits", &self.edits)
            .finish()
    }
}

pub fn interpolate_edits(
    old_snapshot: &TextBufferSnapshot,
    new_snapshot: &TextBufferSnapshot,
    current_edits: Arc<[(Range<Anchor>, Arc<str>)]>,
) -> Option<Vec<(Range<Anchor>, Arc<str>)>> {
    let mut edits = Vec::new();

    let mut model_edits = current_edits.iter().peekable();
    for user_edit in new_snapshot.edits_since::<usize>(&old_snapshot.version) {
        while let Some((model_old_range, _)) = model_edits.peek() {
            let model_old_range = model_old_range.to_offset(old_snapshot);
            if model_old_range.end < user_edit.old.start {
                let (model_old_range, model_new_text) = model_edits.next().unwrap();
                edits.push((model_old_range.clone(), model_new_text.clone()));
            } else {
                break;
            }
        }

        if let Some((model_old_range, model_new_text)) = model_edits.peek() {
            let model_old_offset_range = model_old_range.to_offset(old_snapshot);
            if user_edit.old == model_old_offset_range {
                let user_new_text = new_snapshot
                    .text_for_range(user_edit.new.clone())
                    .collect::<String>();

                if let Some(model_suffix) = model_new_text.strip_prefix(&user_new_text) {
                    if !model_suffix.is_empty() {
                        let anchor = old_snapshot.anchor_after(user_edit.old.end);
                        edits.push((anchor..anchor, model_suffix.into()));
                    }

                    model_edits.next();
                    continue;
                }
            }
        }

        return None;
    }

    edits.extend(model_edits.cloned());

    if edits.is_empty() { None } else { Some(edits) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{App, Entity, TestAppContext, prelude::*};
    use language::{Buffer, ToOffset as _};

    #[gpui::test]
    async fn test_edit_prediction_basic_interpolation(cx: &mut TestAppContext) {
        let buffer = cx.new(|cx| Buffer::local("Lorem ipsum dolor", cx));
        let edits: Arc<[(Range<Anchor>, Arc<str>)]> = cx.update(|cx| {
            to_prediction_edits([(2..5, "REM".into()), (9..11, "".into())], &buffer, cx).into()
        });

        let edit_preview = cx
            .read(|cx| buffer.read(cx).preview_edits(edits.clone(), cx))
            .await;

        let prediction = EditPrediction {
            id: EditPredictionId("prediction-1".into()),
            edits,
            snapshot: cx.read(|cx| buffer.read(cx).snapshot()),
            buffer: buffer.clone(),
            edit_preview,
        };

        cx.update(|cx| {
            assert_eq!(
                from_prediction_edits(
                    &prediction.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(2..5, "REM".into()), (9..11, "".into())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(2..5, "")], None, cx));
            assert_eq!(
                from_prediction_edits(
                    &prediction.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(2..2, "REM".into()), (6..8, "".into())]
            );

            buffer.update(cx, |buffer, cx| buffer.undo(cx));
            assert_eq!(
                from_prediction_edits(
                    &prediction.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(2..5, "REM".into()), (9..11, "".into())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(2..5, "R")], None, cx));
            assert_eq!(
                from_prediction_edits(
                    &prediction.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(3..3, "EM".into()), (7..9, "".into())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(3..3, "E")], None, cx));
            assert_eq!(
                from_prediction_edits(
                    &prediction.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(4..4, "M".into()), (8..10, "".into())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(4..4, "M")], None, cx));
            assert_eq!(
                from_prediction_edits(
                    &prediction.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(9..11, "".into())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(4..5, "")], None, cx));
            assert_eq!(
                from_prediction_edits(
                    &prediction.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(4..4, "M".into()), (8..10, "".into())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(8..10, "")], None, cx));
            assert_eq!(
                from_prediction_edits(
                    &prediction.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(4..4, "M".into())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(4..6, "")], None, cx));
            assert_eq!(prediction.interpolate(&buffer.read(cx).snapshot()), None);
        })
    }

    fn to_prediction_edits(
        iterator: impl IntoIterator<Item = (Range<usize>, Arc<str>)>,
        buffer: &Entity<Buffer>,
        cx: &App,
    ) -> Vec<(Range<Anchor>, Arc<str>)> {
        let buffer = buffer.read(cx);
        iterator
            .into_iter()
            .map(|(range, text)| {
                (
                    buffer.anchor_after(range.start)..buffer.anchor_before(range.end),
                    text,
                )
            })
            .collect()
    }

    fn from_prediction_edits(
        editor_edits: &[(Range<Anchor>, Arc<str>)],
        buffer: &Entity<Buffer>,
        cx: &App,
    ) -> Vec<(Range<usize>, Arc<str>)> {
        let buffer = buffer.read(cx);
        editor_edits
            .iter()
            .map(|(range, text)| {
                (
                    range.start.to_offset(buffer)..range.end.to_offset(buffer),
                    text.clone(),
                )
            })
            .collect()
    }
}
