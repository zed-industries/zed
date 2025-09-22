use anyhow::{Context as _, Result, anyhow};
use arrayvec::ArrayVec;
use client::{Client, EditPredictionUsage, UserStore};
use cloud_llm_client::predict_edits_v3::{self, Signature};
use cloud_llm_client::{
    EXPIRED_LLM_TOKEN_HEADER_NAME, MINIMUM_REQUIRED_VERSION_HEADER_NAME, ZED_VERSION_HEADER_NAME,
};
use edit_prediction::{DataCollectionState, Direction, EditPredictionProvider};
use edit_prediction_context::{
    DeclarationId, EditPredictionContext, EditPredictionExcerptOptions, SyntaxIndex,
    SyntaxIndexState,
};
use futures::AsyncReadExt as _;
use gpui::http_client::Method;
use gpui::{
    App, Entity, EntityId, Global, SemanticVersion, SharedString, Subscription, Task, http_client,
    prelude::*,
};
use language::{Anchor, Buffer, OffsetRangeExt as _, ToPoint};
use language::{BufferSnapshot, EditPreview};
use language_model::{LlmApiToken, RefreshLlmTokenListener};
use project::Project;
use release_channel::AppVersion;
use std::cmp;
use std::collections::{HashMap, VecDeque, hash_map};
use std::path::PathBuf;
use std::str::FromStr as _;
use std::time::{Duration, Instant};
use std::{ops::Range, sync::Arc};
use thiserror::Error;
use util::ResultExt as _;
use uuid::Uuid;
use workspace::notifications::{ErrorMessagePrompt, NotificationId, show_app_notification};

const BUFFER_CHANGE_GROUPING_INTERVAL: Duration = Duration::from_secs(1);

/// Maximum number of events to track.
const MAX_EVENT_COUNT: usize = 16;

#[derive(Clone)]
struct ZetaGlobal(Entity<Zeta>);

impl Global for ZetaGlobal {}

pub struct Zeta {
    client: Arc<Client>,
    user_store: Entity<UserStore>,
    llm_token: LlmApiToken,
    _llm_token_subscription: Subscription,
    projects: HashMap<EntityId, ZetaProject>,
    excerpt_options: EditPredictionExcerptOptions,
    update_required: bool,
}

struct ZetaProject {
    syntax_index: Entity<SyntaxIndex>,
    events: VecDeque<Event>,
    registered_buffers: HashMap<gpui::EntityId, RegisteredBuffer>,
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

impl Zeta {
    pub fn global(
        client: &Arc<Client>,
        user_store: &Entity<UserStore>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.try_global::<ZetaGlobal>()
            .map(|global| global.0.clone())
            .unwrap_or_else(|| {
                let zeta = cx.new(|cx| Self::new(client.clone(), user_store.clone(), cx));
                cx.set_global(ZetaGlobal(zeta.clone()));
                zeta
            })
    }

    fn new(client: Arc<Client>, user_store: Entity<UserStore>, cx: &mut Context<Self>) -> Self {
        let refresh_llm_token_listener = RefreshLlmTokenListener::global(cx);

        Self {
            projects: HashMap::new(),
            client,
            user_store,
            excerpt_options: EditPredictionExcerptOptions {
                max_bytes: 512,
                min_bytes: 128,
                target_before_cursor_over_total_bytes: 0.5,
            },
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
        }
    }

    pub fn usage(&self, cx: &App) -> Option<EditPredictionUsage> {
        self.user_store.read(cx).edit_prediction_usage()
    }

    pub fn register_project(&mut self, project: &Entity<Project>, cx: &mut App) {
        self.get_or_init_zeta_project(project, cx);
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

    fn get_or_init_zeta_project(
        &mut self,
        project: &Entity<Project>,
        cx: &mut App,
    ) -> &mut ZetaProject {
        self.projects
            .entry(project.entity_id())
            .or_insert_with(|| ZetaProject {
                syntax_index: cx.new(|cx| SyntaxIndex::new(project, cx)),
                events: VecDeque::new(),
                registered_buffers: HashMap::new(),
            })
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
            let old_snapshot =
                std::mem::replace(&mut registered_buffer.snapshot, new_snapshot.clone());
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

    pub fn request_prediction(
        &mut self,
        project: &Entity<Project>,
        buffer: &Entity<Buffer>,
        position: language::Anchor,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<EditPrediction>>> {
        let project_state = self.projects.get(&project.entity_id());

        let index_state = project_state.map(|state| {
            state
                .syntax_index
                .read_with(cx, |index, _cx| index.state().clone())
        });
        let excerpt_options = self.excerpt_options.clone();
        let snapshot = buffer.read(cx).snapshot();
        let Some(excerpt_path) = snapshot.file().map(|path| path.full_path(cx)) else {
            return Task::ready(Err(anyhow!("No file path for excerpt")));
        };
        let client = self.client.clone();
        let llm_token = self.llm_token.clone();
        let app_version = AppVersion::global(cx);
        let worktree_snapshots = project
            .read(cx)
            .worktrees(cx)
            .map(|worktree| worktree.read(cx).snapshot())
            .collect::<Vec<_>>();

        let request_task = cx.background_spawn({
            let snapshot = snapshot.clone();
            async move {
                let index_state = if let Some(index_state) = index_state {
                    Some(index_state.lock_owned().await)
                } else {
                    None
                };

                let cursor_point = position.to_point(&snapshot);

                // TODO: make this only true if debug view is open
                let debug_info = true;

                let Some(request) = EditPredictionContext::gather_context(
                    cursor_point,
                    &snapshot,
                    &excerpt_options,
                    index_state.as_deref(),
                )
                .map(|context| {
                    make_cloud_request(
                        excerpt_path.clone(),
                        context,
                        // TODO pass everything
                        Vec::new(),
                        false,
                        Vec::new(),
                        None,
                        debug_info,
                        &worktree_snapshots,
                        index_state.as_deref(),
                    )
                }) else {
                    return Ok(None);
                };

                anyhow::Ok(Some(
                    Self::perform_request(client, llm_token, app_version, request).await?,
                ))
            }
        });

        let buffer = buffer.clone();

        cx.spawn(async move |this, cx| {
            match request_task.await {
                Ok(Some((response, usage))) => {
                    log::debug!("predicted edits: {:?}", &response.edits);

                    if let Some(usage) = usage {
                        this.update(cx, |this, cx| {
                            this.user_store.update(cx, |user_store, cx| {
                                user_store.update_edit_prediction_usage(usage, cx);
                            });
                        })
                        .ok();
                    }

                    // TODO telemetry: duration, etc

                    // TODO produce smaller edits by diffing against snapshot first
                    //
                    // Cloud returns entire snippets/excerpts ranges as they were included
                    // in the request, but we should display smaller edits to the user.
                    //
                    // We can do this by computing a diff of each one against the snapshot.
                    // Similar to zeta::Zeta::compute_edits, but per edit.
                    let edits = response
                        .edits
                        .into_iter()
                        .map(|edit| {
                            // TODO edits to different files
                            (
                                snapshot.anchor_before(edit.range.start)
                                    ..snapshot.anchor_before(edit.range.end),
                                edit.content,
                            )
                        })
                        .collect::<Vec<_>>()
                        .into();

                    let Some((edits, snapshot, edit_preview_task)) =
                        buffer.read_with(cx, |buffer, cx| {
                            let new_snapshot = buffer.snapshot();
                            let edits: Arc<[_]> =
                                interpolate(&snapshot, &new_snapshot, edits)?.into();
                            Some((edits.clone(), new_snapshot, buffer.preview_edits(edits, cx)))
                        })?
                    else {
                        return Ok(None);
                    };

                    Ok(Some(EditPrediction {
                        id: EditPredictionId(response.request_id),
                        edits,
                        snapshot,
                        edit_preview: edit_preview_task.await,
                    }))
                }
                Ok(None) => Ok(None),
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

                    Err(err)
                }
            }
        })
    }

    async fn perform_request(
        client: Arc<Client>,
        llm_token: LlmApiToken,
        app_version: SemanticVersion,
        request: predict_edits_v3::PredictEditsRequest,
    ) -> Result<(
        predict_edits_v3::PredictEditsResponse,
        Option<EditPredictionUsage>,
    )> {
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
                            .build_zed_llm_url("/predict_edits/v3", &[])?
                            .as_ref(),
                    )
                };
            let request = request_builder
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", token))
                .header(ZED_VERSION_HEADER_NAME, app_version.to_string())
                .body(serde_json::to_string(&request)?.into())?;

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

                let mut body = Vec::new();
                response.body_mut().read_to_end(&mut body).await?;
                return Ok((serde_json::from_slice(&body)?, usage));
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

#[derive(Error, Debug)]
#[error(
    "You must update to Zed version {minimum_version} or higher to continue using edit predictions."
)]
pub struct ZedUpdateRequiredError {
    minimum_version: SemanticVersion,
}

pub struct ZetaEditPredictionProvider {
    zeta: Entity<Zeta>,
    current_prediction: Option<CurrentEditPrediction>,
    next_pending_prediction_id: usize,
    pending_predictions: ArrayVec<PendingPrediction, 2>,
    last_request_timestamp: Instant,
}

impl ZetaEditPredictionProvider {
    pub const THROTTLE_TIMEOUT: Duration = Duration::from_millis(300);

    pub fn new(
        project: Option<&Entity<Project>>,
        client: &Arc<Client>,
        user_store: &Entity<UserStore>,
        cx: &mut App,
    ) -> Self {
        let zeta = Zeta::global(client, user_store, cx);
        if let Some(project) = project {
            zeta.update(cx, |zeta, cx| {
                zeta.register_project(project, cx);
            });
        }

        Self {
            zeta,
            current_prediction: None,
            next_pending_prediction_id: 0,
            pending_predictions: ArrayVec::new(),
            last_request_timestamp: Instant::now(),
        }
    }
}

#[derive(Clone)]
struct CurrentEditPrediction {
    buffer_id: EntityId,
    prediction: EditPrediction,
}

impl CurrentEditPrediction {
    fn should_replace_prediction(&self, old_prediction: &Self, snapshot: &BufferSnapshot) -> bool {
        if self.buffer_id != old_prediction.buffer_id {
            return true;
        }

        let Some(old_edits) = old_prediction.prediction.interpolate(snapshot) else {
            return true;
        };
        let Some(new_edits) = self.prediction.interpolate(snapshot) else {
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
    id: EditPredictionId,
    edits: Arc<[(Range<Anchor>, String)]>,
    snapshot: BufferSnapshot,
    edit_preview: EditPreview,
}

impl EditPrediction {
    fn interpolate(&self, new_snapshot: &BufferSnapshot) -> Option<Vec<(Range<Anchor>, String)>> {
        interpolate(&self.snapshot, new_snapshot, self.edits.clone())
    }
}

struct PendingPrediction {
    id: usize,
    _task: Task<()>,
}

impl EditPredictionProvider for ZetaEditPredictionProvider {
    fn name() -> &'static str {
        "zed-predict2"
    }

    fn display_name() -> &'static str {
        "Zed's Edit Predictions 2"
    }

    fn show_completions_in_menu() -> bool {
        true
    }

    fn show_tab_accept_marker() -> bool {
        true
    }

    fn data_collection_state(&self, _cx: &App) -> DataCollectionState {
        // TODO [zeta2]
        DataCollectionState::Unsupported
    }

    fn toggle_data_collection(&mut self, _cx: &mut App) {
        // TODO [zeta2]
    }

    fn usage(&self, cx: &App) -> Option<client::EditPredictionUsage> {
        self.zeta.read(cx).usage(cx)
    }

    fn is_enabled(
        &self,
        _buffer: &Entity<language::Buffer>,
        _cursor_position: language::Anchor,
        _cx: &App,
    ) -> bool {
        true
    }

    fn is_refreshing(&self) -> bool {
        !self.pending_predictions.is_empty()
    }

    fn refresh(
        &mut self,
        project: Option<Entity<project::Project>>,
        buffer: Entity<language::Buffer>,
        cursor_position: language::Anchor,
        _debounce: bool,
        cx: &mut Context<Self>,
    ) {
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

        if let Some(current_prediction) = self.current_prediction.as_ref() {
            let snapshot = buffer.read(cx).snapshot();
            if current_prediction
                .prediction
                .interpolate(&snapshot)
                .is_some()
            {
                return;
            }
        }

        let pending_prediction_id = self.next_pending_prediction_id;
        self.next_pending_prediction_id += 1;
        let last_request_timestamp = self.last_request_timestamp;

        let task = cx.spawn(async move |this, cx| {
            if let Some(timeout) = (last_request_timestamp + Self::THROTTLE_TIMEOUT)
                .checked_duration_since(Instant::now())
            {
                cx.background_executor().timer(timeout).await;
            }

            let prediction_request = this.update(cx, |this, cx| {
                this.last_request_timestamp = Instant::now();
                this.zeta.update(cx, |zeta, cx| {
                    zeta.request_prediction(&project, &buffer, cursor_position, cx)
                })
            });

            let prediction = match prediction_request {
                Ok(prediction_request) => {
                    let prediction_request = prediction_request.await;
                    prediction_request.map(|c| {
                        c.map(|prediction| CurrentEditPrediction {
                            buffer_id: buffer.entity_id(),
                            prediction,
                        })
                    })
                }
                Err(error) => Err(error),
            };

            this.update(cx, |this, cx| {
                if this.pending_predictions[0].id == pending_prediction_id {
                    this.pending_predictions.remove(0);
                } else {
                    this.pending_predictions.clear();
                }

                let Some(new_prediction) = prediction
                    .context("edit prediction failed")
                    .log_err()
                    .flatten()
                else {
                    cx.notify();
                    return;
                };

                if let Some(old_prediction) = this.current_prediction.as_ref() {
                    let snapshot = buffer.read(cx).snapshot();
                    if new_prediction.should_replace_prediction(old_prediction, &snapshot) {
                        this.current_prediction = Some(new_prediction);
                    }
                } else {
                    this.current_prediction = Some(new_prediction);
                }

                cx.notify();
            })
            .ok();
        });

        // We always maintain at most two pending predictions. When we already
        // have two, we replace the newest one.
        if self.pending_predictions.len() <= 1 {
            self.pending_predictions.push(PendingPrediction {
                id: pending_prediction_id,
                _task: task,
            });
        } else if self.pending_predictions.len() == 2 {
            self.pending_predictions.pop();
            self.pending_predictions.push(PendingPrediction {
                id: pending_prediction_id,
                _task: task,
            });
        }

        cx.notify();
    }

    fn cycle(
        &mut self,
        _buffer: Entity<language::Buffer>,
        _cursor_position: language::Anchor,
        _direction: Direction,
        _cx: &mut Context<Self>,
    ) {
    }

    fn accept(&mut self, _cx: &mut Context<Self>) {
        // TODO [zeta2] report accept
        self.current_prediction.take();
        self.pending_predictions.clear();
    }

    fn discard(&mut self, _cx: &mut Context<Self>) {
        self.pending_predictions.clear();
        self.current_prediction.take();
    }

    fn suggest(
        &mut self,
        buffer: &Entity<language::Buffer>,
        cursor_position: language::Anchor,
        cx: &mut Context<Self>,
    ) -> Option<edit_prediction::EditPrediction> {
        let CurrentEditPrediction {
            buffer_id,
            prediction,
            ..
        } = self.current_prediction.as_mut()?;

        // Invalidate previous prediction if it was generated for a different buffer.
        if *buffer_id != buffer.entity_id() {
            self.current_prediction.take();
            return None;
        }

        let buffer = buffer.read(cx);
        let Some(edits) = prediction.interpolate(&buffer.snapshot()) else {
            self.current_prediction.take();
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
            id: Some(prediction.id.to_string().into()),
            edits: edits[edit_start_ix..edit_end_ix].to_vec(),
            edit_preview: Some(prediction.edit_preview.clone()),
        })
    }
}

fn make_cloud_request(
    excerpt_path: PathBuf,
    context: EditPredictionContext,
    events: Vec<predict_edits_v3::Event>,
    can_collect_data: bool,
    diagnostic_groups: Vec<predict_edits_v3::DiagnosticGroup>,
    git_info: Option<cloud_llm_client::PredictEditsGitInfo>,
    debug_info: bool,
    worktrees: &Vec<worktree::Snapshot>,
    index_state: Option<&SyntaxIndexState>,
) -> predict_edits_v3::PredictEditsRequest {
    let mut signatures = Vec::new();
    let mut declaration_to_signature_index = HashMap::default();
    let mut referenced_declarations = Vec::new();

    for snippet in context.snippets {
        let project_entry_id = snippet.declaration.project_entry_id();
        // TODO: Use full paths (worktree rooted) - need to move full_path method to the snapshot.
        // Note that currently full_path is currently being used for excerpt_path.
        let Some(path) = worktrees.iter().find_map(|worktree| {
            let abs_path = worktree.abs_path();
            worktree
                .entry_for_id(project_entry_id)
                .map(|e| abs_path.join(&e.path))
        }) else {
            continue;
        };

        let parent_index = index_state.and_then(|index_state| {
            snippet.declaration.parent().and_then(|parent| {
                add_signature(
                    parent,
                    &mut declaration_to_signature_index,
                    &mut signatures,
                    index_state,
                )
            })
        });

        let (text, text_is_truncated) = snippet.declaration.item_text();
        referenced_declarations.push(predict_edits_v3::ReferencedDeclaration {
            path,
            text: text.into(),
            range: snippet.declaration.item_range(),
            text_is_truncated,
            signature_range: snippet.declaration.signature_range_in_item_text(),
            parent_index,
            score_components: snippet.score_components,
            signature_score: snippet.scores.signature,
            declaration_score: snippet.scores.declaration,
        });
    }

    let excerpt_parent = index_state.and_then(|index_state| {
        context
            .excerpt
            .parent_declarations
            .last()
            .and_then(|(parent, _)| {
                add_signature(
                    *parent,
                    &mut declaration_to_signature_index,
                    &mut signatures,
                    index_state,
                )
            })
    });

    predict_edits_v3::PredictEditsRequest {
        excerpt_path,
        excerpt: context.excerpt_text.body,
        excerpt_range: context.excerpt.range,
        cursor_offset: context.cursor_offset_in_excerpt,
        referenced_declarations,
        signatures,
        excerpt_parent,
        events,
        can_collect_data,
        diagnostic_groups,
        git_info,
        debug_info,
    }
}

fn add_signature(
    declaration_id: DeclarationId,
    declaration_to_signature_index: &mut HashMap<DeclarationId, usize>,
    signatures: &mut Vec<Signature>,
    index: &SyntaxIndexState,
) -> Option<usize> {
    if let Some(signature_index) = declaration_to_signature_index.get(&declaration_id) {
        return Some(*signature_index);
    }
    let Some(parent_declaration) = index.declaration(declaration_id) else {
        log::error!("bug: missing parent declaration");
        return None;
    };
    let parent_index = parent_declaration.parent().and_then(|parent| {
        add_signature(parent, declaration_to_signature_index, signatures, index)
    });
    let (text, text_is_truncated) = parent_declaration.signature_text();
    let signature_index = signatures.len();
    signatures.push(Signature {
        text: text.into(),
        text_is_truncated,
        parent_index,
    });
    declaration_to_signature_index.insert(declaration_id, signature_index);
    Some(signature_index)
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use language::ToOffset as _;

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
