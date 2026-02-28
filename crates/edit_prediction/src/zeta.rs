use crate::cursor_excerpt::compute_excerpt_ranges;
use crate::prediction::EditPredictionResult;
use crate::{
    CurrentEditPrediction, DebugEvent, EditPredictionFinishedDebugEvent, EditPredictionId,
    EditPredictionModelInput, EditPredictionStartedDebugEvent, EditPredictionStore, ollama,
};
use anyhow::{Context as _, Result};
use cloud_llm_client::predict_edits_v3::{RawCompletionRequest, RawCompletionResponse};
use cloud_llm_client::{AcceptEditPredictionBody, EditPredictionRejectReason};
use edit_prediction_types::PredictedCursorPosition;
use futures::AsyncReadExt as _;
use gpui::{App, AppContext as _, Task, http_client, prelude::*};
use language::language_settings::{OpenAiCompatibleEditPredictionSettings, all_language_settings};
use language::{BufferSnapshot, ToOffset as _, ToPoint, text_diff};
use release_channel::AppVersion;
use text::{Anchor, Bias};

use std::env;
use std::ops::Range;
use std::{path::Path, sync::Arc, time::Instant};
use zeta_prompt::{
    CURSOR_MARKER, EditPredictionModelKind, ZetaFormat, clean_zeta2_model_output,
    format_zeta_prompt, get_prefill, prompt_input_contains_special_tokens,
    zeta1::{self, EDITABLE_REGION_END_MARKER},
};

pub fn request_prediction_with_zeta(
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
    let settings = &all_language_settings(None, cx).edit_predictions;
    let provider = settings.provider;
    let custom_server_settings = match provider {
        settings::EditPredictionProvider::Ollama => settings.ollama.clone(),
        settings::EditPredictionProvider::OpenAiCompatibleApi => {
            settings.open_ai_compatible_api.clone()
        }
        _ => None,
    };

    let http_client = cx.http_client();
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

    let can_collect_data = is_open_source && store.is_data_collection_enabled(cx);

    let request_task = cx.background_spawn({
        async move {
            let zeta_version = raw_config
                .as_ref()
                .map(|config| config.format)
                .unwrap_or(ZetaFormat::default());

            let cursor_offset = position.to_offset(&snapshot);
            let editable_range_in_excerpt: Range<usize>;
            let (full_context_offset_range, prompt_input) = zeta2_prompt_input(
                &snapshot,
                related_files,
                events,
                excerpt_path,
                cursor_offset,
                zeta_version,
                preferred_model,
                is_open_source,
                can_collect_data,
            );

            if prompt_input_contains_special_tokens(&prompt_input, zeta_version) {
                return Ok((None, None));
            }

            let is_zeta1 = preferred_model == Some(EditPredictionModelKind::Zeta1);
            let excerpt_ranges = prompt_input
                .excerpt_ranges
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("excerpt_ranges missing from prompt input"))?;

            if let Some(debug_tx) = &debug_tx {
                let prompt = if is_zeta1 {
                    zeta1::format_zeta1_from_input(
                        &prompt_input,
                        excerpt_ranges.editable_350.clone(),
                        excerpt_ranges.editable_350_context_150.clone(),
                    )
                } else {
                    format_zeta_prompt(&prompt_input, zeta_version)
                };
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

            let (request_id, output_text, model_version, usage) = if let Some(custom_settings) =
                &custom_server_settings
            {
                let max_tokens = custom_settings.max_output_tokens * 4;

                if is_zeta1 {
                    let ranges = excerpt_ranges;
                    let prompt = zeta1::format_zeta1_from_input(
                        &prompt_input,
                        ranges.editable_350.clone(),
                        ranges.editable_350_context_150.clone(),
                    );
                    editable_range_in_excerpt = ranges.editable_350.clone();
                    let stop_tokens = vec![
                        EDITABLE_REGION_END_MARKER.to_string(),
                        format!("{EDITABLE_REGION_END_MARKER}\n"),
                        format!("{EDITABLE_REGION_END_MARKER}\n\n"),
                        format!("{EDITABLE_REGION_END_MARKER}\n\n\n"),
                    ];

                    let (response_text, request_id) = send_custom_server_request(
                        provider,
                        custom_settings,
                        prompt,
                        max_tokens,
                        stop_tokens,
                        &http_client,
                    )
                    .await?;

                    let request_id = EditPredictionId(request_id.into());
                    let output_text = zeta1::clean_zeta1_model_output(&response_text);

                    (request_id, output_text, None, None)
                } else {
                    let prompt = format_zeta_prompt(&prompt_input, zeta_version);
                    let prefill = get_prefill(&prompt_input, zeta_version);
                    let prompt = format!("{prompt}{prefill}");

                    editable_range_in_excerpt = prompt_input
                        .excerpt_ranges
                        .as_ref()
                        .map(|ranges| zeta_prompt::excerpt_range_for_format(zeta_version, ranges).0)
                        .unwrap_or(prompt_input.editable_range_in_excerpt.clone());

                    let (response_text, request_id) = send_custom_server_request(
                        provider,
                        custom_settings,
                        prompt,
                        max_tokens,
                        vec![],
                        &http_client,
                    )
                    .await?;

                    let request_id = EditPredictionId(request_id.into());
                    let output_text = if response_text.is_empty() {
                        None
                    } else {
                        let output = format!("{prefill}{response_text}");
                        Some(clean_zeta2_model_output(&output, zeta_version).to_string())
                    };

                    (request_id, output_text, None, None)
                }
            } else if let Some(config) = &raw_config {
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

                editable_range_in_excerpt = prompt_input
                    .excerpt_ranges
                    .as_ref()
                    .map(|ranges| zeta_prompt::excerpt_range_for_format(config.format, ranges).1)
                    .unwrap_or(prompt_input.editable_range_in_excerpt.clone());

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

                (request_id, output_text, None, usage)
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
                editable_range_in_excerpt = response.editable_range;
                let model_version = response.model_version;

                (request_id, output_text, model_version, usage)
            };

            let received_response_at = Instant::now();

            log::trace!("Got edit prediction response");

            let Some(mut output_text) = output_text else {
                return Ok((Some((request_id, None, model_version)), usage));
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

            let editable_range_in_buffer = editable_range_in_excerpt.start
                + full_context_offset_range.start
                ..editable_range_in_excerpt.end + full_context_offset_range.start;

            let mut old_text = snapshot
                .text_for_range(editable_range_in_buffer.clone())
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
                editable_range_in_buffer.start,
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
                        editable_range_in_buffer,
                    )),
                    model_version,
                )),
                usage,
            ))
        }
    });

    cx.spawn(async move |this, cx| {
        let Some((id, prediction, model_version)) =
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
            editable_range_in_buffer,
        )) = prediction
        else {
            return Ok(Some(EditPredictionResult {
                id,
                prediction: Err(EditPredictionRejectReason::Empty),
            }));
        };

        if can_collect_data {
            this.update(cx, |this, cx| {
                this.enqueue_settled_prediction(
                    id.clone(),
                    &project,
                    &edited_buffer,
                    &edited_buffer_snapshot,
                    editable_range_in_buffer,
                    cx,
                );
            })
            .ok();
        }

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
                model_version,
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
    can_collect_data: bool,
) -> (Range<usize>, zeta_prompt::ZetaPromptInput) {
    let cursor_point = cursor_offset.to_point(snapshot);

    let (full_context, full_context_offset_range, excerpt_ranges) =
        compute_excerpt_ranges(cursor_point, snapshot);

    let related_files = crate::filter_redundant_excerpts(
        related_files,
        excerpt_path.as_ref(),
        full_context.start.row..full_context.end.row,
    );

    let full_context_start_offset = full_context_offset_range.start;
    let full_context_start_row = full_context.start.row;

    let editable_offset_range = match preferred_model {
        Some(EditPredictionModelKind::Zeta1) => excerpt_ranges.editable_350.clone(),
        _ => zeta_prompt::excerpt_range_for_format(zeta_format, &excerpt_ranges).0,
    };

    let cursor_offset_in_excerpt = cursor_offset - full_context_start_offset;

    let prompt_input = zeta_prompt::ZetaPromptInput {
        cursor_path: excerpt_path,
        cursor_excerpt: snapshot
            .text_for_range(full_context)
            .collect::<String>()
            .into(),
        editable_range_in_excerpt: editable_offset_range,
        cursor_offset_in_excerpt,
        excerpt_start_row: Some(full_context_start_row),
        events,
        related_files,
        excerpt_ranges: Some(excerpt_ranges),
        preferred_model,
        in_open_source_repo: is_open_source,
        can_collect_data,
    };
    (full_context_offset_range, prompt_input)
}

pub(crate) async fn send_custom_server_request(
    provider: settings::EditPredictionProvider,
    settings: &OpenAiCompatibleEditPredictionSettings,
    prompt: String,
    max_tokens: u32,
    stop_tokens: Vec<String>,
    http_client: &Arc<dyn http_client::HttpClient>,
) -> Result<(String, String)> {
    match provider {
        settings::EditPredictionProvider::Ollama => {
            let response =
                ollama::make_request(settings.clone(), prompt, stop_tokens, http_client.clone())
                    .await?;
            Ok((response.response, response.created_at))
        }
        _ => {
            let request = RawCompletionRequest {
                model: settings.model.clone(),
                prompt,
                max_tokens: Some(max_tokens),
                temperature: None,
                stop: stop_tokens
                    .into_iter()
                    .map(std::borrow::Cow::Owned)
                    .collect(),
                environment: None,
            };

            let request_body = serde_json::to_string(&request)?;
            let http_request = http_client::Request::builder()
                .method(http_client::Method::POST)
                .uri(settings.api_url.as_ref())
                .header("Content-Type", "application/json")
                .body(http_client::AsyncBody::from(request_body))?;

            let mut response = http_client.send(http_request).await?;
            let status = response.status();

            if !status.is_success() {
                let mut body = String::new();
                response.body_mut().read_to_string(&mut body).await?;
                anyhow::bail!("custom server error: {} - {}", status, body);
            }

            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;

            let parsed: RawCompletionResponse =
                serde_json::from_str(&body).context("Failed to parse completion response")?;
            let text = parsed
                .choices
                .into_iter()
                .next()
                .map(|choice| choice.text)
                .unwrap_or_default();
            Ok((text, parsed.id))
        }
    }
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
    let model_version = current_prediction.prediction.model_version;
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
                        model_version: model_version.clone(),
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

pub fn compute_edits(
    old_text: String,
    new_text: &str,
    offset: usize,
    snapshot: &BufferSnapshot,
) -> Vec<(Range<Anchor>, Arc<str>)> {
    compute_edits_and_cursor_position(old_text, new_text, offset, None, snapshot).0
}

pub fn compute_edits_and_cursor_position(
    old_text: String,
    new_text: &str,
    offset: usize,
    cursor_offset_in_new_text: Option<usize>,
    snapshot: &BufferSnapshot,
) -> (
    Vec<(Range<Anchor>, Arc<str>)>,
    Option<PredictedCursorPosition>,
) {
    let diffs = text_diff(&old_text, new_text);

    // Delta represents the cumulative change in byte count from all preceding edits.
    // new_offset = old_offset + delta, so old_offset = new_offset - delta
    let mut delta: isize = 0;
    let mut cursor_position: Option<PredictedCursorPosition> = None;
    let buffer_len = snapshot.len();

    let edits = diffs
        .iter()
        .map(|(raw_old_range, new_text)| {
            // Compute cursor position if it falls within or before this edit.
            if let (Some(cursor_offset), None) = (cursor_offset_in_new_text, cursor_position) {
                let edit_start_in_new = (raw_old_range.start as isize + delta) as usize;
                let edit_end_in_new = edit_start_in_new + new_text.len();

                if cursor_offset < edit_start_in_new {
                    let cursor_in_old = (cursor_offset as isize - delta) as usize;
                    let buffer_offset = (offset + cursor_in_old).min(buffer_len);
                    cursor_position = Some(PredictedCursorPosition::at_anchor(
                        snapshot.anchor_after(buffer_offset),
                    ));
                } else if cursor_offset < edit_end_in_new {
                    let buffer_offset = (offset + raw_old_range.start).min(buffer_len);
                    let offset_within_insertion = cursor_offset - edit_start_in_new;
                    cursor_position = Some(PredictedCursorPosition::new(
                        snapshot.anchor_before(buffer_offset),
                        offset_within_insertion,
                    ));
                }

                delta += new_text.len() as isize - raw_old_range.len() as isize;
            }

            // Compute the edit with prefix/suffix trimming.
            let mut old_range = raw_old_range.clone();
            let old_slice = &old_text[old_range.clone()];

            let prefix_len = common_prefix(old_slice.chars(), new_text.chars());
            let suffix_len = common_prefix(
                old_slice[prefix_len..].chars().rev(),
                new_text[prefix_len..].chars().rev(),
            );

            old_range.start += offset;
            old_range.end += offset;
            old_range.start += prefix_len;
            old_range.end -= suffix_len;

            old_range.start = old_range.start.min(buffer_len);
            old_range.end = old_range.end.min(buffer_len);

            let new_text = new_text[prefix_len..new_text.len() - suffix_len].into();
            let range = if old_range.is_empty() {
                let anchor = snapshot.anchor_after(old_range.start);
                anchor..anchor
            } else {
                snapshot.anchor_after(old_range.start)..snapshot.anchor_before(old_range.end)
            };
            (range, new_text)
        })
        .collect();

    if let (Some(cursor_offset), None) = (cursor_offset_in_new_text, cursor_position) {
        let cursor_in_old = (cursor_offset as isize - delta) as usize;
        let buffer_offset = snapshot.clip_offset(offset + cursor_in_old, Bias::Right);
        cursor_position = Some(PredictedCursorPosition::at_anchor(
            snapshot.anchor_after(buffer_offset),
        ));
    }

    (edits, cursor_position)
}

fn common_prefix<T1: Iterator<Item = char>, T2: Iterator<Item = char>>(a: T1, b: T2) -> usize {
    a.zip(b)
        .take_while(|(a, b)| a == b)
        .map(|(a, _)| a.len_utf8())
        .sum()
}
