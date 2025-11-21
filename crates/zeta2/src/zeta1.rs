mod input_excerpt;

use std::ops::Range;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::LlmApiToken;
use crate::ZedUpdateRequiredError;
use crate::Zeta;
use crate::prediction::EditPrediction;
use crate::{EditPredictionId, Event};
use anyhow::Result;
use client::{Client, EditPredictionUsage};
use cloud_llm_client::{
    EXPIRED_LLM_TOKEN_HEADER_NAME, MINIMUM_REQUIRED_VERSION_HEADER_NAME, PredictEditsResponse,
    ZED_VERSION_HEADER_NAME,
};
use cloud_llm_client::{PredictEditsBody, PredictEditsGitInfo};
use gpui::http_client::Method;
use gpui::{App, AsyncApp, Context, Entity, Task};
use gpui::{SemanticVersion, SharedString, http_client};
use input_excerpt::excerpt_for_cursor_position;
use language::{Anchor, Buffer, BufferSnapshot, ToPoint as _, text_diff};
use project::Project;
use project::ProjectPath;
use release_channel::AppVersion;
use uuid::Uuid;
use workspace::notifications::{ErrorMessagePrompt, NotificationId, show_app_notification};

const CURSOR_MARKER: &str = "<|user_cursor_is_here|>";
const START_OF_FILE_MARKER: &str = "<|start_of_file|>";
const EDITABLE_REGION_START_MARKER: &str = "<|editable_region_start|>";
const EDITABLE_REGION_END_MARKER: &str = "<|editable_region_end|>";
const BUFFER_CHANGE_GROUPING_INTERVAL: Duration = Duration::from_secs(1);
const ZED_PREDICT_DATA_COLLECTION_CHOICE: &str = "zed_predict_data_collection_choice";

const MAX_CONTEXT_TOKENS: usize = 150;
const MAX_REWRITE_TOKENS: usize = 350;
const MAX_EVENT_TOKENS: usize = 500;

pub struct PerformPredictEditsParams {
    pub client: Arc<Client>,
    pub llm_token: LlmApiToken,
    pub app_version: SemanticVersion,
    pub body: PredictEditsBody,
}

pub(crate) fn request_prediction_with_zeta1(
    zeta: &mut Zeta,
    project: &Entity<Project>,
    buffer: &Entity<Buffer>,
    position: language::Anchor,
    cx: &mut Context<Zeta>,
) -> Task<Result<Option<EditPrediction>>> {
    let buffer = buffer.clone();
    let buffer_snapshotted_at = Instant::now();
    let snapshot = buffer.read(cx).snapshot();
    let client = zeta.client.clone();
    let llm_token = zeta.llm_token.clone();
    let app_version = AppVersion::global(cx);

    let zeta_project = zeta.get_or_init_zeta_project(project, cx);
    let this = cx.entity();
    let mut events = Vec::with_capacity(zeta_project.events.len());
    events.extend(zeta_project.events.iter().cloned());
    let events = Arc::new(events);

    let (git_info, can_collect_file) = if let Some(file) = snapshot.file() {
        let can_collect_file = zeta.can_collect_file(file, cx);
        let git_info = if can_collect_file {
            git_info_for_file(project, &ProjectPath::from_file(file.as_ref(), cx), cx)
        } else {
            None
        };
        (git_info, can_collect_file)
    } else {
        (None, false)
    };

    let full_path: Arc<Path> = snapshot
        .file()
        .map(|f| Arc::from(f.full_path(cx).as_path()))
        .unwrap_or_else(|| Arc::from(Path::new("untitled")));
    let full_path_str = full_path.to_string_lossy().into_owned();
    let cursor_point = position.to_point(&snapshot);
    let cursor_offset = cursor_point.to_offset(&snapshot);
    let prompt_for_events = {
        let events = events.clone();
        move || prompt_for_events_impl(&events, MAX_EVENT_TOKENS, cx)
    };
    let gather_task = gather_context(
        full_path_str,
        &snapshot,
        cursor_point,
        prompt_for_events,
        cx,
    );

    cx.spawn(async move |this, cx| {
        let GatherContextOutput {
            mut body,
            editable_range,
            included_events_count,
        } = gather_task.await?;
        let done_gathering_context_at = Instant::now();

        let included_events = &events[events.len() - included_events_count..events.len()];
        body.can_collect_data = can_collect_file
            && this
                .read_with(cx, |this, cx| this.can_collect_events(included_events, cx))
                .unwrap_or(false);
        if body.can_collect_data {
            body.git_info = git_info;
        }

        log::debug!(
            "Events:\n{}\nExcerpt:\n{:?}",
            body.input_events,
            body.input_excerpt
        );

        let input_outline = body.outline.clone().unwrap_or_default();
        let input_events = body.input_events.clone();
        let input_excerpt = body.input_excerpt.clone();

        let response = perform_predict_edits(PerformPredictEditsParams {
            client,
            llm_token,
            app_version,
            body,
        })
        .await;
        let (response, usage) = match response {
            Ok(response) => response,
            Err(err) => {
                if err.is::<ZedUpdateRequiredError>() {
                    cx.update(|cx| {
                        this.update(cx, |zeta, _cx| {
                            zeta.update_required = true;
                        });

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
                    })
                    .ok();
                }

                return Err(err);
            }
        };

        let received_response_at = Instant::now();
        log::debug!("completion response: {}", &response.output_excerpt);

        if let Some(usage) = usage {
            this.update(cx, |this, cx| {
                this.user_store.update(cx, |user_store, cx| {
                    user_store.update_edit_prediction_usage(usage, cx);
                });
            })
            .ok();
        }

        let edit_prediction = process_completion_response(
            response,
            buffer,
            &snapshot,
            editable_range,
            cursor_offset,
            full_path,
            input_outline,
            input_events,
            input_excerpt,
            buffer_snapshotted_at,
            cx,
        )
        .await;

        let finished_at = Instant::now();

        // record latency for ~1% of requests
        if rand::random::<u8>() <= 2 {
            telemetry::event!(
                "Edit Prediction Request",
                context_latency = done_gathering_context_at
                    .duration_since(buffer_snapshotted_at)
                    .as_millis(),
                request_latency = received_response_at
                    .duration_since(done_gathering_context_at)
                    .as_millis(),
                process_latency = finished_at.duration_since(received_response_at).as_millis()
            );
        }

        edit_prediction
    })
}

pub fn perform_predict_edits(
    params: PerformPredictEditsParams,
) -> impl Future<Output = Result<(PredictEditsResponse, Option<EditPredictionUsage>)>> {
    async move {
        let PerformPredictEditsParams {
            client,
            llm_token,
            app_version,
            body,
            ..
        } = params;

        let http_client = client.http_client();
        let mut token = llm_token.acquire(&client).await?;
        let mut did_retry = false;

        loop {
            let request_builder = http_client::Request::builder().method(Method::POST);
            let request_builder =
                if let Ok(predict_edits_url) = std::env::var("ZED_PREDICT_EDITS_URL") {
                    request_builder.uri(predict_edits_url)
                } else {
                    request_builder.uri(
                        http_client
                            .build_zed_llm_url("/predict_edits/v2", &[])?
                            .as_ref(),
                    )
                };
            let request = request_builder
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", token))
                .header(ZED_VERSION_HEADER_NAME, app_version.to_string())
                .body(serde_json::to_string(&body)?.into())?;

            let mut response = http_client.send(request).await?;

            if let Some(minimum_required_version) = response
                .headers()
                .get(MINIMUM_REQUIRED_VERSION_HEADER_NAME)
                .and_then(|version| SemanticVersion::from_str(version.to_str().ok()?).ok())
            {
                anyhow::ensure!(
                    app_version >= minimum_required_version,
                    ZedUpdateRequiredError {
                        minimum_version: minimum_required_version
                    }
                );
            }

            if response.status().is_success() {
                let usage = EditPredictionUsage::from_headers(response.headers()).ok();

                let mut body = String::new();
                response.body_mut().read_to_string(&mut body).await?;
                return Ok((serde_json::from_str(&body)?, usage));
            } else if !did_retry
                && response
                    .headers()
                    .get(EXPIRED_LLM_TOKEN_HEADER_NAME)
                    .is_some()
            {
                did_retry = true;
                token = llm_token.refresh(&client).await?;
            } else {
                let mut body = String::new();
                response.body_mut().read_to_string(&mut body).await?;
                anyhow::bail!(
                    "error predicting edits.\nStatus: {:?}\nBody: {}",
                    response.status(),
                    body
                );
            }
        }
    }
}

fn process_completion_response(
    prediction_response: PredictEditsResponse,
    buffer: Entity<Buffer>,
    snapshot: &BufferSnapshot,
    editable_range: Range<usize>,
    cursor_offset: usize,
    path: Arc<Path>,
    input_outline: String,
    input_events: String,
    input_excerpt: String,
    buffer_snapshotted_at: Instant,
    cx: &AsyncApp,
) -> Task<Result<Option<EditPrediction>>> {
    let snapshot = snapshot.clone();
    let request_id = prediction_response.request_id;
    let output_excerpt = prediction_response.output_excerpt;
    cx.spawn(async move |cx| {
        let output_excerpt: Arc<str> = output_excerpt.into();

        let edits: Arc<[(Range<Anchor>, Arc<str>)]> = cx
            .background_spawn({
                let output_excerpt = output_excerpt.clone();
                let editable_range = editable_range.clone();
                let snapshot = snapshot.clone();
                async move { parse_edits(output_excerpt, editable_range, &snapshot) }
            })
            .await?
            .into();

        let Some((edits, snapshot, edit_preview)) = buffer.read_with(cx, {
            let edits = edits.clone();
            move |buffer, cx| {
                let new_snapshot = buffer.snapshot();
                let edits: Arc<[(Range<Anchor>, Arc<str>)]> =
                    edit_prediction::interpolate_edits(&snapshot, &new_snapshot, &edits)?.into();
                Some((edits.clone(), new_snapshot, buffer.preview_edits(edits, cx)))
            }
        })?
        else {
            return anyhow::Ok(None);
        };

        let request_id = Uuid::from_str(&request_id).context("failed to parse request id")?;

        let edit_preview = edit_preview.await;

        Ok(Some(EditPrediction {
            id: EditPredictionId(request_id),
            edits,
            edit_preview,
            snapshot,
            buffer,
        }))
    })
}

fn parse_edits(
    output_excerpt: Arc<str>,
    editable_range: Range<usize>,
    snapshot: &BufferSnapshot,
) -> Result<Vec<(Range<Anchor>, Arc<str>)>> {
    let content = output_excerpt.replace(CURSOR_MARKER, "");

    let start_markers = content
        .match_indices(EDITABLE_REGION_START_MARKER)
        .collect::<Vec<_>>();
    anyhow::ensure!(
        start_markers.len() == 1,
        "expected exactly one start marker, found {}",
        start_markers.len()
    );

    let end_markers = content
        .match_indices(EDITABLE_REGION_END_MARKER)
        .collect::<Vec<_>>();
    anyhow::ensure!(
        end_markers.len() == 1,
        "expected exactly one end marker, found {}",
        end_markers.len()
    );

    let sof_markers = content
        .match_indices(START_OF_FILE_MARKER)
        .collect::<Vec<_>>();
    anyhow::ensure!(
        sof_markers.len() <= 1,
        "expected at most one start-of-file marker, found {}",
        sof_markers.len()
    );

    let codefence_start = start_markers[0].0;
    let content = &content[codefence_start..];

    let newline_ix = content.find('\n').context("could not find newline")?;
    let content = &content[newline_ix + 1..];

    let codefence_end = content
        .rfind(&format!("\n{EDITABLE_REGION_END_MARKER}"))
        .context("could not find end marker")?;
    let new_text = &content[..codefence_end];

    let old_text = snapshot
        .text_for_range(editable_range.clone())
        .collect::<String>();

    Ok(compute_edits(
        old_text,
        new_text,
        editable_range.start,
        snapshot,
    ))
}

pub fn compute_edits(
    old_text: String,
    new_text: &str,
    offset: usize,
    snapshot: &BufferSnapshot,
) -> Vec<(Range<Anchor>, Arc<str>)> {
    text_diff(&old_text, new_text)
        .into_iter()
        .map(|(mut old_range, new_text)| {
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

            let new_text = new_text[prefix_len..new_text.len() - suffix_len].into();
            let range = if old_range.is_empty() {
                let anchor = snapshot.anchor_after(old_range.start);
                anchor..anchor
            } else {
                snapshot.anchor_after(old_range.start)..snapshot.anchor_before(old_range.end)
            };
            (range, new_text)
        })
        .collect()
}

fn common_prefix<T1: Iterator<Item = char>, T2: Iterator<Item = char>>(a: T1, b: T2) -> usize {
    a.zip(b)
        .take_while(|(a, b)| a == b)
        .map(|(a, _)| a.len_utf8())
        .sum()
}

fn git_info_for_file(
    project: &Entity<Project>,
    project_path: &ProjectPath,
    cx: &App,
) -> Option<PredictEditsGitInfo> {
    let git_store = project.read(cx).git_store().read(cx);
    if let Some((repository, _repo_path)) =
        git_store.repository_and_path_for_project_path(project_path, cx)
    {
        let repository = repository.read(cx);
        let head_sha = repository
            .head_commit
            .as_ref()
            .map(|head_commit| head_commit.sha.to_string());
        let remote_origin_url = repository.remote_origin_url.clone();
        let remote_upstream_url = repository.remote_upstream_url.clone();
        if head_sha.is_none() && remote_origin_url.is_none() && remote_upstream_url.is_none() {
            return None;
        }
        Some(PredictEditsGitInfo {
            head_sha,
            remote_origin_url,
            remote_upstream_url,
        })
    } else {
        None
    }
}

pub struct GatherContextOutput {
    pub body: PredictEditsBody,
    pub editable_range: Range<usize>,
    pub included_events_count: usize,
}

pub fn gather_context(
    full_path_str: String,
    snapshot: &BufferSnapshot,
    cursor_point: language::Point,
    prompt_for_events: impl FnOnce() -> (String, usize) + Send + 'static,
    cx: &App,
) -> Task<Result<GatherContextOutput>> {
    cx.background_spawn({
        let snapshot = snapshot.clone();
        async move {
            let input_excerpt = excerpt_for_cursor_position(
                cursor_point,
                &full_path_str,
                &snapshot,
                MAX_REWRITE_TOKENS,
                MAX_CONTEXT_TOKENS,
            );
            let (input_events, included_events_count) = prompt_for_events();
            let editable_range = input_excerpt.editable_range.to_offset(&snapshot);

            let body = PredictEditsBody {
                input_events,
                input_excerpt: input_excerpt.prompt,
                can_collect_data: false,
                diagnostic_groups: None,
                git_info: None,
                outline: None,
                speculated_output: None,
            };

            Ok(GatherContextOutput {
                body,
                editable_range,
                included_events_count,
            })
        }
    })
}

fn prompt_for_events_impl(
    events: &[Event],
    mut remaining_tokens: usize,
    cx: &App,
) -> (String, usize) {
    let mut result = String::new();
    for (ix, event) in events.iter().rev().enumerate() {
        let Some(event) = event.to_request_event(cx) else {
            continue;
        };
        let event_string = event.to_string();
        let event_tokens = guess_token_count(event_string.len());
        if event_tokens > remaining_tokens {
            return (result, ix);
        }

        if !result.is_empty() {
            result.insert_str(0, "\n\n");
        }
        result.insert_str(0, &event_string);
        remaining_tokens -= event_tokens;
    }
    return (result, events.len());
}

/// Typical number of string bytes per token for the purposes of limiting model input. This is
/// intentionally low to err on the side of underestimating limits.
const BYTES_PER_TOKEN_GUESS: usize = 3;

fn guess_token_count(bytes: usize) -> usize {
    bytes / BYTES_PER_TOKEN_GUESS
}
