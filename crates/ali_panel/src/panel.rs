//! Ali Command Center Panel
//!
//! A bottom-docked panel that provides always-on access to Ali,
//! the Chief of Staff who orchestrates all Convergio agents.
//! This panel embeds a terminal running `convergio` CLI directly.

use crate::AliPanelSettings;
use anyhow::Result;
use db::kvp::KEY_VALUE_STORE;
use gpui::{
    actions, div, prelude::*, Action, App, AsyncWindowContext, Context, Entity,
    EventEmitter, FocusHandle, Focusable, InteractiveElement, ParentElement, Pixels,
    Render, Styled, Subscription, Task, WeakEntity, Window,
};
use project::Project;
use serde::{Deserialize, Serialize};
use collections::HashMap;
use task::{Shell, SpawnInTerminal, TaskId};
use terminal_view::TerminalView;
use ui::{prelude::*, Button, Icon, IconName, IconSize, Label, Tooltip};
use util::ResultExt;
use workspace::{
    Workspace, WorkspaceId,
    dock::{DockPosition, Panel, PanelEvent},
};

const ALI_PANEL_KEY: &str = "AliPanel";
const ALI_BUILD_VERSION: &str = "v1.0.1 build 2025-12-21 21:00";

actions!(
    ali_panel,
    [
        ToggleFocus,
        RestartConvergio,
    ]
);

#[derive(Serialize, Deserialize)]
struct SerializedAliPanel {
    height: Option<f32>,
}

pub struct AliPanel {
    focus_handle: FocusHandle,
    height: Option<Pixels>,
    terminal_view: Option<Entity<TerminalView>>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    workspace_id: Option<WorkspaceId>,
    _subscriptions: Vec<Subscription>,
}

pub fn init(cx: &mut App) {
    AliPanelSettings::register(cx);

    cx.observe_new(|workspace: &mut Workspace, _, _| {
        // ToggleFocus should toggle the AliPanel in the bottom dock
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<AliPanel>(window, cx);
        });

        // RestartConvergio restarts the convergio CLI process
        workspace.register_action(|workspace, _: &RestartConvergio, window, cx| {
            if let Some(panel) = workspace.panel::<AliPanel>(cx) {
                panel.update(cx, |ali_panel, cx| {
                    ali_panel.restart_convergio(window, cx);
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
        workspace_id: Option<WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let weak_workspace = workspace.weak_handle();

        let mut panel = Self {
            focus_handle,
            height: None,
            terminal_view: None,
            workspace: weak_workspace,
            project,
            workspace_id,
            _subscriptions: Vec::new(),
        };

        // Spawn convergio CLI terminal
        panel.spawn_convergio_terminal(window, cx);

        panel
    }

    /// Spawn a terminal running `convergio` CLI
    fn spawn_convergio_terminal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let project = self.project.clone();
        let workspace = self.workspace.clone();
        let workspace_id = self.workspace_id;

        // Get the working directory from the project
        let cwd = project.read(cx).worktrees(cx).next()
            .and_then(|wt| wt.read(cx).abs_path().to_path_buf().into())
            .or_else(|| std::env::current_dir().ok());

        // Create a SpawnInTerminal for convergio CLI
        let spawn_task = SpawnInTerminal {
            id: TaskId("ali-convergio".into()),
            full_label: "Convergio CLI".into(),
            label: "Ali Command Center".into(),
            command: Some("convergio".into()),
            args: vec![],
            command_label: "convergio".into(),
            cwd,
            env: HashMap::default(),
            use_new_terminal: true,
            allow_concurrent_runs: false,
            reveal: task::RevealStrategy::Always,
            reveal_target: task::RevealTarget::Dock,
            hide: task::HideStrategy::Never,
            show_command: false,
            show_summary: false,
            show_rerun: false,
            shell: Shell::System,
        };

        let project_weak = project.downgrade();
        let terminal_task = project.update(cx, |project, cx| {
            project.create_terminal_task(spawn_task, cx)
        });

        cx.spawn_in(window, async move |this, cx| {
            let terminal = terminal_task.await?;

            this.update_in(cx, |this, window, cx| {
                let terminal_view = cx.new(|cx| {
                    TerminalView::new(
                        terminal,
                        workspace,
                        workspace_id,
                        project_weak,
                        window,
                        cx,
                    )
                });
                this.terminal_view = Some(terminal_view);
                cx.notify();
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    /// Restart the convergio CLI process
    pub fn restart_convergio(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        log::info!("Ali Command Center: Restarting convergio CLI");
        self.terminal_view = None;
        self.spawn_convergio_terminal(window, cx);
        cx.notify();
    }

    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        workspace.update_in(&mut cx, |workspace, window, cx| {
            let project = workspace.project().clone();
            let workspace_id = workspace.database_id();

            Ok(cx.new(|cx| AliPanel::new(workspace, project, workspace_id, window, cx)))
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
                        Label::new(format!("ALI - Command Center [{}]", ALI_BUILD_VERSION))
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
                        Button::new("restart-convergio", "Restart")
                            .icon(IconName::RotateCw)
                            .icon_size(IconSize::Small)
                            .icon_position(ui::IconPosition::Start)
                            .style(ui::ButtonStyle::Subtle)
                            .tooltip(Tooltip::text("Restart Convergio CLI"))
                            .on_click(|_, window, cx| {
                                window.dispatch_action(Box::new(RestartConvergio), cx);
                            })
                    )
            )
    }
}

impl Focusable for AliPanel {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        // Delegate focus to the terminal view if it exists
        if let Some(terminal_view) = &self.terminal_view {
            terminal_view.focus_handle(cx)
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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                        if let Some(terminal_view) = &self.terminal_view {
                            this.child(terminal_view.clone())
                        } else {
                            this.child(
                                div()
                                    .size_full()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        Label::new("$ Starting Convergio CLI...")
                                            .color(Color::Success) // Terminal green
                                    )
                            )
                        }
                    })
            )
    }
}
