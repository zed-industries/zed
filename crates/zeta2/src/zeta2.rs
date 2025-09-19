use anyhow::{Context as _, Result};
use arrayvec::ArrayVec;
use client::{Client, EditPredictionUsage, UserStore};
use cloud_llm_client::predict_edits_v3::{self, Signature};
use cloud_llm_client::{
    EXPIRED_LLM_TOKEN_HEADER_NAME, MINIMUM_REQUIRED_VERSION_HEADER_NAME, ZED_VERSION_HEADER_NAME,
};
use edit_prediction::{DataCollectionState, Direction, EditPrediction, EditPredictionProvider};
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
use language::BufferSnapshot;
use language::{Anchor, Buffer, OffsetRangeExt as _, ToPoint};
use language_model::{LlmApiToken, RefreshLlmTokenListener};
use project::Project;
use release_channel::AppVersion;
use std::collections::HashMap;
use std::str::FromStr as _;
use std::time::{Duration, Instant};
use std::{ops::Range, sync::Arc};
use thiserror::Error;
use util::ResultExt as _;
use workspace::notifications::{ErrorMessagePrompt, NotificationId, show_app_notification};

#[derive(Clone)]
struct ZetaGlobal(Entity<Zeta>);

impl Global for ZetaGlobal {}

pub struct Zeta {
    client: Arc<Client>,
    user_store: Entity<UserStore>,
    llm_token: LlmApiToken,
    _llm_token_subscription: Subscription,
    projects: HashMap<EntityId, RegisteredProject>,
    excerpt_options: EditPredictionExcerptOptions,
    update_required: bool,
}

struct RegisteredProject {
    syntax_index: Entity<SyntaxIndex>,
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
        self.projects
            .entry(project.entity_id())
            .or_insert_with(|| RegisteredProject {
                syntax_index: cx.new(|cx| SyntaxIndex::new(project, cx)),
            });
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
        let client = self.client.clone();
        let llm_token = self.llm_token.clone();
        let app_version = AppVersion::global(cx);

        let request_task = cx.background_spawn({
            let snapshot = snapshot.clone();
            async move {
                let index_state = if let Some(index_state) = index_state {
                    Some(index_state.lock_owned().await)
                } else {
                    None
                };

                let cursor_point = position.to_point(&snapshot);

                let Some(request) = EditPredictionContext::gather_context(
                    cursor_point,
                    &snapshot,
                    &excerpt_options,
                    index_state.as_deref(),
                )
                .map(|context| {
                    make_cloud_request(
                        context,
                        // TODO pass everything
                        Vec::new(),
                        false,
                        Vec::new(),
                        None,
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

                    let Some((edits, edit_preview_task)) = buffer.read_with(cx, |buffer, cx| {
                        let new_snapshot = buffer.snapshot();
                        let edits: Arc<[_]> = interpolate(&snapshot, &new_snapshot, edits)?.into();
                        Some((edits.clone().to_vec(), buffer.preview_edits(edits, cx)))
                    })?
                    else {
                        return Ok(None);
                    };

                    Ok(Some(EditPrediction {
                        // todo!
                        id: None,
                        edits,
                        edit_preview: Some(edit_preview_task.await),
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
                            .build_zed_llm_url("/predict_edits/v2", &[])?
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
    fn should_replace_prediction(
        &self,
        _old_completion: &Self,
        _snapshot: &BufferSnapshot,
    ) -> bool {
        true
        // TODO
        // if self.buffer_id != old_completion.buffer_id {
        //     return true;
        // }

        // let Some(old_edits) = old_completion.completion.interpolate(snapshot) else {
        //     return true;
        // };
        // let Some(new_edits) = self.completion.interpolate(snapshot) else {
        //     return false;
        // };

        // if old_edits.len() == 1 && new_edits.len() == 1 {
        //     let (old_range, old_text) = &old_edits[0];
        //     let (new_range, new_text) = &new_edits[0];
        //     new_range == old_range && new_text.starts_with(old_text)
        // } else {
        //     true
        // }
    }
}

struct PendingPrediction {
    id: usize,
    _task: Task<()>,
}

impl EditPredictionProvider for ZetaEditPredictionProvider {
    fn name() -> &'static str {
        // TODO [zeta2]
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

        // TODO [zeta2] check account
        // if self
        //     .zeta
        //     .read(cx)
        //     .user_store
        //     .read_with(cx, |user_store, _cx| {
        //         user_store.account_too_young() || user_store.has_overdue_invoices()
        //     })
        // {
        //     return;
        // }

        // TODO [zeta2] try to interpolate current request

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
        _cursor_position: language::Anchor,
        _cx: &mut Context<Self>,
    ) -> Option<EditPrediction> {
        let current_prediction = self.current_prediction.take()?;

        if current_prediction.buffer_id != buffer.entity_id() {
            return None;
        }

        // TODO [zeta2] interpolate

        Some(current_prediction.prediction)
    }
}

fn make_cloud_request(
    context: EditPredictionContext,
    events: Vec<predict_edits_v3::Event>,
    can_collect_data: bool,
    diagnostic_groups: Vec<predict_edits_v3::DiagnosticGroup>,
    git_info: Option<cloud_llm_client::PredictEditsGitInfo>,
    index_state: Option<&SyntaxIndexState>,
) -> predict_edits_v3::PredictEditsRequest {
    let mut signatures = Vec::new();
    let mut declaration_to_signature_index = HashMap::default();
    let mut referenced_declarations = Vec::new();

    for snippet in context.snippets {
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
            text: text.into(),
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
        excerpt: context.excerpt_text.body,
        referenced_declarations,
        signatures,
        excerpt_parent,
        // todo!
        events,
        can_collect_data,
        diagnostic_groups,
        git_info,
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
