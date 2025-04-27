mod completion_diff_element;
mod init;
mod input_excerpt;
mod license_detection;
mod onboarding_modal;
mod onboarding_telemetry;
mod rate_completion_modal;

pub(crate) use completion_diff_element::*;
use db::kvp::KEY_VALUE_STORE;
pub use init::*;
use inline_completion::{DataCollectionState, EditPredictionUsage};
use license_detection::LICENSE_FILES_TO_CHECK;
pub use license_detection::is_license_eligible_for_data_collection;
pub use rate_completion_modal::*;

use anyhow::{Context as _, Result, anyhow};
use arrayvec::ArrayVec;
use client::{Client, UserStore};
use collections::{HashMap, HashSet, VecDeque};
use futures::AsyncReadExt;
use gpui::{
    App, AppContext as _, AsyncApp, Context, Entity, EntityId, Global, SemanticVersion,
    Subscription, Task, WeakEntity, actions,
};
use http_client::{HttpClient, Method};
use input_excerpt::excerpt_for_cursor_position;
use language::{
    Anchor, Buffer, BufferSnapshot, EditPreview, OffsetRangeExt, ToOffset, ToPoint, text_diff,
};
use language_model::{LlmApiToken, RefreshLlmTokenListener};
use postage::watch;
use project::Project;
use release_channel::AppVersion;
use settings::WorktreeId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};
use std::str::FromStr;
use std::{
    borrow::Cow,
    cmp,
    fmt::Write,
    future::Future,
    mem,
    ops::Range,
    path::Path,
    rc::Rc,
    sync::Arc,
    sync::LazyLock,
    time::{Duration, Instant},
};
use telemetry_events::InlineCompletionRating;
use thiserror::Error;
use util::ResultExt;
use uuid::Uuid;
use workspace::Workspace;
use workspace::notifications::{ErrorMessagePrompt, NotificationId};
use worktree::Worktree;
use zed_llm_client::{
    EXPIRED_LLM_TOKEN_HEADER_NAME, MINIMUM_REQUIRED_VERSION_HEADER_NAME, PredictEditsBody,
    PredictEditsResponse,
};

const CURSOR_MARKER: &'static str = "<|user_cursor_is_here|>";
const START_OF_FILE_MARKER: &'static str = "<|start_of_file|>";
const EDITABLE_REGION_START_MARKER: &'static str = "<|editable_region_start|>";
const EDITABLE_REGION_END_MARKER: &'static str = "<|editable_region_end|>";
const BUFFER_CHANGE_GROUPING_INTERVAL: Duration = Duration::from_secs(1);
const ZED_PREDICT_DATA_COLLECTION_CHOICE: &str = "zed_predict_data_collection_choice";

const MAX_CONTEXT_TOKENS: usize = 150;
const MAX_REWRITE_TOKENS: usize = 350;
const MAX_EVENT_TOKENS: usize = 500;

/// Maximum number of events to track.
const MAX_EVENT_COUNT: usize = 16;

actions!(edit_prediction, [ClearHistory]);

static ZED_PREDICT_EDITS_URL: LazyLock<Option<String>> =
    LazyLock::new(|| std::env::var("ZED_PREDICT_EDITS_URL").ok());

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct ZetaSettingsContent {
    predict_edits_url: Option<String>,
}

#[derive(Deserialize)]
pub struct ZetaSettings {
    pub predict_edits_url: String,
}

impl Settings for ZetaSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = ZetaSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        let mut result = sources.json_merge::<Self>()?;
        if let Some(predict_edits_url) = &*ZED_PREDICT_EDITS_URL {
            result.predict_edits_url.clone_from(predict_edits_url)
        }
        Ok(result)
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct InlineCompletionId(Uuid);

impl From<InlineCompletionId> for gpui::ElementId {
    fn from(value: InlineCompletionId) -> Self {
        gpui::ElementId::Uuid(value.0)
    }
}

impl std::fmt::Display for InlineCompletionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone)]
struct ZetaGlobal(Entity<Zeta>);

impl Global for ZetaGlobal {}

#[derive(Clone)]
pub struct InlineCompletion {
    id: InlineCompletionId,
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
    request_sent_at: Instant,
    response_received_at: Instant,
}

impl InlineCompletion {
    fn latency(&self) -> Duration {
        self.response_received_at
            .duration_since(self.request_sent_at)
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

    let mut model_edits = current_edits.into_iter().peekable();
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

impl std::fmt::Debug for InlineCompletion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InlineCompletion")
            .field("id", &self.id)
            .field("path", &self.path)
            .field("edits", &self.edits)
            .finish_non_exhaustive()
    }
}

pub struct Zeta {
    workspace: Option<WeakEntity<Workspace>>,
    client: Arc<Client>,
    events: VecDeque<Event>,
    registered_buffers: HashMap<gpui::EntityId, RegisteredBuffer>,
    shown_completions: VecDeque<InlineCompletion>,
    rated_completions: HashSet<InlineCompletionId>,
    data_collection_choice: Entity<DataCollectionChoice>,
    llm_token: LlmApiToken,
    _llm_token_subscription: Subscription,
    last_usage: Option<EditPredictionUsage>,
    /// Whether the terms of service have been accepted.
    tos_accepted: bool,
    /// Whether an update to a newer version of Zed is required to continue using Zeta.
    update_required: bool,
    _user_store_subscription: Subscription,
    license_detection_watchers: HashMap<WorktreeId, Rc<LicenseDetectionWatcher>>,
}

impl Zeta {
    pub fn global(cx: &mut App) -> Option<Entity<Self>> {
        cx.try_global::<ZetaGlobal>().map(|global| global.0.clone())
    }

    pub fn register(
        workspace: Option<WeakEntity<Workspace>>,
        worktree: Option<Entity<Worktree>>,
        client: Arc<Client>,
        user_store: Entity<UserStore>,
        cx: &mut App,
    ) -> Entity<Self> {
        let this = Self::global(cx).unwrap_or_else(|| {
            let entity = cx.new(|cx| Self::new(workspace, client, user_store, cx));
            cx.set_global(ZetaGlobal(entity.clone()));
            entity
        });

        this.update(cx, move |this, cx| {
            if let Some(worktree) = worktree {
                worktree.update(cx, |worktree, cx| {
                    this.license_detection_watchers
                        .entry(worktree.id())
                        .or_insert_with(|| Rc::new(LicenseDetectionWatcher::new(worktree, cx)));
                });
            }
        });

        this
    }

    pub fn clear_history(&mut self) {
        self.events.clear();
    }

    fn new(
        workspace: Option<WeakEntity<Workspace>>,
        client: Arc<Client>,
        user_store: Entity<UserStore>,
        cx: &mut Context<Self>,
    ) -> Self {
        let refresh_llm_token_listener = RefreshLlmTokenListener::global(cx);

        let data_collection_choice = Self::load_data_collection_choices();
        let data_collection_choice = cx.new(|_| data_collection_choice);

        Self {
            workspace,
            client,
            events: VecDeque::new(),
            shown_completions: VecDeque::new(),
            rated_completions: HashSet::default(),
            registered_buffers: HashMap::default(),
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
            last_usage: None,
            tos_accepted: user_store
                .read(cx)
                .current_user_has_accepted_terms()
                .unwrap_or(false),
            update_required: false,
            _user_store_subscription: cx.subscribe(&user_store, |this, user_store, event, cx| {
                match event {
                    client::user::Event::PrivateUserInfoUpdated => {
                        this.tos_accepted = user_store
                            .read(cx)
                            .current_user_has_accepted_terms()
                            .unwrap_or(false);
                    }
                    _ => {}
                }
            }),
            license_detection_watchers: HashMap::default(),
        }
    }

    fn push_event(&mut self, event: Event) {
        if let Some(Event::BufferChange {
            new_snapshot: last_new_snapshot,
            timestamp: last_timestamp,
            ..
        }) = self.events.back_mut()
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

        self.events.push_back(event);
        if self.events.len() >= MAX_EVENT_COUNT {
            self.events.drain(..MAX_EVENT_COUNT / 2);
        }
    }

    pub fn register_buffer(&mut self, buffer: &Entity<Buffer>, cx: &mut Context<Self>) {
        let buffer_id = buffer.entity_id();
        let weak_buffer = buffer.downgrade();

        if let std::collections::hash_map::Entry::Vacant(entry) =
            self.registered_buffers.entry(buffer_id)
        {
            let snapshot = buffer.read(cx).snapshot();

            entry.insert(RegisteredBuffer {
                snapshot,
                _subscriptions: [
                    cx.subscribe(buffer, move |this, buffer, event, cx| {
                        this.handle_buffer_event(buffer, event, cx);
                    }),
                    cx.observe_release(buffer, move |this, _buffer, _cx| {
                        this.registered_buffers.remove(&weak_buffer.entity_id());
                    }),
                ],
            });
        };
    }

    fn handle_buffer_event(
        &mut self,
        buffer: Entity<Buffer>,
        event: &language::BufferEvent,
        cx: &mut Context<Self>,
    ) {
        if let language::BufferEvent::Edited = event {
            self.report_changes_for_buffer(&buffer, cx);
        }
    }

    fn request_completion_impl<F, R>(
        &mut self,
        workspace: Option<Entity<Workspace>>,
        project: Option<&Entity<Project>>,
        buffer: &Entity<Buffer>,
        cursor: language::Anchor,
        can_collect_data: bool,
        cx: &mut Context<Self>,
        perform_predict_edits: F,
    ) -> Task<Result<Option<InlineCompletion>>>
    where
        F: FnOnce(PerformPredictEditsParams) -> R + 'static,
        R: Future<Output = Result<(PredictEditsResponse, Option<EditPredictionUsage>)>>
            + Send
            + 'static,
    {
        let snapshot = self.report_changes_for_buffer(&buffer, cx);
        let diagnostic_groups = snapshot.diagnostic_groups(None);
        let cursor_point = cursor.to_point(&snapshot);
        let cursor_offset = cursor_point.to_offset(&snapshot);
        let events = self.events.clone();
        let path: Arc<Path> = snapshot
            .file()
            .map(|f| Arc::from(f.full_path(cx).as_path()))
            .unwrap_or_else(|| Arc::from(Path::new("untitled")));

        let zeta = cx.entity();
        let client = self.client.clone();
        let llm_token = self.llm_token.clone();
        let app_version = AppVersion::global(cx);

        let buffer = buffer.clone();

        let local_lsp_store =
            project.and_then(|project| project.read(cx).lsp_store().read(cx).as_local());
        let diagnostic_groups = if let Some(local_lsp_store) = local_lsp_store {
            Some(
                diagnostic_groups
                    .into_iter()
                    .filter_map(|(language_server_id, diagnostic_group)| {
                        let language_server =
                            local_lsp_store.running_language_server_for_id(language_server_id)?;

                        Some((
                            language_server.name(),
                            diagnostic_group.resolve::<usize>(&snapshot),
                        ))
                    })
                    .collect::<Vec<_>>(),
            )
        } else {
            None
        };

        cx.spawn(async move |this, cx| {
            let request_sent_at = Instant::now();

            struct BackgroundValues {
                input_events: String,
                input_excerpt: String,
                speculated_output: String,
                editable_range: Range<usize>,
                input_outline: String,
            }

            let values = cx
                .background_spawn({
                    let snapshot = snapshot.clone();
                    let path = path.clone();
                    async move {
                        let path = path.to_string_lossy();
                        let input_excerpt = excerpt_for_cursor_position(
                            cursor_point,
                            &path,
                            &snapshot,
                            MAX_REWRITE_TOKENS,
                            MAX_CONTEXT_TOKENS,
                        );
                        let input_events = prompt_for_events(&events, MAX_EVENT_TOKENS);
                        let input_outline = prompt_for_outline(&snapshot);

                        anyhow::Ok(BackgroundValues {
                            input_events,
                            input_excerpt: input_excerpt.prompt,
                            speculated_output: input_excerpt.speculated_output,
                            editable_range: input_excerpt.editable_range.to_offset(&snapshot),
                            input_outline,
                        })
                    }
                })
                .await?;

            log::debug!(
                "Events:\n{}\nExcerpt:\n{:?}",
                values.input_events,
                values.input_excerpt
            );

            let body = PredictEditsBody {
                input_events: values.input_events.clone(),
                input_excerpt: values.input_excerpt.clone(),
                speculated_output: Some(values.speculated_output),
                outline: Some(values.input_outline.clone()),
                can_collect_data,
                diagnostic_groups: diagnostic_groups.and_then(|diagnostic_groups| {
                    diagnostic_groups
                        .into_iter()
                        .map(|(name, diagnostic_group)| {
                            Ok((name.to_string(), serde_json::to_value(diagnostic_group)?))
                        })
                        .collect::<Result<Vec<_>>>()
                        .log_err()
                }),
            };

            let predict_edits_url = cx.update(|cx| ZetaSettings::get_global(cx).predict_edits_url.clone()).unwrap_or_default();

            let response = perform_predict_edits(PerformPredictEditsParams {
                client,
                llm_token,
                app_version,
                body,
                predict_edits_url,
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

                            if let Some(workspace) = workspace {
                                workspace.update(cx, |workspace, cx| {
                                    workspace.show_notification(
                                        NotificationId::unique::<ZedUpdateRequiredError>(),
                                        cx,
                                        |cx| {
                                            cx.new(|cx| {
                                                ErrorMessagePrompt::new(err.to_string(), cx)
                                                    .with_link_button(
                                                        "Update Zed",
                                                        "https://zed.dev/releases",
                                                    )
                                            })
                                        },
                                    );
                                });
                            }
                        })
                        .ok();
                    }

                    return Err(err);
                }
            };

            log::debug!("completion response: {}", &response.output_excerpt);

            if let Some(usage) = usage {
                this.update(cx, |this, _cx| {
                    this.last_usage = Some(usage);
                })
                .ok();
            }

            Self::process_completion_response(
                response,
                buffer,
                &snapshot,
                values.editable_range,
                cursor_offset,
                path,
                values.input_outline,
                values.input_events,
                values.input_excerpt,
                request_sent_at,
                &cx,
            )
            .await
        })
    }

    // Generates several example completions of various states to fill the Zeta completion modal
    #[cfg(any(test, feature = "test-support"))]
    pub fn fill_with_fake_completions(&mut self, cx: &mut Context<Self>) -> Task<()> {
        use language::Point;

        let test_buffer_text = indoc::indoc! {r#"a longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg line
            And maybe a short line

            Then a few lines

            and then another
            "#};

        let project = None;
        let buffer = cx.new(|cx| Buffer::local(test_buffer_text, cx));
        let position = buffer.read(cx).anchor_before(Point::new(1, 0));

        let completion_tasks = vec![
            self.fake_completion(
                project,
                &buffer,
                position,
                PredictEditsResponse {
                    request_id: Uuid::parse_str("e7861db5-0cea-4761-b1c5-ad083ac53a80").unwrap(),
                    output_excerpt: format!("{EDITABLE_REGION_START_MARKER}
a longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg line
[here's an edit]
And maybe a short line
Then a few lines
and then another
{EDITABLE_REGION_END_MARKER}
                        ", ),
                },
                cx,
            ),
            self.fake_completion(
                project,
                &buffer,
                position,
                PredictEditsResponse {
                    request_id: Uuid::parse_str("077c556a-2c49-44e2-bbc6-dafc09032a5e").unwrap(),
                    output_excerpt: format!(r#"{EDITABLE_REGION_START_MARKER}
a longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg line
And maybe a short line
[and another edit]
Then a few lines
and then another
{EDITABLE_REGION_END_MARKER}
                        "#),
                },
                cx,
            ),
            self.fake_completion(
                project,
                &buffer,
                position,
                PredictEditsResponse {
                    request_id: Uuid::parse_str("df8c7b23-3d1d-4f99-a306-1f6264a41277").unwrap(),
                    output_excerpt: format!(r#"{EDITABLE_REGION_START_MARKER}
a longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg line
And maybe a short line

Then a few lines

and then another
{EDITABLE_REGION_END_MARKER}
                        "#),
                },
                cx,
            ),
            self.fake_completion(
                project,
                &buffer,
                position,
                PredictEditsResponse {
                    request_id: Uuid::parse_str("c743958d-e4d8-44a8-aa5b-eb1e305c5f5c").unwrap(),
                    output_excerpt: format!(r#"{EDITABLE_REGION_START_MARKER}
a longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg line
And maybe a short line

Then a few lines

and then another
{EDITABLE_REGION_END_MARKER}
                        "#),
                },
                cx,
            ),
            self.fake_completion(
                project,
                &buffer,
                position,
                PredictEditsResponse {
                    request_id: Uuid::parse_str("ff5cd7ab-ad06-4808-986e-d3391e7b8355").unwrap(),
                    output_excerpt: format!(r#"{EDITABLE_REGION_START_MARKER}
a longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg line
And maybe a short line
Then a few lines
[a third completion]
and then another
{EDITABLE_REGION_END_MARKER}
                        "#),
                },
                cx,
            ),
            self.fake_completion(
                project,
                &buffer,
                position,
                PredictEditsResponse {
                    request_id: Uuid::parse_str("83cafa55-cdba-4b27-8474-1865ea06be94").unwrap(),
                    output_excerpt: format!(r#"{EDITABLE_REGION_START_MARKER}
a longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg line
And maybe a short line
and then another
[fourth completion example]
{EDITABLE_REGION_END_MARKER}
                        "#),
                },
                cx,
            ),
            self.fake_completion(
                project,
                &buffer,
                position,
                PredictEditsResponse {
                    request_id: Uuid::parse_str("d5bd3afd-8723-47c7-bd77-15a3a926867b").unwrap(),
                    output_excerpt: format!(r#"{EDITABLE_REGION_START_MARKER}
a longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg line
And maybe a short line
Then a few lines
and then another
[fifth and final completion]
{EDITABLE_REGION_END_MARKER}
                        "#),
                },
                cx,
            ),
        ];

        cx.spawn(async move |zeta, cx| {
            for task in completion_tasks {
                task.await.unwrap();
            }

            zeta.update(cx, |zeta, _cx| {
                zeta.shown_completions.get_mut(2).unwrap().edits = Arc::new([]);
                zeta.shown_completions.get_mut(3).unwrap().edits = Arc::new([]);
            })
            .ok();
        })
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn fake_completion(
        &mut self,
        project: Option<&Entity<Project>>,
        buffer: &Entity<Buffer>,
        position: language::Anchor,
        response: PredictEditsResponse,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<InlineCompletion>>> {
        use std::future::ready;

        self.request_completion_impl(None, project, buffer, position, false, cx, |_params| {
            ready(Ok((response, None)))
        })
    }

    pub fn request_completion(
        &mut self,
        project: Option<&Entity<Project>>,
        buffer: &Entity<Buffer>,
        position: language::Anchor,
        can_collect_data: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<InlineCompletion>>> {
        let workspace = self
            .workspace
            .as_ref()
            .and_then(|workspace| workspace.upgrade());
        self.request_completion_impl(
            workspace,
            project,
            buffer,
            position,
            can_collect_data,
            cx,
            Self::perform_predict_edits,
        )
    }

    fn perform_predict_edits(
        params: PerformPredictEditsParams,
    ) -> impl Future<Output = Result<(PredictEditsResponse, Option<EditPredictionUsage>)>> {
        async move {
            let PerformPredictEditsParams {
                client,
                llm_token,
                app_version,
                body,
                predict_edits_url,
                ..
            } = params;

            let http_client = client.http_client();
            let mut token = llm_token.acquire(&client).await?;
            let mut did_retry = false;

            loop {
                let request_builder = http_client::Request::builder().method(Method::POST);
                let request_builder =
                    if predict_edits_url != "" {
                        request_builder.uri(&predict_edits_url)
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
                    .body(serde_json::to_string(&body)?.into())?;

                let mut response = http_client.send(request).await?;

                if let Some(minimum_required_version) = response
                    .headers()
                    .get(MINIMUM_REQUIRED_VERSION_HEADER_NAME)
                    .and_then(|version| SemanticVersion::from_str(version.to_str().ok()?).ok())
                {
                    if app_version < minimum_required_version {
                        return Err(anyhow!(ZedUpdateRequiredError {
                            minimum_version: minimum_required_version
                        }));
                    }
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
                    return Err(anyhow!(
                        "error predicting edits.\nStatus: {:?}\nBody: {}",
                        response.status(),
                        body
                    ));
                }
            }
        }
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
        request_sent_at: Instant,
        cx: &AsyncApp,
    ) -> Task<Result<Option<InlineCompletion>>> {
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

            Ok(Some(InlineCompletion {
                id: InlineCompletionId(request_id),
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
                request_sent_at,
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
            &snapshot,
        ))
    }

    pub fn compute_edits(
        old_text: String,
        new_text: &str,
        offset: usize,
        snapshot: &BufferSnapshot,
    ) -> Vec<(Range<Anchor>, String)> {
        text_diff(&old_text, &new_text)
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

    pub fn is_completion_rated(&self, completion_id: InlineCompletionId) -> bool {
        self.rated_completions.contains(&completion_id)
    }

    pub fn completion_shown(&mut self, completion: &InlineCompletion, cx: &mut Context<Self>) {
        self.shown_completions.push_front(completion.clone());
        if self.shown_completions.len() > 50 {
            let completion = self.shown_completions.pop_back().unwrap();
            self.rated_completions.remove(&completion.id);
        }
        cx.notify();
    }

    pub fn rate_completion(
        &mut self,
        completion: &InlineCompletion,
        rating: InlineCompletionRating,
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

    pub fn shown_completions(&self) -> impl DoubleEndedIterator<Item = &InlineCompletion> {
        self.shown_completions.iter()
    }

    pub fn shown_completions_len(&self) -> usize {
        self.shown_completions.len()
    }

    fn report_changes_for_buffer(
        &mut self,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> BufferSnapshot {
        self.register_buffer(buffer, cx);

        let registered_buffer = self
            .registered_buffers
            .get_mut(&buffer.entity_id())
            .unwrap();
        let new_snapshot = buffer.read(cx).snapshot();

        if new_snapshot.version != registered_buffer.snapshot.version {
            let old_snapshot = mem::replace(&mut registered_buffer.snapshot, new_snapshot.clone());
            self.push_event(Event::BufferChange {
                old_snapshot,
                new_snapshot: new_snapshot.clone(),
                timestamp: Instant::now(),
            });
        }

        new_snapshot
    }

    fn load_data_collection_choices() -> DataCollectionChoice {
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
}

struct PerformPredictEditsParams {
    pub client: Arc<Client>,
    pub llm_token: LlmApiToken,
    pub app_version: SemanticVersion,
    pub body: PredictEditsBody,
    pub predict_edits_url: String,
}

#[derive(Error, Debug)]
#[error(
    "You must update to Zed version {minimum_version} or higher to continue using edit predictions."
)]
pub struct ZedUpdateRequiredError {
    minimum_version: SemanticVersion,
}

struct LicenseDetectionWatcher {
    is_open_source_rx: watch::Receiver<bool>,
    _is_open_source_task: Task<()>,
}

impl LicenseDetectionWatcher {
    pub fn new(worktree: &Worktree, cx: &mut Context<Worktree>) -> Self {
        let (mut is_open_source_tx, is_open_source_rx) = watch::channel_with::<bool>(false);

        // Check if worktree is a single file, if so we do not need to check for a LICENSE file
        let task = if worktree.abs_path().is_file() {
            Task::ready(())
        } else {
            let loaded_files = LICENSE_FILES_TO_CHECK
                .iter()
                .map(Path::new)
                .map(|file| worktree.load_file(file, cx))
                .collect::<ArrayVec<_, { LICENSE_FILES_TO_CHECK.len() }>>();

            cx.background_spawn(async move {
                for loaded_file in loaded_files.into_iter() {
                    let Ok(loaded_file) = loaded_file.await else {
                        continue;
                    };

                    let path = &loaded_file.file.path;
                    if is_license_eligible_for_data_collection(&loaded_file.text) {
                        log::info!("detected '{path:?}' as open source license");
                        *is_open_source_tx.borrow_mut() = true;
                    } else {
                        log::info!("didn't detect '{path:?}' as open source license");
                    }

                    // stop on the first license that successfully read
                    return;
                }

                log::debug!("didn't find a license file to check, assuming closed source");
            })
        };

        Self {
            is_open_source_rx,
            _is_open_source_task: task,
        }
    }

    /// Answers false until we find out it's open source
    pub fn is_project_open_source(&self) -> bool {
        *self.is_open_source_rx.borrow()
    }
}

fn common_prefix<T1: Iterator<Item = char>, T2: Iterator<Item = char>>(a: T1, b: T2) -> usize {
    a.zip(b)
        .take_while(|(a, b)| a == b)
        .map(|(a, _)| a.len_utf8())
        .sum()
}

fn prompt_for_outline(snapshot: &BufferSnapshot) -> String {
    let mut input_outline = String::new();

    writeln!(
        input_outline,
        "```{}",
        snapshot
            .file()
            .map_or(Cow::Borrowed("untitled"), |file| file
                .path()
                .to_string_lossy())
    )
    .unwrap();

    if let Some(outline) = snapshot.outline(None) {
        for item in &outline.items {
            let spacing = " ".repeat(item.depth);
            writeln!(input_outline, "{}{}", spacing, item.text).unwrap();
        }
    }

    writeln!(input_outline, "```").unwrap();

    input_outline
}

fn prompt_for_events(events: &VecDeque<Event>, mut remaining_tokens: usize) -> String {
    let mut result = String::new();
    for event in events.iter().rev() {
        let event_string = event.to_prompt();
        let event_tokens = tokens_for_bytes(event_string.len());
        if event_tokens > remaining_tokens {
            break;
        }

        if !result.is_empty() {
            result.insert_str(0, "\n\n");
        }
        result.insert_str(0, &event_string);
        remaining_tokens -= event_tokens;
    }
    result
}

struct RegisteredBuffer {
    snapshot: BufferSnapshot,
    _subscriptions: [gpui::Subscription; 2],
}

#[derive(Clone)]
enum Event {
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
                    .unwrap_or(Path::new("untitled"));
                let new_path = new_snapshot
                    .file()
                    .map(|f| f.path().as_ref())
                    .unwrap_or(Path::new("untitled"));
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
struct CurrentInlineCompletion {
    buffer_id: EntityId,
    completion: InlineCompletion,
}

impl CurrentInlineCompletion {
    fn should_replace_completion(&self, old_completion: &Self, snapshot: &BufferSnapshot) -> bool {
        if self.buffer_id != old_completion.buffer_id {
            return true;
        }

        let Some(old_edits) = old_completion.completion.interpolate(&snapshot) else {
            return true;
        };
        let Some(new_edits) = self.completion.interpolate(&snapshot) else {
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

pub struct ProviderDataCollection {
    /// When set to None, data collection is not possible in the provider buffer
    choice: Option<Entity<DataCollectionChoice>>,
    license_detection_watcher: Option<Rc<LicenseDetectionWatcher>>,
}

impl ProviderDataCollection {
    pub fn new(zeta: Entity<Zeta>, buffer: Option<Entity<Buffer>>, cx: &mut App) -> Self {
        let choice_and_watcher = buffer.and_then(|buffer| {
            let file = buffer.read(cx).file()?;

            if !file.is_local() || file.is_private() {
                return None;
            }

            let zeta = zeta.read(cx);
            let choice = zeta.data_collection_choice.clone();

            let license_detection_watcher = zeta
                .license_detection_watchers
                .get(&file.worktree_id(cx))
                .cloned()?;

            Some((choice, license_detection_watcher))
        });

        if let Some((choice, watcher)) = choice_and_watcher {
            ProviderDataCollection {
                choice: Some(choice),
                license_detection_watcher: Some(watcher),
            }
        } else {
            ProviderDataCollection {
                choice: None,
                license_detection_watcher: None,
            }
        }
    }

    pub fn can_collect_data(&self, cx: &App) -> bool {
        self.is_data_collection_enabled(cx) && self.is_project_open_source()
    }

    pub fn is_data_collection_enabled(&self, cx: &App) -> bool {
        self.choice
            .as_ref()
            .is_some_and(|choice| choice.read(cx).is_enabled())
    }

    fn is_project_open_source(&self) -> bool {
        self.license_detection_watcher
            .as_ref()
            .is_some_and(|watcher| watcher.is_project_open_source())
    }

    pub fn toggle(&mut self, cx: &mut App) {
        if let Some(choice) = self.choice.as_mut() {
            let new_choice = choice.update(cx, |choice, _cx| {
                let new_choice = choice.toggle();
                *choice = new_choice;
                new_choice
            });

            db::write_and_log(cx, move || {
                KEY_VALUE_STORE.write_kvp(
                    ZED_PREDICT_DATA_COLLECTION_CHOICE.into(),
                    new_choice.is_enabled().to_string(),
                )
            });
        }
    }
}

pub struct ZetaInlineCompletionProvider {
    zeta: Entity<Zeta>,
    pending_completions: ArrayVec<PendingCompletion, 2>,
    next_pending_completion_id: usize,
    current_completion: Option<CurrentInlineCompletion>,
    /// None if this is entirely disabled for this provider
    provider_data_collection: ProviderDataCollection,
    last_request_timestamp: Instant,
}

impl ZetaInlineCompletionProvider {
    pub const THROTTLE_TIMEOUT: Duration = Duration::from_millis(300);

    pub fn new(zeta: Entity<Zeta>, provider_data_collection: ProviderDataCollection) -> Self {
        Self {
            zeta,
            pending_completions: ArrayVec::new(),
            next_pending_completion_id: 0,
            current_completion: None,
            provider_data_collection,
            last_request_timestamp: Instant::now(),
        }
    }
}

impl inline_completion::EditPredictionProvider for ZetaInlineCompletionProvider {
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
        let is_project_open_source = self.provider_data_collection.is_project_open_source();

        if self.provider_data_collection.is_data_collection_enabled(cx) {
            DataCollectionState::Enabled {
                is_project_open_source,
            }
        } else {
            DataCollectionState::Disabled {
                is_project_open_source,
            }
        }
    }

    fn toggle_data_collection(&mut self, cx: &mut App) {
        self.provider_data_collection.toggle(cx);
    }

    fn usage(&self, cx: &App) -> Option<EditPredictionUsage> {
        self.zeta.read(cx).last_usage
    }

    fn is_enabled(
        &self,
        _buffer: &Entity<Buffer>,
        _cursor_position: language::Anchor,
        _cx: &App,
    ) -> bool {
        true
    }

    fn needs_terms_acceptance(&self, cx: &App) -> bool {
        !self.zeta.read(cx).tos_accepted
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
        if !self.zeta.read(cx).tos_accepted {
            return;
        }

        if self.zeta.read(cx).update_required {
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
        let can_collect_data = self.provider_data_collection.can_collect_data(cx);
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
                    zeta.request_completion(
                        project.as_ref(),
                        &buffer,
                        position,
                        can_collect_data,
                        cx,
                    )
                })
            });

            let completion = match completion_request {
                Ok(completion_request) => {
                    let completion_request = completion_request.await;
                    completion_request.map(|c| {
                        c.map(|completion| CurrentInlineCompletion {
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
                    if new_completion.should_replace_completion(&old_completion, &snapshot) {
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
        _direction: inline_completion::Direction,
        _cx: &mut Context<Self>,
    ) {
        // Right now we don't support cycling.
    }

    fn accept(&mut self, _cx: &mut Context<Self>) {
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
    ) -> Option<inline_completion::InlineCompletion> {
        let CurrentInlineCompletion {
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

        Some(inline_completion::InlineCompletion {
            id: Some(completion.id.to_string().into()),
            edits: edits[edit_start_ix..edit_end_ix].to_vec(),
            edit_preview: Some(completion.edit_preview.clone()),
        })
    }
}

fn tokens_for_bytes(bytes: usize) -> usize {
    /// Typical number of string bytes per token for the purposes of limiting model input. This is
    /// intentionally low to err on the side of underestimating limits.
    const BYTES_PER_TOKEN_GUESS: usize = 3;
    bytes / BYTES_PER_TOKEN_GUESS
}

#[cfg(test)]
mod tests {
    use client::test::FakeServer;
    use clock::FakeSystemClock;
    use gpui::TestAppContext;
    use http_client::FakeHttpClient;
    use indoc::indoc;
    use language::Point;
    use rpc::proto;
    use settings::SettingsStore;

    use super::*;

    #[gpui::test]
    async fn test_inline_completion_basic_interpolation(cx: &mut TestAppContext) {
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

        let completion = InlineCompletion {
            edits,
            edit_preview,
            path: Path::new("").into(),
            snapshot: cx.read(|cx| buffer.read(cx).snapshot()),
            id: InlineCompletionId(Uuid::new_v4()),
            excerpt_range: 0..0,
            cursor_offset: 0,
            input_outline: "".into(),
            input_events: "".into(),
            input_excerpt: "".into(),
            output_excerpt: "".into(),
            request_sent_at: Instant::now(),
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
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            client::init_settings(cx);
        });

        let edits = edits_for_prediction(
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
        .await;
        assert_eq!(
            edits,
            [
                (Point::new(2, 20)..Point::new(2, 20), "_1".to_string()),
                (Point::new(2, 32)..Point::new(2, 32), "_1".to_string()),
            ]
        );

        let edits = edits_for_prediction(
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
        .await;
        assert_eq!(
            edits,
            [
                (
                    Point::new(1, 26)..Point::new(1, 26),
                    " brown fox jumps over the lazy dog".to_string()
                ),
                (Point::new(1, 27)..Point::new(1, 27), ";".to_string()),
            ]
        );
    }

    #[gpui::test]
    async fn test_inline_completion_end_of_buffer(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            client::init_settings(cx);
        });

        let buffer_content = "lorem\n";
        let completion_response = indoc! {"
            ```animals.js
            <|start_of_file|>
            <|editable_region_start|>
            lorem
            ipsum
            <|editable_region_end|>
            ```"};

        let http_client = FakeHttpClient::create(move |_| async move {
            Ok(http_client::Response::builder()
                .status(200)
                .body(
                    serde_json::to_string(&PredictEditsResponse {
                        request_id: Uuid::parse_str("7e86480f-3536-4d2c-9334-8213e3445d45")
                            .unwrap(),
                        output_excerpt: completion_response.to_string(),
                    })
                    .unwrap()
                    .into(),
                )
                .unwrap())
        });

        let client = cx.update(|cx| Client::new(Arc::new(FakeSystemClock::new()), http_client, cx));
        cx.update(|cx| {
            RefreshLlmTokenListener::register(client.clone(), cx);
        });
        let server = FakeServer::for_client(42, &client, cx).await;
        let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
        let zeta = cx.new(|cx| Zeta::new(None, client, user_store, cx));

        let buffer = cx.new(|cx| Buffer::local(buffer_content, cx));
        let cursor = buffer.read_with(cx, |buffer, _| buffer.anchor_before(Point::new(1, 0)));
        let completion_task = zeta.update(cx, |zeta, cx| {
            zeta.request_completion(None, &buffer, cursor, false, cx)
        });

        let token_request = server.receive::<proto::GetLlmToken>().await.unwrap();
        server.respond(
            token_request.receipt(),
            proto::GetLlmTokenResponse { token: "".into() },
        );

        let completion = completion_task.await.unwrap().unwrap();
        buffer.update(cx, |buffer, cx| {
            buffer.edit(completion.edits.iter().cloned(), None, cx)
        });
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.text()),
            "lorem\nipsum"
        );
    }

    async fn edits_for_prediction(
        buffer_content: &str,
        completion_response: &str,
        cx: &mut TestAppContext,
    ) -> Vec<(Range<Point>, String)> {
        let completion_response = completion_response.to_string();
        let http_client = FakeHttpClient::create(move |_| {
            let completion = completion_response.clone();
            async move {
                Ok(http_client::Response::builder()
                    .status(200)
                    .body(
                        serde_json::to_string(&PredictEditsResponse {
                            request_id: Uuid::new_v4(),
                            output_excerpt: completion,
                        })
                        .unwrap()
                        .into(),
                    )
                    .unwrap())
            }
        });

        let client = cx.update(|cx| Client::new(Arc::new(FakeSystemClock::new()), http_client, cx));
        cx.update(|cx| {
            RefreshLlmTokenListener::register(client.clone(), cx);
        });
        let server = FakeServer::for_client(42, &client, cx).await;
        let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
        let zeta = cx.new(|cx| Zeta::new(None, client, user_store, cx));

        let buffer = cx.new(|cx| Buffer::local(buffer_content, cx));
        let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());
        let cursor = buffer.read_with(cx, |buffer, _| buffer.anchor_before(Point::new(1, 0)));
        let completion_task = zeta.update(cx, |zeta, cx| {
            zeta.request_completion(None, &buffer, cursor, false, cx)
        });

        let token_request = server.receive::<proto::GetLlmToken>().await.unwrap();
        server.respond(
            token_request.receipt(),
            proto::GetLlmTokenResponse { token: "".into() },
        );

        let completion = completion_task.await.unwrap().unwrap();
        completion
            .edits
            .into_iter()
            .map(|(old_range, new_text)| (old_range.to_point(&snapshot), new_text.clone()))
            .collect::<Vec<_>>()
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
        if std::env::var("RUST_LOG").is_ok() {
            env_logger::init();
        }
    }
}
