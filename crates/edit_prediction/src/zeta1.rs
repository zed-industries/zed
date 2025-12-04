mod input_excerpt;

use std::{fmt::Write, ops::Range, path::Path, sync::Arc, time::Instant};

use crate::{
    EditPredictionId, EditPredictionStore, ZedUpdateRequiredError,
    prediction::{EditPredictionInputs, EditPredictionResult},
};
use anyhow::{Context as _, Result};
use cloud_llm_client::{
    PredictEditsBody, PredictEditsGitInfo, PredictEditsRequestTrigger, PredictEditsResponse,
    predict_edits_v3::Event,
};
use gpui::{App, AppContext as _, AsyncApp, Context, Entity, SharedString, Task};
use input_excerpt::excerpt_for_cursor_position;
use language::{
    Anchor, Buffer, BufferSnapshot, OffsetRangeExt as _, Point, ToPoint as _, text_diff,
};
use project::{Project, ProjectPath};
use release_channel::AppVersion;
use workspace::notifications::{ErrorMessagePrompt, NotificationId, show_app_notification};

const CURSOR_MARKER: &str = "<|user_cursor_is_here|>";
const START_OF_FILE_MARKER: &str = "<|start_of_file|>";
const EDITABLE_REGION_START_MARKER: &str = "<|editable_region_start|>";
const EDITABLE_REGION_END_MARKER: &str = "<|editable_region_end|>";

pub(crate) const MAX_CONTEXT_TOKENS: usize = 150;
pub(crate) const MAX_REWRITE_TOKENS: usize = 350;
pub(crate) const MAX_EVENT_TOKENS: usize = 500;

pub(crate) fn request_prediction_with_zeta1(
    store: &mut EditPredictionStore,
    project: &Entity<Project>,
    buffer: &Entity<Buffer>,
    snapshot: BufferSnapshot,
    position: language::Anchor,
    events: Vec<Arc<Event>>,
    trigger: PredictEditsRequestTrigger,
    cx: &mut Context<EditPredictionStore>,
) -> Task<Result<Option<EditPredictionResult>>> {
    let buffer = buffer.clone();
    let buffer_snapshotted_at = Instant::now();
    let client = store.client.clone();
    let llm_token = store.llm_token.clone();
    let app_version = AppVersion::global(cx);

    let (git_info, can_collect_file) = if let Some(file) = snapshot.file() {
        let can_collect_file = store.can_collect_file(project, file, cx);
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
    let prompt_for_events = {
        let events = events.clone();
        move || prompt_for_events_impl(&events, MAX_EVENT_TOKENS)
    };
    let gather_task = gather_context(
        full_path_str,
        &snapshot,
        cursor_point,
        prompt_for_events,
        trigger,
        cx,
    );

    cx.spawn(async move |this, cx| {
        let GatherContextOutput {
            mut body,
            context_range,
            editable_range,
            included_events_count,
        } = gather_task.await?;
        let done_gathering_context_at = Instant::now();

        let included_events = &events[events.len() - included_events_count..events.len()];
        body.can_collect_data = can_collect_file
            && this
                .read_with(cx, |this, _| this.can_collect_events(included_events))
                .unwrap_or(false);
        if body.can_collect_data {
            body.git_info = git_info;
        }

        log::debug!(
            "Events:\n{}\nExcerpt:\n{:?}",
            body.input_events,
            body.input_excerpt
        );

        let http_client = client.http_client();

        let response = EditPredictionStore::send_api_request::<PredictEditsResponse>(
            |request| {
                let uri = if let Ok(predict_edits_url) = std::env::var("ZED_PREDICT_EDITS_URL") {
                    predict_edits_url
                } else {
                    http_client
                        .build_zed_llm_url("/predict_edits/v2", &[])?
                        .as_str()
                        .into()
                };
                Ok(request
                    .uri(uri)
                    .body(serde_json::to_string(&body)?.into())?)
            },
            client,
            llm_token,
            app_version,
        )
        .await;

        let inputs = EditPredictionInputs {
            events: included_events.into(),
            included_files: vec![cloud_llm_client::predict_edits_v3::RelatedFile {
                path: full_path.clone(),
                max_row: cloud_llm_client::predict_edits_v3::Line(snapshot.max_point().row),
                excerpts: vec![cloud_llm_client::predict_edits_v3::Excerpt {
                    start_line: cloud_llm_client::predict_edits_v3::Line(context_range.start.row),
                    text: snapshot
                        .text_for_range(context_range)
                        .collect::<String>()
                        .into(),
                }],
            }],
            cursor_point: cloud_llm_client::predict_edits_v3::Point {
                column: cursor_point.column,
                line: cloud_llm_client::predict_edits_v3::Line(cursor_point.row),
            },
            cursor_path: full_path,
        };

        // let response = perform_predict_edits(PerformPredictEditsParams {
        //     client,
        //     llm_token,
        //     app_version,
        //     body,
        // })
        // .await;

        let (response, usage) = match response {
            Ok(response) => response,
            Err(err) => {
                if err.is::<ZedUpdateRequiredError>() {
                    cx.update(|cx| {
                        this.update(cx, |ep_store, _cx| {
                            ep_store.update_required = true;
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
            inputs,
            buffer_snapshotted_at,
            received_response_at,
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

        edit_prediction.map(Some)
    })
}

fn process_completion_response(
    prediction_response: PredictEditsResponse,
    buffer: Entity<Buffer>,
    snapshot: &BufferSnapshot,
    editable_range: Range<usize>,
    inputs: EditPredictionInputs,
    buffer_snapshotted_at: Instant,
    received_response_at: Instant,
    cx: &AsyncApp,
) -> Task<Result<EditPredictionResult>> {
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

        let id = EditPredictionId(request_id.into());
        Ok(EditPredictionResult::new(
            id,
            &buffer,
            &snapshot,
            edits,
            buffer_snapshotted_at,
            received_response_at,
            inputs,
            cx,
        )
        .await)
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
    pub context_range: Range<Point>,
    pub editable_range: Range<usize>,
    pub included_events_count: usize,
}

pub fn gather_context(
    full_path_str: String,
    snapshot: &BufferSnapshot,
    cursor_point: language::Point,
    prompt_for_events: impl FnOnce() -> (String, usize) + Send + 'static,
    trigger: PredictEditsRequestTrigger,
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
                trigger,
            };

            Ok(GatherContextOutput {
                body,
                context_range: input_excerpt.context_range,
                editable_range,
                included_events_count,
            })
        }
    })
}

fn prompt_for_events_impl(events: &[Arc<Event>], mut remaining_tokens: usize) -> (String, usize) {
    let mut result = String::new();
    for (ix, event) in events.iter().rev().enumerate() {
        let event_string = format_event(event.as_ref());
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

pub fn format_event(event: &Event) -> String {
    match event {
        Event::BufferChange {
            path,
            old_path,
            diff,
            ..
        } => {
            let mut prompt = String::new();

            if old_path != path {
                writeln!(
                    prompt,
                    "User renamed {} to {}\n",
                    old_path.display(),
                    path.display()
                )
                .unwrap();
            }

            if !diff.is_empty() {
                write!(
                    prompt,
                    "User edited {}:\n```diff\n{}\n```",
                    path.display(),
                    diff
                )
                .unwrap();
            }

            prompt
        }
    }
}

/// Typical number of string bytes per token for the purposes of limiting model input. This is
/// intentionally low to err on the side of underestimating limits.
pub(crate) const BYTES_PER_TOKEN_GUESS: usize = 3;

fn guess_token_count(bytes: usize) -> usize {
    bytes / BYTES_PER_TOKEN_GUESS
}
