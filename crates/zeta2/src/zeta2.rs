use anyhow::{Context as _, Result, anyhow};
use chrono::TimeDelta;
use client::{Client, EditPredictionUsage, UserStore};
use cloud_llm_client::predict_edits_v3::{self, PromptFormat, Signature};
use cloud_llm_client::{
    EXPIRED_LLM_TOKEN_HEADER_NAME, MINIMUM_REQUIRED_VERSION_HEADER_NAME, ZED_VERSION_HEADER_NAME,
};
use cloud_zeta2_prompt::DEFAULT_MAX_PROMPT_BYTES;
use edit_prediction_context::{
    DeclarationId, EditPredictionContext, EditPredictionExcerptOptions, SyntaxIndex,
    SyntaxIndexState,
};
use futures::AsyncReadExt as _;
use futures::channel::mpsc;
use gpui::http_client::Method;
use gpui::{
    App, Entity, EntityId, Global, SemanticVersion, SharedString, Subscription, Task, WeakEntity,
    http_client, prelude::*,
};
use language::BufferSnapshot;
use language::{Buffer, DiagnosticSet, LanguageServerId, ToOffset as _, ToPoint};
use language_model::{LlmApiToken, RefreshLlmTokenListener};
use project::Project;
use release_channel::AppVersion;
use std::collections::{HashMap, VecDeque, hash_map};
use std::path::Path;
use std::str::FromStr as _;
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;
use util::rel_path::RelPathBuf;
use util::some_or_debug_panic;
use workspace::notifications::{ErrorMessagePrompt, NotificationId, show_app_notification};

mod prediction;
mod provider;

use crate::prediction::{EditPrediction, edits_from_response, interpolate_edits};
pub use provider::ZetaEditPredictionProvider;

const BUFFER_CHANGE_GROUPING_INTERVAL: Duration = Duration::from_secs(1);

/// Maximum number of events to track.
const MAX_EVENT_COUNT: usize = 16;

pub const DEFAULT_EXCERPT_OPTIONS: EditPredictionExcerptOptions = EditPredictionExcerptOptions {
    max_bytes: 512,
    min_bytes: 128,
    target_before_cursor_over_total_bytes: 0.5,
};

pub const DEFAULT_OPTIONS: ZetaOptions = ZetaOptions {
    excerpt: DEFAULT_EXCERPT_OPTIONS,
    max_prompt_bytes: DEFAULT_MAX_PROMPT_BYTES,
    max_diagnostic_bytes: 2048,
    prompt_format: PromptFormat::MarkedExcerpt,
};

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
    debug_tx: Option<mpsc::UnboundedSender<Result<PredictionDebugInfo, String>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ZetaOptions {
    pub excerpt: EditPredictionExcerptOptions,
    pub max_prompt_bytes: usize,
    pub max_diagnostic_bytes: usize,
    pub prompt_format: predict_edits_v3::PromptFormat,
}

pub struct PredictionDebugInfo {
    pub context: EditPredictionContext,
    pub retrieval_time: TimeDelta,
    pub request: RequestDebugInfo,
    pub buffer: WeakEntity<Buffer>,
    pub position: language::Anchor,
}

pub type RequestDebugInfo = predict_edits_v3::DebugInfo;

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
            projects: HashMap::new(),
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

    pub fn debug_info(&mut self) -> mpsc::UnboundedReceiver<Result<PredictionDebugInfo, String>> {
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
        let options = self.options.clone();
        let snapshot = buffer.read(cx).snapshot();
        let Some(excerpt_path) = snapshot.file().map(|path| path.full_path(cx).into()) else {
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
                    .map(|event| match event {
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

                            predict_edits_v3::Event::BufferChange {
                                old_path,
                                path,
                                diff: language::unified_diff(
                                    &old_snapshot.text(),
                                    &new_snapshot.text(),
                                ),
                                //todo: Actually detect if this edit was predicted or not
                                predicted: false,
                            }
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let diagnostics = snapshot.diagnostic_sets().clone();

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

                let Some(context) = EditPredictionContext::gather_context(
                    cursor_point,
                    &snapshot,
                    &options.excerpt,
                    index_state.as_deref(),
                ) else {
                    return Ok(None);
                };

                let debug_context = if let Some(debug_tx) = debug_tx {
                    Some((debug_tx, context.clone()))
                } else {
                    None
                };

                let (diagnostic_groups, diagnostic_groups_truncated) =
                    Self::gather_nearby_diagnostics(
                        cursor_offset,
                        &diagnostics,
                        &snapshot,
                        options.max_diagnostic_bytes,
                    );

                let request = make_cloud_request(
                    excerpt_path,
                    context,
                    events,
                    // TODO data collection
                    false,
                    diagnostic_groups,
                    diagnostic_groups_truncated,
                    None,
                    debug_context.is_some(),
                    &worktree_snapshots,
                    index_state.as_deref(),
                    Some(options.max_prompt_bytes),
                    options.prompt_format,
                );

                let retrieval_time = chrono::Utc::now() - before_retrieval;
                let response = Self::perform_request(client, llm_token, app_version, request).await;

                if let Some((debug_tx, context)) = debug_context {
                    debug_tx
                        .unbounded_send(response.as_ref().map_err(|err| err.to_string()).and_then(
                            |response| {
                                let Some(request) =
                                    some_or_debug_panic(response.0.debug_info.clone())
                                else {
                                    return Err("Missing debug info".to_string());
                                };
                                Ok(PredictionDebugInfo {
                                    context,
                                    request,
                                    retrieval_time,
                                    buffer: buffer.downgrade(),
                                    position,
                                })
                            },
                        ))
                        .ok();
                }

                let (response, usage) = response?;
                let edits = edits_from_response(&response.edits, &snapshot);

                anyhow::Ok(Some((response.request_id, edits, usage)))
            }
        });

        let buffer = buffer.clone();

        cx.spawn(async move |this, cx| {
            match request_task.await {
                Ok(Some((id, edits, usage))) => {
                    if let Some(usage) = usage {
                        this.update(cx, |this, cx| {
                            this.user_store.update(cx, |user_store, cx| {
                                user_store.update_edit_prediction_usage(usage, cx);
                            });
                        })
                        .ok();
                    }

                    // TODO telemetry: duration, etc
                    let Some((edits, snapshot, edit_preview_task)) =
                        buffer.read_with(cx, |buffer, cx| {
                            let new_snapshot = buffer.snapshot();
                            let edits: Arc<[_]> =
                                interpolate_edits(&snapshot, &new_snapshot, edits)?.into();
                            Some((edits.clone(), new_snapshot, buffer.preview_edits(edits, cx)))
                        })?
                    else {
                        return Ok(None);
                    };

                    Ok(Some(EditPrediction {
                        id: id.into(),
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
                &options.excerpt,
                index_state.as_deref(),
            )
            .context("Failed to select excerpt")
            .map(|context| {
                make_cloud_request(
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
}

#[derive(Error, Debug)]
#[error(
    "You must update to Zed version {minimum_version} or higher to continue using edit predictions."
)]
pub struct ZedUpdateRequiredError {
    minimum_version: SemanticVersion,
}

fn make_cloud_request(
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
        range: parent_declaration.signature_range(),
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
    use cloud_llm_client::predict_edits_v3;
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
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;
    use uuid::Uuid;

    use crate::Zeta;

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
        assert_eq!(request.cursor_offset, 10);

        respond_tx
            .send(predict_edits_v3::PredictEditsResponse {
                request_id: Uuid::new_v4(),
                edits: vec![predict_edits_v3::Edit {
                    path: Path::new(path!("root/foo.md")).into(),
                    range: 0..snapshot.len(),
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
                    range: 0..snapshot.len(),
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
                                serde_json::to_string(&res_rx.await.unwrap()).unwrap()
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
