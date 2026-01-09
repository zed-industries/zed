use anyhow::Result;
use arrayvec::ArrayVec;
use client::{Client, EditPredictionUsage, UserStore};
use cloud_llm_client::predict_edits_v3::{self, PromptFormat};
use cloud_llm_client::{
    EXPIRED_LLM_TOKEN_HEADER_NAME, EditPredictionRejectReason, EditPredictionRejection,
    MAX_EDIT_PREDICTION_REJECTIONS_PER_REQUEST, MINIMUM_REQUIRED_VERSION_HEADER_NAME,
    PredictEditsRequestTrigger, RejectEditPredictionsBodyRef, ZED_VERSION_HEADER_NAME,
};
use collections::{HashMap, HashSet};
use db::kvp::{Dismissable, KEY_VALUE_STORE};
use edit_prediction_context::EditPredictionExcerptOptions;
use edit_prediction_context::{RelatedExcerptStore, RelatedExcerptStoreEvent, RelatedFile};
use feature_flags::{FeatureFlag, FeatureFlagAppExt as _};
use futures::{
    AsyncReadExt as _, FutureExt as _, StreamExt as _,
    channel::mpsc::{self, UnboundedReceiver},
    select_biased,
};
use gpui::BackgroundExecutor;
use gpui::http_client::Url;
use gpui::{
    App, AsyncApp, Entity, EntityId, Global, SharedString, Subscription, Task, WeakEntity, actions,
    http_client::{self, AsyncBody, Method},
    prelude::*,
};
use language::language_settings::all_language_settings;
use language::{Anchor, Buffer, File, Point, TextBufferSnapshot, ToOffset, ToPoint};
use language::{BufferSnapshot, OffsetRangeExt};
use language_model::{LlmApiToken, RefreshLlmTokenListener};
use project::{Project, ProjectPath, WorktreeId};
use release_channel::AppVersion;
use semver::Version;
use serde::de::DeserializeOwned;
use settings::{EditPredictionProvider, SettingsStore, update_settings_file};
use std::collections::{VecDeque, hash_map};
use text::Edit;
use workspace::Workspace;

use std::ops::Range;
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr as _;
use std::sync::{Arc, LazyLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::{env, mem};
use thiserror::Error;
use util::{RangeExt as _, ResultExt as _};
use workspace::notifications::{ErrorMessagePrompt, NotificationId, show_app_notification};

pub mod cursor_excerpt;
pub mod example_spec;
mod license_detection;
pub mod mercury;
mod onboarding_modal;
pub mod open_ai_response;
mod prediction;
pub mod sweep_ai;

pub mod udiff;

mod capture_example;
mod zed_edit_prediction_delegate;
pub mod zeta1;
pub mod zeta2;

#[cfg(test)]
mod edit_prediction_tests;

use crate::capture_example::should_sample_edit_prediction_example_capture;
use crate::license_detection::LicenseDetectionWatcher;
use crate::mercury::Mercury;
use crate::onboarding_modal::ZedPredictModal;
pub use crate::prediction::EditPrediction;
pub use crate::prediction::EditPredictionId;
use crate::prediction::EditPredictionResult;
pub use crate::sweep_ai::SweepAi;
pub use capture_example::capture_example;
pub use language_model::ApiKeyState;
pub use telemetry_events::EditPredictionRating;
pub use zed_edit_prediction_delegate::ZedEditPredictionDelegate;

actions!(
    edit_prediction,
    [
        /// Resets the edit prediction onboarding state.
        ResetOnboarding,
        /// Clears the edit prediction history.
        ClearHistory,
    ]
);

/// Maximum number of events to track.
const EVENT_COUNT_MAX: usize = 6;
const CHANGE_GROUPING_LINE_SPAN: u32 = 8;
const LAST_CHANGE_GROUPING_TIME: Duration = Duration::from_secs(1);
const ZED_PREDICT_DATA_COLLECTION_CHOICE: &str = "zed_predict_data_collection_choice";
const REJECT_REQUEST_DEBOUNCE: Duration = Duration::from_secs(15);

pub struct SweepFeatureFlag;

impl FeatureFlag for SweepFeatureFlag {
    const NAME: &str = "sweep-ai";
}

pub struct MercuryFeatureFlag;

impl FeatureFlag for MercuryFeatureFlag {
    const NAME: &str = "mercury";
}

pub const DEFAULT_OPTIONS: ZetaOptions = ZetaOptions {
    context: EditPredictionExcerptOptions {
        max_bytes: 512,
        min_bytes: 128,
        target_before_cursor_over_total_bytes: 0.5,
    },
    prompt_format: PromptFormat::DEFAULT,
};

static USE_OLLAMA: LazyLock<bool> =
    LazyLock::new(|| env::var("ZED_ZETA2_OLLAMA").is_ok_and(|var| !var.is_empty()));

static EDIT_PREDICTIONS_MODEL_ID: LazyLock<String> = LazyLock::new(|| {
    match env::var("ZED_ZETA2_MODEL").as_deref() {
        Ok("zeta2-exp") => "4w5n28vw", // Fine-tuned model @ Baseten
        Ok(model) => model,
        Err(_) if *USE_OLLAMA => "qwen3-coder:30b",
        Err(_) => "yqvev8r3", // Vanilla qwen3-coder @ Baseten
    }
    .to_string()
});

pub struct Zeta2FeatureFlag;

impl FeatureFlag for Zeta2FeatureFlag {
    const NAME: &'static str = "zeta2";

    fn enabled_for_staff() -> bool {
        true
    }
}

pub struct EditPredictionExampleCaptureFeatureFlag;

impl FeatureFlag for EditPredictionExampleCaptureFeatureFlag {
    const NAME: &'static str = "edit-prediction-example-capture";

    fn enabled_for_staff() -> bool {
        true
    }
}

#[derive(Clone)]
struct EditPredictionStoreGlobal(Entity<EditPredictionStore>);

impl Global for EditPredictionStoreGlobal {}

pub struct EditPredictionStore {
    client: Arc<Client>,
    user_store: Entity<UserStore>,
    llm_token: LlmApiToken,
    _llm_token_subscription: Subscription,
    projects: HashMap<EntityId, ProjectState>,
    use_context: bool,
    options: ZetaOptions,
    update_required: bool,
    #[cfg(feature = "cli-support")]
    eval_cache: Option<Arc<dyn EvalCache>>,
    edit_prediction_model: EditPredictionModel,
    pub sweep_ai: SweepAi,
    pub mercury: Mercury,
    data_collection_choice: DataCollectionChoice,
    reject_predictions_tx: mpsc::UnboundedSender<EditPredictionRejection>,
    shown_predictions: VecDeque<EditPrediction>,
    rated_predictions: HashSet<EditPredictionId>,
    custom_predict_edits_url: Option<Arc<Url>>,
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub enum EditPredictionModel {
    #[default]
    Zeta1,
    Zeta2,
    Sweep,
    Mercury,
}

pub struct EditPredictionModelInput {
    project: Entity<Project>,
    buffer: Entity<Buffer>,
    snapshot: BufferSnapshot,
    position: Anchor,
    events: Vec<Arc<zeta_prompt::Event>>,
    related_files: Arc<[RelatedFile]>,
    recent_paths: VecDeque<ProjectPath>,
    trigger: PredictEditsRequestTrigger,
    diagnostic_search_range: Range<Point>,
    debug_tx: Option<mpsc::UnboundedSender<DebugEvent>>,
    pub user_actions: Vec<UserActionRecord>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ZetaOptions {
    pub context: EditPredictionExcerptOptions,
    pub prompt_format: predict_edits_v3::PromptFormat,
}

#[derive(Debug)]
pub enum DebugEvent {
    ContextRetrievalStarted(ContextRetrievalStartedDebugEvent),
    ContextRetrievalFinished(ContextRetrievalFinishedDebugEvent),
    EditPredictionStarted(EditPredictionStartedDebugEvent),
    EditPredictionFinished(EditPredictionFinishedDebugEvent),
}

#[derive(Debug)]
pub struct ContextRetrievalStartedDebugEvent {
    pub project_entity_id: EntityId,
    pub timestamp: Instant,
    pub search_prompt: String,
}

#[derive(Debug)]
pub struct ContextRetrievalFinishedDebugEvent {
    pub project_entity_id: EntityId,
    pub timestamp: Instant,
    pub metadata: Vec<(&'static str, SharedString)>,
}

#[derive(Debug)]
pub struct EditPredictionStartedDebugEvent {
    pub buffer: WeakEntity<Buffer>,
    pub position: Anchor,
    pub prompt: Option<String>,
}

#[derive(Debug)]
pub struct EditPredictionFinishedDebugEvent {
    pub buffer: WeakEntity<Buffer>,
    pub position: Anchor,
    pub model_output: Option<String>,
}

pub type RequestDebugInfo = predict_edits_v3::DebugInfo;

const USER_ACTION_HISTORY_SIZE: usize = 16;

#[derive(Clone, Debug)]
pub struct UserActionRecord {
    pub action_type: UserActionType,
    pub buffer_id: EntityId,
    pub line_number: u32,
    pub offset: usize,
    pub timestamp_epoch_ms: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserActionType {
    InsertChar,
    InsertSelection,
    DeleteChar,
    DeleteSelection,
    CursorMovement,
}

/// An event with associated metadata for reconstructing buffer state.
#[derive(Clone)]
pub struct StoredEvent {
    pub event: Arc<zeta_prompt::Event>,
    pub old_snapshot: TextBufferSnapshot,
}

struct ProjectState {
    events: VecDeque<StoredEvent>,
    last_event: Option<LastEvent>,
    recent_paths: VecDeque<ProjectPath>,
    registered_buffers: HashMap<gpui::EntityId, RegisteredBuffer>,
    current_prediction: Option<CurrentEditPrediction>,
    next_pending_prediction_id: usize,
    pending_predictions: ArrayVec<PendingPrediction, 2>,
    debug_tx: Option<mpsc::UnboundedSender<DebugEvent>>,
    last_prediction_refresh: Option<(EntityId, Instant)>,
    cancelled_predictions: HashSet<usize>,
    context: Entity<RelatedExcerptStore>,
    license_detection_watchers: HashMap<WorktreeId, Rc<LicenseDetectionWatcher>>,
    user_actions: VecDeque<UserActionRecord>,
    _subscription: gpui::Subscription,
}

impl ProjectState {
    fn record_user_action(&mut self, action: UserActionRecord) {
        if self.user_actions.len() >= USER_ACTION_HISTORY_SIZE {
            self.user_actions.pop_front();
        }
        self.user_actions.push_back(action);
    }

    pub fn events(&self, cx: &App) -> Vec<StoredEvent> {
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

    pub fn events_split_by_pause(&self, cx: &App) -> Vec<StoredEvent> {
        self.events
            .iter()
            .cloned()
            .chain(self.last_event.as_ref().iter().flat_map(|event| {
                let (one, two) = event.split_by_pause();
                let one = one.finalize(&self.license_detection_watchers, cx);
                let two = two.and_then(|two| two.finalize(&self.license_detection_watchers, cx));
                one.into_iter().chain(two)
            }))
            .collect()
    }

    fn cancel_pending_prediction(
        &mut self,
        pending_prediction: PendingPrediction,
        cx: &mut Context<EditPredictionStore>,
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

    fn active_buffer(
        &self,
        project: &Entity<Project>,
        cx: &App,
    ) -> Option<(Entity<Buffer>, Option<Anchor>)> {
        let project = project.read(cx);
        let active_path = project.path_for_entry(project.active_entry()?, cx)?;
        let active_buffer = project.buffer_store().read(cx).get_by_path(&active_path)?;
        let registered_buffer = self.registered_buffers.get(&active_buffer.entity_id())?;
        Some((active_buffer, registered_buffer.last_position))
    }
}

#[derive(Debug, Clone)]
struct CurrentEditPrediction {
    pub requested_by: PredictionRequestedBy,
    pub prediction: EditPrediction,
    pub was_shown: bool,
    pub shown_with: Option<edit_prediction_types::SuggestionDisplayType>,
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
    file: Option<Arc<dyn File>>,
    snapshot: TextBufferSnapshot,
    last_position: Option<Anchor>,
    _subscriptions: [gpui::Subscription; 2],
}

#[derive(Clone)]
struct LastEvent {
    old_snapshot: TextBufferSnapshot,
    new_snapshot: TextBufferSnapshot,
    old_file: Option<Arc<dyn File>>,
    new_file: Option<Arc<dyn File>>,
    edit_range: Option<Range<Anchor>>,
    snapshot_after_last_editing_pause: Option<TextBufferSnapshot>,
    last_edit_time: Option<Instant>,
}

impl LastEvent {
    pub fn finalize(
        &self,
        license_detection_watchers: &HashMap<WorktreeId, Rc<LicenseDetectionWatcher>>,
        cx: &App,
    ) -> Option<StoredEvent> {
        let path = buffer_path_with_id_fallback(self.new_file.as_ref(), &self.new_snapshot, cx);
        let old_path = buffer_path_with_id_fallback(self.old_file.as_ref(), &self.old_snapshot, cx);

        let in_open_source_repo =
            [self.new_file.as_ref(), self.old_file.as_ref()]
                .iter()
                .all(|file| {
                    file.is_some_and(|file| {
                        license_detection_watchers
                            .get(&file.worktree_id(cx))
                            .is_some_and(|watcher| watcher.is_project_open_source())
                    })
                });

        let diff = compute_diff_between_snapshots(&self.old_snapshot, &self.new_snapshot)?;

        if path == old_path && diff.is_empty() {
            None
        } else {
            Some(StoredEvent {
                event: Arc::new(zeta_prompt::Event::BufferChange {
                    old_path,
                    path,
                    diff,
                    in_open_source_repo,
                    // TODO: Actually detect if this edit was predicted or not
                    predicted: false,
                }),
                old_snapshot: self.old_snapshot.clone(),
            })
        }
    }

    pub fn split_by_pause(&self) -> (LastEvent, Option<LastEvent>) {
        let Some(boundary_snapshot) = self.snapshot_after_last_editing_pause.as_ref() else {
            return (self.clone(), None);
        };

        let before = LastEvent {
            old_snapshot: self.old_snapshot.clone(),
            new_snapshot: boundary_snapshot.clone(),
            old_file: self.old_file.clone(),
            new_file: self.new_file.clone(),
            edit_range: None,
            snapshot_after_last_editing_pause: None,
            last_edit_time: self.last_edit_time,
        };

        let after = LastEvent {
            old_snapshot: boundary_snapshot.clone(),
            new_snapshot: self.new_snapshot.clone(),
            old_file: self.old_file.clone(),
            new_file: self.new_file.clone(),
            edit_range: None,
            snapshot_after_last_editing_pause: None,
            last_edit_time: self.last_edit_time,
        };

        (before, Some(after))
    }
}

pub(crate) fn compute_diff_between_snapshots(
    old_snapshot: &TextBufferSnapshot,
    new_snapshot: &TextBufferSnapshot,
) -> Option<String> {
    let edits: Vec<Edit<usize>> = new_snapshot
        .edits_since::<usize>(&old_snapshot.version)
        .collect();

    let (first_edit, last_edit) = edits.first().zip(edits.last())?;

    let old_start_point = old_snapshot.offset_to_point(first_edit.old.start);
    let old_end_point = old_snapshot.offset_to_point(last_edit.old.end);
    let new_start_point = new_snapshot.offset_to_point(first_edit.new.start);
    let new_end_point = new_snapshot.offset_to_point(last_edit.new.end);

    const CONTEXT_LINES: u32 = 3;

    let old_context_start_row = old_start_point.row.saturating_sub(CONTEXT_LINES);
    let new_context_start_row = new_start_point.row.saturating_sub(CONTEXT_LINES);
    let old_context_end_row =
        (old_end_point.row + 1 + CONTEXT_LINES).min(old_snapshot.max_point().row);
    let new_context_end_row =
        (new_end_point.row + 1 + CONTEXT_LINES).min(new_snapshot.max_point().row);

    let old_start_line_offset = old_snapshot.point_to_offset(Point::new(old_context_start_row, 0));
    let new_start_line_offset = new_snapshot.point_to_offset(Point::new(new_context_start_row, 0));
    let old_end_line_offset = old_snapshot
        .point_to_offset(Point::new(old_context_end_row + 1, 0).min(old_snapshot.max_point()));
    let new_end_line_offset = new_snapshot
        .point_to_offset(Point::new(new_context_end_row + 1, 0).min(new_snapshot.max_point()));
    let old_edit_range = old_start_line_offset..old_end_line_offset;
    let new_edit_range = new_start_line_offset..new_end_line_offset;

    let old_region_text: String = old_snapshot.text_for_range(old_edit_range).collect();
    let new_region_text: String = new_snapshot.text_for_range(new_edit_range).collect();

    let diff = language::unified_diff_with_offsets(
        &old_region_text,
        &new_region_text,
        old_context_start_row,
        new_context_start_row,
    );

    Some(diff)
}

fn buffer_path_with_id_fallback(
    file: Option<&Arc<dyn File>>,
    snapshot: &TextBufferSnapshot,
    cx: &App,
) -> Arc<Path> {
    if let Some(file) = file {
        file.full_path(cx).into()
    } else {
        Path::new(&format!("untitled-{}", snapshot.remote_id())).into()
    }
}

impl EditPredictionStore {
    pub fn try_global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<EditPredictionStoreGlobal>()
            .map(|global| global.0.clone())
    }

    pub fn global(
        client: &Arc<Client>,
        user_store: &Entity<UserStore>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.try_global::<EditPredictionStoreGlobal>()
            .map(|global| global.0.clone())
            .unwrap_or_else(|| {
                let ep_store = cx.new(|cx| Self::new(client.clone(), user_store.clone(), cx));
                cx.set_global(EditPredictionStoreGlobal(ep_store.clone()));
                ep_store
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
            #[cfg(feature = "cli-support")]
            eval_cache: None,
            edit_prediction_model: EditPredictionModel::Zeta2,
            sweep_ai: SweepAi::new(cx),
            mercury: Mercury::new(cx),
            data_collection_choice,
            reject_predictions_tx: reject_tx,
            rated_predictions: Default::default(),
            shown_predictions: Default::default(),
            custom_predict_edits_url: match env::var("ZED_PREDICT_EDITS_URL") {
                Ok(custom_url) => Url::parse(&custom_url).log_err().map(Into::into),
                Err(_) => {
                    if *USE_OLLAMA {
                        Some(
                            Url::parse("http://localhost:11434/v1/chat/completions")
                                .unwrap()
                                .into(),
                        )
                    } else {
                        None
                    }
                }
            },
        };

        this.configure_context_retrieval(cx);
        let weak_this = cx.weak_entity();
        cx.on_flags_ready(move |_, cx| {
            weak_this
                .update(cx, |this, cx| this.configure_context_retrieval(cx))
                .ok();
        })
        .detach();
        cx.observe_global::<SettingsStore>(|this, cx| {
            this.configure_context_retrieval(cx);
        })
        .detach();

        this
    }

    #[cfg(test)]
    pub fn set_custom_predict_edits_url(&mut self, url: Url) {
        self.custom_predict_edits_url = Some(url.into());
    }

    pub fn set_edit_prediction_model(&mut self, model: EditPredictionModel) {
        self.edit_prediction_model = model;
    }

    pub fn has_sweep_api_token(&self, cx: &App) -> bool {
        self.sweep_ai.api_token.read(cx).has_key()
    }

    pub fn has_mercury_api_token(&self, cx: &App) -> bool {
        self.mercury.api_token.read(cx).has_key()
    }

    #[cfg(feature = "cli-support")]
    pub fn with_eval_cache(&mut self, cache: Arc<dyn EvalCache>) {
        self.eval_cache = Some(cache);
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
        for project_state in self.projects.values_mut() {
            project_state.events.clear();
            project_state.last_event.take();
        }
    }

    pub fn clear_history_for_project(&mut self, project: &Entity<Project>) {
        if let Some(project_state) = self.projects.get_mut(&project.entity_id()) {
            project_state.events.clear();
            project_state.last_event.take();
        }
    }

    pub fn edit_history_for_project(
        &self,
        project: &Entity<Project>,
        cx: &App,
    ) -> Vec<StoredEvent> {
        self.projects
            .get(&project.entity_id())
            .map(|project_state| project_state.events(cx))
            .unwrap_or_default()
    }

    pub fn edit_history_for_project_with_pause_split_last_event(
        &self,
        project: &Entity<Project>,
        cx: &App,
    ) -> Vec<StoredEvent> {
        self.projects
            .get(&project.entity_id())
            .map(|project_state| project_state.events_split_by_pause(cx))
            .unwrap_or_default()
    }

    pub fn context_for_project<'a>(
        &'a self,
        project: &Entity<Project>,
        cx: &'a App,
    ) -> Arc<[RelatedFile]> {
        self.projects
            .get(&project.entity_id())
            .map(|project| project.context.read(cx).related_files())
            .unwrap_or_else(|| vec![].into())
    }

    pub fn context_for_project_with_buffers<'a>(
        &'a self,
        project: &Entity<Project>,
        cx: &'a App,
    ) -> Option<impl 'a + Iterator<Item = (RelatedFile, Entity<Buffer>)>> {
        self.projects
            .get(&project.entity_id())
            .map(|project| project.context.read(cx).related_files_with_buffers())
    }

    pub fn usage(&self, cx: &App) -> Option<EditPredictionUsage> {
        if self.edit_prediction_model == EditPredictionModel::Zeta2 {
            self.user_store.read(cx).edit_prediction_usage()
        } else {
            None
        }
    }

    pub fn register_project(&mut self, project: &Entity<Project>, cx: &mut Context<Self>) {
        self.get_or_init_project(project, cx);
    }

    pub fn register_buffer(
        &mut self,
        buffer: &Entity<Buffer>,
        project: &Entity<Project>,
        cx: &mut Context<Self>,
    ) {
        let project_state = self.get_or_init_project(project, cx);
        Self::register_buffer_impl(project_state, buffer, project, cx);
    }

    fn get_or_init_project(
        &mut self,
        project: &Entity<Project>,
        cx: &mut Context<Self>,
    ) -> &mut ProjectState {
        let entity_id = project.entity_id();
        self.projects
            .entry(entity_id)
            .or_insert_with(|| ProjectState {
                context: {
                    let related_excerpt_store = cx.new(|cx| RelatedExcerptStore::new(project, cx));
                    cx.subscribe(&related_excerpt_store, move |this, _, event, _| {
                        this.handle_excerpt_store_event(entity_id, event);
                    })
                    .detach();
                    related_excerpt_store
                },
                events: VecDeque::new(),
                last_event: None,
                recent_paths: VecDeque::new(),
                debug_tx: None,
                registered_buffers: HashMap::default(),
                current_prediction: None,
                cancelled_predictions: HashSet::default(),
                pending_predictions: ArrayVec::new(),
                next_pending_prediction_id: 0,
                last_prediction_refresh: None,
                license_detection_watchers: HashMap::default(),
                user_actions: VecDeque::with_capacity(USER_ACTION_HISTORY_SIZE),
                _subscription: cx.subscribe(&project, Self::handle_project_event),
            })
    }

    pub fn remove_project(&mut self, project: &Entity<Project>) {
        self.projects.remove(&project.entity_id());
    }

    fn handle_excerpt_store_event(
        &mut self,
        project_entity_id: EntityId,
        event: &RelatedExcerptStoreEvent,
    ) {
        if let Some(project_state) = self.projects.get(&project_entity_id) {
            if let Some(debug_tx) = project_state.debug_tx.clone() {
                match event {
                    RelatedExcerptStoreEvent::StartedRefresh => {
                        debug_tx
                            .unbounded_send(DebugEvent::ContextRetrievalStarted(
                                ContextRetrievalStartedDebugEvent {
                                    project_entity_id: project_entity_id,
                                    timestamp: Instant::now(),
                                    search_prompt: String::new(),
                                },
                            ))
                            .ok();
                    }
                    RelatedExcerptStoreEvent::FinishedRefresh {
                        cache_hit_count,
                        cache_miss_count,
                        mean_definition_latency,
                        max_definition_latency,
                    } => {
                        debug_tx
                            .unbounded_send(DebugEvent::ContextRetrievalFinished(
                                ContextRetrievalFinishedDebugEvent {
                                    project_entity_id: project_entity_id,
                                    timestamp: Instant::now(),
                                    metadata: vec![
                                        (
                                            "Cache Hits",
                                            format!(
                                                "{}/{}",
                                                cache_hit_count,
                                                cache_hit_count + cache_miss_count
                                            )
                                            .into(),
                                        ),
                                        (
                                            "Max LSP Time",
                                            format!("{} ms", max_definition_latency.as_millis())
                                                .into(),
                                        ),
                                        (
                                            "Mean LSP Time",
                                            format!("{} ms", mean_definition_latency.as_millis())
                                                .into(),
                                        ),
                                    ],
                                },
                            ))
                            .ok();
                    }
                }
            }
        }
    }

    pub fn debug_info(
        &mut self,
        project: &Entity<Project>,
        cx: &mut Context<Self>,
    ) -> mpsc::UnboundedReceiver<DebugEvent> {
        let project_state = self.get_or_init_project(project, cx);
        let (debug_watch_tx, debug_watch_rx) = mpsc::unbounded();
        project_state.debug_tx = Some(debug_watch_tx);
        debug_watch_rx
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
                let Some(project_state) = self.projects.get_mut(&project.entity_id()) else {
                    return;
                };
                let path = project.read(cx).path_for_entry(*active_entry_id, cx);
                if let Some(path) = path {
                    if let Some(ix) = project_state
                        .recent_paths
                        .iter()
                        .position(|probe| probe == &path)
                    {
                        project_state.recent_paths.remove(ix);
                    }
                    project_state.recent_paths.push_front(path);
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
        project_state: &'a mut ProjectState,
        buffer: &Entity<Buffer>,
        project: &Entity<Project>,
        cx: &mut Context<Self>,
    ) -> &'a mut RegisteredBuffer {
        let buffer_id = buffer.entity_id();

        if let Some(file) = buffer.read(cx).file() {
            let worktree_id = file.worktree_id(cx);
            if let Some(worktree) = project.read(cx).worktree_for_id(worktree_id, cx) {
                project_state
                    .license_detection_watchers
                    .entry(worktree_id)
                    .or_insert_with(|| {
                        let project_entity_id = project.entity_id();
                        cx.observe_release(&worktree, move |this, _worktree, _cx| {
                            let Some(project_state) = this.projects.get_mut(&project_entity_id)
                            else {
                                return;
                            };
                            project_state
                                .license_detection_watchers
                                .remove(&worktree_id);
                        })
                        .detach();
                        Rc::new(LicenseDetectionWatcher::new(&worktree, cx))
                    });
            }
        }

        match project_state.registered_buffers.entry(buffer_id) {
            hash_map::Entry::Occupied(entry) => entry.into_mut(),
            hash_map::Entry::Vacant(entry) => {
                let buf = buffer.read(cx);
                let snapshot = buf.text_snapshot();
                let file = buf.file().cloned();
                let project_entity_id = project.entity_id();
                entry.insert(RegisteredBuffer {
                    snapshot,
                    file,
                    last_position: None,
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
                            let Some(project_state) = this.projects.get_mut(&project_entity_id)
                            else {
                                return;
                            };
                            project_state.registered_buffers.remove(&buffer_id);
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
        let project_state = self.get_or_init_project(project, cx);
        let registered_buffer = Self::register_buffer_impl(project_state, buffer, project, cx);

        let buf = buffer.read(cx);
        let new_file = buf.file().cloned();
        let new_snapshot = buf.text_snapshot();
        if new_snapshot.version == registered_buffer.snapshot.version {
            return;
        }

        let old_file = mem::replace(&mut registered_buffer.file, new_file.clone());
        let old_snapshot = mem::replace(&mut registered_buffer.snapshot, new_snapshot.clone());
        let mut num_edits = 0usize;
        let mut total_deleted = 0usize;
        let mut total_inserted = 0usize;
        let mut edit_range: Option<Range<Anchor>> = None;
        let mut last_offset: Option<usize> = None;

        for (edit, anchor_range) in
            new_snapshot.anchored_edits_since::<usize>(&old_snapshot.version)
        {
            num_edits += 1;
            total_deleted += edit.old.len();
            total_inserted += edit.new.len();
            edit_range = Some(match edit_range {
                None => anchor_range,
                Some(acc) => acc.start..anchor_range.end,
            });
            last_offset = Some(edit.new.end);
        }

        if num_edits > 0 {
            let action_type = match (total_deleted, total_inserted, num_edits) {
                (0, ins, n) if ins == n => UserActionType::InsertChar,
                (0, _, _) => UserActionType::InsertSelection,
                (del, 0, n) if del == n => UserActionType::DeleteChar,
                (_, 0, _) => UserActionType::DeleteSelection,
                (_, ins, n) if ins == n => UserActionType::InsertChar,
                (_, _, _) => UserActionType::InsertSelection,
            };

            if let Some(offset) = last_offset {
                let point = new_snapshot.offset_to_point(offset);
                let timestamp_epoch_ms = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_millis() as u64)
                    .unwrap_or(0);
                project_state.record_user_action(UserActionRecord {
                    action_type,
                    buffer_id: buffer.entity_id(),
                    line_number: point.row,
                    offset,
                    timestamp_epoch_ms,
                });
            }
        }

        let events = &mut project_state.events;

        let now = cx.background_executor().now();
        if let Some(last_event) = project_state.last_event.as_mut() {
            let is_next_snapshot_of_same_buffer = old_snapshot.remote_id()
                == last_event.new_snapshot.remote_id()
                && old_snapshot.version == last_event.new_snapshot.version;

            let should_coalesce = is_next_snapshot_of_same_buffer
                && edit_range
                    .as_ref()
                    .zip(last_event.edit_range.as_ref())
                    .is_some_and(|(a, b)| {
                        let a = a.to_point(&new_snapshot);
                        let b = b.to_point(&new_snapshot);
                        if a.start > b.end {
                            a.start.row.abs_diff(b.end.row) <= CHANGE_GROUPING_LINE_SPAN
                        } else if b.start > a.end {
                            b.start.row.abs_diff(a.end.row) <= CHANGE_GROUPING_LINE_SPAN
                        } else {
                            true
                        }
                    });

            if should_coalesce {
                let pause_elapsed = last_event
                    .last_edit_time
                    .map(|t| now.duration_since(t) >= LAST_CHANGE_GROUPING_TIME)
                    .unwrap_or(false);
                if pause_elapsed {
                    last_event.snapshot_after_last_editing_pause =
                        Some(last_event.new_snapshot.clone());
                }

                last_event.edit_range = edit_range;
                last_event.new_snapshot = new_snapshot;
                last_event.last_edit_time = Some(now);
                return;
            }
        }

        if let Some(event) = project_state.last_event.take() {
            if let Some(event) = event.finalize(&project_state.license_detection_watchers, cx) {
                if events.len() + 1 >= EVENT_COUNT_MAX {
                    events.pop_front();
                }
                events.push_back(event);
            }
        }

        project_state.last_event = Some(LastEvent {
            old_file,
            new_file,
            old_snapshot,
            new_snapshot,
            edit_range,
            snapshot_after_last_editing_pause: None,
            last_edit_time: Some(now),
        });
    }

    fn prediction_at(
        &mut self,
        buffer: &Entity<Buffer>,
        position: Option<language::Anchor>,
        project: &Entity<Project>,
        cx: &App,
    ) -> Option<BufferEditPrediction<'_>> {
        let project_state = self.projects.get_mut(&project.entity_id())?;
        if let Some(position) = position
            && let Some(buffer) = project_state
                .registered_buffers
                .get_mut(&buffer.entity_id())
        {
            buffer.last_position = Some(position);
        }

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
        let Some(project_state) = self.projects.get_mut(&project.entity_id()) else {
            return;
        };

        let Some(current_prediction) = project_state.current_prediction.take() else {
            return;
        };

        for pending_prediction in mem::take(&mut project_state.pending_predictions) {
            project_state.cancel_pending_prediction(pending_prediction, cx);
        }

        match self.edit_prediction_model {
            EditPredictionModel::Sweep => {
                sweep_ai::edit_prediction_accepted(self, current_prediction, cx)
            }
            EditPredictionModel::Mercury => {}
            EditPredictionModel::Zeta1 | EditPredictionModel::Zeta2 => {
                zeta2::edit_prediction_accepted(self, current_prediction, cx)
            }
        }
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
                true,
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

    fn did_show_current_prediction(
        &mut self,
        project: &Entity<Project>,
        display_type: edit_prediction_types::SuggestionDisplayType,
        cx: &mut Context<Self>,
    ) {
        let Some(project_state) = self.projects.get_mut(&project.entity_id()) else {
            return;
        };

        let Some(current_prediction) = project_state.current_prediction.as_mut() else {
            return;
        };

        let is_jump = display_type == edit_prediction_types::SuggestionDisplayType::Jump;
        let previous_shown_with = current_prediction.shown_with;

        if previous_shown_with.is_none() || !is_jump {
            current_prediction.shown_with = Some(display_type);
        }

        let is_first_non_jump_show = !current_prediction.was_shown && !is_jump;

        if is_first_non_jump_show {
            current_prediction.was_shown = true;
        }

        let display_type_changed = previous_shown_with != Some(display_type);

        if self.edit_prediction_model == EditPredictionModel::Sweep && display_type_changed {
            sweep_ai::edit_prediction_shown(
                &self.sweep_ai,
                self.client.clone(),
                &current_prediction.prediction,
                display_type,
                cx,
            );
        }

        if is_first_non_jump_show {
            self.shown_predictions
                .push_front(current_prediction.prediction.clone());
            if self.shown_predictions.len() > 50 {
                let completion = self.shown_predictions.pop_back().unwrap();
                self.rated_predictions.remove(&completion.id);
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
            EditPredictionModel::Zeta1 | EditPredictionModel::Zeta2 => {
                if self.custom_predict_edits_url.is_some() {
                    return;
                }
            }
            EditPredictionModel::Sweep | EditPredictionModel::Mercury => return,
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
        let Some(project_state) = self.projects.get_mut(&project.entity_id()) else {
            return;
        };

        // Prefer predictions from buffer
        if project_state.current_prediction.is_some() {
            return;
        };

        self.queue_prediction_refresh(project.clone(), project.entity_id(), cx, move |this, cx| {
            let Some((active_buffer, snapshot, cursor_point)) = this
                .read_with(cx, |this, cx| {
                    let project_state = this.projects.get(&project.entity_id())?;
                    let (buffer, position) = project_state.active_buffer(&project, cx)?;
                    let snapshot = buffer.read(cx).snapshot();

                    if !Self::predictions_enabled_at(&snapshot, position, cx) {
                        return None;
                    }

                    let cursor_point = position
                        .map(|pos| pos.to_point(&snapshot))
                        .unwrap_or_default();

                    Some((buffer, snapshot, cursor_point))
                })
                .log_err()
                .flatten()
            else {
                return Task::ready(anyhow::Ok(None));
            };

            cx.spawn(async move |cx| {
                let Some((jump_buffer, jump_position)) = Self::next_diagnostic_location(
                    active_buffer,
                    &snapshot,
                    Default::default(),
                    cursor_point,
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
                            .get_or_init_project(&project, cx)
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

    fn predictions_enabled_at(
        snapshot: &BufferSnapshot,
        position: Option<language::Anchor>,
        cx: &App,
    ) -> bool {
        let file = snapshot.file();
        let all_settings = all_language_settings(file, cx);
        if !all_settings.show_edit_predictions(snapshot.language(), cx)
            || file.is_some_and(|file| !all_settings.edit_predictions_enabled_for_file(file, cx))
        {
            return false;
        }

        if let Some(last_position) = position {
            let settings = snapshot.settings_at(last_position, cx);

            if !settings.edit_predictions_disabled_in.is_empty()
                && let Some(scope) = snapshot.language_scope_at(last_position)
                && let Some(scope_name) = scope.override_name()
                && settings
                    .edit_predictions_disabled_in
                    .iter()
                    .any(|s| s == scope_name)
            {
                return false;
            }
        }

        true
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
        let project_state = self.get_or_init_project(&project, cx);
        let pending_prediction_id = project_state.next_pending_prediction_id;
        project_state.next_pending_prediction_id += 1;
        let last_request = project_state.last_prediction_refresh;

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
                let project_state = this.get_or_init_project(&project, cx);
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
                let project_state = this.get_or_init_project(&project, cx);

                let is_cancelled = project_state
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
                                shown_with: None,
                            };

                            if let Some(current_prediction) =
                                project_state.current_prediction.as_ref()
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

                let project_state = this.get_or_init_project(&project, cx);

                if let Some(new_prediction) = new_current_prediction {
                    project_state.current_prediction = Some(new_prediction);
                }

                let mut pending_predictions = mem::take(&mut project_state.pending_predictions);
                for (ix, pending_prediction) in pending_predictions.iter().enumerate() {
                    if pending_prediction.id == pending_prediction_id {
                        pending_predictions.remove(ix);
                        for pending_prediction in pending_predictions.drain(0..ix) {
                            project_state.cancel_pending_prediction(pending_prediction, cx)
                        }
                        break;
                    }
                }
                this.get_or_init_project(&project, cx).pending_predictions = pending_predictions;
                cx.notify();
            })
            .ok();

            new_prediction_id
        });

        if project_state.pending_predictions.len() <= 1 {
            project_state.pending_predictions.push(PendingPrediction {
                id: pending_prediction_id,
                task,
            });
        } else if project_state.pending_predictions.len() == 2 {
            let pending_prediction = project_state.pending_predictions.pop().unwrap();
            project_state.pending_predictions.push(PendingPrediction {
                id: pending_prediction_id,
                task,
            });
            project_state.cancel_pending_prediction(pending_prediction, cx);
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

        self.get_or_init_project(&project, cx);
        let project_state = self.projects.get(&project.entity_id()).unwrap();
        let stored_events = project_state.events(cx);
        let has_events = !stored_events.is_empty();
        let events: Vec<Arc<zeta_prompt::Event>> =
            stored_events.into_iter().map(|e| e.event).collect();
        let debug_tx = project_state.debug_tx.clone();

        let snapshot = active_buffer.read(cx).snapshot();
        let cursor_point = position.to_point(&snapshot);
        let current_offset = position.to_offset(&snapshot);

        let mut user_actions: Vec<UserActionRecord> =
            project_state.user_actions.iter().cloned().collect();

        if let Some(last_action) = user_actions.last() {
            if last_action.buffer_id == active_buffer.entity_id()
                && current_offset != last_action.offset
            {
                let timestamp_epoch_ms = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_millis() as u64)
                    .unwrap_or(0);
                user_actions.push(UserActionRecord {
                    action_type: UserActionType::CursorMovement,
                    buffer_id: active_buffer.entity_id(),
                    line_number: cursor_point.row,
                    offset: current_offset,
                    timestamp_epoch_ms,
                });
            }
        }
        let diagnostic_search_start = cursor_point.row.saturating_sub(DIAGNOSTIC_LINES_RANGE);
        let diagnostic_search_end = cursor_point.row + DIAGNOSTIC_LINES_RANGE;
        let diagnostic_search_range =
            Point::new(diagnostic_search_start, 0)..Point::new(diagnostic_search_end, 0);

        let related_files = if self.use_context {
            self.context_for_project(&project, cx)
        } else {
            Vec::new().into()
        };

        let inputs = EditPredictionModelInput {
            project: project.clone(),
            buffer: active_buffer.clone(),
            snapshot: snapshot.clone(),
            position,
            events,
            related_files,
            recent_paths: project_state.recent_paths.clone(),
            trigger,
            diagnostic_search_range: diagnostic_search_range.clone(),
            debug_tx,
            user_actions,
        };

        let can_collect_example = snapshot
            .file()
            .is_some_and(|file| self.can_collect_file(&project, file, cx))
            && self.can_collect_events(&inputs.events, cx);

        if can_collect_example && should_sample_edit_prediction_example_capture(cx) {
            let events_for_capture =
                self.edit_history_for_project_with_pause_split_last_event(&project, cx);
            if let Some(example_task) = capture_example::capture_example(
                project.clone(),
                active_buffer.clone(),
                position,
                events_for_capture,
                cx,
            ) {
                cx.spawn(async move |_this, _cx| {
                    let example = example_task.await?;
                    telemetry::event!("Edit Prediction Example Captured", example = example);
                    anyhow::Ok(())
                })
                .detach_and_log_err(cx);
            }
        }
        let task = match self.edit_prediction_model {
            EditPredictionModel::Zeta1 => zeta1::request_prediction_with_zeta1(self, inputs, cx),
            EditPredictionModel::Zeta2 => zeta2::request_prediction_with_zeta2(self, inputs, cx),
            EditPredictionModel::Sweep => self.sweep_ai.request_prediction_with_sweep(inputs, cx),
            EditPredictionModel::Mercury => self.mercury.request_prediction(inputs, cx),
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
            });

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
            });

            if let Some(buffer_task) = buffer_task {
                let closest_buffer = buffer_task.await?;

                jump_location = closest_buffer
                    .read_with(cx, |buffer, _cx| {
                        buffer
                            .buffer_diagnostics(None)
                            .into_iter()
                            .min_by_key(|entry| entry.diagnostic.severity)
                            .map(|entry| entry.range.start)
                    })
                    .map(|position| (closest_buffer, position));
            }
        }

        anyhow::Ok(jump_location)
    }

    async fn send_raw_llm_request(
        request: open_ai::Request,
        client: Arc<Client>,
        llm_token: LlmApiToken,
        app_version: Version,
        #[cfg(feature = "cli-support")] eval_cache: Option<Arc<dyn EvalCache>>,
        #[cfg(feature = "cli-support")] eval_cache_kind: EvalCacheEntryKind,
    ) -> Result<(open_ai::Response, Option<EditPredictionUsage>)> {
        let url = client
            .http_client()
            .build_zed_llm_url("/predict_edits/raw", &[])?;

        #[cfg(feature = "cli-support")]
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
            true,
        )
        .await?;

        #[cfg(feature = "cli-support")]
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
                    });
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
        require_auth: bool,
    ) -> Result<(Res, Option<EditPredictionUsage>)>
    where
        Res: DeserializeOwned,
    {
        let http_client = client.http_client();

        let mut token = if require_auth {
            Some(llm_token.acquire(&client).await?)
        } else {
            llm_token.acquire(&client).await.ok()
        };
        let mut did_retry = false;

        loop {
            let request_builder = http_client::Request::builder().method(Method::POST);

            let mut request_builder = request_builder
                .header("Content-Type", "application/json")
                .header(ZED_VERSION_HEADER_NAME, app_version.to_string());

            // Only add Authorization header if we have a token
            if let Some(ref token_value) = token {
                request_builder =
                    request_builder.header("Authorization", format!("Bearer {}", token_value));
            }

            let request = build(request_builder)?;

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
                && token.is_some()
                && response
                    .headers()
                    .get(EXPIRED_LLM_TOKEN_HEADER_NAME)
                    .is_some()
            {
                did_retry = true;
                token = Some(llm_token.refresh(&client).await?);
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

    pub fn refresh_context(
        &mut self,
        project: &Entity<Project>,
        buffer: &Entity<language::Buffer>,
        cursor_position: language::Anchor,
        cx: &mut Context<Self>,
    ) {
        if self.use_context {
            self.get_or_init_project(project, cx)
                .context
                .update(cx, |store, cx| {
                    store.refresh(buffer.clone(), cursor_position, cx);
                });
        }
    }

    #[cfg(feature = "cli-support")]
    pub fn set_context_for_buffer(
        &mut self,
        project: &Entity<Project>,
        related_files: Vec<RelatedFile>,
        cx: &mut Context<Self>,
    ) {
        self.get_or_init_project(project, cx)
            .context
            .update(cx, |store, _| {
                store.set_related_files(related_files);
            });
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
        let Some(project_state) = self.projects.get(&project.entity_id()) else {
            return false;
        };
        project_state
            .license_detection_watchers
            .get(&file.worktree_id(cx))
            .as_ref()
            .is_some_and(|watcher| watcher.is_project_open_source())
    }

    fn can_collect_file(&self, project: &Entity<Project>, file: &Arc<dyn File>, cx: &App) -> bool {
        self.data_collection_choice.is_enabled(cx) && self.is_file_open_source(project, file, cx)
    }

    fn can_collect_events(&self, events: &[Arc<zeta_prompt::Event>], cx: &App) -> bool {
        if !self.data_collection_choice.is_enabled(cx) {
            return false;
        }
        events.iter().all(|event| {
            matches!(
                event.as_ref(),
                zeta_prompt::Event::BufferChange {
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

    fn toggle_data_collection_choice(&mut self, cx: &mut Context<Self>) {
        self.data_collection_choice = self.data_collection_choice.toggle();
        let new_choice = self.data_collection_choice;
        let is_enabled = new_choice.is_enabled(cx);
        db::write_and_log(cx, move || {
            KEY_VALUE_STORE.write_kvp(
                ZED_PREDICT_DATA_COLLECTION_CHOICE.into(),
                is_enabled.to_string(),
            )
        });
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
            output = prediction
                .edit_preview
                .as_unified_diff(prediction.snapshot.file(), &prediction.edits),
            feedback
        );
        self.client.telemetry().flush_events().detach();
        cx.notify();
    }

    fn configure_context_retrieval(&mut self, cx: &mut Context<'_, EditPredictionStore>) {
        self.use_context = cx.has_flag::<Zeta2FeatureFlag>()
            && all_language_settings(None, cx).edit_predictions.use_context;
    }
}

#[derive(Error, Debug)]
#[error(
    "You must update to Zed version {minimum_version} or higher to continue using edit predictions."
)]
pub struct ZedUpdateRequiredError {
    minimum_version: Version,
}

#[cfg(feature = "cli-support")]
pub type EvalCacheKey = (EvalCacheEntryKind, u64);

#[cfg(feature = "cli-support")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EvalCacheEntryKind {
    Context,
    Search,
    Prediction,
}

#[cfg(feature = "cli-support")]
impl std::fmt::Display for EvalCacheEntryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalCacheEntryKind::Search => write!(f, "search"),
            EvalCacheEntryKind::Context => write!(f, "context"),
            EvalCacheEntryKind::Prediction => write!(f, "prediction"),
        }
    }
}

#[cfg(feature = "cli-support")]
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
    pub fn is_enabled(self, cx: &App) -> bool {
        if cx.is_staff() {
            return true;
        }
        match self {
            Self::Enabled => true,
            Self::NotAnswered | Self::Disabled => false,
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
    cx.observe_new(move |workspace: &mut Workspace, _, _cx| {
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
