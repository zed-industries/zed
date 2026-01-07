#[cfg(feature = "cli-support")]
use crate::EvalCacheEntryKind;
use crate::open_ai_response::text_from_response;
use crate::prediction::EditPredictionResult;
use crate::{
    CurrentEditPrediction, DebugEvent, EDIT_PREDICTIONS_MODEL_ID, EditPredictionFinishedDebugEvent,
    EditPredictionId, EditPredictionModelInput, EditPredictionStartedDebugEvent,
    EditPredictionStore,
};
use anyhow::{Result, anyhow};
use cloud_llm_client::{AcceptEditPredictionBody, EditPredictionRejectReason};
use gpui::{App, Task, prelude::*};
use language::{OffsetRangeExt as _, ToOffset as _, ToPoint};
use release_channel::AppVersion;

use std::env;
use std::{path::Path, sync::Arc, time::Instant};
use zeta_prompt::CURSOR_MARKER;
use zeta_prompt::format_zeta_prompt;

const MAX_CONTEXT_TOKENS: usize = 150;
const MAX_REWRITE_TOKENS: usize = 350;

pub fn request_prediction_with_zeta2(
    store: &mut EditPredictionStore,
    EditPredictionModelInput {
        buffer,
        snapshot,
        position,
        related_files,
        events,
        debug_tx,
        ..
    }: EditPredictionModelInput,
    cx: &mut Context<EditPredictionStore>,
) -> Task<Result<Option<EditPredictionResult>>> {
    let buffer_snapshotted_at = Instant::now();

    let Some(excerpt_path) = snapshot
        .file()
        .map(|file| -> Arc<Path> { file.full_path(cx).into() })
    else {
        return Task::ready(Err(anyhow!("No file path for excerpt")));
    };

    let client = store.client.clone();
    let llm_token = store.llm_token.clone();
    let app_version = AppVersion::global(cx);

    #[cfg(feature = "cli-support")]
    let eval_cache = store.eval_cache.clone();

    let request_task = cx.background_spawn({
        async move {
            let cursor_offset = position.to_offset(&snapshot);
            let (editable_offset_range, prompt_input) = zeta2_prompt_input(
                &snapshot,
                related_files,
                events,
                excerpt_path,
                cursor_offset,
            );

            let prompt = format_zeta_prompt(&prompt_input);

            if let Some(debug_tx) = &debug_tx {
                debug_tx
                    .unbounded_send(DebugEvent::EditPredictionStarted(
                        EditPredictionStartedDebugEvent {
                            buffer: buffer.downgrade(),
                            prompt: Some(prompt.clone()),
                            position,
                        },
                    ))
                    .ok();
            }

            let request = open_ai::Request {
                model: EDIT_PREDICTIONS_MODEL_ID.clone(),
                messages: vec![open_ai::RequestMessage::User {
                    content: open_ai::MessageContent::Plain(prompt),
                }],
                stream: false,
                max_completion_tokens: None,
                stop: Default::default(),
                temperature: Default::default(),
                tool_choice: None,
                parallel_tool_calls: None,
                tools: vec![],
                prompt_cache_key: None,
                reasoning_effort: None,
            };

            log::trace!("Sending edit prediction request");

            let response = EditPredictionStore::send_raw_llm_request(
                request,
                client,
                llm_token,
                app_version,
                #[cfg(feature = "cli-support")]
                eval_cache,
                #[cfg(feature = "cli-support")]
                EvalCacheEntryKind::Prediction,
            )
            .await;
            let received_response_at = Instant::now();

            log::trace!("Got edit prediction response");

            let (res, usage) = response?;
            let request_id = EditPredictionId(res.id.clone().into());
            let Some(mut output_text) = text_from_response(res) else {
                return Ok((Some((request_id, None)), usage));
            };

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

            if output_text.contains(CURSOR_MARKER) {
                log::trace!("Stripping out {CURSOR_MARKER} from response");
                output_text = output_text.replace(CURSOR_MARKER, "");
            }

            let old_text = snapshot
                .text_for_range(editable_offset_range.clone())
                .collect::<String>();
            let edits: Vec<_> = language::text_diff(&old_text, &output_text)
                .into_iter()
                .map(|(range, text)| {
                    (
                        snapshot.anchor_after(editable_offset_range.start + range.start)
                            ..snapshot.anchor_before(editable_offset_range.start + range.end),
                        text,
                    )
                })
                .collect();

            anyhow::Ok((
                Some((
                    request_id,
                    Some((
                        prompt_input,
                        buffer,
                        snapshot.clone(),
                        edits,
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

        let Some((inputs, edited_buffer, edited_buffer_snapshot, edits, received_response_at)) =
            prediction
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
    related_files: Arc<[zeta_prompt::RelatedFile]>,
    events: Vec<Arc<zeta_prompt::Event>>,
    excerpt_path: Arc<Path>,
    cursor_offset: usize,
) -> (std::ops::Range<usize>, zeta_prompt::ZetaPromptInput) {
    let cursor_point = cursor_offset.to_point(snapshot);

    let (editable_range, context_range) =
        crate::cursor_excerpt::editable_and_context_ranges_for_cursor_position(
            cursor_point,
            snapshot,
            MAX_CONTEXT_TOKENS,
            MAX_REWRITE_TOKENS,
        );

    let context_start_offset = context_range.start.to_offset(snapshot);
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
    if store.custom_predict_edits_url.is_some() && custom_accept_url.is_none() {
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

#[cfg(feature = "cli-support")]
pub fn zeta2_output_for_patch(input: &zeta_prompt::ZetaPromptInput, patch: &str) -> Result<String> {
    let text = &input.cursor_excerpt;
    let editable_region = input.editable_range_in_excerpt.clone();
    let old_prefix = &text[..editable_region.start];
    let old_suffix = &text[editable_region.end..];

    let new = crate::udiff::apply_diff_to_string(patch, text)?;
    if !new.starts_with(old_prefix) || !new.ends_with(old_suffix) {
        anyhow::bail!("Patch shouldn't affect text outside of editable region");
    }

    Ok(new[editable_region.start..new.len() - old_suffix.len()].to_string())
}
