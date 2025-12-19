//! Ali Command Center Panel
//!
//! A bottom-docked panel that provides always-on access to Ali,
//! the Chief of Staff who orchestrates all Convergio agents.

use crate::AliPanelSettings;
use agent_ui::{ExternalAgent, NewExternalAgentThread};
use anyhow::Result;
use db::kvp::KEY_VALUE_STORE;
use editor::Editor;
use gpui::{
    actions, div, prelude::*, Action, App, AsyncWindowContext, Context, Entity,
    EventEmitter, FocusHandle, Focusable, InteractiveElement, KeyBinding, ParentElement, Pixels,
    Render, Styled, Subscription, WeakEntity, Window,
};
use serde::{Deserialize, Serialize};
use ui::{prelude::*, Button, ButtonStyle, Icon, IconName, IconSize, Label};
use util::ResultExt;
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};

const ALI_PANEL_KEY: &str = "AliPanel";
const ALI_SERVER_NAME: &str = "Convergio-Ali";

actions!(
    ali_panel,
    [
        ToggleFocus,
        InvokeAli,
        SendToAli,
    ]
);

#[derive(Clone, Debug)]
pub struct ActiveAgent {
    pub name: String,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
struct SerializedAliPanel {
    height: Option<f32>,
}

pub struct AliPanel {
    focus_handle: FocusHandle,
    height: Option<Pixels>,
    input_editor: Entity<Editor>,
    active_agents: Vec<ActiveAgent>,
    is_expanded: bool,
    _input_subscription: Subscription,
}

pub fn init(cx: &mut App) {
    AliPanelSettings::register(cx);

    // Bind Enter key in Ali panel input to send message
    cx.bind_keys([
        KeyBinding::new("enter", SendToAli, Some("Editor && AliPanel")),
    ]);

    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|_workspace, _: &ToggleFocus, window, cx| {
            let action = NewExternalAgentThread::with_agent(ExternalAgent::Custom {
                name: ALI_SERVER_NAME.into()
            });
            window.dispatch_action(action.boxed_clone(), cx);
        });

        workspace.register_action(|_workspace, _: &InvokeAli, window, cx| {
            // Open Ali chat when invoked
            let action = NewExternalAgentThread::with_agent(ExternalAgent::Custom {
                name: ALI_SERVER_NAME.into()
            });
            window.dispatch_action(action.boxed_clone(), cx);
        });
    })
    .detach();
}

impl AliPanel {
    pub fn new(_workspace: &Workspace, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        let input_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Ask Ali anything... (Enter to chat)", window, cx);
            editor
        });

        // Subscribe to editor events
        let subscription = cx.subscribe(&input_editor, |_this, _editor, event: &editor::EditorEvent, _cx| {
            if let editor::EditorEvent::BufferEdited { .. } = event {
                // Input changed - could show preview here
            }
        });

        // Active agents will be populated when agents are actually in use
        let active_agents = Vec::new();

        Self {
            focus_handle,
            height: None,
            input_editor,
            active_agents,
            is_expanded: false,
            _input_subscription: subscription,
        }
    }

    fn send_to_ali(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let text = self.input_editor.read(cx).text(cx);
        if !text.is_empty() {
            log::info!("Sending to Ali: {}", text);
            // Clear the input
            self.input_editor.update(cx, |editor, cx| {
                editor.clear(window, cx);
            });
        }
        // Open Ali chat
        self.open_ali_chat(window, cx);
    }

    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        workspace.update_in(&mut cx, |workspace, window, cx| {
            cx.new(|cx| AliPanel::new(workspace, window, cx))
        })
    }

    fn serialize(&self, cx: &mut Context<Self>) {
        let height = self.height.map(|h| f32::from(h));
        cx.background_executor()
            .spawn(async move {
                let serialized = serde_json::to_string(&SerializedAliPanel { height }).ok();
                if let Some(serialized) = serialized {
                    KEY_VALUE_STORE
                        .write_kvp(ALI_PANEL_KEY.to_string(), serialized)
                        .await
                        .log_err();
                }
            })
            .detach();
    }

    fn toggle_expand(&mut self, cx: &mut Context<Self>) {
        self.is_expanded = !self.is_expanded;
        cx.notify();
    }

    fn open_ali_chat(&self, window: &mut Window, cx: &mut Context<Self>) {
        log::info!("Opening Ali Command Center chat");
        let action = NewExternalAgentThread::with_agent(ExternalAgent::Custom {
            name: ALI_SERVER_NAME.into()
        });
        window.dispatch_action(action.boxed_clone(), cx);
    }

    fn render_status_bar(&self, cx: &Context<Self>) -> impl IntoElement {
        let active_count = self.active_agents.len();

        div()
            .flex()
            .items_center()
            .justify_between()
            .px_3()
            .py_1()
            .bg(cx.theme().colors().title_bar_background)
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        Icon::new(IconName::Ai)
                            .size(IconSize::Small)
                            .color(Color::Accent)
                    )
                    .child(
                        Label::new("ALI - Command Center")
                            .size(LabelSize::Small)
                            .weight(gpui::FontWeight::BOLD)
                    )
                    .child(
                        Label::new(format!("({} agents active)", active_count))
                            .size(LabelSize::XSmall)
                            .color(Color::Muted)
                    )
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        Button::new("expand", if self.is_expanded { "Collapse" } else { "Expand" })
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.toggle_expand(cx);
                            }))
                    )
                    .child(
                        Button::new("open-chat", "Open Chat")
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.open_ali_chat(window, cx);
                            }))
                    )
            )
    }

    fn render_agents_status(&self, cx: &Context<Self>) -> impl IntoElement {
        div()
            .px_3()
            .py_2()
            .flex()
            .flex_wrap()
            .gap_2()
            .children(
                self.active_agents.iter().map(|agent| {
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .px_2()
                        .py_1()
                        .rounded_md()
                        .bg(cx.theme().colors().surface_background)
                        .child(
                            div()
                                .w_2()
                                .h_2()
                                .rounded_full()
                                .bg(cx.theme().status().success) // Green dot for active
                        )
                        .child(
                            Label::new(agent.name.clone())
                                .size(LabelSize::Small)
                                .weight(gpui::FontWeight::MEDIUM)
                        )
                        .child(
                            Label::new(format!(": {}", agent.status))
                                .size(LabelSize::XSmall)
                                .color(Color::Muted)
                        )
                })
            )
    }

    fn render_input_bar(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_2()
            .px_3()
            .py_2()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .child(
                Icon::new(IconName::ArrowRight)
                    .size(IconSize::Small)
                    .color(Color::Muted)
            )
            .child(
                div()
                    .flex_1()
                    .child(self.input_editor.clone())
            )
    }
}

impl Focusable for AliPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for AliPanel {}

impl Panel for AliPanel {
    fn persistent_name() -> &'static str {
        "AliPanel"
    }

    fn panel_key() -> &'static str {
        "ali_panel"
    }

    fn position(&self, _window: &Window, cx: &App) -> DockPosition {
        AliPanelSettings::get_global(cx).dock
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Bottom)
    }

    fn set_position(&mut self, _position: DockPosition, _window: &mut Window, _cx: &mut Context<Self>) {
        // Ali panel is always at the bottom
    }

    fn size(&self, _window: &Window, cx: &App) -> Pixels {
        self.height
            .unwrap_or_else(|| AliPanelSettings::get_global(cx).default_height)
    }

    fn set_size(&mut self, size: Option<Pixels>, window: &mut Window, cx: &mut Context<Self>) {
        self.height = size;
        cx.notify();
        cx.defer_in(window, |this, _, cx| {
            this.serialize(cx);
        });
    }

    fn icon(&self, _window: &Window, cx: &App) -> Option<IconName> {
        AliPanelSettings::get_global(cx)
            .button
            .then_some(IconName::Ai)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Ali Command Center")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        2 // Higher than convergio_panel (3), lower than terminal (1)
    }
}

impl Render for AliPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("ali-panel")
            .key_context("AliPanel")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(|this, _: &SendToAli, window, cx| {
                this.send_to_ali(window, cx);
            }))
            .size_full()
            .flex()
            .flex_col()
            .bg(cx.theme().colors().panel_background)
            .child(self.render_status_bar(cx))
            .when(self.is_expanded || !self.active_agents.is_empty(), |this| {
                this.child(self.render_agents_status(cx))
            })
            .child(self.render_input_bar(window, cx))
    }
}
