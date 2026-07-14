use crate::{
    askpass_modal::AskPassModal, branch_diff::BranchDiff, review_comment_modal::ReviewCommentModal,
};
use anyhow::{Context as _, Result, anyhow};
use askpass::AskPassDelegate;
use git::{
    GitHostingProviderRegistry, parse_git_remote_url,
    repository::{FetchOptions, Remote},
};
use github_pull_requests::{
    DeviceAuthorization, DeviceFlowPoll, GitHubAuthentication, GitHubClient, PullRequestDetails,
    PullRequestList, PullRequestSummary, ReviewEvent, ReviewThread,
};
use gpui::{
    Action, App, AppContext as _, AsyncWindowContext, Context, Entity, EventEmitter, FocusHandle,
    Focusable, InteractiveElement, PromptLevel, Render, SharedString, Task, WeakEntity, Window,
    actions, px,
};
use http_client::HttpClient;
use project::Project;
use std::{sync::Arc, time::Instant};
use ui::{Button, ButtonStyle, IconButton, IconName, Label, prelude::*};
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};

actions!(
    pull_requests_panel,
    [
        /// Focuses the Pull Requests panel.
        ToggleFocus,
        /// Refreshes pull requests for the active repository.
        Refresh,
        /// Signs in to GitHub.
        SignIn,
        /// Signs out of GitHub.
        SignOut,
    ]
);

const PANEL_KEY: &str = "PullRequestsPanel";

pub fn register(workspace: &mut Workspace) {
    workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
        workspace.toggle_panel_focus::<PullRequestsPanel>(window, cx);
    });
}

enum PanelState {
    LoadingCredentials,
    SignedOut,
    Authorizing(DeviceAuthorization),
    Loading,
    Ready(PullRequestList),
    Review(PullRequestDetails),
    Error(SharedString),
}

pub struct PullRequestsPanel {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    focus_handle: FocusHandle,
    authentication: Option<Arc<GitHubAuthentication>>,
    http_client: Arc<dyn HttpClient>,
    client: Option<Arc<GitHubClient>>,
    state: PanelState,
    position: DockPosition,
    active_task: Option<Task<Result<()>>>,
}

impl PullRequestsPanel {
    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        workspace.update_in(&mut cx, |workspace, _window, cx| {
            let project = workspace.project().clone();
            let http_client = cx.http_client();
            let authentication = GitHubAuthentication::new(
                http_client.clone(),
                zed_credentials_provider::global(cx),
            )
            .map(Arc::new);
            cx.new(|cx| {
                let mut panel = Self {
                    workspace: workspace.weak_handle(),
                    project,
                    focus_handle: cx.focus_handle(),
                    authentication: authentication.as_ref().ok().cloned(),
                    http_client,
                    client: None,
                    state: PanelState::LoadingCredentials,
                    position: DockPosition::Left,
                    active_task: None,
                };
                match authentication {
                    Ok(_) => panel.load_credentials(cx),
                    Err(error) => panel.state = PanelState::Error(error.to_string().into()),
                }
                panel
            })
        })
    }

    fn load_credentials(&mut self, cx: &mut Context<Self>) {
        let Some(authentication) = self.authentication.clone() else {
            self.state = PanelState::SignedOut;
            return;
        };
        let http_client = self.http_client.clone();
        let task = cx.spawn(async move |this, cx| {
            match authentication.load_valid(&*cx).await {
                Ok(Some(credentials)) => {
                    let client = Arc::new(GitHubClient::new(http_client, credentials.access_token));
                    this.update(cx, |this, cx| {
                        this.client = Some(client);
                        this.refresh(cx);
                    })?;
                }
                Ok(None) => {
                    this.update(cx, |this, cx| {
                        this.state = PanelState::SignedOut;
                        cx.notify();
                    })?;
                }
                Err(error) => {
                    this.update(cx, |this, cx| {
                        this.state = PanelState::Error(error.to_string().into());
                        cx.notify();
                    })?;
                }
            }
            anyhow::Ok(())
        });
        self.active_task = Some(task);
    }

    fn repository_coordinates(&self, cx: &App) -> Result<(Arc<str>, Arc<str>)> {
        let repository = self
            .project
            .read(cx)
            .active_repository(cx)
            .context("No active Git repository")?;
        let remote_url = repository
            .read(cx)
            .default_remote_url()
            .context("The active repository has no GitHub remote")?;
        let (provider, remote) =
            parse_git_remote_url(GitHostingProviderRegistry::global(cx), &remote_url)
                .context("The active repository is not hosted on GitHub")?;
        if provider.name() != "GitHub" {
            return Err(anyhow!(
                "GitHub.com is the only supported pull request host"
            ));
        }
        Ok((remote.owner, remote.repo))
    }

    fn refresh(&mut self, cx: &mut Context<Self>) {
        let Some(client) = self.client.clone() else {
            self.state = PanelState::SignedOut;
            cx.notify();
            return;
        };
        let coordinates = self.repository_coordinates(cx);
        self.state = PanelState::Loading;
        cx.notify();
        let task = cx.spawn(async move |this, cx| {
            let result = match coordinates {
                Ok((owner, repository)) => client.list_pull_requests(&owner, &repository).await,
                Err(error) => Err(error),
            };
            this.update(cx, |this, cx| {
                this.state = match result {
                    Ok(pull_requests) => PanelState::Ready(pull_requests),
                    Err(error) => PanelState::Error(error.to_string().into()),
                };
                this.active_task = None;
                cx.notify();
            })?;
            anyhow::Ok(())
        });
        self.active_task = Some(task);
    }

    fn sign_in(&mut self, cx: &mut Context<Self>) {
        let Some(authentication) = self.authentication.clone() else {
            self.state = PanelState::Error("GitHub sign-in is not configured in this build".into());
            cx.notify();
            return;
        };
        let http_client = self.http_client.clone();
        self.state = PanelState::Loading;
        let task = cx.spawn(async move |this, cx| {
            let authorization = authentication.request_device_authorization().await?;
            cx.update(|cx| cx.open_url(&authorization.verification_uri));
            this.update(cx, |this, cx| {
                this.state = PanelState::Authorizing(authorization.clone());
                cx.notify();
            })?;
            let expires_at = Instant::now() + authorization.expires_in;
            let mut interval = authorization.interval;
            loop {
                cx.background_executor().timer(interval).await;
                if Instant::now() >= expires_at {
                    return Err(anyhow!("GitHub device authorization expired"));
                }
                match authentication
                    .poll_device_authorization(&authorization.device_code)
                    .await?
                {
                    DeviceFlowPoll::Pending => {}
                    DeviceFlowPoll::SlowDown(additional_delay) => {
                        interval += additional_delay;
                    }
                    DeviceFlowPoll::Complete(credentials) => {
                        authentication.store(&credentials, &*cx).await?;
                        let client =
                            Arc::new(GitHubClient::new(http_client, credentials.access_token));
                        this.update(cx, |this, cx| {
                            this.client = Some(client);
                            this.refresh(cx);
                        })?;
                        return Ok(());
                    }
                    DeviceFlowPoll::AccessDenied => {
                        return Err(anyhow!("GitHub authorization was denied"));
                    }
                    DeviceFlowPoll::Expired => {
                        return Err(anyhow!("GitHub device authorization expired"));
                    }
                }
            }
        });
        self.active_task = Some(cx.spawn(async move |this, cx| {
            if let Err(error) = task.await {
                this.update(cx, |this, cx| {
                    this.active_task = None;
                    this.state = PanelState::Error(error.to_string().into());
                    cx.notify();
                })?;
            }
            anyhow::Ok(())
        }));
    }

    fn sign_out(&mut self, cx: &mut Context<Self>) {
        let Some(authentication) = self.authentication.clone() else {
            return;
        };
        let task = cx.spawn(async move |this, cx| {
            authentication.clear(&*cx).await?;
            this.update(cx, |this, cx| {
                this.client = None;
                this.state = PanelState::SignedOut;
                this.active_task = None;
                cx.notify();
            })?;
            anyhow::Ok(())
        });
        self.active_task = Some(cx.spawn(async move |this, cx| {
            if let Err(error) = task.await {
                this.update(cx, |this, cx| {
                    this.active_task = None;
                    this.state = PanelState::Error(error.to_string().into());
                    cx.notify();
                })?;
            }
            anyhow::Ok(())
        }));
    }

    fn askpass_delegate(
        &self,
        operation: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AskPassDelegate {
        let workspace = self.workspace.clone();
        let operation = operation.into();
        let window = window.window_handle();
        AskPassDelegate::new(&mut cx.to_async(), move |prompt, tx, cx| {
            window
                .update(cx, |_, window, cx| {
                    workspace.update(cx, |workspace, cx| {
                        workspace.toggle_modal(window, cx, |window, cx| {
                            AskPassModal::new(operation.clone(), prompt.into(), tx, window, cx)
                        });
                    })
                })
                .ok();
        })
    }

    fn open_pull_request(
        &mut self,
        pull_request: PullRequestSummary,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(client) = self.client.clone() else {
            self.state = PanelState::SignedOut;
            cx.notify();
            return;
        };
        let Some(repository) = self.project.read(cx).active_repository(cx) else {
            self.state = PanelState::Error("No active Git repository".into());
            cx.notify();
            return;
        };
        let has_git_changes = repository.read(cx).status_summary().count > 0;
        let has_unsaved_edits = self
            .project
            .read(cx)
            .opened_buffers(cx)
            .into_iter()
            .any(|buffer| buffer.read(cx).has_unsaved_edits());
        if has_git_changes || has_unsaved_edits {
            self.state = PanelState::Error(
                "Save and commit or stash local changes before checking out a pull request".into(),
            );
            cx.notify();
            return;
        }

        let Some(remote) = repository.read(cx).default_remote() else {
            self.state = PanelState::Error("The active repository has no Git remote".into());
            cx.notify();
            return;
        };
        let askpass = self.askpass_delegate("fetch pull request", window, cx);
        let base_askpass = self.askpass_delegate("fetch pull request base", window, cx);
        let project = self.project.clone();
        let workspace = self.workspace.clone();
        self.state = PanelState::Loading;
        cx.notify();
        let task = cx.spawn_in(window, async move |this, cx| {
            let namespace = format!(
                "refs/zed/pull-requests/{}/{}/{}",
                pull_request.id.owner, pull_request.id.repository, pull_request.id.number
            );
            let head_ref = format!("{namespace}/head");
            let base_ref = format!("{namespace}/base");
            repository
                .update(cx, |repository, cx| {
                    repository.fetch(
                        FetchOptions::Refspec {
                            remote: Remote {
                                name: remote.name.clone(),
                            },
                            source: format!("refs/pull/{}/head", pull_request.id.number).into(),
                            destination: head_ref.clone().into(),
                        },
                        askpass,
                        cx,
                    )
                })
                .await??;

            repository
                .update(cx, |repository, cx| {
                    repository.fetch(
                        FetchOptions::Refspec {
                            remote,
                            source: format!("refs/heads/{}", pull_request.base_ref).into(),
                            destination: base_ref.clone().into(),
                        },
                        base_askpass,
                        cx,
                    )
                })
                .await??;

            let short_sha = pull_request.head_sha.get(..12).unwrap_or(&pull_request.head_sha);
            let branch_name = format!("zed-pr/{}-{}", pull_request.id.number, short_sha);
            let branches = repository
                .update(cx, |repository, _| repository.branches())
                .await??;
            if let Some(existing) = branches
                .branches
                .iter()
                .find(|branch| !branch.is_remote() && branch.name() == branch_name)
            {
                let existing_sha = existing
                    .most_recent_commit
                    .as_ref()
                    .map(|commit| commit.sha.as_ref());
                if existing_sha != Some(pull_request.head_sha.as_ref()) {
                    return Err(anyhow!(
                        "Local branch {branch_name} points to a different commit; rename or delete it before retrying"
                    ));
                }
                repository
                    .update(cx, |repository, _| repository.change_branch(branch_name))
                    .await??;
            } else {
                repository
                    .update(cx, |repository, _| {
                        repository.create_branch(branch_name, Some(head_ref))
                    })
                    .await??;
            }

            workspace.update_in(cx, |workspace, window, cx| {
                BranchDiff::deploy_branch_diff_with_base_ref(
                    workspace,
                    project,
                    repository,
                    base_ref.into(),
                    Some(format!("PR #{}: {}", pull_request.id.number, pull_request.title).into()),
                    window,
                    cx,
                );
            })?;
            let details = client.pull_request_details(&pull_request.id).await?;
            this.update(cx, |this, cx| {
                this.active_task = None;
                this.state = PanelState::Review(details);
                cx.notify();
            })?;
            anyhow::Ok(())
        });
        self.active_task = Some(cx.spawn_in(window, async move |this, cx| {
            if let Err(error) = task.await {
                this.update(cx, |this, cx| {
                    this.active_task = None;
                    this.state = PanelState::Error(error.to_string().into());
                    cx.notify();
                })?;
            }
            anyhow::Ok(())
        }));
    }

    fn approve(
        &mut self,
        details: PullRequestDetails,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(client) = self.client.clone() else {
            return;
        };
        let answer = window.prompt(
            PromptLevel::Info,
            &format!("Approve pull request #{}?", details.summary.id.number),
            Some(&details.summary.title),
            &["Approve", "Cancel"],
            cx,
        );
        let task = cx.spawn(async move |this, cx| {
            if answer.await != Ok(0) {
                this.update(cx, |this, _| this.active_task = None)?;
                return Ok(());
            }
            let review_id = match &details.pending_review {
                Some(review) => review.id.clone(),
                None => {
                    client
                        .create_pending_review(&details.summary.id, &details.summary.head_sha)
                        .await?
                }
            };
            client
                .submit_review(&details.summary.id, &review_id, ReviewEvent::Approve, "")
                .await?;
            let details = client.pull_request_details(&details.summary.id).await?;
            this.update(cx, |this, cx| {
                this.active_task = None;
                this.state = PanelState::Review(details);
                cx.notify();
            })?;
            anyhow::Ok(())
        });
        self.active_task = Some(cx.spawn(async move |this, cx| {
            if let Err(error) = task.await {
                this.update(cx, |this, cx| {
                    this.active_task = None;
                    this.state = PanelState::Error(error.to_string().into());
                    cx.notify();
                })?;
            }
            anyhow::Ok(())
        }));
    }

    fn set_thread_resolved(
        &mut self,
        details: PullRequestDetails,
        thread: ReviewThread,
        cx: &mut Context<Self>,
    ) {
        let Some(client) = self.client.clone() else {
            return;
        };
        self.state = PanelState::Loading;
        cx.notify();
        let task = cx.spawn(async move |this, cx| {
            client
                .set_thread_resolved(&thread.id, !thread.is_resolved)
                .await?;
            let details = client.pull_request_details(&details.summary.id).await?;
            this.update(cx, |this, cx| {
                this.active_task = None;
                this.state = PanelState::Review(details);
                cx.notify();
            })?;
            anyhow::Ok(())
        });
        self.active_task = Some(cx.spawn(async move |this, cx| {
            if let Err(error) = task.await {
                this.update(cx, |this, cx| {
                    this.active_task = None;
                    this.state = PanelState::Error(error.to_string().into());
                    cx.notify();
                })?;
            }
            anyhow::Ok(())
        }));
    }

    fn review_body_prompt(
        &self,
        title: SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> futures::channel::oneshot::Receiver<String> {
        let (sender, receiver) = futures::channel::oneshot::channel();
        let workspace = self.workspace.clone();
        workspace
            .update(cx, |workspace, cx| {
                workspace.toggle_modal(window, cx, |window, cx| {
                    ReviewCommentModal::new(title, sender, window, cx)
                });
            })
            .ok();
        receiver
    }

    fn request_changes(
        &mut self,
        details: PullRequestDetails,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(client) = self.client.clone() else {
            return;
        };
        let body = self.review_body_prompt("Request changes".into(), window, cx);
        let task = cx.spawn(async move |this, cx| {
            let Ok(body) = body.await else {
                return Ok(());
            };
            let review_id = match &details.pending_review {
                Some(review) => review.id.clone(),
                None => {
                    client
                        .create_pending_review(&details.summary.id, &details.summary.head_sha)
                        .await?
                }
            };
            client
                .submit_review(
                    &details.summary.id,
                    &review_id,
                    ReviewEvent::RequestChanges,
                    &body,
                )
                .await?;
            let details = client.pull_request_details(&details.summary.id).await?;
            this.update(cx, |this, cx| {
                this.active_task = None;
                this.state = PanelState::Review(details);
                cx.notify();
            })?;
            anyhow::Ok(())
        });
        self.active_task = Some(cx.spawn(async move |this, cx| {
            if let Err(error) = task.await {
                this.update(cx, |this, cx| {
                    this.active_task = None;
                    this.state = PanelState::Error(error.to_string().into());
                    cx.notify();
                })?;
            }
            anyhow::Ok(())
        }));
    }

    fn reply_to_thread(
        &mut self,
        details: PullRequestDetails,
        thread: ReviewThread,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(client) = self.client.clone() else {
            return;
        };
        let Some(comment_id) = thread.comments.last().map(|comment| comment.id.clone()) else {
            return;
        };
        let body = self.review_body_prompt("Reply to review thread".into(), window, cx);
        let task = cx.spawn(async move |this, cx| {
            let Ok(body) = body.await else {
                return Ok(());
            };
            client
                .reply_to_comment(&details.summary.id, comment_id, &body)
                .await?;
            let details = client.pull_request_details(&details.summary.id).await?;
            this.update(cx, |this, cx| {
                this.active_task = None;
                this.state = PanelState::Review(details);
                cx.notify();
            })?;
            anyhow::Ok(())
        });
        self.active_task = Some(cx.spawn(async move |this, cx| {
            if let Err(error) = task.await {
                this.update(cx, |this, cx| {
                    this.active_task = None;
                    this.state = PanelState::Error(error.to_string().into());
                    cx.notify();
                })?;
            }
            anyhow::Ok(())
        }));
    }

    fn render_review(&self, details: &PullRequestDetails, cx: &mut Context<Self>) -> AnyElement {
        let mut content = v_flex()
            .p_3()
            .gap_2()
            .child(Label::new(details.summary.title.clone()).size(LabelSize::Large))
            .child(
                Label::new(format!(
                    "#{} · {} → {} · +{} −{} · {} files",
                    details.summary.id.number,
                    details.summary.head_ref,
                    details.summary.base_ref,
                    details.additions,
                    details.deletions,
                    details.changed_files,
                ))
                .size(LabelSize::Small)
                .color(Color::Muted),
            )
            .when(!details.body.is_empty(), |this| {
                this.child(Label::new(details.body.clone()).size(LabelSize::Small))
            })
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        Button::new("approve-pull-request", "Approve")
                            .style(ButtonStyle::Filled)
                            .on_click({
                                let details = details.clone();
                                cx.listener(move |this, _, window, cx| {
                                    this.approve(details.clone(), window, cx)
                                })
                            }),
                    )
                    .child(
                        Button::new("request-pull-request-changes", "Request Changes").on_click({
                            let details = details.clone();
                            cx.listener(move |this, _, window, cx| {
                                this.request_changes(details.clone(), window, cx)
                            })
                        }),
                    )
                    .child(
                        Button::new("open-pull-request-on-github", "Open on GitHub").on_click({
                            let url = details.summary.html_url.clone();
                            move |_, _, cx| cx.open_url(&url)
                        }),
                    ),
            )
            .child(Label::new(format!("Checks ({})", details.checks.len())))
            .children(details.checks.iter().map(|check| {
                Label::new(format!(
                    "{} · {:?}",
                    check.name,
                    check.conclusion.unwrap_or_else(|| match check.state {
                        github_pull_requests::CheckState::Queued
                        | github_pull_requests::CheckState::InProgress => {
                            github_pull_requests::CheckConclusion::Unknown
                        }
                        github_pull_requests::CheckState::Completed => {
                            github_pull_requests::CheckConclusion::Unknown
                        }
                    })
                ))
                .size(LabelSize::Small)
            }))
            .child(Label::new(format!(
                "Review threads ({})",
                details.threads.len()
            )));
        for (thread_index, thread) in details.threads.iter().enumerate() {
            let thread_for_click = thread.clone();
            let details_for_click = details.clone();
            let label = if thread.is_resolved {
                "Reopen"
            } else {
                "Resolve"
            };
            let mut thread_content = v_flex().p_2().gap_1().border_1().rounded_sm();
            for comment in &thread.comments {
                thread_content = thread_content.child(
                    v_flex()
                        .child(Label::new(comment.author.login.clone()).size(LabelSize::XSmall))
                        .child(Label::new(comment.body.clone()).size(LabelSize::Small)),
                );
            }
            if thread.viewer_can_resolve {
                thread_content = thread_content.child(
                    Button::new(("resolve-review-thread", thread_index), label).on_click(
                        cx.listener(move |this, _, _, cx| {
                            this.set_thread_resolved(
                                details_for_click.clone(),
                                thread_for_click.clone(),
                                cx,
                            )
                        }),
                    ),
                );
            }
            if thread.viewer_can_reply && !thread.comments.is_empty() {
                let thread_for_click = thread.clone();
                let details_for_click = details.clone();
                thread_content = thread_content.child(
                    Button::new(("reply-review-thread", thread_index), "Reply").on_click(
                        cx.listener(move |this, _, window, cx| {
                            this.reply_to_thread(
                                details_for_click.clone(),
                                thread_for_click.clone(),
                                window,
                                cx,
                            )
                        }),
                    ),
                );
            }
            content = content.child(thread_content);
        }
        content.into_any_element()
    }

    fn render_pull_request(
        &self,
        pull_request: &PullRequestSummary,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let title = pull_request.title.clone();
        let subtitle: SharedString = format!(
            "#{} · {}",
            pull_request.id.number, pull_request.author.login
        )
        .into();
        let pull_request = pull_request.clone();
        v_flex()
            .id(("pull-request", pull_request.id.number as usize))
            .px_2()
            .py_1()
            .gap_0p5()
            .hover(|style| style.bg(cx.theme().colors().element_hover))
            .cursor_pointer()
            .child(Label::new(title).size(LabelSize::Small))
            .child(
                Label::new(subtitle)
                    .size(LabelSize::XSmall)
                    .color(Color::Muted),
            )
            .on_click(cx.listener(move |this, _, window, cx| {
                this.open_pull_request(pull_request.clone(), window, cx)
            }))
    }

    fn render_list(&self, pull_requests: &PullRequestList, cx: &mut Context<Self>) -> AnyElement {
        let mut content = v_flex().gap_1();
        content = content.child(
            h_flex()
                .px_2()
                .child(Label::new("Waiting for My Review").size(LabelSize::Small)),
        );
        if pull_requests.waiting_for_review.is_empty() {
            content = content.child(
                Label::new("No pull requests are waiting for you")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            );
        } else {
            for pull_request in &pull_requests.waiting_for_review {
                content = content.child(self.render_pull_request(pull_request, cx));
            }
        }
        content = content.child(
            h_flex()
                .mt_2()
                .px_2()
                .child(Label::new("Created by Me").size(LabelSize::Small)),
        );
        for pull_request in &pull_requests.authored_by_viewer {
            content = content.child(self.render_pull_request(pull_request, cx));
        }
        content.into_any_element()
    }
}

impl EventEmitter<PanelEvent> for PullRequestsPanel {}

impl Focusable for PullRequestsPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for PullRequestsPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let header = h_flex()
            .h_8()
            .px_2()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .child(Label::new("Pull Requests"))
            .child(
                h_flex()
                    .child(
                        IconButton::new("refresh-pull-requests", IconName::ArrowCircle)
                            .on_click(cx.listener(|this, _, _, cx| this.refresh(cx))),
                    )
                    .when(self.client.is_some(), |this| {
                        this.child(
                            IconButton::new("sign-out-github", IconName::Exit)
                                .on_click(cx.listener(|this, _, _, cx| this.sign_out(cx))),
                        )
                    }),
            );
        let body = match &self.state {
            PanelState::LoadingCredentials | PanelState::Loading => v_flex()
                .p_3()
                .child(Label::new("Loading pull requests…").color(Color::Muted))
                .into_any_element(),
            PanelState::SignedOut => v_flex()
                .p_3()
                .gap_2()
                .child(Label::new(
                    "Sign in to review GitHub pull requests from Zed.",
                ))
                .child(
                    Button::new("sign-in-github", "Sign in to GitHub")
                        .style(ButtonStyle::Filled)
                        .on_click(cx.listener(|this, _, _, cx| this.sign_in(cx))),
                )
                .into_any_element(),
            PanelState::Authorizing(authorization) => v_flex()
                .p_3()
                .gap_2()
                .child(Label::new("Enter this code on GitHub:"))
                .child(Label::new(authorization.user_code.clone()).size(LabelSize::Large))
                .child(Label::new(authorization.verification_uri.clone()).color(Color::Muted))
                .into_any_element(),
            PanelState::Ready(pull_requests) => self.render_list(pull_requests, cx),
            PanelState::Review(details) => self.render_review(details, cx),
            PanelState::Error(error) => v_flex()
                .p_3()
                .gap_2()
                .child(Label::new(error.clone()).color(Color::Error))
                .child(
                    Button::new("retry-pull-requests", "Retry").on_click(cx.listener(
                        |this, _, _, cx| {
                            if this.client.is_some() {
                                this.refresh(cx);
                            } else {
                                this.sign_in(cx);
                            }
                        },
                    )),
                )
                .into_any_element(),
        };
        v_flex()
            .id("pull-requests-panel")
            .size_full()
            .track_focus(&self.focus_handle)
            .child(header)
            .child(
                div()
                    .id("pull-requests-scroll")
                    .flex_1()
                    .overflow_y_scroll()
                    .child(body),
            )
    }
}

impl Panel for PullRequestsPanel {
    fn persistent_name() -> &'static str {
        PANEL_KEY
    }

    fn panel_key() -> &'static str {
        PANEL_KEY
    }

    fn position(&self, _: &Window, _: &App) -> DockPosition {
        self.position
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, _: &mut Context<Self>) {
        self.position = position;
    }

    fn default_size(&self, _: &Window, _: &App) -> gpui::Pixels {
        px(320.)
    }

    fn icon(&self, _: &Window, _: &App) -> Option<IconName> {
        Some(IconName::Github)
    }

    fn icon_tooltip(&self, _: &Window, _: &App) -> Option<&'static str> {
        Some("Pull Requests")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn starts_open(&self, _: &Window, _: &App) -> bool {
        false
    }

    fn activation_priority(&self) -> u32 {
        4
    }
}
