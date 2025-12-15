use agent::{HistoryEntry, HistoryEntryId, HistoryStore, NativeAgentServer};
use agent_servers::AgentServer;
use agent_settings::AgentSettings;
use agent_ui::acp::AcpThreadView;
use fs::Fs;
use gpui::{
    Entity, EventEmitter, Focusable, Pixels, SharedString, Subscription, WeakEntity, prelude::*,
};
use project::Project;
use prompt_store::PromptStore;
use serde::{Deserialize, Serialize};
use settings::DockSide;
use settings::Settings as _;
use std::rc::Rc;
use std::sync::Arc;
use ui::{
    App, Clickable as _, Context, DynamicSpacing, IconButton, IconName, IconSize, IntoElement,
    Label, LabelCommon as _, LabelSize, Render, Tab, Window, div,
};
use workspace::Workspace;
use workspace::dock::{ClosePane, MinimizePane, UtilityPane, UtilityPanePosition};
use workspace::utility_pane::UtilityPaneSlot;

pub const DEFAULT_UTILITY_PANE_WIDTH: Pixels = gpui::px(400.0);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SerializedHistoryEntryId {
    AcpThread(String),
    TextThread(String),
}

impl From<HistoryEntryId> for SerializedHistoryEntryId {
    fn from(id: HistoryEntryId) -> Self {
        match id {
            HistoryEntryId::AcpThread(session_id) => {
                SerializedHistoryEntryId::AcpThread(session_id.0.to_string())
            }
            HistoryEntryId::TextThread(path) => {
                SerializedHistoryEntryId::TextThread(path.to_string_lossy().to_string())
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializedAgentThreadPane {
    pub expanded: bool,
    pub width: Option<Pixels>,
    pub thread_id: Option<SerializedHistoryEntryId>,
}

pub enum AgentsUtilityPaneEvent {
    StateChanged,
}

impl EventEmitter<AgentsUtilityPaneEvent> for AgentThreadPane {}
impl EventEmitter<MinimizePane> for AgentThreadPane {}
impl EventEmitter<ClosePane> for AgentThreadPane {}

struct ActiveThreadView {
    view: Entity<AcpThreadView>,
    thread_id: HistoryEntryId,
    _notify: Subscription,
}

pub struct AgentThreadPane {
    focus_handle: gpui::FocusHandle,
    expanded: bool,
    width: Option<Pixels>,
    thread_view: Option<ActiveThreadView>,
    workspace: WeakEntity<Workspace>,
}

impl AgentThreadPane {
    pub fn new(workspace: WeakEntity<Workspace>, cx: &mut ui::Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        Self {
            focus_handle,
            expanded: false,
            width: None,
            thread_view: None,
            workspace,
        }
    }

    pub fn thread_id(&self) -> Option<HistoryEntryId> {
        self.thread_view.as_ref().map(|tv| tv.thread_id.clone())
    }

    pub fn serialize(&self) -> SerializedAgentThreadPane {
        SerializedAgentThreadPane {
            expanded: self.expanded,
            width: self.width,
            thread_id: self.thread_id().map(SerializedHistoryEntryId::from),
        }
    }

    pub fn open_thread(
        &mut self,
        entry: HistoryEntry,
        fs: Arc<dyn Fs>,
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        history_store: Entity<HistoryStore>,
        prompt_store: Option<Entity<PromptStore>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let thread_id = entry.id();

        let resume_thread = match &entry {
            HistoryEntry::AcpThread(thread) => Some(thread.clone()),
            HistoryEntry::TextThread(_) => None,
        };

        let agent: Rc<dyn AgentServer> = Rc::new(NativeAgentServer::new(fs, history_store.clone()));

        let thread_view = cx.new(|cx| {
            AcpThreadView::new(
                agent,
                resume_thread,
                None,
                workspace,
                project,
                history_store,
                prompt_store,
                true,
                window,
                cx,
            )
        });

        let notify = cx.observe(&thread_view, |_, _, cx| {
            cx.notify();
        });

        self.thread_view = Some(ActiveThreadView {
            view: thread_view,
            thread_id,
            _notify: notify,
        });

        cx.notify();
    }

    fn title(&self, cx: &App) -> SharedString {
        if let Some(active_thread_view) = &self.thread_view {
            let thread_view = active_thread_view.view.read(cx);
            if let Some(thread) = thread_view.thread() {
                let title = thread.read(cx).title();
                if !title.is_empty() {
                    return title;
                }
            }
            thread_view.title(cx)
        } else {
            "Thread".into()
        }
    }

    fn render_header(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let position = self.position(window, cx);
        let slot = match position {
            UtilityPanePosition::Left => UtilityPaneSlot::Left,
            UtilityPanePosition::Right => UtilityPaneSlot::Right,
        };

        let workspace = self.workspace.clone();
        let toggle_icon = self.toggle_icon(cx);
        let title = self.title(cx);

        let make_toggle_button = |workspace: WeakEntity<Workspace>, cx: &App| {
            div().px(DynamicSpacing::Base06.rems(cx)).child(
                IconButton::new("toggle_utility_pane", toggle_icon)
                    .icon_size(IconSize::Small)
                    .on_click(move |_, window, cx| {
                        workspace
                            .update(cx, |workspace, cx| {
                                workspace.toggle_utility_pane(slot, window, cx)
                            })
                            .ok();
                    }),
            )
        };

        let make_close_button = |id: &'static str, cx: &mut Context<Self>| {
            let on_click = cx.listener(|this, _: &gpui::ClickEvent, _window, cx| {
                cx.emit(ClosePane);
                this.thread_view = None;
                cx.notify();
            });
            div().px(DynamicSpacing::Base06.rems(cx)).child(
                IconButton::new(id, IconName::Close)
                    .icon_size(IconSize::Small)
                    .on_click(on_click),
            )
        };

        let make_title_label = |title: SharedString, cx: &App| {
            div()
                .px(DynamicSpacing::Base06.rems(cx))
                .child(Label::new(title).size(LabelSize::Small))
        };

        div()
            .id("utility-pane-header")
            .flex()
            .flex_none()
            .items_center()
            .w_full()
            .h(Tab::container_height(cx))
            .when(slot == UtilityPaneSlot::Left, |this| {
                this.child(make_toggle_button(workspace.clone(), cx))
                    .child(make_title_label(title.clone(), cx))
                    .child(div().flex_grow())
                    .child(make_close_button("close_utility_pane_left", cx))
            })
            .when(slot == UtilityPaneSlot::Right, |this| {
                this.child(make_close_button("close_utility_pane_right", cx))
                    .child(make_title_label(title.clone(), cx))
                    .child(div().flex_grow())
                    .child(make_toggle_button(workspace.clone(), cx))
            })
    }
}

impl Focusable for AgentThreadPane {
    fn focus_handle(&self, cx: &ui::App) -> gpui::FocusHandle {
        if let Some(thread_view) = &self.thread_view {
            thread_view.view.focus_handle(cx)
        } else {
            self.focus_handle.clone()
        }
    }
}

impl UtilityPane for AgentThreadPane {
    fn position(&self, _window: &Window, cx: &App) -> UtilityPanePosition {
        match AgentSettings::get_global(cx).agents_panel_dock {
            DockSide::Left => UtilityPanePosition::Left,
            DockSide::Right => UtilityPanePosition::Right,
        }
    }

    fn toggle_icon(&self, _cx: &App) -> IconName {
        IconName::Thread
    }

    fn expanded(&self, _cx: &App) -> bool {
        self.expanded
    }

    fn set_expanded(&mut self, expanded: bool, cx: &mut Context<Self>) {
        self.expanded = expanded;
        cx.emit(AgentsUtilityPaneEvent::StateChanged);
        cx.notify();
    }

    fn width(&self, _cx: &App) -> Pixels {
        self.width.unwrap_or(DEFAULT_UTILITY_PANE_WIDTH)
    }

    fn set_width(&mut self, width: Option<Pixels>, cx: &mut Context<Self>) {
        self.width = width;
        cx.emit(AgentsUtilityPaneEvent::StateChanged);
        cx.notify();
    }
}

impl Render for AgentThreadPane {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = if let Some(thread_view) = &self.thread_view {
            div().size_full().child(thread_view.view.clone())
        } else {
            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child(Label::new("Select a thread to view details").size(LabelSize::Default))
        };

        div()
            .size_full()
            .flex()
            .flex_col()
            .child(self.render_header(window, cx))
            .child(content)
    }
}
