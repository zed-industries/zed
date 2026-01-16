use crate::prediction::EditPredictionResult;
use crate::{
    CurrentEditPrediction, DebugEvent, EDIT_PREDICTIONS_MODEL_ID, EditPredictionFinishedDebugEvent,
    EditPredictionId, EditPredictionModel2, EditPredictionModelInput,
    EditPredictionStartedDebugEvent, EditPredictionStore, ZedUpdateRequiredError,
};
use anyhow::{Result, anyhow};
use client::{Client, EditPredictionUsage, UserStore};
use cloud_llm_client::predict_edits_v3::RawCompletionRequest;
use cloud_llm_client::{
    AcceptEditPredictionBody, EditPredictionRejectReason, EditPredictionRejection,
};
use futures::channel::mpsc;
use gpui::{App, Entity, SharedString, Task, http_client::Url, prelude::*};
use language::{OffsetRangeExt as _, ToOffset as _, ToPoint};
use language_model::LlmApiToken;
use release_channel::AppVersion;
use workspace::notifications::{ErrorMessagePrompt, NotificationId, show_app_notification};

use std::env;
use std::{path::Path, sync::Arc, time::Instant};
use zeta_prompt::format_zeta_prompt;
use zeta_prompt::{CURSOR_MARKER, ZetaVersion};

pub const MAX_CONTEXT_TOKENS: usize = 350;

pub fn max_editable_tokens(version: ZetaVersion) -> usize {
    match version {
        ZetaVersion::V0112MiddleAtEnd | ZetaVersion::V0113Ordered => 150,
        ZetaVersion::V0114180EditableRegion => 180,
    }
}

pub struct Zeta2Model {
    client: Arc<Client>,
    llm_token: LlmApiToken,
    user_store: Entity<UserStore>,
    custom_predict_edits_url: Option<Arc<Url>>,
    reject_predictions_tx: mpsc::UnboundedSender<EditPredictionRejection>,
    version: ZetaVersion,
}

impl Zeta2Model {
    pub fn new(
        client: Arc<Client>,
        llm_token: LlmApiToken,
        user_store: Entity<UserStore>,
        custom_predict_edits_url: Option<Arc<Url>>,
        reject_predictions_tx: mpsc::UnboundedSender<EditPredictionRejection>,
        version: ZetaVersion,
    ) -> Self {
        Self {
            client,
            llm_token,
            user_store,
            custom_predict_edits_url,
            reject_predictions_tx,
            version,
        }
    }
}

impl EditPredictionModel2 for Zeta2Model {
    fn requires_context(&self) -> bool {
        true
    }

    fn requires_edit_history(&self) -> bool {
        true
    }

    fn is_enabled(&self, _cx: &App) -> bool {
        true
    }

    fn usage(&self, cx: &App) -> Option<EditPredictionUsage> {
        self.user_store.read(cx).edit_prediction_usage()
    }

    fn request_prediction(
        &self,
        inputs: EditPredictionModelInput,
        cx: &mut App,
    ) -> Task<Result<Option<EditPredictionResult>>> {
        let EditPredictionModelInput {
            buffer,
            snapshot,
            position,
            related_files,
            events,
            debug_tx,
            ..
        } = inputs;
        let zeta_version = self.version;
        let client = self.client.clone();
        let llm_token = self.llm_token.clone();
        let user_store = self.user_store.clone();
        let custom_url = self.custom_predict_edits_url.clone();
        let buffer_snapshotted_at = Instant::now();

        let Some(excerpt_path) = snapshot
            .file()
            .map(|file| -> Arc<Path> { file.full_path(cx).into() })
        else {
            return Task::ready(Err(anyhow!("No file path for excerpt")));
        };

        let app_version = AppVersion::global(cx);

        let request_task = cx.background_spawn({
            async move {
                let cursor_offset = position.to_offset(&snapshot);
                let (editable_offset_range, prompt_input) = zeta2_prompt_input(
                    &snapshot,
                    related_files,
                    events,
                    excerpt_path,
                    cursor_offset,
                    zeta_version,
                );

                if let Some(debug_tx) = &debug_tx {
                    let prompt = format_zeta_prompt(&prompt_input, zeta_version);
                    debug_tx
                        .unbounded_send(DebugEvent::EditPredictionStarted(
                            EditPredictionStartedDebugEvent {
                                buffer: buffer.downgrade(),
                                prompt: Some(prompt),
                                position,
                            },
                        ))
                        .ok();
                }

                log::trace!("Sending edit prediction request");

                let (request_id, output_text, usage) = if let Some(custom_url) = custom_url {
                    let prompt = format_zeta_prompt(&prompt_input, zeta_version);
                    let request = RawCompletionRequest {
                        model: EDIT_PREDICTIONS_MODEL_ID.clone().unwrap_or_default(),
                        prompt,
                        temperature: None,
                        stop: vec![],
                        max_tokens: Some(2048),
                    };

                    let (mut response, usage) = EditPredictionStore::send_raw_llm_request(
                        request,
                        client,
                        Some(custom_url),
                        llm_token,
                        app_version,
                    )
                    .await?;

                    let request_id = EditPredictionId(response.id.clone().into());
                    let output_text = response.choices.pop().map(|choice| choice.text);
                    (request_id, output_text, usage)
                } else {
                    let (response, usage) = EditPredictionStore::send_v3_request(
                        prompt_input.clone(),
                        zeta_version,
                        client,
                        llm_token,
                        app_version,
                    )
                    .await?;

                    let request_id = EditPredictionId(response.request_id.into());
                    let output_text = if response.output.is_empty() {
                        None
                    } else {
                        Some(response.output)
                    };
                    (request_id, output_text, usage)
                };

                let received_response_at = Instant::now();

                log::trace!("Got edit prediction response");

                let Some(mut output_text) = output_text else {
                    return Ok((Some((request_id, None)), usage));
                };

                if let Some(debug_tx) = &debug_tx {
                    debug_tx
                        .unbounded_send(DebugEvent::EditPredictionFinished(
                            EditPredictionFinishedDebugEvent {
                                buffer: buffer.downgrade(),
                                position,
                                model_output: Some(output_text.clone()),
                            },
                        ))
                        .ok();
                }

                if output_text.contains(CURSOR_MARKER) {
                    log::trace!("Stripping out {CURSOR_MARKER} from response");
                    output_text = output_text.replace(CURSOR_MARKER, "");
                }

                let mut old_text = snapshot
                    .text_for_range(editable_offset_range.clone())
                    .collect::<String>();

                if !output_text.is_empty() && !output_text.ends_with('\n') {
                    output_text.push('\n');
                }
                if !old_text.is_empty() && !old_text.ends_with('\n') {
                    old_text.push('\n');
                }

                let edits: Vec<_> = language::text_diff(&old_text, &output_text)
                    .into_iter()
                    .map(|(range, text)| {
                        (
                            snapshot.anchor_after(editable_offset_range.start + range.start)
                                ..snapshot.anchor_before(editable_offset_range.start + range.end),
                            text,
                        )
                    })
                    .collect();

                anyhow::Ok((
                    Some((
                        request_id,
                        Some((
                            prompt_input,
                            buffer,
                            snapshot.clone(),
                            edits,
                            received_response_at,
                        )),
                    )),
                    usage,
                ))
            }
        });

        cx.spawn({
            async move |cx| {
                let result = request_task.await;

                let data = match result {
                    Ok((data, usage)) => {
                        if let Some(usage) = usage {
                            user_store.update(cx, |user_store, cx| {
                                user_store.update_edit_prediction_usage(usage, cx);
                            });
                        }
                        data
                    }
                    Err(err) => {
                        if err.is::<ZedUpdateRequiredError>() {
                            cx.update(|cx| {
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
                            });
                        }
                        return Err(err);
                    }
                };

                let Some((id, prediction)) = data else {
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

    fn report_rejected_prediction(&self, rejection: EditPredictionRejection) {
        self.reject_predictions_tx.unbounded_send(rejection).ok();
    }

    fn report_accepted_prediction(&self, prediction: CurrentEditPrediction, cx: &mut App) {
        let custom_predict_edits_url = self.custom_predict_edits_url.as_ref();
        let custom_accept_url = env::var("ZED_ACCEPT_PREDICTION_URL").ok();
        if custom_predict_edits_url.is_some() && custom_accept_url.is_none() {
            return;
        }

        let request_id = prediction.prediction.id.to_string();
        let require_auth = custom_accept_url.is_none();
        let client = self.client.clone();
        let llm_token = self.llm_token.clone();
        let app_version = AppVersion::global(cx);

        cx.background_spawn(async move {
            let url = if let Some(accept_edits_url) = custom_accept_url {
                Url::parse(&accept_edits_url)?
            } else {
                client
                    .http_client()
                    .build_zed_llm_url("/predict_edits/accept", &[])?
            };

            let response = EditPredictionStore::send_api_request::<()>(
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
                require_auth,
            )
            .await;

            response?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }
}

pub fn zeta2_prompt_input(
    snapshot: &language::BufferSnapshot,
    related_files: Vec<zeta_prompt::RelatedFile>,
    events: Vec<Arc<zeta_prompt::Event>>,
    excerpt_path: Arc<Path>,
    cursor_offset: usize,
    zeta_version: ZetaVersion,
) -> (std::ops::Range<usize>, zeta_prompt::ZetaPromptInput) {
    let cursor_point = cursor_offset.to_point(snapshot);

    let (editable_range, context_range) =
        crate::cursor_excerpt::editable_and_context_ranges_for_cursor_position(
            cursor_point,
            snapshot,
            max_editable_tokens(zeta_version),
            MAX_CONTEXT_TOKENS,
        );

    let related_files = crate::filter_redundant_excerpts(
        related_files,
        excerpt_path.as_ref(),
        context_range.start.row..context_range.end.row,
    );

    let context_start_offset = context_range.start.to_offset(snapshot);
    let editable_offset_range = editable_range.to_offset(snapshot);
    let cursor_offset_in_excerpt = cursor_offset - context_start_offset;
    let editable_range_in_excerpt = (editable_offset_range.start - context_start_offset)
        ..(editable_offset_range.end - context_start_offset);

    let prompt_input = zeta_prompt::ZetaPromptInput {
        cursor_path: excerpt_path,
        cursor_excerpt: snapshot
            .text_for_range(context_range)
            .collect::<String>()
            .into(),
        editable_range_in_excerpt,
        cursor_offset_in_excerpt,
        events,
        related_files,
    };
    (editable_offset_range, prompt_input)
}

#[allow(dead_code)]
pub(crate) fn edit_prediction_accepted(
    store: &EditPredictionStore,
    current_prediction: CurrentEditPrediction,
    cx: &App,
) {
    edit_prediction_accepted_impl(
        &store.client,
        &store.llm_token,
        store.custom_predict_edits_url.as_ref(),
        current_prediction,
        cx,
    );
}

fn edit_prediction_accepted_impl(
    client: &Arc<Client>,
    llm_token: &LlmApiToken,
    custom_predict_edits_url: Option<&Arc<Url>>,
    current_prediction: CurrentEditPrediction,
    cx: &App,
) {
    let custom_accept_url = env::var("ZED_ACCEPT_PREDICTION_URL").ok();
    if custom_predict_edits_url.is_some() && custom_accept_url.is_none() {
        return;
    }

    let request_id = current_prediction.prediction.id.to_string();
    let require_auth = custom_accept_url.is_none();
    let client = client.clone();
    let llm_token = llm_token.clone();
    let app_version = AppVersion::global(cx);

    cx.background_spawn(async move {
        let url = if let Some(accept_edits_url) = custom_accept_url {
            Url::parse(&accept_edits_url)?
        } else {
            client
                .http_client()
                .build_zed_llm_url("/predict_edits/accept", &[])?
        };

        let response = EditPredictionStore::send_api_request::<()>(
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
            require_auth,
        )
        .await;

        response?;
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}
