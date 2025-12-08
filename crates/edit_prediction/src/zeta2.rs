#[cfg(feature = "eval-support")]
use crate::EvalCacheEntryKind;
use crate::cursor_excerpt::editable_and_context_ranges_for_cursor_position;
use crate::open_ai_response::text_from_response;
use crate::prediction::EditPredictionResult;
use crate::{
    DebugEvent, EDIT_PREDICTIONS_MODEL_ID, EditPredictionId, EditPredictionInputs,
    EditPredictionRequestedDebugEvent, EditPredictionStore,
};
use anyhow::{Result, anyhow, bail};
use cloud_llm_client::predict_edits_v3::{self, Event, PromptFormat};
use cloud_llm_client::{EditPredictionRejectReason, PredictEditsRequestTrigger};
use cloud_zeta2_prompt::CURSOR_MARKER;
use edit_prediction_context::{EditPredictionExcerpt, Line};
use edit_prediction_context::{RelatedExcerpt, RelatedFile};
use futures::channel::oneshot;
use gpui::{Entity, Task, prelude::*};
use language::{Anchor, BufferSnapshot, OffsetRangeExt};
use language::{Buffer, Point, ToOffset as _, ToPoint};
use project::{Project, ProjectItem as _};
use release_channel::AppVersion;
use std::{
    env,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};

pub(crate) const MAX_CONTEXT_TOKENS: usize = 150;
pub(crate) const MAX_REWRITE_TOKENS: usize = 350;

pub fn request_prediction_with_zeta2(
    store: &mut EditPredictionStore,
    project: &Entity<Project>,
    active_buffer: &Entity<Buffer>,
    active_snapshot: BufferSnapshot,
    position: Anchor,
    events: Vec<Arc<Event>>,
    mut included_files: Vec<RelatedFile>,
    trigger: PredictEditsRequestTrigger,
    cx: &mut Context<EditPredictionStore>,
) -> Task<Result<Option<EditPredictionResult>>> {
    let options = store.options.clone();
    let buffer_snapshotted_at = Instant::now();

    let Some((excerpt_path, active_project_path)) = active_snapshot
        .file()
        .map(|file| -> Arc<Path> { file.full_path(cx).into() })
        .zip(active_buffer.read(cx).project_path(cx))
    else {
        return Task::ready(Err(anyhow!("No file path for excerpt")));
    };

    let client = store.client.clone();
    let llm_token = store.llm_token.clone();
    let app_version = AppVersion::global(cx);
    let debug_tx = store.debug_tx.clone();

    let file = active_buffer.read(cx).file();

    let active_file_full_path = file.as_ref().map(|f| f.full_path(cx));

    // TODO data collection
    let can_collect_data = file
        .as_ref()
        .map_or(false, |file| store.can_collect_file(project, file, cx));

    #[cfg(feature = "eval-support")]
    let eval_cache = store.eval_cache.clone();

    let request_task = cx.background_spawn({
        let active_buffer = active_buffer.clone();
        async move {
            let cursor_offset = position.to_offset(&active_snapshot);
            let cursor_point = cursor_offset.to_point(&active_snapshot);

            let before_retrieval = Instant::now();

            let excerpt_options = options.context;

            let (editable_range, context_range) = editable_and_context_ranges_for_cursor_position(
                cursor_point,
                &active_snapshot,
                MAX_REWRITE_TOKENS,
                MAX_CONTEXT_TOKENS,
            );
            let excerpt = active_snapshot
                .text_for_range(context_range.clone())
                .collect::<String>();

            let context_offset = context_range.start.to_offset(&active_snapshot);
            let editable_offset_range = editable_range.to_offset(&active_snapshot);
            let excerpt_anchor_range = active_snapshot.anchor_after(context_offset)
                ..active_snapshot.anchor_before(context_offset);

            let included_files = included_files
                .iter()
                .map(|related_file| predict_edits_v3::RelatedFile {
                    path: Arc::from(related_file.path.path.as_std_path()),
                    max_row: Line(related_file.max_row),
                    excerpts: related_file
                        .excerpts
                        .iter()
                        .map(|excerpt| predict_edits_v3::Excerpt {
                            start_line: Line(excerpt.point_range.start.row),
                            text: excerpt.text.to_string().into(),
                        })
                        .collect(),
                })
                .collect::<Vec<_>>();

            let cloud_request = predict_edits_v3::PredictEditsRequest {
                excerpt_path,
                excerpt,
                cursor_point: predict_edits_v3::Point {
                    line: predict_edits_v3::Line(cursor_point.row),
                    column: cursor_point.column,
                },
                related_files: included_files,
                events,
                can_collect_data,
                debug_info: debug_tx.is_some(),
                prompt_max_bytes: Some(options.max_prompt_bytes),
                prompt_format: options.prompt_format,
                git_info: None,
                trigger,
                editable_range_in_excerpt: (editable_offset_range.start - context_offset)
                    ..(editable_offset_range.end - context_offset),
                cursor_offset_in_excerpt: cursor_offset - context_offset,
            };

            let prompt_result = cloud_zeta2_prompt::build_prompt(&cloud_request);

            let inputs = EditPredictionInputs {
                included_files: cloud_request.related_files,
                events: cloud_request.events,
                cursor_point: cloud_request.cursor_point,
                cursor_path: cloud_request.excerpt_path,
            };

            let retrieval_time = Instant::now() - before_retrieval;

            let debug_response_tx = if let Some(debug_tx) = &debug_tx {
                let (response_tx, response_rx) = oneshot::channel();

                debug_tx
                    .unbounded_send(DebugEvent::EditPredictionRequested(
                        EditPredictionRequestedDebugEvent {
                            inputs: inputs.clone(),
                            retrieval_time,
                            buffer: active_buffer.downgrade(),
                            local_prompt: match prompt_result.as_ref() {
                                Ok(prompt) => Ok(prompt.clone()),
                                Err(err) => Err(err.to_string()),
                            },
                            position,
                            response_rx,
                        },
                    ))
                    .ok();
                Some(response_tx)
            } else {
                None
            };

            if cfg!(debug_assertions) && env::var("ZED_ZETA2_SKIP_REQUEST").is_ok() {
                if let Some(debug_response_tx) = debug_response_tx {
                    debug_response_tx
                        .send((Err("Request skipped".to_string()), Duration::ZERO))
                        .ok();
                }
                anyhow::bail!("Skipping request because ZED_ZETA2_SKIP_REQUEST is set")
            }

            let prompt = prompt_result?;

            eprintln!("prompt:\n{prompt}");

            let generation_params =
                cloud_zeta2_prompt::generation_params(cloud_request.prompt_format);
            let request = open_ai::Request {
                model: EDIT_PREDICTIONS_MODEL_ID.clone(),
                messages: vec![open_ai::RequestMessage::User {
                    content: open_ai::MessageContent::Plain(prompt),
                }],
                stream: false,
                max_completion_tokens: None,
                max_tokens: Some(1024 * 4),
                stop: generation_params.stop.unwrap_or_default(),
                temperature: generation_params.temperature.or(Some(0.7)),
                tool_choice: None,
                parallel_tool_calls: None,
                tools: vec![],
                prompt_cache_key: None,
                reasoning_effort: None,
            };

            log::trace!("Sending edit prediction request");

            let before_request = Instant::now();
            let response = EditPredictionStore::send_raw_llm_request(
                request,
                client,
                llm_token,
                app_version,
                #[cfg(feature = "eval-support")]
                eval_cache,
                #[cfg(feature = "eval-support")]
                EvalCacheEntryKind::Prediction,
            )
            .await;
            let received_response_at = Instant::now();
            let request_time = received_response_at - before_request;

            log::trace!("Got edit prediction response");

            if let Some(debug_response_tx) = debug_response_tx {
                debug_response_tx
                    .send((
                        response
                            .as_ref()
                            .map_err(|err| err.to_string())
                            .map(|response| response.0.clone()),
                        request_time,
                    ))
                    .ok();
            }

            let (res, usage) = response?;
            let request_id = EditPredictionId(res.id.clone().into());
            let Some(mut output_text) = text_from_response(res) else {
                return Ok((Some((request_id, None)), usage));
            };

            if output_text.contains(CURSOR_MARKER) {
                log::trace!("Stripping out {CURSOR_MARKER} from response");
                output_text = output_text.replace(CURSOR_MARKER, "");
            }

            let get_buffer_from_context = |path: &Path| {
                if Some(path) == active_file_full_path.as_deref() {
                    Some((
                        &active_snapshot,
                        std::slice::from_ref(&excerpt_anchor_range),
                    ))
                } else {
                    None
                }
            };

            let edits = match options.prompt_format {
                PromptFormat::Minimal | PromptFormat::MinimalQwen | PromptFormat::SeedCoder1120 => {
                    if output_text.contains("--- a/\n+++ b/\nNo edits") {
                        vec![]
                    } else {
                        crate::udiff::parse_diff(&output_text, get_buffer_from_context)
                            .await?
                            .1
                    }
                }
                PromptFormat::OldTextNewText => {
                    crate::xml_edits::parse_xml_edits(&output_text, get_buffer_from_context)
                        .await?
                        .1
                }
                PromptFormat::Zeta => {
                    let old_text = active_snapshot
                        .text_for_range(editable_offset_range.clone())
                        .collect::<String>();
                    let new_text = output_text.trim_end_matches("<|im_end|>");
                    eprintln!("OUTPUT:\n<old_text>\n{old_text}\n</old_text>\n<new_text>\n{new_text}\n</new_text>");

                    language::text_diff(&old_text, &new_text)
                        .into_iter()
                        .map(|(range, text)| {
                            (
                                active_snapshot
                                    .anchor_after(editable_offset_range.start + range.start)
                                    ..active_snapshot
                                        .anchor_before(editable_offset_range.start + range.end),
                                text,
                            )
                        })
                        .collect()
                }
                _ => {
                    bail!("unsupported prompt format {}", options.prompt_format)
                }
            };

            anyhow::Ok((
                Some((
                    request_id,
                    Some((
                        inputs,
                        active_buffer,
                        active_snapshot.clone(),
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
