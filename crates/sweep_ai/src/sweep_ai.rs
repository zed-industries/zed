use anyhow::Result;
use collections::HashMap;
use edit_prediction::DataCollectionState;
use gpui::{
    App, AppContext as _, AsyncApp, Context, Entity, EntityId, Global, SemanticVersion,
    SharedString, Subscription, Task,
};
use http_client::{AsyncBody, HttpClient, Method, Request, Response};
use language::{
    Anchor, Buffer, BufferSnapshot, EditPreview, File, OffsetRangeExt, ToOffset, ToPoint, text_diff,
};
use project::{Project, ProjectPath};
use settings::WorktreeId;
use std::collections::{VecDeque, hash_map};
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
use util::rel_path::RelPath;
use workspace::notifications::{ErrorMessagePrompt, NotificationId, show_app_notification};
use worktree::Worktree;

const BUFFER_CHANGE_GROUPING_INTERVAL: Duration = Duration::from_secs(1);
const MAX_EVENT_COUNT: usize = 16;

#[derive(Clone)]
struct SweepAiGlobal(Entity<SweepAi>);

impl Global for SweepAiGlobal {}

#[derive(Clone)]
pub struct EditPrediction {
    path: Arc<Path>,
    excerpt_range: Range<usize>,
    cursor_offset: usize,
    edits: Arc<[(Range<Anchor>, Arc<str>)]>,
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

    fn interpolate(&self, new_snapshot: &BufferSnapshot) -> Option<Vec<(Range<Anchor>, Arc<str>)>> {
        edit_prediction::interpolate_edits(&self.snapshot, new_snapshot, &self.edits)
    }
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

pub struct SweepAi {
    projects: HashMap<EntityId, SweepAiProject>,
    shown_completions: VecDeque<EditPrediction>,
    http_client: Arc<dyn HttpClient>,
}

struct SweepAiProject {
    events: VecDeque<Event>,
    registered_buffers: HashMap<gpui::EntityId, RegisteredBuffer>,
}

impl SweepAi {
    pub fn global(cx: &mut App) -> Option<Entity<Self>> {
        cx.try_global::<SweepAiGlobal>()
            .map(|global| global.0.clone())
    }

    pub fn register(worktree: Option<Entity<Worktree>>, cx: &mut App) -> Entity<Self> {
        Self::global(cx).unwrap_or_else(|| {
            let entity = cx.new(|cx| Self::new(cx));
            cx.set_global(SweepAiGlobal(entity.clone()));
            entity
        })
    }

    pub fn clear_history(&mut self) {
        for zeta_project in self.projects.values_mut() {
            zeta_project.events.clear();
        }
    }

    fn new(http_client: Arc<dyn HttpClient>, cx: &mut Context<Self>) -> Self {
        Self {
            http_client,
            projects: HashMap::default(),
            shown_completions: VecDeque::new(),
        }
    }

    fn get_or_init_sweep_ai_project(
        &mut self,
        project: &Entity<Project>,
        cx: &mut Context<Self>,
    ) -> &mut SweepAiProject {
        let project_id = project.entity_id();
        match self.projects.entry(project_id) {
            hash_map::Entry::Occupied(entry) => entry.into_mut(),
            hash_map::Entry::Vacant(entry) => {
                cx.observe_release(project, move |this, _, _cx| {
                    this.projects.remove(&project_id);
                })
                .detach();
                entry.insert(SweepAiProject {
                    events: VecDeque::with_capacity(MAX_EVENT_COUNT),
                    registered_buffers: HashMap::default(),
                })
            }
        }
    }

    fn push_event(zeta_project: &mut SweepAiProject, event: Event) {
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
        let zeta_project = self.get_or_init_sweep_ai_project(project, cx);
        Self::register_buffer_impl(zeta_project, buffer, project, cx);
    }

    fn register_buffer_impl<'a>(
        zeta_project: &'a mut SweepAiProject,
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

    pub fn request_completion(
        &mut self,
        project: &Entity<Project>,
        buffer: &Entity<Buffer>,
        position: language::Anchor,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<EditPrediction>>> {
        todo!();
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
                    .unwrap_or(RelPath::unix("untitled").unwrap());
                let new_path = new_snapshot
                    .file()
                    .map(|f| f.path().as_ref())
                    .unwrap_or(RelPath::unix("untitled").unwrap());
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
            new_range == old_range && new_text.starts_with(old_text.as_ref())
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
    project: Entity<Project>,
}

impl ZetaEditPredictionProvider {
    pub const THROTTLE_TIMEOUT: Duration = Duration::from_millis(300);

    pub fn new(
        zeta: Entity<Zeta>,
        project: Entity<Project>,
        singleton_buffer: Option<Entity<Buffer>>,
    ) -> Self {
        Self {
            zeta,
            singleton_buffer,
            pending_completions: ArrayVec::new(),
            next_pending_completion_id: 0,
            current_completion: None,
            last_request_timestamp: Instant::now(),
            project,
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
        buffer: Entity<Buffer>,
        position: language::Anchor,
        _debounce: bool,
        cx: &mut Context<Self>,
    ) {
        if self.zeta.read(cx).update_required {
            return;
        }

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

        let project = self.project.clone();
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

        Some(edit_prediction::EditPrediction::Local {
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
    use clock::{FakeSystemClock, ReplicaId};
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
        let edits: Arc<[(Range<Anchor>, Arc<str>)]> = cx.update(|cx| {
            to_completion_edits([(2..5, "REM".into()), (9..11, "".into())], &buffer, cx).into()
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
                vec![(2..5, "REM".into()), (9..11, "".into())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(2..5, "")], None, cx));
            assert_eq!(
                from_completion_edits(
                    &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(2..2, "REM".into()), (6..8, "".into())]
            );

            buffer.update(cx, |buffer, cx| buffer.undo(cx));
            assert_eq!(
                from_completion_edits(
                    &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(2..5, "REM".into()), (9..11, "".into())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(2..5, "R")], None, cx));
            assert_eq!(
                from_completion_edits(
                    &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(3..3, "EM".into()), (7..9, "".into())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(3..3, "E")], None, cx));
            assert_eq!(
                from_completion_edits(
                    &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(4..4, "M".into()), (8..10, "".into())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(4..4, "M")], None, cx));
            assert_eq!(
                from_completion_edits(
                    &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(9..11, "".into())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(4..5, "")], None, cx));
            assert_eq!(
                from_completion_edits(
                    &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(4..4, "M".into()), (8..10, "".into())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(8..10, "")], None, cx));
            assert_eq!(
                from_completion_edits(
                    &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(4..4, "M".into())]
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
                ReplicaId::new(1),
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
                                        request_id: Uuid::new_v4().to_string(),
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
        iterator: impl IntoIterator<Item = (Range<usize>, Arc<str>)>,
        buffer: &Entity<Buffer>,
        cx: &App,
    ) -> Vec<(Range<Anchor>, Arc<str>)> {
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
        editor_edits: &[(Range<Anchor>, Arc<str>)],
        buffer: &Entity<Buffer>,
        cx: &App,
    ) -> Vec<(Range<usize>, Arc<str>)> {
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
