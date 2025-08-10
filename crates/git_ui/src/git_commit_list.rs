use crate::commit_view::CommitView;
use git::{blame::ParsedCommitMessage, repository::CommitSummary};
use gpui::{
    App, Entity, ListScrollEvent, ListState, ParentElement, Render, Task, WeakEntity, list,
    prelude::*,
};
use project::{
    Project,
    git_store::{GitStoreEvent, Repository},
};
use time::OffsetDateTime;
use ui::{ListItem, prelude::*};
use workspace::Workspace;

const COMMITS_PER_PAGE: u64 = 20;

#[derive(Clone, Debug)]
pub struct CommitDetails {
    pub sha: SharedString,
    pub author_name: SharedString,
    pub author_email: SharedString,
    pub commit_time: OffsetDateTime,
    pub message: Option<ParsedCommitMessage>,
}

pub struct GitCommitList {
    pub(crate) active_repository: Option<Entity<Repository>>,
    pub(crate) project: Entity<Project>,
    pub(crate) workspace: WeakEntity<Workspace>,

    commits: Vec<CommitDetails>,
    commits_loading: bool,

    commits_list: ListState,
}

impl GitCommitList {
    pub fn new(workspace: &mut Workspace, window: &mut Window, cx: &mut App) -> Entity<Self> {
        let project = workspace.project().clone();
        let git_store = project.read(cx).git_store().clone();
        let active_repository = project.read(cx).active_repository(cx);
        let workspace = workspace.weak_handle();

        cx.new(|cx| {
            cx.subscribe_in(
                &git_store,
                window,
                move |this: &mut GitCommitList, _git_store, event, _window, cx| match event {
                    GitStoreEvent::ActiveRepositoryChanged(_) => {
                        // TODO: Reset state and reload commits,
                        this.active_repository = this.project.read(cx).active_repository(cx);
                    }
                    _ => {}
                },
            )
            .detach();

            let commits_list = ListState::new(0, gpui::ListAlignment::Top, px(1000.));
            commits_list.set_scroll_handler(cx.listener(
                |this: &mut Self, event: &ListScrollEvent, window, cx| {
                    if event.visible_range.end >= this.commits.len() - 5 && this.has_next_page() {
                        this.load_next_history_page(window, cx);
                    }
                },
            ));

            let this = Self {
                active_repository,
                project,
                workspace,
                commits: Vec::new(),
                commits_loading: false,
                commits_list,
            };

            this.load_next_history_page(window, cx);

            this
        })
    }

    // We probabbly have a next page if the length of all pages matches the per page amount
    fn has_next_page(&self) -> bool {
        self.commits.len() % (COMMITS_PER_PAGE as usize) == 0
    }

    fn load_next_history_page(&self, window: &mut Window, cx: &mut Context<Self>) {
        // Skip if already loading a page
        if self.commits_loading {
            return;
        }

        let skip = self.commits.len() as u64;

        cx.spawn_in(window, async move |this, cx| {
            let details = this.update(cx, |list: &mut GitCommitList, cx| {
                list.commits_loading = true;
                list.load_commit_history(cx, skip, COMMITS_PER_PAGE)
            })?;

            let details = details.await?;

            let commits: Vec<crate::git_commit_list::CommitDetails> = details
                .into_iter()
                .map(|commit| CommitDetails {
                    sha: commit.sha.clone(),
                    author_name: commit.author_name.clone(),
                    author_email: commit.author_email.clone(),
                    commit_time: OffsetDateTime::from_unix_timestamp(commit.commit_timestamp)
                        // TODO: Handle properly
                        .unwrap(),
                    message: Some(ParsedCommitMessage {
                        message: commit.message.clone(),
                        ..Default::default()
                    }),
                })
                .collect();

            this.update(cx, |this: &mut GitCommitList, cx| {
                this.commits_loading = false;
                this.commits.extend(commits);
                this.commits_list
                    .splice(0..this.commits_list.item_count(), this.commits.len());
                cx.notify();
            })
        })
        .detach();
    }

    fn load_commit_history(
        &self,
        cx: &mut Context<Self>,
        skip: u64,
        max_count: u64,
    ) -> Task<anyhow::Result<Vec<git::repository::CommitDetails>>> {
        let Some(repo) = self.active_repository.clone() else {
            return Task::ready(Err(anyhow::anyhow!("no active repo")));
        };
        repo.update(cx, |repo, cx| {
            let git_log = repo.git_log(skip, max_count);
            cx.spawn(async move |_, _| git_log.await?)
        })
    }

    fn render_element(
        &self,
        item_id: ElementId,
        commit: &CommitDetails,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let commit_summary = CommitSummary {
            sha: commit.sha.clone(),
            subject: commit
                .message
                .as_ref()
                .map_or(Default::default(), |message| {
                    message
                        .message
                        .split('\n')
                        .next()
                        .unwrap()
                        .trim_end()
                        .to_string()
                        .into()
                }),
            commit_timestamp: commit.commit_time.unix_timestamp(),
            has_parent: false,
        };

        ListItem::new(item_id)
            .child(
                h_flex()
                    .child(
                        h_flex()
                            .items_center()
                            .h_8()
                            .text_xs()
                            .text_color(Color::Default.color(cx))
                            .child(commit_summary.subject.clone()),
                    )
                    .child(
                        h_flex()
                            .items_center()
                            .h_8()
                            .text_xs()
                            .text_color(Color::Hidden.color(cx))
                            .child(commit.author_name.clone())
                            .ml_1(),
                    ),
            )
            .on_click({
                let commit = commit_summary.clone();
                let workspace = self.workspace.clone();
                let repo = self.active_repository.as_ref().map(|repo| repo.downgrade());
                move |_, window, cx| {
                    let repo = match repo.clone() {
                        Some(repo) => repo,
                        None => return,
                    };
                    CommitView::open(
                        commit.clone(),
                        repo.clone(),
                        workspace.clone().clone(),
                        window,
                        cx,
                    );
                }
            })
    }
}

impl Render for GitCommitList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .border_t_1()
            .border_color(cx.theme().colors().border.opacity(0.8))
            .flex_shrink()
            .h_48()
            .w_full()
            .child(
                list(
                    self.commits_list.clone(),
                    cx.processor(move |list, index, window, cx| {
                        let item: &CommitDetails = &list.commits[index];

                        list.render_element(ElementId::Name(item.sha.clone()), item, window, cx)
                            .into_any_element()
                    }),
                )
                .size_full(),
            )
    }
}
