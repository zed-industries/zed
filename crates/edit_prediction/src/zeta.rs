use crate::{
    CurrentEditPrediction, DebugEvent, EditPredictionFinishedDebugEvent, EditPredictionId,
    EditPredictionModelInput, EditPredictionStartedDebugEvent, EditPredictionStore, StoredEvent,
    ZedUpdateRequiredError,
    cursor_excerpt::{self, compute_cursor_excerpt, compute_syntax_ranges},
    prediction::EditPredictionResult,
};
use anyhow::Result;
use cloud_llm_client::{
    AcceptEditPredictionBody, EditPredictionRejectReason, predict_edits_v3::RawCompletionRequest,
};
use edit_prediction_types::PredictedCursorPosition;
use gpui::{App, AppContext as _, Entity, Task, WeakEntity, prelude::*};
use language::{
    Buffer, BufferSnapshot, DiagnosticSeverity, OffsetRangeExt as _, ToOffset as _,
    language_settings::all_language_settings, text_diff,
};
use release_channel::AppVersion;
use settings::EditPredictionPromptFormat;
use text::{Anchor, Bias, Point};
use ui::SharedString;
use workspace::notifications::{ErrorMessagePrompt, NotificationId, show_app_notification};
use zeta_prompt::{ParsedOutput, ZetaPromptInput};

use std::{env, ops::Range, path::Path, sync::Arc};
use zeta_prompt::{
    ZetaFormat, format_zeta_prompt, get_prefill, parse_zeta2_model_output,
    parsed_output_from_editable_region, prompt_input_contains_special_tokens,
    stop_tokens_for_format,
    zeta1::{self, EDITABLE_REGION_END_MARKER},
};

use crate::open_ai_compatible::{
    load_open_ai_compatible_api_key_if_needed, send_custom_server_request,
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
        mode,
        trigger,
        project,
        diagnostic_search_range,
        can_collect_data,
        is_open_source,
        ..
    }: EditPredictionModelInput,
    capture_data: Option<Vec<StoredEvent>>,
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
    let request_start = cx.background_executor().now();
    let raw_config = store.zeta2_raw_config().cloned();
    let preferred_experiment = store.preferred_experiment().map(|s| s.to_owned());
    let open_ai_compatible_api_key = load_open_ai_compatible_api_key_if_needed(provider, cx);

    let excerpt_path: Arc<Path> = snapshot
        .file()
        .map(|file| -> Arc<Path> { file.full_path(cx).into() })
        .unwrap_or_else(|| Arc::from(Path::new("untitled")));

    let repo_url = if can_collect_data {
        let buffer_id = buffer.read(cx).remote_id();
        project
            .read(cx)
            .git_store()
            .read(cx)
            .repository_and_path_for_buffer_id(buffer_id, cx)
            .and_then(|(repo, _)| repo.read(cx).default_remote_url())
    } else {
        None
    };
    let client = store.client.clone();
    let llm_token = store.llm_token.clone();
    let organization_id = store
        .user_store
        .read(cx)
        .current_organization()
        .map(|organization| organization.id.clone());
    let app_version = AppVersion::global(cx);

    struct Prediction {
        prompt_input: ZetaPromptInput,
        buffer: Entity<Buffer>,
        snapshot: BufferSnapshot,
        edits: Vec<(Range<Anchor>, Arc<str>)>,
        cursor_position: Option<PredictedCursorPosition>,
        editable_range_in_buffer: Range<usize>,
        model_version: Option<String>,
    }

    let request_task = cx.background_spawn({
        async move {
            let zeta_version = raw_config
                .as_ref()
                .map(|config| config.format)
                .unwrap_or(ZetaFormat::default());

            let cursor_offset = position.to_offset(&snapshot);
            let (full_context_offset_range, prompt_input) = zeta2_prompt_input(
                &snapshot,
                related_files,
                events,
                diagnostic_search_range,
                excerpt_path,
                cursor_offset,
                is_open_source,
                can_collect_data,
                repo_url,
            );

            if prompt_input_contains_special_tokens(&prompt_input, zeta_version) {
                return Err(anyhow::anyhow!("prompt contains special tokens"));
            }

            let formatted_prompt = format_zeta_prompt(&prompt_input, zeta_version);

            if let Some(debug_tx) = &debug_tx {
                debug_tx
                    .unbounded_send(DebugEvent::EditPredictionStarted(
                        EditPredictionStartedDebugEvent {
                            buffer: buffer.downgrade(),
                            prompt: formatted_prompt.clone(),
                            position,
                        },
                    ))
                    .ok();
            }

            log::trace!("Sending edit prediction request");

            let Some((request_id, output, model_version, usage)) =
                (if let Some(custom_settings) = &custom_server_settings {
                    let max_tokens = custom_settings.max_output_tokens * 4;

                    Some(match custom_settings.prompt_format {
                        EditPredictionPromptFormat::Zeta => {
                            let ranges = &prompt_input.excerpt_ranges;
                            let editable_range_in_excerpt = ranges.editable_350.clone();
                            let prompt = zeta1::format_zeta1_from_input(
                                &prompt_input,
                                editable_range_in_excerpt.clone(),
                                ranges.editable_350_context_150.clone(),
                            );
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
                                open_ai_compatible_api_key.clone(),
                                &http_client,
                            )
                            .await?;

                            let request_id = EditPredictionId(request_id.into());
                            let output_text = zeta1::clean_zeta1_model_output(&response_text);
                            let parsed_output = output_text.map(|text| ParsedOutput {
                                new_editable_region: text,
                                range_in_excerpt: editable_range_in_excerpt,
                                cursor_offset_in_new_editable_region: None,
                            });

                            (request_id, parsed_output, None, None)
                        }
                        EditPredictionPromptFormat::Zeta2 => {
                            let Some(prompt) = formatted_prompt.clone() else {
                                return Ok((None, None));
                            };
                            let prefill = get_prefill(&prompt_input, zeta_version);
                            let prompt = format!("{prompt}{prefill}");

                            let (response_text, request_id) = send_custom_server_request(
                                provider,
                                custom_settings,
                                prompt,
                                max_tokens,
                                stop_tokens_for_format(zeta_version)
                                    .iter()
                                    .map(|token| token.to_string())
                                    .collect(),
                                open_ai_compatible_api_key.clone(),
                                &http_client,
                            )
                            .await?;

                            let request_id = EditPredictionId(request_id.into());
                            let output_text = if response_text.is_empty() {
                                None
                            } else {
                                let output = format!("{prefill}{response_text}");
                                Some(parse_zeta2_model_output(
                                    &output,
                                    zeta_version,
                                    &prompt_input,
                                )?)
                            };

                            (request_id, output_text, None, None)
                        }
                        _ => anyhow::bail!("unsupported prompt format"),
                    })
                } else if let Some(config) = &raw_config {
                    let Some(prompt) = format_zeta_prompt(&prompt_input, config.format) else {
                        return Ok((None, None));
                    };
                    let prefill = get_prefill(&prompt_input, config.format);
                    let prompt = format!("{prompt}{prefill}");
                    let environment = config
                        .environment
                        .clone()
                        .or_else(|| Some(config.format.to_string().to_lowercase()));
                    let request = RawCompletionRequest {
                        model: config.model_id.clone().unwrap_or_default(),
                        prompt,
                        temperature: None,
                        stop: stop_tokens_for_format(config.format)
                            .iter()
                            .map(|token| std::borrow::Cow::Borrowed(*token))
                            .collect(),
                        max_tokens: Some(2048),
                        environment,
                    };

                    let (mut response, usage) = EditPredictionStore::send_raw_llm_request(
                        request,
                        client,
                        None,
                        llm_token,
                        organization_id,
                        app_version,
                    )
                    .await?;

                    let request_id = EditPredictionId(response.id.clone().into());
                    let output = if let Some(choice) = response.choices.pop() {
                        let response = &choice.text;
                        let output = format!("{prefill}{response}");
                        Some(parse_zeta2_model_output(
                            &output,
                            config.format,
                            &prompt_input,
                        )?)
                    } else {
                        None
                    };

                    Some((request_id, output, None, usage))
                } else {
                    // Use V3 endpoint - server handles model/version selection and suffix stripping
                    let (response, usage) = EditPredictionStore::send_v3_request(
                        prompt_input.clone(),
                        preferred_experiment.clone(),
                        client,
                        llm_token,
                        organization_id,
                        app_version,
                        trigger,
                        mode,
                    )
                    .await?;

                    let request_id = EditPredictionId(response.request_id.into());
                    let output_text = Some(response.output).filter(|s| !s.is_empty());
                    let model_version = response.model_version;
                    let parsed_output = parsed_output_from_editable_region(
                        response.editable_range,
                        output_text.unwrap_or_default(),
                    );

                    Some((request_id, Some(parsed_output), model_version, usage))
                })
            else {
                return Ok((None, None));
            };

            log::trace!("Got edit prediction response");

            let Some(ParsedOutput {
                new_editable_region: mut output_text,
                range_in_excerpt: editable_range_in_excerpt,
                cursor_offset_in_new_editable_region: cursor_offset_in_output,
            }) = output
            else {
                return Ok((Some((request_id, None)), None));
            };

            let editable_range_in_buffer = editable_range_in_excerpt.start
                + full_context_offset_range.start
                ..editable_range_in_excerpt.end + full_context_offset_range.start;

            let mut old_text = snapshot
                .text_for_range(editable_range_in_buffer.clone())
                .collect::<String>();

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
                    Some(Prediction {
                        prompt_input,
                        buffer,
                        snapshot: snapshot.clone(),
                        edits,
                        cursor_position,
                        editable_range_in_buffer,
                        model_version,
                    }),
                )),
                usage,
            ))
        }
    });

    cx.spawn(async move |this, cx| {
        let Some((id, prediction)) = handle_api_response(&this, request_task.await, cx)? else {
            return Ok(None);
        };
        let request_duration = cx.background_executor().now() - request_start;

        let Some(Prediction {
            prompt_input: inputs,
            buffer: edited_buffer,
            snapshot: edited_buffer_snapshot,
            edits,
            cursor_position,
            editable_range_in_buffer,
            model_version,
        }) = prediction
        else {
            return Ok(Some(EditPredictionResult {
                id,
                e2e_latency: request_duration,
                prediction: Err(EditPredictionRejectReason::Empty),
            }));
        };

        let result = EditPredictionResult::new(
            id,
            &edited_buffer,
            &edited_buffer_snapshot,
            edits.into(),
            cursor_position,
            inputs,
            model_version,
            request_duration,
            cx,
        )
        .await;

        if can_collect_data && let Ok(prediction) = &result.prediction {
            let weak_this = this.clone();
            let request_id = prediction.id.clone();
            let edited_buffer = edited_buffer.clone();
            let edited_buffer_snapshot = edited_buffer_snapshot.clone();
            let editable_range_in_buffer = editable_range_in_buffer.clone();
            let edit_preview = prediction.edit_preview.clone();
            let example_task = capture_data.and_then(|stored_events| {
                cx.update(|cx| {
                    crate::capture_example(
                        project.clone(),
                        edited_buffer.clone(),
                        position,
                        stored_events,
                        false,
                        cx,
                    )
                })
            });
            cx.spawn(async move |cx| {
                let example_spec = if let Some(task) = example_task {
                    task.await.ok()
                } else {
                    None
                };

                weak_this
                    .update(cx, |this, cx| {
                        this.enqueue_settled_prediction(
                            request_id.clone(),
                            &project,
                            &edited_buffer,
                            &edited_buffer_snapshot,
                            editable_range_in_buffer,
                            &edit_preview,
                            example_spec,
                            request_duration,
                            cx,
                        );
                    })
                    .ok();
            })
            .detach();
        }

        Ok(Some(result))
    })
}

fn handle_api_response<T>(
    this: &WeakEntity<EditPredictionStore>,
    response: Result<(T, Option<client::EditPredictionUsage>)>,
    cx: &mut gpui::AsyncApp,
) -> Result<T> {
    match response {
        Ok((data, usage)) => {
            if let Some(usage) = usage {
                this.update(cx, |this, cx| {
                    this.user_store.update(cx, |user_store, cx| {
                        user_store.update_edit_prediction_usage(usage, cx);
                    });
                })
                .ok();
            }
            Ok(data)
        }
        Err(err) => {
            if err.is::<ZedUpdateRequiredError>() {
                cx.update(|cx| {
                    this.update(cx, |this, _cx| {
                        this.update_required = true;
                    })
                    .ok();

                    let error_message: SharedString = err.to_string().into();
                    show_app_notification(
                        NotificationId::unique::<ZedUpdateRequiredError>(),
                        cx,
                        move |cx| {
                            cx.new(|cx| {
                                ErrorMessagePrompt::new(error_message.clone(), cx)
                                    .with_link_button("Update Zed", "https://zed.dev/releases")
                            })
                        },
                    );
                });
            }
            Err(err)
        }
    }
}

pub(crate) fn active_buffer_diagnostics(
    snapshot: &language::BufferSnapshot,
    diagnostic_search_range: Range<Point>,
    additional_context_token_count: usize,
) -> Vec<zeta_prompt::ActiveBufferDiagnostic> {
    snapshot
        .diagnostics_in_range::<Point, Point>(diagnostic_search_range, false)
        .map(|entry| {
            let severity = match entry.diagnostic.severity {
                DiagnosticSeverity::ERROR => Some(1),
                DiagnosticSeverity::WARNING => Some(2),
                DiagnosticSeverity::INFORMATION => Some(3),
                DiagnosticSeverity::HINT => Some(4),
                _ => None,
            };
            let diagnostic_point_range = entry.range.clone();
            let snippet_point_range = cursor_excerpt::expand_context_syntactically_then_linewise(
                snapshot,
                diagnostic_point_range.clone(),
                additional_context_token_count,
            );
            let snippet = snapshot
                .text_for_range(snippet_point_range.clone())
                .collect::<String>();
            let snippet_start_offset = snippet_point_range.start.to_offset(snapshot);
            let diagnostic_offset_range = diagnostic_point_range.to_offset(snapshot);
            zeta_prompt::ActiveBufferDiagnostic {
                severity,
                message: entry.diagnostic.message.clone(),
                snippet,
                snippet_buffer_row_range: diagnostic_point_range.start.row
                    ..diagnostic_point_range.end.row,
                diagnostic_range_in_snippet: diagnostic_offset_range.start - snippet_start_offset
                    ..diagnostic_offset_range.end - snippet_start_offset,
            }
        })
        .collect()
}

pub fn zeta2_prompt_input(
    snapshot: &language::BufferSnapshot,
    related_files: Vec<zeta_prompt::RelatedFile>,
    events: Vec<Arc<zeta_prompt::Event>>,
    diagnostic_search_range: Range<Point>,
    excerpt_path: Arc<Path>,
    cursor_offset: usize,
    is_open_source: bool,
    can_collect_data: bool,
    repo_url: Option<String>,
) -> (Range<usize>, zeta_prompt::ZetaPromptInput) {
    let (excerpt_point_range, excerpt_offset_range, cursor_offset_in_excerpt) =
        compute_cursor_excerpt(snapshot, cursor_offset);

    let cursor_excerpt: Arc<str> = snapshot
        .text_for_range(excerpt_point_range.clone())
        .collect::<String>()
        .into();
    let syntax_ranges = compute_syntax_ranges(snapshot, cursor_offset, &excerpt_offset_range);
    let excerpt_ranges = zeta_prompt::compute_legacy_excerpt_ranges(
        &cursor_excerpt,
        cursor_offset_in_excerpt,
        &syntax_ranges,
    );

    let active_buffer_diagnostics =
        active_buffer_diagnostics(snapshot, diagnostic_search_range, 100);

    let prompt_input = zeta_prompt::ZetaPromptInput {
        cursor_path: excerpt_path,
        cursor_excerpt,
        cursor_offset_in_excerpt,
        excerpt_start_row: Some(excerpt_point_range.start.row),
        events,
        related_files: Some(related_files),
        active_buffer_diagnostics,
        excerpt_ranges,
        syntax_ranges: Some(syntax_ranges),
        in_open_source_repo: is_open_source,
        can_collect_data,
        repo_url,
    };
    (excerpt_offset_range, prompt_input)
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
    let e2e_latency = current_prediction.e2e_latency;
    let require_auth = custom_accept_url.is_none();
    let client = store.client.clone();
    let llm_token = store.llm_token.clone();
    let organization_id = store
        .user_store
        .read(cx)
        .current_organization()
        .map(|organization| organization.id.clone());
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
                        e2e_latency_ms: Some(e2e_latency.as_millis()),
                    })?
                    .into(),
                );
                Ok(req?)
            },
            client,
            llm_token,
            organization_id,
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
