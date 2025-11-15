use anyhow::Result;
use git::repository::{FileHistory, FileHistoryEntry, RepoPath};
use gpui::{
    AnyElement, AnyView, App, Context, Entity, EventEmitter, FocusHandle, Focusable, IntoElement,
    ListSizingBehavior, Render, Task, UniformListScrollHandle, WeakEntity, Window, actions, rems,
    uniform_list,
};
use project::{
    Project, ProjectPath,
    git_store::{GitStore, Repository},
};
use std::any::{Any, TypeId};
use time::OffsetDateTime;
use ui::{Icon, IconName, Label, LabelCommon as _, SharedString, prelude::*};
use util::{ResultExt, truncate_and_trailoff};
use workspace::{
    Item, Workspace,
    item::{ItemEvent, SaveOptions},
    searchable::SearchableItemHandle,
};

use crate::commit_view::CommitView;

actions!(git, [ViewCommitFromHistory]);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(|_workspace, _: &ViewCommitFromHistory, _window, _cx| {});
    })
    .detach();
}

pub struct FileHistoryView {
    history: FileHistory,
    repository: WeakEntity<Repository>,
    workspace: WeakEntity<Workspace>,
    selected_entry: Option<usize>,
    scroll_handle: UniformListScrollHandle,
    focus_handle: FocusHandle,
}

impl FileHistoryView {
    pub fn open(
        path: RepoPath,
        git_store: WeakEntity<GitStore>,
        repo: WeakEntity<Repository>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let file_history_task = git_store
            .update(cx, |git_store, cx| {
                repo.upgrade()
                    .map(|repo| git_store.file_history(&repo, path.clone(), cx))
            })
            .ok()
            .flatten();

        window
            .spawn(cx, async move |cx| {
                let file_history = file_history_task?.await.log_err()?;
                let repo = repo.upgrade()?;

                workspace
                    .update_in(cx, |workspace, window, cx| {
                        let project = workspace.project();
                        let view = cx.new(|cx| {
                            FileHistoryView::new(
                                file_history,
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
                                let view = item.downcast::<FileHistoryView>();
                                view.is_some_and(|v| v.read(cx).history.path == path)
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
        history: FileHistory,
        repository: Entity<Repository>,
        workspace: WeakEntity<Workspace>,
        _project: Entity<Project>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let scroll_handle = UniformListScrollHandle::new();

        Self {
            history,
            repository: repository.downgrade(),
            workspace,
            selected_entry: None,
            scroll_handle,
            focus_handle,
        }
    }

    fn list_item_height(&self) -> Rems {
        rems(1.75)
    }

    fn render_commit_entry(
        &self,
        ix: usize,
        entry: &FileHistoryEntry,
        _window: &Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        let short_sha = if entry.sha.len() >= 7 {
            entry.sha[..7].to_string()
        } else {
            entry.sha.to_string()
        };

        let commit_time = OffsetDateTime::from_unix_timestamp(entry.commit_timestamp)
            .unwrap_or_else(|_| OffsetDateTime::UNIX_EPOCH);
        let relative_timestamp = time_format::format_localized_timestamp(
            commit_time,
            OffsetDateTime::now_utc(),
            time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC),
            time_format::TimestampFormat::Relative,
        );

        let selected = self.selected_entry == Some(ix);
        let sha = entry.sha.clone();
        let repo = self.repository.clone();
        let workspace = self.workspace.clone();
        let file_path = self.history.path.clone();

        let base_bg = if selected {
            cx.theme().status().info.alpha(0.15)
        } else {
            cx.theme().colors().element_background
        };

        let hover_bg = if selected {
            cx.theme().status().info.alpha(0.2)
        } else {
            cx.theme().colors().element_hover
        };

        h_flex()
            .id(("commit", ix))
            .h(self.list_item_height())
            .w_full()
            .items_center()
            .px(rems(0.75))
            .gap_2()
            .bg(base_bg)
            .hover(|style| style.bg(hover_bg))
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, window, cx| {
                this.selected_entry = Some(ix);
                cx.notify();

                // Open the commit view filtered to show only this file's changes
                if let Some(repo) = repo.upgrade() {
                    let sha_str = sha.to_string();
                    CommitView::open(
                        sha_str,
                        repo.downgrade(),
                        workspace.clone(),
                        None,
                        Some(file_path.clone()),
                        window,
                        cx,
                    );
                }
            }))
            .child(
                div()
                    .flex_none()
                    .w(rems(4.5))
                    .text_color(cx.theme().status().info)
                    .font_family(".SystemUIFontMonospaced-Regular")
                    .child(short_sha),
            )
            .child(
                Label::new(truncate_and_trailoff(&entry.subject, 60))
                    .single_line()
                    .color(ui::Color::Default),
            )
            .child(div().flex_1())
            .child(
                Label::new(truncate_and_trailoff(&entry.author_name, 20))
                    .size(LabelSize::Small)
                    .color(ui::Color::Muted)
                    .single_line(),
            )
            .child(
                div().flex_none().w(rems(6.5)).child(
                    Label::new(relative_timestamp)
                        .size(LabelSize::Small)
                        .color(ui::Color::Muted)
                        .single_line(),
                ),
            )
            .into_any_element()
    }
}

impl EventEmitter<ItemEvent> for FileHistoryView {}

impl Focusable for FileHistoryView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for FileHistoryView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let file_name = self.history.path.file_name().unwrap_or("File");
        let entry_count = self.history.entries.len();

        v_flex()
            .size_full()
            .child(
                h_flex()
                    .px(rems(0.75))
                    .py(rems(0.5))
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .bg(cx.theme().colors().title_bar_background)
                    .items_center()
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .child(
                                Icon::new(IconName::FileGit)
                                    .size(IconSize::Small)
                                    .color(ui::Color::Muted),
                            )
                            .child(
                                Label::new(format!("History: {}", file_name))
                                    .size(LabelSize::Default),
                            ),
                    )
                    .child(
                        Label::new(format!("{} commits", entry_count))
                            .size(LabelSize::Small)
                            .color(ui::Color::Muted),
                    ),
            )
            .child({
                let view = cx.weak_entity();
                uniform_list(
                    "file-history-list",
                    entry_count,
                    move |range, window, cx| {
                        let Some(view) = view.upgrade() else {
                            return Vec::new();
                        };
                        view.update(cx, |this, cx| {
                            let mut items = Vec::with_capacity(range.end - range.start);
                            for ix in range {
                                if let Some(entry) = this.history.entries.get(ix) {
                                    items.push(this.render_commit_entry(ix, entry, window, cx));
                                }
                            }
                            items
                        })
                    },
                )
                .flex_1()
                .size_full()
                .with_sizing_behavior(ListSizingBehavior::Auto)
                .track_scroll(self.scroll_handle.clone())
            })
    }
}

impl Item for FileHistoryView {
    type Event = ItemEvent;

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
        f(*event)
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        let file_name = self
            .history
            .path
            .file_name()
            .map(|name| name.to_string())
            .unwrap_or_else(|| "File".to_string());
        format!("History: {}", file_name).into()
    }

    fn tab_tooltip_text(&self, _cx: &App) -> Option<SharedString> {
        Some(format!("Git history for {}", self.history.path.as_unix_str()).into())
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::FileGit))
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("file history")
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<workspace::WorkspaceId>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>> {
        Task::ready(None)
    }

    fn navigate(&mut self, _: Box<dyn Any>, _window: &mut Window, _: &mut Context<Self>) -> bool {
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
        _theme: &theme::Theme,
        _cx: &App,
    ) -> Option<Vec<workspace::item::BreadcrumbText>> {
        None
    }

    fn added_to_workspace(
        &mut self,
        _workspace: &mut Workspace,
        window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        window.focus(&self.focus_handle);
    }

    fn show_toolbar(&self) -> bool {
        true
    }

    fn pixel_position_of_cursor(&self, _: &App) -> Option<gpui::Point<gpui::Pixels>> {
        None
    }

    fn as_searchable(&self, _: &Entity<Self>) -> Option<Box<dyn SearchableItemHandle>> {
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
        _type_id: TypeId,
        _self_handle: &'a Entity<Self>,
        _: &'a App,
    ) -> Option<AnyView> {
        None
    }
}
