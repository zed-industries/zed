mod api;

use anyhow::{Context as _, Result};
use arrayvec::ArrayVec;
use collections::HashMap;
use futures::AsyncReadExt as _;
use gpui::{App, AppContext as _, Context, Entity, EntityId, Global, Task};
use http_client::{AsyncBody, Method};
use language::{Anchor, Buffer, BufferSnapshot, EditPreview, ToOffset as _, ToPoint, text_diff};
use project::Project;
use std::collections::{VecDeque, hash_map};
use std::fmt::{self, Display};
use std::mem;
use std::{
    cmp,
    fmt::Write,
    ops::Range,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};
use util::ResultExt;
use util::rel_path::RelPath;
use uuid::Uuid;

use crate::api::{ActionType, AutocompleteRequest, AutocompleteResponse, UserAction};

const BUFFER_CHANGE_GROUPING_INTERVAL: Duration = Duration::from_secs(1);
const MAX_EVENT_COUNT: usize = 16;

#[derive(Clone)]
struct SweepAiGlobal(Entity<SweepAi>);

impl Global for SweepAiGlobal {}

#[derive(Clone)]
pub struct EditPrediction {
    id: EditPredictionId,
    path: Arc<Path>,
    edits: Arc<[(Range<Anchor>, Arc<str>)]>,
    snapshot: BufferSnapshot,
    edit_preview: EditPreview,
}

impl EditPrediction {
    fn interpolate(&self, new_snapshot: &BufferSnapshot) -> Option<Vec<(Range<Anchor>, Arc<str>)>> {
        edit_prediction::interpolate_edits(&self.snapshot, new_snapshot, &self.edits)
    }
}

impl fmt::Debug for EditPrediction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EditPrediction")
            .field("path", &self.path)
            .field("edits", &self.edits)
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct EditPredictionId(String);

impl Display for EditPredictionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct SweepAi {
    projects: HashMap<EntityId, SweepAiProject>,
    shown_completions: VecDeque<EditPrediction>,
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

    pub fn register(cx: &mut App) -> Entity<Self> {
        Self::global(cx).unwrap_or_else(|| {
            let entity = cx.new(|_cx| Self::new());
            cx.set_global(SweepAiGlobal(entity.clone()));
            entity
        })
    }

    pub fn clear_history(&mut self) {
        for sweep_ai_project in self.projects.values_mut() {
            sweep_ai_project.events.clear();
        }
    }

    fn new() -> Self {
        Self {
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

    fn push_event(sweep_ai_project: &mut SweepAiProject, event: Event) {
        let events = &mut sweep_ai_project.events;

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
        let sweep_ai_project = self.get_or_init_sweep_ai_project(project, cx);
        Self::register_buffer_impl(sweep_ai_project, buffer, project, cx);
    }

    fn register_buffer_impl<'a>(
        sweep_ai_project: &'a mut SweepAiProject,
        buffer: &Entity<Buffer>,
        project: &Entity<Project>,
        cx: &mut Context<Self>,
    ) -> &'a mut RegisteredBuffer {
        let buffer_id = buffer.entity_id();
        match sweep_ai_project.registered_buffers.entry(buffer_id) {
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
                            let Some(sweep_ai_project) = this.projects.get_mut(&project_entity_id)
                            else {
                                return;
                            };
                            sweep_ai_project.registered_buffers.remove(&buffer_id);
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
        let snapshot = buffer.read(cx).snapshot();
        let full_path: Arc<Path> = snapshot
            .file()
            .map(|file| file.full_path(cx))
            .unwrap_or_else(|| "untitled".into())
            .into();

        let project_file = project::File::from_dyn(snapshot.file());
        let repo_name = project_file
            .map(|file| file.worktree.read(cx).root_name_str())
            .unwrap_or("untitled")
            .into();
        let offset = position.to_offset(&snapshot);

        let project_state = self.get_or_init_sweep_ai_project(project, cx);
        let events = project_state.events.clone();

        let client = reqwest::Client::builder().use_rustls_tls().build().unwrap();
        let tokio_handle = reqwest_client::runtime().handle().clone();

        let result = cx.background_spawn({
            let full_path = full_path.clone();
            async move {
                let text = snapshot.text();

                let mut recent_changes = String::new();

                for event in events {
                    writeln!(&mut recent_changes, "{event}")?;
                }

                let request_body = AutocompleteRequest {
                    debug_info: "Zed".into(),
                    device_id: "todo".into(),
                    repo_name,
                    branch: None,
                    file_path: full_path.clone(),
                    file_contents: text.clone(),
                    original_file_contents: text,
                    cursor_position: offset,
                    recent_changes: recent_changes.clone(),
                    // todo! what's the difference?
                    recent_changes_high_res: recent_changes,
                    // todo! rest
                    changes_above_cursor: false,
                    file_chunks: vec![],
                    retrieval_chunks: vec![],
                    recent_user_actions: vec![],
                    multiple_suggestions: false,
                    privacy_mode_enabled: false,
                    client_ip: None,
                    ping: false,
                };

                let mut buf: Vec<u8> = Vec::new();
                let writer = brotli::CompressorWriter::new(&mut buf, 4096, 11, 22);
                serde_json::to_writer(writer, &request_body)?;

                // let body: AsyncBody = buf.into();

                const SWEEP_API_URL: &str =
                    "https://autocomplete.sweep.dev/backend/next_edit_autocomplete";

                // let request = http_client::Request::builder()
                //     .uri(SWEEP_API_URL)
                //     .header("Content-Type", "application/json")
                //     .header(
                //         "Authorization",
                //         // todo!
                //         format!("Bearer {}", std::env::var("SWEEP_TOKEN").unwrap()),
                //     )
                //     // .header("Connection", "keep-alive")
                //     .header("Content-Encoding", "br")
                //     .method(Method::POST)
                //     .body(body)?;

                let request = client
                    .post(SWEEP_API_URL)
                    .header("Content-Type", "application/json")
                    .header(
                        "Authorization",
                        // todo!
                        format!("Bearer {}", std::env::var("SWEEP_TOKEN").unwrap()),
                    )
                    // .header("Connection", "keep-alive")
                    .header("Content-Encoding", "br")
                    .body(buf);

                let response = tokio_handle
                    .spawn(async move { request.send().await })
                    .await??;
                // let mut response = http_client.send(request).await?;

                dbg!(response.headers());
                dbg!(response.status());

                // let mut body: Vec<u8> = Vec::new();
                let body = response.bytes().await?;
                // let body = response.body_mut().collect().await;
                eprintln!("{body:?}");

                // if !response.status().is_success() {
                //     anyhow::bail!(
                //         "Request failed with status: {:?}\nBody: {}",
                //         response.status(),
                //         String::from_utf8_lossy(&body),
                //     );
                // };

                let response: AutocompleteResponse = serde_json::from_slice(&body)?;
                dbg!(&response);

                let old_text = snapshot
                    .text_for_range(response.start_index..response.end_index)
                    .collect::<String>();
                let edits = text_diff(&old_text, &response.completion)
                    .into_iter()
                    .map(|(range, text)| {
                        (
                            snapshot.anchor_after(response.start_index + range.start)
                                ..snapshot.anchor_before(response.start_index + range.end),
                            text,
                        )
                    })
                    .collect::<Vec<_>>();

                anyhow::Ok((response.autocomplete_id, edits, snapshot))
            }
        });

        let buffer = buffer.clone();

        cx.spawn(async move |_, cx| {
            let (id, edits, old_snapshot) = result.await?;

            if edits.is_empty() {
                return anyhow::Ok(None);
            }

            let Some((edits, new_snapshot, preview_task)) =
                buffer.read_with(cx, |buffer, cx| {
                    let new_snapshot = buffer.snapshot();

                    let edits: Arc<[(Range<Anchor>, Arc<str>)]> =
                        edit_prediction::interpolate_edits(&old_snapshot, &new_snapshot, &edits)?
                            .into();
                    let preview_task = buffer.preview_edits(edits.clone(), cx);

                    Some((edits, new_snapshot, preview_task))
                })?
            else {
                return anyhow::Ok(None);
            };

            let prediction = EditPrediction {
                id: EditPredictionId(id),
                path: full_path,
                edits,
                snapshot: new_snapshot,
                edit_preview: preview_task.await,
            };

            anyhow::Ok(Some(prediction))
        })
    }

    fn report_changes_for_buffer(
        &mut self,
        buffer: &Entity<Buffer>,
        project: &Entity<Project>,
        cx: &mut Context<Self>,
    ) -> BufferSnapshot {
        let sweep_ai_project = self.get_or_init_sweep_ai_project(project, cx);
        let registered_buffer = Self::register_buffer_impl(sweep_ai_project, buffer, project, cx);

        let new_snapshot = buffer.read(cx).snapshot();
        if new_snapshot.version != registered_buffer.snapshot.version {
            let old_snapshot = mem::replace(&mut registered_buffer.snapshot, new_snapshot.clone());
            Self::push_event(
                sweep_ai_project,
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

impl Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::BufferChange {
                old_snapshot,
                new_snapshot,
                ..
            } => {
                let old_path = old_snapshot
                    .file()
                    .map(|f| f.path().as_ref())
                    .unwrap_or(RelPath::unix("untitled").unwrap());
                let new_path = new_snapshot
                    .file()
                    .map(|f| f.path().as_ref())
                    .unwrap_or(RelPath::unix("untitled").unwrap());
                if old_path != new_path {
                    // todo! confirm how to do this for sweep
                    writeln!(f, "User renamed {:?} to {:?}\n", old_path, new_path)?;
                }

                let diff = language::unified_diff(&old_snapshot.text(), &new_snapshot.text());
                if !diff.is_empty() {
                    write!(
                        f,
                        "File: {}:\n{}\n",
                        new_path.display(util::paths::PathStyle::Posix),
                        diff
                    )?
                }

                fmt::Result::Ok(())
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

pub struct SweepAiEditPredictionProvider {
    sweep_ai: Entity<SweepAi>,
    singleton_buffer: Option<Entity<Buffer>>,
    pending_completions: ArrayVec<PendingCompletion, 2>,
    next_pending_completion_id: usize,
    current_completion: Option<CurrentEditPrediction>,
    last_request_timestamp: Instant,
    project: Entity<Project>,
}

impl SweepAiEditPredictionProvider {
    pub const THROTTLE_TIMEOUT: Duration = Duration::from_millis(300);

    pub fn new(
        sweep_ai: Entity<SweepAi>,
        project: Entity<Project>,
        singleton_buffer: Option<Entity<Buffer>>,
    ) -> Self {
        Self {
            sweep_ai,
            singleton_buffer,
            pending_completions: ArrayVec::new(),
            next_pending_completion_id: 0,
            current_completion: None,
            last_request_timestamp: Instant::now(),
            project,
        }
    }
}

impl edit_prediction::EditPredictionProvider for SweepAiEditPredictionProvider {
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

    fn is_enabled(
        &self,
        _buffer: &Entity<Buffer>,
        _cursor_position: language::Anchor,
        _cx: &App,
    ) -> bool {
        // todo!
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
                this.sweep_ai.update(cx, |sweep_ai, cx| {
                    sweep_ai.request_completion(&project, &buffer, position, cx)
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
                        this.current_completion = Some(new_completion);
                    }
                } else {
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
