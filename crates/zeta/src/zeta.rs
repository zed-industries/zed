mod completion_diff_element;
mod init;
mod input_excerpt;
mod license_detection;
mod onboarding_modal;
mod onboarding_telemetry;
mod rate_completion_modal;

pub(crate) use completion_diff_element::*;
use db::kvp::{Dismissable, KEY_VALUE_STORE};
use edit_prediction::DataCollectionState;
pub use init::*;
use license_detection::LicenseDetectionWatcher;
pub use rate_completion_modal::*;

use anyhow::{Context as _, Result, anyhow};
use arrayvec::ArrayVec;
use client::{Client, EditPredictionUsage, UserStore};
use cloud_llm_client::{
    AcceptEditPredictionBody, EXPIRED_LLM_TOKEN_HEADER_NAME, MINIMUM_REQUIRED_VERSION_HEADER_NAME,
    PredictEditsBody, PredictEditsGitInfo, PredictEditsResponse, ZED_VERSION_HEADER_NAME,
};
use collections::{HashMap, HashSet, VecDeque};
use futures::AsyncReadExt;
use gpui::{
    App, AppContext as _, AsyncApp, Context, Entity, EntityId, Global, SemanticVersion,
    SharedString, Subscription, Task, actions,
};
use http_client::{AsyncBody, HttpClient, Method, Request, Response};
use input_excerpt::excerpt_for_cursor_position;
use language::{
    Anchor, Buffer, BufferSnapshot, EditPreview, File, OffsetRangeExt, ToOffset, ToPoint, text_diff,
};
use language_model::{LlmApiToken, RefreshLlmTokenListener};
use project::{Project, ProjectPath};
use release_channel::AppVersion;
use settings::WorktreeId;
use std::collections::hash_map;
use std::mem;
use std::str::FromStr;
use std::{
    cmp,
    fmt::Write,
    future::Future,
    ops::Range,
    path::Path,
    rc::Rc,
    sync::Arc,
    time::{Duration, Instant},
};
use telemetry_events::EditPredictionRating;
use thiserror::Error;
use util::ResultExt;
use util::rel_path::RelPath;
use uuid::Uuid;
use workspace::notifications::{ErrorMessagePrompt, NotificationId, show_app_notification};
use worktree::Worktree;

const CURSOR_MARKER: &str = "<|user_cursor_is_here|>";
const START_OF_FILE_MARKER: &str = "<|start_of_file|>";
const EDITABLE_REGION_START_MARKER: &str = "<|editable_region_start|>";
const EDITABLE_REGION_END_MARKER: &str = "<|editable_region_end|>";
const BUFFER_CHANGE_GROUPING_INTERVAL: Duration = Duration::from_secs(1);
const ZED_PREDICT_DATA_COLLECTION_CHOICE: &str = "zed_predict_data_collection_choice";

const MAX_CONTEXT_TOKENS: usize = 150;
const MAX_REWRITE_TOKENS: usize = 350;
const MAX_EVENT_TOKENS: usize = 500;

/// Maximum number of events to track.
const MAX_EVENT_COUNT: usize = 16;

actions!(
    edit_prediction,
    [
        /// Clears the edit prediction history.
        ClearHistory
    ]
);

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

struct ZedPredictUpsell;

impl Dismissable for ZedPredictUpsell {
    const KEY: &'static str = "dismissed-edit-predict-upsell";

    fn dismissed() -> bool {
        // To make this backwards compatible with older versions of Zed, we
        // check if the user has seen the previous Edit Prediction Onboarding
        // before, by checking the data collection choice which was written to
        // the database once the user clicked on "Accept and Enable"
        if KEY_VALUE_STORE
            .read_kvp(ZED_PREDICT_DATA_COLLECTION_CHOICE)
            .log_err()
            .is_some_and(|s| s.is_some())
        {
            return true;
        }

        KEY_VALUE_STORE
            .read_kvp(Self::KEY)
            .log_err()
            .is_some_and(|s| s.is_some())
    }
}

pub fn should_show_upsell_modal() -> bool {
    !ZedPredictUpsell::dismissed()
}

#[derive(Clone)]
struct ZetaGlobal(Entity<Zeta>);

impl Global for ZetaGlobal {}

#[derive(Clone)]
pub struct EditPrediction {
    id: EditPredictionId,
    path: Arc<Path>,
    excerpt_range: Range<usize>,
    cursor_offset: usize,
    edits: Arc<[(Range<Anchor>, String)]>,
    snapshot: BufferSnapshot,
    edit_preview: EditPreview,
    input_outline: Arc<str>,
    input_events: Arc<str>,
    input_excerpt: Arc<str>,
    output_excerpt: Arc<str>,
    buffer_snapshotted_at: Instant,
    response_received_at: Instant,
}

impl EditPrediction {
    fn latency(&self) -> Duration {
        self.response_received_at
            .duration_since(self.buffer_snapshotted_at)
    }

    fn interpolate(&self, new_snapshot: &BufferSnapshot) -> Option<Vec<(Range<Anchor>, String)>> {
        interpolate(&self.snapshot, new_snapshot, self.edits.clone())
    }
}

fn interpolate(
    old_snapshot: &BufferSnapshot,
    new_snapshot: &BufferSnapshot,
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

impl std::fmt::Debug for EditPrediction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EditPrediction")
            .field("id", &self.id)
            .field("path", &self.path)
            .field("edits", &self.edits)
            .finish_non_exhaustive()
    }
}

pub struct Zeta {
    projects: HashMap<EntityId, ZetaProject>,
    client: Arc<Client>,
    shown_completions: VecDeque<EditPrediction>,
    rated_completions: HashSet<EditPredictionId>,
    data_collection_choice: DataCollectionChoice,
    llm_token: LlmApiToken,
    _llm_token_subscription: Subscription,
    /// Whether an update to a newer version of Zed is required to continue using Zeta.
    update_required: bool,
    user_store: Entity<UserStore>,
    license_detection_watchers: HashMap<WorktreeId, Rc<LicenseDetectionWatcher>>,
}

struct ZetaProject {
    events: VecDeque<Event>,
    registered_buffers: HashMap<gpui::EntityId, RegisteredBuffer>,
}

impl Zeta {
    pub fn global(cx: &mut App) -> Option<Entity<Self>> {
        cx.try_global::<ZetaGlobal>().map(|global| global.0.clone())
    }

    pub fn register(
        worktree: Option<Entity<Worktree>>,
        client: Arc<Client>,
        user_store: Entity<UserStore>,
        cx: &mut App,
    ) -> Entity<Self> {
        let this = Self::global(cx).unwrap_or_else(|| {
            let entity = cx.new(|cx| Self::new(client, user_store, cx));
            cx.set_global(ZetaGlobal(entity.clone()));
            entity
        });

        this.update(cx, move |this, cx| {
            if let Some(worktree) = worktree {
                let worktree_id = worktree.read(cx).id();
                this.license_detection_watchers
                    .entry(worktree_id)
                    .or_insert_with(|| Rc::new(LicenseDetectionWatcher::new(&worktree, cx)));
            }
        });

        this
    }

    pub fn clear_history(&mut self) {
        for zeta_project in self.projects.values_mut() {
            zeta_project.events.clear();
        }
    }

    pub fn usage(&self, cx: &App) -> Option<EditPredictionUsage> {
        self.user_store.read(cx).edit_prediction_usage()
    }

    fn new(client: Arc<Client>, user_store: Entity<UserStore>, cx: &mut Context<Self>) -> Self {
        let refresh_llm_token_listener = RefreshLlmTokenListener::global(cx);
        let data_collection_choice = Self::load_data_collection_choice();
        Self {
            projects: HashMap::default(),
            client,
            shown_completions: VecDeque::new(),
            rated_completions: HashSet::default(),
            data_collection_choice,
            llm_token: LlmApiToken::default(),
            _llm_token_subscription: cx.subscribe(
                &refresh_llm_token_listener,
                |this, _listener, _event, cx| {
                    let client = this.client.clone();
                    let llm_token = this.llm_token.clone();
                    cx.spawn(async move |_this, _cx| {
                        llm_token.refresh(&client).await?;
                        anyhow::Ok(())
                    })
                    .detach_and_log_err(cx);
                },
            ),
            update_required: false,
            license_detection_watchers: HashMap::default(),
            user_store,
        }
    }

    fn get_or_init_zeta_project(
        &mut self,
        project: &Entity<Project>,
        cx: &mut Context<Self>,
    ) -> &mut ZetaProject {
        let project_id = project.entity_id();
        match self.projects.entry(project_id) {
            hash_map::Entry::Occupied(entry) => entry.into_mut(),
            hash_map::Entry::Vacant(entry) => {
                cx.observe_release(project, move |this, _, _cx| {
                    this.projects.remove(&project_id);
                })
                .detach();
                entry.insert(ZetaProject {
                    events: VecDeque::with_capacity(MAX_EVENT_COUNT),
                    registered_buffers: HashMap::default(),
                })
            }
        }
    }

    fn push_event(zeta_project: &mut ZetaProject, event: Event) {
        let events = &mut zeta_project.events;

        if let Some(Event::BufferChange {
            new_snapshot: last_new_snapshot,
            timestamp: last_timestamp,
            ..
        }) = events.back_mut()
        {
            // Coalesce edits for the same buffer when they happen one after the other.
            let Event::BufferChange {
                old_snapshot,
                new_snapshot,
                timestamp,
            } = &event;

            if timestamp.duration_since(*last_timestamp) <= BUFFER_CHANGE_GROUPING_INTERVAL
                && old_snapshot.remote_id() == last_new_snapshot.remote_id()
                && old_snapshot.version == last_new_snapshot.version
            {
                *last_new_snapshot = new_snapshot.clone();
                *last_timestamp = *timestamp;
                return;
            }
        }

        if events.len() >= MAX_EVENT_COUNT {
            // These are halved instead of popping to improve prompt caching.
            events.drain(..MAX_EVENT_COUNT / 2);
        }

        events.push_back(event);
    }

    pub fn register_buffer(
        &mut self,
        buffer: &Entity<Buffer>,
        project: &Entity<Project>,
        cx: &mut Context<Self>,
    ) {
        let zeta_project = self.get_or_init_zeta_project(project, cx);
        Self::register_buffer_impl(zeta_project, buffer, project, cx);
    }

    fn register_buffer_impl<'a>(
        zeta_project: &'a mut ZetaProject,
        buffer: &Entity<Buffer>,
        project: &Entity<Project>,
        cx: &mut Context<Self>,
    ) -> &'a mut RegisteredBuffer {
        let buffer_id = buffer.entity_id();
        match zeta_project.registered_buffers.entry(buffer_id) {
            hash_map::Entry::Occupied(entry) => entry.into_mut(),
            hash_map::Entry::Vacant(entry) => {
                let snapshot = buffer.read(cx).snapshot();
                let project_entity_id = project.entity_id();
                entry.insert(RegisteredBuffer {
                    snapshot,
                    _subscriptions: [
                        cx.subscribe(buffer, {
                            let project = project.downgrade();
                            move |this, buffer, event, cx| {
                                if let language::BufferEvent::Edited = event
                                    && let Some(project) = project.upgrade()
                                {
                                    this.report_changes_for_buffer(&buffer, &project, cx);
                                }
                            }
                        }),
                        cx.observe_release(buffer, move |this, _buffer, _cx| {
                            let Some(zeta_project) = this.projects.get_mut(&project_entity_id)
                            else {
                                return;
                            };
                            zeta_project.registered_buffers.remove(&buffer_id);
                        }),
                    ],
                })
            }
        }
    }

    fn request_completion_impl<F, R>(
        &mut self,
        project: &Entity<Project>,
        buffer: &Entity<Buffer>,
        cursor: language::Anchor,
        cx: &mut Context<Self>,
        perform_predict_edits: F,
    ) -> Task<Result<Option<EditPrediction>>>
    where
        F: FnOnce(PerformPredictEditsParams) -> R + 'static,
        R: Future<Output = Result<(PredictEditsResponse, Option<EditPredictionUsage>)>>
            + Send
            + 'static,
    {
        let buffer = buffer.clone();
        let buffer_snapshotted_at = Instant::now();
        let snapshot = self.report_changes_for_buffer(&buffer, project, cx);
        let zeta = cx.entity();
        let client = self.client.clone();
        let llm_token = self.llm_token.clone();
        let app_version = AppVersion::global(cx);

        let zeta_project = self.get_or_init_zeta_project(project, cx);
        let mut events = Vec::with_capacity(zeta_project.events.len());
        events.extend(zeta_project.events.iter().cloned());
        let events = Arc::new(events);

        let (git_info, can_collect_file) = if let Some(file) = snapshot.file() {
            let can_collect_file = self.can_collect_file(file, cx);
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
        let full_path_str = full_path.to_string_lossy().to_string();
        let cursor_point = cursor.to_point(&snapshot);
        let cursor_offset = cursor_point.to_offset(&snapshot);
        let prompt_for_events = {
            let events = events.clone();
            move || prompt_for_events_impl(&events, MAX_EVENT_TOKENS)
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
                            zeta.update(cx, |zeta, _cx| {
                                zeta.update_required = true;
                            });

                            let error_message: SharedString = err.to_string().into();
                            show_app_notification(
                                NotificationId::unique::<ZedUpdateRequiredError>(),
                                cx,
                                move |cx| {
                                    cx.new(|cx| {
                                        ErrorMessagePrompt::new(error_message.clone(), cx)
                                            .with_link_button(
                                                "Update Zed",
                                                "https://zed.dev/releases",
                                            )
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

            let edit_prediction = Self::process_completion_response(
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

    #[cfg(any(test, feature = "test-support"))]
    pub fn fake_completion(
        &mut self,
        project: &Entity<Project>,
        buffer: &Entity<Buffer>,
        position: language::Anchor,
        response: PredictEditsResponse,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<EditPrediction>>> {
        self.request_completion_impl(project, buffer, position, cx, |_params| {
            std::future::ready(Ok((response, None)))
        })
    }

    pub fn request_completion(
        &mut self,
        project: &Entity<Project>,
        buffer: &Entity<Buffer>,
        position: language::Anchor,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<EditPrediction>>> {
        self.request_completion_impl(project, buffer, position, cx, Self::perform_predict_edits)
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

    fn accept_edit_prediction(
        &mut self,
        request_id: EditPredictionId,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let client = self.client.clone();
        let llm_token = self.llm_token.clone();
        let app_version = AppVersion::global(cx);
        cx.spawn(async move |this, cx| {
            let http_client = client.http_client();
            let mut response = llm_token_retry(&llm_token, &client, |token| {
                let request_builder = http_client::Request::builder().method(Method::POST);
                let request_builder =
                    if let Ok(accept_prediction_url) = std::env::var("ZED_ACCEPT_PREDICTION_URL") {
                        request_builder.uri(accept_prediction_url)
                    } else {
                        request_builder.uri(
                            http_client
                                .build_zed_llm_url("/predict_edits/accept", &[])?
                                .as_ref(),
                        )
                    };
                Ok(request_builder
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {}", token))
                    .header(ZED_VERSION_HEADER_NAME, app_version.to_string())
                    .body(
                        serde_json::to_string(&AcceptEditPredictionBody {
                            request_id: request_id.0,
                        })?
                        .into(),
                    )?)
            })
            .await?;

            if let Some(minimum_required_version) = response
                .headers()
                .get(MINIMUM_REQUIRED_VERSION_HEADER_NAME)
                .and_then(|version| SemanticVersion::from_str(version.to_str().ok()?).ok())
                && app_version < minimum_required_version
            {
                return Err(anyhow!(ZedUpdateRequiredError {
                    minimum_version: minimum_required_version
                }));
            }

            if response.status().is_success() {
                if let Some(usage) = EditPredictionUsage::from_headers(response.headers()).ok() {
                    this.update(cx, |this, cx| {
                        this.user_store.update(cx, |user_store, cx| {
                            user_store.update_edit_prediction_usage(usage, cx);
                        });
                    })?;
                }

                Ok(())
            } else {
                let mut body = String::new();
                response.body_mut().read_to_string(&mut body).await?;
                Err(anyhow!(
                    "error accepting edit prediction.\nStatus: {:?}\nBody: {}",
                    response.status(),
                    body
                ))
            }
        })
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

            let edits: Arc<[(Range<Anchor>, String)]> = cx
                .background_spawn({
                    let output_excerpt = output_excerpt.clone();
                    let editable_range = editable_range.clone();
                    let snapshot = snapshot.clone();
                    async move { Self::parse_edits(output_excerpt, editable_range, &snapshot) }
                })
                .await?
                .into();

            let Some((edits, snapshot, edit_preview)) = buffer.read_with(cx, {
                let edits = edits.clone();
                |buffer, cx| {
                    let new_snapshot = buffer.snapshot();
                    let edits: Arc<[(Range<Anchor>, String)]> =
                        interpolate(&snapshot, &new_snapshot, edits)?.into();
                    Some((edits.clone(), new_snapshot, buffer.preview_edits(edits, cx)))
                }
            })?
            else {
                return anyhow::Ok(None);
            };

            let edit_preview = edit_preview.await;

            Ok(Some(EditPrediction {
                id: EditPredictionId(request_id),
                path,
                excerpt_range: editable_range,
                cursor_offset,
                edits,
                edit_preview,
                snapshot,
                input_outline: input_outline.into(),
                input_events: input_events.into(),
                input_excerpt: input_excerpt.into(),
                output_excerpt,
                buffer_snapshotted_at,
                response_received_at: Instant::now(),
            }))
        })
    }

    fn parse_edits(
        output_excerpt: Arc<str>,
        editable_range: Range<usize>,
        snapshot: &BufferSnapshot,
    ) -> Result<Vec<(Range<Anchor>, String)>> {
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

        Ok(Self::compute_edits(
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
    ) -> Vec<(Range<Anchor>, String)> {
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

                let new_text = new_text[prefix_len..new_text.len() - suffix_len].to_string();
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

    pub fn is_completion_rated(&self, completion_id: EditPredictionId) -> bool {
        self.rated_completions.contains(&completion_id)
    }

    pub fn completion_shown(&mut self, completion: &EditPrediction, cx: &mut Context<Self>) {
        self.shown_completions.push_front(completion.clone());
        if self.shown_completions.len() > 50 {
            let completion = self.shown_completions.pop_back().unwrap();
            self.rated_completions.remove(&completion.id);
        }
        cx.notify();
    }

    pub fn rate_completion(
        &mut self,
        completion: &EditPrediction,
        rating: EditPredictionRating,
        feedback: String,
        cx: &mut Context<Self>,
    ) {
        self.rated_completions.insert(completion.id);
        telemetry::event!(
            "Edit Prediction Rated",
            rating,
            input_events = completion.input_events,
            input_excerpt = completion.input_excerpt,
            input_outline = completion.input_outline,
            output_excerpt = completion.output_excerpt,
            feedback
        );
        self.client.telemetry().flush_events().detach();
        cx.notify();
    }

    pub fn shown_completions(&self) -> impl DoubleEndedIterator<Item = &EditPrediction> {
        self.shown_completions.iter()
    }

    pub fn shown_completions_len(&self) -> usize {
        self.shown_completions.len()
    }

    fn report_changes_for_buffer(
        &mut self,
        buffer: &Entity<Buffer>,
        project: &Entity<Project>,
        cx: &mut Context<Self>,
    ) -> BufferSnapshot {
        let zeta_project = self.get_or_init_zeta_project(project, cx);
        let registered_buffer = Self::register_buffer_impl(zeta_project, buffer, project, cx);

        let new_snapshot = buffer.read(cx).snapshot();
        if new_snapshot.version != registered_buffer.snapshot.version {
            let old_snapshot = mem::replace(&mut registered_buffer.snapshot, new_snapshot.clone());
            Self::push_event(
                zeta_project,
                Event::BufferChange {
                    old_snapshot,
                    new_snapshot: new_snapshot.clone(),
                    timestamp: Instant::now(),
                },
            );
        }

        new_snapshot
    }

    fn can_collect_file(&self, file: &Arc<dyn File>, cx: &App) -> bool {
        self.data_collection_choice.is_enabled() && self.is_file_open_source(file, cx)
    }

    fn can_collect_events(&self, events: &[Event], cx: &App) -> bool {
        if !self.data_collection_choice.is_enabled() {
            return false;
        }
        let mut last_checked_file = None;
        for event in events {
            match event {
                Event::BufferChange {
                    old_snapshot,
                    new_snapshot,
                    ..
                } => {
                    if let Some(old_file) = old_snapshot.file()
                        && let Some(new_file) = new_snapshot.file()
                    {
                        if let Some(last_checked_file) = last_checked_file
                            && Arc::ptr_eq(last_checked_file, old_file)
                            && Arc::ptr_eq(last_checked_file, new_file)
                        {
                            continue;
                        }
                        if !self.can_collect_file(old_file, cx) {
                            return false;
                        }
                        if !Arc::ptr_eq(old_file, new_file) && !self.can_collect_file(new_file, cx)
                        {
                            return false;
                        }
                        last_checked_file = Some(new_file);
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn is_file_open_source(&self, file: &Arc<dyn File>, cx: &App) -> bool {
        if !file.is_local() || file.is_private() {
            return false;
        }
        self.license_detection_watchers
            .get(&file.worktree_id(cx))
            .is_some_and(|watcher| watcher.is_project_open_source())
    }

    fn load_data_collection_choice() -> DataCollectionChoice {
        let choice = KEY_VALUE_STORE
            .read_kvp(ZED_PREDICT_DATA_COLLECTION_CHOICE)
            .log_err()
            .flatten();

        match choice.as_deref() {
            Some("true") => DataCollectionChoice::Enabled,
            Some("false") => DataCollectionChoice::Disabled,
            Some(_) => {
                log::error!("unknown value in '{ZED_PREDICT_DATA_COLLECTION_CHOICE}'");
                DataCollectionChoice::NotAnswered
            }
            None => DataCollectionChoice::NotAnswered,
        }
    }

    fn toggle_data_collection_choice(&mut self, cx: &mut Context<Self>) {
        self.data_collection_choice = self.data_collection_choice.toggle();
        let new_choice = self.data_collection_choice;
        db::write_and_log(cx, move || {
            KEY_VALUE_STORE.write_kvp(
                ZED_PREDICT_DATA_COLLECTION_CHOICE.into(),
                new_choice.is_enabled().to_string(),
            )
        });
    }
}

pub struct PerformPredictEditsParams {
    pub client: Arc<Client>,
    pub llm_token: LlmApiToken,
    pub app_version: SemanticVersion,
    pub body: PredictEditsBody,
}

#[derive(Error, Debug)]
#[error(
    "You must update to Zed version {minimum_version} or higher to continue using edit predictions."
)]
pub struct ZedUpdateRequiredError {
    minimum_version: SemanticVersion,
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

fn prompt_for_events_impl(events: &[Event], mut remaining_tokens: usize) -> (String, usize) {
    let mut result = String::new();
    for (ix, event) in events.iter().rev().enumerate() {
        let event_string = event.to_prompt();
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

struct RegisteredBuffer {
    snapshot: BufferSnapshot,
    _subscriptions: [gpui::Subscription; 2],
}

#[derive(Clone)]
pub enum Event {
    BufferChange {
        old_snapshot: BufferSnapshot,
        new_snapshot: BufferSnapshot,
        timestamp: Instant,
    },
}

impl Event {
    fn to_prompt(&self) -> String {
        match self {
            Event::BufferChange {
                old_snapshot,
                new_snapshot,
                ..
            } => {
                let mut prompt = String::new();

                let old_path = old_snapshot
                    .file()
                    .map(|f| f.path().as_ref())
                    .unwrap_or(RelPath::new("untitled").unwrap());
                let new_path = new_snapshot
                    .file()
                    .map(|f| f.path().as_ref())
                    .unwrap_or(RelPath::new("untitled").unwrap());
                if old_path != new_path {
                    writeln!(prompt, "User renamed {:?} to {:?}\n", old_path, new_path).unwrap();
                }

                let diff = language::unified_diff(&old_snapshot.text(), &new_snapshot.text());
                if !diff.is_empty() {
                    write!(
                        prompt,
                        "User edited {:?}:\n```diff\n{}\n```",
                        new_path, diff
                    )
                    .unwrap();
                }

                prompt
            }
        }
    }
}

#[derive(Debug, Clone)]
struct CurrentEditPrediction {
    buffer_id: EntityId,
    completion: EditPrediction,
}

impl CurrentEditPrediction {
    fn should_replace_completion(&self, old_completion: &Self, snapshot: &BufferSnapshot) -> bool {
        if self.buffer_id != old_completion.buffer_id {
            return true;
        }

        let Some(old_edits) = old_completion.completion.interpolate(snapshot) else {
            return true;
        };
        let Some(new_edits) = self.completion.interpolate(snapshot) else {
            return false;
        };

        if old_edits.len() == 1 && new_edits.len() == 1 {
            let (old_range, old_text) = &old_edits[0];
            let (new_range, new_text) = &new_edits[0];
            new_range == old_range && new_text.starts_with(old_text)
        } else {
            true
        }
    }
}

struct PendingCompletion {
    id: usize,
    _task: Task<()>,
}

#[derive(Debug, Clone, Copy)]
pub enum DataCollectionChoice {
    NotAnswered,
    Enabled,
    Disabled,
}

impl DataCollectionChoice {
    pub fn is_enabled(self) -> bool {
        match self {
            Self::Enabled => true,
            Self::NotAnswered | Self::Disabled => false,
        }
    }

    pub fn is_answered(self) -> bool {
        match self {
            Self::Enabled | Self::Disabled => true,
            Self::NotAnswered => false,
        }
    }

    #[must_use]
    pub fn toggle(&self) -> DataCollectionChoice {
        match self {
            Self::Enabled => Self::Disabled,
            Self::Disabled => Self::Enabled,
            Self::NotAnswered => Self::Enabled,
        }
    }
}

impl From<bool> for DataCollectionChoice {
    fn from(value: bool) -> Self {
        match value {
            true => DataCollectionChoice::Enabled,
            false => DataCollectionChoice::Disabled,
        }
    }
}

async fn llm_token_retry(
    llm_token: &LlmApiToken,
    client: &Arc<Client>,
    build_request: impl Fn(String) -> Result<Request<AsyncBody>>,
) -> Result<Response<AsyncBody>> {
    let mut did_retry = false;
    let http_client = client.http_client();
    let mut token = llm_token.acquire(client).await?;
    loop {
        let request = build_request(token.clone())?;
        let response = http_client.send(request).await?;

        if !did_retry
            && !response.status().is_success()
            && response
                .headers()
                .get(EXPIRED_LLM_TOKEN_HEADER_NAME)
                .is_some()
        {
            did_retry = true;
            token = llm_token.refresh(client).await?;
            continue;
        }

        return Ok(response);
    }
}

pub struct ZetaEditPredictionProvider {
    zeta: Entity<Zeta>,
    singleton_buffer: Option<Entity<Buffer>>,
    pending_completions: ArrayVec<PendingCompletion, 2>,
    next_pending_completion_id: usize,
    current_completion: Option<CurrentEditPrediction>,
    last_request_timestamp: Instant,
}

impl ZetaEditPredictionProvider {
    pub const THROTTLE_TIMEOUT: Duration = Duration::from_millis(300);

    pub fn new(zeta: Entity<Zeta>, singleton_buffer: Option<Entity<Buffer>>) -> Self {
        Self {
            zeta,
            singleton_buffer,
            pending_completions: ArrayVec::new(),
            next_pending_completion_id: 0,
            current_completion: None,
            last_request_timestamp: Instant::now(),
        }
    }
}

impl edit_prediction::EditPredictionProvider for ZetaEditPredictionProvider {
    fn name() -> &'static str {
        "zed-predict"
    }

    fn display_name() -> &'static str {
        "Zed's Edit Predictions"
    }

    fn show_completions_in_menu() -> bool {
        true
    }

    fn show_tab_accept_marker() -> bool {
        true
    }

    fn data_collection_state(&self, cx: &App) -> DataCollectionState {
        if let Some(buffer) = &self.singleton_buffer
            && let Some(file) = buffer.read(cx).file()
        {
            let is_project_open_source = self.zeta.read(cx).is_file_open_source(file, cx);
            if self.zeta.read(cx).data_collection_choice.is_enabled() {
                DataCollectionState::Enabled {
                    is_project_open_source,
                }
            } else {
                DataCollectionState::Disabled {
                    is_project_open_source,
                }
            }
        } else {
            return DataCollectionState::Disabled {
                is_project_open_source: false,
            };
        }
    }

    fn toggle_data_collection(&mut self, cx: &mut App) {
        self.zeta
            .update(cx, |zeta, cx| zeta.toggle_data_collection_choice(cx));
    }

    fn usage(&self, cx: &App) -> Option<EditPredictionUsage> {
        self.zeta.read(cx).usage(cx)
    }

    fn is_enabled(
        &self,
        _buffer: &Entity<Buffer>,
        _cursor_position: language::Anchor,
        _cx: &App,
    ) -> bool {
        true
    }
    fn is_refreshing(&self) -> bool {
        !self.pending_completions.is_empty()
    }

    fn refresh(
        &mut self,
        project: Option<Entity<Project>>,
        buffer: Entity<Buffer>,
        position: language::Anchor,
        _debounce: bool,
        cx: &mut Context<Self>,
    ) {
        if self.zeta.read(cx).update_required {
            return;
        }
        let Some(project) = project else {
            return;
        };

        if self
            .zeta
            .read(cx)
            .user_store
            .read_with(cx, |user_store, _cx| {
                user_store.account_too_young() || user_store.has_overdue_invoices()
            })
        {
            return;
        }

        if let Some(current_completion) = self.current_completion.as_ref() {
            let snapshot = buffer.read(cx).snapshot();
            if current_completion
                .completion
                .interpolate(&snapshot)
                .is_some()
            {
                return;
            }
        }

        let pending_completion_id = self.next_pending_completion_id;
        self.next_pending_completion_id += 1;
        let last_request_timestamp = self.last_request_timestamp;

        let task = cx.spawn(async move |this, cx| {
            if let Some(timeout) = (last_request_timestamp + Self::THROTTLE_TIMEOUT)
                .checked_duration_since(Instant::now())
            {
                cx.background_executor().timer(timeout).await;
            }

            let completion_request = this.update(cx, |this, cx| {
                this.last_request_timestamp = Instant::now();
                this.zeta.update(cx, |zeta, cx| {
                    zeta.request_completion(&project, &buffer, position, cx)
                })
            });

            let completion = match completion_request {
                Ok(completion_request) => {
                    let completion_request = completion_request.await;
                    completion_request.map(|c| {
                        c.map(|completion| CurrentEditPrediction {
                            buffer_id: buffer.entity_id(),
                            completion,
                        })
                    })
                }
                Err(error) => Err(error),
            };
            let Some(new_completion) = completion
                .context("edit prediction failed")
                .log_err()
                .flatten()
            else {
                this.update(cx, |this, cx| {
                    if this.pending_completions[0].id == pending_completion_id {
                        this.pending_completions.remove(0);
                    } else {
                        this.pending_completions.clear();
                    }

                    cx.notify();
                })
                .ok();
                return;
            };

            this.update(cx, |this, cx| {
                if this.pending_completions[0].id == pending_completion_id {
                    this.pending_completions.remove(0);
                } else {
                    this.pending_completions.clear();
                }

                if let Some(old_completion) = this.current_completion.as_ref() {
                    let snapshot = buffer.read(cx).snapshot();
                    if new_completion.should_replace_completion(old_completion, &snapshot) {
                        this.zeta.update(cx, |zeta, cx| {
                            zeta.completion_shown(&new_completion.completion, cx);
                        });
                        this.current_completion = Some(new_completion);
                    }
                } else {
                    this.zeta.update(cx, |zeta, cx| {
                        zeta.completion_shown(&new_completion.completion, cx);
                    });
                    this.current_completion = Some(new_completion);
                }

                cx.notify();
            })
            .ok();
        });

        // We always maintain at most two pending completions. When we already
        // have two, we replace the newest one.
        if self.pending_completions.len() <= 1 {
            self.pending_completions.push(PendingCompletion {
                id: pending_completion_id,
                _task: task,
            });
        } else if self.pending_completions.len() == 2 {
            self.pending_completions.pop();
            self.pending_completions.push(PendingCompletion {
                id: pending_completion_id,
                _task: task,
            });
        }
    }

    fn cycle(
        &mut self,
        _buffer: Entity<Buffer>,
        _cursor_position: language::Anchor,
        _direction: edit_prediction::Direction,
        _cx: &mut Context<Self>,
    ) {
        // Right now we don't support cycling.
    }

    fn accept(&mut self, cx: &mut Context<Self>) {
        let completion_id = self
            .current_completion
            .as_ref()
            .map(|completion| completion.completion.id);
        if let Some(completion_id) = completion_id {
            self.zeta
                .update(cx, |zeta, cx| {
                    zeta.accept_edit_prediction(completion_id, cx)
                })
                .detach();
        }
        self.pending_completions.clear();
    }

    fn discard(&mut self, _cx: &mut Context<Self>) {
        self.pending_completions.clear();
        self.current_completion.take();
    }

    fn suggest(
        &mut self,
        buffer: &Entity<Buffer>,
        cursor_position: language::Anchor,
        cx: &mut Context<Self>,
    ) -> Option<edit_prediction::EditPrediction> {
        let CurrentEditPrediction {
            buffer_id,
            completion,
            ..
        } = self.current_completion.as_mut()?;

        // Invalidate previous completion if it was generated for a different buffer.
        if *buffer_id != buffer.entity_id() {
            self.current_completion.take();
            return None;
        }

        let buffer = buffer.read(cx);
        let Some(edits) = completion.interpolate(&buffer.snapshot()) else {
            self.current_completion.take();
            return None;
        };

        let cursor_row = cursor_position.to_point(buffer).row;
        let (closest_edit_ix, (closest_edit_range, _)) =
            edits.iter().enumerate().min_by_key(|(_, (range, _))| {
                let distance_from_start = cursor_row.abs_diff(range.start.to_point(buffer).row);
                let distance_from_end = cursor_row.abs_diff(range.end.to_point(buffer).row);
                cmp::min(distance_from_start, distance_from_end)
            })?;

        let mut edit_start_ix = closest_edit_ix;
        for (range, _) in edits[..edit_start_ix].iter().rev() {
            let distance_from_closest_edit =
                closest_edit_range.start.to_point(buffer).row - range.end.to_point(buffer).row;
            if distance_from_closest_edit <= 1 {
                edit_start_ix -= 1;
            } else {
                break;
            }
        }

        let mut edit_end_ix = closest_edit_ix + 1;
        for (range, _) in &edits[edit_end_ix..] {
            let distance_from_closest_edit =
                range.start.to_point(buffer).row - closest_edit_range.end.to_point(buffer).row;
            if distance_from_closest_edit <= 1 {
                edit_end_ix += 1;
            } else {
                break;
            }
        }

        Some(edit_prediction::EditPrediction {
            id: Some(completion.id.to_string().into()),
            edits: edits[edit_start_ix..edit_end_ix].to_vec(),
            edit_preview: Some(completion.edit_preview.clone()),
        })
    }
}

/// Typical number of string bytes per token for the purposes of limiting model input. This is
/// intentionally low to err on the side of underestimating limits.
const BYTES_PER_TOKEN_GUESS: usize = 3;

fn guess_token_count(bytes: usize) -> usize {
    bytes / BYTES_PER_TOKEN_GUESS
}

#[cfg(test)]
mod tests {
    use client::test::FakeServer;
    use clock::FakeSystemClock;
    use cloud_api_types::{CreateLlmTokenResponse, LlmToken};
    use gpui::TestAppContext;
    use http_client::FakeHttpClient;
    use indoc::indoc;
    use language::Point;
    use parking_lot::Mutex;
    use serde_json::json;
    use settings::SettingsStore;
    use util::{path, rel_path::rel_path};

    use super::*;

    const BSD_0_TXT: &str = include_str!("../license_examples/0bsd.txt");

    #[gpui::test]
    async fn test_edit_prediction_basic_interpolation(cx: &mut TestAppContext) {
        let buffer = cx.new(|cx| Buffer::local("Lorem ipsum dolor", cx));
        let edits: Arc<[(Range<Anchor>, String)]> = cx.update(|cx| {
            to_completion_edits(
                [(2..5, "REM".to_string()), (9..11, "".to_string())],
                &buffer,
                cx,
            )
            .into()
        });

        let edit_preview = cx
            .read(|cx| buffer.read(cx).preview_edits(edits.clone(), cx))
            .await;

        let completion = EditPrediction {
            edits,
            edit_preview,
            path: Path::new("").into(),
            snapshot: cx.read(|cx| buffer.read(cx).snapshot()),
            id: EditPredictionId(Uuid::new_v4()),
            excerpt_range: 0..0,
            cursor_offset: 0,
            input_outline: "".into(),
            input_events: "".into(),
            input_excerpt: "".into(),
            output_excerpt: "".into(),
            buffer_snapshotted_at: Instant::now(),
            response_received_at: Instant::now(),
        };

        cx.update(|cx| {
            assert_eq!(
                from_completion_edits(
                    &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(2..5, "REM".to_string()), (9..11, "".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(2..5, "")], None, cx));
            assert_eq!(
                from_completion_edits(
                    &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(2..2, "REM".to_string()), (6..8, "".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.undo(cx));
            assert_eq!(
                from_completion_edits(
                    &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(2..5, "REM".to_string()), (9..11, "".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(2..5, "R")], None, cx));
            assert_eq!(
                from_completion_edits(
                    &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(3..3, "EM".to_string()), (7..9, "".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(3..3, "E")], None, cx));
            assert_eq!(
                from_completion_edits(
                    &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(4..4, "M".to_string()), (8..10, "".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(4..4, "M")], None, cx));
            assert_eq!(
                from_completion_edits(
                    &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(9..11, "".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(4..5, "")], None, cx));
            assert_eq!(
                from_completion_edits(
                    &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(4..4, "M".to_string()), (8..10, "".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(8..10, "")], None, cx));
            assert_eq!(
                from_completion_edits(
                    &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(4..4, "M".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(4..6, "")], None, cx));
            assert_eq!(completion.interpolate(&buffer.read(cx).snapshot()), None);
        })
    }

    #[gpui::test]
    async fn test_clean_up_diff(cx: &mut TestAppContext) {
        init_test(cx);

        assert_eq!(
            apply_edit_prediction(
                indoc! {"
                    fn main() {
                        let word_1 = \"lorem\";
                        let range = word.len()..word.len();
                    }
                "},
                indoc! {"
                    <|editable_region_start|>
                    fn main() {
                        let word_1 = \"lorem\";
                        let range = word_1.len()..word_1.len();
                    }

                    <|editable_region_end|>
                "},
                cx,
            )
            .await,
            indoc! {"
                fn main() {
                    let word_1 = \"lorem\";
                    let range = word_1.len()..word_1.len();
                }
            "},
        );

        assert_eq!(
            apply_edit_prediction(
                indoc! {"
                    fn main() {
                        let story = \"the quick\"
                    }
                "},
                indoc! {"
                    <|editable_region_start|>
                    fn main() {
                        let story = \"the quick brown fox jumps over the lazy dog\";
                    }

                    <|editable_region_end|>
                "},
                cx,
            )
            .await,
            indoc! {"
                fn main() {
                    let story = \"the quick brown fox jumps over the lazy dog\";
                }
            "},
        );
    }

    #[gpui::test]
    async fn test_edit_prediction_end_of_buffer(cx: &mut TestAppContext) {
        init_test(cx);

        let buffer_content = "lorem\n";
        let completion_response = indoc! {"
            ```animals.js
            <|start_of_file|>
            <|editable_region_start|>
            lorem
            ipsum
            <|editable_region_end|>
            ```"};

        assert_eq!(
            apply_edit_prediction(buffer_content, completion_response, cx).await,
            "lorem\nipsum"
        );
    }

    #[gpui::test]
    async fn test_can_collect_data(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(path!("/project"), json!({ "LICENSE": BSD_0_TXT }))
            .await;

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/project/src/main.rs"), cx)
            })
            .await
            .unwrap();

        let (zeta, captured_request, _) = make_test_zeta(&project, cx).await;
        zeta.update(cx, |zeta, _cx| {
            zeta.data_collection_choice = DataCollectionChoice::Enabled
        });

        run_edit_prediction(&buffer, &project, &zeta, cx).await;
        assert_eq!(
            captured_request.lock().clone().unwrap().can_collect_data,
            true
        );

        zeta.update(cx, |zeta, _cx| {
            zeta.data_collection_choice = DataCollectionChoice::Disabled
        });

        run_edit_prediction(&buffer, &project, &zeta, cx).await;
        assert_eq!(
            captured_request.lock().clone().unwrap().can_collect_data,
            false
        );
    }

    #[gpui::test]
    async fn test_no_data_collection_for_remote_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        let project = Project::test(fs.clone(), [], cx).await;

        let buffer = cx.new(|_cx| {
            Buffer::remote(
                language::BufferId::new(1).unwrap(),
                1,
                language::Capability::ReadWrite,
                "fn main() {\n    println!(\"Hello\");\n}",
            )
        });

        let (zeta, captured_request, _) = make_test_zeta(&project, cx).await;
        zeta.update(cx, |zeta, _cx| {
            zeta.data_collection_choice = DataCollectionChoice::Enabled
        });

        run_edit_prediction(&buffer, &project, &zeta, cx).await;
        assert_eq!(
            captured_request.lock().clone().unwrap().can_collect_data,
            false
        );
    }

    #[gpui::test]
    async fn test_no_data_collection_for_private_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                "LICENSE": BSD_0_TXT,
                ".env": "SECRET_KEY=secret"
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/project/.env", cx)
            })
            .await
            .unwrap();

        let (zeta, captured_request, _) = make_test_zeta(&project, cx).await;
        zeta.update(cx, |zeta, _cx| {
            zeta.data_collection_choice = DataCollectionChoice::Enabled
        });

        run_edit_prediction(&buffer, &project, &zeta, cx).await;
        assert_eq!(
            captured_request.lock().clone().unwrap().can_collect_data,
            false
        );
    }

    #[gpui::test]
    async fn test_no_data_collection_for_untitled_buffer(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        let project = Project::test(fs.clone(), [], cx).await;
        let buffer = cx.new(|cx| Buffer::local("", cx));

        let (zeta, captured_request, _) = make_test_zeta(&project, cx).await;
        zeta.update(cx, |zeta, _cx| {
            zeta.data_collection_choice = DataCollectionChoice::Enabled
        });

        run_edit_prediction(&buffer, &project, &zeta, cx).await;
        assert_eq!(
            captured_request.lock().clone().unwrap().can_collect_data,
            false
        );
    }

    #[gpui::test]
    async fn test_no_data_collection_when_closed_source(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(path!("/project"), json!({ "main.rs": "fn main() {}" }))
            .await;

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/project/main.rs", cx)
            })
            .await
            .unwrap();

        let (zeta, captured_request, _) = make_test_zeta(&project, cx).await;
        zeta.update(cx, |zeta, _cx| {
            zeta.data_collection_choice = DataCollectionChoice::Enabled
        });

        run_edit_prediction(&buffer, &project, &zeta, cx).await;
        assert_eq!(
            captured_request.lock().clone().unwrap().can_collect_data,
            false
        );
    }

    #[gpui::test]
    async fn test_data_collection_status_changes_on_move(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/open_source_worktree"),
            json!({ "LICENSE": BSD_0_TXT, "main.rs": "" }),
        )
        .await;
        fs.insert_tree(path!("/closed_source_worktree"), json!({ "main.rs": "" }))
            .await;

        let project = Project::test(
            fs.clone(),
            [
                path!("/open_source_worktree").as_ref(),
                path!("/closed_source_worktree").as_ref(),
            ],
            cx,
        )
        .await;
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/open_source_worktree/main.rs"), cx)
            })
            .await
            .unwrap();

        let (zeta, captured_request, _) = make_test_zeta(&project, cx).await;
        zeta.update(cx, |zeta, _cx| {
            zeta.data_collection_choice = DataCollectionChoice::Enabled
        });

        run_edit_prediction(&buffer, &project, &zeta, cx).await;
        assert_eq!(
            captured_request.lock().clone().unwrap().can_collect_data,
            true
        );

        let closed_source_file = project
            .update(cx, |project, cx| {
                let worktree2 = project
                    .worktree_for_root_name("closed_source_worktree", cx)
                    .unwrap();
                worktree2.update(cx, |worktree2, cx| {
                    worktree2.load_file(rel_path("main.rs"), cx)
                })
            })
            .await
            .unwrap()
            .file;

        buffer.update(cx, |buffer, cx| {
            buffer.file_updated(closed_source_file, cx);
        });

        run_edit_prediction(&buffer, &project, &zeta, cx).await;
        assert_eq!(
            captured_request.lock().clone().unwrap().can_collect_data,
            false
        );
    }

    #[gpui::test]
    async fn test_no_data_collection_for_events_in_uncollectable_buffers(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/worktree1"),
            json!({ "LICENSE": BSD_0_TXT, "main.rs": "", "other.rs": "" }),
        )
        .await;
        fs.insert_tree(path!("/worktree2"), json!({ "private.rs": "" }))
            .await;

        let project = Project::test(
            fs.clone(),
            [path!("/worktree1").as_ref(), path!("/worktree2").as_ref()],
            cx,
        )
        .await;
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/worktree1/main.rs"), cx)
            })
            .await
            .unwrap();
        let private_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/worktree2/file.rs"), cx)
            })
            .await
            .unwrap();

        let (zeta, captured_request, _) = make_test_zeta(&project, cx).await;
        zeta.update(cx, |zeta, _cx| {
            zeta.data_collection_choice = DataCollectionChoice::Enabled
        });

        run_edit_prediction(&buffer, &project, &zeta, cx).await;
        assert_eq!(
            captured_request.lock().clone().unwrap().can_collect_data,
            true
        );

        // this has a side effect of registering the buffer to watch for edits
        run_edit_prediction(&private_buffer, &project, &zeta, cx).await;
        assert_eq!(
            captured_request.lock().clone().unwrap().can_collect_data,
            false
        );

        private_buffer.update(cx, |private_buffer, cx| {
            private_buffer.edit([(0..0, "An edit for the history!")], None, cx);
        });

        run_edit_prediction(&buffer, &project, &zeta, cx).await;
        assert_eq!(
            captured_request.lock().clone().unwrap().can_collect_data,
            false
        );

        // make an edit that uses too many bytes, causing private_buffer edit to not be able to be
        // included
        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                [(0..0, " ".repeat(MAX_EVENT_TOKENS * BYTES_PER_TOKEN_GUESS))],
                None,
                cx,
            );
        });

        run_edit_prediction(&buffer, &project, &zeta, cx).await;
        assert_eq!(
            captured_request.lock().clone().unwrap().can_collect_data,
            true
        );
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            client::init_settings(cx);
            Project::init_settings(cx);
        });
    }

    async fn apply_edit_prediction(
        buffer_content: &str,
        completion_response: &str,
        cx: &mut TestAppContext,
    ) -> String {
        let fs = project::FakeFs::new(cx.executor());
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let buffer = cx.new(|cx| Buffer::local(buffer_content, cx));
        let (zeta, _, response) = make_test_zeta(&project, cx).await;
        *response.lock() = completion_response.to_string();
        let edit_prediction = run_edit_prediction(&buffer, &project, &zeta, cx).await;
        buffer.update(cx, |buffer, cx| {
            buffer.edit(edit_prediction.edits.iter().cloned(), None, cx)
        });
        buffer.read_with(cx, |buffer, _| buffer.text())
    }

    async fn run_edit_prediction(
        buffer: &Entity<Buffer>,
        project: &Entity<Project>,
        zeta: &Entity<Zeta>,
        cx: &mut TestAppContext,
    ) -> EditPrediction {
        let cursor = buffer.read_with(cx, |buffer, _| buffer.anchor_before(Point::new(1, 0)));
        zeta.update(cx, |zeta, cx| zeta.register_buffer(buffer, &project, cx));
        cx.background_executor.run_until_parked();
        let completion_task = zeta.update(cx, |zeta, cx| {
            zeta.request_completion(&project, buffer, cursor, cx)
        });
        completion_task.await.unwrap().unwrap()
    }

    async fn make_test_zeta(
        project: &Entity<Project>,
        cx: &mut TestAppContext,
    ) -> (
        Entity<Zeta>,
        Arc<Mutex<Option<PredictEditsBody>>>,
        Arc<Mutex<String>>,
    ) {
        let default_response = indoc! {"
            ```main.rs
            <|start_of_file|>
            <|editable_region_start|>
            hello world
            <|editable_region_end|>
            ```"
        };
        let captured_request: Arc<Mutex<Option<PredictEditsBody>>> = Arc::new(Mutex::new(None));
        let completion_response: Arc<Mutex<String>> =
            Arc::new(Mutex::new(default_response.to_string()));
        let http_client = FakeHttpClient::create({
            let captured_request = captured_request.clone();
            let completion_response = completion_response.clone();
            move |req| {
                let captured_request = captured_request.clone();
                let completion_response = completion_response.clone();
                async move {
                    match (req.method(), req.uri().path()) {
                        (&Method::POST, "/client/llm_tokens") => {
                            Ok(http_client::Response::builder()
                                .status(200)
                                .body(
                                    serde_json::to_string(&CreateLlmTokenResponse {
                                        token: LlmToken("the-llm-token".to_string()),
                                    })
                                    .unwrap()
                                    .into(),
                                )
                                .unwrap())
                        }
                        (&Method::POST, "/predict_edits/v2") => {
                            let mut request_body = String::new();
                            req.into_body().read_to_string(&mut request_body).await?;
                            *captured_request.lock() =
                                Some(serde_json::from_str(&request_body).unwrap());
                            Ok(http_client::Response::builder()
                                .status(200)
                                .body(
                                    serde_json::to_string(&PredictEditsResponse {
                                        request_id: Uuid::new_v4(),
                                        output_excerpt: completion_response.lock().clone(),
                                    })
                                    .unwrap()
                                    .into(),
                                )
                                .unwrap())
                        }
                        _ => Ok(http_client::Response::builder()
                            .status(404)
                            .body("Not Found".into())
                            .unwrap()),
                    }
                }
            }
        });

        let client = cx.update(|cx| Client::new(Arc::new(FakeSystemClock::new()), http_client, cx));
        cx.update(|cx| {
            RefreshLlmTokenListener::register(client.clone(), cx);
        });
        let _server = FakeServer::for_client(42, &client, cx).await;

        let zeta = cx.new(|cx| {
            let mut zeta = Zeta::new(client, project.read(cx).user_store(), cx);

            let worktrees = project.read(cx).worktrees(cx).collect::<Vec<_>>();
            for worktree in worktrees {
                let worktree_id = worktree.read(cx).id();
                zeta.license_detection_watchers
                    .entry(worktree_id)
                    .or_insert_with(|| Rc::new(LicenseDetectionWatcher::new(&worktree, cx)));
            }

            zeta
        });

        (zeta, captured_request, completion_response)
    }

    fn to_completion_edits(
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

    fn from_completion_edits(
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

    #[ctor::ctor]
    fn init_logger() {
        zlog::init_test();
    }
}
