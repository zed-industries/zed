use acp_thread::{AcpThread, AcpThreadEvent, ThreadStatus};
use agent::{DbThreadMetadata, ThreadStore};
use agent_client_protocol as acp;
use agent_ui::acp::AcpThreadView;
use gpui::{
    App, Context, Entity, EventEmitter, FocusHandle, Focusable, Pixels, Render, SharedString,
    Subscription, Task, WeakEntity, Window, px,
};
use picker::{Picker, PickerDelegate};
use std::collections::HashMap;
use std::sync::Arc;
use theme::ActiveTheme;
use ui::utils::TRAFFIC_LIGHT_PADDING;
use ui::{KeyBinding, Tab, ThreadItem, Tooltip, prelude::*};
use ui_input::ErasedEditor;
use util::maybe;

use workspace::{
    FocusWorkspaceSidebar, MultiWorkspace, NewWorkspaceInWindow, Sidebar as WorkspaceSidebar,
    SidebarEvent, ToggleWorkspaceSidebar,
};

const DEFAULT_WIDTH: Pixels = px(320.0);
const MIN_WIDTH: Pixels = px(200.0);
const MAX_WIDTH: Pixels = px(800.0);

#[derive(Clone, Debug)]
pub struct AgentThreadInfo {
    pub title: SharedString,
    pub icon: IconName,
    pub running: bool,
    pub worktree_label: SharedString,
}

struct AgentThreadsPickerDelegate {
    thread_ids: Vec<acp::SessionId>,
    threads: HashMap<acp::SessionId, AgentThreadInfo>,
    historic_threads: Vec<DbThreadMetadata>,
    selected_index: usize,
}

impl AgentThreadsPickerDelegate {
    fn new(historic_threads: Vec<DbThreadMetadata>) -> Self {
        Self {
            thread_ids: Vec::new(),
            threads: HashMap::new(),
            historic_threads,
            selected_index: 0,
        }
    }
}

impl PickerDelegate for AgentThreadsPickerDelegate {
    type ListItem = AnyElement;

    fn match_count(&self) -> usize {
        self.thread_ids.len() + self.historic_threads.len()
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
        "Search threads…".into()
    }

    fn update_matches(
        &mut self,
        _query: String,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        if self.thread_ids.is_empty() {
            self.selected_index = 0;
        } else {
            self.selected_index = self.selected_index.min(self.thread_ids.len() - 1);
        }
        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn render_match(
        &self,
        index: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        maybe!({
            let session_id = self.thread_ids.get(index)?;
            let thread = self.threads.get(session_id)?;
            Some(
                ThreadItem::new(("agent-thread", index), thread.title.clone())
                    .icon(thread.icon)
                    .running(thread.running)
                    .selected(selected)
                    .worktree(thread.worktree_label.clone())
                    .into_any_element(),
            )
        })
        .or_else(|| {
            let historic_thread = self
                .historic_threads
                .get(index.saturating_sub(self.thread_ids.len()))?;

            Some(
                ThreadItem::new(
                    ("historic_agent_thread", index),
                    historic_thread.title.clone(),
                )
                .icon(IconName::ZedAgent)
                .selected(selected)
                .into_any_element(),
            )
        })
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
    picker: Entity<Picker<AgentThreadsPickerDelegate>>,
    _observe_threads: Subscription,
    _thread_subscriptions: Vec<Subscription>,
}

impl EventEmitter<SidebarEvent> for Sidebar {}

impl Sidebar {
    pub fn new(
        multi_workspace: Entity<MultiWorkspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let picker = cx.new(|cx| {
            let thread_store = ThreadStore::global(cx);
            let mut historic_threads: Vec<_> = thread_store.read(cx).entries().collect();
            historic_threads.sort_by(|this, other| this.updated_at.cmp(&other.updated_at));
            let delegate = AgentThreadsPickerDelegate::new(historic_threads);

            cx.observe(
                &thread_store,
                |picker: &mut Picker<AgentThreadsPickerDelegate>, thread_store, cx| {
                    let mut historic_threads: Vec<_> = thread_store.read(cx).entries().collect();
                    historic_threads.sort_by(|this, other| other.updated_at.cmp(&this.updated_at));

                    picker.delegate.historic_threads = historic_threads;
                },
            )
            .detach();

            Picker::list(delegate, window, cx)
                .max_height(None)
                .show_scrollbar(true)
                .modal(false)
        });

        let sidebar = cx.weak_entity();
        let observe_threads = cx.observe_new::<AcpThreadView>(move |thread_view, window, cx| {
            Self::observe_new_acp_thread_view(&sidebar, thread_view, window, cx);
        });

        Self {
            multi_workspace,
            width: DEFAULT_WIDTH,
            picker,
            _observe_threads: observe_threads,
            _thread_subscriptions: Vec::new(),
        }
    }

    fn observe_new_acp_thread_view(
        sidebar: &WeakEntity<Self>,
        thread_view: &mut AcpThreadView,
        window: Option<&mut Window>,
        cx: &mut Context<AcpThreadView>,
    ) {
        let icon = thread_view.agent_icon;
        let thread = thread_view.thread.read(cx);
        let session_id = thread.session_id().clone();
        let title = thread.title();
        let running = thread.status() == ThreadStatus::Generating;

        let worktree_label: SharedString = thread
            .project()
            .read(cx)
            .visible_worktrees(cx)
            .filter_map(|worktree| {
                worktree
                    .read(cx)
                    .abs_path()
                    .file_name()
                    .map(|name| name.to_string_lossy().to_string())
            })
            .collect::<Vec<_>>()
            .join(", ")
            .into();

        sidebar
            .update(cx, |sidebar, cx| {
                sidebar.picker.update(cx, |picker, cx| {
                    let info = AgentThreadInfo {
                        title,
                        icon,
                        running,
                        worktree_label,
                    };
                    picker.delegate.threads.insert(session_id.clone(), info);
                    picker.delegate.thread_ids.push(session_id);

                    if let Some(window) = window {
                        picker.refresh(window, cx);
                    }

                    cx.notify();
                });

                sidebar
                    ._thread_subscriptions
                    .push(cx.subscribe(&thread_view.thread, Self::on_thread_event));
            })
            .ok();
    }

    fn on_thread_event(
        &mut self,
        thread: Entity<AcpThread>,
        event: &AcpThreadEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            AcpThreadEvent::TitleUpdated => {
                let thread = thread.read(cx);
                let session_id = thread.session_id().clone();
                let title = thread.title();
                self.picker.update(cx, |picker, _cx| {
                    if let Some(info) = picker.delegate.threads.get_mut(&session_id) {
                        info.title = title;
                    }
                });
            }
            AcpThreadEvent::NewEntry
            | AcpThreadEvent::Stopped
            | AcpThreadEvent::Error
            | AcpThreadEvent::Retry(_) => {
                let thread = thread.read(cx);
                let session_id = thread.session_id().clone();
                let running = thread.status() == ThreadStatus::Generating;
                self.picker.update(cx, |picker, _cx| {
                    if let Some(info) = picker.delegate.threads.get_mut(&session_id) {
                        info.running = running;
                    }
                });
            }
            _ => {}
        }

        self.picker.update(cx, |_, cx| cx.notify());
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

    fn has_notifications(&self, _cx: &App) -> bool {
        false
    }
}

impl Focusable for Sidebar {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.read(cx).focus_handle(cx)
    }
}

impl Render for Sidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let titlebar_height = ui::utils::platform_title_bar_height(window);
        let ui_font = theme::setup_ui_font(window, cx);
        let is_focused = self.focus_handle(cx).is_focused(window);

        let focus_tooltip_label = if is_focused {
            "Focus Workspace"
        } else {
            "Focus Sidebar"
        };

        v_flex()
            .id("workspace-sidebar")
            .key_context("WorkspaceSidebar")
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
                    .pb_px()
                    .pr_1()
                    .when(cfg!(target_os = "macos"), |this| {
                        this.pl(px(TRAFFIC_LIGHT_PADDING))
                    })
                    .when(cfg!(not(target_os = "macos")), |this| this.pl_2())
                    .justify_between()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child({
                        let focus_handle = cx.focus_handle();
                        IconButton::new("close-sidebar", IconName::WorkspaceNavOpen)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::element(move |_, cx| {
                                v_flex()
                                    .gap_1()
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .justify_between()
                                            .child(Label::new("Close Sidebar"))
                                            .child(KeyBinding::for_action_in(
                                                &ToggleWorkspaceSidebar,
                                                &focus_handle,
                                                cx,
                                            )),
                                    )
                                    .child(
                                        h_flex()
                                            .pt_1()
                                            .gap_2()
                                            .border_t_1()
                                            .border_color(cx.theme().colors().border_variant)
                                            .justify_between()
                                            .child(Label::new(focus_tooltip_label))
                                            .child(KeyBinding::for_action_in(
                                                &FocusWorkspaceSidebar,
                                                &focus_handle,
                                                cx,
                                            )),
                                    )
                                    .into_any_element()
                            }))
                            .on_click(cx.listener(|_this, _, _window, cx| {
                                cx.emit(SidebarEvent::Close);
                            }))
                    })
                    .child(
                        IconButton::new("new-workspace", IconName::Plus)
                            .icon_size(IconSize::Small)
                            .tooltip(|_window, cx| {
                                Tooltip::for_action("New Workspace", &NewWorkspaceInWindow, cx)
                            })
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.multi_workspace.update(cx, |multi_workspace, cx| {
                                    multi_workspace.create_workspace(window, cx);
                                });
                            })),
                    ),
            )
            .child(self.picker.clone())
    }
}
