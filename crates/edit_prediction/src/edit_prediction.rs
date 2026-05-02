use anyhow::Result;
use client::{Client, EditPredictionUsage, NeedsLlmTokenRefresh, UserStore, global_llm_token};
use cloud_api_client::LlmApiToken;
use cloud_api_types::{OrganizationId, SubmitEditPredictionFeedbackBody};
use cloud_llm_client::predict_edits_v3::{
    PREDICT_EDITS_MODE_HEADER_NAME, PredictEditsMode, PredictEditsV3Request,
    PredictEditsV3Response, RawCompletionRequest, RawCompletionResponse,
};
use cloud_llm_client::{
    EditPredictionRejectReason, EditPredictionRejection,
    MAX_EDIT_PREDICTION_REJECTIONS_PER_REQUEST, MINIMUM_REQUIRED_VERSION_HEADER_NAME,
    PREFERRED_EXPERIMENT_HEADER_NAME, PredictEditsRequestTrigger, RejectEditPredictionsBodyRef,
    ZED_VERSION_HEADER_NAME,
};
use collections::{HashMap, HashSet};
use copilot::{Copilot, Reinstall, SignIn, SignOut};
use credentials_provider::CredentialsProvider;
use db::kvp::{Dismissable, KeyValueStore};
use edit_prediction_context::{RelatedExcerptStore, RelatedExcerptStoreEvent, RelatedFile};
use feature_flags::{FeatureFlag, FeatureFlagAppExt as _, PresenceFlag, register_feature_flag};
use futures::{
    AsyncReadExt as _, FutureExt as _, StreamExt as _,
    channel::mpsc::{self, UnboundedReceiver},
    select_biased,
};
use gpui::BackgroundExecutor;
use gpui::http_client::Url;
use gpui::{
    App, AsyncApp, Entity, EntityId, Global, SharedString, Task, WeakEntity, actions,
    http_client::{self, AsyncBody, Method},
    prelude::*,
};
use heapless::Vec as ArrayVec;
use language::{
    Anchor, Buffer, BufferSnapshot, EditPredictionsMode, EditPreview, File, OffsetRangeExt, Point,
    TextBufferSnapshot, ToOffset, ToPoint, language_settings::all_language_settings,
};
use project::{DisableAiSettings, Project, ProjectPath, WorktreeId};
use release_channel::AppVersion;
use semver::Version;
use serde::de::DeserializeOwned;
use settings::{
    EditPredictionDataCollectionChoice, EditPredictionPromptFormat, EditPredictionProvider,
    Settings as _, update_settings_file,
};
use std::collections::{VecDeque, hash_map};
use std::env;
use text::{AnchorRangeExt, Edit};
use workspace::{AppState, Workspace};
use zeta_prompt::{ZetaFormat, ZetaPromptInput};

use std::mem;
use std::ops::Range;
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr as _;
use std::sync::Arc;
use std::time::{Duration, Instant};

use thiserror::Error;
use util::{RangeExt as _, ResultExt as _};

pub mod cursor_excerpt;
pub mod example_spec;
pub mod fim;
mod license_detection;
pub mod mercury;
pub mod metrics;
pub mod ollama;
mod onboarding_modal;
pub mod open_ai_response;
mod prediction;

pub mod udiff;

mod capture_example;
pub mod open_ai_compatible;
mod zed_edit_prediction_delegate;
pub mod zeta;

#[cfg(test)]
mod edit_prediction_tests;

use crate::cursor_excerpt::expand_context_syntactically_then_linewise;
use crate::example_spec::ExampleSpec;
use crate::license_detection::LicenseDetectionWatcher;
use crate::mercury::Mercury;
pub use crate::metrics::{KeptRateResult, compute_kept_rate};
use crate::onboarding_modal::ZedPredictModal;
pub use crate::prediction::EditPrediction;
pub use crate::prediction::EditPredictionId;
use crate::prediction::EditPredictionResult;
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
const EVENT_COUNT_MAX: usize = 10;
const CHANGE_GROUPING_LINE_SPAN: u32 = 8;
const EDIT_HISTORY_DIFF_SIZE_LIMIT: usize = 2048 * 3; // ~2048 tokens or ~50% of typical prompt budget
const COLLABORATOR_EDIT_LOCALITY_CONTEXT_TOKENS: usize = 512;
const LAST_CHANGE_GROUPING_TIME: Duration = Duration::from_secs(1);
const ZED_PREDICT_DATA_COLLECTION_CHOICE: &str = "zed_predict_data_collection_choice";
const REJECT_REQUEST_DEBOUNCE: Duration = Duration::from_secs(15);
const EDIT_PREDICTION_SETTLED_EVENT: &str = "Edit Prediction Settled";
const EDIT_PREDICTION_SETTLED_TTL: Duration = Duration::from_secs(60 * 5);
const EDIT_PREDICTION_SETTLED_QUIESCENCE: Duration = Duration::from_secs(10);

pub struct EditPredictionJumpsFeatureFlag;

impl FeatureFlag for EditPredictionJumpsFeatureFlag {
    const NAME: &'static str = "edit_prediction_jumps";
    type Value = PresenceFlag;
}
register_feature_flag!(EditPredictionJumpsFeatureFlag);

#[derive(Clone)]
struct EditPredictionStoreGlobal(Entity<EditPredictionStore>);

impl Global for EditPredictionStoreGlobal {}

/// Configuration for using the raw Zeta2 endpoint.
/// When set, the client uses the raw endpoint and constructs the prompt itself.
/// The version is also used as the Baseten environment name (lowercased).
#[derive(Clone)]
pub struct Zeta2RawConfig {
    pub model_id: Option<String>,
    pub environment: Option<String>,
    pub format: ZetaFormat,
}

pub struct EditPredictionStore {
    client: Arc<Client>,
    user_store: Entity<UserStore>,
    llm_token: LlmApiToken,
    _fetch_experiments_task: Task<()>,
    projects: HashMap<EntityId, ProjectState>,
    update_required: bool,
    edit_prediction_model: EditPredictionModel,
    zeta2_raw_config: Option<Zeta2RawConfig>,
    preferred_experiment: Option<String>,
    available_experiments: Vec<String>,
    pub mercury: Mercury,
    legacy_data_collection_enabled: bool,
    reject_predictions_tx: mpsc::UnboundedSender<EditPredictionRejectionPayload>,
    settled_predictions_tx: mpsc::UnboundedSender<Instant>,
    shown_predictions: VecDeque<EditPrediction>,
    rated_predictions: HashSet<EditPredictionId>,
    #[cfg(test)]
    settled_event_callback: Option<Box<dyn Fn(EditPredictionId, String)>>,
    credentials_provider: Arc<dyn CredentialsProvider>,
}

pub(crate) struct EditPredictionRejectionPayload {
    rejection: EditPredictionRejection,
    organization_id: Option<OrganizationId>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EditPredictionModel {
    Zeta,
    Fim { format: EditPredictionPromptFormat },
    Mercury,
}

#[derive(Clone)]
pub struct EditPredictionModelInput {
    project: Entity<Project>,
    buffer: Entity<Buffer>,
    snapshot: BufferSnapshot,
    position: Anchor,
    events: Vec<Arc<zeta_prompt::Event>>,
    related_files: Vec<RelatedFile>,
    mode: PredictEditsMode,
    trigger: PredictEditsRequestTrigger,
    diagnostic_search_range: Range<Point>,
    debug_tx: Option<mpsc::UnboundedSender<DebugEvent>>,
    can_collect_data: bool,
    is_open_source: bool,
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

/// An event with associated metadata for reconstructing buffer state.
#[derive(Clone)]
pub struct StoredEvent {
    pub event: Arc<zeta_prompt::Event>,
    pub old_snapshot: TextBufferSnapshot,
    pub new_snapshot_version: clock::Global,
    pub total_edit_range: Range<Anchor>,
}

impl StoredEvent {
    fn can_merge(
        &self,
        next_old_event: &StoredEvent,
        latest_snapshot: &TextBufferSnapshot,
        latest_edit_range: &Range<Anchor>,
    ) -> bool {
        // Events must be for the same buffer and be contiguous across included snapshots to be mergeable.
        if self.old_snapshot.remote_id() != next_old_event.old_snapshot.remote_id() {
            return false;
        }
        if self.old_snapshot.remote_id() != latest_snapshot.remote_id() {
            return false;
        }
        if self.new_snapshot_version != next_old_event.old_snapshot.version {
            return false;
        }
        if !latest_snapshot
            .version
            .observed_all(&next_old_event.new_snapshot_version)
        {
            return false;
        }

        let a_is_predicted = matches!(
            self.event.as_ref(),
            zeta_prompt::Event::BufferChange {
                predicted: true,
                ..
            }
        );
        let b_is_predicted = matches!(
            next_old_event.event.as_ref(),
            zeta_prompt::Event::BufferChange {
                predicted: true,
                ..
            }
        );

        // If events come from the same source (both predicted or both manual) then
        // we would have coalesced them already.
        if a_is_predicted == b_is_predicted {
            return false;
        }

        let left_range = self.total_edit_range.to_point(latest_snapshot);
        let right_range = next_old_event.total_edit_range.to_point(latest_snapshot);
        let latest_range = latest_edit_range.to_point(latest_snapshot);

        // Events near to the latest edit are not merged if their sources differ.
        if lines_between_ranges(&left_range, &latest_range)
            .min(lines_between_ranges(&right_range, &latest_range))
            <= CHANGE_GROUPING_LINE_SPAN
        {
            return false;
        }

        // Events that are distant from each other are not merged.
        if lines_between_ranges(&left_range, &right_range) > CHANGE_GROUPING_LINE_SPAN {
            return false;
        }

        true
    }
}

fn lines_between_ranges(left: &Range<Point>, right: &Range<Point>) -> u32 {
    if left.start > right.end {
        return left.start.row - right.end.row;
    }
    if right.start > left.end {
        return right.start.row - left.end.row;
    }
    0
}

struct ProjectState {
    events: VecDeque<StoredEvent>,
    last_event: Option<LastEvent>,
    recent_paths: VecDeque<ProjectPath>,
    registered_buffers: HashMap<gpui::EntityId, RegisteredBuffer>,
    current_prediction: Option<CurrentEditPrediction>,
    next_pending_prediction_id: usize,
    pending_predictions: ArrayVec<PendingPrediction, 2, u8>,
    debug_tx: Option<mpsc::UnboundedSender<DebugEvent>>,
    last_edit_prediction_refresh: Option<(EntityId, Instant)>,
    last_jump_prediction_refresh: Option<(EntityId, Instant)>,
    cancelled_predictions: HashSet<usize>,
    context: Entity<RelatedExcerptStore>,
    license_detection_watchers: HashMap<WorktreeId, Rc<LicenseDetectionWatcher>>,
    _subscriptions: [gpui::Subscription; 2],
    copilot: Option<Entity<Copilot>>,
}

impl ProjectState {
    pub fn events(&self, cx: &App) -> Vec<StoredEvent> {
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

        if pending_prediction.drop_on_cancel {
            drop(pending_prediction.task);
        } else {
            cx.spawn(async move |this, cx| {
                let Some((prediction_id, model_version)) = pending_prediction.task.await else {
                    return;
                };

                this.update(cx, |this, cx| {
                    this.reject_prediction(
                        prediction_id,
                        EditPredictionRejectReason::Canceled,
                        false,
                        model_version,
                        None,
                        cx,
                    );
                })
                .ok();
            })
            .detach()
        }
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
    pub e2e_latency: std::time::Duration,
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

const DIAGNOSTIC_LINES_RANGE: u32 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticSearchScope {
    Local,
    Global,
}

#[derive(Debug)]
struct PendingPrediction {
    id: usize,
    task: Task<Option<(EditPredictionId, Option<String>)>>,
    /// If true, the task is dropped immediately on cancel (cancelling the HTTP request).
    /// If false, the task is awaited to completion so rejection can be reported.
    drop_on_cancel: bool,
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

#[derive(Clone)]
struct PendingSettledPrediction {
    request_id: EditPredictionId,
    editable_anchor_range: Range<Anchor>,
    editable_region_before_prediction: String,
    predicted_editable_region: String,
    ts_error_count_before_prediction: usize,
    ts_error_count_after_prediction: usize,
    example: Option<ExampleSpec>,
    enqueued_at: Instant,
    last_edit_at: Instant,
    e2e_latency: std::time::Duration,
}

struct RegisteredBuffer {
    file: Option<Arc<dyn File>>,
    snapshot: TextBufferSnapshot,
    pending_predictions: Vec<PendingSettledPrediction>,
    last_position: Option<Anchor>,
    _subscriptions: [gpui::Subscription; 2],
}

#[derive(Clone)]
struct LastEvent {
    old_snapshot: TextBufferSnapshot,
    new_snapshot: TextBufferSnapshot,
    old_file: Option<Arc<dyn File>>,
    new_file: Option<Arc<dyn File>>,
    latest_edit_range: Range<Anchor>,
    total_edit_range: Range<Anchor>,
    total_edit_range_at_last_pause_boundary: Option<Range<Anchor>>,
    predicted: bool,
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

        let (diff, edit_range) = compute_diff_between_snapshots_in_range(
            &self.old_snapshot,
            &self.new_snapshot,
            &self.total_edit_range,
        )?;

        if path == old_path && diff.is_empty() {
            None
        } else {
            Some(StoredEvent {
                event: Arc::new(zeta_prompt::Event::BufferChange {
                    old_path,
                    path,
                    diff,
                    in_open_source_repo,
                    predicted: self.predicted,
                }),
                old_snapshot: self.old_snapshot.clone(),
                new_snapshot_version: self.new_snapshot.version.clone(),
                total_edit_range: self.new_snapshot.anchor_before(edit_range.start)
                    ..self.new_snapshot.anchor_before(edit_range.end),
            })
        }
    }

    pub fn split_by_pause(&self) -> (LastEvent, Option<LastEvent>) {
        let Some(boundary_snapshot) = self.snapshot_after_last_editing_pause.as_ref() else {
            return (self.clone(), None);
        };

        let total_edit_range_before_pause = self
            .total_edit_range_at_last_pause_boundary
            .clone()
            .unwrap_or_else(|| self.total_edit_range.clone());

        let Some(total_edit_range_after_pause) =
            compute_total_edit_range_between_snapshots(boundary_snapshot, &self.new_snapshot)
        else {
            return (self.clone(), None);
        };

        let latest_edit_range_before_pause = total_edit_range_before_pause.clone();
        let latest_edit_range_after_pause = total_edit_range_after_pause.clone();

        let before = LastEvent {
            old_snapshot: self.old_snapshot.clone(),
            new_snapshot: boundary_snapshot.clone(),
            old_file: self.old_file.clone(),
            new_file: self.new_file.clone(),
            latest_edit_range: latest_edit_range_before_pause,
            total_edit_range: total_edit_range_before_pause,
            total_edit_range_at_last_pause_boundary: None,
            predicted: self.predicted,
            snapshot_after_last_editing_pause: None,
            last_edit_time: self.last_edit_time,
        };

        let after = LastEvent {
            old_snapshot: boundary_snapshot.clone(),
            new_snapshot: self.new_snapshot.clone(),
            old_file: self.old_file.clone(),
            new_file: self.new_file.clone(),
            latest_edit_range: latest_edit_range_after_pause,
            total_edit_range: total_edit_range_after_pause,
            total_edit_range_at_last_pause_boundary: None,
            predicted: self.predicted,
            snapshot_after_last_editing_pause: None,
            last_edit_time: self.last_edit_time,
        };

        (before, Some(after))
    }
}

fn compute_total_edit_range_between_snapshots(
    old_snapshot: &TextBufferSnapshot,
    new_snapshot: &TextBufferSnapshot,
) -> Option<Range<Anchor>> {
    let edits: Vec<Edit<usize>> = new_snapshot
        .edits_since::<usize>(&old_snapshot.version)
        .collect();

    let (first_edit, last_edit) = edits.first().zip(edits.last())?;
    let new_start_point = new_snapshot.offset_to_point(first_edit.new.start);
    let new_end_point = new_snapshot.offset_to_point(last_edit.new.end);

    Some(new_snapshot.anchor_before(new_start_point)..new_snapshot.anchor_before(new_end_point))
}

fn compute_old_range_for_new_range(
    old_snapshot: &TextBufferSnapshot,
    new_snapshot: &TextBufferSnapshot,
    total_edit_range: &Range<Anchor>,
) -> Option<Range<Point>> {
    let new_start_offset = total_edit_range.start.to_offset(new_snapshot);
    let new_end_offset = total_edit_range.end.to_offset(new_snapshot);

    let edits: Vec<Edit<usize>> = new_snapshot
        .edits_since::<usize>(&old_snapshot.version)
        .collect();
    let mut old_start_offset = None;
    let mut old_end_offset = None;
    let mut delta: isize = 0;

    for edit in &edits {
        if old_start_offset.is_none() && new_start_offset <= edit.new.end {
            old_start_offset = Some(if new_start_offset < edit.new.start {
                new_start_offset.checked_add_signed(-delta)?
            } else {
                edit.old.start
            });
        }

        if old_end_offset.is_none() && new_end_offset <= edit.new.end {
            old_end_offset = Some(if new_end_offset < edit.new.start {
                new_end_offset.checked_add_signed(-delta)?
            } else {
                edit.old.end
            });
        }

        delta += edit.new.len() as isize - edit.old.len() as isize;
    }

    let old_start_offset =
        old_start_offset.unwrap_or_else(|| new_start_offset.saturating_add_signed(-delta));
    let old_end_offset =
        old_end_offset.unwrap_or_else(|| new_end_offset.saturating_add_signed(-delta));

    Some(
        old_snapshot.offset_to_point(old_start_offset)
            ..old_snapshot.offset_to_point(old_end_offset),
    )
}

fn compute_diff_between_snapshots_in_range(
    old_snapshot: &TextBufferSnapshot,
    new_snapshot: &TextBufferSnapshot,
    total_edit_range: &Range<Anchor>,
) -> Option<(String, Range<Point>)> {
    let new_start_point = total_edit_range.start.to_point(new_snapshot);
    let new_end_point = total_edit_range.end.to_point(new_snapshot);
    let old_range = compute_old_range_for_new_range(old_snapshot, new_snapshot, total_edit_range)?;
    let old_start_point = old_range.start;
    let old_end_point = old_range.end;

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

    if new_edit_range.len() > EDIT_HISTORY_DIFF_SIZE_LIMIT
        || old_edit_range.len() > EDIT_HISTORY_DIFF_SIZE_LIMIT
    {
        return None;
    }

    let old_region_text: String = old_snapshot.text_for_range(old_edit_range).collect();
    let new_region_text: String = new_snapshot.text_for_range(new_edit_range).collect();

    let diff = language::unified_diff_with_offsets(
        &old_region_text,
        &new_region_text,
        old_context_start_row,
        new_context_start_row,
    );

    Some((diff, new_start_point..new_end_point))
}

pub(crate) fn buffer_path_with_id_fallback(
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
        let llm_token = global_llm_token(cx);
        let legacy_data_collection_enabled = Self::load_legacy_data_collection_enabled(cx);

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

        let (settled_predictions_tx, settled_predictions_rx) = mpsc::unbounded();
        cx.spawn(async move |this, cx| {
            Self::run_settled_predictions_worker(this, settled_predictions_rx, cx).await;
        })
        .detach();

        let mut current_user = user_store.read(cx).watch_current_user();
        let fetch_experiments_task = cx.spawn(async move |this, cx| {
            while current_user.borrow().is_none() {
                current_user.next().await;
            }

            this.update(cx, |this, cx| {
                if cx.is_staff() {
                    this.refresh_available_experiments(cx);
                }
            })
            .log_err();
        });

        let credentials_provider = zed_credentials_provider::global(cx);

        let this = Self {
            projects: HashMap::default(),
            client,
            user_store,
            llm_token,
            _fetch_experiments_task: fetch_experiments_task,
            update_required: false,
            edit_prediction_model: EditPredictionModel::Zeta,
            zeta2_raw_config: Self::zeta2_raw_config_from_env(),
            preferred_experiment: None,
            available_experiments: Vec::new(),
            mercury: Mercury::new(cx),
            legacy_data_collection_enabled,

            reject_predictions_tx: reject_tx,
            settled_predictions_tx,
            rated_predictions: Default::default(),
            shown_predictions: Default::default(),
            #[cfg(test)]
            settled_event_callback: None,

            credentials_provider,
        };

        this
    }

    fn zeta2_raw_config_from_env() -> Option<Zeta2RawConfig> {
        let version_str = env::var("ZED_ZETA_FORMAT").ok()?;
        let format = ZetaFormat::parse(&version_str).ok()?;
        let model_id = env::var("ZED_ZETA_MODEL").ok();
        let environment = env::var("ZED_ZETA_ENVIRONMENT").ok();
        Some(Zeta2RawConfig {
            model_id,
            environment,
            format,
        })
    }

    pub fn set_edit_prediction_model(&mut self, model: EditPredictionModel) {
        self.edit_prediction_model = model;
    }

    pub fn set_zeta2_raw_config(&mut self, config: Zeta2RawConfig) {
        self.zeta2_raw_config = Some(config);
    }

    pub fn zeta2_raw_config(&self) -> Option<&Zeta2RawConfig> {
        self.zeta2_raw_config.as_ref()
    }

    pub fn preferred_experiment(&self) -> Option<&str> {
        self.preferred_experiment.as_deref()
    }

    pub fn set_preferred_experiment(&mut self, experiment: Option<String>) {
        self.preferred_experiment = experiment;
    }

    pub fn available_experiments(&self) -> &[String] {
        &self.available_experiments
    }

    pub fn active_experiment(&self) -> Option<&str> {
        self.preferred_experiment.as_deref().or_else(|| {
            self.shown_predictions
                .iter()
                .find_map(|p| p.model_version.as_ref())
                .and_then(|model_version| model_version.strip_prefix("zeta2:"))
        })
    }

    pub fn refresh_available_experiments(&mut self, cx: &mut Context<Self>) {
        let client = self.client.clone();
        let llm_token = self.llm_token.clone();
        let app_version = AppVersion::global(cx);
        let organization_id = self
            .user_store
            .read(cx)
            .current_organization()
            .map(|organization| organization.id.clone());

        cx.spawn(async move |this, cx| {
            let experiments = cx
                .background_spawn(async move {
                    let http_client = client.http_client();
                    let token = client
                        .acquire_llm_token(&llm_token, organization_id.clone())
                        .await?;
                    let url = http_client.build_zed_llm_url("/edit_prediction_experiments", &[])?;
                    let request = http_client::Request::builder()
                        .method(Method::GET)
                        .uri(url.as_ref())
                        .header("Authorization", format!("Bearer {}", token))
                        .header(ZED_VERSION_HEADER_NAME, app_version.to_string())
                        .body(Default::default())?;
                    let mut response = http_client.send(request).await?;
                    if response.status().is_success() {
                        let mut body = Vec::new();
                        response.body_mut().read_to_end(&mut body).await?;
                        let experiments: Vec<String> = serde_json::from_slice(&body)?;
                        Ok(experiments)
                    } else {
                        let mut body = String::new();
                        response.body_mut().read_to_string(&mut body).await?;
                        anyhow::bail!(
                            "Failed to fetch experiments: {:?}\nBody: {}",
                            response.status(),
                            body
                        );
                    }
                })
                .await?;
            this.update(cx, |this, cx| {
                this.available_experiments = experiments;
                cx.notify();
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    pub fn icons(&self, cx: &App) -> edit_prediction_types::EditPredictionIconSet {
        use ui::IconName;
        match self.edit_prediction_model {
            EditPredictionModel::Mercury => {
                edit_prediction_types::EditPredictionIconSet::new(IconName::Inception)
            }
            EditPredictionModel::Zeta => {
                edit_prediction_types::EditPredictionIconSet::new(IconName::ZedPredict)
                    .with_disabled(IconName::ZedPredictDisabled)
                    .with_up(IconName::ZedPredictUp)
                    .with_down(IconName::ZedPredictDown)
                    .with_error(IconName::ZedPredictError)
            }
            EditPredictionModel::Fim { .. } => {
                let settings = &all_language_settings(None, cx).edit_predictions;
                match settings.provider {
                    EditPredictionProvider::Ollama => {
                        edit_prediction_types::EditPredictionIconSet::new(IconName::AiOllama)
                    }
                    _ => {
                        edit_prediction_types::EditPredictionIconSet::new(IconName::AiOpenAiCompat)
                    }
                }
            }
        }
    }

    pub fn has_mercury_api_token(&self, cx: &App) -> bool {
        self.mercury.api_token.read(cx).has_key()
    }

    pub fn mercury_has_payment_required_error(&self) -> bool {
        self.mercury.has_payment_required_error()
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

    pub fn context_for_project<'a>(
        &'a self,
        project: &Entity<Project>,
        cx: &'a mut App,
    ) -> Vec<RelatedFile> {
        self.projects
            .get(&project.entity_id())
            .map(|project_state| {
                project_state.context.update(cx, |context, cx| {
                    context
                        .related_files_with_buffers(cx)
                        .map(|(mut related_file, buffer)| {
                            related_file.in_open_source_repo = buffer
                                .read(cx)
                                .file()
                                .map_or(false, |file| self.is_file_open_source(&project, file, cx));
                            related_file
                        })
                        .collect()
                })
            })
            .unwrap_or_default()
    }

    pub fn copilot_for_project(&self, project: &Entity<Project>) -> Option<Entity<Copilot>> {
        self.projects
            .get(&project.entity_id())
            .and_then(|project| project.copilot.clone())
    }

    pub fn start_copilot_for_project(
        &mut self,
        project: &Entity<Project>,
        cx: &mut Context<Self>,
    ) -> Option<Entity<Copilot>> {
        if DisableAiSettings::get(None, cx).disable_ai {
            return None;
        }
        let state = self.get_or_init_project(project, cx);

        if state.copilot.is_some() {
            return state.copilot.clone();
        }
        let _project = project.clone();
        let project = project.read(cx);

        let node = project.node_runtime().cloned();
        if let Some(node) = node {
            let next_id = project.languages().next_language_server_id();
            let fs = project.fs().clone();

            let copilot = cx.new(|cx| Copilot::new(Some(_project), next_id, fs, node, cx));
            state.copilot = Some(copilot.clone());
            Some(copilot)
        } else {
            None
        }
    }

    pub fn context_for_project_with_buffers<'a>(
        &'a self,
        project: &Entity<Project>,
        cx: &'a mut App,
    ) -> Vec<(RelatedFile, Entity<Buffer>)> {
        self.projects
            .get(&project.entity_id())
            .map(|project| {
                project.context.update(cx, |context, cx| {
                    context.related_files_with_buffers(cx).collect()
                })
            })
            .unwrap_or_default()
    }

    pub fn usage(&self, cx: &App) -> Option<EditPredictionUsage> {
        if matches!(self.edit_prediction_model, EditPredictionModel::Zeta) {
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
                last_edit_prediction_refresh: None,
                last_jump_prediction_refresh: None,
                license_detection_watchers: HashMap::default(),
                _subscriptions: [
                    cx.subscribe(&project, Self::handle_project_event),
                    cx.observe_release(&project, move |this, _, cx| {
                        this.projects.remove(&entity_id);
                        cx.notify();
                    }),
                ],
                copilot: None,
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
        if !is_ep_store_provider(all_language_settings(None, cx).edit_predictions.provider) {
            return;
        }
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
                if cx.has_flag::<EditPredictionJumpsFeatureFlag>() {
                    self.refresh_prediction_from_diagnostics(
                        project,
                        DiagnosticSearchScope::Global,
                        cx,
                    );
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
                    pending_predictions: Vec::new(),
                    _subscriptions: [
                        cx.subscribe(buffer, {
                            let project = project.downgrade();
                            move |this, buffer, event, cx| {
                                if let language::BufferEvent::Edited { is_local } = event
                                    && let Some(project) = project.upgrade()
                                {
                                    this.report_changes_for_buffer(
                                        &buffer, &project, false, *is_local, cx,
                                    );
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
        is_predicted: bool,
        is_local: bool,
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
        let mut edit_range: Option<Range<Anchor>> = None;
        let now = cx.background_executor().now();

        for (_edit, anchor_range) in
            new_snapshot.anchored_edits_since::<usize>(&old_snapshot.version)
        {
            edit_range = Some(match edit_range {
                None => anchor_range,
                Some(acc) => acc.start..anchor_range.end,
            });
        }

        let Some(edit_range) = edit_range else {
            return;
        };

        for pending_prediction in &mut registered_buffer.pending_predictions {
            if edit_range.overlaps(&pending_prediction.editable_anchor_range, &new_snapshot) {
                pending_prediction.last_edit_at = now;
            }
        }

        let include_in_history = is_local
            || collaborator_edit_overlaps_locality_region(
                project_state,
                project,
                buffer,
                &buf.snapshot(),
                &edit_range,
                cx,
            );

        if !include_in_history {
            return;
        }

        let is_recordable_history_edit =
            compute_diff_between_snapshots_in_range(&old_snapshot, &new_snapshot, &edit_range)
                .is_some();

        let events = &mut project_state.events;

        if !is_recordable_history_edit {
            if let Some(event) = project_state.last_event.take() {
                if let Some(event) = event.finalize(&project_state.license_detection_watchers, cx) {
                    if events.len() + 1 >= EVENT_COUNT_MAX {
                        events.pop_front();
                    }
                    events.push_back(event);
                }
            }
            return;
        }

        if let Some(last_event) = project_state.last_event.as_mut() {
            let is_next_snapshot_of_same_buffer = old_snapshot.remote_id()
                == last_event.new_snapshot.remote_id()
                && old_snapshot.version == last_event.new_snapshot.version;

            let prediction_source_changed = is_predicted != last_event.predicted;

            let should_coalesce = is_next_snapshot_of_same_buffer
                && !prediction_source_changed
                && lines_between_ranges(
                    &edit_range.to_point(&new_snapshot),
                    &last_event.latest_edit_range.to_point(&new_snapshot),
                ) <= CHANGE_GROUPING_LINE_SPAN;

            if should_coalesce {
                let pause_elapsed = last_event
                    .last_edit_time
                    .map(|t| now.duration_since(t) >= LAST_CHANGE_GROUPING_TIME)
                    .unwrap_or(false);
                if pause_elapsed {
                    last_event.snapshot_after_last_editing_pause =
                        Some(last_event.new_snapshot.clone());
                    last_event.total_edit_range_at_last_pause_boundary =
                        Some(last_event.total_edit_range.clone());
                }

                last_event.latest_edit_range = edit_range.clone();
                last_event.total_edit_range =
                    merge_anchor_ranges(&last_event.total_edit_range, &edit_range, &new_snapshot);
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

        merge_trailing_events_if_needed(events, &old_snapshot, &new_snapshot, &edit_range);

        project_state.last_event = Some(LastEvent {
            old_file,
            new_file,
            old_snapshot,
            new_snapshot,
            latest_edit_range: edit_range.clone(),
            total_edit_range: edit_range,
            total_edit_range_at_last_pause_boundary: None,
            predicted: is_predicted,
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
        let Some(current_prediction) = self
            .projects
            .get_mut(&project.entity_id())
            .and_then(|project_state| project_state.current_prediction.take())
        else {
            return;
        };

        self.report_changes_for_buffer(
            &current_prediction.prediction.buffer,
            project,
            true,
            true,
            cx,
        );

        // can't hold &mut project_state ref across report_changes_for_buffer_call
        let Some(project_state) = self.projects.get_mut(&project.entity_id()) else {
            return;
        };

        for pending_prediction in mem::take(&mut project_state.pending_predictions) {
            project_state.cancel_pending_prediction(pending_prediction, cx);
        }

        match self.edit_prediction_model {
            EditPredictionModel::Mercury => {
                mercury::edit_prediction_accepted(
                    current_prediction.prediction.id,
                    self.client.http_client(),
                    cx,
                );
            }
            EditPredictionModel::Zeta => {
                let is_cloud = !matches!(
                    all_language_settings(None, cx).edit_predictions.provider,
                    EditPredictionProvider::Ollama | EditPredictionProvider::OpenAiCompatibleApi
                );
                if is_cloud {
                    zeta::edit_prediction_accepted(self, current_prediction, cx)
                }
            }
            EditPredictionModel::Fim { .. } => {}
        }
    }

    async fn handle_rejected_predictions(
        rx: UnboundedReceiver<EditPredictionRejectionPayload>,
        client: Arc<Client>,
        llm_token: LlmApiToken,
        app_version: Version,
        background_executor: BackgroundExecutor,
    ) {
        let mut rx = std::pin::pin!(rx.peekable());
        let mut batched = Vec::new();

        while let Some(EditPredictionRejectionPayload {
            rejection,
            organization_id,
        }) = rx.next().await
        {
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
                organization_id,
                app_version.clone(),
                true,
            )
            .await;

            if result.log_err().is_some() {
                batched.drain(start..);
            }
        }
    }

    async fn run_settled_predictions_worker(
        this: WeakEntity<Self>,
        mut rx: UnboundedReceiver<Instant>,
        cx: &mut AsyncApp,
    ) {
        let mut next_wake_time: Option<Instant> = None;
        loop {
            let now = cx.background_executor().now();
            if let Some(wake_time) = next_wake_time.take() {
                cx.background_executor()
                    .timer(wake_time.duration_since(now))
                    .await;
            } else {
                let Some(new_enqueue_time) = rx.next().await else {
                    break;
                };
                next_wake_time = Some(new_enqueue_time + EDIT_PREDICTION_SETTLED_QUIESCENCE);
                while rx.next().now_or_never().flatten().is_some() {}
                continue;
            }

            let Some(this) = this.upgrade() else {
                break;
            };

            let now = cx.background_executor().now();
            let mut oldest_edited_at = None;
            let mut ready_predictions = Vec::new();

            this.update(cx, |this, _| {
                for (_, project_state) in this.projects.iter_mut() {
                    for (_, registered_buffer) in project_state.registered_buffers.iter_mut() {
                        let mut pending_index = 0;
                        while pending_index < registered_buffer.pending_predictions.len() {
                            let pending_prediction =
                                &registered_buffer.pending_predictions[pending_index];
                            let age = now.saturating_duration_since(pending_prediction.enqueued_at);
                            if age >= EDIT_PREDICTION_SETTLED_TTL {
                                registered_buffer.pending_predictions.remove(pending_index);
                                continue;
                            }

                            let quiet_for =
                                now.saturating_duration_since(pending_prediction.last_edit_at);
                            if quiet_for >= EDIT_PREDICTION_SETTLED_QUIESCENCE {
                                let pending_prediction =
                                    registered_buffer.pending_predictions.remove(pending_index);
                                let settled_editable_region = registered_buffer
                                    .snapshot
                                    .text_for_range(
                                        pending_prediction.editable_anchor_range.clone(),
                                    )
                                    .collect::<String>();
                                ready_predictions
                                    .push((pending_prediction, settled_editable_region));
                                continue;
                            }

                            if oldest_edited_at
                                .is_none_or(|time| pending_prediction.last_edit_at < time)
                            {
                                oldest_edited_at = Some(pending_prediction.last_edit_at);
                            }
                            pending_index += 1;
                        }
                    }
                }
            });

            for (pending_prediction, settled_editable_region) in ready_predictions {
                let PendingSettledPrediction {
                    request_id,
                    editable_region_before_prediction,
                    predicted_editable_region,
                    ts_error_count_before_prediction,
                    ts_error_count_after_prediction,
                    example,
                    e2e_latency,
                    ..
                } = pending_prediction;
                let settled_editable_region_for_metrics = settled_editable_region.clone();
                let kept_rate_result = cx
                    .background_spawn(async move {
                        compute_kept_rate(
                            &editable_region_before_prediction,
                            &predicted_editable_region,
                            &settled_editable_region_for_metrics,
                        )
                    })
                    .await;

                #[cfg(test)]
                {
                    let request_id = request_id.clone();
                    let settled_editable_region = settled_editable_region.clone();
                    this.update(cx, |this, _| {
                        if let Some(callback) = &this.settled_event_callback {
                            callback(request_id, settled_editable_region);
                        }
                    });
                }

                telemetry::event!(
                    EDIT_PREDICTION_SETTLED_EVENT,
                    request_id = request_id.0.clone(),
                    settled_editable_region,
                    ts_error_count_before_prediction,
                    ts_error_count_after_prediction,
                    edit_bytes_candidate_new = kept_rate_result.candidate_new_chars,
                    edit_bytes_reference_new = kept_rate_result.reference_new_chars,
                    edit_bytes_candidate_deleted = kept_rate_result.candidate_deleted_chars,
                    edit_bytes_reference_deleted = kept_rate_result.reference_deleted_chars,
                    edit_bytes_kept = kept_rate_result.kept_chars,
                    edit_bytes_correctly_deleted = kept_rate_result.correctly_deleted_chars,
                    edit_bytes_discarded = kept_rate_result.discarded_chars,
                    edit_bytes_context = kept_rate_result.context_chars,
                    edit_bytes_kept_rate = kept_rate_result.kept_rate,
                    edit_bytes_recall_rate = kept_rate_result.recall_rate,
                    example,
                    e2e_latency = e2e_latency.as_millis(),
                );
            }

            next_wake_time = oldest_edited_at.map(|time| time + EDIT_PREDICTION_SETTLED_QUIESCENCE);
        }
    }

    pub(crate) fn enqueue_settled_prediction(
        &mut self,
        request_id: EditPredictionId,
        project: &Entity<Project>,
        edited_buffer: &Entity<Buffer>,
        edited_buffer_snapshot: &BufferSnapshot,
        editable_offset_range: Range<usize>,
        edit_preview: &EditPreview,
        example: Option<ExampleSpec>,
        e2e_latency: std::time::Duration,
        cx: &mut Context<Self>,
    ) {
        let this = &mut *self;
        let project_state = this.get_or_init_project(project, cx);
        let Some(registered_buffer) = project_state
            .registered_buffers
            .get_mut(&edited_buffer.entity_id())
        else {
            return;
        };

        let editable_region_before_prediction = edited_buffer_snapshot
            .text_for_range(editable_offset_range.clone())
            .collect::<String>();
        let editable_anchor_range_for_result =
            edited_buffer_snapshot.anchor_range_inside(editable_offset_range.clone());
        let predicted_editable_region = edit_preview
            .result_text_snapshot()
            .text_for_range(editable_anchor_range_for_result.clone())
            .collect();
        let ts_error_count_before_prediction = crate::metrics::count_tree_sitter_errors(
            edited_buffer_snapshot
                .syntax_layers_for_range(editable_anchor_range_for_result.clone(), true),
        );
        let ts_error_count_after_prediction = crate::metrics::count_tree_sitter_errors(
            edit_preview.result_syntax_snapshot().layers_for_range(
                editable_anchor_range_for_result,
                edit_preview.result_text_snapshot(),
                true,
            ),
        );
        let editable_anchor_range =
            edited_buffer_snapshot.anchor_range_inside(editable_offset_range);
        let now = cx.background_executor().now();
        registered_buffer
            .pending_predictions
            .push(PendingSettledPrediction {
                request_id,
                editable_anchor_range,
                editable_region_before_prediction,
                predicted_editable_region,
                ts_error_count_before_prediction,
                ts_error_count_after_prediction,
                example,
                e2e_latency,
                enqueued_at: now,
                last_edit_at: now,
            });
        this.settled_predictions_tx.unbounded_send(now).ok();
    }

    fn reject_current_prediction(
        &mut self,
        reason: EditPredictionRejectReason,
        project: &Entity<Project>,
        cx: &App,
    ) {
        if let Some(project_state) = self.projects.get_mut(&project.entity_id()) {
            project_state.pending_predictions.clear();
            if let Some(prediction) = project_state.current_prediction.take() {
                let model_version = prediction.prediction.model_version.clone();
                self.reject_prediction(
                    prediction.prediction.id,
                    reason,
                    prediction.was_shown,
                    model_version,
                    Some(prediction.e2e_latency),
                    cx,
                );
            }
        };
    }

    fn did_show_current_prediction(
        &mut self,
        project: &Entity<Project>,
        display_type: edit_prediction_types::SuggestionDisplayType,
        _cx: &mut Context<Self>,
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
        model_version: Option<String>,
        e2e_latency: Option<std::time::Duration>,
        cx: &App,
    ) {
        match self.edit_prediction_model {
            EditPredictionModel::Zeta => {
                let is_cloud = !matches!(
                    all_language_settings(None, cx).edit_predictions.provider,
                    EditPredictionProvider::Ollama | EditPredictionProvider::OpenAiCompatibleApi
                );

                if is_cloud {
                    let organization_id = self
                        .user_store
                        .read(cx)
                        .current_organization()
                        .map(|organization| organization.id.clone());

                    self.reject_predictions_tx
                        .unbounded_send(EditPredictionRejectionPayload {
                            rejection: EditPredictionRejection {
                                request_id: prediction_id.to_string(),
                                reason,
                                was_shown,
                                model_version,
                                e2e_latency_ms: e2e_latency.map(|latency| latency.as_millis()),
                            },
                            organization_id,
                        })
                        .log_err();
                }
            }
            EditPredictionModel::Mercury => {
                mercury::edit_prediction_rejected(
                    prediction_id,
                    was_shown,
                    reason,
                    self.client.http_client(),
                    cx,
                );
            }
            EditPredictionModel::Fim { .. } => {}
        }
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
        self.queue_prediction_refresh(
            project.clone(),
            PredictEditsRequestTrigger::Other,
            buffer.entity_id(),
            cx,
            move |this, cx| {
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
            },
        )
    }

    pub fn refresh_prediction_from_diagnostics(
        &mut self,
        project: Entity<Project>,
        scope: DiagnosticSearchScope,
        cx: &mut Context<Self>,
    ) {
        if !is_ep_store_provider(all_language_settings(None, cx).edit_predictions.provider) {
            return;
        }

        if currently_following(&project, cx) {
            return;
        }

        let Some(project_state) = self.projects.get_mut(&project.entity_id()) else {
            return;
        };

        // Prefer predictions from buffer
        if project_state.current_prediction.is_some() {
            log::debug!(
                "edit_prediction: diagnostic refresh skipped, current prediction already exists"
            );
            return;
        }

        self.queue_prediction_refresh(
            project.clone(),
            PredictEditsRequestTrigger::Diagnostics,
            project.entity_id(),
            cx,
            move |this, cx| {
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
                    let diagnostic_search_range = match scope {
                        DiagnosticSearchScope::Local => {
                            let diagnostic_search_start =
                                cursor_point.row.saturating_sub(DIAGNOSTIC_LINES_RANGE);
                            let diagnostic_search_end = cursor_point.row + DIAGNOSTIC_LINES_RANGE;
                            Point::new(diagnostic_search_start, 0)
                                ..Point::new(diagnostic_search_end, 0)
                        }
                        DiagnosticSearchScope::Global => Default::default(),
                    };

                    let Some((jump_buffer, jump_position)) = Self::next_diagnostic_location(
                        active_buffer,
                        &snapshot,
                        diagnostic_search_range,
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
                                    model_version: prediction_result.model_version,
                                    e2e_latency: prediction_result.e2e_latency,
                                }
                            },
                            PredictionRequestedBy::DiagnosticsUpdate,
                        ))
                    })
                })
            },
        );
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

    pub const THROTTLE_TIMEOUT: Duration = Duration::from_millis(300);
}

fn currently_following(project: &Entity<Project>, cx: &App) -> bool {
    let Some(app_state) = AppState::try_global(cx) else {
        return false;
    };

    app_state
        .workspace_store
        .read(cx)
        .workspaces()
        .filter_map(|workspace| workspace.upgrade())
        .any(|workspace| {
            workspace.read(cx).project().entity_id() == project.entity_id()
                && workspace
                    .read(cx)
                    .leader_for_pane(workspace.read(cx).active_pane())
                    .is_some()
        })
}

fn is_ep_store_provider(provider: EditPredictionProvider) -> bool {
    match provider {
        EditPredictionProvider::Zed
        | EditPredictionProvider::Mercury
        | EditPredictionProvider::Ollama
        | EditPredictionProvider::OpenAiCompatibleApi => true,
        EditPredictionProvider::None
        | EditPredictionProvider::Copilot
        | EditPredictionProvider::Codestral => false,
    }
}

impl EditPredictionStore {
    fn queue_prediction_refresh(
        &mut self,
        project: Entity<Project>,
        request_trigger: PredictEditsRequestTrigger,
        throttle_entity: EntityId,
        cx: &mut Context<Self>,
        do_refresh: impl FnOnce(
            WeakEntity<Self>,
            &mut AsyncApp,
        )
            -> Task<Result<Option<(EditPredictionResult, PredictionRequestedBy)>>>
        + 'static,
    ) {
        fn select_throttle(
            project_state: &mut ProjectState,
            request_trigger: PredictEditsRequestTrigger,
        ) -> &mut Option<(EntityId, Instant)> {
            match request_trigger {
                PredictEditsRequestTrigger::Diagnostics => {
                    &mut project_state.last_jump_prediction_refresh
                }
                _ => &mut project_state.last_edit_prediction_refresh,
            }
        }

        let (needs_acceptance_tracking, max_pending_predictions) =
            match all_language_settings(None, cx).edit_predictions.provider {
                EditPredictionProvider::Zed | EditPredictionProvider::Mercury => (true, 2),
                EditPredictionProvider::Ollama => (false, 1),
                EditPredictionProvider::OpenAiCompatibleApi => (false, 2),
                EditPredictionProvider::None
                | EditPredictionProvider::Copilot
                | EditPredictionProvider::Codestral => {
                    log::error!("queue_prediction_refresh called with non-store provider");
                    return;
                }
            };

        let drop_on_cancel = !needs_acceptance_tracking;
        let throttle_timeout = Self::THROTTLE_TIMEOUT;
        let project_state = self.get_or_init_project(&project, cx);
        let pending_prediction_id = project_state.next_pending_prediction_id;
        project_state.next_pending_prediction_id += 1;
        let throttle_at_enqueue = *select_throttle(project_state, request_trigger);

        let task = cx.spawn(async move |this, cx| {
            let throttle_wait = this
                .update(cx, |this, cx| {
                    let project_state = this.get_or_init_project(&project, cx);
                    let throttle = *select_throttle(project_state, request_trigger);

                    let now = cx.background_executor().now();
                    throttle.and_then(|(last_entity, last_timestamp)| {
                        if throttle_entity != last_entity {
                            return None;
                        }
                        (last_timestamp + throttle_timeout).checked_duration_since(now)
                    })
                })
                .ok()
                .flatten();

            if let Some(timeout) = throttle_wait {
                cx.background_executor().timer(timeout).await;
            }

            // If this task was cancelled before the throttle timeout expired,
            // do not perform a request. Also skip if another task already
            // proceeded since we were enqueued (duplicate).
            let mut is_cancelled = true;
            this.update(cx, |this, cx| {
                let project_state = this.get_or_init_project(&project, cx);
                let was_cancelled = project_state
                    .cancelled_predictions
                    .remove(&pending_prediction_id);
                if was_cancelled {
                    return;
                }

                // Another request has been already sent since this was enqueued
                if *select_throttle(project_state, request_trigger) != throttle_at_enqueue {
                    return;
                }

                let new_refresh = (throttle_entity, cx.background_executor().now());
                *select_throttle(project_state, request_trigger) = Some(new_refresh);
                is_cancelled = false;
            })
            .ok();
            if is_cancelled {
                return None;
            }

            let new_prediction_result = do_refresh(this.clone(), cx).await.log_err().flatten();
            let new_prediction_metadata = new_prediction_result
                .as_ref()
                .map(|(prediction, _)| (prediction.id.clone(), prediction.model_version.clone()));

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
                                e2e_latency: prediction_result.e2e_latency,
                            };

                            if let Some(current_prediction) =
                                project_state.current_prediction.as_ref()
                            {
                                if new_prediction.should_replace_prediction(&current_prediction, cx)
                                {
                                    this.reject_current_prediction(
                                        EditPredictionRejectReason::Replaced,
                                        &project,
                                        cx,
                                    );

                                    Some(new_prediction)
                                } else {
                                    this.reject_prediction(
                                        new_prediction.prediction.id,
                                        EditPredictionRejectReason::CurrentPreferred,
                                        false,
                                        new_prediction.prediction.model_version,
                                        Some(new_prediction.e2e_latency),
                                        cx,
                                    );
                                    None
                                }
                            } else {
                                Some(new_prediction)
                            }
                        }
                        Err(reject_reason) => {
                            this.reject_prediction(
                                prediction_result.id,
                                reject_reason,
                                false,
                                prediction_result.model_version,
                                Some(prediction_result.e2e_latency),
                                cx,
                            );
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

            new_prediction_metadata
        });

        if project_state.pending_predictions.len() < max_pending_predictions {
            project_state
                .pending_predictions
                .push(PendingPrediction {
                    id: pending_prediction_id,
                    task,
                    drop_on_cancel,
                })
                .unwrap();
        } else {
            let pending_prediction = project_state.pending_predictions.pop().unwrap();
            project_state
                .pending_predictions
                .push(PendingPrediction {
                    id: pending_prediction_id,
                    task,
                    drop_on_cancel,
                })
                .unwrap();
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
            cx.has_flag::<EditPredictionJumpsFeatureFlag>(),
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
        self.get_or_init_project(&project, cx);
        let project_state = self.projects.get(&project.entity_id()).unwrap();
        let stored_events = project_state.events(cx);
        let has_events = !stored_events.is_empty();
        let events: Vec<Arc<zeta_prompt::Event>> =
            stored_events.iter().map(|e| e.event.clone()).collect();
        let debug_tx = project_state.debug_tx.clone();

        let snapshot = active_buffer.read(cx).snapshot();
        let cursor_point = position.to_point(&snapshot);
        let diagnostic_search_start = cursor_point.row.saturating_sub(DIAGNOSTIC_LINES_RANGE);
        let diagnostic_search_end = cursor_point.row + DIAGNOSTIC_LINES_RANGE;
        let diagnostic_search_range =
            Point::new(diagnostic_search_start, 0)..Point::new(diagnostic_search_end, 0);

        let related_files = self.context_for_project(&project, cx);
        let mode = match all_language_settings(snapshot.file(), cx).edit_predictions_mode() {
            EditPredictionsMode::Eager => PredictEditsMode::Eager,
            EditPredictionsMode::Subtle => PredictEditsMode::Subtle,
        };

        let is_open_source = snapshot
            .file()
            .map_or(false, |file| self.is_file_open_source(&project, file, cx))
            && events.iter().all(|event| event.in_open_source_repo())
            && related_files.iter().all(|file| file.in_open_source_repo);

        let can_collect_data = !cfg!(test)
            && is_open_source
            && self.is_data_collection_enabled(cx)
            && matches!(self.edit_prediction_model, EditPredictionModel::Zeta);
        let inputs = EditPredictionModelInput {
            project: project.clone(),
            buffer: active_buffer,
            snapshot,
            position,
            events,
            related_files,
            mode,
            trigger,
            diagnostic_search_range,
            debug_tx,
            can_collect_data,
            is_open_source,
        };

        let capture_data = (can_collect_data && rand::random_ratio(1, 1000)).then(|| stored_events);

        let task = match self.edit_prediction_model {
            EditPredictionModel::Zeta => {
                zeta::request_prediction_with_zeta(self, inputs, capture_data, cx)
            }
            EditPredictionModel::Fim { format } => fim::request_prediction(inputs, format, cx),
            EditPredictionModel::Mercury => {
                self.mercury
                    .request_prediction(inputs, self.credentials_provider.clone(), cx)
            }
        };

        cx.spawn(async move |this, cx| {
            let prediction = task.await?;

            // Only fall back to diagnostics-based prediction if we got a
            // the model had nothing to suggest for the buffer
            if prediction.is_none()
                && allow_jump
                && has_events
                && !matches!(trigger, PredictEditsRequestTrigger::Diagnostics)
            {
                this.update(cx, |this, cx| {
                    this.refresh_prediction_from_diagnostics(
                        project,
                        DiagnosticSearchScope::Local,
                        cx,
                    );
                })?;
                return anyhow::Ok(None);
            }

            Ok(prediction)
        })
    }

    pub(crate) async fn next_diagnostic_location(
        active_buffer: Entity<Buffer>,
        active_buffer_snapshot: &BufferSnapshot,
        active_buffer_diagnostic_search_range: Range<Point>,
        active_buffer_cursor_point: Point,
        project: &Entity<Project>,
        cx: &mut AsyncApp,
    ) -> Result<Option<(Entity<Buffer>, language::Anchor)>> {
        let collaborator_cursor_rows: Vec<u32> = active_buffer_snapshot
            .selections_in_range(
                Anchor::min_max_range_for_buffer(active_buffer_snapshot.remote_id()),
                false,
            )
            .flat_map(|(_, _, _, selections)| {
                selections.map(|s| s.head().to_point(active_buffer_snapshot).row)
            })
            .collect();

        let mut jump_location = active_buffer_snapshot
            .diagnostic_groups(None)
            .into_iter()
            .filter_map(|(_, group)| {
                let range = &group.entries[group.primary_ix]
                    .range
                    .to_point(&active_buffer_snapshot);
                if range.overlaps(&active_buffer_diagnostic_search_range) {
                    return None;
                }
                let near_collaborator = collaborator_cursor_rows.iter().any(|&collab_row| {
                    range.start.row.abs_diff(collab_row) <= DIAGNOSTIC_LINES_RANGE
                });
                let near_local = active_buffer_cursor_point.row.abs_diff(range.start.row)
                    <= DIAGNOSTIC_LINES_RANGE;
                if near_collaborator && !near_local {
                    return None;
                }
                Some(range.start)
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

            let mut candidates: Vec<(ProjectPath, usize)> = project.read_with(cx, |project, cx| {
                project
                    .diagnostic_summaries(false, cx)
                    .filter(|(path, _, _)| Some(path) != active_buffer_path.as_ref())
                    .map(|(path, _, _)| {
                        let shared_prefix = path
                            .path
                            .components()
                            .zip(
                                active_buffer_path
                                    .as_ref()
                                    .map(|p| p.path.components())
                                    .unwrap_or_default(),
                            )
                            .take_while(|(a, b)| a == b)
                            .count();
                        (path, shared_prefix)
                    })
                    .collect()
            });

            candidates.sort_by_key(|c| std::cmp::Reverse(c.1));

            for (path, _) in candidates {
                let candidate_buffer = project
                    .update(cx, |project, cx| project.open_buffer(path, cx))
                    .await?;

                let (has_collaborators, diagnostic_position) =
                    candidate_buffer.read_with(cx, |buffer, _cx| {
                        let snapshot = buffer.snapshot();
                        let has_collaborators = snapshot
                            .selections_in_range(
                                Anchor::min_max_range_for_buffer(snapshot.remote_id()),
                                false,
                            )
                            .next()
                            .is_some();
                        let position = buffer
                            .buffer_diagnostics(None)
                            .into_iter()
                            .min_by_key(|entry| entry.diagnostic.severity)
                            .map(|entry| entry.range.start);
                        (has_collaborators, position)
                    });

                if has_collaborators {
                    continue;
                }

                if let Some(position) = diagnostic_position {
                    jump_location = Some((candidate_buffer, position));
                    break;
                }
            }
        }

        anyhow::Ok(jump_location)
    }

    async fn send_raw_llm_request(
        request: RawCompletionRequest,
        client: Arc<Client>,
        custom_url: Option<Arc<Url>>,
        llm_token: LlmApiToken,
        organization_id: Option<OrganizationId>,
        app_version: Version,
    ) -> Result<(RawCompletionResponse, Option<EditPredictionUsage>)> {
        let url = if let Some(custom_url) = custom_url {
            custom_url.as_ref().clone()
        } else {
            client
                .http_client()
                .build_zed_llm_url("/predict_edits/raw", &[])?
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
            organization_id,
            app_version,
            true,
        )
        .await
    }

    pub(crate) async fn send_v3_request(
        input: ZetaPromptInput,
        preferred_experiment: Option<String>,
        client: Arc<Client>,
        llm_token: LlmApiToken,
        organization_id: Option<OrganizationId>,
        app_version: Version,
        trigger: PredictEditsRequestTrigger,
        mode: PredictEditsMode,
    ) -> Result<(PredictEditsV3Response, Option<EditPredictionUsage>)> {
        let url = client
            .http_client()
            .build_zed_llm_url("/predict_edits/v3", &[])?;

        let request = PredictEditsV3Request { input, trigger };

        let json_bytes = serde_json::to_vec(&request)?;
        let compressed = zstd::encode_all(&json_bytes[..], 3)?;

        Self::send_api_request(
            |builder| {
                let builder = builder
                    .uri(url.as_ref())
                    .header("Content-Encoding", "zstd")
                    .header(PREDICT_EDITS_MODE_HEADER_NAME, mode.as_ref());
                let builder = if let Some(preferred_experiment) = preferred_experiment.as_deref() {
                    builder.header(PREFERRED_EXPERIMENT_HEADER_NAME, preferred_experiment)
                } else {
                    builder
                };
                let req = builder.body(compressed.clone().into());
                Ok(req?)
            },
            client,
            llm_token,
            organization_id,
            app_version,
            true,
        )
        .await
    }

    async fn send_api_request<Res>(
        build: impl Fn(http_client::http::request::Builder) -> Result<http_client::Request<AsyncBody>>,
        client: Arc<Client>,
        llm_token: LlmApiToken,
        organization_id: Option<OrganizationId>,
        app_version: Version,
        require_auth: bool,
    ) -> Result<(Res, Option<EditPredictionUsage>)>
    where
        Res: DeserializeOwned,
    {
        let http_client = client.http_client();
        let mut token = if require_auth {
            Some(
                client
                    .acquire_llm_token(&llm_token, organization_id.clone())
                    .await?,
            )
        } else {
            client
                .acquire_llm_token(&llm_token, organization_id.clone())
                .await
                .ok()
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
            } else if !did_retry && token.is_some() && response.needs_llm_token_refresh() {
                did_retry = true;
                token = Some(
                    client
                        .refresh_llm_token(&llm_token, organization_id.clone())
                        .await?,
                );
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
        self.get_or_init_project(project, cx)
            .context
            .update(cx, |store, cx| {
                store.refresh(buffer.clone(), cursor_position, cx);
            });
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
            .update(cx, |store, cx| {
                store.set_related_files(related_files, cx);
            });
    }

    #[cfg(feature = "cli-support")]
    pub fn set_recent_paths_for_project(
        &mut self,
        project: &Entity<Project>,
        paths: impl IntoIterator<Item = project::ProjectPath>,
        cx: &mut Context<Self>,
    ) {
        let project_state = self.get_or_init_project(project, cx);
        project_state.recent_paths = paths.into_iter().collect();
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

    pub(crate) fn is_data_collection_enabled(&self, cx: &App) -> bool {
        if !self.is_data_collection_allowed_by_organization(cx) {
            return false;
        }

        if cx.is_staff() {
            return true;
        }

        match all_language_settings(None, cx)
            .edit_predictions
            .allow_data_collection
        {
            EditPredictionDataCollectionChoice::Yes => true,
            EditPredictionDataCollectionChoice::No => false,
            // Fall back to the legacy KV entry captured when the store was
            // created, preserving existing users' choices without per-request
            // database reads.
            EditPredictionDataCollectionChoice::Default => self.legacy_data_collection_enabled,
        }
    }

    fn load_legacy_data_collection_enabled(cx: &App) -> bool {
        KeyValueStore::global(cx)
            .read_kvp(ZED_PREDICT_DATA_COLLECTION_CHOICE)
            .log_err()
            .flatten()
            .as_deref()
            == Some("true")
    }

    pub(crate) fn is_data_collection_allowed_by_organization(&self, cx: &App) -> bool {
        self.user_store
            .read(cx)
            .current_organization_configuration()
            .is_none_or(|organization_configuration| {
                organization_configuration
                    .edit_prediction
                    .is_feedback_enabled
            })
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
        let organization = self.user_store.read(cx).current_organization();

        self.rated_predictions.insert(prediction.id.clone());

        cx.background_spawn({
            let client = self.client.clone();
            let prediction_id = prediction.id.to_string();
            let inputs = serde_json::to_value(&prediction.inputs);
            let output = prediction
                .edit_preview
                .as_unified_diff(prediction.snapshot.file(), &prediction.edits);
            async move {
                client
                    .cloud_client()
                    .submit_edit_prediction_feedback(SubmitEditPredictionFeedbackBody {
                        organization_id: organization.map(|organization| organization.id.clone()),
                        request_id: prediction_id,
                        rating: match rating {
                            EditPredictionRating::Positive => "positive".to_string(),
                            EditPredictionRating::Negative => "negative".to_string(),
                        },
                        inputs: inputs?,
                        output,
                        feedback,
                    })
                    .await?;

                anyhow::Ok(())
            }
        })
        .detach_and_log_err(cx);

        cx.notify();
    }
}

fn collaborator_edit_overlaps_locality_region(
    project_state: &ProjectState,
    project: &Entity<Project>,
    buffer: &Entity<Buffer>,
    snapshot: &BufferSnapshot,
    edit_range: &Range<Anchor>,
    cx: &App,
) -> bool {
    let Some((active_buffer, Some(position))) = project_state.active_buffer(project, cx) else {
        return false;
    };

    if active_buffer.entity_id() != buffer.entity_id() {
        return false;
    }

    let locality_point_range = expand_context_syntactically_then_linewise(
        snapshot,
        (position..position).to_point(snapshot),
        COLLABORATOR_EDIT_LOCALITY_CONTEXT_TOKENS,
    );
    let locality_anchor_range = snapshot.anchor_range_inside(locality_point_range);

    edit_range.overlaps(&locality_anchor_range, snapshot)
}

fn merge_trailing_events_if_needed(
    events: &mut VecDeque<StoredEvent>,
    end_snapshot: &TextBufferSnapshot,
    latest_snapshot: &TextBufferSnapshot,
    latest_edit_range: &Range<Anchor>,
) {
    if let Some(last_event) = events.back() {
        if last_event.old_snapshot.remote_id() != latest_snapshot.remote_id() {
            return;
        }
        if !latest_snapshot
            .version
            .observed_all(&last_event.new_snapshot_version)
        {
            return;
        }
    }

    let mut next_old_event = None;
    let mut mergeable_count = 0;
    for old_event in events.iter().rev() {
        if let Some(next_old_event) = next_old_event
            && !old_event.can_merge(next_old_event, latest_snapshot, latest_edit_range)
        {
            break;
        }
        mergeable_count += 1;
        next_old_event = Some(old_event);
    }

    if mergeable_count <= 1 {
        return;
    }

    let mut events_to_merge = events.range(events.len() - mergeable_count..).peekable();
    let oldest_event = events_to_merge.peek().unwrap();
    let oldest_snapshot = oldest_event.old_snapshot.clone();
    let newest_snapshot = end_snapshot;
    let mut merged_edit_range = oldest_event.total_edit_range.clone();

    for event in events.range(events.len() - mergeable_count + 1..) {
        merged_edit_range =
            merge_anchor_ranges(&merged_edit_range, &event.total_edit_range, latest_snapshot);
    }

    if let Some((diff, edit_range)) = compute_diff_between_snapshots_in_range(
        &oldest_snapshot,
        newest_snapshot,
        &merged_edit_range,
    ) {
        let merged_event = match oldest_event.event.as_ref() {
            zeta_prompt::Event::BufferChange {
                old_path,
                path,
                in_open_source_repo,
                ..
            } => StoredEvent {
                event: Arc::new(zeta_prompt::Event::BufferChange {
                    old_path: old_path.clone(),
                    path: path.clone(),
                    diff,
                    in_open_source_repo: *in_open_source_repo,
                    predicted: events_to_merge.all(|e| {
                        matches!(
                            e.event.as_ref(),
                            zeta_prompt::Event::BufferChange {
                                predicted: true,
                                ..
                            }
                        )
                    }),
                }),
                old_snapshot: oldest_snapshot.clone(),
                new_snapshot_version: newest_snapshot.version.clone(),
                total_edit_range: newest_snapshot.anchor_before(edit_range.start)
                    ..newest_snapshot.anchor_before(edit_range.end),
            },
        };
        events.truncate(events.len() - mergeable_count);
        events.push_back(merged_event);
    }
}

fn merge_anchor_ranges(
    left: &Range<Anchor>,
    right: &Range<Anchor>,
    snapshot: &TextBufferSnapshot,
) -> Range<Anchor> {
    let start = if left.start.cmp(&right.start, snapshot).is_le() {
        left.start
    } else {
        right.start
    };
    let end = if left.end.cmp(&right.end, snapshot).is_ge() {
        left.end
    } else {
        right.end
    };
    start..end
}

#[derive(Error, Debug)]
#[error(
    "You must update to Zed version {minimum_version} or higher to continue using edit predictions."
)]
pub struct ZedUpdateRequiredError {
    minimum_version: Version,
}

struct ZedPredictUpsell;

fn is_upsell_dismissed(cx: &App) -> bool {
    // To make this backwards compatible with older versions of Zed, we
    // check if the user has seen the previous Edit Prediction Onboarding
    // before, by checking the data collection choice which was written to
    // the database once the user clicked on "Accept and Enable"
    let kvp = KeyValueStore::global(cx);
    if kvp
        .read_kvp(ZED_PREDICT_DATA_COLLECTION_CHOICE)
        .log_err()
        .is_some_and(|s| s.is_some())
    {
        return true;
    }

    kvp.read_kvp(ZedPredictUpsell::KEY)
        .log_err()
        .is_some_and(|s| s.is_some())
}

impl Dismissable for ZedPredictUpsell {
    const KEY: &'static str = "dismissed-edit-predict-upsell";

    fn dismissed(cx: &App) -> bool {
        is_upsell_dismissed(cx)
    }
}

pub fn should_show_upsell_modal(cx: &App) -> bool {
    !is_upsell_dismissed(cx)
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
                    .edit_predictions
                    .get_or_insert_default()
                    .provider = Some(EditPredictionProvider::None)
            });
        });
        fn copilot_for_project(project: &Entity<Project>, cx: &mut App) -> Option<Entity<Copilot>> {
            EditPredictionStore::try_global(cx).and_then(|store| {
                store.update(cx, |this, cx| this.start_copilot_for_project(project, cx))
            })
        }

        workspace.register_action(|workspace, _: &SignIn, window, cx| {
            if let Some(copilot) = copilot_for_project(workspace.project(), cx) {
                copilot_ui::initiate_sign_in(copilot, window, cx);
            }
        });
        workspace.register_action(|workspace, _: &Reinstall, window, cx| {
            if let Some(copilot) = copilot_for_project(workspace.project(), cx) {
                copilot_ui::reinstall_and_sign_in(copilot, window, cx);
            }
        });
        workspace.register_action(|workspace, _: &SignOut, window, cx| {
            if let Some(copilot) = copilot_for_project(workspace.project(), cx) {
                copilot_ui::initiate_sign_out(copilot, window, cx);
            }
        });
    })
    .detach();
}
