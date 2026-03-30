use anyhow::Result;

use git::repository::FileHistoryEntry;
use git::{GitHostingProviderRegistry, GitRemote, parse_git_remote_url};
use gpui::{
    AnyElement, AnyEntity, App, Context, Entity, EventEmitter, FocusHandle, Focusable, IntoElement,
    Render, ScrollStrategy, Task, UniformListScrollHandle, WeakEntity, Window, uniform_list,
};
use project::{
    Project, ProjectPath,
    git_store::{GitStore, Repository},
};
use std::any::{Any, TypeId};
use std::sync::Arc;

use time::OffsetDateTime;
use ui::{Chip, ListItem, WithScrollbar, prelude::*};
use util::ResultExt;
use workspace::{
    Item, Workspace,
    item::{ItemEvent, SaveOptions},
};

use crate::commit_tooltip::CommitAvatar;
use crate::commit_view::CommitView;

const PAGE_SIZE: usize = 50;
const LOADING_THRESHOLD: usize = 10;

pub struct CommitLogView {
    entries: Vec<FileHistoryEntry>,
    repository: WeakEntity<Repository>,
    git_store: WeakEntity<GitStore>,
    workspace: WeakEntity<Workspace>,
    remote: Option<GitRemote>,
    selected_entry: Option<usize>,
    scroll_handle: UniformListScrollHandle,
    focus_handle: FocusHandle,
    loading_more: bool,
    has_more: bool,
}

impl CommitLogView {
    pub fn open(
        git_store: WeakEntity<GitStore>,
        repo: WeakEntity<Repository>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let commit_log_task = git_store
            .update(cx, |git_store, cx| {
                repo.upgrade().map(|repo| {
                    git_store.commit_log_paginated(&repo, 0, Some(PAGE_SIZE), cx)
                })
            })
            .ok()
            .flatten();

        window
            .spawn(cx, async move |cx| {
                let commit_log = commit_log_task?.await.log_err()?;
                let repo = repo.upgrade()?;

                workspace
                    .update_in(cx, |workspace, window, cx| {
                        let project = workspace.project();
                        let view = cx.new(|cx| {
                            CommitLogView::new(
                                commit_log.entries,
                                git_store.clone(),
                                repo.clone(),
                                workspace.weak_handle(),
                                project.clone(),
                                window,
                                cx,
                            )
                        });

                        let pane = workspace.active_pane();
                        pane.update(cx, |pane, cx| {
                            let ix = pane.items().position(|item| {
                                item.downcast::<CommitLogView>().is_some()
                            });
                            if let Some(ix) = ix {
                                pane.activate_item(ix, true, true, window, cx);
                            } else {
                                pane.add_item(Box::new(view), true, true, None, window, cx);
                            }
                        })
                    })
                    .log_err()
            })
            .detach();
    }

    fn new(
        entries: Vec<FileHistoryEntry>,
        git_store: WeakEntity<GitStore>,
        repository: Entity<Repository>,
        workspace: WeakEntity<Workspace>,
        _project: Entity<Project>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let scroll_handle = UniformListScrollHandle::new();
        let has_more = entries.len() >= PAGE_SIZE;

        let snapshot = repository.read(cx).snapshot();
        let remote_url = snapshot
            .remote_upstream_url
            .as_ref()
            .or(snapshot.remote_origin_url.as_ref());

        let remote = remote_url.and_then(|url| {
            let provider_registry = GitHostingProviderRegistry::default_global(cx);
            parse_git_remote_url(provider_registry, url).map(|(host, parsed)| GitRemote {
                host,
                owner: parsed.owner.into(),
                repo: parsed.repo.into(),
            })
        });

        Self {
            entries,
            git_store,
            repository: repository.downgrade(),
            workspace,
            remote,
            selected_entry: None,
            scroll_handle,
            focus_handle,
            loading_more: false,
            has_more,
        }
    }

    fn load_more(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.loading_more || !self.has_more {
            return;
        }

        self.loading_more = true;
        cx.notify();

        let current_count = self.entries.len();
        let git_store = self.git_store.clone();
        let repo = self.repository.clone();

        let this = cx.weak_entity();
        let task = window.spawn(cx, async move |cx| {
            let commit_log_task = git_store
                .update(cx, |git_store, cx| {
                    repo.upgrade().map(|repo| {
                        git_store.commit_log_paginated(
                            &repo,
                            current_count,
                            Some(PAGE_SIZE),
                            cx,
                        )
                    })
                })
                .ok()
                .flatten();

            if let Some(task) = commit_log_task {
                if let Ok(more_log) = task.await {
                    this.update(cx, |this, cx| {
                        this.loading_more = false;
                        this.has_more = more_log.entries.len() >= PAGE_SIZE;
                        this.entries.extend(more_log.entries);
                        cx.notify();
                    })
                    .ok();
                }
            }
        });

        task.detach();
    }

    fn select_next(&mut self, _: &menu::SelectNext, _: &mut Window, cx: &mut Context<Self>) {
        let entry_count = self.entries.len();
        let ix = match self.selected_entry {
            _ if entry_count == 0 => None,
            None => Some(0),
            Some(ix) => {
                if ix == entry_count - 1 {
                    Some(0)
                } else {
                    Some(ix + 1)
                }
            }
        };
        self.select_ix(ix, cx);
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let entry_count = self.entries.len();
        let ix = match self.selected_entry {
            _ if entry_count == 0 => None,
            None => Some(entry_count - 1),
            Some(ix) => {
                if ix == 0 {
                    Some(entry_count - 1)
                } else {
                    Some(ix - 1)
                }
            }
        };
        self.select_ix(ix, cx);
    }

    fn select_first(&mut self, _: &menu::SelectFirst, _: &mut Window, cx: &mut Context<Self>) {
        let entry_count = self.entries.len();
        let ix = if entry_count != 0 { Some(0) } else { None };
        self.select_ix(ix, cx);
    }

    fn select_last(&mut self, _: &menu::SelectLast, _: &mut Window, cx: &mut Context<Self>) {
        let entry_count = self.entries.len();
        let ix = if entry_count != 0 {
            Some(entry_count - 1)
        } else {
            None
        };
        self.select_ix(ix, cx);
    }

    fn select_ix(&mut self, ix: Option<usize>, cx: &mut Context<Self>) {
        self.selected_entry = ix;
        if let Some(ix) = ix {
            self.scroll_handle.scroll_to_item(ix, ScrollStrategy::Top);
        }
        cx.notify();
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        self.open_commit_view(window, cx);
    }

    fn open_commit_view(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(entry) = self
            .selected_entry
            .and_then(|ix| self.entries.get(ix))
        else {
            return;
        };

        if let Some(repo) = self.repository.upgrade() {
            let sha_str = entry.sha.to_string();
            CommitView::open(
                sha_str,
                repo.downgrade(),
                self.workspace.clone(),
                None,
                None,
                window,
                cx,
            );
        }
    }

    fn render_commit_avatar(
        &self,
        sha: &SharedString,
        author_email: Option<SharedString>,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        CommitAvatar::new(sha, author_email, self.remote.as_ref())
            .size(rems_from_px(20.))
            .render(window, cx)
    }

    fn render_commit_entry(
        &self,
        ix: usize,
        entry: &FileHistoryEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let pr_number = entry
            .subject
            .rfind("(#")
            .and_then(|start| {
                let rest = &entry.subject[start + 2..];
                rest.find(')')
                    .and_then(|end| rest[..end].parse::<u32>().ok())
            })
            .map(|num| format!("#{}", num))
            .unwrap_or_else(|| {
                if entry.sha.len() >= 7 {
                    entry.sha[..7].to_string()
                } else {
                    entry.sha.to_string()
                }
            });

        let commit_time = OffsetDateTime::from_unix_timestamp(entry.commit_timestamp)
            .unwrap_or_else(|_| OffsetDateTime::UNIX_EPOCH);
        let relative_timestamp = time_format::format_localized_timestamp(
            commit_time,
            OffsetDateTime::now_utc(),
            time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC),
            time_format::TimestampFormat::Relative,
        );

        ListItem::new(("commit", ix))
            .toggle_state(Some(ix) == self.selected_entry)
            .child(
                h_flex()
                    .h_8()
                    .w_full()
                    .pl_0p5()
                    .pr_2p5()
                    .gap_2()
                    .child(
                        div()
                            .w(rems_from_px(52.))
                            .flex_none()
                            .child(Chip::new(pr_number)),
                    )
                    .child(self.render_commit_avatar(
                        &entry.sha,
                        Some(entry.author_email.clone()),
                        window,
                        cx,
                    ))
                    .child(
                        h_flex()
                            .min_w_0()
                            .w_full()
                            .justify_between()
                            .child(
                                h_flex()
                                    .min_w_0()
                                    .w_full()
                                    .gap_1()
                                    .child(
                                        Label::new(entry.author_name.clone())
                                            .size(LabelSize::Small)
                                            .color(Color::Default)
                                            .truncate(),
                                    )
                                    .child(
                                        Label::new(&entry.subject)
                                            .size(LabelSize::Small)
                                            .color(Color::Muted)
                                            .truncate(),
                                    ),
                            )
                            .child(
                                h_flex().flex_none().child(
                                    Label::new(relative_timestamp)
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                ),
                            ),
                    ),
            )
            .on_click(cx.listener(move |this, _, window, cx| {
                this.selected_entry = Some(ix);
                cx.notify();

                this.open_commit_view(window, cx);
            }))
            .into_any_element()
    }
}

impl EventEmitter<ItemEvent> for CommitLogView {}

impl Focusable for CommitLogView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CommitLogView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entry_count = self.entries.len();

        v_flex()
            .id("commit_log_view")
            .key_context("CommitLogView")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::confirm))
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(
                h_flex()
                    .h(rems_from_px(41.))
                    .pl_3()
                    .pr_2()
                    .justify_between()
                    .border_b_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(
                        Label::new("Commit Log")
                            .color(Color::Muted),
                    )
                    .child(
                        Label::new(format!("{} commits", entry_count))
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .child(
                v_flex()
                    .flex_1()
                    .size_full()
                    .child({
                        let view = cx.weak_entity();
                        uniform_list(
                            "commit-log-list",
                            entry_count,
                            move |range, window, cx| {
                                let Some(view) = view.upgrade() else {
                                    return Vec::new();
                                };
                                view.update(cx, |this, cx| {
                                    if range.end + LOADING_THRESHOLD >= this.entries.len() {
                                        this.load_more(window, cx);
                                    }
                                    let mut items = Vec::with_capacity(range.end - range.start);
                                    for ix in range {
                                        if let Some(entry) = this.entries.get(ix) {
                                            items.push(
                                                this.render_commit_entry(ix, entry, window, cx),
                                            );
                                        }
                                    }
                                    items
                                })
                            },
                        )
                        .flex_1()
                        .size_full()
                        .track_scroll(&self.scroll_handle)
                    })
                    .vertical_scrollbar_for(&self.scroll_handle, window, cx),
            )
    }
}

impl Item for CommitLogView {
    type Event = ItemEvent;

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(ItemEvent)) {
        f(*event)
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Commit Log".into()
    }

    fn tab_tooltip_text(&self, _cx: &App) -> Option<SharedString> {
        Some("Git commit log".into())
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::GitBranch))
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("commit log")
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<workspace::WorkspaceId>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>> {
        Task::ready(None)
    }

    fn navigate(
        &mut self,
        _: Arc<dyn Any + Send>,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> bool {
        false
    }

    fn deactivated(&mut self, _window: &mut Window, _: &mut Context<Self>) {}

    fn can_save(&self, _: &App) -> bool {
        false
    }

    fn save(
        &mut self,
        _options: SaveOptions,
        _project: Entity<Project>,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn save_as(
        &mut self,
        _project: Entity<Project>,
        _path: ProjectPath,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn reload(
        &mut self,
        _project: Entity<Project>,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn is_dirty(&self, _: &App) -> bool {
        false
    }

    fn has_conflict(&self, _: &App) -> bool {
        false
    }

    fn breadcrumbs(
        &self,
        _cx: &App,
    ) -> Option<(Vec<workspace::item::HighlightedText>, Option<gpui::Font>)> {
        None
    }

    fn added_to_workspace(
        &mut self,
        _workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus(&self.focus_handle, cx);
    }

    fn show_toolbar(&self) -> bool {
        true
    }

    fn pixel_position_of_cursor(&self, _: &App) -> Option<gpui::Point<gpui::Pixels>> {
        None
    }

    fn set_nav_history(
        &mut self,
        _: workspace::ItemNavHistory,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) {
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        _: &'a App,
    ) -> Option<AnyEntity> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.clone().into())
        } else {
            None
        }
    }
}
