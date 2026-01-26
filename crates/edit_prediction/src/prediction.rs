use std::{
    ops::Range,
    sync::Arc,
    time::{Duration, Instant},
};

use cloud_llm_client::EditPredictionRejectReason;
use edit_prediction_types::interpolate_edits;
use gpui::{AsyncApp, Entity, SharedString};
use language::{Anchor, Buffer, BufferSnapshot, EditPreview, TextBufferSnapshot};
use zeta_prompt::ZetaPromptInput;

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

/// A prediction response that was returned from the provider, whether it was ultimately valid or not.
pub struct EditPredictionResult {
    pub id: EditPredictionId,
    pub prediction: Result<EditPrediction, EditPredictionRejectReason>,
}

impl EditPredictionResult {
    pub async fn new(
        id: EditPredictionId,
        edited_buffer: &Entity<Buffer>,
        edited_buffer_snapshot: &BufferSnapshot,
        edits: Arc<[(Range<Anchor>, Arc<str>)]>,
        buffer_snapshotted_at: Instant,
        response_received_at: Instant,
        inputs: ZetaPromptInput,
        cx: &mut AsyncApp,
    ) -> Self {
        if edits.is_empty() {
            return Self {
                id,
                prediction: Err(EditPredictionRejectReason::Empty),
            };
        }

        let Some((edits, snapshot, edit_preview_task)) =
            edited_buffer.read_with(cx, |buffer, cx| {
                let new_snapshot = buffer.snapshot();
                let edits: Arc<[_]> =
                    interpolate_edits(&edited_buffer_snapshot, &new_snapshot, &edits)?.into();

                Some((edits.clone(), new_snapshot, buffer.preview_edits(edits, cx)))
            })
        else {
            return Self {
                id,
                prediction: Err(EditPredictionRejectReason::InterpolatedEmpty),
            };
        };

        let edit_preview = edit_preview_task.await;

        Self {
            id: id.clone(),
            prediction: Ok(EditPrediction {
                id,
                edits,
                snapshot,
                edit_preview,
                inputs,
                buffer: edited_buffer.clone(),
                buffer_snapshotted_at,
                response_received_at,
            }),
        }
    }
}

#[derive(Clone)]
pub struct EditPrediction {
    pub id: EditPredictionId,
    pub edits: Arc<[(Range<Anchor>, Arc<str>)]>,
    pub snapshot: BufferSnapshot,
    pub edit_preview: EditPreview,
    pub buffer: Entity<Buffer>,
    pub buffer_snapshotted_at: Instant,
    pub response_received_at: Instant,
    pub inputs: zeta_prompt::ZetaPromptInput,
}

impl EditPrediction {
    pub fn interpolate(
        &self,
        new_snapshot: &TextBufferSnapshot,
    ) -> Option<Vec<(Range<Anchor>, Arc<str>)>> {
        interpolate_edits(&self.snapshot, new_snapshot, &self.edits)
    }

    pub fn targets_buffer(&self, buffer: &Buffer) -> bool {
        self.snapshot.remote_id() == buffer.remote_id()
    }

    pub fn latency(&self) -> Duration {
        self.response_received_at - self.buffer_snapshotted_at
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use gpui::{App, Entity, TestAppContext, prelude::*};
    use language::{Buffer, ToOffset as _};
    use zeta_prompt::ZetaPromptInput;

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
            inputs: ZetaPromptInput {
                events: vec![],
                related_files: vec![],
                cursor_path: Path::new("path.txt").into(),
                cursor_offset_in_excerpt: 0,
                cursor_excerpt: "".into(),
                editable_range_in_excerpt: 0..0,
            },
            buffer_snapshotted_at: Instant::now(),
            response_received_at: Instant::now(),
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
