//! Ali Command Center Panel
//!
//! A bottom-docked panel that provides always-on access to Ali,
//! the Chief of Staff who orchestrates all Convergio agents.
//! This panel embeds a full chat interface directly in the bottom dock.

use crate::AliPanelSettings;
use agent::HistoryStore;
use agent_servers::CustomAgentServer;
use agent_ui::acp::AcpThreadView;
use anyhow::Result;
use db::kvp::KEY_VALUE_STORE;
use fs::Fs;
use gpui::{
    actions, div, prelude::*, Action, App, AsyncWindowContext, Context, Entity,
    EventEmitter, FocusHandle, Focusable, InteractiveElement, ParentElement, Pixels,
    Render, Styled, Subscription, WeakEntity, Window,
};
use project::Project;
use prompt_store::PromptStore;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::sync::Arc;
use ui::{prelude::*, Button, ButtonLike, Icon, IconName, IconSize, Label, Tooltip};
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
        NewConversation,
    ]
);

#[derive(Serialize, Deserialize)]
struct SerializedAliPanel {
    height: Option<f32>,
}

pub struct AliPanel {
    focus_handle: FocusHandle,
    height: Option<Pixels>,
    thread_view: Option<Entity<AcpThreadView>>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    history_store: Entity<HistoryStore>,
    prompt_store: Option<Entity<PromptStore>>,
    fs: Arc<dyn Fs>,
    tried_resume: bool,
    pending_resume_thread: Option<agent::DbThreadMetadata>,
    _history_subscription: Subscription,
}

pub fn init(cx: &mut App) {
    AliPanelSettings::register(cx);

    cx.observe_new(|workspace: &mut Workspace, _, _| {
        // ToggleFocus should toggle the AliPanel in the bottom dock
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<AliPanel>(window, cx);
        });

        // NewConversation starts a fresh conversation with Ali
        workspace.register_action(|workspace, _: &NewConversation, window, cx| {
            if let Some(panel) = workspace.panel::<AliPanel>(cx) {
                panel.update(cx, |ali_panel, cx| {
                    ali_panel.new_conversation(window, cx);
                });
            }
        });
    })
    .detach();
}

impl AliPanel {
    pub fn new(
        workspace: &Workspace,
        project: Entity<Project>,
        history_store: Entity<HistoryStore>,
        prompt_store: Option<Entity<PromptStore>>,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let weak_workspace = workspace.weak_handle();

        // Observe history store changes to catch when threads are loaded
        let history_subscription = cx.observe(&history_store, |this, _, cx| {
            // When history changes, try to resume if we haven't already
            this.try_resume_existing_thread(cx);
        });

        // Create the Ali thread view immediately (without resume - threads not loaded yet)
        let server = Rc::new(CustomAgentServer::new(ALI_SERVER_NAME.into()));

        // Try immediate resume (might work if history already loaded)
        let resume_thread = history_store.read(cx).thread_by_agent_name(ALI_SERVER_NAME).cloned();
        let tried_resume = resume_thread.is_some();

        let thread_view = cx.new(|cx| {
            AcpThreadView::new(
                server,
                resume_thread,
                None,
                weak_workspace.clone(),
                project.clone(),
                history_store.clone(),
                prompt_store.clone(),
                true, // focus
                window,
                cx,
            )
        });

        Self {
            focus_handle,
            height: None,
            thread_view: Some(thread_view),
            workspace: weak_workspace,
            project,
            history_store,
            prompt_store,
            fs,
            tried_resume,
            pending_resume_thread: None,
            _history_subscription: history_subscription,
        }
    }

    /// Try to resume an existing Ali thread when history becomes available
    fn try_resume_existing_thread(&mut self, cx: &mut Context<Self>) {
        // Only try once to avoid recreating the view repeatedly
        if self.tried_resume {
            return;
        }

        // Check if we now have an Ali thread in history
        let resume_thread = self.history_store.read(cx).thread_by_agent_name(ALI_SERVER_NAME).cloned();

        if let Some(thread_metadata) = resume_thread {
            log::info!("Ali panel: Found existing thread {} to resume", thread_metadata.id);
            self.tried_resume = true;

            // Store the thread metadata to use when we have window access
            // The thread view will be recreated on next render with the proper context
            self.pending_resume_thread = Some(thread_metadata);
            cx.notify();
        }
    }

    /// Called during render to handle pending resume
    fn handle_pending_resume(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(thread_metadata) = self.pending_resume_thread.take() {
            let server = Rc::new(CustomAgentServer::new(ALI_SERVER_NAME.into()));

            let new_thread_view = cx.new(|cx| {
                AcpThreadView::new(
                    server,
                    Some(thread_metadata),
                    None,
                    self.workspace.clone(),
                    self.project.clone(),
                    self.history_store.clone(),
                    self.prompt_store.clone(),
                    false, // don't focus - we're resuming in background
                    window,
                    cx,
                )
            });
            self.thread_view = Some(new_thread_view);
        }
    }

    /// Start a new conversation with Ali (clears current thread)
    pub fn new_conversation(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        log::info!("Ali panel: Starting new conversation");

        // Create a fresh Ali thread view (no resume)
        let server = Rc::new(CustomAgentServer::new(ALI_SERVER_NAME.into()));

        let new_thread_view = cx.new(|cx| {
            AcpThreadView::new(
                server,
                None, // No resume - start fresh
                None,
                self.workspace.clone(),
                self.project.clone(),
                self.history_store.clone(),
                self.prompt_store.clone(),
                true, // focus the new conversation
                window,
                cx,
            )
        });
        self.thread_view = Some(new_thread_view);
        self.tried_resume = true; // Don't try to resume after this
        cx.notify();
    }

    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        workspace.update_in(&mut cx, |workspace, window, cx| {
            let project = workspace.project().clone();
            let fs = workspace.app_state().fs.clone();

            // Get history store from AgentPanel (required)
            let agent_panel = workspace
                .panel::<agent_ui::AgentPanel>(cx)
                .ok_or_else(|| anyhow::anyhow!("AgentPanel must be registered before AliPanel"))?;
            let history_store = agent_panel.read(cx).history_store().clone();

            // Get prompt store if available
            let prompt_store = agent_panel.read(cx).prompt_store().cloned();

            Ok(cx.new(|cx| AliPanel::new(workspace, project, history_store, prompt_store, fs, window, cx)))
        })?
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

    fn render_header(&self, cx: &Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_between()
            .px_3()
            .py_1()
            // Terminal-style dark background
            .bg(gpui::rgb(0x1a1a1a))
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        Icon::new(IconName::Brain)
                            .size(IconSize::Small)
                            .color(Color::Accent)
                    )
                    .child(
                        Label::new("ALI - Command Center")
                            .size(LabelSize::Small)
                            .weight(gpui::FontWeight::BOLD)
                            .color(Color::Success) // Terminal-style green text
                    )
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(
                        Button::new("new-conversation", "New")
                            .icon(IconName::Plus)
                            .icon_size(IconSize::Small)
                            .icon_position(ui::IconPosition::Start)
                            .style(ui::ButtonStyle::Subtle)
                            .tooltip(Tooltip::text("Start a new conversation"))
                            .on_click(|_, window, cx| {
                                window.dispatch_action(Box::new(NewConversation), cx);
                            })
                    )
            )
    }
}

impl Focusable for AliPanel {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        // Delegate focus to the thread view if it exists
        if let Some(thread_view) = &self.thread_view {
            thread_view.focus_handle(cx)
        } else {
            self.focus_handle.clone()
        }
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
            .then_some(IconName::Brain)
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
        // Handle pending resume if we have one
        self.handle_pending_resume(window, cx);

        div()
            .id("ali-panel")
            .key_context("AliPanel")
            .track_focus(&self.focus_handle)
            .size_full()
            .flex()
            .flex_col()
            // Terminal-style dark background
            .bg(gpui::rgb(0x0d0d0d))
            .child(self.render_header(cx))
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .bg(gpui::rgb(0x0d0d0d)) // Consistent terminal background
                    .map(|this| {
                        if let Some(thread_view) = &self.thread_view {
                            this.child(thread_view.clone())
                        } else {
                            this.child(
                                div()
                                    .size_full()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        Label::new("$ Initializing Ali...")
                                            .color(Color::Success) // Terminal green
                                    )
                            )
                        }
                    })
            )
    }
}
