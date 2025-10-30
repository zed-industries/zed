use anyhow::{Context as _, Result, anyhow};
use chrono::TimeDelta;
use client::{Client, EditPredictionUsage, UserStore};
use cloud_llm_client::predict_edits_v3::{self, PromptFormat, Signature};
use cloud_llm_client::{
    AcceptEditPredictionBody, EXPIRED_LLM_TOKEN_HEADER_NAME, MINIMUM_REQUIRED_VERSION_HEADER_NAME,
    ZED_VERSION_HEADER_NAME,
};
use cloud_zeta2_prompt::{DEFAULT_MAX_PROMPT_BYTES, build_prompt};
use collections::HashMap;
use edit_prediction_context::{
    DeclarationId, DeclarationStyle, EditPredictionContext, EditPredictionContextOptions,
    EditPredictionExcerpt, EditPredictionExcerptOptions, EditPredictionScoreOptions, Line,
    SyntaxIndex, SyntaxIndexState,
};
use feature_flags::{FeatureFlag, FeatureFlagAppExt as _};
use futures::AsyncReadExt as _;
use futures::channel::{mpsc, oneshot};
use gpui::http_client::{AsyncBody, Method};
use gpui::{
    App, Entity, EntityId, Global, SemanticVersion, SharedString, Subscription, Task, WeakEntity,
    http_client, prelude::*,
};
use language::{Anchor, Buffer, DiagnosticSet, LanguageServerId, ToOffset as _, ToPoint};
use language::{BufferSnapshot, OffsetRangeExt};
use language_model::{LlmApiToken, RefreshLlmTokenListener};
use project::Project;
use release_channel::AppVersion;
use serde::de::DeserializeOwned;
use std::collections::{VecDeque, hash_map};
use std::fmt::Write;
use std::ops::Range;
use std::path::Path;
use std::str::FromStr as _;
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;
use util::ResultExt as _;
use util::rel_path::RelPathBuf;
use workspace::notifications::{ErrorMessagePrompt, NotificationId, show_app_notification};

pub mod merge_excerpts;
mod prediction;
mod provider;
pub mod related_excerpts;

use crate::merge_excerpts::merge_excerpts;
use crate::prediction::EditPrediction;
use crate::related_excerpts::find_related_excerpts;
pub use crate::related_excerpts::{LlmContextOptions, SearchToolQuery};
pub use provider::ZetaEditPredictionProvider;

const BUFFER_CHANGE_GROUPING_INTERVAL: Duration = Duration::from_secs(1);

/// Maximum number of events to track.
const MAX_EVENT_COUNT: usize = 16;

pub const DEFAULT_EXCERPT_OPTIONS: EditPredictionExcerptOptions = EditPredictionExcerptOptions {
    max_bytes: 512,
    min_bytes: 128,
    target_before_cursor_over_total_bytes: 0.5,
};

pub const DEFAULT_CONTEXT_OPTIONS: ContextMode = ContextMode::Llm(DEFAULT_LLM_CONTEXT_OPTIONS);

pub const DEFAULT_LLM_CONTEXT_OPTIONS: LlmContextOptions = LlmContextOptions {
    excerpt: DEFAULT_EXCERPT_OPTIONS,
};

pub const DEFAULT_SYNTAX_CONTEXT_OPTIONS: EditPredictionContextOptions =
    EditPredictionContextOptions {
        use_imports: true,
        max_retrieved_declarations: 0,
        excerpt: DEFAULT_EXCERPT_OPTIONS,
        score: EditPredictionScoreOptions {
            omit_excerpt_overlaps: true,
        },
    };

pub const DEFAULT_OPTIONS: ZetaOptions = ZetaOptions {
    context: DEFAULT_CONTEXT_OPTIONS,
    max_prompt_bytes: DEFAULT_MAX_PROMPT_BYTES,
    max_diagnostic_bytes: 2048,
    prompt_format: PromptFormat::DEFAULT,
    file_indexing_parallelism: 1,
};

pub struct Zeta2FeatureFlag;

impl FeatureFlag for Zeta2FeatureFlag {
    const NAME: &'static str = "zeta2";

    fn enabled_for_staff() -> bool {
        false
    }
}

#[derive(Clone)]
struct ZetaGlobal(Entity<Zeta>);

impl Global for ZetaGlobal {}

pub struct Zeta {
    client: Arc<Client>,
    user_store: Entity<UserStore>,
    llm_token: LlmApiToken,
    _llm_token_subscription: Subscription,
    projects: HashMap<EntityId, ZetaProject>,
    options: ZetaOptions,
    update_required: bool,
    debug_tx: Option<mpsc::UnboundedSender<ZetaDebugInfo>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ZetaOptions {
    pub context: ContextMode,
    pub max_prompt_bytes: usize,
    pub max_diagnostic_bytes: usize,
    pub prompt_format: predict_edits_v3::PromptFormat,
    pub file_indexing_parallelism: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextMode {
    Llm(LlmContextOptions),
    Syntax(EditPredictionContextOptions),
}

impl ContextMode {
    pub fn excerpt(&self) -> &EditPredictionExcerptOptions {
        match self {
            ContextMode::Llm(options) => &options.excerpt,
            ContextMode::Syntax(options) => &options.excerpt,
        }
    }
}

pub enum ZetaDebugInfo {
    ContextRetrievalStarted(ZetaContextRetrievalStartedDebugInfo),
    SearchQueriesGenerated(ZetaSearchQueryDebugInfo),
    SearchQueriesExecuted(ZetaContextRetrievalDebugInfo),
    SearchResultsFiltered(ZetaContextRetrievalDebugInfo),
    ContextRetrievalFinished(ZetaContextRetrievalDebugInfo),
    EditPredicted(ZetaEditPredictionDebugInfo),
}

pub struct ZetaContextRetrievalStartedDebugInfo {
    pub project: Entity<Project>,
    pub timestamp: Instant,
    pub search_prompt: String,
}

pub struct ZetaContextRetrievalDebugInfo {
    pub project: Entity<Project>,
    pub timestamp: Instant,
}

pub struct ZetaEditPredictionDebugInfo {
    pub request: predict_edits_v3::PredictEditsRequest,
    pub retrieval_time: TimeDelta,
    pub buffer: WeakEntity<Buffer>,
    pub position: language::Anchor,
    pub local_prompt: Result<String, String>,
    pub response_rx: oneshot::Receiver<Result<predict_edits_v3::PredictEditsResponse, String>>,
}

pub struct ZetaSearchQueryDebugInfo {
    pub project: Entity<Project>,
    pub timestamp: Instant,
    pub queries: Vec<SearchToolQuery>,
}

pub type RequestDebugInfo = predict_edits_v3::DebugInfo;

struct ZetaProject {
    syntax_index: Entity<SyntaxIndex>,
    events: VecDeque<Event>,
    registered_buffers: HashMap<gpui::EntityId, RegisteredBuffer>,
    current_prediction: Option<CurrentEditPrediction>,
    context: Option<HashMap<Entity<Buffer>, Vec<Range<Anchor>>>>,
    refresh_context_task: Option<Task<Option<()>>>,
    refresh_context_debounce_task: Option<Task<Option<()>>>,
    refresh_context_timestamp: Option<Instant>,
}

#[derive(Debug, Clone)]
struct CurrentEditPrediction {
    pub requested_by_buffer_id: EntityId,
    pub prediction: EditPrediction,
}

impl CurrentEditPrediction {
    fn should_replace_prediction(&self, old_prediction: &Self, cx: &App) -> bool {
        let Some(new_edits) = self
            .prediction
            .interpolate(&self.prediction.buffer.read(cx))
        else {
            return false;
        };

        if self.prediction.buffer != old_prediction.prediction.buffer {
            return true;
        }

        let Some(old_edits) = old_prediction
            .prediction
            .interpolate(&old_prediction.prediction.buffer.read(cx))
        else {
            return true;
        };

        // This reduces the occurrence of UI thrash from replacing edits
        //
        // TODO: This is fairly arbitrary - should have a more general heuristic that handles multiple edits.
        if self.requested_by_buffer_id == self.prediction.buffer.entity_id()
            && self.requested_by_buffer_id == old_prediction.prediction.buffer.entity_id()
            && old_edits.len() == 1
            && new_edits.len() == 1
        {
            let (old_range, old_text) = &old_edits[0];
            let (new_range, new_text) = &new_edits[0];
            new_range == old_range && new_text.starts_with(old_text)
        } else {
            true
        }
    }
}

/// A prediction from the perspective of a buffer.
#[derive(Debug)]
enum BufferEditPrediction<'a> {
    Local { prediction: &'a EditPrediction },
    Jump { prediction: &'a EditPrediction },
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
    pub fn to_request_event(&self, cx: &App) -> Option<predict_edits_v3::Event> {
        match self {
            Event::BufferChange {
                old_snapshot,
                new_snapshot,
                ..
            } => {
                let path = new_snapshot.file().map(|f| f.full_path(cx));

                let old_path = old_snapshot.file().and_then(|f| {
                    let old_path = f.full_path(cx);
                    if Some(&old_path) != path.as_ref() {
                        Some(old_path)
                    } else {
                        None
                    }
                });

                // TODO [zeta2] move to bg?
                let diff = language::unified_diff(&old_snapshot.text(), &new_snapshot.text());

                if path == old_path && diff.is_empty() {
                    None
                } else {
                    Some(predict_edits_v3::Event::BufferChange {
                        old_path,
                        path,
                        diff,
                        //todo: Actually detect if this edit was predicted or not
                        predicted: false,
                    })
                }
            }
        }
    }
}

impl Zeta {
    pub fn try_global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<ZetaGlobal>().map(|global| global.0.clone())
    }

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

    pub fn new(client: Arc<Client>, user_store: Entity<UserStore>, cx: &mut Context<Self>) -> Self {
        let refresh_llm_token_listener = RefreshLlmTokenListener::global(cx);

        Self {
            projects: HashMap::default(),
            client,
            user_store,
            options: DEFAULT_OPTIONS,
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
            debug_tx: None,
        }
    }

    pub fn debug_info(&mut self) -> mpsc::UnboundedReceiver<ZetaDebugInfo> {
        let (debug_watch_tx, debug_watch_rx) = mpsc::unbounded();
        self.debug_tx = Some(debug_watch_tx);
        debug_watch_rx
    }

    pub fn options(&self) -> &ZetaOptions {
        &self.options
    }

    pub fn set_options(&mut self, options: ZetaOptions) {
        self.options = options;
    }

    pub fn clear_history(&mut self) {
        for zeta_project in self.projects.values_mut() {
            zeta_project.events.clear();
        }
    }

    pub fn history_for_project(&self, project: &Entity<Project>) -> impl Iterator<Item = &Event> {
        self.projects
            .get(&project.entity_id())
            .map(|project| project.events.iter())
            .into_iter()
            .flatten()
    }

    pub fn context_for_project(
        &self,
        project: &Entity<Project>,
    ) -> impl Iterator<Item = (Entity<Buffer>, &[Range<Anchor>])> {
        self.projects
            .get(&project.entity_id())
            .and_then(|project| {
                Some(
                    project
                        .context
                        .as_ref()?
                        .iter()
                        .map(|(buffer, ranges)| (buffer.clone(), ranges.as_slice())),
                )
            })
            .into_iter()
            .flatten()
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
                syntax_index: cx.new(|cx| {
                    SyntaxIndex::new(project, self.options.file_indexing_parallelism, cx)
                }),
                events: VecDeque::new(),
                registered_buffers: HashMap::default(),
                current_prediction: None,
                context: None,
                refresh_context_task: None,
                refresh_context_debounce_task: None,
                refresh_context_timestamp: None,
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

    fn current_prediction_for_buffer(
        &self,
        buffer: &Entity<Buffer>,
        project: &Entity<Project>,
        cx: &App,
    ) -> Option<BufferEditPrediction<'_>> {
        let project_state = self.projects.get(&project.entity_id())?;

        let CurrentEditPrediction {
            requested_by_buffer_id,
            prediction,
        } = project_state.current_prediction.as_ref()?;

        if prediction.targets_buffer(buffer.read(cx), cx) {
            Some(BufferEditPrediction::Local { prediction })
        } else if *requested_by_buffer_id == buffer.entity_id() {
            Some(BufferEditPrediction::Jump { prediction })
        } else {
            None
        }
    }

    fn accept_current_prediction(&mut self, project: &Entity<Project>, cx: &mut Context<Self>) {
        let Some(project_state) = self.projects.get_mut(&project.entity_id()) else {
            return;
        };

        let Some(prediction) = project_state.current_prediction.take() else {
            return;
        };
        let request_id = prediction.prediction.id.into();

        let client = self.client.clone();
        let llm_token = self.llm_token.clone();
        let app_version = AppVersion::global(cx);
        cx.spawn(async move |this, cx| {
            let url = if let Ok(predict_edits_url) = std::env::var("ZED_ACCEPT_PREDICTION_URL") {
                http_client::Url::parse(&predict_edits_url)?
            } else {
                client
                    .http_client()
                    .build_zed_llm_url("/predict_edits/accept", &[])?
            };

            let response = cx
                .background_spawn(Self::send_api_request::<()>(
                    move |builder| {
                        let req = builder.uri(url.as_ref()).body(
                            serde_json::to_string(&AcceptEditPredictionBody { request_id })?.into(),
                        );
                        Ok(req?)
                    },
                    client,
                    llm_token,
                    app_version,
                ))
                .await;

            Self::handle_api_response(&this, response, cx)?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn discard_current_prediction(&mut self, project: &Entity<Project>) {
        if let Some(project_state) = self.projects.get_mut(&project.entity_id()) {
            project_state.current_prediction.take();
        };
    }

    pub fn refresh_prediction(
        &mut self,
        project: &Entity<Project>,
        buffer: &Entity<Buffer>,
        position: language::Anchor,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let request_task = self.request_prediction(project, buffer, position, cx);
        let buffer = buffer.clone();
        let project = project.clone();

        cx.spawn(async move |this, cx| {
            if let Some(prediction) = request_task.await? {
                this.update(cx, |this, cx| {
                    let project_state = this
                        .projects
                        .get_mut(&project.entity_id())
                        .context("Project not found")?;

                    let new_prediction = CurrentEditPrediction {
                        requested_by_buffer_id: buffer.entity_id(),
                        prediction: prediction,
                    };

                    if project_state
                        .current_prediction
                        .as_ref()
                        .is_none_or(|old_prediction| {
                            new_prediction.should_replace_prediction(&old_prediction, cx)
                        })
                    {
                        project_state.current_prediction = Some(new_prediction);
                    }
                    anyhow::Ok(())
                })??;
            }
            Ok(())
        })
    }

    fn request_prediction(
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
        let options = self.options.clone();
        let snapshot = buffer.read(cx).snapshot();
        let Some(excerpt_path) = snapshot
            .file()
            .map(|path| -> Arc<Path> { path.full_path(cx).into() })
        else {
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
        let debug_tx = self.debug_tx.clone();

        let events = project_state
            .map(|state| {
                state
                    .events
                    .iter()
                    .filter_map(|event| event.to_request_event(cx))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let diagnostics = snapshot.diagnostic_sets().clone();

        let parent_abs_path = project::File::from_dyn(buffer.read(cx).file()).and_then(|f| {
            let mut path = f.worktree.read(cx).absolutize(&f.path);
            if path.pop() { Some(path) } else { None }
        });

        // TODO data collection
        let can_collect_data = cx.is_staff();

        let mut included_files = project_state
            .and_then(|project_state| project_state.context.as_ref())
            .unwrap_or(&HashMap::default())
            .iter()
            .filter_map(|(buffer, ranges)| {
                let buffer = buffer.read(cx);
                Some((
                    buffer.snapshot(),
                    buffer.file()?.full_path(cx).into(),
                    ranges.clone(),
                ))
            })
            .collect::<Vec<_>>();

        let request_task = cx.background_spawn({
            let snapshot = snapshot.clone();
            let buffer = buffer.clone();
            async move {
                let index_state = if let Some(index_state) = index_state {
                    Some(index_state.lock_owned().await)
                } else {
                    None
                };

                let cursor_offset = position.to_offset(&snapshot);
                let cursor_point = cursor_offset.to_point(&snapshot);

                let before_retrieval = chrono::Utc::now();

                let (diagnostic_groups, diagnostic_groups_truncated) =
                    Self::gather_nearby_diagnostics(
                        cursor_offset,
                        &diagnostics,
                        &snapshot,
                        options.max_diagnostic_bytes,
                    );

                let request = match options.context {
                    ContextMode::Llm(context_options) => {
                        let Some(excerpt) = EditPredictionExcerpt::select_from_buffer(
                            cursor_point,
                            &snapshot,
                            &context_options.excerpt,
                            index_state.as_deref(),
                        ) else {
                            return Ok((None, None));
                        };

                        let excerpt_anchor_range = snapshot.anchor_after(excerpt.range.start)
                            ..snapshot.anchor_before(excerpt.range.end);

                        if let Some(buffer_ix) = included_files
                            .iter()
                            .position(|(buffer, _, _)| buffer.remote_id() == snapshot.remote_id())
                        {
                            let (buffer, _, ranges) = &mut included_files[buffer_ix];
                            let range_ix = ranges
                                .binary_search_by(|probe| {
                                    probe
                                        .start
                                        .cmp(&excerpt_anchor_range.start, buffer)
                                        .then(excerpt_anchor_range.end.cmp(&probe.end, buffer))
                                })
                                .unwrap_or_else(|ix| ix);

                            ranges.insert(range_ix, excerpt_anchor_range);
                            let last_ix = included_files.len() - 1;
                            included_files.swap(buffer_ix, last_ix);
                        } else {
                            included_files.push((
                                snapshot,
                                excerpt_path.clone(),
                                vec![excerpt_anchor_range],
                            ));
                        }

                        let included_files = included_files
                            .into_iter()
                            .map(|(buffer, path, ranges)| {
                                let excerpts = merge_excerpts(
                                    &buffer,
                                    ranges.iter().map(|range| {
                                        let point_range = range.to_point(&buffer);
                                        Line(point_range.start.row)..Line(point_range.end.row)
                                    }),
                                );
                                predict_edits_v3::IncludedFile {
                                    path,
                                    max_row: Line(buffer.max_point().row),
                                    excerpts,
                                }
                            })
                            .collect::<Vec<_>>();

                        predict_edits_v3::PredictEditsRequest {
                            excerpt_path,
                            excerpt: String::new(),
                            excerpt_line_range: Line(0)..Line(0),
                            excerpt_range: 0..0,
                            cursor_point: predict_edits_v3::Point {
                                line: predict_edits_v3::Line(cursor_point.row),
                                column: cursor_point.column,
                            },
                            included_files,
                            referenced_declarations: vec![],
                            events,
                            can_collect_data,
                            diagnostic_groups,
                            diagnostic_groups_truncated,
                            debug_info: debug_tx.is_some(),
                            prompt_max_bytes: Some(options.max_prompt_bytes),
                            prompt_format: options.prompt_format,
                            // TODO [zeta2]
                            signatures: vec![],
                            excerpt_parent: None,
                            git_info: None,
                        }
                    }
                    ContextMode::Syntax(context_options) => {
                        let Some(context) = EditPredictionContext::gather_context(
                            cursor_point,
                            &snapshot,
                            parent_abs_path.as_deref(),
                            &context_options,
                            index_state.as_deref(),
                        ) else {
                            return Ok((None, None));
                        };

                        make_syntax_context_cloud_request(
                            excerpt_path,
                            context,
                            events,
                            can_collect_data,
                            diagnostic_groups,
                            diagnostic_groups_truncated,
                            None,
                            debug_tx.is_some(),
                            &worktree_snapshots,
                            index_state.as_deref(),
                            Some(options.max_prompt_bytes),
                            options.prompt_format,
                        )
                    }
                };

                let retrieval_time = chrono::Utc::now() - before_retrieval;

                let debug_response_tx = if let Some(debug_tx) = &debug_tx {
                    let (response_tx, response_rx) = oneshot::channel();

                    let local_prompt = build_prompt(&request)
                        .map(|(prompt, _)| prompt)
                        .map_err(|err| err.to_string());

                    debug_tx
                        .unbounded_send(ZetaDebugInfo::EditPredicted(ZetaEditPredictionDebugInfo {
                            request: request.clone(),
                            retrieval_time,
                            buffer: buffer.downgrade(),
                            local_prompt,
                            position,
                            response_rx,
                        }))
                        .ok();
                    Some(response_tx)
                } else {
                    None
                };

                if cfg!(debug_assertions) && std::env::var("ZED_ZETA2_SKIP_REQUEST").is_ok() {
                    if let Some(debug_response_tx) = debug_response_tx {
                        debug_response_tx
                            .send(Err("Request skipped".to_string()))
                            .ok();
                    }
                    anyhow::bail!("Skipping request because ZED_ZETA2_SKIP_REQUEST is set")
                }

                let response =
                    Self::send_prediction_request(client, llm_token, app_version, request).await;

                if let Some(debug_response_tx) = debug_response_tx {
                    debug_response_tx
                        .send(
                            response
                                .as_ref()
                                .map_err(|err| err.to_string())
                                .map(|response| response.0.clone()),
                        )
                        .ok();
                }

                response.map(|(res, usage)| (Some(res), usage))
            }
        });

        let buffer = buffer.clone();

        cx.spawn({
            let project = project.clone();
            async move |this, cx| {
                let Some(response) = Self::handle_api_response(&this, request_task.await, cx)?
                else {
                    return Ok(None);
                };

                // TODO telemetry: duration, etc
                Ok(EditPrediction::from_response(response, &snapshot, &buffer, &project, cx).await)
            }
        })
    }

    async fn send_prediction_request(
        client: Arc<Client>,
        llm_token: LlmApiToken,
        app_version: SemanticVersion,
        request: predict_edits_v3::PredictEditsRequest,
    ) -> Result<(
        predict_edits_v3::PredictEditsResponse,
        Option<EditPredictionUsage>,
    )> {
        let url = if let Ok(predict_edits_url) = std::env::var("ZED_PREDICT_EDITS_URL") {
            http_client::Url::parse(&predict_edits_url)?
        } else {
            client
                .http_client()
                .build_zed_llm_url("/predict_edits/v3", &[])?
        };

        Self::send_api_request(
            |builder| {
                let req = builder
                    .uri(url.as_ref())
                    .body(serde_json::to_string(&request)?.into());
                Ok(req?)
            },
            client,
            llm_token,
            app_version,
        )
        .await
    }

    fn handle_api_response<T>(
        this: &WeakEntity<Self>,
        response: Result<(T, Option<EditPredictionUsage>)>,
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
                    })
                    .ok();
                }
                Err(err)
            }
        }
    }

    async fn send_api_request<Res>(
        build: impl Fn(http_client::http::request::Builder) -> Result<http_client::Request<AsyncBody>>,
        client: Arc<Client>,
        llm_token: LlmApiToken,
        app_version: SemanticVersion,
    ) -> Result<(Res, Option<EditPredictionUsage>)>
    where
        Res: DeserializeOwned,
    {
        let http_client = client.http_client();
        let mut token = llm_token.acquire(&client).await?;
        let mut did_retry = false;

        loop {
            let request_builder = http_client::Request::builder().method(Method::POST);

            let request = build(
                request_builder
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {}", token))
                    .header(ZED_VERSION_HEADER_NAME, app_version.to_string()),
            )?;

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
                    "Request failed with status: {:?}\nBody: {}",
                    response.status(),
                    body
                );
            }
        }
    }

    pub const CONTEXT_RETRIEVAL_IDLE_DURATION: Duration = Duration::from_secs(10);
    pub const CONTEXT_RETRIEVAL_DEBOUNCE_DURATION: Duration = Duration::from_secs(3);

    // Refresh the related excerpts when the user just beguns editing after
    // an idle period, and after they pause editing.
    fn refresh_context_if_needed(
        &mut self,
        project: &Entity<Project>,
        buffer: &Entity<language::Buffer>,
        cursor_position: language::Anchor,
        cx: &mut Context<Self>,
    ) {
        if !matches!(&self.options().context, ContextMode::Llm { .. }) {
            return;
        }

        let Some(zeta_project) = self.projects.get_mut(&project.entity_id()) else {
            return;
        };

        let now = Instant::now();
        let was_idle = zeta_project
            .refresh_context_timestamp
            .map_or(true, |timestamp| {
                now - timestamp > Self::CONTEXT_RETRIEVAL_IDLE_DURATION
            });
        zeta_project.refresh_context_timestamp = Some(now);
        zeta_project.refresh_context_debounce_task = Some(cx.spawn({
            let buffer = buffer.clone();
            let project = project.clone();
            async move |this, cx| {
                if was_idle {
                    log::debug!("refetching edit prediction context after idle");
                } else {
                    cx.background_executor()
                        .timer(Self::CONTEXT_RETRIEVAL_DEBOUNCE_DURATION)
                        .await;
                    log::debug!("refetching edit prediction context after pause");
                }
                this.update(cx, |this, cx| {
                    this.refresh_context(project, buffer, cursor_position, cx);
                })
                .ok()
            }
        }));
    }

    // Refresh the related excerpts asynchronously. Ensure the task runs to completion,
    // and avoid spawning more than one concurrent task.
    fn refresh_context(
        &mut self,
        project: Entity<Project>,
        buffer: Entity<language::Buffer>,
        cursor_position: language::Anchor,
        cx: &mut Context<Self>,
    ) {
        let Some(zeta_project) = self.projects.get_mut(&project.entity_id()) else {
            return;
        };

        let debug_tx = self.debug_tx.clone();

        zeta_project
            .refresh_context_task
            .get_or_insert(cx.spawn(async move |this, cx| {
                let related_excerpts = this
                    .update(cx, |this, cx| {
                        let Some(zeta_project) = this.projects.get(&project.entity_id()) else {
                            return Task::ready(anyhow::Ok(HashMap::default()));
                        };

                        let ContextMode::Llm(options) = &this.options().context else {
                            return Task::ready(anyhow::Ok(HashMap::default()));
                        };

                        let mut edit_history_unified_diff = String::new();

                        for event in zeta_project.events.iter() {
                            if let Some(event) = event.to_request_event(cx) {
                                writeln!(&mut edit_history_unified_diff, "{event}").ok();
                            }
                        }

                        find_related_excerpts(
                            buffer.clone(),
                            cursor_position,
                            &project,
                            edit_history_unified_diff,
                            options,
                            debug_tx,
                            cx,
                        )
                    })
                    .ok()?
                    .await
                    .log_err()
                    .unwrap_or_default();
                this.update(cx, |this, _cx| {
                    let Some(zeta_project) = this.projects.get_mut(&project.entity_id()) else {
                        return;
                    };
                    zeta_project.context = Some(related_excerpts);
                    zeta_project.refresh_context_task.take();
                    if let Some(debug_tx) = &this.debug_tx {
                        debug_tx
                            .unbounded_send(ZetaDebugInfo::ContextRetrievalFinished(
                                ZetaContextRetrievalDebugInfo {
                                    project,
                                    timestamp: Instant::now(),
                                },
                            ))
                            .ok();
                    }
                })
                .ok()
            }));
    }

    fn gather_nearby_diagnostics(
        cursor_offset: usize,
        diagnostic_sets: &[(LanguageServerId, DiagnosticSet)],
        snapshot: &BufferSnapshot,
        max_diagnostics_bytes: usize,
    ) -> (Vec<predict_edits_v3::DiagnosticGroup>, bool) {
        // TODO: Could make this more efficient
        let mut diagnostic_groups = Vec::new();
        for (language_server_id, diagnostics) in diagnostic_sets {
            let mut groups = Vec::new();
            diagnostics.groups(*language_server_id, &mut groups, &snapshot);
            diagnostic_groups.extend(
                groups
                    .into_iter()
                    .map(|(_, group)| group.resolve::<usize>(&snapshot)),
            );
        }

        // sort by proximity to cursor
        diagnostic_groups.sort_by_key(|group| {
            let range = &group.entries[group.primary_ix].range;
            if range.start >= cursor_offset {
                range.start - cursor_offset
            } else if cursor_offset >= range.end {
                cursor_offset - range.end
            } else {
                (cursor_offset - range.start).min(range.end - cursor_offset)
            }
        });

        let mut results = Vec::new();
        let mut diagnostic_groups_truncated = false;
        let mut diagnostics_byte_count = 0;
        for group in diagnostic_groups {
            let raw_value = serde_json::value::to_raw_value(&group).unwrap();
            diagnostics_byte_count += raw_value.get().len();
            if diagnostics_byte_count > max_diagnostics_bytes {
                diagnostic_groups_truncated = true;
                break;
            }
            results.push(predict_edits_v3::DiagnosticGroup(raw_value));
        }

        (results, diagnostic_groups_truncated)
    }

    // TODO: Dedupe with similar code in request_prediction?
    pub fn cloud_request_for_zeta_cli(
        &mut self,
        project: &Entity<Project>,
        buffer: &Entity<Buffer>,
        position: language::Anchor,
        cx: &mut Context<Self>,
    ) -> Task<Result<predict_edits_v3::PredictEditsRequest>> {
        let project_state = self.projects.get(&project.entity_id());

        let index_state = project_state.map(|state| {
            state
                .syntax_index
                .read_with(cx, |index, _cx| index.state().clone())
        });
        let options = self.options.clone();
        let snapshot = buffer.read(cx).snapshot();
        let Some(excerpt_path) = snapshot.file().map(|path| path.full_path(cx)) else {
            return Task::ready(Err(anyhow!("No file path for excerpt")));
        };
        let worktree_snapshots = project
            .read(cx)
            .worktrees(cx)
            .map(|worktree| worktree.read(cx).snapshot())
            .collect::<Vec<_>>();

        let parent_abs_path = project::File::from_dyn(buffer.read(cx).file()).and_then(|f| {
            let mut path = f.worktree.read(cx).absolutize(&f.path);
            if path.pop() { Some(path) } else { None }
        });

        cx.background_spawn(async move {
            let index_state = if let Some(index_state) = index_state {
                Some(index_state.lock_owned().await)
            } else {
                None
            };

            let cursor_point = position.to_point(&snapshot);

            let debug_info = true;
            EditPredictionContext::gather_context(
                cursor_point,
                &snapshot,
                parent_abs_path.as_deref(),
                match &options.context {
                    ContextMode::Llm(_) => {
                        // TODO
                        panic!("Llm mode not supported in zeta cli yet");
                    }
                    ContextMode::Syntax(edit_prediction_context_options) => {
                        edit_prediction_context_options
                    }
                },
                index_state.as_deref(),
            )
            .context("Failed to select excerpt")
            .map(|context| {
                make_syntax_context_cloud_request(
                    excerpt_path.into(),
                    context,
                    // TODO pass everything
                    Vec::new(),
                    false,
                    Vec::new(),
                    false,
                    None,
                    debug_info,
                    &worktree_snapshots,
                    index_state.as_deref(),
                    Some(options.max_prompt_bytes),
                    options.prompt_format,
                )
            })
        })
    }

    pub fn wait_for_initial_indexing(
        &mut self,
        project: &Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let zeta_project = self.get_or_init_zeta_project(project, cx);
        zeta_project
            .syntax_index
            .read(cx)
            .wait_for_initial_file_indexing(cx)
    }
}

#[derive(Error, Debug)]
#[error(
    "You must update to Zed version {minimum_version} or higher to continue using edit predictions."
)]
pub struct ZedUpdateRequiredError {
    minimum_version: SemanticVersion,
}

fn make_syntax_context_cloud_request(
    excerpt_path: Arc<Path>,
    context: EditPredictionContext,
    events: Vec<predict_edits_v3::Event>,
    can_collect_data: bool,
    diagnostic_groups: Vec<predict_edits_v3::DiagnosticGroup>,
    diagnostic_groups_truncated: bool,
    git_info: Option<cloud_llm_client::PredictEditsGitInfo>,
    debug_info: bool,
    worktrees: &Vec<worktree::Snapshot>,
    index_state: Option<&SyntaxIndexState>,
    prompt_max_bytes: Option<usize>,
    prompt_format: PromptFormat,
) -> predict_edits_v3::PredictEditsRequest {
    let mut signatures = Vec::new();
    let mut declaration_to_signature_index = HashMap::default();
    let mut referenced_declarations = Vec::new();

    for snippet in context.declarations {
        let project_entry_id = snippet.declaration.project_entry_id();
        let Some(path) = worktrees.iter().find_map(|worktree| {
            worktree.entry_for_id(project_entry_id).map(|entry| {
                let mut full_path = RelPathBuf::new();
                full_path.push(worktree.root_name());
                full_path.push(&entry.path);
                full_path
            })
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
            path: path.as_std_path().into(),
            text: text.into(),
            range: snippet.declaration.item_line_range(),
            text_is_truncated,
            signature_range: snippet.declaration.signature_range_in_item_text(),
            parent_index,
            signature_score: snippet.score(DeclarationStyle::Signature),
            declaration_score: snippet.score(DeclarationStyle::Declaration),
            score_components: snippet.components,
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
        excerpt_line_range: context.excerpt.line_range,
        excerpt_range: context.excerpt.range,
        cursor_point: predict_edits_v3::Point {
            line: predict_edits_v3::Line(context.cursor_point.row),
            column: context.cursor_point.column,
        },
        referenced_declarations,
        included_files: vec![],
        signatures,
        excerpt_parent,
        events,
        can_collect_data,
        diagnostic_groups,
        diagnostic_groups_truncated,
        git_info,
        debug_info,
        prompt_max_bytes,
        prompt_format,
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
        range: parent_declaration.signature_line_range(),
    });
    declaration_to_signature_index.insert(declaration_id, signature_index);
    Some(signature_index)
}

#[cfg(test)]
mod tests {
    use std::{
        path::{Path, PathBuf},
        sync::Arc,
    };

    use client::UserStore;
    use clock::FakeSystemClock;
    use cloud_llm_client::predict_edits_v3::{self, Point};
    use edit_prediction_context::Line;
    use futures::{
        AsyncReadExt, StreamExt,
        channel::{mpsc, oneshot},
    };
    use gpui::{
        Entity, TestAppContext,
        http_client::{FakeHttpClient, Response},
        prelude::*,
    };
    use indoc::indoc;
    use language::{LanguageServerId, OffsetRangeExt as _};
    use pretty_assertions::{assert_eq, assert_matches};
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;
    use uuid::Uuid;

    use crate::{BufferEditPrediction, Zeta};

    #[gpui::test]
    async fn test_current_state(cx: &mut TestAppContext) {
        let (zeta, mut req_rx) = init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "1.txt": "Hello!\nHow\nBye",
                "2.txt": "Hola!\nComo\nAdios"
            }),
        )
        .await;
        let project = Project::test(fs, vec![path!("/root").as_ref()], cx).await;

        zeta.update(cx, |zeta, cx| {
            zeta.register_project(&project, cx);
        });

        let buffer1 = project
            .update(cx, |project, cx| {
                let path = project.find_project_path(path!("root/1.txt"), cx).unwrap();
                project.open_buffer(path, cx)
            })
            .await
            .unwrap();
        let snapshot1 = buffer1.read_with(cx, |buffer, _cx| buffer.snapshot());
        let position = snapshot1.anchor_before(language::Point::new(1, 3));

        // Prediction for current file

        let prediction_task = zeta.update(cx, |zeta, cx| {
            zeta.refresh_prediction(&project, &buffer1, position, cx)
        });
        let (_request, respond_tx) = req_rx.next().await.unwrap();
        respond_tx
            .send(predict_edits_v3::PredictEditsResponse {
                request_id: Uuid::new_v4(),
                edits: vec![predict_edits_v3::Edit {
                    path: Path::new(path!("root/1.txt")).into(),
                    range: Line(0)..Line(snapshot1.max_point().row + 1),
                    content: "Hello!\nHow are you?\nBye".into(),
                }],
                debug_info: None,
            })
            .unwrap();
        prediction_task.await.unwrap();

        zeta.read_with(cx, |zeta, cx| {
            let prediction = zeta
                .current_prediction_for_buffer(&buffer1, &project, cx)
                .unwrap();
            assert_matches!(prediction, BufferEditPrediction::Local { .. });
        });

        // Prediction for another file
        let prediction_task = zeta.update(cx, |zeta, cx| {
            zeta.refresh_prediction(&project, &buffer1, position, cx)
        });
        let (_request, respond_tx) = req_rx.next().await.unwrap();
        respond_tx
            .send(predict_edits_v3::PredictEditsResponse {
                request_id: Uuid::new_v4(),
                edits: vec![predict_edits_v3::Edit {
                    path: Path::new(path!("root/2.txt")).into(),
                    range: Line(0)..Line(snapshot1.max_point().row + 1),
                    content: "Hola!\nComo estas?\nAdios".into(),
                }],
                debug_info: None,
            })
            .unwrap();
        prediction_task.await.unwrap();
        zeta.read_with(cx, |zeta, cx| {
            let prediction = zeta
                .current_prediction_for_buffer(&buffer1, &project, cx)
                .unwrap();
            assert_matches!(
                prediction,
                BufferEditPrediction::Jump { prediction } if prediction.path.as_ref() == Path::new(path!("root/2.txt"))
            );
        });

        let buffer2 = project
            .update(cx, |project, cx| {
                let path = project.find_project_path(path!("root/2.txt"), cx).unwrap();
                project.open_buffer(path, cx)
            })
            .await
            .unwrap();

        zeta.read_with(cx, |zeta, cx| {
            let prediction = zeta
                .current_prediction_for_buffer(&buffer2, &project, cx)
                .unwrap();
            assert_matches!(prediction, BufferEditPrediction::Local { .. });
        });
    }

    #[gpui::test]
    async fn test_simple_request(cx: &mut TestAppContext) {
        let (zeta, mut req_rx) = init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "foo.md":  "Hello!\nHow\nBye"
            }),
        )
        .await;
        let project = Project::test(fs, vec![path!("/root").as_ref()], cx).await;

        let buffer = project
            .update(cx, |project, cx| {
                let path = project.find_project_path(path!("root/foo.md"), cx).unwrap();
                project.open_buffer(path, cx)
            })
            .await
            .unwrap();
        let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
        let position = snapshot.anchor_before(language::Point::new(1, 3));

        let prediction_task = zeta.update(cx, |zeta, cx| {
            zeta.request_prediction(&project, &buffer, position, cx)
        });

        let (request, respond_tx) = req_rx.next().await.unwrap();
        assert_eq!(
            request.excerpt_path.as_ref(),
            Path::new(path!("root/foo.md"))
        );
        assert_eq!(
            request.cursor_point,
            Point {
                line: Line(1),
                column: 3
            }
        );

        respond_tx
            .send(predict_edits_v3::PredictEditsResponse {
                request_id: Uuid::new_v4(),
                edits: vec![predict_edits_v3::Edit {
                    path: Path::new(path!("root/foo.md")).into(),
                    range: Line(0)..Line(snapshot.max_point().row + 1),
                    content: "Hello!\nHow are you?\nBye".into(),
                }],
                debug_info: None,
            })
            .unwrap();

        let prediction = prediction_task.await.unwrap().unwrap();

        assert_eq!(prediction.edits.len(), 1);
        assert_eq!(
            prediction.edits[0].0.to_point(&snapshot).start,
            language::Point::new(1, 3)
        );
        assert_eq!(prediction.edits[0].1, " are you?");
    }

    #[gpui::test]
    async fn test_request_events(cx: &mut TestAppContext) {
        let (zeta, mut req_rx) = init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "foo.md": "Hello!\n\nBye"
            }),
        )
        .await;
        let project = Project::test(fs, vec![path!("/root").as_ref()], cx).await;

        let buffer = project
            .update(cx, |project, cx| {
                let path = project.find_project_path(path!("root/foo.md"), cx).unwrap();
                project.open_buffer(path, cx)
            })
            .await
            .unwrap();

        zeta.update(cx, |zeta, cx| {
            zeta.register_buffer(&buffer, &project, cx);
        });

        buffer.update(cx, |buffer, cx| {
            buffer.edit(vec![(7..7, "How")], None, cx);
        });

        let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
        let position = snapshot.anchor_before(language::Point::new(1, 3));

        let prediction_task = zeta.update(cx, |zeta, cx| {
            zeta.request_prediction(&project, &buffer, position, cx)
        });

        let (request, respond_tx) = req_rx.next().await.unwrap();

        assert_eq!(request.events.len(), 1);
        assert_eq!(
            request.events[0],
            predict_edits_v3::Event::BufferChange {
                path: Some(PathBuf::from(path!("root/foo.md"))),
                old_path: None,
                diff: indoc! {"
                        @@ -1,3 +1,3 @@
                         Hello!
                        -
                        +How
                         Bye
                    "}
                .to_string(),
                predicted: false
            }
        );

        respond_tx
            .send(predict_edits_v3::PredictEditsResponse {
                request_id: Uuid::new_v4(),
                edits: vec![predict_edits_v3::Edit {
                    path: Path::new(path!("root/foo.md")).into(),
                    range: Line(0)..Line(snapshot.max_point().row + 1),
                    content: "Hello!\nHow are you?\nBye".into(),
                }],
                debug_info: None,
            })
            .unwrap();

        let prediction = prediction_task.await.unwrap().unwrap();

        assert_eq!(prediction.edits.len(), 1);
        assert_eq!(
            prediction.edits[0].0.to_point(&snapshot).start,
            language::Point::new(1, 3)
        );
        assert_eq!(prediction.edits[0].1, " are you?");
    }

    #[gpui::test]
    async fn test_request_diagnostics(cx: &mut TestAppContext) {
        let (zeta, mut req_rx) = init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "foo.md": "Hello!\nBye"
            }),
        )
        .await;
        let project = Project::test(fs, vec![path!("/root").as_ref()], cx).await;

        let path_to_buffer_uri = lsp::Uri::from_file_path(path!("/root/foo.md")).unwrap();
        let diagnostic = lsp::Diagnostic {
            range: lsp::Range::new(lsp::Position::new(1, 1), lsp::Position::new(1, 5)),
            severity: Some(lsp::DiagnosticSeverity::ERROR),
            message: "\"Hello\" deprecated. Use \"Hi\" instead".to_string(),
            ..Default::default()
        };

        project.update(cx, |project, cx| {
            project.lsp_store().update(cx, |lsp_store, cx| {
                // Create some diagnostics
                lsp_store
                    .update_diagnostics(
                        LanguageServerId(0),
                        lsp::PublishDiagnosticsParams {
                            uri: path_to_buffer_uri.clone(),
                            diagnostics: vec![diagnostic],
                            version: None,
                        },
                        None,
                        language::DiagnosticSourceKind::Pushed,
                        &[],
                        cx,
                    )
                    .unwrap();
            });
        });

        let buffer = project
            .update(cx, |project, cx| {
                let path = project.find_project_path(path!("root/foo.md"), cx).unwrap();
                project.open_buffer(path, cx)
            })
            .await
            .unwrap();

        let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
        let position = snapshot.anchor_before(language::Point::new(0, 0));

        let _prediction_task = zeta.update(cx, |zeta, cx| {
            zeta.request_prediction(&project, &buffer, position, cx)
        });

        let (request, _respond_tx) = req_rx.next().await.unwrap();

        assert_eq!(request.diagnostic_groups.len(), 1);
        let value = serde_json::from_str::<serde_json::Value>(request.diagnostic_groups[0].0.get())
            .unwrap();
        // We probably don't need all of this. TODO define a specific diagnostic type in predict_edits_v3
        assert_eq!(
            value,
            json!({
                "entries": [{
                    "range": {
                        "start": 8,
                        "end": 10
                    },
                    "diagnostic": {
                        "source": null,
                        "code": null,
                        "code_description": null,
                        "severity": 1,
                        "message": "\"Hello\" deprecated. Use \"Hi\" instead",
                        "markdown": null,
                        "group_id": 0,
                        "is_primary": true,
                        "is_disk_based": false,
                        "is_unnecessary": false,
                        "source_kind": "Pushed",
                        "data": null,
                        "underline": true
                    }
                }],
                "primary_ix": 0
            })
        );
    }

    fn init_test(
        cx: &mut TestAppContext,
    ) -> (
        Entity<Zeta>,
        mpsc::UnboundedReceiver<(
            predict_edits_v3::PredictEditsRequest,
            oneshot::Sender<predict_edits_v3::PredictEditsResponse>,
        )>,
    ) {
        cx.update(move |cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);

            let (req_tx, req_rx) = mpsc::unbounded();

            let http_client = FakeHttpClient::create({
                move |req| {
                    let uri = req.uri().path().to_string();
                    let mut body = req.into_body();
                    let req_tx = req_tx.clone();
                    async move {
                        let resp = match uri.as_str() {
                            "/client/llm_tokens" => serde_json::to_string(&json!({
                                "token": "test"
                            }))
                            .unwrap(),
                            "/predict_edits/v3" => {
                                let mut buf = Vec::new();
                                body.read_to_end(&mut buf).await.ok();
                                let req = serde_json::from_slice(&buf).unwrap();

                                let (res_tx, res_rx) = oneshot::channel();
                                req_tx.unbounded_send((req, res_tx)).unwrap();
                                serde_json::to_string(&res_rx.await?).unwrap()
                            }
                            _ => {
                                panic!("Unexpected path: {}", uri)
                            }
                        };

                        Ok(Response::builder().body(resp.into()).unwrap())
                    }
                }
            });

            let client = client::Client::new(Arc::new(FakeSystemClock::new()), http_client, cx);
            client.cloud_client().set_credentials(1, "test".into());

            language_model::init(client.clone(), cx);

            let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
            let zeta = Zeta::global(&client, &user_store, cx);

            (zeta, req_rx)
        })
    }
}
