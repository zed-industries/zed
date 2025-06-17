use crate::agent_panel::AgentPanel;
use anyhow::Result;
use collections::HashMap;
use gpui::{
    actions, div, Action, App, AsyncWindowContext, Context, ElementId, Entity, 
    EventEmitter, FocusHandle, Focusable, IntoElement, ParentElement, Pixels, 
    Render, Styled, Task, WeakEntity, Window
};
use prompt_store::PromptBuilder;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ui::{prelude::*, IconButton, IconName, Tooltip};
use workspace::{Panel, Workspace};
use workspace::dock::{DockPosition, PanelEvent};
use zed_actions::assistant::ToggleFocus;

actions!(agent_panel_manager, [SpawnInstance, CloseActiveInstance, NextInstance, PreviousInstance]);

#[derive(Serialize, Deserialize)]
struct SerializedAgentPanelManager {
    width: Option<Pixels>,
    active_instance: u32,
}

pub struct AgentPanelManager {
    instances: HashMap<u32, Entity<AgentPanel>>,
    active_instance: u32,
    next_id: u32,
    max_instances: u32,
    workspace: WeakEntity<Workspace>,
    width: Option<Pixels>,
    zoomed: bool,
    focus_handle: FocusHandle,
}

impl AgentPanelManager {
    pub fn load(
        workspace: WeakEntity<Workspace>,
        prompt_builder: Arc<PromptBuilder>,
        cx: AsyncWindowContext,
    ) -> Task<Result<Entity<Self>>> {
        cx.spawn(async move |cx| {
            // Create multiple agent panels upfront
            let mut agent_panels = Vec::new();
            
            for i in 1..=3 {
                let panel_task = AgentPanel::load(
                    workspace.clone(),
                    prompt_builder.clone(),
                    cx.clone(),
                );
                
                match panel_task.await {
                    Ok(panel) => agent_panels.push((i, panel)),
                    Err(e) => {
                        log::warn!("Failed to create agent panel {}: {}", i, e);
                    }
                }
            }
            
            let manager = workspace.update_in(cx, |workspace, _window, cx| {
                cx.new(|cx| Self::new(workspace, agent_panels, cx))
            })?;

            Ok(manager)
        })
    }

    fn new(
        workspace: &Workspace,
        agent_panels: Vec<(u32, Entity<AgentPanel>)>,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let workspace_weak = workspace.weak_handle();
        
        let mut instances = HashMap::new();
        let mut active_instance = 1;
        let mut next_id = 1;
        
        for (id, panel) in agent_panels {
            instances.insert(id, panel);
            if id >= next_id {
                next_id = id + 1;
            }
        }
        
        if !instances.is_empty() {
            active_instance = *instances.keys().min().unwrap();
        }
        
        Self {
            instances,
            active_instance,
            next_id,
            max_instances: 5, // Configurable maximum
            workspace: workspace_weak,
            width: None,
            zoomed: false,
            focus_handle,
        }
    }
    
    pub fn spawn_instance(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.instances.len() >= self.max_instances as usize {
            return; // Don't exceed maximum instances
        }
        
        let id = self.next_id;
        self.next_id += 1;
        
        if let Some(workspace) = self.workspace.upgrade() {
            // For now, we'll clone an existing instance as a workaround
            // In a full implementation, you'd create a new AgentPanel here
            if let Some(first_panel) = self.instances.values().next() {
                // This is a simplified approach - in reality you'd want to create
                // a completely new agent panel with its own state
                self.instances.insert(id, first_panel.clone());
                self.active_instance = id;
                cx.notify();
            }
        }
    }
    
    pub fn close_active_instance(&mut self, cx: &mut Context<Self>) {
        if self.instances.len() <= 1 {
            return; // Keep at least one instance
        }
        
        let active_id = self.active_instance;
        self.instances.remove(&active_id);
        
        // Switch to next available instance
        if let Some(&next_id) = self.instances.keys().next() {
            self.active_instance = next_id;
        }
        
        cx.notify();
    }
    
    pub fn switch_to_instance(&mut self, id: u32, cx: &mut Context<Self>) {
        if self.instances.contains_key(&id) {
            self.active_instance = id;
            cx.notify();
        }
    }
    
    pub fn next_instance(&mut self, cx: &mut Context<Self>) {
        let mut ids: Vec<u32> = self.instances.keys().copied().collect();
        ids.sort();
        if let Some(current_idx) = ids.iter().position(|&id| id == self.active_instance) {
            let next_idx = (current_idx + 1) % ids.len();
            self.active_instance = ids[next_idx];
            cx.notify();
        }
    }
    
    pub fn previous_instance(&mut self, cx: &mut Context<Self>) {
        let mut ids: Vec<u32> = self.instances.keys().copied().collect();
        ids.sort();
        if let Some(current_idx) = ids.iter().position(|&id| id == self.active_instance) {
            let prev_idx = if current_idx == 0 { ids.len() - 1 } else { current_idx - 1 };
            self.active_instance = ids[prev_idx];
            cx.notify();
        }
    }
    
    pub fn toggle_focus(
        workspace: &mut Workspace,
        _: &ToggleFocus,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        if workspace
            .panel::<Self>(cx)
            .is_some_and(|panel| panel.read(cx).enabled(cx))
        {
            workspace.toggle_panel_focus::<Self>(window, cx);
        }
    }
    
    fn render_tab_bar(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .gap_1()
            .p_2()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().editor_active_line_background)
            .children({
                let mut ids: Vec<u32> = self.instances.keys().copied().collect();
                ids.sort();
                ids.into_iter().map(|id| {
                    let is_active = id == self.active_instance;
                    
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .bg(cx.theme().colors().editor_active_line_background)
                        .child(
                            div()
                                .px_3()
                                .py_1()
                                .rounded_xl()
                                .when(is_active, |div| {
                                    div.bg(cx.theme().colors().element_selected)
                                        .text_color(cx.theme().colors().text)
                                })
                                .when(!is_active, |div| {
                                    div.hover(|div| div.bg(cx.theme().colors().element_hover))
                                        .text_color(cx.theme().colors().text_muted)
                                })
                                .child(format!("Agent {}", id))
                                .cursor_pointer()
                                .on_mouse_down(gpui::MouseButton::Left, cx.listener(move |this, _, _, cx| {
                                    this.switch_to_instance(id, cx);
                                }))
                        )
                        .when(self.instances.len() > 1, |flex| {
                            flex.child(
                                IconButton::new(
                                    ElementId::Name(format!("close-agent-{}", id).into()),
                                    IconName::Close
                                )
                                .size(ui::ButtonSize::None)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    if this.active_instance == id {
                                        this.close_active_instance(cx);
                                    }
                                }))
                            )
                        })
                }).collect::<Vec<_>>()
            })
            .when(self.instances.len() < self.max_instances as usize, |div| {
                div.child(
                    IconButton::new("spawn-agent", IconName::Plus)
                        .tooltip(move |window, cx| Tooltip::text("New Agent Instance")(window, cx))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.spawn_instance(window, cx);
                        }))
                )
            })
    }
    
    fn render_content(&self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(agent_panel) = self.instances.get(&self.active_instance) {
            div()
                .size_full()
                .child(agent_panel.clone())
        } else {
            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .text_color(_cx.theme().colors().text_muted)
                        .child("No agent instances available")
                )
        }
    }
}

impl Focusable for AgentPanelManager {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for AgentPanelManager {}

impl Panel for AgentPanelManager {
    fn persistent_name() -> &'static str {
        "AgentPanelManager"
    }
    
    fn position(&self, _window: &Window, _cx: &App) -> DockPosition {
        DockPosition::Right
    }
    
    fn position_is_valid(&self, _position: DockPosition) -> bool {
        true
    }
    
    fn set_position(&mut self, _position: DockPosition, _window: &mut Window, _cx: &mut Context<Self>) {}
    
    fn size(&self, _window: &Window, _cx: &App) -> Pixels {
        self.width.unwrap_or_else(|| px(640.))
    }
    
    fn set_size(&mut self, size: Option<Pixels>, _window: &mut Window, _cx: &mut Context<Self>) {
        self.width = size;
    }
    
    fn set_active(&mut self, _active: bool, _window: &mut Window, _cx: &mut Context<Self>) {}
    
    fn remote_id() -> Option<workspace::dock::PanelId> {
        None
    }
    
    fn icon(&self, _window: &Window, _cx: &App) -> Option<ui::IconName> {
        Some(IconName::ZedAssistant)
    }
    
    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Multi-Agent Assistant")
    }
    
    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }
    
    fn activation_priority(&self) -> u32 {
        2
    }
    
    fn enabled(&self, _cx: &App) -> bool {
        !self.instances.is_empty()
    }
    
    fn is_zoomed(&self, _window: &Window, _cx: &App) -> bool {
        self.zoomed
    }
    
    fn set_zoomed(&mut self, zoomed: bool, _window: &mut Window, _cx: &mut Context<Self>) {
        self.zoomed = zoomed;
    }
}

impl Render for AgentPanelManager {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(self.render_tab_bar(window, cx))
            .child(self.render_content(window, cx))
    }
}

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace
                .register_action(|workspace, _: &SpawnInstance, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanelManager>(cx) {
                        panel.update(cx, |panel, cx| panel.spawn_instance(window, cx));
                        workspace.focus_panel::<AgentPanelManager>(window, cx);
                    }
                })
                .register_action(|workspace, _: &CloseActiveInstance, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanelManager>(cx) {
                        panel.update(cx, |panel, cx| panel.close_active_instance(cx));
                        workspace.focus_panel::<AgentPanelManager>(window, cx);
                    }
                })
                .register_action(|workspace, _: &NextInstance, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanelManager>(cx) {
                        panel.update(cx, |panel, cx| panel.next_instance(cx));
                        workspace.focus_panel::<AgentPanelManager>(window, cx);
                    }
                })
                .register_action(|workspace, _: &PreviousInstance, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanelManager>(cx) {
                        panel.update(cx, |panel, cx| panel.previous_instance(cx));
                        workspace.focus_panel::<AgentPanelManager>(window, cx);
                    }
                });
        },
    )
    .detach();
}