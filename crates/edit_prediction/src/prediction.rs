use std::{ops::Range, sync::Arc};

use cloud_llm_client::{EditPredictionRejectReason, PredictEditsRequestTrigger};
use edit_prediction_types::{PredictedCursorPosition, interpolate_edits};
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
    pub prediction: EditPrediction,
    pub reject_reason: Option<EditPredictionRejectReason>,
    pub e2e_latency: std::time::Duration,
}

impl EditPredictionResult {
    pub async fn new(
        id: EditPredictionId,
        edited_buffer: &Entity<Buffer>,
        edited_buffer_snapshot: &BufferSnapshot,
        edits: Arc<[(Range<Anchor>, Arc<str>)]>,
        cursor_position: Option<PredictedCursorPosition>,
        editable_range: Option<Range<Anchor>>,
        inputs: ZetaPromptInput,
        model_version: Option<String>,
        trigger: PredictEditsRequestTrigger,
        e2e_latency: std::time::Duration,
        cx: &mut AsyncApp,
    ) -> Self {
        let (edits, new_snapshot) = (!edits.is_empty())
            .then(|| {
                edited_buffer.read_with(cx, |buffer, _cx| {
                    let new_snapshot = buffer.snapshot();
                    let edits: Arc<[(Range<Anchor>, Arc<str>)]> =
                        interpolate_edits(&edited_buffer_snapshot, &new_snapshot, &edits)
                            .map(Arc::from)
                            .unwrap_or_default();
                    let snapshot = (!edits.is_empty()).then_some(new_snapshot);
                    (Some(edits), snapshot)
                })
            })
            .unwrap_or_default();
        let snapshot = new_snapshot.unwrap_or_else(|| edited_buffer_snapshot.clone());

        let reject_reason = match edits.as_ref() {
            None => Some(EditPredictionRejectReason::Empty),
            Some(edits) if edits.is_empty() => Some(EditPredictionRejectReason::InterpolatedEmpty),
            Some(_) => None,
        };
        let edits = edits.unwrap_or_default();

        let edit_preview = if !edits.is_empty() {
            edited_buffer
                .read_with(cx, |buffer, cx| buffer.preview_edits(edits.clone(), cx))
                .await
        } else {
            EditPreview::unchanged(edited_buffer_snapshot)
        };

        Self {
            prediction: EditPrediction {
                id,
                edits,
                cursor_position,
                editable_range,
                snapshot,
                edit_preview,
                inputs,
                buffer: edited_buffer.clone(),
                model_version,
                trigger,
            },
            reject_reason,
            e2e_latency,
        }
    }
}

#[derive(Clone)]
pub struct EditPrediction {
    pub id: EditPredictionId,
    pub edits: Arc<[(Range<Anchor>, Arc<str>)]>,
    pub cursor_position: Option<PredictedCursorPosition>,
    pub editable_range: Option<Range<Anchor>>,
    pub snapshot: BufferSnapshot,
    pub edit_preview: EditPreview,
    pub buffer: Entity<Buffer>,
    pub inputs: zeta_prompt::ZetaPromptInput,
    pub model_version: Option<String>,
    pub trigger: PredictEditsRequestTrigger,
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
            cursor_position: None,
            editable_range: None,
            snapshot: cx.read(|cx| buffer.read(cx).snapshot()),
            buffer: buffer.clone(),
            edit_preview,
            model_version: None,
            trigger: PredictEditsRequestTrigger::Other,
            inputs: ZetaPromptInput {
                events: vec![],
                related_files: Some(vec![]),
                active_buffer_diagnostics: vec![],
                cursor_path: Path::new("path.txt").into(),
                cursor_offset_in_excerpt: 0,
                cursor_excerpt: "".into(),
                excerpt_start_row: None,
                excerpt_ranges: Default::default(),
                syntax_ranges: None,
                in_open_source_repo: false,
                can_collect_data: false,
                repo_url: None,
            },
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
