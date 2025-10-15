use std::{borrow::Cow, ops::Range, path::Path, sync::Arc};

use anyhow::Context as _;
use cloud_llm_client::predict_edits_v3;
use gpui::{App, AsyncApp, Entity};
use language::{
    Anchor, Buffer, BufferSnapshot, EditPreview, OffsetRangeExt, TextBufferSnapshot, text_diff,
};
use project::Project;
use util::ResultExt;
use uuid::Uuid;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct EditPredictionId(Uuid);

impl From<EditPredictionId> for gpui::ElementId {
    fn from(value: EditPredictionId) -> Self {
        gpui::ElementId::Uuid(value.0)
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
    pub path: Arc<Path>,
    pub edits: Arc<[(Range<Anchor>, String)]>,
    pub snapshot: BufferSnapshot,
    pub edit_preview: EditPreview,
    // We keep a reference to the buffer so that we do not need to reload it from disk when applying the prediction.
    pub buffer: Entity<Buffer>,
}

impl EditPrediction {
    pub async fn from_response(
        response: predict_edits_v3::PredictEditsResponse,
        active_buffer_old_snapshot: &TextBufferSnapshot,
        active_buffer: &Entity<Buffer>,
        project: &Entity<Project>,
        cx: &mut AsyncApp,
    ) -> Option<Self> {
        // TODO only allow cloud to return one path
        let Some(path) = response.edits.first().map(|e| e.path.clone()) else {
            return None;
        };

        let is_same_path = active_buffer
            .read_with(cx, |buffer, cx| buffer_path_eq(buffer, &path, cx))
            .ok()?;

        let (buffer, edits, snapshot, edit_preview_task) = if is_same_path {
            active_buffer
                .read_with(cx, |buffer, cx| {
                    let new_snapshot = buffer.snapshot();
                    let edits = edits_from_response(&response.edits, &active_buffer_old_snapshot);
                    let edits: Arc<[_]> =
                        interpolate_edits(active_buffer_old_snapshot, &new_snapshot, edits)?.into();

                    Some((
                        active_buffer.clone(),
                        edits.clone(),
                        new_snapshot,
                        buffer.preview_edits(edits, cx),
                    ))
                })
                .ok()??
        } else {
            let buffer_handle = project
                .update(cx, |project, cx| {
                    let project_path = project
                        .find_project_path(&path, cx)
                        .context("Failed to find project path for zeta edit")?;
                    anyhow::Ok(project.open_buffer(project_path, cx))
                })
                .ok()?
                .log_err()?
                .await
                .context("Failed to open buffer for zeta edit")
                .log_err()?;

            buffer_handle
                .read_with(cx, |buffer, cx| {
                    let snapshot = buffer.snapshot();
                    let edits = edits_from_response(&response.edits, &snapshot);
                    if edits.is_empty() {
                        return None;
                    }
                    Some((
                        buffer_handle.clone(),
                        edits.clone(),
                        snapshot,
                        buffer.preview_edits(edits, cx),
                    ))
                })
                .ok()??
        };

        let edit_preview = edit_preview_task.await;

        Some(EditPrediction {
            id: EditPredictionId(response.request_id),
            path,
            edits,
            snapshot,
            edit_preview,
            buffer,
        })
    }

    pub fn interpolate(
        &self,
        new_snapshot: &TextBufferSnapshot,
    ) -> Option<Vec<(Range<Anchor>, String)>> {
        interpolate_edits(&self.snapshot, new_snapshot, self.edits.clone())
    }

    pub fn targets_buffer(&self, buffer: &Buffer, cx: &App) -> bool {
        buffer_path_eq(buffer, &self.path, cx)
    }
}

impl std::fmt::Debug for EditPrediction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EditPrediction")
            .field("id", &self.id)
            .field("path", &self.path)
            .field("edits", &self.edits)
            .finish()
    }
}

pub fn buffer_path_eq(buffer: &Buffer, path: &Path, cx: &App) -> bool {
    buffer.file().map(|p| p.full_path(cx)).as_deref() == Some(path)
}

pub fn interpolate_edits(
    old_snapshot: &TextBufferSnapshot,
    new_snapshot: &TextBufferSnapshot,
    current_edits: Arc<[(Range<Anchor>, String)]>,
) -> Option<Vec<(Range<Anchor>, String)>> {
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
                        edits.push((anchor..anchor, model_suffix.to_string()));
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

pub fn line_range_to_point_range(range: Range<predict_edits_v3::Line>) -> Range<language::Point> {
    language::Point::new(range.start.0, 0)..language::Point::new(range.end.0, 0)
}

fn edits_from_response(
    edits: &[predict_edits_v3::Edit],
    snapshot: &TextBufferSnapshot,
) -> Arc<[(Range<Anchor>, String)]> {
    edits
        .iter()
        .flat_map(|edit| {
            let point_range = line_range_to_point_range(edit.range.clone());
            let offset = point_range.to_offset(snapshot).start;
            let old_text = snapshot.text_for_range(point_range);

            excerpt_edits_from_response(
                old_text.collect::<Cow<str>>(),
                &edit.content,
                offset,
                &snapshot,
            )
        })
        .collect::<Vec<_>>()
        .into()
}

fn excerpt_edits_from_response(
    old_text: Cow<str>,
    new_text: &str,
    offset: usize,
    snapshot: &TextBufferSnapshot,
) -> impl Iterator<Item = (Range<Anchor>, String)> {
    text_diff(&old_text, new_text)
        .into_iter()
        .map(move |(mut old_range, new_text)| {
            old_range.start += offset;
            old_range.end += offset;

            let prefix_len = common_prefix(
                snapshot.chars_for_range(old_range.clone()),
                new_text.chars(),
            );
            old_range.start += prefix_len;

            let suffix_len = common_prefix(
                snapshot.reversed_chars_for_range(old_range.clone()),
                new_text[prefix_len..].chars().rev(),
            );
            old_range.end = old_range.end.saturating_sub(suffix_len);

            let new_text = new_text[prefix_len..new_text.len() - suffix_len].to_string();
            let range = if old_range.is_empty() {
                let anchor = snapshot.anchor_after(old_range.start);
                anchor..anchor
            } else {
                snapshot.anchor_after(old_range.start)..snapshot.anchor_before(old_range.end)
            };
            (range, new_text)
        })
}

fn common_prefix<T1: Iterator<Item = char>, T2: Iterator<Item = char>>(a: T1, b: T2) -> usize {
    a.zip(b)
        .take_while(|(a, b)| a == b)
        .map(|(a, _)| a.len_utf8())
        .sum()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use cloud_llm_client::predict_edits_v3;
    use edit_prediction_context::Line;
    use gpui::{App, Entity, TestAppContext, prelude::*};
    use indoc::indoc;
    use language::{Buffer, ToOffset as _};

    #[gpui::test]
    async fn test_compute_edits(cx: &mut TestAppContext) {
        let old = indoc! {r#"
            fn main() {
                let args =
                println!("{}", args[1])
            }
        "#};

        let new = indoc! {r#"
            fn main() {
                let args = std::env::args();
                println!("{}", args[1]);
            }
        "#};

        let buffer = cx.new(|cx| Buffer::local(old, cx));
        let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());

        // TODO cover more cases when multi-file is supported
        let big_edits = vec![predict_edits_v3::Edit {
            path: PathBuf::from("test.txt").into(),
            range: Line(0)..Line(old.lines().count() as u32),
            content: new.into(),
        }];

        let edits = edits_from_response(&big_edits, &snapshot);
        assert_eq!(edits.len(), 2);
        assert_eq!(
            edits[0].0.to_point(&snapshot).start,
            language::Point::new(1, 14)
        );
        assert_eq!(edits[0].1, " std::env::args();");
        assert_eq!(
            edits[1].0.to_point(&snapshot).start,
            language::Point::new(2, 27)
        );
        assert_eq!(edits[1].1, ";");
    }

    #[gpui::test]
    async fn test_edit_prediction_basic_interpolation(cx: &mut TestAppContext) {
        let buffer = cx.new(|cx| Buffer::local("Lorem ipsum dolor", cx));
        let edits: Arc<[(Range<Anchor>, String)]> = cx.update(|cx| {
            to_prediction_edits(
                [(2..5, "REM".to_string()), (9..11, "".to_string())],
                &buffer,
                cx,
            )
            .into()
        });

        let edit_preview = cx
            .read(|cx| buffer.read(cx).preview_edits(edits.clone(), cx))
            .await;

        let prediction = EditPrediction {
            id: EditPredictionId(Uuid::new_v4()),
            edits,
            snapshot: cx.read(|cx| buffer.read(cx).snapshot()),
            path: Path::new("test.txt").into(),
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
                vec![(2..5, "REM".to_string()), (9..11, "".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(2..5, "")], None, cx));
            assert_eq!(
                from_prediction_edits(
                    &prediction.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(2..2, "REM".to_string()), (6..8, "".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.undo(cx));
            assert_eq!(
                from_prediction_edits(
                    &prediction.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(2..5, "REM".to_string()), (9..11, "".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(2..5, "R")], None, cx));
            assert_eq!(
                from_prediction_edits(
                    &prediction.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(3..3, "EM".to_string()), (7..9, "".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(3..3, "E")], None, cx));
            assert_eq!(
                from_prediction_edits(
                    &prediction.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(4..4, "M".to_string()), (8..10, "".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(4..4, "M")], None, cx));
            assert_eq!(
                from_prediction_edits(
                    &prediction.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(9..11, "".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(4..5, "")], None, cx));
            assert_eq!(
                from_prediction_edits(
                    &prediction.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(4..4, "M".to_string()), (8..10, "".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(8..10, "")], None, cx));
            assert_eq!(
                from_prediction_edits(
                    &prediction.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(4..4, "M".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(4..6, "")], None, cx));
            assert_eq!(prediction.interpolate(&buffer.read(cx).snapshot()), None);
        })
    }

    fn to_prediction_edits(
        iterator: impl IntoIterator<Item = (Range<usize>, String)>,
        buffer: &Entity<Buffer>,
        cx: &App,
    ) -> Vec<(Range<Anchor>, String)> {
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
        editor_edits: &[(Range<Anchor>, String)],
        buffer: &Entity<Buffer>,
        cx: &App,
    ) -> Vec<(Range<usize>, String)> {
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
