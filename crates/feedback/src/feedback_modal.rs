use std::{ops::RangeInclusive, sync::Arc, time::Duration};

use anyhow::{anyhow, bail};
use bitflags::bitflags;
use client::Client;
use db::kvp::KEY_VALUE_STORE;
use editor::{Editor, EditorEvent};
use futures::AsyncReadExt;
use gpui::{
    div, rems, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model,
    PromptLevel, Render, Task, View, ViewContext,
};
use http::HttpClient;
use isahc::Request;
use language::Buffer;
use project::Project;
use regex::Regex;
use serde_derive::Serialize;
use ui::{prelude::*, Button, ButtonStyle, IconPosition, Tooltip};
use util::ResultExt;
use workspace::notifications::NotificationId;
use workspace::{DismissDecision, ModalView, Toast, Workspace};

use crate::{system_specs::SystemSpecs, GiveFeedback, OpenZedRepo};

// For UI testing purposes
const SEND_SUCCESS_IN_DEV_MODE: bool = true;
const SEND_TIME_IN_DEV_MODE: Duration = Duration::from_secs(2);

// Temporary, until tests are in place
#[cfg(debug_assertions)]
const DEV_MODE: bool = true;

#[cfg(not(debug_assertions))]
const DEV_MODE: bool = false;

const DATABASE_KEY_NAME: &str = "email_address";
const EMAIL_REGEX: &str = r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b";
const FEEDBACK_CHAR_LIMIT: RangeInclusive<i32> = 10..=5000;
const FEEDBACK_SUBMISSION_ERROR_TEXT: &str =
    "Feedback failed to submit, see error log for details.";

#[derive(Serialize)]
struct FeedbackRequestBody<'a> {
    feedback_text: &'a str,
    email: Option<String>,
    metrics_id: Option<Arc<str>>,
    installation_id: Option<Arc<str>>,
    system_specs: SystemSpecs,
    is_staff: bool,
}

bitflags! {
    #[derive(Debug, Clone, PartialEq)]
    struct InvalidStateFlags: u8 {
        const EmailAddress = 0b00000001;
        const CharacterCount = 0b00000010;
    }
}

#[derive(Debug, Clone, PartialEq)]
enum CannotSubmitReason {
    InvalidState { flags: InvalidStateFlags },
    AwaitingSubmission,
}

#[derive(Debug, Clone, PartialEq)]
enum SubmissionState {
    CanSubmit,
    CannotSubmit { reason: CannotSubmitReason },
}

pub struct FeedbackModal {
    system_specs: SystemSpecs,
    feedback_editor: View<Editor>,
    email_address_editor: View<Editor>,
    submission_state: Option<SubmissionState>,
    dismiss_modal: bool,
    character_count: i32,
}

impl FocusableView for FeedbackModal {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.feedback_editor.focus_handle(cx)
    }
}
impl EventEmitter<DismissEvent> for FeedbackModal {}

impl ModalView for FeedbackModal {
    fn on_before_dismiss(&mut self, cx: &mut ViewContext<Self>) -> DismissDecision {
        self.update_email_in_store(cx);

        if self.dismiss_modal {
            return DismissDecision::Dismiss(true);
        }

        let has_feedback = self.feedback_editor.read(cx).text_option(cx).is_some();
        if !has_feedback {
            return DismissDecision::Dismiss(true);
        }

        let answer = cx.prompt(PromptLevel::Info, "Discard feedback?", None, &["Yes", "No"]);

        cx.spawn(move |this, mut cx| async move {
            if answer.await.ok() == Some(0) {
                this.update(&mut cx, |this, cx| {
                    this.dismiss_modal = true;
                    cx.emit(DismissEvent)
                })
                .log_err();
            }
        })
        .detach();

        DismissDecision::Pending
    }
}

impl FeedbackModal {
    pub fn register(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        let _handle = cx.view().downgrade();
        workspace.register_action(move |workspace, _: &GiveFeedback, cx| {
            let markdown = workspace
                .app_state()
                .languages
                .language_for_name("Markdown");

            let project = workspace.project().clone();
            let is_local_project = project.read(cx).is_local();

            if !is_local_project {
                struct FeedbackInRemoteProject;

                workspace.show_toast(
                    Toast::new(
                        NotificationId::unique::<FeedbackInRemoteProject>(),
                        "You can only submit feedback in your own project.",
                    ),
                    cx,
                );
                return;
            }

            cx.spawn(|workspace, mut cx| async move {
                let markdown = markdown.await.log_err();
                let buffer = project.update(&mut cx, |project, cx| {
                    project.create_local_buffer("", markdown, cx)
                })?;

                workspace.update(&mut cx, |workspace, cx| {
                    let system_specs = SystemSpecs::new(cx);

                    workspace.toggle_modal(cx, move |cx| {
                        FeedbackModal::new(system_specs, project, buffer, cx)
                    });
                })?;

                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        });
    }

    pub fn new(
        system_specs: SystemSpecs,
        project: Model<Project>,
        buffer: Model<Buffer>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let email_address_editor = cx.new_view(|cx| {
            let mut editor = Editor::single_line(cx);
            editor.set_placeholder_text("Email address (optional)", cx);

            if let Ok(Some(email_address)) = KEY_VALUE_STORE.read_kvp(DATABASE_KEY_NAME) {
                editor.set_text(email_address, cx)
            }

            editor
        });

        let feedback_editor = cx.new_view(|cx| {
            let mut editor = Editor::for_buffer(buffer, Some(project.clone()), cx);
            editor.set_placeholder_text(
                "You can use markdown to organize your feedback with code and links.",
                cx,
            );
            editor.set_show_gutter(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_show_inline_completions(false);
            editor.set_vertical_scroll_margin(5, cx);
            editor.set_use_modal_editing(false);
            editor
        });

        cx.subscribe(&feedback_editor, |this, editor, event: &EditorEvent, cx| {
            if *event == EditorEvent::Edited {
                this.character_count = editor
                    .read(cx)
                    .buffer()
                    .read(cx)
                    .as_singleton()
                    .expect("Feedback editor is never a multi-buffer")
                    .read(cx)
                    .len() as i32;
                cx.notify();
            }
        })
        .detach();

        Self {
            system_specs: system_specs.clone(),
            feedback_editor,
            email_address_editor,
            submission_state: None,
            dismiss_modal: false,
            character_count: 0,
        }
    }

    pub fn submit(&mut self, cx: &mut ViewContext<Self>) -> Task<anyhow::Result<()>> {
        let feedback_text = self.feedback_editor.read(cx).text(cx).trim().to_string();
        let email = self.email_address_editor.read(cx).text_option(cx);

        let answer = cx.prompt(
            PromptLevel::Info,
            "Ready to submit your feedback?",
            None,
            &["Yes, Submit!", "No"],
        );
        let client = Client::global(cx).clone();
        let specs = self.system_specs.clone();
        cx.spawn(|this, mut cx| async move {
            let answer = answer.await.ok();
            if answer == Some(0) {
                this.update(&mut cx, |this, cx| {
                    this.submission_state = Some(SubmissionState::CannotSubmit {
                        reason: CannotSubmitReason::AwaitingSubmission,
                    });
                    cx.notify();
                })
                .log_err();

                let res =
                    FeedbackModal::submit_feedback(&feedback_text, email, client, specs).await;

                match res {
                    Ok(_) => {
                        this.update(&mut cx, |this, cx| {
                            this.dismiss_modal = true;
                            cx.notify();
                            cx.emit(DismissEvent)
                        })
                        .ok();
                    }
                    Err(error) => {
                        log::error!("{}", error);
                        this.update(&mut cx, |this, cx| {
                            let prompt = cx.prompt(
                                PromptLevel::Critical,
                                FEEDBACK_SUBMISSION_ERROR_TEXT,
                                None,
                                &["OK"],
                            );
                            cx.spawn(|_, _cx| async move {
                                prompt.await.ok();
                            })
                            .detach();

                            this.submission_state = Some(SubmissionState::CanSubmit);
                            cx.notify();
                        })
                        .log_err();
                    }
                }
            }
        })
        .detach();

        Task::ready(Ok(()))
    }

    async fn submit_feedback(
        feedback_text: &str,
        email: Option<String>,
        zed_client: Arc<Client>,
        system_specs: SystemSpecs,
    ) -> anyhow::Result<()> {
        if DEV_MODE {
            smol::Timer::after(SEND_TIME_IN_DEV_MODE).await;

            if SEND_SUCCESS_IN_DEV_MODE {
                return Ok(());
            } else {
                return Err(anyhow!("Error submitting feedback"));
            }
        }

        let telemetry = zed_client.telemetry();
        let metrics_id = telemetry.metrics_id();
        let installation_id = telemetry.installation_id();
        let is_staff = telemetry.is_staff();
        let http_client = zed_client.http_client();
        let feedback_endpoint = http_client.build_url("/api/feedback");
        let request = FeedbackRequestBody {
            feedback_text: &feedback_text,
            email,
            metrics_id,
            installation_id,
            system_specs,
            is_staff: is_staff.unwrap_or(false),
        };
        let json_bytes = serde_json::to_vec(&request)?;
        let request = Request::post(feedback_endpoint)
            .header("content-type", "application/json")
            .body(json_bytes.into())?;
        let mut response = http_client.send(request).await?;
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        let response_status = response.status();
        if !response_status.is_success() {
            bail!("Feedback API failed with error: {}", response_status)
        }
        Ok(())
    }

    fn update_submission_state(&mut self, cx: &mut ViewContext<Self>) {
        if self.awaiting_submission() {
            return;
        }

        let mut invalid_state_flags = InvalidStateFlags::empty();

        let valid_email_address = match self.email_address_editor.read(cx).text_option(cx) {
            Some(email_address) => Regex::new(EMAIL_REGEX).unwrap().is_match(&email_address),
            None => true,
        };

        if !valid_email_address {
            invalid_state_flags |= InvalidStateFlags::EmailAddress;
        }

        if !FEEDBACK_CHAR_LIMIT.contains(&self.character_count) {
            invalid_state_flags |= InvalidStateFlags::CharacterCount;
        }

        if invalid_state_flags.is_empty() {
            self.submission_state = Some(SubmissionState::CanSubmit);
        } else {
            self.submission_state = Some(SubmissionState::CannotSubmit {
                reason: CannotSubmitReason::InvalidState {
                    flags: invalid_state_flags,
                },
            });
        }
    }

    fn update_email_in_store(&self, cx: &mut ViewContext<Self>) {
        let email = self.email_address_editor.read(cx).text_option(cx);

        cx.spawn(|_, _| async move {
            match email {
                Some(email) => {
                    KEY_VALUE_STORE
                        .write_kvp(DATABASE_KEY_NAME.to_string(), email)
                        .await
                        .ok();
                }
                None => {
                    KEY_VALUE_STORE
                        .delete_kvp(DATABASE_KEY_NAME.to_string())
                        .await
                        .ok();
                }
            }
        })
        .detach();
    }

    fn valid_email_address(&self) -> bool {
        !self.in_invalid_state(InvalidStateFlags::EmailAddress)
    }

    fn valid_character_count(&self) -> bool {
        !self.in_invalid_state(InvalidStateFlags::CharacterCount)
    }

    fn in_invalid_state(&self, flag: InvalidStateFlags) -> bool {
        match self.submission_state {
            Some(SubmissionState::CannotSubmit {
                reason: CannotSubmitReason::InvalidState { ref flags },
            }) => flags.contains(flag),
            _ => false,
        }
    }

    fn awaiting_submission(&self) -> bool {
        matches!(
            self.submission_state,
            Some(SubmissionState::CannotSubmit {
                reason: CannotSubmitReason::AwaitingSubmission
            })
        )
    }

    fn can_submit(&self) -> bool {
        matches!(self.submission_state, Some(SubmissionState::CanSubmit))
    }

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent)
    }
}

impl Render for FeedbackModal {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.update_submission_state(cx);

        let submit_button_text = if self.awaiting_submission() {
            "Submitting..."
        } else {
            "Submit"
        };

        let open_zed_repo = cx.listener(|_, _, cx| cx.dispatch_action(Box::new(OpenZedRepo)));

        v_flex()
            .elevation_3(cx)
            .key_context("GiveFeedback")
            .on_action(cx.listener(Self::cancel))
            .min_w(rems(40.))
            .max_w(rems(96.))
            .h(rems(32.))
            .p_4()
            .gap_2()
            .child(Headline::new("Give Feedback"))
            .child(
                Label::new(if self.character_count < *FEEDBACK_CHAR_LIMIT.start() {
                    format!(
                        "Feedback must be at least {} characters.",
                        FEEDBACK_CHAR_LIMIT.start()
                    )
                } else {
                    format!(
                        "Characters: {}",
                        *FEEDBACK_CHAR_LIMIT.end() - self.character_count
                    )
                })
                .color(if self.valid_character_count() {
                    Color::Success
                } else {
                    Color::Error
                }),
            )
            .child(
                div()
                    .flex_1()
                    .bg(cx.theme().colors().editor_background)
                    .p_2()
                    .border_1()
                    .rounded_md()
                    .border_color(cx.theme().colors().border)
                    .child(self.feedback_editor.clone()),
            )
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        h_flex()
                            .bg(cx.theme().colors().editor_background)
                            .p_2()
                            .border_1()
                            .rounded_md()
                            .border_color(if self.valid_email_address() {
                                cx.theme().colors().border
                            } else {
                                cx.theme().status().error_border
                            })
                            .child(self.email_address_editor.clone()),
                    )
                    .child(
                        Label::new("Provide an email address if you want us to be able to reply.")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .child(
                h_flex()
                    .justify_between()
                    .gap_1()
                    .child(
                        Button::new("zed_repository", "Zed Repository")
                            .style(ButtonStyle::Transparent)
                            .icon(IconName::ExternalLink)
                            .icon_position(IconPosition::End)
                            .icon_size(IconSize::Small)
                            .on_click(open_zed_repo),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .child(
                                Button::new("cancel_feedback", "Cancel")
                                    .style(ButtonStyle::Subtle)
                                    .color(Color::Muted)
                                    .on_click(cx.listener(move |_, _, cx| {
                                        cx.spawn(|this, mut cx| async move {
                                            this.update(&mut cx, |_, cx| cx.emit(DismissEvent))
                                                .ok();
                                        })
                                        .detach();
                                    })),
                            )
                            .child(
                                Button::new("submit_feedback", submit_button_text)
                                    .color(Color::Accent)
                                    .style(ButtonStyle::Filled)
                                    .on_click(cx.listener(|this, _, cx| {
                                        this.submit(cx).detach();
                                    }))
                                    .tooltip(move |cx| {
                                        Tooltip::text("Submit feedback to the Zed team.", cx)
                                    })
                                    .when(!self.can_submit(), |this| this.disabled(true)),
                            ),
                    ),
            )
    }
}

// TODO: Testing of various button states, dismissal prompts, etc. :)
