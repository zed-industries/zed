use fuzzy::StringMatchCandidate;
use gpui::{
    Action, App, Context, Entity, EventEmitter, Pixels, Render, SharedString, Subscription, Task,
    Window, px,
};
use picker::{Picker, PickerDelegate};
use std::sync::Arc;
use theme::ActiveTheme;
use ui::utils::TRAFFIC_LIGHT_PADDING;
use ui::{Tab, ThreadItem, Tooltip, prelude::*};
use ui_input::ErasedEditor;
use util::ResultExt as _;
use workspace::{
    MultiWorkspace, NewWorkspaceInWindow, Sidebar as WorkspaceSidebar, SidebarEvent,
    ToggleWorkspaceSidebar, Workspace,
};

const DEFAULT_WIDTH: Pixels = px(320.0);
const MIN_WIDTH: Pixels = px(200.0);
const MAX_WIDTH: Pixels = px(800.0);
const DEFAULT_THREAD_TITLE: &str = "The Last Thread Title Here"; // TODO: Delete this after when pulling from db
const DEFAULT_THREAD_TIMESTAMP: &str = "12:10 AM"; // TODO: Delete this after when pulling from db
const MAX_MATCHES: usize = 100;

#[derive(Clone)]
struct WorkspaceThreadEntry {
    index: usize,
    title: SharedString,
    timestamp: SharedString,
    worktree_label: SharedString,
}

#[derive(Clone)]
struct WorkspaceThreadMatch {
    entry: WorkspaceThreadEntry,
    positions: Vec<usize>,
}

impl WorkspaceThreadEntry {
    fn new(index: usize, workspace: &Entity<Workspace>, cx: &App) -> Self {
        let workspace_ref = workspace.read(cx);

        let worktree_names: Vec<String> = workspace_ref
            .worktrees(cx)
            .filter_map(|worktree| {
                worktree
                    .read(cx)
                    .abs_path()
                    .file_name()
                    .map(|name| name.to_string_lossy().to_string())
            })
            .collect();

        let worktree_label: SharedString = if worktree_names.is_empty() {
            format!("Workspace {}", index + 1).into()
        } else {
            worktree_names.join(", ").into()
        };

        Self {
            index,
            title: SharedString::new_static(DEFAULT_THREAD_TITLE),
            timestamp: SharedString::new_static(DEFAULT_THREAD_TIMESTAMP),
            worktree_label,
        }
    }
}

struct WorkspacePickerDelegate {
    multi_workspace: Entity<MultiWorkspace>,
    entries: Vec<WorkspaceThreadEntry>,
    active_workspace_index: usize,
    matches: Vec<WorkspaceThreadMatch>,
    selected_index: usize,
    query: String,
}

impl WorkspacePickerDelegate {
    fn new(multi_workspace: Entity<MultiWorkspace>) -> Self {
        Self {
            multi_workspace,
            entries: Vec::new(),
            active_workspace_index: 0,
            matches: Vec::new(),
            selected_index: 0,
            query: String::new(),
        }
    }

    fn set_entries(&mut self, entries: Vec<WorkspaceThreadEntry>, active_workspace_index: usize) {
        self.entries = entries;
        self.active_workspace_index = active_workspace_index;
    }
}

impl PickerDelegate for WorkspacePickerDelegate {
    type ListItem = ThreadItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        index: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = index;
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search…".into()
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        if self.query.is_empty() {
            None
        } else {
            Some("No threads match your search.".into())
        }
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let query = query.trim().to_string();
        self.query = query.clone();

        let entries = self.entries.clone();
        if query.is_empty() {
            self.matches = entries
                .into_iter()
                .map(|entry| WorkspaceThreadMatch {
                    entry,
                    positions: Vec::new(),
                })
                .collect();

            self.selected_index = self
                .active_workspace_index
                .min(self.matches.len().saturating_sub(1));
            return Task::ready(());
        }

        let executor = cx.background_executor().clone();

        cx.spawn(async move |this, cx| {
            let matches = cx
                .background_spawn(async move {
                    let candidates: Vec<StringMatchCandidate> = entries
                        .iter()
                        .enumerate()
                        .map(|(index, entry)| {
                            StringMatchCandidate::new(index, entry.title.as_ref())
                        })
                        .collect();

                    fuzzy::match_strings(
                        &candidates,
                        &query,
                        false,
                        true,
                        MAX_MATCHES,
                        &Default::default(),
                        executor,
                    )
                    .await
                    .into_iter()
                    .filter_map(|search_match| {
                        let entry = entries.get(search_match.candidate_id)?.clone();
                        Some(WorkspaceThreadMatch {
                            entry,
                            positions: search_match.positions,
                        })
                    })
                    .collect::<Vec<_>>()
                })
                .await;

            this.update(cx, |this, _cx| {
                this.delegate.matches = matches;
                this.delegate.selected_index = 0;
            })
            .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(selected_match) = self.matches.get(self.selected_index) else {
            return;
        };

        let target_index = selected_match.entry.index;

        self.multi_workspace.update(cx, |multi_workspace, cx| {
            multi_workspace.activate_index(target_index, cx);
        });
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn render_match(
        &self,
        index: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let match_entry = self.matches.get(index)?;
        let WorkspaceThreadMatch { entry, positions } = match_entry;

        let thread_item = ThreadItem::new(("workspace-item", entry.index), entry.title.clone())
            .timestamp(entry.timestamp.clone())
            .worktree(entry.worktree_label.clone())
            .selected(selected);

        let thread_item = if positions.is_empty() {
            thread_item
        } else {
            thread_item.highlight_positions(positions.clone())
        };

        Some(thread_item)
    }

    fn render_editor(
        &self,
        editor: &Arc<dyn ErasedEditor>,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Div {
        h_flex()
            .h(Tab::container_height(cx))
            .w_full()
            .px_2()
            .gap_2()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                Icon::new(IconName::MagnifyingGlass)
                    .color(Color::Muted)
                    .size(IconSize::Small),
            )
            .child(editor.render(window, cx))
    }
}

pub struct Sidebar {
    multi_workspace: Entity<MultiWorkspace>,
    width: Pixels,
    picker: Entity<Picker<WorkspacePickerDelegate>>,
    _subscription: Subscription,
}

impl EventEmitter<SidebarEvent> for Sidebar {}

impl Sidebar {
    pub fn new(
        multi_workspace: Entity<MultiWorkspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = WorkspacePickerDelegate::new(multi_workspace.clone());
        let picker = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx)
                .max_height(None)
                .show_scrollbar(true)
                .modal(false)
        });

        let subscription = cx.observe_in(
            &multi_workspace,
            window,
            |this, multi_workspace, window, cx| {
                this.queue_refresh(multi_workspace.clone(), window, cx);
            },
        );

        let mut this = Self {
            multi_workspace,
            width: DEFAULT_WIDTH,
            picker,
            _subscription: subscription,
        };
        this.queue_refresh(this.multi_workspace.clone(), window, cx);
        this
    }

    fn build_entries(
        multi_workspace: &MultiWorkspace,
        cx: &App,
    ) -> (Vec<WorkspaceThreadEntry>, usize) {
        let entries = multi_workspace
            .workspaces()
            .iter()
            .enumerate()
            .map(|(index, workspace)| WorkspaceThreadEntry::new(index, workspace, cx))
            .collect();
        (entries, multi_workspace.active_workspace_index())
    }

    fn queue_refresh(
        &mut self,
        multi_workspace: Entity<MultiWorkspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.defer_in(window, move |this, window, cx| {
            let (entries, active_index) = multi_workspace.read_with(cx, |multi_workspace, cx| {
                Self::build_entries(multi_workspace, cx)
            });
            this.picker.update(cx, |picker, cx| {
                picker.delegate.set_entries(entries, active_index);
                let query = picker.query(cx);
                picker.update_matches(query, window, cx);
            });
        });
    }
}

impl WorkspaceSidebar for Sidebar {
    fn width(&self, _cx: &App) -> Pixels {
        self.width
    }

    fn set_width(&mut self, width: Option<Pixels>, cx: &mut Context<Self>) {
        self.width = width.unwrap_or(DEFAULT_WIDTH).clamp(MIN_WIDTH, MAX_WIDTH);
        cx.notify();
    }
}

impl Render for Sidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let titlebar_height = ui::utils::platform_title_bar_height(window);
        let ui_font = theme::setup_ui_font(window, cx);

        v_flex()
            .id("workspace-sidebar")
            .font(ui_font)
            .h_full()
            .w(self.width)
            .bg(cx.theme().colors().surface_background)
            .border_r_1()
            .border_color(cx.theme().colors().border)
            .child(
                h_flex()
                    .flex_none()
                    .h(titlebar_height)
                    .w_full()
                    .mt_px()
                    .pr_2()
                    .when(cfg!(target_os = "macos"), |this| {
                        this.pl(px(TRAFFIC_LIGHT_PADDING))
                    })
                    .justify_between()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        IconButton::new("close-sidebar", IconName::WorkspaceNavOpen)
                            .icon_size(IconSize::Small)
                            .tooltip(|_window, cx| {
                                Tooltip::for_action("Close Sidebar", &ToggleWorkspaceSidebar, cx)
                            })
                            .on_click(cx.listener(|_this, _, _window, cx| {
                                cx.emit(SidebarEvent::Close);
                            })),
                    )
                    .child(
                        IconButton::new("new-workspace", IconName::Plus)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text("New Workspace"))
                            .on_click(cx.listener(|_this, _, window, cx| {
                                window.dispatch_action(NewWorkspaceInWindow.boxed_clone(), cx);
                            })),
                    ),
            )
            .child(self.picker.clone())
    }
}
