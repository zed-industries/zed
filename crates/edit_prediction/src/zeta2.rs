use crate::prediction::EditPredictionResult;
use crate::zeta1::compute_edits_and_cursor_position;
use crate::{
    CurrentEditPrediction, DebugEvent, EditPredictionFinishedDebugEvent, EditPredictionId,
    EditPredictionModelInput, EditPredictionStartedDebugEvent, EditPredictionStore,
};
use anyhow::{Result, anyhow};
use cloud_llm_client::predict_edits_v3::RawCompletionRequest;
use cloud_llm_client::{AcceptEditPredictionBody, EditPredictionRejectReason};
use gpui::{App, Task, prelude::*};
use language::{OffsetRangeExt as _, ToOffset as _, ToPoint};
use release_channel::AppVersion;

use std::env;
use std::{path::Path, sync::Arc, time::Instant};
use zeta_prompt::{CURSOR_MARKER, ZetaFormat, clean_zeta2_model_output, format_zeta_prompt};

pub const MAX_CONTEXT_TOKENS: usize = 350;

pub fn max_editable_tokens(format: ZetaFormat) -> usize {
    match format {
        ZetaFormat::V0112MiddleAtEnd | ZetaFormat::V0113Ordered => 150,
        ZetaFormat::V0114180EditableRegion => 180,
        ZetaFormat::V0120GitMergeMarkers => 180,
        ZetaFormat::V0131GitMergeMarkersPrefix => 180,
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
        ..
    }: EditPredictionModelInput,
    cx: &mut Context<EditPredictionStore>,
) -> Task<Result<Option<EditPredictionResult>>> {
    let buffer_snapshotted_at = Instant::now();
    let raw_config = store.zeta2_raw_config().cloned();

    let Some(excerpt_path) = snapshot
        .file()
        .map(|file| -> Arc<Path> { file.full_path(cx).into() })
    else {
        return Task::ready(Err(anyhow!("No file path for excerpt")));
    };

    let client = store.client.clone();
    let llm_token = store.llm_token.clone();
    let app_version = AppVersion::global(cx);

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
            );

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
                    clean_zeta2_model_output(&choice.text, config.format).to_string()
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

            // Client-side cursor marker processing (applies to both raw and v3 responses)
            let cursor_offset_in_output = output_text.find(CURSOR_MARKER);
            if let Some(offset) = cursor_offset_in_output {
                log::trace!("Stripping out {CURSOR_MARKER} from response at offset {offset}");
                output_text.replace_range(offset..offset + CURSOR_MARKER.len(), "");
            }

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

            let (edits, cursor_position) = compute_edits_and_cursor_position(
                old_text,
                &output_text,
                editable_offset_range.start,
                cursor_offset_in_output,
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
) -> (std::ops::Range<usize>, zeta_prompt::ZetaPromptInput) {
    let cursor_point = cursor_offset.to_point(snapshot);

    let (editable_range, context_range) =
        crate::cursor_excerpt::editable_and_context_ranges_for_cursor_position(
            cursor_point,
            snapshot,
            max_editable_tokens(zeta_format),
            MAX_CONTEXT_TOKENS,
        );

    let related_files = crate::filter_redundant_excerpts(
        related_files,
        excerpt_path.as_ref(),
        context_range.start.row..context_range.end.row,
    );

    let context_start_offset = context_range.start.to_offset(snapshot);
    let context_start_row = context_range.start.row;
    let editable_offset_range = editable_range.to_offset(snapshot);
    let cursor_offset_in_excerpt = cursor_offset - context_start_offset;
    let editable_range_in_excerpt = (editable_offset_range.start - context_start_offset)
        ..(editable_offset_range.end - context_start_offset);

    let prompt_input = zeta_prompt::ZetaPromptInput {
        cursor_path: excerpt_path,
        cursor_excerpt: snapshot
            .text_for_range(context_range)
            .collect::<String>()
            .into(),
        editable_range_in_excerpt,
        cursor_offset_in_excerpt,
        excerpt_start_row: Some(context_start_row),
        events,
        related_files,
    };
    (editable_offset_range, prompt_input)
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
