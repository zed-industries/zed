use anyhow::Result;
use chrono::{DateTime, Local, Utc};
use gpui::{
    actions, div, px, rems, App, ClipboardItem, Context, Entity, EventEmitter, FocusHandle,
    Focusable, InteractiveElement, IntoElement, ParentElement, Render, SharedString,
    StatefulInteractiveElement, Styled, Task, TextStyle, Window,
};
use menu::{Cancel, Confirm};
use pull_requests::{
    PullRequest, PullRequestComment, PullRequestManager, PullRequestReview, PullRequestState,
    ReviewState,
};
use settings::Settings;
use theme::ActiveTheme;
use ui::{
    h_flex, prelude::*, v_flex, Avatar, Button, ButtonStyle, Clickable, Color, FluentBuilder, Icon,
    IconButton, IconName, IconSize, Label, LabelCommon, LabelSize,
};
use workspace::Workspace;

actions!(
    pr_detail_panel,
    [
        ApprovePullRequest,
        RequestChanges,
        SubmitComment,
        MergePullRequest,
        ClosePullRequest,
        RefreshDetails,
        OpenDiff,
        OpenInBrowser,
    ]
);

pub struct PullRequestDetailPanel {
    manager: Entity<PullRequestManager>,
    pull_request: Option<PullRequest>,
    comments: Vec<PullRequestComment>,
    reviews: Vec<PullRequestReview>,
    focus_handle: FocusHandle,
    comment_draft: String,
    review_comment_draft: String,
    is_submitting: bool,
    show_review_form: bool,
    _subscriptions: Vec<gpui::Subscription>,
}

impl PullRequestDetailPanel {
    pub fn new(
        manager: Entity<PullRequestManager>,
        workspace: &Workspace,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let subscriptions =
            vec![cx.subscribe(&manager.read(cx).store().clone(), Self::on_store_event)];

        Self {
            manager,
            pull_request: None,
            comments: Vec::new(),
            reviews: Vec::new(),
            focus_handle,
            comment_draft: String::new(),
            review_comment_draft: String::new(),
            is_submitting: false,
            show_review_form: false,
            _subscriptions: subscriptions,
        }
    }

    pub fn set_pull_request(&mut self, pr: PullRequest, cx: &mut Context<Self>) {
        self.pull_request = Some(pr.clone());
        self.load_details(cx);
        cx.notify();
    }

    fn load_details(&mut self, cx: &mut Context<Self>) {
        if let Some(pr) = &self.pull_request {
            let pr_number = pr.number;
            let manager = self.manager.clone();

            cx.spawn(async move |this, cx| {
                // Load comments and reviews
                // TODO: Implement API calls to fetch comments and reviews

                this.update(cx, |panel, cx| {
                    // Update UI with fetched data
                    cx.notify();
                })?;

                Ok::<(), anyhow::Error>(())
            })
            .detach_and_log_err(cx);
        }
    }

    fn on_store_event(
        &mut self,
        _store: Entity<pull_requests::PullRequestStore>,
        _event: &pull_requests::PullRequestStoreEvent,
        cx: &mut Context<Self>,
    ) {
        cx.notify();
    }

    fn submit_comment(&mut self, _: &SubmitComment, cx: &mut Context<Self>) {
        if self.comment_draft.trim().is_empty() {
            return;
        }

        if let Some(pr) = &self.pull_request {
            let comment_text = self.comment_draft.clone();
            let pr_number = pr.number;
            let manager = self.manager.clone();

            self.is_submitting = true;
            cx.notify();

            cx.spawn(async move |this, cx| {
                // TODO: Implement API call to submit comment

                this.update(cx, |panel, cx| {
                    panel.comment_draft.clear();
                    panel.is_submitting = false;
                    panel.load_details(cx);
                })?;

                Ok::<(), anyhow::Error>(())
            })
            .detach_and_log_err(cx);
        }
    }

    fn approve_pull_request(&mut self, _: &ApprovePullRequest, cx: &mut Context<Self>) {
        if let Some(pr) = &self.pull_request {
            let pr_number = pr.number;
            let manager = self.manager.clone();
            let review_comment = self.review_comment_draft.clone();

            self.is_submitting = true;
            cx.notify();

            cx.spawn(async move |this, cx| {
                // TODO: Implement API call to approve PR

                this.update(cx, |panel, cx| {
                    panel.review_comment_draft.clear();
                    panel.show_review_form = false;
                    panel.is_submitting = false;
                    panel.load_details(cx);
                })?;

                Ok::<(), anyhow::Error>(())
            })
            .detach_and_log_err(cx);
        }
    }

    fn merge_pull_request(&mut self, _: &MergePullRequest, cx: &mut Context<Self>) {
        if let Some(pr) = &self.pull_request {
            if !pr.mergeable.unwrap_or(false) {
                return;
            }

            let pr_number = pr.number;
            let manager = self.manager.clone();

            self.is_submitting = true;
            cx.notify();

            cx.spawn(async move |this, cx| {
                // TODO: Implement API call to merge PR

                this.update(cx, |panel, cx| {
                    panel.is_submitting = false;
                    panel.load_details(cx);
                })?;

                Ok::<(), anyhow::Error>(())
            })
            .detach_and_log_err(cx);
        }
    }

    fn render_pr_header(&self, pr: &PullRequest, cx: &mut Context<Self>) -> impl IntoElement {
        let state_badge = match pr.state {
            PullRequestState::Open => {
                if pr.draft {
                    h_flex()
                        .gap_1()
                        .child(Icon::new(IconName::PullRequest).size(IconSize::Small))
                        .child(Label::new("Draft").size(LabelSize::Small))
                        .px_2()
                        .py_1()
                        .rounded_md()
                        .bg(Color::Modified.color(cx))
                } else {
                    h_flex()
                        .gap_1()
                        .child(Icon::new(IconName::PullRequest).size(IconSize::Small))
                        .child(Label::new("Open").size(LabelSize::Small))
                        .px_2()
                        .py_1()
                        .rounded_md()
                        .bg(Color::Success.color(cx))
                }
            }
            PullRequestState::Closed => h_flex()
                .gap_1()
                .child(Icon::new(IconName::Close).size(IconSize::Small))
                .child(Label::new("Closed").size(LabelSize::Small))
                .px_2()
                .py_1()
                .rounded_md()
                .bg(Color::Error.color(cx)),
            PullRequestState::Merged => h_flex()
                .gap_1()
                .child(Icon::new(IconName::GitBranch).size(IconSize::Small))
                .child(Label::new("Merged").size(LabelSize::Small))
                .px_2()
                .py_1()
                .rounded_md()
                .bg(Color::Accent.color(cx)),
        };

        v_flex()
            .gap_2()
            .p_4()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                h_flex()
                    .gap_2()
                    .child(state_badge)
                    .child(Label::new(format!("#{}", pr.number)).color(Color::Muted)),
            )
            .child(Label::new(SharedString::from(pr.title.clone())).size(LabelSize::Large))
            .child(
                h_flex()
                    .gap_1()
                    .child(Avatar::new(pr.user.avatar_url.to_string()).size(rems(1.0)))
                    .child(
                        Label::new(SharedString::from(pr.user.login.clone()))
                            .size(LabelSize::Small),
                    )
                    .child(
                        Label::new("wants to merge")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(
                        Label::new(SharedString::from(format!(
                            "{} commits into {}",
                            pr.commits, pr.base.ref_name
                        )))
                        .size(LabelSize::Small),
                    ),
            )
    }

    fn render_pr_stats(&self, pr: &PullRequest, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_4()
            .p_2()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        Icon::new(IconName::Plus)
                            .size(IconSize::Small)
                            .color(Color::Success),
                    )
                    .child(Label::new(format!("+{}", pr.additions)).size(LabelSize::Small)),
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        Icon::new(IconName::Trash)
                            .size(IconSize::Small)
                            .color(Color::Error),
                    )
                    .child(Label::new(format!("-{}", pr.deletions)).size(LabelSize::Small)),
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(Icon::new(IconName::File).size(IconSize::Small))
                    .child(
                        Label::new(format!("{} files", pr.changed_files)).size(LabelSize::Small),
                    ),
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(Icon::new(IconName::Circle).size(IconSize::Small))
                    .child(Label::new(format!("{} commits", pr.commits)).size(LabelSize::Small)),
            )
    }

    fn render_pr_description(&self, pr: &PullRequest, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .p_4()
            .child(Label::new("Description").size(LabelSize::Default))
            .child(
                div()
                    .p_2()
                    .rounded_md()
                    .bg(cx.theme().colors().element_background)
                    .child(
                        // TODO: Render markdown properly
                        Label::new(SharedString::from(pr.body.clone()))
                            .size(LabelSize::Small)
                            .color(Color::Default),
                    ),
            )
    }

    fn render_action_buttons(&self, pr: &PullRequest, cx: &mut Context<Self>) -> impl IntoElement {
        let can_merge = pr.mergeable.unwrap_or(false) && pr.state == PullRequestState::Open;

        h_flex()
            .gap_2()
            .p_4()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .when(pr.state == PullRequestState::Open, |this| {
                this.child(
                    Button::new("approve", "Approve")
                        .style(ButtonStyle::Filled)
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.approve_pull_request(&ApprovePullRequest, cx);
                        })),
                )
                .child(
                    Button::new("request-changes", "Request Changes")
                        .style(ButtonStyle::Subtle)
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.show_review_form = true;
                            cx.notify();
                        })),
                )
            })
            .when(can_merge, |this| {
                this.child(
                    Button::new("merge", "Merge Pull Request")
                        .style(ButtonStyle::Filled)
                        .color(Color::Success)
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.merge_pull_request(&MergePullRequest, cx);
                        }))
                        .when(self.is_submitting, |btn| btn.disabled(true)),
                )
            })
            .child(
                IconButton::new("open-browser", IconName::ArrowUpRight)
                    .tooltip(ui::Tooltip::text("Open in Browser"))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        if let Some(pr) = &this.pull_request {
                            cx.open_url(&pr.html_url);
                        }
                    })),
            )
            .child(
                IconButton::new("refresh", IconName::Rerun)
                    .tooltip(ui::Tooltip::text("Refresh"))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.load_details(cx);
                    })),
            )
    }

    fn render_comments(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .p_4()
            .child(
                h_flex()
                    .justify_between()
                    .child(Label::new("Comments").size(LabelSize::Default))
                    .child(
                        Label::new(format!("{} comments", self.comments.len()))
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .children(
                self.comments
                    .iter()
                    .map(|comment| self.render_comment(comment, cx)),
            )
            .child(self.render_comment_form(cx))
    }

    fn render_comment(
        &self,
        comment: &PullRequestComment,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        h_flex()
            .gap_2()
            .p_2()
            .rounded_md()
            .bg(cx.theme().colors().element_background)
            .child(Avatar::new(comment.user.avatar_url.to_string()).size(rems(1.5)))
            .child(
                v_flex()
                    .gap_1()
                    .flex_1()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Label::new(SharedString::from(comment.user.login.clone()))
                                    .size(LabelSize::Small),
                            )
                            .child(
                                Label::new(format_time_ago(comment.created_at))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            ),
                    )
                    .child(
                        Label::new(SharedString::from(comment.body.clone())).size(LabelSize::Small),
                    ),
            )
    }

    fn render_comment_form(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(
                div()
                    .h(px(100.0))
                    .w_full()
                    .rounded_md()
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .p_2()
                    .bg(cx.theme().colors().editor_background)
                    .child(
                        Label::new(if self.comment_draft.is_empty() {
                            SharedString::from("Leave a comment...")
                        } else {
                            SharedString::from(self.comment_draft.clone())
                        })
                        .size(LabelSize::Small)
                        .color(if self.comment_draft.is_empty() {
                            Color::Muted
                        } else {
                            Color::Default
                        }),
                    ),
            )
            .child(
                h_flex().justify_end().child(
                    Button::new("submit-comment", "Comment")
                        .style(ButtonStyle::Filled)
                        .disabled(self.comment_draft.trim().is_empty() || self.is_submitting)
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.submit_comment(&SubmitComment, cx);
                        })),
                ),
            )
    }
}

impl Render for PullRequestDetailPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(pr) = &self.pull_request {
            v_flex()
                .size_full()
                .child(self.render_pr_header(pr, cx))
                .child(self.render_action_buttons(pr, cx))
                .child(self.render_pr_stats(pr, cx))
                .child(
                    div()
                        .flex_1()
                        .overflow_y_scroll()
                        .track_focus(&self.focus_handle)
                        .child(self.render_pr_description(pr, cx))
                        .child(self.render_comments(cx)),
                )
        } else {
            v_flex()
                .size_full()
                .items_center()
                .justify_center()
                .child(Label::new("No pull request selected").color(Color::Muted))
        }
    }
}

impl Focusable for PullRequestDetailPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for PullRequestDetailPanel {}

fn format_time_ago(time: DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(time);

    if duration.num_days() > 365 {
        format!("{} years ago", duration.num_days() / 365)
    } else if duration.num_days() > 30 {
        format!("{} months ago", duration.num_days() / 30)
    } else if duration.num_days() > 0 {
        format!("{} days ago", duration.num_days())
    } else if duration.num_hours() > 0 {
        format!("{} hours ago", duration.num_hours())
    } else if duration.num_minutes() > 0 {
        format!("{} minutes ago", duration.num_minutes())
    } else {
        "just now".to_string()
    }
}
