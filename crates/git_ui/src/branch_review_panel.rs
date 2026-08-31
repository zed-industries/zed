use crate::{branch_diff::BranchDiff, branch_review::ReviewEvent, project_diff::CompareWithBranch};
use crate::{
    github_review::{Checkout, CommentTarget, DiffSide, GitHubRepo, PullRequest},
    github_review_ui::{GitHubReview, GitHubReviewEvent},
};
use anyhow::{Context as _, Result, ensure};
use db::kvp::KeyValueStore;
use gpui::{
    AsyncWindowContext, Entity, EventEmitter, FocusHandle, Focusable, Subscription, Task,
    WeakEntity,
};
use project::git_store::{GitStoreEvent, diff_buffer_list::DiffBase};
use serde::{Deserialize, Serialize};
use settings::{IntoGpui as _, Settings};
use std::path::PathBuf;
use ui::{Tooltip, prelude::*};
use util::ResultExt as _;
use workspace::{
    Panel, Workspace,
    dock::{DockPosition, PanelEvent},
};

gpui::actions!(branch_review, [ToggleFocus, CloseReview]);

pub(crate) fn register(workspace: &mut Workspace) {
    workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
        workspace.toggle_panel_focus::<BranchReviewPanel>(window, cx);
    });
    workspace.register_action(|workspace, _: &CloseReview, _, cx| {
        if let Some(panel) = workspace.panel::<BranchReviewPanel>(cx) {
            panel.update(cx, |panel, cx| panel.close_review(cx));
        }
    });
}

#[derive(Clone, Debug, settings::RegisterSetting)]
struct ReviewPanelSettings {
    button: bool,
    dock: DockPosition,
    default_width: Pixels,
}

impl Settings for ReviewPanelSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let settings = content.branch_review_panel.as_ref();
        Self {
            button: settings.and_then(|s| s.button).unwrap_or(true),
            dock: settings
                .and_then(|s| s.dock)
                .map(Into::into)
                .unwrap_or(DockPosition::Left),
            default_width: settings
                .and_then(|s| s.default_width)
                .map(|value| value.into_gpui())
                .unwrap_or(px(320.)),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SavedComparison {
    worktree: PathBuf,
    branch: String,
    base_ref: String,
    #[serde(default)]
    checkout: Option<Checkout>,
}

// Keeping the item alive preserves live buffers and navigation when its tab is
// closed. The item only holds a weak workspace reference, avoiding a cycle.
struct ReviewSession {
    item: Entity<BranchDiff>,
    comparison: SavedComparison,
    _subscription: Subscription,
}

pub struct BranchReviewPanel {
    workspace: WeakEntity<Workspace>,
    github: Entity<GitHubReview>,
    checkout_task: Option<Task<()>>,
    pending_checkout: Option<Checkout>,
    ignored_item: Option<gpui::EntityId>,
    write_generation: u64,
    focus_handle: FocusHandle,
    session: Option<ReviewSession>,
    pending_restore: Option<SavedComparison>,
    storage_key: Option<String>,
    write_task: Option<Task<()>>,
    error: Option<String>,
    _subscriptions: Vec<Subscription>,
}

impl BranchReviewPanel {
    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        let (storage_key, database) = workspace.read_with(&cx, |workspace, cx| {
            let key = workspace
                .database_id()
                .map(|id| i64::from(id).to_string())
                .or_else(|| workspace.session_id())
                .map(|id| format!("branch_review_panel_v1:{id}"));
            (key, KeyValueStore::global(cx))
        })?;
        let restored = if let Some(key) = storage_key.clone() {
            cx.background_spawn(async move {
                database
                    .read_kvp(&key)?
                    .map(|value| serde_json::from_str::<Option<SavedComparison>>(&value))
                    .transpose()
                    .map(Option::flatten)
                    .map_err(anyhow::Error::from)
            })
            .await
        } else {
            Ok(None)
        };
        workspace.update_in(&mut cx, |workspace, window, cx| {
            let handle = cx.entity();
            cx.new(|cx| Self::new(handle, workspace, storage_key, restored, window, cx))
        })
    }

    fn new(
        workspace_handle: Entity<Workspace>,
        workspace: &Workspace,
        storage_key: Option<String>,
        restored: Result<Option<SavedComparison>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let git_store = workspace.project().read(cx).git_store().clone();
        let github = cx.new(|cx| GitHubReview::new(workspace.project().clone(), window, cx));
        let subscriptions = vec![
            cx.subscribe_in(&github, window, |this, _, event, window, cx| match event {
                GitHubReviewEvent::Open { repo, pr } => {
                    this.open_pull_request(repo.clone(), pr.clone(), window, cx)
                }
                GitHubReviewEvent::CommentSelection => this.comment_selection(window, cx),
                GitHubReviewEvent::CommentsLoaded(comments) => {
                    if let Some(session) = &this.session {
                        let review = session.item.read(cx).review.clone();
                        review.update(cx, |review, cx| review.set_comments(comments.clone(), cx));
                    }
                }
            }),
            cx.subscribe_in(&workspace_handle, window, |_, _, event, window, cx| {
                if matches!(
                    event,
                    workspace::Event::ActiveItemChanged | workspace::Event::ItemAdded { .. }
                ) {
                    cx.defer_in(window, |this, window, cx| {
                        this.follow_review_item(window, cx)
                    });
                }
            }),
            cx.subscribe_in(&git_store, window, |_, _, event, window, cx| {
                if matches!(
                    event,
                    GitStoreEvent::RepositoryUpdated(_, _, _)
                        | GitStoreEvent::RepositoryAdded
                        | GitStoreEvent::RepositoryRemoved(_)
                        | GitStoreEvent::ActiveRepositoryChanged(_)
                ) {
                    cx.defer_in(window, |this, window, cx| {
                        this.reconcile_repository(window, cx)
                    });
                }
            }),
            cx.on_app_quit(|this, _| {
                let pending = this.write_task.take();
                async move {
                    if let Some(pending) = pending {
                        pending.await;
                    }
                }
            }),
        ];
        let (pending_restore, error) = match restored {
            Ok(saved) => (saved, None),
            Err(error) => (
                None,
                Some(format!("Could not restore review selection: {error:#}")),
            ),
        };
        cx.defer_in(window, |this, window, cx| {
            this.follow_review_item(window, cx);
            this.reconcile_repository(window, cx);
        });
        Self {
            workspace: workspace_handle.downgrade(),
            github,
            checkout_task: None,
            pending_checkout: None,
            ignored_item: None,
            write_generation: 0,
            focus_handle: cx.focus_handle(),
            session: None,
            pending_restore,
            storage_key,
            write_task: None,
            error,
            _subscriptions: subscriptions,
        }
    }

    fn follow_review_item(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(item) = self
            .workspace
            .read_with(cx, |workspace, cx| {
                workspace.active_item_as::<BranchDiff>(cx)
            })
            .ok()
            .flatten()
        else {
            return;
        };
        if self.ignored_item == Some(item.entity_id()) {
            return;
        }
        if self.session.as_ref().is_some_and(|session| session.item == item && matches!(item.read(cx).diff_base(cx), DiffBase::Merge { base_ref } if base_ref.as_ref() == session.comparison.base_ref)) { return; }
        let branch_diff = item.read(cx);
        let Some(repo) = branch_diff.repo(cx) else {
            return;
        };
        let repo = repo.read(cx);
        let Some(branch) = repo.branch.as_ref() else {
            return;
        };
        let DiffBase::Merge { base_ref } = branch_diff.diff_base(cx) else {
            return;
        };
        let comparison = SavedComparison {
            worktree: repo.work_directory_abs_path.to_path_buf(),
            branch: branch.ref_name.to_string(),
            base_ref: base_ref.to_string(),
            checkout: self.pending_checkout.take().or_else(|| {
                self.pending_restore
                    .as_ref()
                    .filter(|saved| {
                        saved.worktree == repo.work_directory_abs_path.as_ref()
                            && saved.branch == branch.ref_name.as_ref()
                            && saved.base_ref == base_ref.as_ref()
                    })
                    .and_then(|saved| saved.checkout.clone())
            }),
        };
        let review = branch_diff.review.clone();
        if let Some(checkout) = &comparison.checkout {
            match checkout.review_key(&comparison.worktree) {
                Ok(key) => review.update(cx, |review, cx| review.set_storage_key(Some(key), cx)),
                Err(error) => self.error = Some(error.to_string()),
            }
            self.github.update(cx, |github, cx| {
                github.attach(checkout.clone(), window, cx);
                github.set_review(review.downgrade());
            });
        } else {
            review.update(cx, |review, cx| {
                review.set_storage_key(None, cx);
                review.set_comments(Vec::new(), cx);
            });
            self.github.update(cx, |github, cx| github.detach(cx));
        }
        self.ignored_item = None;
        let subscription = cx.subscribe_in(
            &review,
            window,
            |this, _, event: &ReviewEvent, window, cx| match event {
                ReviewEvent::OpenDiff => {
                    cx.defer_in(window, |this, window, cx| this.open_diff(window, cx))
                }
                ReviewEvent::Reply(id) => {
                    this.github.update(cx, |github, cx| {
                        github.select_target(CommentTarget::Reply { comment_id: *id }, window, cx)
                    });
                }
            },
        );
        let item_subscription = cx.observe_in(&item, window, |_, _, window, cx| {
            cx.defer_in(window, |this, window, cx| {
                this.follow_review_item(window, cx)
            });
        });
        self.pending_restore = None;
        self.session = Some(ReviewSession {
            item,
            comparison,
            _subscription: Subscription::join(subscription, item_subscription),
        });
        self.persist(cx);
        let workspace = self.workspace.clone();
        window.defer(cx, move |window, cx| {
            workspace
                .update(cx, |workspace, cx| {
                    workspace.focus_panel::<BranchReviewPanel>(window, cx);
                })
                .log_err();
        });
        cx.notify();
    }

    fn reconcile_repository(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        if self
            .checkout_task
            .as_ref()
            .is_some_and(|task| !task.is_ready())
        {
            return;
        }
        let repository = workspace.read(cx).project().read(cx).active_repository(cx);
        self.github.update(cx, |github, cx| {
            github.set_repository(repository.clone(), cx)
        });
        let Some(repo) = repository else {
            if self.session.is_some() {
                self.close_review(cx);
            }
            return;
        };
        let snapshot = repo.read(cx).snapshot();
        let matches = |saved: &SavedComparison| {
            snapshot.work_directory_abs_path.as_ref() == saved.worktree
                && snapshot
                    .branch
                    .as_ref()
                    .is_some_and(|branch| branch.ref_name.as_ref() == saved.branch)
        };
        if self
            .session
            .as_ref()
            .is_some_and(|session| !matches(&session.comparison))
        {
            self.close_review(cx);
        }
        if self.pending_restore.as_ref().is_some_and(matches) {
            if let Some(saved) = self.pending_restore.take() {
                self.pending_checkout = saved.checkout.clone();
                workspace.update(cx, |workspace, cx| {
                    BranchDiff::deploy_branch_diff_with_base_ref(
                        workspace,
                        workspace.project().clone(),
                        repo,
                        saved.base_ref.into(),
                        None,
                        window,
                        cx,
                    );
                });
            }
        }
    }

    fn open_pull_request(
        &mut self,
        repo: GitHubRepo,
        pr: PullRequest,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.github.read(cx).is_posting()
            || self
                .checkout_task
                .as_ref()
                .is_some_and(|task| !task.is_ready())
        {
            return;
        }
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let project = workspace.read(cx).project().clone();
        let Some(repository) = project.read(cx).active_repository(cx) else {
            return;
        };
        if !self.github.read(cx).matches_repository(&repository, cx) {
            self.error = Some(
                "The active repository changed. Browse PRs for the selected repository again."
                    .into(),
            );
            cx.notify();
            return;
        }
        let root = repository.read(cx).work_directory_abs_path.to_path_buf();
        let previous = self.github.read(cx).checkout.clone();
        self.error = None;
        let weak_repository = repository.downgrade();
        let project_for_job = project.clone();
        let job = repository.update(cx, |repository, _| repository.send_job("open_pull_request", Some("Opening PR checkout".into()), move |state, cx| async move {
            let project = project_for_job;
            crate::github_review::checkout_pull_request(cx.background_executor(), root, repo, pr, previous, || {
                let project = project.read_with(&cx, |project, cx| {
                    let repository = weak_repository.upgrade().context("Repository closed")?;
                    ensure!(project.is_local() && !project.is_read_only(cx), "PR checkout requires a writable local project");
                    ensure!(repository.read(cx).is_trusted() && matches!(&state, project::git_store::RepositoryState::Local(local) if local.backend.is_trusted()), "Trust this project before opening a PR checkout");
                    ensure!(project.active_repository(cx).as_ref() == Some(&repository), "The active repository changed during checkout");
                    ensure!(!project.buffer_store().read(cx).buffers().any(|buffer| buffer.read(cx).is_dirty()), "Save or close unsaved buffers before opening a PR checkout");
                    anyhow::Ok(())
                });
                project
            }).await
        }));
        self.checkout_task = Some(cx.spawn_in(window, async move |this, cx| {
            let result: Result<()> = async {
                let checkout = job.await??;
                let mut observed = false;
                for _ in 0..100 {
                    observed = cx.update(|_, cx| repository.read(cx).branch.as_ref().is_some_and(|branch| branch.name() == checkout.branch))?;
                    if observed { break; }
                    cx.background_executor().timer(std::time::Duration::from_millis(50)).await;
                }
                ensure!(observed, "Git switched branches, but the project has not refreshed. Open Branch Review again after the project refreshes.");
                let item = cx.update(|window, cx| BranchDiff::new_with_branch_base(project.clone(), workspace.clone(), checkout.base_ref.clone().into(), repository, None, window, cx))?.await?;
                this.update(cx, |this, _| { this.pending_checkout = Some(checkout); this.ignored_item = None; })?;
                workspace.update_in(cx, |workspace, window, cx| workspace.add_item_to_active_pane(Box::new(item), None, true, window, cx))?;
                Ok(())
            }.await;
            this.update_in(cx, |this, window, cx| {
                this.checkout_task = None;
                if let Err(error) = result { this.error = Some(format!("{error:#}")); }
                this.follow_review_item(window, cx);
                this.reconcile_repository(window, cx);
                cx.notify();
            }).log_err();
        }));
        cx.notify();
    }

    fn comment_selection(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let result: Result<CommentTarget> = (|| {
            let session = self.session.as_ref().context("Open a PR review first")?;
            let checkout = session
                .comparison
                .checkout
                .as_ref()
                .context("This is a local branch comparison")?;
            let split = session.item.read(cx).editor(cx);
            let split = split.read(cx);
            let editor_handle = split.focused_editor();
            let editor = editor_handle.read(cx);
            let selection = editor.selections.newest_anchor();
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let ranges: Vec<_> = snapshot
                .range_to_buffer_ranges_with_deleted_hunks(selection.start..selection.end)
                .collect();
            ensure!(
                ranges.len() == 1,
                "Select lines on one side of one file to comment"
            );
            let (buffer, range, deleted) = ranges
                .first()
                .context("Select a changed line in the diff")?;
            let side = if deleted.is_some() || split.lhs_editor() == Some(editor_handle) {
                DiffSide::Left
            } else {
                DiffSide::Right
            };
            let path = session
                .item
                .read(cx)
                .review
                .read(cx)
                .path_for_buffer(buffer.remote_id(), cx)
                .context("The selected excerpt is not part of this PR comparison")?
                .to_string();
            use language::ToPoint as _;
            let start = range.start.to_point(buffer);
            let end = range.end.to_point(buffer);
            let line = if end.column == 0 && end.row > start.row {
                end.row
            } else {
                end.row + 1
            };
            Ok(CommentTarget::Inline {
                path,
                side,
                start_line: start.row + 1,
                line,
                head_sha: checkout.pull_request.head.sha.clone(),
                base_sha: checkout.pull_request.base.sha.clone(),
            })
        })();
        self.github.update(cx, |github, cx| match result {
            Ok(target) => github.select_target(target, window, cx),
            Err(error) => github.show_error(format!("{error:#}"), cx),
        });
    }

    fn open_diff(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(session) = &self.session else {
            return;
        };
        let item = session.item.clone();
        self.workspace
            .update(cx, |workspace, cx| {
                if !workspace.activate_item(&item, true, true, window, cx) {
                    workspace.add_item_to_active_pane(Box::new(item), None, true, window, cx);
                }
            })
            .log_err();
    }

    fn close_review(&mut self, cx: &mut Context<Self>) {
        if let Some(session) = &self.session {
            session
                .item
                .read(cx)
                .review
                .clone()
                .update(cx, |review, cx| review.set_comments(Vec::new(), cx));
        }
        self.ignored_item = self
            .session
            .as_ref()
            .map(|session| session.item.entity_id());
        self.session = None;
        self.pending_restore = None;
        self.github.update(cx, |github, cx| github.close_review(cx));
        self.persist(cx);
        cx.notify();
    }

    fn persist(&mut self, cx: &mut Context<Self>) {
        let Some(key) = self.storage_key.clone() else {
            return;
        };
        let value = match serde_json::to_string(
            &self.session.as_ref().map(|session| &session.comparison),
        ) {
            Ok(value) => value,
            Err(error) => {
                self.error = Some(error.to_string());
                return;
            }
        };
        self.write_generation += 1;
        let generation = self.write_generation;
        let database = KeyValueStore::global(cx);
        let previous = self.write_task.take();
        self.write_task = Some(cx.spawn(async move |this, cx| {
            if let Some(previous) = previous {
                previous.await;
            }
            let result = database.write_kvp(key, value).await;
            this.update(cx, |this, cx| {
                if generation != this.write_generation {
                    return;
                }
                this.error = result
                    .err()
                    .map(|error| format!("Review selection was not saved: {error}"));
                cx.notify();
            })
            .log_err();
        }));
    }
}

impl Focusable for BranchReviewPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
impl EventEmitter<PanelEvent> for BranchReviewPanel {}

impl Panel for BranchReviewPanel {
    fn persistent_name() -> &'static str {
        "BranchReviewPanel"
    }
    fn panel_key() -> &'static str {
        "branch_review"
    }
    fn position(&self, _: &Window, cx: &App) -> DockPosition {
        ReviewPanelSettings::get_global(cx).dock
    }
    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }
    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        if let Ok(fs) = self.workspace.read_with(cx, |workspace, cx| {
            workspace.project().read(cx).fs().clone()
        }) {
            settings::update_settings_file(fs, cx, move |settings, _| {
                settings.branch_review_panel.get_or_insert_default().dock = Some(position.into());
            });
        }
    }
    fn default_size(&self, _: &Window, cx: &App) -> Pixels {
        ReviewPanelSettings::get_global(cx).default_width
    }
    fn icon(&self, _: &Window, cx: &App) -> Option<IconName> {
        ReviewPanelSettings::get_global(cx)
            .button
            .then_some(IconName::PullRequest)
    }
    fn icon_tooltip(&self, _: &Window, _: &App) -> Option<&'static str> {
        Some("Branch Review")
    }
    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }
    fn activation_priority(&self) -> u32 {
        4
    }
    fn hide_button_setting(&self, _: &App) -> Option<workspace::HideStatusItem> {
        Some(workspace::HideStatusItem::new(|settings| {
            settings.branch_review_panel.get_or_insert_default().button = Some(false);
        }))
    }
}

impl Render for BranchReviewPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .track_focus(&self.focus_handle)
            .key_context("BranchReviewPanel")
            .bg(cx.theme().colors().panel_background)
            .child(
                h_flex()
                    .px_2()
                    .py_1()
                    .gap_1()
                    .child(Label::new("Branch Review").size(LabelSize::Small))
                    .child(div().flex_1())
                    .child(
                        IconButton::new("select-comparison", IconName::GitBranch)
                            .tooltip(Tooltip::text("Select a branch to compare"))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.ignored_item = None;
                                this.workspace
                                    .update(cx, |workspace, cx| {
                                        BranchDiff::compare_with_branch(
                                            workspace,
                                            &CompareWithBranch,
                                            window,
                                            cx,
                                        )
                                    })
                                    .log_err();
                            })),
                    )
                    .when(self.session.is_some(), |header| {
                        header.child(
                            IconButton::new("close-review", IconName::Close)
                                .tooltip(Tooltip::text("Close review"))
                                .on_click(cx.listener(|this, _, _, cx| this.close_review(cx))),
                        )
                    }),
            )
            .when_some(self.error.clone(), |view, error| {
                view.child(
                    div()
                        .p_2()
                        .child(Label::new(error).size(LabelSize::Small).color(Color::Error)),
                )
            })
            .when(self.github.read(cx).has_github(), |view| {
                view.child(
                    h_flex()
                        .px_2()
                        .gap_1()
                        .child(Button::new("review-files", "Files").on_click(cx.listener(
                            |this, _, _, cx| {
                                this.github.update(cx, |github, cx| github.show_files(cx));
                            },
                        )))
                        .child(
                            Button::new("review-discussion", "Discussion")
                                .disabled(self.github.read(cx).checkout.is_none())
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.github
                                        .update(cx, |github, cx| github.refresh_discussion(cx));
                                })),
                        )
                        .child(
                            Button::new("browse-github-prs", "PRs")
                                .disabled(
                                    self.checkout_task
                                        .as_ref()
                                        .is_some_and(|task| !task.is_ready()),
                                )
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.github.update(cx, |github, cx| github.show_browser(cx));
                                })),
                        ),
                )
            })
            .when(
                self.checkout_task
                    .as_ref()
                    .is_some_and(|task| !task.is_ready()),
                |view| view.child(Label::new("Preparing PR checkout…").size(LabelSize::Small)),
            )
            .map(|view| {
                if self.github.read(cx).is_visible() {
                    view.child(div().flex_1().min_h_0().child(self.github.clone()))
                } else if let Some(session) = &self.session {
                    view.child(
                        div().px_2().pb_1().child(
                            Label::new(format!("Base: {}", session.comparison.base_ref))
                                .size(LabelSize::Small),
                        ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_h_0()
                            .child(session.item.read(cx).review.clone()),
                    )
                } else {
                    view.child(
                        v_flex()
                            .p_3()
                            .gap_2()
                            .child(Label::new("Select a branch to compare").size(LabelSize::Small))
                            .child(
                                Button::new("choose-review-branch", "Select branch").on_click(
                                    cx.listener(|this, _, window, cx| {
                                        this.ignored_item = None;
                                        this.workspace
                                            .update(cx, |workspace, cx| {
                                                BranchDiff::compare_with_branch(
                                                    workspace,
                                                    &CompareWithBranch,
                                                    window,
                                                    cx,
                                                )
                                            })
                                            .log_err();
                                    }),
                                ),
                            ),
                    )
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use project::Project;
    use serde_json::json;
    use settings::SettingsStore;
    use std::path::Path;
    use util::path;
    use workspace::MultiWorkspace;

    #[gpui::test]
    async fn dock_retains_the_review_when_tabs_close_and_restores_validated_selection(
        cx: &mut TestAppContext,
    ) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            crate::init(cx);
        });
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({".git":{"logs":{"refs":{"heads":{"feature":"created\n"}}}},"a.txt":"changed"}),
        )
        .await;
        let git_dir = Path::new(path!("/project/.git"));
        fs.set_branch_name(git_dir, Some("feature"));
        fs.set_head_and_index_for_repo(git_dir, &[("a.txt", "changed".into())]);
        fs.set_merge_base_content_for_repo(git_dir, &[("a.txt", "base".into())]);
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |multi, _| multi.workspace().clone());
        let panel = workspace.update_in(cx, |workspace_ref, window, cx| {
            let workspace = cx.entity();
            let panel = cx.new(|cx| {
                BranchReviewPanel::new(
                    workspace,
                    workspace_ref,
                    Some("test-review-panel".into()),
                    Ok(None),
                    window,
                    cx,
                )
            });
            workspace_ref.add_panel(panel.clone(), window, cx);
            panel
        });
        let diff = cx
            .update(|window, cx| {
                BranchDiff::new_with_default_branch(project.clone(), workspace.clone(), window, cx)
            })
            .await
            .unwrap();
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(Box::new(diff.clone()), None, true, window, cx)
        });
        cx.run_until_parked();
        panel.read_with(cx, |panel, _| {
            assert_eq!(panel.session.as_ref().unwrap().item, diff)
        });
        let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());
        pane.update_in(cx, |pane, window, cx| {
            pane.remove_item(diff.entity_id(), false, false, window, cx)
        });
        cx.run_until_parked();
        panel.read_with(cx, |panel, _| {
            assert_eq!(panel.session.as_ref().unwrap().item, diff)
        });
        panel.update_in(cx, |panel, window, cx| panel.open_diff(window, cx));
        cx.run_until_parked();
        workspace.read_with(cx, |workspace, cx| {
            assert_eq!(
                workspace.active_item_as::<BranchDiff>(cx),
                Some(diff.clone())
            )
        });
        let saved = panel.read_with(cx, |panel, _| {
            panel.session.as_ref().unwrap().comparison.clone()
        });
        assert_eq!(saved.base_ref, "origin/main");
        panel.update(cx, |panel, cx| panel.close_review(cx));
        panel.update_in(cx, |panel, window, cx| panel.follow_review_item(window, cx));
        panel.read_with(cx, |panel, _| {
            assert!(
                panel.session.is_none(),
                "Closing review must not immediately attach the same tab"
            )
        });
        panel.update_in(cx, |panel, window, cx| {
            panel.pending_restore = Some(saved);
            panel.ignored_item = None;
            panel.reconcile_repository(window, cx);
            panel.follow_review_item(window, cx);
        });
        cx.run_until_parked();
        panel.read_with(cx, |panel, _| assert!(panel.session.is_some()));
        fs.insert_file(
            path!("/project/.git/logs/refs/heads/other"),
            b"other branch\n".to_vec(),
        )
        .await;
        fs.set_branch_name(git_dir, Some("other"));
        cx.run_until_parked();
        panel.update_in(cx, |panel, window, cx| {
            panel.reconcile_repository(window, cx)
        });
        panel.read_with(cx, |panel, _| assert!(panel.session.is_none()));
    }
}
