use agent_ui::acp::{AcpServerView, AcpThreadHistory, ThreadActivityEvent};
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
    ToggleWorkspaceSidebar, WorkspaceId,
};

const DEFAULT_WIDTH: Pixels = px(320.0);
const MIN_WIDTH: Pixels = px(200.0);
const MAX_WIDTH: Pixels = px(800.0);
const MAX_MATCHES: usize = 100;

#[derive(Default)]
struct ThreadRuntimeState {
    is_running: bool,
    has_unread_changes: bool,
}

struct AgentThread {
    thread_id: Arc<str>,
    title: SharedString,
    workspace_id: Option<WorkspaceId>,
    worktree_paths: Vec<String>,
    runtime_state: ThreadRuntimeState,
}

impl AgentThread {
    fn worktree_label(&self) -> SharedString {
        if self.worktree_paths.is_empty() {
            return "No workspace".into();
        }
        let names: Vec<&str> = self
            .worktree_paths
            .iter()
            .filter_map(|path| std::path::Path::new(path).file_name()?.to_str())
            .collect();
        if names.is_empty() {
            "No workspace".into()
        } else {
            names.join(", ").into()
        }
    }
}

#[derive(Clone)]
struct ThreadEntry {
    index: usize,
    thread_id: Arc<str>,
    title: SharedString,
    worktree_label: SharedString,
    workspace_id: Option<WorkspaceId>,
    is_running: bool,
}

#[derive(Clone)]
struct ThreadMatch {
    entry: ThreadEntry,
    positions: Vec<usize>,
}

struct ThreadPickerDelegate {
    multi_workspace: Entity<MultiWorkspace>,
    entries: Vec<ThreadEntry>,
    matches: Vec<ThreadMatch>,
    selected_index: usize,
    query: String,
}

impl ThreadPickerDelegate {
    fn new(multi_workspace: Entity<MultiWorkspace>) -> Self {
        Self {
            multi_workspace,
            entries: Vec::new(),
            matches: Vec::new(),
            selected_index: 0,
            query: String::new(),
        }
    }

    fn set_entries(&mut self, entries: Vec<ThreadEntry>) {
        self.entries = entries;
    }
}

impl PickerDelegate for ThreadPickerDelegate {
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
        let previously_selected_thread_id = self
            .matches
            .get(self.selected_index)
            .map(|m| m.entry.thread_id.clone());

        if query.is_empty() {
            self.matches = entries
                .into_iter()
                .map(|entry| ThreadMatch {
                    entry,
                    positions: Vec::new(),
                })
                .collect();

            self.selected_index = previously_selected_thread_id
                .and_then(|id| self.matches.iter().position(|m| m.entry.thread_id == id))
                .unwrap_or(0);
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
                        Some(ThreadMatch {
                            entry,
                            positions: search_match.positions,
                        })
                    })
                    .collect::<Vec<_>>()
                })
                .await;

            this.update(cx, |this, _cx| {
                this.delegate.matches = matches;
                this.delegate.selected_index = previously_selected_thread_id
                    .and_then(|id| {
                        this.delegate
                            .matches
                            .iter()
                            .position(|m| m.entry.thread_id == id)
                    })
                    .unwrap_or(0);
            })
            .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(selected_match) = self.matches.get(self.selected_index) else {
            return;
        };
        let Some(workspace_id) = selected_match.entry.workspace_id else {
            return;
        };

        self.multi_workspace.update(cx, |multi_workspace, cx| {
            for (index, workspace) in multi_workspace.workspaces().iter().enumerate() {
                if workspace.read(cx).database_id() == Some(workspace_id) {
                    multi_workspace.activate_index(index, cx);
                    return;
                }
            }
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
        let ThreadMatch { entry, positions } = match_entry;

        let thread_item = ThreadItem::new(("thread-item", entry.index), entry.title.clone())
            .worktree(entry.worktree_label.clone())
            .running(entry.is_running)
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
    width: Pixels,
    active_threads: Vec<AgentThread>,
    picker: Entity<Picker<ThreadPickerDelegate>>,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<SidebarEvent> for Sidebar {}

impl Sidebar {
    pub fn new(
        multi_workspace: Entity<MultiWorkspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = ThreadPickerDelegate::new(multi_workspace);
        let picker = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx)
                .max_height(None)
                .show_scrollbar(true)
                .modal(false)
        });

        let weak_sidebar = cx.weak_entity();

        let observe_server_views = cx.observe_new::<AcpServerView>({
            let weak_sidebar = weak_sidebar.clone();
            move |_view, _window, cx| {
                let entity = cx.entity();
                weak_sidebar
                    .update(cx, |sidebar, cx| {
                        sidebar._subscriptions.push(cx.subscribe(
                            &entity,
                            |sidebar, _emitter, event, cx| {
                                sidebar.handle_thread_activity(event, cx);
                            },
                        ));
                    })
                    .ok();
            }
        });

        let observe_thread_history =
            cx.observe_new::<AcpThreadHistory>(move |_view, _window, cx| {
                let entity = cx.entity();
                weak_sidebar
                    .update(cx, |sidebar, cx| {
                        sidebar._subscriptions.push(cx.subscribe(
                            &entity,
                            |sidebar, _emitter, event, cx| {
                                sidebar.handle_thread_activity(event, cx);
                            },
                        ));
                    })
                    .ok();
            });

        Self {
            width: DEFAULT_WIDTH,
            active_threads: Vec::new(),
            picker,
            _subscriptions: vec![observe_server_views, observe_thread_history],
        }
    }

    fn handle_thread_activity(&mut self, event: &ThreadActivityEvent, cx: &mut Context<Self>) {
        match event {
            ThreadActivityEvent::MessageSent {
                thread_id,
                title,
                workspace_id,
                worktree_paths,
            } => {
                if let Some(position) = self.thread_position(thread_id) {
                    let thread = &mut self.active_threads[position];
                    thread.title = title.clone();
                    thread.workspace_id = *workspace_id;
                    thread.worktree_paths = worktree_paths.clone();
                    thread.runtime_state.is_running = true;
                    self.move_thread_to_front(position);
                } else {
                    self.active_threads.insert(
                        0,
                        AgentThread {
                            thread_id: thread_id.clone(),
                            title: title.clone(),
                            workspace_id: *workspace_id,
                            worktree_paths: worktree_paths.clone(),
                            runtime_state: ThreadRuntimeState {
                                is_running: true,
                                has_unread_changes: false,
                            },
                        },
                    );
                }
            }
            ThreadActivityEvent::Stopped { thread_id } => {
                if let Some(position) = self.thread_position(thread_id) {
                    self.active_threads[position].runtime_state.is_running = false;
                }
            }
            ThreadActivityEvent::Updated { thread_id } => {
                if let Some(position) = self.thread_position(thread_id) {
                    self.active_threads[position]
                        .runtime_state
                        .has_unread_changes = true;
                    self.move_thread_to_front(position);
                }
            }
            ThreadActivityEvent::Deleted { thread_id } => {
                if let Some(position) = self.thread_position(thread_id) {
                    self.active_threads.remove(position);
                }
            }
            ThreadActivityEvent::DeletedAll => {
                self.active_threads.clear();
            }
            ThreadActivityEvent::TitleChanged { thread_id, title } => {
                if let Some(position) = self.thread_position(thread_id) {
                    self.active_threads[position].title = title.clone();
                }
            }
        }

        self.refresh_picker(cx);
        cx.notify();
    }

    fn thread_position(&self, thread_id: &Arc<str>) -> Option<usize> {
        self.active_threads
            .iter()
            .position(|thread| thread.thread_id == *thread_id)
    }

    fn move_thread_to_front(&mut self, position: usize) {
        if position > 0 {
            let thread = self.active_threads.remove(position);
            self.active_threads.insert(0, thread);
        }
    }

    fn build_entries(&self) -> Vec<ThreadEntry> {
        self.active_threads
            .iter()
            .enumerate()
            .map(|(index, thread)| ThreadEntry {
                index,
                thread_id: thread.thread_id.clone(),
                title: thread.title.clone(),
                worktree_label: thread.worktree_label(),
                workspace_id: thread.workspace_id,
                is_running: thread.runtime_state.is_running,
            })
            .collect()
    }

    fn refresh_picker(&mut self, cx: &mut Context<Self>) {
        let entries = self.build_entries();
        self.picker.update(cx, |picker, _cx| {
            let previously_selected_thread_id = picker
                .delegate
                .matches
                .get(picker.delegate.selected_index)
                .map(|m| m.entry.thread_id.clone());

            picker.delegate.set_entries(entries.clone());
            picker.delegate.matches = entries
                .into_iter()
                .map(|entry| ThreadMatch {
                    entry,
                    positions: Vec::new(),
                })
                .collect();
            picker.delegate.selected_index = previously_selected_thread_id
                .and_then(|id| {
                    picker
                        .delegate
                        .matches
                        .iter()
                        .position(|m| m.entry.thread_id == id)
                })
                .unwrap_or(0);
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
