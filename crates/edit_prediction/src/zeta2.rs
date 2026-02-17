use crate::cursor_excerpt::{compute_excerpt_ranges, excerpt_ranges_to_byte_offsets};
use crate::prediction::EditPredictionResult;
use crate::zeta1::compute_edits_and_cursor_position;
use crate::{
    CurrentEditPrediction, DebugEvent, EditPredictionFinishedDebugEvent, EditPredictionId,
    EditPredictionModelInput, EditPredictionStartedDebugEvent, EditPredictionStore,
};
use anyhow::Result;
use cloud_llm_client::predict_edits_v3::RawCompletionRequest;
use cloud_llm_client::{AcceptEditPredictionBody, EditPredictionRejectReason};
use edit_prediction_types::{PredictedCursorPosition, PredictedSelection};
use gpui::{App, Task, prelude::*};
use language::{OffsetRangeExt as _, ToOffset as _, ToPoint, text_diff};
use release_channel::AppVersion;
use text::Bias;

use std::env;
use std::ops::Range;
use std::{path::Path, sync::Arc, time::Instant};
use zeta_prompt::{
    CURSOR_MARKER, EditPredictionModelKind, SELECTION_START_MARKER, ZetaFormat,
    clean_zeta2_model_output, format_zeta_prompt, get_prefill,
    prompt_input_contains_special_tokens,
};

pub const MAX_CONTEXT_TOKENS: usize = 350;

pub fn max_editable_tokens(format: ZetaFormat) -> usize {
    match format {
        ZetaFormat::V0112MiddleAtEnd | ZetaFormat::V0113Ordered => 150,
        ZetaFormat::V0114180EditableRegion => 180,
        ZetaFormat::V0120GitMergeMarkers => 180,
        ZetaFormat::V0131GitMergeMarkersPrefix => 180,
        ZetaFormat::V0211Prefill => 180,
        ZetaFormat::V0211SeedCoder => 180,
    }
}

pub fn request_prediction_with_zeta2(
    store: &mut EditPredictionStore,
    EditPredictionModelInput {
        buffer,
        snapshot,
        position,
        related_files,
        events,
        debug_tx,
        trigger,
        project,
        ..
    }: EditPredictionModelInput,
    preferred_model: Option<EditPredictionModelKind>,
    cx: &mut Context<EditPredictionStore>,
) -> Task<Result<Option<EditPredictionResult>>> {
    let buffer_snapshotted_at = Instant::now();
    let raw_config = store.zeta2_raw_config().cloned();

    let excerpt_path: Arc<Path> = snapshot
        .file()
        .map(|file| -> Arc<Path> { file.full_path(cx).into() })
        .unwrap_or_else(|| Arc::from(Path::new("untitled")));

    let client = store.client.clone();
    let llm_token = store.llm_token.clone();
    let app_version = AppVersion::global(cx);

    let is_open_source = snapshot
        .file()
        .map_or(false, |file| store.is_file_open_source(&project, file, cx))
        && events.iter().all(|event| event.in_open_source_repo())
        && related_files.iter().all(|file| file.in_open_source_repo);

    let request_task = cx.background_spawn({
        async move {
            let zeta_version = raw_config
                .as_ref()
                .map(|config| config.format)
                .unwrap_or(ZetaFormat::default());

            let cursor_offset = position.to_offset(&snapshot);
            let (editable_offset_range, prompt_input) = zeta2_prompt_input(
                &snapshot,
                related_files,
                events,
                excerpt_path,
                cursor_offset,
                zeta_version,
                preferred_model,
                is_open_source,
            );

            if prompt_input_contains_special_tokens(&prompt_input, zeta_version) {
                return Ok((None, None));
            }

            if let Some(debug_tx) = &debug_tx {
                let prompt = format_zeta_prompt(&prompt_input, zeta_version);
                debug_tx
                    .unbounded_send(DebugEvent::EditPredictionStarted(
                        EditPredictionStartedDebugEvent {
                            buffer: buffer.downgrade(),
                            prompt: Some(prompt),
                            position,
                        },
                    ))
                    .ok();
            }

            log::trace!("Sending edit prediction request");

            let (request_id, output_text, usage) = if let Some(config) = &raw_config {
                let prompt = format_zeta_prompt(&prompt_input, config.format);
                let prefill = get_prefill(&prompt_input, config.format);
                let prompt = format!("{prompt}{prefill}");
                let request = RawCompletionRequest {
                    model: config.model_id.clone().unwrap_or_default(),
                    prompt,
                    temperature: None,
                    stop: vec![],
                    max_tokens: Some(2048),
                    environment: Some(config.format.to_string().to_lowercase()),
                };

                let (mut response, usage) = EditPredictionStore::send_raw_llm_request(
                    request,
                    client,
                    None,
                    llm_token,
                    app_version,
                )
                .await?;

                let request_id = EditPredictionId(response.id.clone().into());
                let output_text = response.choices.pop().map(|choice| {
                    let response = &choice.text;
                    let output = format!("{prefill}{response}");
                    clean_zeta2_model_output(&output, config.format).to_string()
                });

                (request_id, output_text, usage)
            } else {
                // Use V3 endpoint - server handles model/version selection and suffix stripping
                let (response, usage) = EditPredictionStore::send_v3_request(
                    prompt_input.clone(),
                    client,
                    llm_token,
                    app_version,
                    trigger,
                )
                .await?;

                let request_id = EditPredictionId(response.request_id.into());
                let output_text = if response.output.is_empty() {
                    None
                } else {
                    Some(response.output)
                };
                (request_id, output_text, usage)
            };

            let received_response_at = Instant::now();

            log::trace!("Got edit prediction response");

            let Some(mut output_text) = output_text else {
                return Ok((Some((request_id, None)), usage));
            };

            // Client-side marker processing (applies to both raw and v3 responses)
            let (stripped_text, selection_ranges, cursor_offset_in_output) =
                extract_selections_and_cursor(&output_text);
            output_text = stripped_text;

            if let Some(debug_tx) = &debug_tx {
                debug_tx
                    .unbounded_send(DebugEvent::EditPredictionFinished(
                        EditPredictionFinishedDebugEvent {
                            buffer: buffer.downgrade(),
                            position,
                            model_output: Some(output_text.clone()),
                        },
                    ))
                    .ok();
            }

            let mut old_text = snapshot
                .text_for_range(editable_offset_range.clone())
                .collect::<String>();

            if !output_text.is_empty() && !output_text.ends_with('\n') {
                output_text.push('\n');
            }
            if !old_text.is_empty() && !old_text.ends_with('\n') {
                old_text.push('\n');
            }

            let (edits, cursor_position, tabstop_selections) = compute_edits_cursor_and_selections(
                old_text,
                &output_text,
                editable_offset_range.start,
                cursor_offset_in_output,
                &selection_ranges,
                &snapshot,
            );

            anyhow::Ok((
                Some((
                    request_id,
                    Some((
                        prompt_input,
                        buffer,
                        snapshot.clone(),
                        edits,
                        cursor_position,
                        tabstop_selections,
                        received_response_at,
                    )),
                )),
                usage,
            ))
        }
    });

    cx.spawn(async move |this, cx| {
        let Some((id, prediction)) =
            EditPredictionStore::handle_api_response(&this, request_task.await, cx)?
        else {
            return Ok(None);
        };

        let Some((
            inputs,
            edited_buffer,
            edited_buffer_snapshot,
            edits,
            cursor_position,
            tabstop_selections,
            received_response_at,
        )) = prediction
        else {
            return Ok(Some(EditPredictionResult {
                id,
                prediction: Err(EditPredictionRejectReason::Empty),
            }));
        };

        Ok(Some(
            EditPredictionResult::new(
                id,
                &edited_buffer,
                &edited_buffer_snapshot,
                edits.into(),
                cursor_position,
                tabstop_selections,
                buffer_snapshotted_at,
                received_response_at,
                inputs,
                cx,
            )
            .await,
        ))
    })
}

pub fn zeta2_prompt_input(
    snapshot: &language::BufferSnapshot,
    related_files: Vec<zeta_prompt::RelatedFile>,
    events: Vec<Arc<zeta_prompt::Event>>,
    excerpt_path: Arc<Path>,
    cursor_offset: usize,
    zeta_format: ZetaFormat,
    preferred_model: Option<EditPredictionModelKind>,
    is_open_source: bool,
) -> (std::ops::Range<usize>, zeta_prompt::ZetaPromptInput) {
    let cursor_point = cursor_offset.to_point(snapshot);

    let (full_context, range_points) = compute_excerpt_ranges(cursor_point, snapshot);

    let related_files = crate::filter_redundant_excerpts(
        related_files,
        excerpt_path.as_ref(),
        full_context.start.row..full_context.end.row,
    );

    let full_context_start_offset = full_context.start.to_offset(snapshot);
    let full_context_start_row = full_context.start.row;

    let excerpt_ranges =
        excerpt_ranges_to_byte_offsets(&range_points, full_context_start_offset, snapshot);

    let editable_range = match preferred_model {
        Some(EditPredictionModelKind::Zeta1) => &range_points.editable_350,
        _ => match zeta_format {
            ZetaFormat::V0112MiddleAtEnd | ZetaFormat::V0113Ordered => &range_points.editable_150,
            _ => &range_points.editable_180,
        },
    };

    let editable_offset_range = editable_range.to_offset(snapshot);
    let cursor_offset_in_excerpt = cursor_offset - full_context_start_offset;
    let editable_range_in_excerpt = (editable_offset_range.start - full_context_start_offset)
        ..(editable_offset_range.end - full_context_start_offset);

    let prompt_input = zeta_prompt::ZetaPromptInput {
        cursor_path: excerpt_path,
        cursor_excerpt: snapshot
            .text_for_range(full_context)
            .collect::<String>()
            .into(),
        editable_range_in_excerpt,
        cursor_offset_in_excerpt,
        excerpt_start_row: Some(full_context_start_row),
        events,
        related_files,
        excerpt_ranges: Some(excerpt_ranges),
        preferred_model,
        in_open_source_repo: is_open_source,
    };
    (editable_offset_range, prompt_input)
}

/// Extracts selection markers and cursor position from model output.
///
/// Returns (stripped_text, selection_ranges, cursor_offset) where:
/// - stripped_text: the text with all markers removed
/// - selection_ranges: ranges in the stripped text for each selection (start..end pairs)
/// - cursor_offset: the offset of the first standalone cursor marker (not part of a selection pair)
pub fn extract_selections_and_cursor(
    text_with_markers: &str,
) -> (String, Vec<Range<usize>>, Option<usize>) {
    #[derive(Clone, Copy, PartialEq)]
    enum Kind {
        SelectionStart,
        UserCursor,
    }

    let sel_marker = SELECTION_START_MARKER;
    let cur_marker = CURSOR_MARKER;

    // Collect every marker occurrence in document order.
    let mut markers: Vec<(usize, Kind)> = Vec::new();
    let mut pos = 0;
    while pos < text_with_markers.len() {
        if text_with_markers[pos..].starts_with(sel_marker) {
            markers.push((pos, Kind::SelectionStart));
            pos += sel_marker.len();
        } else if text_with_markers[pos..].starts_with(cur_marker) {
            markers.push((pos, Kind::UserCursor));
            pos += cur_marker.len();
        } else {
            pos += text_with_markers[pos..]
                .chars()
                .next()
                .map_or(1, |c| c.len_utf8());
        }
    }

    // Compute the clean (marker-stripped) offset for each marker.
    let mut clean_offsets = Vec::with_capacity(markers.len());
    let mut removed_bytes = 0usize;
    for &(raw_pos, kind) in &markers {
        clean_offsets.push(raw_pos - removed_bytes);
        removed_bytes += match kind {
            Kind::SelectionStart => sel_marker.len(),
            Kind::UserCursor => cur_marker.len(),
        };
    }

    // Pair markers into selection ranges and find standalone cursor.
    let mut selections = Vec::new();
    let mut cursor_offset = None;
    let mut i = 0;
    while i < markers.len() {
        match markers[i].1 {
            Kind::SelectionStart => {
                if i + 1 < markers.len() && markers[i + 1].1 == Kind::UserCursor {
                    selections.push(clean_offsets[i]..clean_offsets[i + 1]);
                    i += 2;
                } else {
                    // Orphaned selection_start – skip it.
                    i += 1;
                }
            }
            Kind::UserCursor => {
                if i + 1 < markers.len() && markers[i + 1].1 == Kind::SelectionStart {
                    // Backwards pair: UserCursor then SelectionStart.
                    selections.push(clean_offsets[i]..clean_offsets[i + 1]);
                    i += 2;
                } else {
                    // Standalone cursor - this becomes the cursor position
                    // Only use the first standalone cursor
                    if cursor_offset.is_none() {
                        cursor_offset = Some(clean_offsets[i]);
                    }
                    i += 1;
                }
            }
        }
    }

    // Build the stripped text
    let mut stripped = String::with_capacity(text_with_markers.len());
    let mut last_end = 0;
    for &(raw_pos, kind) in &markers {
        stripped.push_str(&text_with_markers[last_end..raw_pos]);
        last_end = raw_pos
            + match kind {
                Kind::SelectionStart => sel_marker.len(),
                Kind::UserCursor => cur_marker.len(),
            };
    }
    stripped.push_str(&text_with_markers[last_end..]);

    (stripped, selections, cursor_offset)
}

/// Computes edits, cursor position, and tabstop selections from a diff.
///
/// This extends `compute_edits_and_cursor_position` to also map selection ranges
/// from the new text to `PredictedSelection`s.
pub fn compute_edits_cursor_and_selections(
    old_text: String,
    new_text: &str,
    offset: usize,
    cursor_offset_in_new_text: Option<usize>,
    selection_ranges_in_new_text: &[Range<usize>],
    snapshot: &language::BufferSnapshot,
) -> (
    Vec<(Range<language::Anchor>, Arc<str>)>,
    Option<PredictedCursorPosition>,
    Vec<PredictedSelection>,
) {
    let (edits, cursor_position) = compute_edits_and_cursor_position(
        old_text.clone(),
        new_text,
        offset,
        cursor_offset_in_new_text,
        snapshot,
    );

    // Map each selection range through the diff
    let tabstop_selections = selection_ranges_in_new_text
        .iter()
        .filter_map(|range| {
            let start = map_offset_in_new_text_to_predicted_position(
                range.start,
                &old_text,
                new_text,
                offset,
                snapshot,
            )?;
            let end = map_offset_in_new_text_to_predicted_position(
                range.end, &old_text, new_text, offset, snapshot,
            )?;
            Some(PredictedSelection::new(start, end))
        })
        .collect();

    (edits, cursor_position, tabstop_selections)
}

/// Maps an offset in the new (post-edit) text to a `PredictedCursorPosition`.
///
/// Uses the diff hunks to determine whether the offset falls inside an insertion
/// (in which case we need anchor + offset) or in unchanged text (anchor only).
fn map_offset_in_new_text_to_predicted_position(
    offset_in_new: usize,
    old_text: &str,
    new_text: &str,
    buffer_offset: usize,
    snapshot: &language::BufferSnapshot,
) -> Option<PredictedCursorPosition> {
    let diffs = text_diff(old_text, new_text);

    // Track cumulative delta: new_offset = old_offset + delta
    let mut delta: isize = 0;

    for (old_range, new_replacement) in &diffs {
        let edit_start_in_new = (old_range.start as isize + delta) as usize;
        let edit_end_in_new = edit_start_in_new + new_replacement.len();

        if offset_in_new < edit_start_in_new {
            // Offset is before this edit, in unchanged text
            let offset_in_old = (offset_in_new as isize - delta) as usize;
            let clamped = (buffer_offset + offset_in_old).min(snapshot.len());
            return Some(PredictedCursorPosition::at_anchor(
                snapshot.anchor_after(clamped),
            ));
        } else if offset_in_new <= edit_end_in_new {
            // Offset is inside this edit's new text
            let offset_within_insertion = offset_in_new - edit_start_in_new;
            let clamped = (buffer_offset + old_range.start).min(snapshot.len());
            return Some(PredictedCursorPosition::new(
                snapshot.anchor_before(clamped),
                offset_within_insertion,
            ));
        }

        delta += new_replacement.len() as isize - old_range.len() as isize;
    }

    // Offset is after all edits, in unchanged text at the end
    let offset_in_old = (offset_in_new as isize - delta) as usize;
    let buffer_target = snapshot.clip_offset(buffer_offset + offset_in_old, Bias::Right);
    Some(PredictedCursorPosition::at_anchor(
        snapshot.anchor_after(buffer_target),
    ))
}

pub(crate) fn edit_prediction_accepted(
    store: &EditPredictionStore,
    current_prediction: CurrentEditPrediction,
    cx: &App,
) {
    let custom_accept_url = env::var("ZED_ACCEPT_PREDICTION_URL").ok();
    if store.zeta2_raw_config().is_some() && custom_accept_url.is_none() {
        return;
    }

    let request_id = current_prediction.prediction.id.to_string();
    let require_auth = custom_accept_url.is_none();
    let client = store.client.clone();
    let llm_token = store.llm_token.clone();
    let app_version = AppVersion::global(cx);

    cx.background_spawn(async move {
        let url = if let Some(accept_edits_url) = custom_accept_url {
            gpui::http_client::Url::parse(&accept_edits_url)?
        } else {
            client
                .http_client()
                .build_zed_llm_url("/predict_edits/accept", &[])?
        };

        let response = EditPredictionStore::send_api_request::<()>(
            move |builder| {
                let req = builder.uri(url.as_ref()).body(
                    serde_json::to_string(&AcceptEditPredictionBody {
                        request_id: request_id.clone(),
                    })?
                    .into(),
                );
                Ok(req?)
            },
            client,
            llm_token,
            app_version,
            require_auth,
        )
        .await;

        response?;
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_selections_and_cursor_basic() {
        let input = "for <|selection_start|>item<|user_cursor|> in collection";
        let (stripped, selections, cursor) = extract_selections_and_cursor(input);

        assert_eq!(stripped, "for item in collection");
        assert_eq!(selections.len(), 1);
        assert_eq!(selections[0], 4..8); // "item"
        assert_eq!(cursor, None);
    }

    #[test]
    fn test_extract_selections_and_cursor_multiple_selections() {
        let input = "for <|selection_start|>item<|user_cursor|> in <|selection_start|>collection<|user_cursor|> {";
        let (stripped, selections, cursor) = extract_selections_and_cursor(input);

        assert_eq!(stripped, "for item in collection {");
        assert_eq!(selections.len(), 2);
        assert_eq!(selections[0], 4..8); // "item"
        assert_eq!(selections[1], 12..22); // "collection"
        assert_eq!(cursor, None);
    }

    #[test]
    fn test_extract_selections_and_cursor_with_standalone_cursor() {
        let input =
            "for <|selection_start|>item<|user_cursor|> in collection {\n    <|user_cursor|>\n}";
        let (stripped, selections, cursor) = extract_selections_and_cursor(input);

        assert_eq!(stripped, "for item in collection {\n    \n}");
        assert_eq!(selections.len(), 1);
        assert_eq!(selections[0], 4..8); // "item"
        assert_eq!(cursor, Some(29)); // position after "    "
    }

    #[test]
    fn test_extract_selections_and_cursor_only_cursor() {
        let input = "hello <|user_cursor|>world";
        let (stripped, selections, cursor) = extract_selections_and_cursor(input);

        assert_eq!(stripped, "hello world");
        assert_eq!(selections.len(), 0);
        assert_eq!(cursor, Some(6));
    }

    #[test]
    fn test_extract_selections_and_cursor_no_markers() {
        let input = "hello world";
        let (stripped, selections, cursor) = extract_selections_and_cursor(input);

        assert_eq!(stripped, "hello world");
        assert_eq!(selections.len(), 0);
        assert_eq!(cursor, None);
    }

    #[test]
    fn test_extract_selections_and_cursor_empty_selection() {
        let input = "hello <|user_cursor|>world";
        let (stripped, selections, cursor) = extract_selections_and_cursor(input);

        assert_eq!(stripped, "hello world");
        assert_eq!(selections.len(), 0);
        assert_eq!(cursor, Some(6));
    }

    #[test]
    fn test_extract_selections_and_cursor_backwards_pair() {
        let input = "for <|user_cursor|>item<|selection_start|> in collection";
        let (stripped, selections, cursor) = extract_selections_and_cursor(input);

        assert_eq!(stripped, "for item in collection");
        assert_eq!(selections.len(), 1);
        assert_eq!(selections[0], 4..8); // "item" - still a valid forward range
        assert_eq!(cursor, None);
    }

    #[test]
    fn test_extract_selections_and_cursor_orphaned_selection_start() {
        let input = "hello <|selection_start|>world";
        let (stripped, selections, cursor) = extract_selections_and_cursor(input);

        assert_eq!(stripped, "hello world");
        assert_eq!(selections.len(), 0);
        assert_eq!(cursor, None);
    }

    #[test]
    fn test_extract_selections_and_cursor_multiple_standalone_cursors() {
        let input = "hello <|user_cursor|>world <|user_cursor|>foo";
        let (stripped, selections, cursor) = extract_selections_and_cursor(input);

        assert_eq!(stripped, "hello world foo");
        assert_eq!(selections.len(), 0);
        // Only the first standalone cursor is used
        assert_eq!(cursor, Some(6));
    }

    #[test]
    fn test_extract_selections_full_example() {
        let input = "for <|selection_start|>item<|user_cursor|> in <|selection_start|>collection<|user_cursor|> {\n    <|user_cursor|>\n}";
        let (stripped, selections, cursor) = extract_selections_and_cursor(input);

        assert_eq!(stripped, "for item in collection {\n    \n}");
        assert_eq!(selections.len(), 2);
        assert_eq!(selections[0], 4..8); // "item"
        assert_eq!(selections[1], 12..22); // "collection"
        assert_eq!(cursor, Some(29)); // standalone cursor in body
    }
}
