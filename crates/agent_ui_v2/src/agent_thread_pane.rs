use acp_thread::AgentSessionInfo;
use agent::{NativeAgentServer, ThreadStore};
use agent_client_protocol as acp;
use agent_servers::AgentServer;
use agent_settings::AgentSettings;
use agent_ui::acp::AcpThreadView;
use editor::Editor;
use fs::Fs;
use gpui::{
    DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, MouseButton, Pixels, ScrollHandle,
    SharedString, Subscription, WeakEntity, actions, prelude::*,
};
use menu::{Cancel, Confirm};
use project::Project;
use prompt_store::PromptStore;
use serde::Deserialize;
use serde::Serialize;
use settings::DockSide;
use settings::Settings as _;
use std::rc::Rc;
use std::sync::Arc;
use ui::{
    ContextMenu, Headline, HeadlineSize, IconButtonShape, Tab, TabBar, TabPosition, Tooltip,
    prelude::*, right_click_menu,
};
use workspace::{
    ModalView, Workspace,
    dock::{ClosePane, MinimizePane, UtilityPane, UtilityPanePosition},
    utility_pane::UtilityPaneSlot,
};

pub const DEFAULT_UTILITY_PANE_WIDTH: Pixels = gpui::px(400.0);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SerializedHistoryEntryId {
    AcpThread(String),
}

impl From<acp::SessionId> for SerializedHistoryEntryId {
    fn from(id: acp::SessionId) -> Self {
        SerializedHistoryEntryId::AcpThread(id.0.to_string())
    }
}

impl SerializedHistoryEntryId {
    pub fn to_session_id(&self) -> acp::SessionId {
        match self {
            SerializedHistoryEntryId::AcpThread(s) => acp::SessionId::new(s.clone()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializedAgentTab {
    pub thread_id: SerializedHistoryEntryId,
    pub custom_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializedAgentThreadPane {
    pub expanded: bool,
    pub width: Option<Pixels>,
    pub tabs: Vec<SerializedAgentTab>,
    pub active_tab_index: usize,
}

pub enum AgentsUtilityPaneEvent {
    StateChanged,
}

impl EventEmitter<AgentsUtilityPaneEvent> for AgentThreadPane {}
impl EventEmitter<MinimizePane> for AgentThreadPane {}
impl EventEmitter<ClosePane> for AgentThreadPane {}

actions!(
    agent_thread_pane,
    [
        RenameAgentTab,
        CloseAllAgentTabs,
        ActivatePreviousAgentTab,
        ActivateNextAgentTab,
    ]
);

#[derive(Clone, PartialEq, Debug)]
pub struct CloseAgentTab {
    pub index: Option<usize>,
}

impl gpui::Action for CloseAgentTab {
    fn name(&self) -> &'static str {
        "agent_thread_pane::CloseAgentTab"
    }

    fn name_for_type() -> &'static str
    where
        Self: Sized,
    {
        "agent_thread_pane::CloseAgentTab"
    }

    fn build(_value: serde_json::Value) -> anyhow::Result<Box<dyn gpui::Action>>
    where
        Self: Sized,
    {
        Ok(Box::new(CloseAgentTab { index: None }))
    }

    fn partial_eq(&self, action: &dyn gpui::Action) -> bool {
        (action as &dyn std::any::Any)
            .downcast_ref::<Self>()
            .map_or(false, |a| self == a)
    }

    fn boxed_clone(&self) -> Box<dyn gpui::Action> {
        Box::new(self.clone())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct CloseOtherAgentTabs {
    pub index: Option<usize>,
}

impl gpui::Action for CloseOtherAgentTabs {
    fn name(&self) -> &'static str {
        "agent_thread_pane::CloseOtherAgentTabs"
    }

    fn name_for_type() -> &'static str
    where
        Self: Sized,
    {
        "agent_thread_pane::CloseOtherAgentTabs"
    }

    fn build(_value: serde_json::Value) -> anyhow::Result<Box<dyn gpui::Action>>
    where
        Self: Sized,
    {
        Ok(Box::new(CloseOtherAgentTabs { index: None }))
    }

    fn partial_eq(&self, action: &dyn gpui::Action) -> bool {
        (action as &dyn std::any::Any)
            .downcast_ref::<Self>()
            .map_or(false, |a| self == a)
    }

    fn boxed_clone(&self) -> Box<dyn gpui::Action> {
        Box::new(self.clone())
    }
}

struct AgentTab {
    thread_id: acp::SessionId,
    custom_name: Option<SharedString>,
    view: Entity<AcpThreadView>,
    _notify: Subscription,
}

#[derive(Clone)]
pub struct DraggedAgentTab {
    pub pane: WeakEntity<AgentThreadPane>,
    pub thread_id: acp::SessionId,
    pub ix: usize,
    pub title: SharedString,
}

impl Render for DraggedAgentTab {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Tab::new("dragged-agent-tab")
            .toggle_state(true)
            .child(Label::new(self.title.clone()))
    }
}

struct RenameAgentTabModal {
    current_title: SharedString,
    editor: Entity<Editor>,
    tab_index: usize,
    pane: WeakEntity<AgentThreadPane>,
}

impl RenameAgentTabModal {
    fn new(
        current_title: SharedString,
        tab_index: usize,
        pane: WeakEntity<AgentThreadPane>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_text(current_title.to_string(), window, cx);
            editor
        });

        Self {
            current_title,
            editor,
            tab_index,
            pane,
        }
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        let new_name = self.editor.read(cx).text(cx);
        let custom_name = if new_name.is_empty() || new_name == self.current_title.as_ref() {
            None
        } else {
            Some(SharedString::from(new_name))
        };

        if let Some(pane) = self.pane.upgrade() {
            let tab_index = self.tab_index;
            pane.update(cx, |pane, cx| {
                if let Some(tab) = pane.tabs.get_mut(tab_index) {
                    tab.custom_name = custom_name;
                    cx.emit(AgentsUtilityPaneEvent::StateChanged);
                    cx.notify();
                }
            });
        }

        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for RenameAgentTabModal {}
impl ModalView for RenameAgentTabModal {}

impl Focusable for RenameAgentTabModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Render for RenameAgentTabModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("RenameAgentTabModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .w(rems(34.))
            .child(
                h_flex()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .w_full()
                    .gap_1p5()
                    .child(Icon::new(IconName::Thread).size(IconSize::XSmall))
                    .child(Headline::new("Rename Tab").size(HeadlineSize::XSmall)),
            )
            .child(div().px_3().pb_3().w_full().child(self.editor.clone()))
    }
}

pub struct AgentThreadPane {
    focus_handle: gpui::FocusHandle,
    expanded: bool,
    width: Option<Pixels>,
    tabs: Vec<AgentTab>,
    active_tab_index: usize,
    tab_bar_scroll_handle: ScrollHandle,
    workspace: WeakEntity<Workspace>,
}

impl AgentThreadPane {
    pub fn new(workspace: WeakEntity<Workspace>, cx: &mut ui::Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        Self {
            focus_handle,
            expanded: false,
            width: None,
            tabs: Vec::new(),
            active_tab_index: 0,
            tab_bar_scroll_handle: ScrollHandle::new(),
            workspace,
        }
    }

    pub fn thread_id(&self) -> Option<acp::SessionId> {
        self.tabs
            .get(self.active_tab_index)
            .map(|tab| tab.thread_id.clone())
    }

    pub fn serialize(&self) -> SerializedAgentThreadPane {
        SerializedAgentThreadPane {
            expanded: self.expanded,
            width: self.width,
            tabs: self
                .tabs
                .iter()
                .map(|tab| SerializedAgentTab {
                    thread_id: SerializedHistoryEntryId::from(tab.thread_id.clone()),
                    custom_name: tab.custom_name.as_ref().map(|s| s.to_string()),
                })
                .collect(),
            active_tab_index: self.active_tab_index,
        }
    }

    pub fn open_thread(
        &mut self,
        entry: AgentSessionInfo,
        fs: Arc<dyn Fs>,
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        thread_store: Entity<ThreadStore>,
        prompt_store: Option<Entity<PromptStore>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let thread_id = entry.session_id.clone();

        if let Some(existing_ix) = self.tabs.iter().position(|t| t.thread_id == thread_id) {
            self.activate_tab(existing_ix, window, cx);
            return;
        }

        let agent: Rc<dyn AgentServer> = Rc::new(NativeAgentServer::new(fs, thread_store.clone()));

        let thread_view = cx.new(|cx| {
            AcpThreadView::new(
                agent,
                Some(entry),
                None,
                workspace,
                project,
                Some(thread_store),
                prompt_store,
                true,
                window,
                cx,
            )
        });

        let notify = cx.observe(&thread_view, |_, _, cx| {
            cx.notify();
        });

        let new_tab = AgentTab {
            thread_id,
            custom_name: None,
            view: thread_view,
            _notify: notify,
        };

        self.tabs.push(new_tab);
        self.active_tab_index = self.tabs.len() - 1;
        self.tab_bar_scroll_handle.scroll_to_item(self.active_tab_index);

        cx.emit(AgentsUtilityPaneEvent::StateChanged);
        cx.notify();
    }

    fn activate_tab(&mut self, ix: usize, _window: &mut Window, cx: &mut Context<Self>) {
        if ix < self.tabs.len() {
            self.active_tab_index = ix;
            self.tab_bar_scroll_handle.scroll_to_item(ix);
            cx.notify();
        }
    }

    fn close_tab_at(&mut self, ix: usize, _window: &mut Window, cx: &mut Context<Self>) {
        if ix >= self.tabs.len() {
            return;
        }

        self.tabs.remove(ix);

        if self.tabs.is_empty() {
            cx.emit(ClosePane);
            return;
        }

        if self.active_tab_index >= self.tabs.len() {
            self.active_tab_index = self.tabs.len() - 1;
        } else if ix < self.active_tab_index {
            self.active_tab_index -= 1;
        }

        cx.emit(AgentsUtilityPaneEvent::StateChanged);
        cx.notify();
    }

    fn close_other_tabs(&mut self, keep_ix: usize, _window: &mut Window, cx: &mut Context<Self>) {
        if keep_ix >= self.tabs.len() {
            return;
        }

        let kept_tab = self.tabs.remove(keep_ix);
        self.tabs.clear();
        self.tabs.push(kept_tab);
        self.active_tab_index = 0;

        cx.emit(AgentsUtilityPaneEvent::StateChanged);
        cx.notify();
    }

    fn close_all_tabs(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.tabs.clear();
        self.active_tab_index = 0;
        cx.emit(ClosePane);
    }

    fn rename_tab_at(&self, ix: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(tab) = self.tabs.get(ix) else {
            return;
        };

        let current_title = self.tab_title(tab, cx);
        let pane = cx.entity().downgrade();

        let _ = self.workspace.update(cx, |workspace, cx| {
            workspace.toggle_modal(window, cx, |window, cx| {
                RenameAgentTabModal::new(current_title, ix, pane, window, cx)
            });
        });
    }

    fn handle_tab_drop(
        &mut self,
        dragged: &DraggedAgentTab,
        target_ix: usize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(pane) = dragged.pane.upgrade() else {
            return;
        };

        if pane.entity_id() != cx.entity_id() {
            return;
        }

        let source_ix = dragged.ix;
        if source_ix == target_ix {
            return;
        }

        let tab = self.tabs.remove(source_ix);

        let insert_ix = if target_ix > source_ix {
            target_ix - 1
        } else {
            target_ix
        };

        self.tabs.insert(insert_ix, tab);

        if self.active_tab_index == source_ix {
            self.active_tab_index = insert_ix;
        } else if source_ix < self.active_tab_index && self.active_tab_index <= insert_ix {
            self.active_tab_index -= 1;
        } else if insert_ix <= self.active_tab_index && self.active_tab_index < source_ix {
            self.active_tab_index += 1;
        }

        cx.emit(AgentsUtilityPaneEvent::StateChanged);
        cx.notify();
    }

    fn tab_title(&self, tab: &AgentTab, cx: &App) -> SharedString {
        if let Some(custom_name) = &tab.custom_name {
            return custom_name.clone();
        }

        let thread_view = tab.view.read(cx);
        if let Some(thread) = thread_view.thread() {
            let title = thread.read(cx).title();
            if !title.is_empty() {
                return title;
            }
        }
        thread_view.title(cx)
    }

    fn render_tab_bar(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let position = self.position(window, cx);
        let slot = match position {
            UtilityPanePosition::Left => UtilityPaneSlot::Left,
            UtilityPanePosition::Right => UtilityPaneSlot::Right,
        };

        let workspace = self.workspace.clone();
        let toggle_icon = self.toggle_icon(cx);

        let pane_toggle_button = {
            let workspace = workspace.clone();
            IconButton::new("toggle_utility_pane", toggle_icon)
                .icon_size(IconSize::Small)
                .tooltip(Tooltip::text("Toggle Agent Pane"))
                .on_click(move |_, window, cx| {
                    workspace
                        .update(cx, |workspace, cx| {
                            workspace.toggle_utility_pane(slot, window, cx)
                        })
                        .ok();
                })
        };

        let close_button = IconButton::new("close_btn", IconName::Close)
            .icon_size(IconSize::Small)
            .tooltip(Tooltip::text("Close Agent Pane"))
            .on_click(cx.listener(|this, _: &gpui::ClickEvent, _window, cx| {
                this.tabs.clear();
                this.active_tab_index = 0;
                cx.emit(ClosePane);
                cx.notify()
            }));

        let mut tab_bar = TabBar::new("agent-tab-bar")
            .track_scroll(&self.tab_bar_scroll_handle)
            .start_child(pane_toggle_button);

        for (ix, tab) in self.tabs.iter().enumerate() {
            tab_bar = tab_bar.child(self.render_single_tab(ix, tab, cx));
        }

        tab_bar.end_child(close_button)
    }

    fn render_single_tab(
        &self,
        ix: usize,
        tab: &AgentTab,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_active = ix == self.active_tab_index;
        let is_first = ix == 0;
        let is_last = ix == self.tabs.len() - 1;
        let thread_id = tab.thread_id.clone();
        let title = self.tab_title(tab, cx);
        let total_tabs = self.tabs.len();

        let position = if is_first && is_last {
            TabPosition::First
        } else if is_first {
            TabPosition::First
        } else if is_last {
            TabPosition::Last
        } else {
            TabPosition::Middle(ix.cmp(&self.active_tab_index))
        };

        let pane = cx.entity().downgrade();
        let title_for_drag = title.clone();
        let title_for_menu = title.clone();

        let close_button = IconButton::new(("close-tab", ix), IconName::Close)
            .shape(IconButtonShape::Square)
            .icon_color(Color::Muted)
            .size(ButtonSize::None)
            .icon_size(IconSize::XSmall)
            .visible_on_hover("")
            .on_click(cx.listener(move |this, _, window, cx| {
                this.close_tab_at(ix, window, cx);
            }));

        let on_click = cx.listener(move |this, _, window, cx| {
            this.activate_tab(ix, window, cx);
        });

        let on_middle_click = cx.listener(move |this, _, window, cx| {
            this.close_tab_at(ix, window, cx);
        });

        let on_drop = cx.listener(move |this, dragged: &DraggedAgentTab, window, cx| {
            this.handle_tab_drop(dragged, ix, window, cx);
        });

        right_click_menu(("agent-tab-menu", ix))
            .trigger(move |_is_menu_active, _window, _cx| {
                Tab::new(("agent-tab", ix))
                    .position(position)
                    .toggle_state(is_active)
                    .on_click(on_click)
                    .on_mouse_down(MouseButton::Middle, on_middle_click)
                    .on_drag(
                        DraggedAgentTab {
                            pane: pane.clone(),
                            thread_id: thread_id.clone(),
                            ix,
                            title: title_for_drag.clone(),
                        },
                        |dragged, _, _, cx| cx.new(|_| dragged.clone()),
                    )
                    .drag_over::<DraggedAgentTab>(move |tab, dragged: &DraggedAgentTab, _, cx| {
                        let styled = tab
                            .bg(cx.theme().colors().drop_target_background)
                            .border_color(cx.theme().colors().drop_target_border)
                            .border_0();

                        if ix < dragged.ix {
                            styled.border_l_2()
                        } else if ix > dragged.ix {
                            styled.border_r_2()
                        } else {
                            styled
                        }
                    })
                    .on_drop(on_drop)
                    .end_slot(close_button)
                    .child(Label::new(title.clone()).truncate())
            })
            .menu(move |window, cx| {
                Self::build_tab_context_menu(
                    ix,
                    total_tabs,
                    title_for_menu.clone(),
                    window,
                    cx,
                )
            })
    }

    fn build_tab_context_menu(
        ix: usize,
        total_tabs: usize,
        _title: SharedString,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<ContextMenu> {
        ContextMenu::build(window, cx, move |menu, _window, _cx| {
            menu.entry("Rename", None, move |_window, cx| {
                cx.dispatch_action(&RenameAgentTab);
            })
            .separator()
            .entry("Close", None, move |_window, cx| {
                cx.dispatch_action(&CloseAgentTab { index: Some(ix) });
            })
            .when(total_tabs > 1, |menu| {
                menu.entry("Close Others", None, move |_window, cx| {
                    cx.dispatch_action(&CloseOtherAgentTabs { index: Some(ix) });
                })
            })
            .entry("Close All", None, move |_window, cx| {
                cx.dispatch_action(&CloseAllAgentTabs);
            })
        })
    }
}

impl Focusable for AgentThreadPane {
    fn focus_handle(&self, cx: &ui::App) -> gpui::FocusHandle {
        if let Some(tab) = self.tabs.get(self.active_tab_index) {
            tab.view.focus_handle(cx)
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
        let content = if let Some(tab) = self.tabs.get(self.active_tab_index) {
            div().size_full().child(tab.view.clone())
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
            .key_context("AgentThreadPane")
            .on_action(cx.listener(|this, _: &RenameAgentTab, window, cx| {
                this.rename_tab_at(this.active_tab_index, window, cx);
            }))
            .on_action(cx.listener(|this, action: &CloseAgentTab, window, cx| {
                let ix = action.index.unwrap_or(this.active_tab_index);
                this.close_tab_at(ix, window, cx);
            }))
            .on_action(cx.listener(|this, action: &CloseOtherAgentTabs, window, cx| {
                let ix = action.index.unwrap_or(this.active_tab_index);
                this.close_other_tabs(ix, window, cx);
            }))
            .on_action(cx.listener(|this, _: &CloseAllAgentTabs, window, cx| {
                this.close_all_tabs(window, cx);
            }))
            .on_action(cx.listener(|this, _: &ActivatePreviousAgentTab, window, cx| {
                if this.active_tab_index > 0 {
                    this.activate_tab(this.active_tab_index - 1, window, cx);
                }
            }))
            .on_action(cx.listener(|this, _: &ActivateNextAgentTab, window, cx| {
                if this.active_tab_index < this.tabs.len().saturating_sub(1) {
                    this.activate_tab(this.active_tab_index + 1, window, cx);
                }
            }))
            .child(self.render_tab_bar(window, cx))
            .child(content)
    }
}
