use anyhow::Result;
use editor::Editor;
use gpui::{
    actions, div, px, rems, App, Context, DismissEvent, Entity, EventEmitter,
    FocusHandle, Focusable, InteractiveElement, IntoElement, ParentElement, Render,
    SharedString, StatefulInteractiveElement, Styled, Task, VisualContext,
    WeakEntity, Window,
};
use pull_requests::{CreatePullRequest, PullRequestManager};
use settings::Settings;
use theme::ActiveTheme;
use ui::{
    h_flex, prelude::*, v_flex, Button, ButtonStyle, Checkbox, Color, FluentBuilder,
    Label, LabelCommon, LabelSize, Modal, ModalHeader,
};
use workspace::Workspace;

actions!(
    create_pr,
    [SubmitPullRequest, CancelCreatePr, ToggleDraft, SelectBaseBranch]
);

pub struct CreatePrModal {
    manager: Entity<PullRequestManager>,
    title_editor: View<Editor>,
    description_editor: View<Editor>,
    base_branch: String,
    head_branch: String,
    is_draft: bool,
    is_submitting: bool,
    error_message: Option<String>,
    focus_handle: FocusHandle,
}

impl CreatePrModal {
    pub fn new(
        manager: Entity<PullRequestManager>,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        
        // Create editors for title and description
        let title_editor = cx.new_view(|cx| {
            let mut editor = Editor::single_line(cx);
            editor.set_placeholder_text("Pull request title", cx);
            editor
        });
        
        let description_editor = cx.new_view(|cx| {
            let mut editor = Editor::multi_line(cx);
            editor.set_placeholder_text(
                "Describe your changes\n\n## What does this PR do?\n\n## Screenshots (if applicable)",
                cx,
            );
            editor
        });

        // Get current branch info
        let (base_branch, head_branch) = Self::get_branch_info(&manager, cx);

        Self {
            manager,
            title_editor,
            description_editor,
            base_branch,
            head_branch,
            is_draft: false,
            is_submitting: false,
            error_message: None,
            focus_handle,
        }
    }

    fn get_branch_info(manager: &Entity<PullRequestManager>, cx: &App) -> (String, String) {
        // Get current branch from git
        // For now, use defaults
        ("main".to_string(), "feature-branch".to_string())
    }

    fn submit(&mut self, _: &SubmitPullRequest, cx: &mut Context<Self>) {
        if self.is_submitting {
            return;
        }

        let title = self.title_editor.read(cx).text(cx).to_string();
        let description = self.description_editor.read(cx).text(cx).to_string();

        if title.trim().is_empty() {
            self.error_message = Some("Title is required".to_string());
            cx.notify();
            return;
        }

        self.is_submitting = true;
        self.error_message = None;
        cx.notify();

        let manager = self.manager.clone();
        let base = self.base_branch.clone();
        let head = self.head_branch.clone();
        let is_draft = self.is_draft;

        cx.spawn(|this, mut cx| async move {
            let result = cx
                .update(|cx| {
                    manager.update(cx, |manager, cx| {
                        let remote = manager.get_current_remote(cx)?;
                        let pr_data = CreatePullRequest {
                            title,
                            body: description,
                            head,
                            base,
                            draft: is_draft,
                        };
                        
                        let api = manager.api_client();
                        cx.background_executor().spawn(async move {
                            api.create_pull_request(&remote, pr_data).await
                        })
                    })
                })?
                .await;

            this.update(&mut cx, |modal, cx| {
                modal.is_submitting = false;
                
                match result {
                    Ok(pr) => {
                        // Success! Close modal and refresh PR list
                        cx.emit(DismissEvent);
                        
                        // Refresh the PR list
                        modal.manager.update(cx, |manager, cx| {
                            manager.refresh_pull_requests(cx)
                        });
                    }
                    Err(e) => {
                        modal.error_message = Some(format!("Failed to create PR: {}", e));
                    }
                }
                cx.notify();
            })?;

            Ok::<(), anyhow::Error>(())
        })
        .detach_and_log_err(cx);
    }

    fn cancel(&mut self, _: &CancelCreatePr, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn toggle_draft(&mut self, _: &ToggleDraft, cx: &mut Context<Self>) {
        self.is_draft = !self.is_draft;
        cx.notify();
    }
}

impl Render for CreatePrModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        
        Modal::new("create-pr-modal", Some(self.focus_handle.clone()))
            .header(ModalHeader::new("Create Pull Request"))
            .section(
                v_flex()
                    .gap_4()
                    .p_4()
                    .child(
                        v_flex()
                            .gap_1()
                            .child(Label::new("Title").size(LabelSize::Small))
                            .child(
                                div()
                                    .border_1()
                                    .border_color(theme.colors().border)
                                    .rounded_md()
                                    .p_2()
                                    .child(self.title_editor.clone()),
                            ),
                    )
                    .child(
                        v_flex()
                            .gap_1()
                            .child(Label::new("Description").size(LabelSize::Small))
                            .child(
                                div()
                                    .border_1()
                                    .border_color(theme.colors().border)
                                    .rounded_md()
                                    .p_2()
                                    .min_h(px(200.0))
                                    .child(self.description_editor.clone()),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap_4()
                            .child(
                                v_flex()
                                    .gap_1()
                                    .flex_1()
                                    .child(Label::new("Base").size(LabelSize::Small))
                                    .child(
                                        Button::new("base-branch", self.base_branch.clone())
                                            .style(ButtonStyle::Subtle)
                                            .full_width(),
                                    ),
                            )
                            .child(
                                v_flex()
                                    .gap_1()
                                    .flex_1()
                                    .child(Label::new("Compare").size(LabelSize::Small))
                                    .child(
                                        Button::new("head-branch", self.head_branch.clone())
                                            .style(ButtonStyle::Subtle)
                                            .full_width()
                                            .disabled(true),
                                    ),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Checkbox::new(
                                    "draft-pr",
                                    if self.is_draft {
                                        ui::ToggleState::Selected
                                    } else {
                                        ui::ToggleState::Unselected
                                    },
                                )
                                .on_click(cx.listener(|this, _, cx| {
                                    this.toggle_draft(&ToggleDraft, cx);
                                })),
                            )
                            .child(Label::new("Create as draft").size(LabelSize::Small)),
                    )
                    .when_some(self.error_message.as_ref(), |this, error| {
                        this.child(
                            div()
                                .p_2()
                                .rounded_md()
                                .bg(theme.status().error_background)
                                .child(Label::new(error.clone()).color(Color::Error)),
                        )
                    }),
            )
            .footer(
                h_flex()
                    .gap_2()
                    .justify_end()
                    .child(
                        Button::new("cancel", "Cancel")
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(|this, _, cx| {
                                this.cancel(&CancelCreatePr, cx);
                            })),
                    )
                    .child(
                        Button::new("create", "Create Pull Request")
                            .style(ButtonStyle::Filled)
                            .disabled(self.is_submitting)
                            .on_click(cx.listener(|this, _, cx| {
                                this.submit(&SubmitPullRequest, cx);
                            })),
                    ),
            )
    }
}

impl EventEmitter<DismissEvent> for CreatePrModal {}

impl Focusable for CreatePrModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

// Extension trait for Task
trait TaskExt {
    fn detach_and_log_err(self, cx: &mut Context<CreatePrModal>);
}

impl TaskExt for Task<Result<()>> {
    fn detach_and_log_err(self, _cx: &mut Context<CreatePrModal>) {
        self.detach();
    }
}