use anyhow::{Context as _, Result, anyhow, bail};
use arrayvec::ArrayVec;
use client::{Client, EditPredictionUsage, UserStore};
use cloud_llm_client::predict_edits_v3::{self, Event, PromptFormat};
use cloud_llm_client::{
    AcceptEditPredictionBody, EXPIRED_LLM_TOKEN_HEADER_NAME, EditPredictionRejectReason,
    EditPredictionRejection, MAX_EDIT_PREDICTION_REJECTIONS_PER_REQUEST,
    MINIMUM_REQUIRED_VERSION_HEADER_NAME, PredictEditsRequestTrigger, RejectEditPredictionsBodyRef,
    ZED_VERSION_HEADER_NAME,
};
use cloud_zeta2_prompt::retrieval_prompt::{SearchToolInput, SearchToolQuery};
use cloud_zeta2_prompt::{CURSOR_MARKER, DEFAULT_MAX_PROMPT_BYTES};
use collections::{HashMap, HashSet};
use command_palette_hooks::CommandPaletteFilter;
use db::kvp::{Dismissable, KEY_VALUE_STORE};
use edit_prediction_context::{
    EditPredictionContextOptions, EditPredictionExcerpt, EditPredictionExcerptOptions,
    EditPredictionScoreOptions, Line, SyntaxIndex,
};
use edit_prediction_context2::{
    RelatedExcerpt, RelatedExcerptStore, RelatedExcerptStoreEvent, RelatedFile,
};
use feature_flags::{FeatureFlag, FeatureFlagAppExt as _, PredictEditsRateCompletionsFeatureFlag};
use futures::{
    AsyncReadExt as _, FutureExt as _, StreamExt as _,
    channel::{
        mpsc::{self, UnboundedReceiver},
        oneshot,
    },
    select_biased,
};
use gpui::BackgroundExecutor;
use gpui::{
    App, AsyncApp, Entity, EntityId, Global, SharedString, Subscription, Task, WeakEntity, actions,
    http_client::{self, AsyncBody, Method},
    prelude::*,
};
use language::language_settings::all_language_settings;
use language::{
    Anchor, Buffer, DiagnosticSet, File, LanguageServerId, Point, ToOffset as _, ToPoint,
};
use language::{BufferSnapshot, OffsetRangeExt};
use language_model::{LlmApiToken, RefreshLlmTokenListener};
use open_ai::FunctionDefinition;
use project::{DisableAiSettings, Project, ProjectItem as _, ProjectPath, WorktreeId};
use release_channel::AppVersion;
use semver::Version;
use serde::de::DeserializeOwned;
use settings::{EditPredictionProvider, Settings, SettingsStore, update_settings_file};
use std::any::{Any as _, TypeId};
use std::collections::{VecDeque, hash_map};
use telemetry_events::EditPredictionRating;
use workspace::Workspace;

use std::ops::Range;
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr as _;
use std::sync::{Arc, LazyLock};
use std::time::{Duration, Instant};
use std::{env, mem};
use thiserror::Error;
use util::{LogErrorFuture, RangeExt as _, ResultExt as _, TryFutureExt};
use workspace::notifications::{ErrorMessagePrompt, NotificationId, show_app_notification};

mod license_detection;
mod onboarding_modal;
mod prediction;
mod provider;
mod rate_prediction_modal;
pub mod retrieval_search;
pub mod sweep_ai;
pub mod udiff;
mod xml_edits;
pub mod zeta1;

#[cfg(test)]
mod zeta_tests;

use crate::license_detection::LicenseDetectionWatcher;
use crate::onboarding_modal::ZedPredictModal;
pub use crate::prediction::EditPrediction;
pub use crate::prediction::EditPredictionId;
pub use crate::prediction::EditPredictionInputs;
use crate::prediction::EditPredictionResult;
use crate::rate_prediction_modal::{
    NextEdit, PreviousEdit, RatePredictionsModal, ThumbsDownActivePrediction,
    ThumbsUpActivePrediction,
};
pub use crate::sweep_ai::SweepAi;
use crate::zeta1::request_prediction_with_zeta1;
pub use provider::ZetaEditPredictionProvider;

actions!(
    edit_prediction,
    [
        /// Resets the edit prediction onboarding state.
        ResetOnboarding,
        /// Opens the rate completions modal.
        RateCompletions,
        /// Clears the edit prediction history.
        ClearHistory,
    ]
);

/// Maximum number of events to track.
const EVENT_COUNT_MAX: usize = 6;
const CHANGE_GROUPING_LINE_SPAN: u32 = 8;
const ZED_PREDICT_DATA_COLLECTION_CHOICE: &str = "zed_predict_data_collection_choice";
const REJECT_REQUEST_DEBOUNCE: Duration = Duration::from_secs(15);

pub struct SweepFeatureFlag;

impl FeatureFlag for SweepFeatureFlag {
    const NAME: &str = "sweep-ai";
}
pub const DEFAULT_EXCERPT_OPTIONS: EditPredictionExcerptOptions = EditPredictionExcerptOptions {
    max_bytes: 512,
    min_bytes: 128,
    target_before_cursor_over_total_bytes: 0.5,
};

pub const DEFAULT_CONTEXT_OPTIONS: ContextMode = ContextMode::Lsp(DEFAULT_EXCERPT_OPTIONS);

pub const DEFAULT_AGENTIC_CONTEXT_OPTIONS: AgenticContextOptions = AgenticContextOptions {
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
    buffer_change_grouping_interval: Duration::from_secs(1),
};

static USE_OLLAMA: LazyLock<bool> =
    LazyLock::new(|| env::var("ZED_ZETA2_OLLAMA").is_ok_and(|var| !var.is_empty()));
static CONTEXT_RETRIEVAL_MODEL_ID: LazyLock<String> = LazyLock::new(|| {
    env::var("ZED_ZETA2_CONTEXT_MODEL").unwrap_or(if *USE_OLLAMA {
        "qwen3-coder:30b".to_string()
    } else {
        "yqvev8r3".to_string()
    })
});
static EDIT_PREDICTIONS_MODEL_ID: LazyLock<String> = LazyLock::new(|| {
    match env::var("ZED_ZETA2_MODEL").as_deref() {
        Ok("zeta2-exp") => "4w5n28vw", // Fine-tuned model @ Baseten
        Ok(model) => model,
        Err(_) if *USE_OLLAMA => "qwen3-coder:30b",
        Err(_) => "yqvev8r3", // Vanilla qwen3-coder @ Baseten
    }
    .to_string()
});
static PREDICT_EDITS_URL: LazyLock<Option<String>> = LazyLock::new(|| {
    env::var("ZED_PREDICT_EDITS_URL").ok().or_else(|| {
        if *USE_OLLAMA {
            Some("http://localhost:11434/v1/chat/completions".into())
        } else {
            None
        }
    })
});

pub struct Zeta2FeatureFlag;

impl FeatureFlag for Zeta2FeatureFlag {
    const NAME: &'static str = "zeta2";

    fn enabled_for_staff() -> bool {
        true
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
    use_context: bool,
    options: ZetaOptions,
    update_required: bool,
    debug_tx: Option<mpsc::UnboundedSender<ZetaDebugInfo>>,
    #[cfg(feature = "eval-support")]
    eval_cache: Option<Arc<dyn EvalCache>>,
    edit_prediction_model: ZetaEditPredictionModel,
    pub sweep_ai: SweepAi,
    data_collection_choice: DataCollectionChoice,
    reject_predictions_tx: mpsc::UnboundedSender<EditPredictionRejection>,
    shown_predictions: VecDeque<EditPrediction>,
    rated_predictions: HashSet<EditPredictionId>,
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub enum ZetaEditPredictionModel {
    #[default]
    Zeta1,
    Zeta2,
    Sweep,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ZetaOptions {
    pub context: ContextMode,
    pub max_prompt_bytes: usize,
    pub max_diagnostic_bytes: usize,
    pub prompt_format: predict_edits_v3::PromptFormat,
    pub file_indexing_parallelism: usize,
    pub buffer_change_grouping_interval: Duration,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextMode {
    Agentic(AgenticContextOptions),
    Syntax(EditPredictionContextOptions),
    Lsp(EditPredictionExcerptOptions),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AgenticContextOptions {
    pub excerpt: EditPredictionExcerptOptions,
}

impl ContextMode {
    pub fn excerpt(&self) -> &EditPredictionExcerptOptions {
        match self {
            ContextMode::Agentic(options) => &options.excerpt,
            ContextMode::Syntax(options) => &options.excerpt,
            ContextMode::Lsp(options) => &options,
        }
    }
}

#[derive(Debug)]
pub enum ZetaDebugInfo {
    ContextRetrievalStarted(ZetaContextRetrievalStartedDebugInfo),
    ContextRetrievalFinished(ZetaContextRetrievalFinishedDebugInfo),
    EditPredictionRequested(ZetaEditPredictionDebugInfo),
}

#[derive(Debug)]
pub struct ZetaContextRetrievalStartedDebugInfo {
    pub project_entity_id: EntityId,
    pub timestamp: Instant,
    pub search_prompt: String,
}

#[derive(Debug)]
pub struct ZetaContextRetrievalFinishedDebugInfo {
    pub project_entity_id: EntityId,
    pub timestamp: Instant,
    pub metadata: Vec<(&'static str, SharedString)>,
}

#[derive(Debug)]
pub struct ZetaEditPredictionDebugInfo {
    pub inputs: EditPredictionInputs,
    pub retrieval_time: Duration,
    pub buffer: WeakEntity<Buffer>,
    pub position: language::Anchor,
    pub local_prompt: Result<String, String>,
    pub response_rx: oneshot::Receiver<(Result<open_ai::Response, String>, Duration)>,
}

pub type RequestDebugInfo = predict_edits_v3::DebugInfo;

struct ZetaProject {
    events: VecDeque<Arc<cloud_llm_client::predict_edits_v3::Event>>,
    last_event: Option<LastEvent>,
    recent_paths: VecDeque<ProjectPath>,
    registered_buffers: HashMap<gpui::EntityId, RegisteredBuffer>,
    current_prediction: Option<CurrentEditPrediction>,
    next_pending_prediction_id: usize,
    pending_predictions: ArrayVec<PendingPrediction, 2>,
    context_updates_tx: smol::channel::Sender<()>,
    context_updates_rx: smol::channel::Receiver<()>,
    last_prediction_refresh: Option<(EntityId, Instant)>,
    cancelled_predictions: HashSet<usize>,
    context: ZetaProjectContext,
    license_detection_watchers: HashMap<WorktreeId, Rc<LicenseDetectionWatcher>>,
    _subscription: gpui::Subscription,
}

enum ZetaProjectContext {
    Syntax(Entity<SyntaxIndex>),
    Lsp(Entity<RelatedExcerptStore>),
    Agentic {
        refresh_context_task: Option<LogErrorFuture<Task<Result<()>>>>,
        refresh_context_debounce_task: Option<Task<Option<()>>>,
        refresh_context_timestamp: Option<Instant>,
        context: Vec<RelatedFile>,
    },
}

impl ZetaProject {
    pub fn events(&self, cx: &App) -> Vec<Arc<cloud_llm_client::predict_edits_v3::Event>> {
        self.events
            .iter()
            .cloned()
            .chain(
                self.last_event
                    .as_ref()
                    .and_then(|event| event.finalize(&self.license_detection_watchers, cx)),
            )
            .collect()
    }

    fn cancel_pending_prediction(
        &mut self,
        pending_prediction: PendingPrediction,
        cx: &mut Context<Zeta>,
    ) {
        self.cancelled_predictions.insert(pending_prediction.id);

        cx.spawn(async move |this, cx| {
            let Some(prediction_id) = pending_prediction.task.await else {
                return;
            };

            this.update(cx, |this, _cx| {
                this.reject_prediction(prediction_id, EditPredictionRejectReason::Canceled, false);
            })
            .ok();
        })
        .detach()
    }
}

#[derive(Debug, Clone)]
struct CurrentEditPrediction {
    pub requested_by: PredictionRequestedBy,
    pub prediction: EditPrediction,
    pub was_shown: bool,
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

        let requested_by_buffer_id = self.requested_by.buffer_id();

        // This reduces the occurrence of UI thrash from replacing edits
        //
        // TODO: This is fairly arbitrary - should have a more general heuristic that handles multiple edits.
        if requested_by_buffer_id == Some(self.prediction.buffer.entity_id())
            && requested_by_buffer_id == Some(old_prediction.prediction.buffer.entity_id())
            && old_edits.len() == 1
            && new_edits.len() == 1
        {
            let (old_range, old_text) = &old_edits[0];
            let (new_range, new_text) = &new_edits[0];
            new_range == old_range && new_text.starts_with(old_text.as_ref())
        } else {
            true
        }
    }
}

#[derive(Debug, Clone)]
enum PredictionRequestedBy {
    DiagnosticsUpdate,
    Buffer(EntityId),
}

impl PredictionRequestedBy {
    pub fn buffer_id(&self) -> Option<EntityId> {
        match self {
            PredictionRequestedBy::DiagnosticsUpdate => None,
            PredictionRequestedBy::Buffer(buffer_id) => Some(*buffer_id),
        }
    }
}

#[derive(Debug)]
struct PendingPrediction {
    id: usize,
    task: Task<Option<EditPredictionId>>,
}

/// A prediction from the perspective of a buffer.
#[derive(Debug)]
enum BufferEditPrediction<'a> {
    Local { prediction: &'a EditPrediction },
    Jump { prediction: &'a EditPrediction },
}

#[cfg(test)]
impl std::ops::Deref for BufferEditPrediction<'_> {
    type Target = EditPrediction;

    fn deref(&self) -> &Self::Target {
        match self {
            BufferEditPrediction::Local { prediction } => prediction,
            BufferEditPrediction::Jump { prediction } => prediction,
        }
    }
}

struct RegisteredBuffer {
    snapshot: BufferSnapshot,
    _subscriptions: [gpui::Subscription; 2],
}

struct LastEvent {
    old_snapshot: BufferSnapshot,
    new_snapshot: BufferSnapshot,
    end_edit_anchor: Option<Anchor>,
}

impl LastEvent {
    pub fn finalize(
        &self,
        license_detection_watchers: &HashMap<WorktreeId, Rc<LicenseDetectionWatcher>>,
        cx: &App,
    ) -> Option<Arc<predict_edits_v3::Event>> {
        let path = buffer_path_with_id_fallback(&self.new_snapshot, cx);
        let old_path = buffer_path_with_id_fallback(&self.old_snapshot, cx);

        let file = self.new_snapshot.file();
        let old_file = self.old_snapshot.file();

        let in_open_source_repo = [file, old_file].iter().all(|file| {
            file.is_some_and(|file| {
                license_detection_watchers
                    .get(&file.worktree_id(cx))
                    .is_some_and(|watcher| watcher.is_project_open_source())
            })
        });

        let diff = language::unified_diff(&self.old_snapshot.text(), &self.new_snapshot.text());

        if path == old_path && diff.is_empty() {
            None
        } else {
            Some(Arc::new(predict_edits_v3::Event::BufferChange {
                old_path,
                path,
                diff,
                in_open_source_repo,
                // TODO: Actually detect if this edit was predicted or not
                predicted: false,
            }))
        }
    }
}

fn buffer_path_with_id_fallback(snapshot: &BufferSnapshot, cx: &App) -> Arc<Path> {
    if let Some(file) = snapshot.file() {
        file.full_path(cx).into()
    } else {
        Path::new(&format!("untitled-{}", snapshot.remote_id())).into()
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
        let data_collection_choice = Self::load_data_collection_choice();

        let llm_token = LlmApiToken::default();

        let (reject_tx, reject_rx) = mpsc::unbounded();
        cx.background_spawn({
            let client = client.clone();
            let llm_token = llm_token.clone();
            let app_version = AppVersion::global(cx);
            let background_executor = cx.background_executor().clone();
            async move {
                Self::handle_rejected_predictions(
                    reject_rx,
                    client,
                    llm_token,
                    app_version,
                    background_executor,
                )
                .await
            }
        })
        .detach();

        let mut this = Self {
            projects: HashMap::default(),
            client,
            user_store,
            options: DEFAULT_OPTIONS,
            use_context: false,
            llm_token,
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
            #[cfg(feature = "eval-support")]
            eval_cache: None,
            edit_prediction_model: ZetaEditPredictionModel::Zeta2,
            sweep_ai: SweepAi::new(cx),
            data_collection_choice,
            reject_predictions_tx: reject_tx,
            rated_predictions: Default::default(),
            shown_predictions: Default::default(),
        };

        this.enable_or_disable_context_retrieval(cx);
        let weak_this = cx.weak_entity();
        cx.on_flags_ready(move |_, cx| {
            weak_this
                .update(cx, |this, cx| this.enable_or_disable_context_retrieval(cx))
                .ok();
        })
        .detach();
        cx.observe_global::<SettingsStore>(|this, cx| {
            this.enable_or_disable_context_retrieval(cx);
        })
        .detach();

        this
    }

    pub fn set_edit_prediction_model(&mut self, model: ZetaEditPredictionModel) {
        self.edit_prediction_model = model;
    }

    pub fn has_sweep_api_token(&self) -> bool {
        self.sweep_ai
            .api_token
            .clone()
            .now_or_never()
            .flatten()
            .is_some()
    }

    #[cfg(feature = "eval-support")]
    pub fn with_eval_cache(&mut self, cache: Arc<dyn EvalCache>) {
        self.eval_cache = Some(cache);
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

    pub fn set_use_context(&mut self, use_context: bool) {
        self.use_context = use_context;
    }

    pub fn clear_history(&mut self) {
        for zeta_project in self.projects.values_mut() {
            zeta_project.events.clear();
        }
    }

    pub fn context_for_project<'a>(
        &'a self,
        project: &Entity<Project>,
        cx: &'a App,
    ) -> &'a [RelatedFile] {
        self.projects
            .get(&project.entity_id())
            .and_then(|project| match &project.context {
                ZetaProjectContext::Syntax(_) => None,
                ZetaProjectContext::Lsp(store) => Some(store.read(cx).related_files()),
                ZetaProjectContext::Agentic { context, .. } => Some(context.as_slice()),
            })
            .unwrap_or(&[])
    }

    pub fn usage(&self, cx: &App) -> Option<EditPredictionUsage> {
        if self.edit_prediction_model == ZetaEditPredictionModel::Zeta2 {
            self.user_store.read(cx).edit_prediction_usage()
        } else {
            None
        }
    }

    pub fn register_project(&mut self, project: &Entity<Project>, cx: &mut Context<Self>) {
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
        cx: &mut Context<Self>,
    ) -> &mut ZetaProject {
        let entity_id = project.entity_id();
        let (context_updates_tx, context_updates_rx) = smol::channel::unbounded();
        self.projects
            .entry(entity_id)
            .or_insert_with(|| ZetaProject {
                context: match &self.options.context {
                    ContextMode::Agentic(_) => ZetaProjectContext::Agentic {
                        refresh_context_task: None,
                        refresh_context_debounce_task: None,
                        refresh_context_timestamp: None,
                        context: Vec::new(),
                    },
                    ContextMode::Syntax(_) => ZetaProjectContext::Syntax(cx.new(|cx| {
                        SyntaxIndex::new(project, self.options.file_indexing_parallelism, cx)
                    })),
                    ContextMode::Lsp(_) => {
                        let related_excerpt_store =
                            cx.new(|cx| RelatedExcerptStore::new(project, cx));
                        cx.subscribe(
                            &related_excerpt_store,
                            move |this, _, event, _| match event {
                                RelatedExcerptStoreEvent::StartedRefresh => {
                                    if let Some(debug_tx) = this.debug_tx.clone() {
                                        debug_tx
                                            .unbounded_send(ZetaDebugInfo::ContextRetrievalStarted(
                                                ZetaContextRetrievalStartedDebugInfo {
                                                    project_entity_id: entity_id,
                                                    timestamp: Instant::now(),
                                                    search_prompt: String::new(),
                                                },
                                            ))
                                            .ok();
                                    }
                                }
                                RelatedExcerptStoreEvent::FinishedRefresh {
                                    cache_hit_count,
                                    cache_miss_count,
                                    mean_definition_latency,
                                    max_definition_latency,
                                } => {
                                    if let Some(debug_tx) = this.debug_tx.clone() {
                                        debug_tx
                                            .unbounded_send(
                                                ZetaDebugInfo::ContextRetrievalFinished(
                                                    ZetaContextRetrievalFinishedDebugInfo {
                                                        project_entity_id: entity_id,
                                                        timestamp: Instant::now(),
                                                        metadata: vec![
                                                            (
                                                                "Cache Hits",
                                                                format!(
                                                                    "{}/{}",
                                                                    cache_hit_count,
                                                                    cache_hit_count
                                                                        + cache_miss_count
                                                                )
                                                                .into(),
                                                            ),
                                                            (
                                                                "Max LSP Time",
                                                                format!(
                                                                    "{} ms",
                                                                    max_definition_latency
                                                                        .as_millis()
                                                                )
                                                                .into(),
                                                            ),
                                                            (
                                                                "Mean LSP Time",
                                                                format!(
                                                                    "{} ms",
                                                                    mean_definition_latency
                                                                        .as_millis()
                                                                )
                                                                .into(),
                                                            ),
                                                        ],
                                                    },
                                                ),
                                            )
                                            .ok();
                                    }
                                    if let Some(project_state) = this.projects.get(&entity_id) {
                                        project_state.context_updates_tx.send_blocking(()).ok();
                                    }
                                }
                            },
                        )
                        .detach();
                        ZetaProjectContext::Lsp(related_excerpt_store)
                    }
                },
                events: VecDeque::new(),
                last_event: None,
                recent_paths: VecDeque::new(),
                context_updates_rx,
                context_updates_tx,
                registered_buffers: HashMap::default(),
                current_prediction: None,
                cancelled_predictions: HashSet::default(),
                pending_predictions: ArrayVec::new(),
                next_pending_prediction_id: 0,
                last_prediction_refresh: None,
                license_detection_watchers: HashMap::default(),
                _subscription: cx.subscribe(&project, Self::handle_project_event),
            })
    }

    pub fn project_context_updates(
        &self,
        project: &Entity<Project>,
    ) -> Option<smol::channel::Receiver<()>> {
        let project_state = self.projects.get(&project.entity_id())?;
        Some(project_state.context_updates_rx.clone())
    }

    fn handle_project_event(
        &mut self,
        project: Entity<Project>,
        event: &project::Event,
        cx: &mut Context<Self>,
    ) {
        // TODO [zeta2] init with recent paths
        match event {
            project::Event::ActiveEntryChanged(Some(active_entry_id)) => {
                let Some(zeta_project) = self.projects.get_mut(&project.entity_id()) else {
                    return;
                };
                let path = project.read(cx).path_for_entry(*active_entry_id, cx);
                if let Some(path) = path {
                    if let Some(ix) = zeta_project
                        .recent_paths
                        .iter()
                        .position(|probe| probe == &path)
                    {
                        zeta_project.recent_paths.remove(ix);
                    }
                    zeta_project.recent_paths.push_front(path);
                }
            }
            project::Event::DiagnosticsUpdated { .. } => {
                if cx.has_flag::<Zeta2FeatureFlag>() {
                    self.refresh_prediction_from_diagnostics(project, cx);
                }
            }
            _ => (),
        }
    }

    fn register_buffer_impl<'a>(
        zeta_project: &'a mut ZetaProject,
        buffer: &Entity<Buffer>,
        project: &Entity<Project>,
        cx: &mut Context<Self>,
    ) -> &'a mut RegisteredBuffer {
        let buffer_id = buffer.entity_id();

        if let Some(file) = buffer.read(cx).file() {
            let worktree_id = file.worktree_id(cx);
            if let Some(worktree) = project.read(cx).worktree_for_id(worktree_id, cx) {
                zeta_project
                    .license_detection_watchers
                    .entry(worktree_id)
                    .or_insert_with(|| {
                        let project_entity_id = project.entity_id();
                        cx.observe_release(&worktree, move |this, _worktree, _cx| {
                            let Some(zeta_project) = this.projects.get_mut(&project_entity_id)
                            else {
                                return;
                            };
                            zeta_project.license_detection_watchers.remove(&worktree_id);
                        })
                        .detach();
                        Rc::new(LicenseDetectionWatcher::new(&worktree, cx))
                    });
            }
        }

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
    ) {
        let project_state = self.get_or_init_zeta_project(project, cx);
        let registered_buffer = Self::register_buffer_impl(project_state, buffer, project, cx);

        let new_snapshot = buffer.read(cx).snapshot();
        if new_snapshot.version == registered_buffer.snapshot.version {
            return;
        }

        let old_snapshot = mem::replace(&mut registered_buffer.snapshot, new_snapshot.clone());
        let end_edit_anchor = new_snapshot
            .anchored_edits_since::<Point>(&old_snapshot.version)
            .last()
            .map(|(_, range)| range.end);
        let events = &mut project_state.events;

        if let Some(LastEvent {
            new_snapshot: last_new_snapshot,
            end_edit_anchor: last_end_edit_anchor,
            ..
        }) = project_state.last_event.as_mut()
        {
            let is_next_snapshot_of_same_buffer = old_snapshot.remote_id()
                == last_new_snapshot.remote_id()
                && old_snapshot.version == last_new_snapshot.version;

            let should_coalesce = is_next_snapshot_of_same_buffer
                && end_edit_anchor
                    .as_ref()
                    .zip(last_end_edit_anchor.as_ref())
                    .is_some_and(|(a, b)| {
                        let a = a.to_point(&new_snapshot);
                        let b = b.to_point(&new_snapshot);
                        a.row.abs_diff(b.row) <= CHANGE_GROUPING_LINE_SPAN
                    });

            if should_coalesce {
                *last_end_edit_anchor = end_edit_anchor;
                *last_new_snapshot = new_snapshot;
                return;
            }
        }

        if events.len() + 1 >= EVENT_COUNT_MAX {
            events.pop_front();
        }

        if let Some(event) = project_state.last_event.take() {
            events.extend(event.finalize(&project_state.license_detection_watchers, cx));
        }

        project_state.last_event = Some(LastEvent {
            old_snapshot,
            new_snapshot,
            end_edit_anchor,
        });
    }

    fn current_prediction_for_buffer(
        &self,
        buffer: &Entity<Buffer>,
        project: &Entity<Project>,
        cx: &App,
    ) -> Option<BufferEditPrediction<'_>> {
        let project_state = self.projects.get(&project.entity_id())?;

        let CurrentEditPrediction {
            requested_by,
            prediction,
            ..
        } = project_state.current_prediction.as_ref()?;

        if prediction.targets_buffer(buffer.read(cx)) {
            Some(BufferEditPrediction::Local { prediction })
        } else {
            let show_jump = match requested_by {
                PredictionRequestedBy::Buffer(requested_by_buffer_id) => {
                    requested_by_buffer_id == &buffer.entity_id()
                }
                PredictionRequestedBy::DiagnosticsUpdate => true,
            };

            if show_jump {
                Some(BufferEditPrediction::Jump { prediction })
            } else {
                None
            }
        }
    }

    fn accept_current_prediction(&mut self, project: &Entity<Project>, cx: &mut Context<Self>) {
        match self.edit_prediction_model {
            ZetaEditPredictionModel::Zeta1 | ZetaEditPredictionModel::Zeta2 => {}
            ZetaEditPredictionModel::Sweep => return,
        }

        let Some(project_state) = self.projects.get_mut(&project.entity_id()) else {
            return;
        };

        let Some(prediction) = project_state.current_prediction.take() else {
            return;
        };
        let request_id = prediction.prediction.id.to_string();
        for pending_prediction in mem::take(&mut project_state.pending_predictions) {
            project_state.cancel_pending_prediction(pending_prediction, cx);
        }

        let client = self.client.clone();
        let llm_token = self.llm_token.clone();
        let app_version = AppVersion::global(cx);
        cx.spawn(async move |this, cx| {
            let url = if let Ok(predict_edits_url) = env::var("ZED_ACCEPT_PREDICTION_URL") {
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
                ))
                .await;

            Self::handle_api_response(&this, response, cx)?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    async fn handle_rejected_predictions(
        rx: UnboundedReceiver<EditPredictionRejection>,
        client: Arc<Client>,
        llm_token: LlmApiToken,
        app_version: Version,
        background_executor: BackgroundExecutor,
    ) {
        let mut rx = std::pin::pin!(rx.peekable());
        let mut batched = Vec::new();

        while let Some(rejection) = rx.next().await {
            batched.push(rejection);

            if batched.len() < MAX_EDIT_PREDICTION_REJECTIONS_PER_REQUEST / 2 {
                select_biased! {
                    next = rx.as_mut().peek().fuse() => {
                        if next.is_some() {
                            continue;
                        }
                    }
                    () = background_executor.timer(REJECT_REQUEST_DEBOUNCE).fuse() => {},
                }
            }

            let url = client
                .http_client()
                .build_zed_llm_url("/predict_edits/reject", &[])
                .unwrap();

            let flush_count = batched
                .len()
                // in case items have accumulated after failure
                .min(MAX_EDIT_PREDICTION_REJECTIONS_PER_REQUEST);
            let start = batched.len() - flush_count;

            let body = RejectEditPredictionsBodyRef {
                rejections: &batched[start..],
            };

            let result = Self::send_api_request::<()>(
                |builder| {
                    let req = builder
                        .uri(url.as_ref())
                        .body(serde_json::to_string(&body)?.into());
                    anyhow::Ok(req?)
                },
                client.clone(),
                llm_token.clone(),
                app_version.clone(),
            )
            .await;

            if result.log_err().is_some() {
                batched.drain(start..);
            }
        }
    }

    fn reject_current_prediction(
        &mut self,
        reason: EditPredictionRejectReason,
        project: &Entity<Project>,
    ) {
        if let Some(project_state) = self.projects.get_mut(&project.entity_id()) {
            project_state.pending_predictions.clear();
            if let Some(prediction) = project_state.current_prediction.take() {
                self.reject_prediction(prediction.prediction.id, reason, prediction.was_shown);
            }
        };
    }

    fn did_show_current_prediction(&mut self, project: &Entity<Project>, _cx: &mut Context<Self>) {
        if let Some(project_state) = self.projects.get_mut(&project.entity_id()) {
            if let Some(current_prediction) = project_state.current_prediction.as_mut() {
                if !current_prediction.was_shown {
                    current_prediction.was_shown = true;
                    self.shown_predictions
                        .push_front(current_prediction.prediction.clone());
                    if self.shown_predictions.len() > 50 {
                        let completion = self.shown_predictions.pop_back().unwrap();
                        self.rated_predictions.remove(&completion.id);
                    }
                }
            }
        }
    }

    fn reject_prediction(
        &mut self,
        prediction_id: EditPredictionId,
        reason: EditPredictionRejectReason,
        was_shown: bool,
    ) {
        match self.edit_prediction_model {
            ZetaEditPredictionModel::Zeta1 | ZetaEditPredictionModel::Zeta2 => {}
            ZetaEditPredictionModel::Sweep => return,
        }

        self.reject_predictions_tx
            .unbounded_send(EditPredictionRejection {
                request_id: prediction_id.to_string(),
                reason,
                was_shown,
            })
            .log_err();
    }

    fn is_refreshing(&self, project: &Entity<Project>) -> bool {
        self.projects
            .get(&project.entity_id())
            .is_some_and(|project_state| !project_state.pending_predictions.is_empty())
    }

    pub fn refresh_prediction_from_buffer(
        &mut self,
        project: Entity<Project>,
        buffer: Entity<Buffer>,
        position: language::Anchor,
        cx: &mut Context<Self>,
    ) {
        self.queue_prediction_refresh(project.clone(), buffer.entity_id(), cx, move |this, cx| {
            let Some(request_task) = this
                .update(cx, |this, cx| {
                    this.request_prediction(
                        &project,
                        &buffer,
                        position,
                        PredictEditsRequestTrigger::Other,
                        cx,
                    )
                })
                .log_err()
            else {
                return Task::ready(anyhow::Ok(None));
            };

            cx.spawn(async move |_cx| {
                request_task.await.map(|prediction_result| {
                    prediction_result.map(|prediction_result| {
                        (
                            prediction_result,
                            PredictionRequestedBy::Buffer(buffer.entity_id()),
                        )
                    })
                })
            })
        })
    }

    pub fn refresh_prediction_from_diagnostics(
        &mut self,
        project: Entity<Project>,
        cx: &mut Context<Self>,
    ) {
        let Some(zeta_project) = self.projects.get_mut(&project.entity_id()) else {
            return;
        };

        // Prefer predictions from buffer
        if zeta_project.current_prediction.is_some() {
            return;
        };

        self.queue_prediction_refresh(project.clone(), project.entity_id(), cx, move |this, cx| {
            let Some(open_buffer_task) = project
                .update(cx, |project, cx| {
                    project
                        .active_entry()
                        .and_then(|entry| project.path_for_entry(entry, cx))
                        .map(|path| project.open_buffer(path, cx))
                })
                .log_err()
                .flatten()
            else {
                return Task::ready(anyhow::Ok(None));
            };

            cx.spawn(async move |cx| {
                let active_buffer = open_buffer_task.await?;
                let snapshot = active_buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;

                let Some((jump_buffer, jump_position)) = Self::next_diagnostic_location(
                    active_buffer,
                    &snapshot,
                    Default::default(),
                    Default::default(),
                    &project,
                    cx,
                )
                .await?
                else {
                    return anyhow::Ok(None);
                };

                let Some(prediction_result) = this
                    .update(cx, |this, cx| {
                        this.request_prediction(
                            &project,
                            &jump_buffer,
                            jump_position,
                            PredictEditsRequestTrigger::Diagnostics,
                            cx,
                        )
                    })?
                    .await?
                else {
                    return anyhow::Ok(None);
                };

                this.update(cx, |this, cx| {
                    Some((
                        if this
                            .get_or_init_zeta_project(&project, cx)
                            .current_prediction
                            .is_none()
                        {
                            prediction_result
                        } else {
                            EditPredictionResult {
                                id: prediction_result.id,
                                prediction: Err(EditPredictionRejectReason::CurrentPreferred),
                            }
                        },
                        PredictionRequestedBy::DiagnosticsUpdate,
                    ))
                })
            })
        });
    }

    #[cfg(not(test))]
    pub const THROTTLE_TIMEOUT: Duration = Duration::from_millis(300);
    #[cfg(test)]
    pub const THROTTLE_TIMEOUT: Duration = Duration::ZERO;

    fn queue_prediction_refresh(
        &mut self,
        project: Entity<Project>,
        throttle_entity: EntityId,
        cx: &mut Context<Self>,
        do_refresh: impl FnOnce(
            WeakEntity<Self>,
            &mut AsyncApp,
        )
            -> Task<Result<Option<(EditPredictionResult, PredictionRequestedBy)>>>
        + 'static,
    ) {
        let zeta_project = self.get_or_init_zeta_project(&project, cx);
        let pending_prediction_id = zeta_project.next_pending_prediction_id;
        zeta_project.next_pending_prediction_id += 1;
        let last_request = zeta_project.last_prediction_refresh;

        let task = cx.spawn(async move |this, cx| {
            if let Some((last_entity, last_timestamp)) = last_request
                && throttle_entity == last_entity
                && let Some(timeout) =
                    (last_timestamp + Self::THROTTLE_TIMEOUT).checked_duration_since(Instant::now())
            {
                cx.background_executor().timer(timeout).await;
            }

            // If this task was cancelled before the throttle timeout expired,
            // do not perform a request.
            let mut is_cancelled = true;
            this.update(cx, |this, cx| {
                let project_state = this.get_or_init_zeta_project(&project, cx);
                if !project_state
                    .cancelled_predictions
                    .remove(&pending_prediction_id)
                {
                    project_state.last_prediction_refresh = Some((throttle_entity, Instant::now()));
                    is_cancelled = false;
                }
            })
            .ok();
            if is_cancelled {
                return None;
            }

            let new_prediction_result = do_refresh(this.clone(), cx).await.log_err().flatten();
            let new_prediction_id = new_prediction_result
                .as_ref()
                .map(|(prediction, _)| prediction.id.clone());

            // When a prediction completes, remove it from the pending list, and cancel
            // any pending predictions that were enqueued before it.
            this.update(cx, |this, cx| {
                let zeta_project = this.get_or_init_zeta_project(&project, cx);

                let is_cancelled = zeta_project
                    .cancelled_predictions
                    .remove(&pending_prediction_id);

                let new_current_prediction = if !is_cancelled
                    && let Some((prediction_result, requested_by)) = new_prediction_result
                {
                    match prediction_result.prediction {
                        Ok(prediction) => {
                            let new_prediction = CurrentEditPrediction {
                                requested_by,
                                prediction,
                                was_shown: false,
                            };

                            if let Some(current_prediction) =
                                zeta_project.current_prediction.as_ref()
                            {
                                if new_prediction.should_replace_prediction(&current_prediction, cx)
                                {
                                    this.reject_current_prediction(
                                        EditPredictionRejectReason::Replaced,
                                        &project,
                                    );

                                    Some(new_prediction)
                                } else {
                                    this.reject_prediction(
                                        new_prediction.prediction.id,
                                        EditPredictionRejectReason::CurrentPreferred,
                                        false,
                                    );
                                    None
                                }
                            } else {
                                Some(new_prediction)
                            }
                        }
                        Err(reject_reason) => {
                            this.reject_prediction(prediction_result.id, reject_reason, false);
                            None
                        }
                    }
                } else {
                    None
                };

                let zeta_project = this.get_or_init_zeta_project(&project, cx);

                if let Some(new_prediction) = new_current_prediction {
                    zeta_project.current_prediction = Some(new_prediction);
                }

                let mut pending_predictions = mem::take(&mut zeta_project.pending_predictions);
                for (ix, pending_prediction) in pending_predictions.iter().enumerate() {
                    if pending_prediction.id == pending_prediction_id {
                        pending_predictions.remove(ix);
                        for pending_prediction in pending_predictions.drain(0..ix) {
                            zeta_project.cancel_pending_prediction(pending_prediction, cx)
                        }
                        break;
                    }
                }
                this.get_or_init_zeta_project(&project, cx)
                    .pending_predictions = pending_predictions;
                cx.notify();
            })
            .ok();

            new_prediction_id
        });

        if zeta_project.pending_predictions.len() <= 1 {
            zeta_project.pending_predictions.push(PendingPrediction {
                id: pending_prediction_id,
                task,
            });
        } else if zeta_project.pending_predictions.len() == 2 {
            let pending_prediction = zeta_project.pending_predictions.pop().unwrap();
            zeta_project.pending_predictions.push(PendingPrediction {
                id: pending_prediction_id,
                task,
            });
            zeta_project.cancel_pending_prediction(pending_prediction, cx);
        }
    }

    pub fn request_prediction(
        &mut self,
        project: &Entity<Project>,
        active_buffer: &Entity<Buffer>,
        position: language::Anchor,
        trigger: PredictEditsRequestTrigger,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<EditPredictionResult>>> {
        self.request_prediction_internal(
            project.clone(),
            active_buffer.clone(),
            position,
            trigger,
            cx.has_flag::<Zeta2FeatureFlag>(),
            cx,
        )
    }

    fn request_prediction_internal(
        &mut self,
        project: Entity<Project>,
        active_buffer: Entity<Buffer>,
        position: language::Anchor,
        trigger: PredictEditsRequestTrigger,
        allow_jump: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<EditPredictionResult>>> {
        const DIAGNOSTIC_LINES_RANGE: u32 = 20;

        self.get_or_init_zeta_project(&project, cx);
        let zeta_project = self.projects.get(&project.entity_id()).unwrap();
        let events = zeta_project.events(cx);
        let has_events = !events.is_empty();

        let snapshot = active_buffer.read(cx).snapshot();
        let cursor_point = position.to_point(&snapshot);
        let diagnostic_search_start = cursor_point.row.saturating_sub(DIAGNOSTIC_LINES_RANGE);
        let diagnostic_search_end = cursor_point.row + DIAGNOSTIC_LINES_RANGE;
        let diagnostic_search_range =
            Point::new(diagnostic_search_start, 0)..Point::new(diagnostic_search_end, 0);

        let task = match self.edit_prediction_model {
            ZetaEditPredictionModel::Zeta1 => request_prediction_with_zeta1(
                self,
                &project,
                &active_buffer,
                snapshot.clone(),
                position,
                events,
                trigger,
                cx,
            ),
            ZetaEditPredictionModel::Zeta2 => self.request_prediction_with_zeta2(
                &project,
                &active_buffer,
                snapshot.clone(),
                position,
                events,
                trigger,
                cx,
            ),
            ZetaEditPredictionModel::Sweep => self.sweep_ai.request_prediction_with_sweep(
                &project,
                &active_buffer,
                snapshot.clone(),
                position,
                events,
                &zeta_project.recent_paths,
                if self.use_context {
                    self.context_for_project(&project, cx).to_vec()
                } else {
                    Vec::new()
                },
                diagnostic_search_range.clone(),
                cx,
            ),
        };

        cx.spawn(async move |this, cx| {
            let prediction = task.await?;

            if prediction.is_none() && allow_jump {
                let cursor_point = position.to_point(&snapshot);
                if has_events
                    && let Some((jump_buffer, jump_position)) = Self::next_diagnostic_location(
                        active_buffer.clone(),
                        &snapshot,
                        diagnostic_search_range,
                        cursor_point,
                        &project,
                        cx,
                    )
                    .await?
                {
                    return this
                        .update(cx, |this, cx| {
                            this.request_prediction_internal(
                                project,
                                jump_buffer,
                                jump_position,
                                trigger,
                                false,
                                cx,
                            )
                        })?
                        .await;
                }

                return anyhow::Ok(None);
            }

            Ok(prediction)
        })
    }

    async fn next_diagnostic_location(
        active_buffer: Entity<Buffer>,
        active_buffer_snapshot: &BufferSnapshot,
        active_buffer_diagnostic_search_range: Range<Point>,
        active_buffer_cursor_point: Point,
        project: &Entity<Project>,
        cx: &mut AsyncApp,
    ) -> Result<Option<(Entity<Buffer>, language::Anchor)>> {
        // find the closest diagnostic to the cursor that wasn't close enough to be included in the last request
        let mut jump_location = active_buffer_snapshot
            .diagnostic_groups(None)
            .into_iter()
            .filter_map(|(_, group)| {
                let range = &group.entries[group.primary_ix]
                    .range
                    .to_point(&active_buffer_snapshot);
                if range.overlaps(&active_buffer_diagnostic_search_range) {
                    None
                } else {
                    Some(range.start)
                }
            })
            .min_by_key(|probe| probe.row.abs_diff(active_buffer_cursor_point.row))
            .map(|position| {
                (
                    active_buffer.clone(),
                    active_buffer_snapshot.anchor_before(position),
                )
            });

        if jump_location.is_none() {
            let active_buffer_path = active_buffer.read_with(cx, |buffer, cx| {
                let file = buffer.file()?;

                Some(ProjectPath {
                    worktree_id: file.worktree_id(cx),
                    path: file.path().clone(),
                })
            })?;

            let buffer_task = project.update(cx, |project, cx| {
                let (path, _, _) = project
                    .diagnostic_summaries(false, cx)
                    .filter(|(path, _, _)| Some(path) != active_buffer_path.as_ref())
                    .max_by_key(|(path, _, _)| {
                        // find the buffer with errors that shares most parent directories
                        path.path
                            .components()
                            .zip(
                                active_buffer_path
                                    .as_ref()
                                    .map(|p| p.path.components())
                                    .unwrap_or_default(),
                            )
                            .take_while(|(a, b)| a == b)
                            .count()
                    })?;

                Some(project.open_buffer(path, cx))
            })?;

            if let Some(buffer_task) = buffer_task {
                let closest_buffer = buffer_task.await?;

                jump_location = closest_buffer
                    .read_with(cx, |buffer, _cx| {
                        buffer
                            .buffer_diagnostics(None)
                            .into_iter()
                            .min_by_key(|entry| entry.diagnostic.severity)
                            .map(|entry| entry.range.start)
                    })?
                    .map(|position| (closest_buffer, position));
            }
        }

        anyhow::Ok(jump_location)
    }

    fn request_prediction_with_zeta2(
        &mut self,
        project: &Entity<Project>,
        active_buffer: &Entity<Buffer>,
        active_snapshot: BufferSnapshot,
        position: language::Anchor,
        events: Vec<Arc<Event>>,
        trigger: PredictEditsRequestTrigger,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<EditPredictionResult>>> {
        let options = self.options.clone();
        let buffer_snapshotted_at = Instant::now();

        let Some((excerpt_path, active_project_path)) = active_snapshot
            .file()
            .map(|file| -> Arc<Path> { file.full_path(cx).into() })
            .zip(active_buffer.read(cx).project_path(cx))
        else {
            return Task::ready(Err(anyhow!("No file path for excerpt")));
        };

        let client = self.client.clone();
        let llm_token = self.llm_token.clone();
        let app_version = AppVersion::global(cx);
        let debug_tx = self.debug_tx.clone();

        let diagnostics = active_snapshot.diagnostic_sets().clone();

        let file = active_buffer.read(cx).file();

        let active_file_full_path = file.as_ref().map(|f| f.full_path(cx));

        // TODO data collection
        let can_collect_data = file
            .as_ref()
            .map_or(false, |file| self.can_collect_file(project, file, cx));

        let mut included_files = self.context_for_project(project, cx).to_vec();

        #[cfg(feature = "eval-support")]
        let eval_cache = self.eval_cache.clone();

        let request_task = cx.background_spawn({
            let active_buffer = active_buffer.clone();
            async move {
                let cursor_offset = position.to_offset(&active_snapshot);
                let cursor_point = cursor_offset.to_point(&active_snapshot);

                let before_retrieval = Instant::now();

                let (diagnostic_groups, diagnostic_groups_truncated) =
                    Self::gather_nearby_diagnostics(
                        cursor_offset,
                        &diagnostics,
                        &active_snapshot,
                        options.max_diagnostic_bytes,
                    );

                let excerpt_options = options.context.excerpt();

                let Some(excerpt) = EditPredictionExcerpt::select_from_buffer(
                    cursor_point,
                    &active_snapshot,
                    &excerpt_options,
                    None,
                ) else {
                    return Ok((None, None));
                };

                let excerpt_anchor_range = active_snapshot.anchor_after(excerpt.range.start)
                    ..active_snapshot.anchor_before(excerpt.range.end);
                let related_excerpt = RelatedExcerpt {
                    anchor_range: excerpt_anchor_range.clone(),
                    point_range: Point::new(excerpt.line_range.start.0, 0)
                        ..Point::new(excerpt.line_range.end.0, 0),
                    text: active_snapshot.as_rope().slice(excerpt.range),
                };

                if let Some(buffer_ix) = included_files
                    .iter()
                    .position(|file| file.buffer.entity_id() == active_buffer.entity_id())
                {
                    let file = &mut included_files[buffer_ix];
                    file.excerpts.push(related_excerpt);
                    file.merge_excerpts();
                    let last_ix = included_files.len() - 1;
                    included_files.swap(buffer_ix, last_ix);
                } else {
                    let active_file = RelatedFile {
                        path: active_project_path,
                        buffer: active_buffer.downgrade(),
                        excerpts: vec![related_excerpt],
                        max_row: active_snapshot.max_point().row,
                    };
                    included_files.push(active_file);
                }

                let included_files = included_files
                    .iter()
                    .map(|related_file| predict_edits_v3::IncludedFile {
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
                    trigger,
                };

                let prompt_result = cloud_zeta2_prompt::build_prompt(&cloud_request);

                let inputs = EditPredictionInputs {
                    included_files: cloud_request.included_files,
                    events: cloud_request.events,
                    cursor_point: cloud_request.cursor_point,
                    cursor_path: cloud_request.excerpt_path,
                };

                let retrieval_time = Instant::now() - before_retrieval;

                let debug_response_tx = if let Some(debug_tx) = &debug_tx {
                    let (response_tx, response_rx) = oneshot::channel();

                    debug_tx
                        .unbounded_send(ZetaDebugInfo::EditPredictionRequested(
                            ZetaEditPredictionDebugInfo {
                                inputs: inputs.clone(),
                                retrieval_time,
                                buffer: active_buffer.downgrade(),
                                local_prompt: match prompt_result.as_ref() {
                                    Ok((prompt, _)) => Ok(prompt.clone()),
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

                let (prompt, _) = prompt_result?;
                let generation_params =
                    cloud_zeta2_prompt::generation_params(cloud_request.prompt_format);
                let request = open_ai::Request {
                    model: EDIT_PREDICTIONS_MODEL_ID.clone(),
                    messages: vec![open_ai::RequestMessage::User {
                        content: open_ai::MessageContent::Plain(prompt),
                    }],
                    stream: false,
                    max_completion_tokens: None,
                    stop: generation_params.stop.unwrap_or_default(),
                    temperature: generation_params.temperature.unwrap_or(0.7),
                    tool_choice: None,
                    parallel_tool_calls: None,
                    tools: vec![],
                    prompt_cache_key: None,
                    reasoning_effort: None,
                };

                log::trace!("Sending edit prediction request");

                let before_request = Instant::now();
                let response = Self::send_raw_llm_request(
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

                let (_, edits) = match options.prompt_format {
                    PromptFormat::NumLinesUniDiff => {
                        // TODO: Implement parsing of multi-file diffs
                        crate::udiff::parse_diff(&output_text, get_buffer_from_context).await?
                    }
                    PromptFormat::Minimal
                    | PromptFormat::MinimalQwen
                    | PromptFormat::SeedCoder1120 => {
                        if output_text.contains("--- a/\n+++ b/\nNo edits") {
                            let edits = vec![];
                            (&active_snapshot, edits)
                        } else {
                            crate::udiff::parse_diff(&output_text, get_buffer_from_context).await?
                        }
                    }
                    PromptFormat::OldTextNewText => {
                        crate::xml_edits::parse_xml_edits(&output_text, get_buffer_from_context)
                            .await?
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

        cx.spawn({
            async move |this, cx| {
                let Some((id, prediction)) =
                    Self::handle_api_response(&this, request_task.await, cx)?
                else {
                    return Ok(None);
                };

                let Some((
                    inputs,
                    edited_buffer,
                    edited_buffer_snapshot,
                    edits,
                    received_response_at,
                )) = prediction
                else {
                    return Ok(Some(EditPredictionResult {
                        id,
                        prediction: Err(EditPredictionRejectReason::Empty),
                    }));
                };

                // TODO telemetry: duration, etc
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
            }
        })
    }

    async fn send_raw_llm_request(
        request: open_ai::Request,
        client: Arc<Client>,
        llm_token: LlmApiToken,
        app_version: Version,
        #[cfg(feature = "eval-support")] eval_cache: Option<Arc<dyn EvalCache>>,
        #[cfg(feature = "eval-support")] eval_cache_kind: EvalCacheEntryKind,
    ) -> Result<(open_ai::Response, Option<EditPredictionUsage>)> {
        let url = if let Some(predict_edits_url) = PREDICT_EDITS_URL.as_ref() {
            http_client::Url::parse(&predict_edits_url)?
        } else {
            client
                .http_client()
                .build_zed_llm_url("/predict_edits/raw", &[])?
        };

        #[cfg(feature = "eval-support")]
        let cache_key = if let Some(cache) = eval_cache {
            use collections::FxHasher;
            use std::hash::{Hash, Hasher};

            let mut hasher = FxHasher::default();
            url.hash(&mut hasher);
            let request_str = serde_json::to_string_pretty(&request)?;
            request_str.hash(&mut hasher);
            let hash = hasher.finish();

            let key = (eval_cache_kind, hash);
            if let Some(response_str) = cache.read(key) {
                return Ok((serde_json::from_str(&response_str)?, None));
            }

            Some((cache, request_str, key))
        } else {
            None
        };

        let (response, usage) = Self::send_api_request(
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
        .await?;

        #[cfg(feature = "eval-support")]
        if let Some((cache, request, key)) = cache_key {
            cache.write(key, &request, &serde_json::to_string_pretty(&response)?);
        }

        Ok((response, usage))
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
        app_version: Version,
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
                .and_then(|version| Version::from_str(version.to_str().ok()?).ok())
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

    pub fn refresh_context_if_needed(
        &mut self,
        project: &Entity<Project>,
        buffer: &Entity<language::Buffer>,
        cursor_position: language::Anchor,
        cx: &mut Context<Self>,
    ) {
        if !self.use_context {
            return;
        }
        let Some(zeta_project) = self.projects.get_mut(&project.entity_id()) else {
            return;
        };

        match &mut zeta_project.context {
            ZetaProjectContext::Syntax(_entity) => {}
            ZetaProjectContext::Lsp(related_excerpt_store) => {
                related_excerpt_store.update(cx, |store, cx| {
                    store.refresh(buffer.clone(), cursor_position, cx);
                });
            }
            ZetaProjectContext::Agentic {
                refresh_context_debounce_task,
                refresh_context_timestamp,
                ..
            } => {
                let now = Instant::now();
                let was_idle = refresh_context_timestamp.map_or(true, |timestamp| {
                    now - timestamp > Self::CONTEXT_RETRIEVAL_IDLE_DURATION
                });
                *refresh_context_timestamp = Some(now);
                *refresh_context_debounce_task = Some(cx.spawn({
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
                            let task = this.refresh_context_with_agentic_retrieval(
                                project.clone(),
                                buffer,
                                cursor_position,
                                cx,
                            );

                            if let Some(zeta_project) = this.projects.get_mut(&project.entity_id())
                            {
                                if let ZetaProjectContext::Agentic {
                                    refresh_context_task,
                                    ..
                                } = &mut zeta_project.context
                                {
                                    *refresh_context_task = Some(task.log_err());
                                }
                            };
                        })
                        .ok()
                    }
                }));
            }
        }
    }

    // Refresh the related excerpts asynchronously. Ensure the task runs to completion,
    // and avoid spawning more than one concurrent task.
    pub fn refresh_context_with_agentic_retrieval(
        &mut self,
        project: Entity<Project>,
        buffer: Entity<language::Buffer>,
        cursor_position: language::Anchor,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let Some(zeta_project) = self.projects.get(&project.entity_id()) else {
            return Task::ready(anyhow::Ok(()));
        };

        let ContextMode::Agentic(options) = &self.options().context else {
            return Task::ready(anyhow::Ok(()));
        };

        let snapshot = buffer.read(cx).snapshot();
        let cursor_point = cursor_position.to_point(&snapshot);
        let Some(cursor_excerpt) = EditPredictionExcerpt::select_from_buffer(
            cursor_point,
            &snapshot,
            &options.excerpt,
            None,
        ) else {
            return Task::ready(Ok(()));
        };

        let app_version = AppVersion::global(cx);
        let client = self.client.clone();
        let llm_token = self.llm_token.clone();
        let debug_tx = self.debug_tx.clone();
        let current_file_path: Arc<Path> = snapshot
            .file()
            .map(|f| f.full_path(cx).into())
            .unwrap_or_else(|| Path::new("untitled").into());

        let prompt = match cloud_zeta2_prompt::retrieval_prompt::build_prompt(
            predict_edits_v3::PlanContextRetrievalRequest {
                excerpt: cursor_excerpt.text(&snapshot).body,
                excerpt_path: current_file_path,
                excerpt_line_range: cursor_excerpt.line_range,
                cursor_file_max_row: Line(snapshot.max_point().row),
                events: zeta_project.events(cx),
            },
        ) {
            Ok(prompt) => prompt,
            Err(err) => {
                return Task::ready(Err(err));
            }
        };

        let retrieval_started_at = Instant::now();

        if let Some(debug_tx) = &debug_tx {
            debug_tx
                .unbounded_send(ZetaDebugInfo::ContextRetrievalStarted(
                    ZetaContextRetrievalStartedDebugInfo {
                        project_entity_id: project.entity_id(),
                        timestamp: retrieval_started_at,
                        search_prompt: prompt.clone(),
                    },
                ))
                .ok();
        }

        pub static TOOL_SCHEMA: LazyLock<(serde_json::Value, String)> = LazyLock::new(|| {
            let schema = language_model::tool_schema::root_schema_for::<SearchToolInput>(
                language_model::LanguageModelToolSchemaFormat::JsonSchemaSubset,
            );

            let description = schema
                .get("description")
                .and_then(|description| description.as_str())
                .unwrap()
                .to_string();

            (schema.into(), description)
        });

        let (tool_schema, tool_description) = TOOL_SCHEMA.clone();

        let request = open_ai::Request {
            model: CONTEXT_RETRIEVAL_MODEL_ID.clone(),
            messages: vec![open_ai::RequestMessage::User {
                content: open_ai::MessageContent::Plain(prompt),
            }],
            stream: false,
            max_completion_tokens: None,
            stop: Default::default(),
            temperature: 0.7,
            tool_choice: None,
            parallel_tool_calls: None,
            tools: vec![open_ai::ToolDefinition::Function {
                function: FunctionDefinition {
                    name: cloud_zeta2_prompt::retrieval_prompt::TOOL_NAME.to_string(),
                    description: Some(tool_description),
                    parameters: Some(tool_schema),
                },
            }],
            prompt_cache_key: None,
            reasoning_effort: None,
        };

        #[cfg(feature = "eval-support")]
        let eval_cache = self.eval_cache.clone();

        cx.spawn(async move |this, cx| {
            log::trace!("Sending search planning request");
            let response = Self::send_raw_llm_request(
                request,
                client,
                llm_token,
                app_version,
                #[cfg(feature = "eval-support")]
                eval_cache.clone(),
                #[cfg(feature = "eval-support")]
                EvalCacheEntryKind::Context,
            )
            .await;
            let mut response = Self::handle_api_response(&this, response, cx)?;
            log::trace!("Got search planning response");

            let choice = response
                .choices
                .pop()
                .context("No choices in retrieval response")?;
            let open_ai::RequestMessage::Assistant {
                content: _,
                tool_calls,
            } = choice.message
            else {
                anyhow::bail!("Retrieval response didn't include an assistant message");
            };

            let mut queries: Vec<SearchToolQuery> = Vec::new();
            for tool_call in tool_calls {
                let open_ai::ToolCallContent::Function { function } = tool_call.content;
                if function.name != cloud_zeta2_prompt::retrieval_prompt::TOOL_NAME {
                    log::warn!(
                        "Context retrieval response tried to call an unknown tool: {}",
                        function.name
                    );

                    continue;
                }

                let input: SearchToolInput = serde_json::from_str(&function.arguments)
                    .with_context(|| format!("invalid search json {}", &function.arguments))?;
                queries.extend(input.queries);
            }

            log::trace!("Running retrieval search: {queries:#?}");
            let query_generation_finished_at = Instant::now();

            let related_excerpts_result = retrieval_search::run_retrieval_searches(
                queries,
                project.clone(),
                #[cfg(feature = "eval-support")]
                eval_cache,
                cx,
            )
            .await;

            log::trace!("Search queries executed");
            let query_execution_finished_at = Instant::now();

            this.update(cx, |this, _cx| {
                let Some(zeta_project) = this.projects.get_mut(&project.entity_id()) else {
                    return Ok(());
                };
                if let ZetaProjectContext::Agentic {
                    refresh_context_task,
                    context,
                    ..
                } = &mut zeta_project.context
                {
                    refresh_context_task.take();
                    if let Some(debug_tx) = &this.debug_tx {
                        debug_tx
                            .unbounded_send(ZetaDebugInfo::ContextRetrievalFinished(
                                ZetaContextRetrievalFinishedDebugInfo {
                                    project_entity_id: project.entity_id(),
                                    timestamp: Instant::now(),
                                    metadata: vec![
                                        (
                                            "query_generation",
                                            format!(
                                                "{:?}",
                                                query_generation_finished_at - retrieval_started_at
                                            )
                                            .into(),
                                        ),
                                        (
                                            "search_execution",
                                            format!(
                                                "{:?}",
                                                query_execution_finished_at
                                                    - query_generation_finished_at
                                            )
                                            .into(),
                                        ),
                                    ],
                                },
                            ))
                            .ok();
                    }
                    match related_excerpts_result {
                        Ok(excerpts) => {
                            *context = excerpts;
                            Ok(())
                        }
                        Err(error) => Err(error),
                    }
                } else {
                    Ok(())
                }
            })?
        })
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

    pub fn wait_for_initial_indexing(
        &mut self,
        project: &Entity<Project>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let zeta_project = self.get_or_init_zeta_project(project, cx);
        if let ZetaProjectContext::Syntax(syntax_index) = &zeta_project.context {
            syntax_index.read(cx).wait_for_initial_file_indexing(cx)
        } else {
            Task::ready(Ok(()))
        }
    }

    fn is_file_open_source(
        &self,
        project: &Entity<Project>,
        file: &Arc<dyn File>,
        cx: &App,
    ) -> bool {
        if !file.is_local() || file.is_private() {
            return false;
        }
        let Some(zeta_project) = self.projects.get(&project.entity_id()) else {
            return false;
        };
        zeta_project
            .license_detection_watchers
            .get(&file.worktree_id(cx))
            .as_ref()
            .is_some_and(|watcher| watcher.is_project_open_source())
    }

    fn can_collect_file(&self, project: &Entity<Project>, file: &Arc<dyn File>, cx: &App) -> bool {
        self.data_collection_choice.is_enabled() && self.is_file_open_source(project, file, cx)
    }

    fn can_collect_events(&self, events: &[Arc<Event>]) -> bool {
        if !self.data_collection_choice.is_enabled() {
            return false;
        }
        events.iter().all(|event| {
            matches!(
                event.as_ref(),
                Event::BufferChange {
                    in_open_source_repo: true,
                    ..
                }
            )
        })
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

    pub fn shown_predictions(&self) -> impl DoubleEndedIterator<Item = &EditPrediction> {
        self.shown_predictions.iter()
    }

    pub fn shown_completions_len(&self) -> usize {
        self.shown_predictions.len()
    }

    pub fn is_prediction_rated(&self, id: &EditPredictionId) -> bool {
        self.rated_predictions.contains(id)
    }

    pub fn rate_prediction(
        &mut self,
        prediction: &EditPrediction,
        rating: EditPredictionRating,
        feedback: String,
        cx: &mut Context<Self>,
    ) {
        self.rated_predictions.insert(prediction.id.clone());
        telemetry::event!(
            "Edit Prediction Rated",
            rating,
            inputs = prediction.inputs,
            output = prediction.edit_preview.as_unified_diff(&prediction.edits),
            feedback
        );
        self.client.telemetry().flush_events().detach();
        cx.notify();
    }

    fn enable_or_disable_context_retrieval(&mut self, cx: &mut Context<'_, Zeta>) {
        self.use_context = cx.has_flag::<Zeta2FeatureFlag>()
            && all_language_settings(None, cx).edit_predictions.use_context;
    }
}

pub fn text_from_response(mut res: open_ai::Response) -> Option<String> {
    let choice = res.choices.pop()?;
    let output_text = match choice.message {
        open_ai::RequestMessage::Assistant {
            content: Some(open_ai::MessageContent::Plain(content)),
            ..
        } => content,
        open_ai::RequestMessage::Assistant {
            content: Some(open_ai::MessageContent::Multipart(mut content)),
            ..
        } => {
            if content.is_empty() {
                log::error!("No output from Baseten completion response");
                return None;
            }

            match content.remove(0) {
                open_ai::MessagePart::Text { text } => text,
                open_ai::MessagePart::Image { .. } => {
                    log::error!("Expected text, got an image");
                    return None;
                }
            }
        }
        _ => {
            log::error!("Invalid response message: {:?}", choice.message);
            return None;
        }
    };
    Some(output_text)
}

#[derive(Error, Debug)]
#[error(
    "You must update to Zed version {minimum_version} or higher to continue using edit predictions."
)]
pub struct ZedUpdateRequiredError {
    minimum_version: Version,
}

#[cfg(feature = "eval-support")]
pub type EvalCacheKey = (EvalCacheEntryKind, u64);

#[cfg(feature = "eval-support")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EvalCacheEntryKind {
    Context,
    Search,
    Prediction,
}

#[cfg(feature = "eval-support")]
impl std::fmt::Display for EvalCacheEntryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalCacheEntryKind::Search => write!(f, "search"),
            EvalCacheEntryKind::Context => write!(f, "context"),
            EvalCacheEntryKind::Prediction => write!(f, "prediction"),
        }
    }
}

#[cfg(feature = "eval-support")]
pub trait EvalCache: Send + Sync {
    fn read(&self, key: EvalCacheKey) -> Option<String>;
    fn write(&self, key: EvalCacheKey, input: &str, value: &str);
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

pub fn init(cx: &mut App) {
    feature_gate_predict_edits_actions(cx);

    cx.observe_new(move |workspace: &mut Workspace, _, _cx| {
        workspace.register_action(|workspace, _: &RateCompletions, window, cx| {
            if cx.has_flag::<PredictEditsRateCompletionsFeatureFlag>() {
                RatePredictionsModal::toggle(workspace, window, cx);
            }
        });

        workspace.register_action(
            move |workspace, _: &zed_actions::OpenZedPredictOnboarding, window, cx| {
                ZedPredictModal::toggle(
                    workspace,
                    workspace.user_store().clone(),
                    workspace.client().clone(),
                    window,
                    cx,
                )
            },
        );

        workspace.register_action(|workspace, _: &ResetOnboarding, _window, cx| {
            update_settings_file(workspace.app_state().fs.clone(), cx, move |settings, _| {
                settings
                    .project
                    .all_languages
                    .features
                    .get_or_insert_default()
                    .edit_prediction_provider = Some(EditPredictionProvider::None)
            });
        });
    })
    .detach();
}

fn feature_gate_predict_edits_actions(cx: &mut App) {
    let rate_completion_action_types = [TypeId::of::<RateCompletions>()];
    let reset_onboarding_action_types = [TypeId::of::<ResetOnboarding>()];
    let zeta_all_action_types = [
        TypeId::of::<RateCompletions>(),
        TypeId::of::<ResetOnboarding>(),
        zed_actions::OpenZedPredictOnboarding.type_id(),
        TypeId::of::<ClearHistory>(),
        TypeId::of::<ThumbsUpActivePrediction>(),
        TypeId::of::<ThumbsDownActivePrediction>(),
        TypeId::of::<NextEdit>(),
        TypeId::of::<PreviousEdit>(),
    ];

    CommandPaletteFilter::update_global(cx, |filter, _cx| {
        filter.hide_action_types(&rate_completion_action_types);
        filter.hide_action_types(&reset_onboarding_action_types);
        filter.hide_action_types(&[zed_actions::OpenZedPredictOnboarding.type_id()]);
    });

    cx.observe_global::<SettingsStore>(move |cx| {
        let is_ai_disabled = DisableAiSettings::get_global(cx).disable_ai;
        let has_feature_flag = cx.has_flag::<PredictEditsRateCompletionsFeatureFlag>();

        CommandPaletteFilter::update_global(cx, |filter, _cx| {
            if is_ai_disabled {
                filter.hide_action_types(&zeta_all_action_types);
            } else if has_feature_flag {
                filter.show_action_types(&rate_completion_action_types);
            } else {
                filter.hide_action_types(&rate_completion_action_types);
            }
        });
    })
    .detach();

    cx.observe_flag::<PredictEditsRateCompletionsFeatureFlag, _>(move |is_enabled, cx| {
        if !DisableAiSettings::get_global(cx).disable_ai {
            if is_enabled {
                CommandPaletteFilter::update_global(cx, |filter, _cx| {
                    filter.show_action_types(&rate_completion_action_types);
                });
            } else {
                CommandPaletteFilter::update_global(cx, |filter, _cx| {
                    filter.hide_action_types(&rate_completion_action_types);
                });
            }
        }
    })
    .detach();
}

#[cfg(test)]
mod tests {
    use std::{path::Path, sync::Arc, time::Duration};

    use client::UserStore;
    use clock::FakeSystemClock;
    use cloud_llm_client::{
        EditPredictionRejectReason, EditPredictionRejection, RejectEditPredictionsBody,
    };
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
    use language::OffsetRangeExt as _;
    use lsp::LanguageServerId;
    use open_ai::Usage;
    use pretty_assertions::{assert_eq, assert_matches};
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;
    use uuid::Uuid;

    use crate::{BufferEditPrediction, EditPredictionId, REJECT_REQUEST_DEBOUNCE, Zeta};

    #[gpui::test]
    async fn test_current_state(cx: &mut TestAppContext) {
        let (zeta, mut requests) = init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "1.txt": "Hello!\nHow\nBye\n",
                "2.txt": "Hola!\nComo\nAdios\n"
            }),
        )
        .await;
        let project = Project::test(fs, vec![path!("/root").as_ref()], cx).await;

        zeta.update(cx, |zeta, cx| {
            zeta.register_project(&project, cx);
        });

        let buffer1 = project
            .update(cx, |project, cx| {
                let path = project.find_project_path(path!("/root/1.txt"), cx).unwrap();
                project.set_active_path(Some(path.clone()), cx);
                project.open_buffer(path, cx)
            })
            .await
            .unwrap();
        let snapshot1 = buffer1.read_with(cx, |buffer, _cx| buffer.snapshot());
        let position = snapshot1.anchor_before(language::Point::new(1, 3));

        // Prediction for current file

        zeta.update(cx, |zeta, cx| {
            zeta.refresh_prediction_from_buffer(project.clone(), buffer1.clone(), position, cx)
        });
        let (_request, respond_tx) = requests.predict.next().await.unwrap();

        respond_tx
            .send(model_response(indoc! {r"
                --- a/root/1.txt
                +++ b/root/1.txt
                @@ ... @@
                 Hello!
                -How
                +How are you?
                 Bye
            "}))
            .unwrap();

        cx.run_until_parked();

        zeta.read_with(cx, |zeta, cx| {
            let prediction = zeta
                .current_prediction_for_buffer(&buffer1, &project, cx)
                .unwrap();
            assert_matches!(prediction, BufferEditPrediction::Local { .. });
        });

        zeta.update(cx, |zeta, _cx| {
            zeta.reject_current_prediction(EditPredictionRejectReason::Discarded, &project);
        });

        // Prediction for diagnostic in another file

        let diagnostic = lsp::Diagnostic {
            range: lsp::Range::new(lsp::Position::new(1, 1), lsp::Position::new(1, 5)),
            severity: Some(lsp::DiagnosticSeverity::ERROR),
            message: "Sentence is incomplete".to_string(),
            ..Default::default()
        };

        project.update(cx, |project, cx| {
            project.lsp_store().update(cx, |lsp_store, cx| {
                lsp_store
                    .update_diagnostics(
                        LanguageServerId(0),
                        lsp::PublishDiagnosticsParams {
                            uri: lsp::Uri::from_file_path(path!("/root/2.txt")).unwrap(),
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

        let (_request, respond_tx) = requests.predict.next().await.unwrap();
        respond_tx
            .send(model_response(indoc! {r#"
                --- a/root/2.txt
                +++ b/root/2.txt
                 Hola!
                -Como
                +Como estas?
                 Adios
            "#}))
            .unwrap();
        cx.run_until_parked();

        zeta.read_with(cx, |zeta, cx| {
            let prediction = zeta
                .current_prediction_for_buffer(&buffer1, &project, cx)
                .unwrap();
            assert_matches!(
                prediction,
                BufferEditPrediction::Jump { prediction } if prediction.snapshot.file().unwrap().full_path(cx) == Path::new(path!("root/2.txt"))
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
        let (zeta, mut requests) = init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "foo.md":  "Hello!\nHow\nBye\n"
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
            zeta.request_prediction(&project, &buffer, position, Default::default(), cx)
        });

        let (_, respond_tx) = requests.predict.next().await.unwrap();

        // TODO Put back when we have a structured request again
        // assert_eq!(
        //     request.excerpt_path.as_ref(),
        //     Path::new(path!("root/foo.md"))
        // );
        // assert_eq!(
        //     request.cursor_point,
        //     Point {
        //         line: Line(1),
        //         column: 3
        //     }
        // );

        respond_tx
            .send(model_response(indoc! { r"
                --- a/root/foo.md
                +++ b/root/foo.md
                @@ ... @@
                 Hello!
                -How
                +How are you?
                 Bye
            "}))
            .unwrap();

        let prediction = prediction_task.await.unwrap().unwrap().prediction.unwrap();

        assert_eq!(prediction.edits.len(), 1);
        assert_eq!(
            prediction.edits[0].0.to_point(&snapshot).start,
            language::Point::new(1, 3)
        );
        assert_eq!(prediction.edits[0].1.as_ref(), " are you?");
    }

    #[gpui::test]
    async fn test_request_events(cx: &mut TestAppContext) {
        let (zeta, mut requests) = init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "foo.md": "Hello!\n\nBye\n"
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
            zeta.request_prediction(&project, &buffer, position, Default::default(), cx)
        });

        let (request, respond_tx) = requests.predict.next().await.unwrap();

        let prompt = prompt_from_request(&request);
        assert!(
            prompt.contains(indoc! {"
            --- a/root/foo.md
            +++ b/root/foo.md
            @@ -1,3 +1,3 @@
             Hello!
            -
            +How
             Bye
        "}),
            "{prompt}"
        );

        respond_tx
            .send(model_response(indoc! {r#"
                --- a/root/foo.md
                +++ b/root/foo.md
                @@ ... @@
                 Hello!
                -How
                +How are you?
                 Bye
            "#}))
            .unwrap();

        let prediction = prediction_task.await.unwrap().unwrap().prediction.unwrap();

        assert_eq!(prediction.edits.len(), 1);
        assert_eq!(
            prediction.edits[0].0.to_point(&snapshot).start,
            language::Point::new(1, 3)
        );
        assert_eq!(prediction.edits[0].1.as_ref(), " are you?");
    }

    #[gpui::test]
    async fn test_empty_prediction(cx: &mut TestAppContext) {
        let (zeta, mut requests) = init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "foo.md":  "Hello!\nHow\nBye\n"
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

        zeta.update(cx, |zeta, cx| {
            zeta.refresh_prediction_from_buffer(project.clone(), buffer.clone(), position, cx);
        });

        const NO_OP_DIFF: &str = indoc! { r"
            --- a/root/foo.md
            +++ b/root/foo.md
            @@ ... @@
             Hello!
            -How
            +How
             Bye
        "};

        let (_, respond_tx) = requests.predict.next().await.unwrap();
        let response = model_response(NO_OP_DIFF);
        let id = response.id.clone();
        respond_tx.send(response).unwrap();

        cx.run_until_parked();

        zeta.read_with(cx, |zeta, cx| {
            assert!(
                zeta.current_prediction_for_buffer(&buffer, &project, cx)
                    .is_none()
            );
        });

        // prediction is reported as rejected
        let (reject_request, _) = requests.reject.next().await.unwrap();

        assert_eq!(
            &reject_request.rejections,
            &[EditPredictionRejection {
                request_id: id,
                reason: EditPredictionRejectReason::Empty,
                was_shown: false
            }]
        );
    }

    #[gpui::test]
    async fn test_interpolated_empty(cx: &mut TestAppContext) {
        let (zeta, mut requests) = init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "foo.md":  "Hello!\nHow\nBye\n"
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

        zeta.update(cx, |zeta, cx| {
            zeta.refresh_prediction_from_buffer(project.clone(), buffer.clone(), position, cx);
        });

        let (_, respond_tx) = requests.predict.next().await.unwrap();

        buffer.update(cx, |buffer, cx| {
            buffer.set_text("Hello!\nHow are you?\nBye", cx);
        });

        let response = model_response(SIMPLE_DIFF);
        let id = response.id.clone();
        respond_tx.send(response).unwrap();

        cx.run_until_parked();

        zeta.read_with(cx, |zeta, cx| {
            assert!(
                zeta.current_prediction_for_buffer(&buffer, &project, cx)
                    .is_none()
            );
        });

        // prediction is reported as rejected
        let (reject_request, _) = requests.reject.next().await.unwrap();

        assert_eq!(
            &reject_request.rejections,
            &[EditPredictionRejection {
                request_id: id,
                reason: EditPredictionRejectReason::InterpolatedEmpty,
                was_shown: false
            }]
        );
    }

    const SIMPLE_DIFF: &str = indoc! { r"
        --- a/root/foo.md
        +++ b/root/foo.md
        @@ ... @@
         Hello!
        -How
        +How are you?
         Bye
    "};

    #[gpui::test]
    async fn test_replace_current(cx: &mut TestAppContext) {
        let (zeta, mut requests) = init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "foo.md":  "Hello!\nHow\nBye\n"
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

        zeta.update(cx, |zeta, cx| {
            zeta.refresh_prediction_from_buffer(project.clone(), buffer.clone(), position, cx);
        });

        let (_, respond_tx) = requests.predict.next().await.unwrap();
        let first_response = model_response(SIMPLE_DIFF);
        let first_id = first_response.id.clone();
        respond_tx.send(first_response).unwrap();

        cx.run_until_parked();

        zeta.read_with(cx, |zeta, cx| {
            assert_eq!(
                zeta.current_prediction_for_buffer(&buffer, &project, cx)
                    .unwrap()
                    .id
                    .0,
                first_id
            );
        });

        // a second request is triggered
        zeta.update(cx, |zeta, cx| {
            zeta.refresh_prediction_from_buffer(project.clone(), buffer.clone(), position, cx);
        });

        let (_, respond_tx) = requests.predict.next().await.unwrap();
        let second_response = model_response(SIMPLE_DIFF);
        let second_id = second_response.id.clone();
        respond_tx.send(second_response).unwrap();

        cx.run_until_parked();

        zeta.read_with(cx, |zeta, cx| {
            // second replaces first
            assert_eq!(
                zeta.current_prediction_for_buffer(&buffer, &project, cx)
                    .unwrap()
                    .id
                    .0,
                second_id
            );
        });

        // first is reported as replaced
        let (reject_request, _) = requests.reject.next().await.unwrap();

        assert_eq!(
            &reject_request.rejections,
            &[EditPredictionRejection {
                request_id: first_id,
                reason: EditPredictionRejectReason::Replaced,
                was_shown: false
            }]
        );
    }

    #[gpui::test]
    async fn test_current_preferred(cx: &mut TestAppContext) {
        let (zeta, mut requests) = init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "foo.md":  "Hello!\nHow\nBye\n"
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

        zeta.update(cx, |zeta, cx| {
            zeta.refresh_prediction_from_buffer(project.clone(), buffer.clone(), position, cx);
        });

        let (_, respond_tx) = requests.predict.next().await.unwrap();
        let first_response = model_response(SIMPLE_DIFF);
        let first_id = first_response.id.clone();
        respond_tx.send(first_response).unwrap();

        cx.run_until_parked();

        zeta.read_with(cx, |zeta, cx| {
            assert_eq!(
                zeta.current_prediction_for_buffer(&buffer, &project, cx)
                    .unwrap()
                    .id
                    .0,
                first_id
            );
        });

        // a second request is triggered
        zeta.update(cx, |zeta, cx| {
            zeta.refresh_prediction_from_buffer(project.clone(), buffer.clone(), position, cx);
        });

        let (_, respond_tx) = requests.predict.next().await.unwrap();
        // worse than current prediction
        let second_response = model_response(indoc! { r"
            --- a/root/foo.md
            +++ b/root/foo.md
            @@ ... @@
             Hello!
            -How
            +How are
             Bye
        "});
        let second_id = second_response.id.clone();
        respond_tx.send(second_response).unwrap();

        cx.run_until_parked();

        zeta.read_with(cx, |zeta, cx| {
            // first is preferred over second
            assert_eq!(
                zeta.current_prediction_for_buffer(&buffer, &project, cx)
                    .unwrap()
                    .id
                    .0,
                first_id
            );
        });

        // second is reported as rejected
        let (reject_request, _) = requests.reject.next().await.unwrap();

        assert_eq!(
            &reject_request.rejections,
            &[EditPredictionRejection {
                request_id: second_id,
                reason: EditPredictionRejectReason::CurrentPreferred,
                was_shown: false
            }]
        );
    }

    #[gpui::test]
    async fn test_cancel_earlier_pending_requests(cx: &mut TestAppContext) {
        let (zeta, mut requests) = init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "foo.md":  "Hello!\nHow\nBye\n"
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

        // start two refresh tasks
        zeta.update(cx, |zeta, cx| {
            zeta.refresh_prediction_from_buffer(project.clone(), buffer.clone(), position, cx);
        });

        let (_, respond_first) = requests.predict.next().await.unwrap();

        zeta.update(cx, |zeta, cx| {
            zeta.refresh_prediction_from_buffer(project.clone(), buffer.clone(), position, cx);
        });

        let (_, respond_second) = requests.predict.next().await.unwrap();

        // wait for throttle
        cx.run_until_parked();

        // second responds first
        let second_response = model_response(SIMPLE_DIFF);
        let second_id = second_response.id.clone();
        respond_second.send(second_response).unwrap();

        cx.run_until_parked();

        zeta.read_with(cx, |zeta, cx| {
            // current prediction is second
            assert_eq!(
                zeta.current_prediction_for_buffer(&buffer, &project, cx)
                    .unwrap()
                    .id
                    .0,
                second_id
            );
        });

        let first_response = model_response(SIMPLE_DIFF);
        let first_id = first_response.id.clone();
        respond_first.send(first_response).unwrap();

        cx.run_until_parked();

        zeta.read_with(cx, |zeta, cx| {
            // current prediction is still second, since first was cancelled
            assert_eq!(
                zeta.current_prediction_for_buffer(&buffer, &project, cx)
                    .unwrap()
                    .id
                    .0,
                second_id
            );
        });

        // first is reported as rejected
        let (reject_request, _) = requests.reject.next().await.unwrap();

        cx.run_until_parked();

        assert_eq!(
            &reject_request.rejections,
            &[EditPredictionRejection {
                request_id: first_id,
                reason: EditPredictionRejectReason::Canceled,
                was_shown: false
            }]
        );
    }

    #[gpui::test]
    async fn test_cancel_second_on_third_request(cx: &mut TestAppContext) {
        let (zeta, mut requests) = init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "foo.md":  "Hello!\nHow\nBye\n"
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

        // start two refresh tasks
        zeta.update(cx, |zeta, cx| {
            zeta.refresh_prediction_from_buffer(project.clone(), buffer.clone(), position, cx);
        });

        let (_, respond_first) = requests.predict.next().await.unwrap();

        zeta.update(cx, |zeta, cx| {
            zeta.refresh_prediction_from_buffer(project.clone(), buffer.clone(), position, cx);
        });

        let (_, respond_second) = requests.predict.next().await.unwrap();

        // wait for throttle, so requests are sent
        cx.run_until_parked();

        zeta.update(cx, |zeta, cx| {
            // start a third request
            zeta.refresh_prediction_from_buffer(project.clone(), buffer.clone(), position, cx);

            // 2 are pending, so 2nd is cancelled
            assert_eq!(
                zeta.get_or_init_zeta_project(&project, cx)
                    .cancelled_predictions
                    .iter()
                    .copied()
                    .collect::<Vec<_>>(),
                [1]
            );
        });

        // wait for throttle
        cx.run_until_parked();

        let (_, respond_third) = requests.predict.next().await.unwrap();

        let first_response = model_response(SIMPLE_DIFF);
        let first_id = first_response.id.clone();
        respond_first.send(first_response).unwrap();

        cx.run_until_parked();

        zeta.read_with(cx, |zeta, cx| {
            // current prediction is first
            assert_eq!(
                zeta.current_prediction_for_buffer(&buffer, &project, cx)
                    .unwrap()
                    .id
                    .0,
                first_id
            );
        });

        let cancelled_response = model_response(SIMPLE_DIFF);
        let cancelled_id = cancelled_response.id.clone();
        respond_second.send(cancelled_response).unwrap();

        cx.run_until_parked();

        zeta.read_with(cx, |zeta, cx| {
            // current prediction is still first, since second was cancelled
            assert_eq!(
                zeta.current_prediction_for_buffer(&buffer, &project, cx)
                    .unwrap()
                    .id
                    .0,
                first_id
            );
        });

        let third_response = model_response(SIMPLE_DIFF);
        let third_response_id = third_response.id.clone();
        respond_third.send(third_response).unwrap();

        cx.run_until_parked();

        zeta.read_with(cx, |zeta, cx| {
            // third completes and replaces first
            assert_eq!(
                zeta.current_prediction_for_buffer(&buffer, &project, cx)
                    .unwrap()
                    .id
                    .0,
                third_response_id
            );
        });

        // second is reported as rejected
        let (reject_request, _) = requests.reject.next().await.unwrap();

        cx.run_until_parked();

        assert_eq!(
            &reject_request.rejections,
            &[
                EditPredictionRejection {
                    request_id: cancelled_id,
                    reason: EditPredictionRejectReason::Canceled,
                    was_shown: false
                },
                EditPredictionRejection {
                    request_id: first_id,
                    reason: EditPredictionRejectReason::Replaced,
                    was_shown: false
                }
            ]
        );
    }

    #[gpui::test]
    async fn test_rejections_flushing(cx: &mut TestAppContext) {
        let (zeta, mut requests) = init_test(cx);

        zeta.update(cx, |zeta, _cx| {
            zeta.reject_prediction(
                EditPredictionId("test-1".into()),
                EditPredictionRejectReason::Discarded,
                false,
            );
            zeta.reject_prediction(
                EditPredictionId("test-2".into()),
                EditPredictionRejectReason::Canceled,
                true,
            );
        });

        cx.executor().advance_clock(REJECT_REQUEST_DEBOUNCE);
        cx.run_until_parked();

        let (reject_request, respond_tx) = requests.reject.next().await.unwrap();
        respond_tx.send(()).unwrap();

        // batched
        assert_eq!(reject_request.rejections.len(), 2);
        assert_eq!(
            reject_request.rejections[0],
            EditPredictionRejection {
                request_id: "test-1".to_string(),
                reason: EditPredictionRejectReason::Discarded,
                was_shown: false
            }
        );
        assert_eq!(
            reject_request.rejections[1],
            EditPredictionRejection {
                request_id: "test-2".to_string(),
                reason: EditPredictionRejectReason::Canceled,
                was_shown: true
            }
        );

        // Reaching batch size limit sends without debounce
        zeta.update(cx, |zeta, _cx| {
            for i in 0..70 {
                zeta.reject_prediction(
                    EditPredictionId(format!("batch-{}", i).into()),
                    EditPredictionRejectReason::Discarded,
                    false,
                );
            }
        });

        // First MAX/2 items are sent immediately
        cx.run_until_parked();
        let (reject_request, respond_tx) = requests.reject.next().await.unwrap();
        respond_tx.send(()).unwrap();

        assert_eq!(reject_request.rejections.len(), 50);
        assert_eq!(reject_request.rejections[0].request_id, "batch-0");
        assert_eq!(reject_request.rejections[49].request_id, "batch-49");

        // Remaining items are debounced with the next batch
        cx.executor().advance_clock(Duration::from_secs(15));
        cx.run_until_parked();

        let (reject_request, respond_tx) = requests.reject.next().await.unwrap();
        respond_tx.send(()).unwrap();

        assert_eq!(reject_request.rejections.len(), 20);
        assert_eq!(reject_request.rejections[0].request_id, "batch-50");
        assert_eq!(reject_request.rejections[19].request_id, "batch-69");

        // Request failure
        zeta.update(cx, |zeta, _cx| {
            zeta.reject_prediction(
                EditPredictionId("retry-1".into()),
                EditPredictionRejectReason::Discarded,
                false,
            );
        });

        cx.executor().advance_clock(REJECT_REQUEST_DEBOUNCE);
        cx.run_until_parked();

        let (reject_request, _respond_tx) = requests.reject.next().await.unwrap();
        assert_eq!(reject_request.rejections.len(), 1);
        assert_eq!(reject_request.rejections[0].request_id, "retry-1");
        // Simulate failure
        drop(_respond_tx);

        // Add another rejection
        zeta.update(cx, |zeta, _cx| {
            zeta.reject_prediction(
                EditPredictionId("retry-2".into()),
                EditPredictionRejectReason::Discarded,
                false,
            );
        });

        cx.executor().advance_clock(REJECT_REQUEST_DEBOUNCE);
        cx.run_until_parked();

        // Retry should include both the failed item and the new one
        let (reject_request, respond_tx) = requests.reject.next().await.unwrap();
        respond_tx.send(()).unwrap();

        assert_eq!(reject_request.rejections.len(), 2);
        assert_eq!(reject_request.rejections[0].request_id, "retry-1");
        assert_eq!(reject_request.rejections[1].request_id, "retry-2");
    }

    // Skipped until we start including diagnostics in prompt
    // #[gpui::test]
    // async fn test_request_diagnostics(cx: &mut TestAppContext) {
    //     let (zeta, mut req_rx) = init_test(cx);
    //     let fs = FakeFs::new(cx.executor());
    //     fs.insert_tree(
    //         "/root",
    //         json!({
    //             "foo.md": "Hello!\nBye"
    //         }),
    //     )
    //     .await;
    //     let project = Project::test(fs, vec![path!("/root").as_ref()], cx).await;

    //     let path_to_buffer_uri = lsp::Uri::from_file_path(path!("/root/foo.md")).unwrap();
    //     let diagnostic = lsp::Diagnostic {
    //         range: lsp::Range::new(lsp::Position::new(1, 1), lsp::Position::new(1, 5)),
    //         severity: Some(lsp::DiagnosticSeverity::ERROR),
    //         message: "\"Hello\" deprecated. Use \"Hi\" instead".to_string(),
    //         ..Default::default()
    //     };

    //     project.update(cx, |project, cx| {
    //         project.lsp_store().update(cx, |lsp_store, cx| {
    //             // Create some diagnostics
    //             lsp_store
    //                 .update_diagnostics(
    //                     LanguageServerId(0),
    //                     lsp::PublishDiagnosticsParams {
    //                         uri: path_to_buffer_uri.clone(),
    //                         diagnostics: vec![diagnostic],
    //                         version: None,
    //                     },
    //                     None,
    //                     language::DiagnosticSourceKind::Pushed,
    //                     &[],
    //                     cx,
    //                 )
    //                 .unwrap();
    //         });
    //     });

    //     let buffer = project
    //         .update(cx, |project, cx| {
    //             let path = project.find_project_path(path!("root/foo.md"), cx).unwrap();
    //             project.open_buffer(path, cx)
    //         })
    //         .await
    //         .unwrap();

    //     let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
    //     let position = snapshot.anchor_before(language::Point::new(0, 0));

    //     let _prediction_task = zeta.update(cx, |zeta, cx| {
    //         zeta.request_prediction(&project, &buffer, position, cx)
    //     });

    //     let (request, _respond_tx) = req_rx.next().await.unwrap();

    //     assert_eq!(request.diagnostic_groups.len(), 1);
    //     let value = serde_json::from_str::<serde_json::Value>(request.diagnostic_groups[0].0.get())
    //         .unwrap();
    //     // We probably don't need all of this. TODO define a specific diagnostic type in predict_edits_v3
    //     assert_eq!(
    //         value,
    //         json!({
    //             "entries": [{
    //                 "range": {
    //                     "start": 8,
    //                     "end": 10
    //                 },
    //                 "diagnostic": {
    //                     "source": null,
    //                     "code": null,
    //                     "code_description": null,
    //                     "severity": 1,
    //                     "message": "\"Hello\" deprecated. Use \"Hi\" instead",
    //                     "markdown": null,
    //                     "group_id": 0,
    //                     "is_primary": true,
    //                     "is_disk_based": false,
    //                     "is_unnecessary": false,
    //                     "source_kind": "Pushed",
    //                     "data": null,
    //                     "underline": true
    //                 }
    //             }],
    //             "primary_ix": 0
    //         })
    //     );
    // }

    fn model_response(text: &str) -> open_ai::Response {
        open_ai::Response {
            id: Uuid::new_v4().to_string(),
            object: "response".into(),
            created: 0,
            model: "model".into(),
            choices: vec![open_ai::Choice {
                index: 0,
                message: open_ai::RequestMessage::Assistant {
                    content: Some(open_ai::MessageContent::Plain(text.to_string())),
                    tool_calls: vec![],
                },
                finish_reason: None,
            }],
            usage: Usage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0,
            },
        }
    }

    fn prompt_from_request(request: &open_ai::Request) -> &str {
        assert_eq!(request.messages.len(), 1);
        let open_ai::RequestMessage::User {
            content: open_ai::MessageContent::Plain(content),
            ..
        } = &request.messages[0]
        else {
            panic!(
                "Request does not have single user message of type Plain. {:#?}",
                request
            );
        };
        content
    }

    struct RequestChannels {
        predict: mpsc::UnboundedReceiver<(open_ai::Request, oneshot::Sender<open_ai::Response>)>,
        reject: mpsc::UnboundedReceiver<(RejectEditPredictionsBody, oneshot::Sender<()>)>,
    }

    fn init_test(cx: &mut TestAppContext) -> (Entity<Zeta>, RequestChannels) {
        cx.update(move |cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            zlog::init_test();

            let (predict_req_tx, predict_req_rx) = mpsc::unbounded();
            let (reject_req_tx, reject_req_rx) = mpsc::unbounded();

            let http_client = FakeHttpClient::create({
                move |req| {
                    let uri = req.uri().path().to_string();
                    let mut body = req.into_body();
                    let predict_req_tx = predict_req_tx.clone();
                    let reject_req_tx = reject_req_tx.clone();
                    async move {
                        let resp = match uri.as_str() {
                            "/client/llm_tokens" => serde_json::to_string(&json!({
                                "token": "test"
                            }))
                            .unwrap(),
                            "/predict_edits/raw" => {
                                let mut buf = Vec::new();
                                body.read_to_end(&mut buf).await.ok();
                                let req = serde_json::from_slice(&buf).unwrap();
                                let (res_tx, res_rx) = oneshot::channel();
                                predict_req_tx.unbounded_send((req, res_tx)).unwrap();
                                serde_json::to_string(&res_rx.await?).unwrap()
                            }
                            "/predict_edits/reject" => {
                                let mut buf = Vec::new();
                                body.read_to_end(&mut buf).await.ok();
                                let req = serde_json::from_slice(&buf).unwrap();

                                let (res_tx, res_rx) = oneshot::channel();
                                reject_req_tx.unbounded_send((req, res_tx)).unwrap();
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

            (
                zeta,
                RequestChannels {
                    predict: predict_req_rx,
                    reject: reject_req_rx,
                },
            )
        })
    }
}
