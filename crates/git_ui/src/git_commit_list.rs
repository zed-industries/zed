use crate::{commit_view::CommitView, git_panel::GitPanel};
use futures::Future;
use git::{GitRemote, blame::ParsedCommitMessage, repository::CommitSummary};
use gpui::{
    App, Asset, Element, Entity, ListHorizontalSizingBehavior, ListSizingBehavior, ParentElement,
    Render, Stateful, Task, UniformListScrollHandle, WeakEntity, prelude::*, uniform_list,
};
use project::{
    Project,
    git_store::{GitStoreEvent, Repository},
};
use std::hash::Hash;
use time::OffsetDateTime;
use ui::{Avatar, ListItem, Scrollbar, ScrollbarState, prelude::*};
use workspace::Workspace;

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

    expanded: bool,
    history: Vec<CommitDetails>,
    scroll_handle: UniformListScrollHandle,
    vertical_scrollbar_state: ScrollbarState,
    horizontal_scrollbar_state: ScrollbarState,
}

impl GitCommitList {
    pub fn new(workspace: &mut Workspace, window: &mut Window, cx: &mut App) -> Entity<Self> {
        let project = workspace.project().clone();
        let git_store = project.read(cx).git_store().clone();
        let active_repository = project.read(cx).active_repository(cx);

        cx.new(|cx| {
            cx.spawn_in(window, async move |this, cx| {
                let details = this.update(cx, |list: &mut GitCommitList, cx| {
                    list.load_commit_history(cx, 0, 50)
                })?;
                println!("Request history");

                let details = details.await?;

                let commit_details: Vec<crate::git_commit_list::CommitDetails> = details
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
                println!("Got history : {}", commit_details.len());

                this.update(cx, |this: &mut GitCommitList, cx| {
                    println!("Updating history : {}", commit_details.len());

                    this.history = commit_details;
                    cx.notify();
                })
            })
            .detach();

            cx.subscribe_in(
                &git_store,
                window,
                move |this, _git_store, event, window, cx| match event {
                    GitStoreEvent::ActiveRepositoryChanged(_) => {
                        this.active_repository = this.project.read(cx).active_repository(cx);
                    }
                    _ => {}
                },
            )
            .detach();

            let scroll_handle = UniformListScrollHandle::new();

            Self {
                history: Vec::new(),
                scroll_handle: scroll_handle.clone(),
                vertical_scrollbar_state: ScrollbarState::new(scroll_handle.clone())
                    .parent_entity(&cx.entity()),
                horizontal_scrollbar_state: ScrollbarState::new(scroll_handle.clone())
                    .parent_entity(&cx.entity()),
                expanded: false,
                active_repository,
                project,
                workspace: workspace.weak_handle(),
            }
        })
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
        window: &mut Window,
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
                            .text_sm()
                            .text_color(Color::Default.color(cx))
                            .child(commit_summary.subject.clone()),
                    )
                    .child(
                        h_flex()
                            .items_center()
                            .h_8()
                            .text_sm()
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

    fn render_vertical_scrollbar(&self, cx: &mut Context<Self>) -> Option<Stateful<Div>> {
        Some(
            div()
                .occlude()
                .id("project-panel-vertical-scroll")
                .on_mouse_move(cx.listener(|_, _, _, cx| {
                    cx.notify();
                    cx.stop_propagation()
                }))
                .on_hover(|_, _, cx| {
                    cx.stop_propagation();
                })
                .on_any_mouse_down(|_, _, cx| {
                    cx.stop_propagation();
                })
                .on_scroll_wheel(cx.listener(|_, _, _, cx| {
                    cx.notify();
                }))
                .h_full()
                .absolute()
                .right_1()
                .top_1()
                .bottom_0()
                .w(px(12.))
                .cursor_default()
                .children(Scrollbar::vertical(self.vertical_scrollbar_state.clone())),
        )
    }

    fn render_horizontal_scrollbar(
        &self,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Stateful<Div>> {
        Scrollbar::horizontal(self.horizontal_scrollbar_state.clone()).map(|scrollbar| {
            div()
                .occlude()
                .id("project-panel-horizontal-scroll")
                .on_mouse_move(cx.listener(|_, _, _, cx| {
                    cx.notify();
                    cx.stop_propagation()
                }))
                .on_hover(|_, _, cx| {
                    cx.stop_propagation();
                })
                .on_any_mouse_down(|_, _, cx| {
                    cx.stop_propagation();
                })
                .on_scroll_wheel(cx.listener(|_, _, _, cx| {
                    cx.notify();
                }))
                .w_full()
                .absolute()
                .right_1()
                .left_1()
                .bottom_0()
                .h(px(12.))
                .cursor_default()
                .child(scrollbar)
        })
    }
}

impl Render for GitCommitList {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let list_contents = uniform_list(
            "git_history",
            self.history.len(),
            cx.processor(move |panel, range, window, cx| {
                let history = panel.history.get(range);

                history
                    .map(|entries: &[CommitDetails]| entries.to_vec())
                    .unwrap_or_default()
                    .iter()
                    .map(|item| {
                        panel
                            .render_element(ElementId::Name(item.sha.clone()), item, window, cx)
                            .into_any_element()
                    })
                    .collect()
            }),
        )
        .with_sizing_behavior(ListSizingBehavior::Infer)
        .with_horizontal_sizing_behavior(ListHorizontalSizingBehavior::Unconstrained)
        .track_scroll(self.scroll_handle.clone());

        v_flex()
            .flex_shrink()
            .h_48()
            .w_full()
            .child(list_contents)
            .children(self.render_vertical_scrollbar(cx))
            .when_some(
                self.render_horizontal_scrollbar(window, cx),
                |this, scrollbar| this.pb_4().child(scrollbar),
            )
    }
}
