use std::{
    collections::BTreeMap,
    path::{Component, Path},
};

use anyhow::{Result, anyhow};
use extension::{
    ExtensionHostProxy, ExtensionPanelAction, ExtensionPanelActionProxy, ExtensionPanelDescriptor,
    ExtensionPanelEvent, ExtensionPanelId, ExtensionPanelLocation, ExtensionPanelUiProxy,
};
use gpui::{
    Action, App, AppContext as _, Context, Entity, EventEmitter, FocusHandle, Focusable,
    InteractiveElement as _, IntoElement, ParentElement, Render, StatefulInteractiveElement as _,
    Styled, Window, actions, div, px,
};
use ui::{Button, Clickable, FluentBuilder as _, IconName};
use ui_input::InputField;

use crate::{Panel, Workspace, WorkspaceStore, dock::DockPosition, dock::PanelEvent};

const EXTENSION_PANELS_KEY: &str = "extension-panels";

actions!(extension_panels, [ToggleExtensionPanels]);

/// A single dock panel which hosts the persistent views requested by extensions.
///
/// Keeping one native panel lets Zed persist dock placement while extensions own
/// their independent identifiers and structured transcript events.
pub struct ExtensionPanels {
    panels: BTreeMap<ExtensionPanelId, ExtensionPanelContent>,
    focus_handle: FocusHandle,
    position: DockPosition,
    is_zoomed: bool,
}

struct ExtensionPanelContent {
    title: String,
    actions: Vec<extension::ExtensionPanelActionDescriptor>,
    input: Option<Entity<InputField>>,
    events: Vec<ExtensionPanelEvent>,
}

impl ExtensionPanels {
    fn new(location: ExtensionPanelLocation, cx: &mut Context<Self>) -> Self {
        Self {
            panels: BTreeMap::new(),
            focus_handle: cx.focus_handle(),
            position: match location {
                ExtensionPanelLocation::Right => DockPosition::Right,
                ExtensionPanelLocation::Bottom => DockPosition::Bottom,
            },
            is_zoomed: false,
        }
    }

    fn open(
        &mut self,
        descriptor: ExtensionPanelDescriptor,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let has_input_action = descriptor
            .actions
            .iter()
            .any(|action| action.requires_input);
        self.panels
            .entry(descriptor.id)
            .or_insert_with(|| ExtensionPanelContent {
                title: descriptor.title,
                actions: descriptor.actions,
                input: has_input_action
                    .then(|| cx.new(|cx| InputField::new(window, cx, "Clojure expression"))),
                events: Vec::new(),
            });
        cx.notify();
    }

    fn send_event(
        &mut self,
        panel: ExtensionPanelId,
        event: ExtensionPanelEvent,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let content = self
            .panels
            .get_mut(&panel)
            .ok_or_else(|| anyhow!("extension panel {} is not open", panel.panel_id))?;
        content.events.push(event);
        cx.notify();
        Ok(())
    }
}

impl EventEmitter<PanelEvent> for ExtensionPanels {}

impl Focusable for ExtensionPanels {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Panel for ExtensionPanels {
    fn persistent_name() -> &'static str {
        "ExtensionPanels"
    }

    fn panel_key() -> &'static str {
        EXTENSION_PANELS_KEY
    }

    fn position(&self, _window: &Window, _cx: &App) -> DockPosition {
        self.position
    }

    fn position_is_valid(&self, _position: DockPosition) -> bool {
        true
    }

    fn set_position(
        &mut self,
        position: DockPosition,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.position = position;
        cx.notify();
    }

    fn default_size(&self, _window: &Window, _cx: &App) -> gpui::Pixels {
        px(360.)
    }

    fn icon(&self, _window: &Window, _cx: &App) -> Option<IconName> {
        None
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Extension Panels")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleExtensionPanels)
    }

    fn activation_priority(&self) -> u32 {
        7
    }

    fn starts_open(&self, _window: &Window, _cx: &App) -> bool {
        true
    }

    fn is_zoomed(&self, _window: &Window, _cx: &App) -> bool {
        self.is_zoomed
    }

    fn set_zoomed(&mut self, zoomed: bool, _window: &mut Window, cx: &mut Context<Self>) {
        self.is_zoomed = zoomed;
        cx.notify();
    }
}

impl Render for ExtensionPanels {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("extension-panels")
            .track_focus(&self.focus_handle)
            .size_full()
            .overflow_y_scroll()
            .flex()
            .flex_col()
            .gap_2()
            .p_2()
            .children(self.panels.iter().map(|(id, content)| {
                div()
                    .id(format!(
                        "extension-panel-{}-{}",
                        id.extension_id, id.panel_id
                    ))
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(content.title.clone())
                    .when_some(content.input.clone(), |this, input| this.child(input))
                    .child(div().flex().gap_1().children(content.actions.iter().map(
                        |descriptor| {
                            let input = content.input.clone();
                            let requires_input = descriptor.requires_input;
                            let action = ExtensionPanelAction {
                                panel: id.clone(),
                                action: descriptor.id.clone(),
                                payload: serde_json::json!({}),
                            };
                            Button::new(
                                format!(
                                    "extension-panel-action-{}-{}-{}",
                                    id.extension_id, id.panel_id, descriptor.id
                                ),
                                descriptor.label.clone(),
                            )
                            .on_click(move |_, _, cx| {
                                let payload = input
                                    .as_ref()
                                    .filter(|_| requires_input)
                                    .map(|input| {
                                        let text = input.read(cx).text(cx);
                                        serde_json::json!({ "input": text })
                                    })
                                    .unwrap_or_else(|| serde_json::json!({}));
                                let action = ExtensionPanelAction {
                                    payload,
                                    ..action.clone()
                                };
                                let task = ExtensionHostProxy::global(cx)
                                    .dispatch_panel_action(action, cx);
                                cx.spawn(async move |_| {
                                    if let Err(error) = task.await {
                                        log::error!("extension panel action failed: {error:#}");
                                    }
                                })
                                .detach();
                            })
                        },
                    )))
                    .child(
                        div()
                            .border_t_1()
                            .pt_2()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .children(content.events.iter().map(render_panel_event)),
                    )
            }))
    }
}

fn render_panel_event(event: &ExtensionPanelEvent) -> impl IntoElement {
    let payload = event
        .payload
        .get("message")
        .or_else(|| event.payload.get("code"))
        .and_then(serde_json::Value::as_str)
        .map(str::to_string)
        .unwrap_or_else(|| event.payload.to_string());
    let (prefix, text) = match event.kind.as_ref() {
        "input" => ("> ", payload),
        "value" => ("=> ", payload),
        "stdout" => ("out  ", payload),
        "stderr" => ("err  ", payload),
        "exception" => ("ex   ", payload),
        "status" => ("· ", payload),
        _ => ("· ", payload),
    };
    div()
        .font_family(".ZedMono")
        .child(format!("{prefix}{text}"))
}

/// Connects extension-host panel calls to the first active workspace window.
pub struct ExtensionPanelsProxy {
    workspace_store: Entity<WorkspaceStore>,
}

impl ExtensionPanelsProxy {
    pub fn new(workspace_store: Entity<WorkspaceStore>) -> Self {
        Self { workspace_store }
    }

    fn first_workspace(&self, cx: &App) -> Result<(gpui::AnyWindowHandle, Entity<Workspace>)> {
        self.workspace_store
            .read(cx)
            .workspaces_with_windows()
            .find_map(|(window, workspace)| {
                workspace.upgrade().map(|workspace| (window, workspace))
            })
            .ok_or_else(|| anyhow!("no workspace is open for the extension panel"))
    }

    fn with_panel(
        &self,
        location: ExtensionPanelLocation,
        cx: &mut App,
        f: impl FnOnce(&Entity<ExtensionPanels>, &mut Window, &mut Context<Workspace>) -> Result<()>,
    ) -> Result<()> {
        let (window_handle, workspace_handle) = self.first_workspace(cx)?;
        window_handle.update(cx, |_, window, cx| {
            workspace_handle.update(cx, |workspace, cx| {
                let panel = workspace.panel::<ExtensionPanels>(cx).unwrap_or_else(|| {
                    let panel = cx.new(|cx| ExtensionPanels::new(location, cx));
                    workspace.add_panel(panel.clone(), window, cx);
                    panel
                });
                f(&panel, window, cx)
            })
        })??;
        Ok(())
    }
}

impl ExtensionPanelUiProxy for ExtensionPanelsProxy {
    fn open_panel(&self, descriptor: ExtensionPanelDescriptor, cx: &mut App) -> Result<()> {
        self.with_panel(descriptor.location, cx, |panel, window, cx| {
            // `with_panel` owns the window while the panel is opened; input
            // fields are constructed lazily inside the panel update below.
            panel.update(cx, |panel, cx| panel.open(descriptor, window, cx));
            Ok(())
        })
    }

    fn send_panel_event(
        &self,
        panel: ExtensionPanelId,
        event: ExtensionPanelEvent,
        cx: &mut App,
    ) -> Result<()> {
        self.with_panel(
            ExtensionPanelLocation::Right,
            cx,
            |extension_panels, _, cx| {
                extension_panels.update(cx, |extension_panels, cx| {
                    extension_panels.send_event(panel, event, cx)
                })
            },
        )
    }

    fn active_worktree_root(&self, cx: &mut App) -> Result<String> {
        let (_, workspace) = self.first_workspace(cx)?;
        let worktree = workspace
            .read(cx)
            .visible_worktrees(cx)
            .next()
            .ok_or_else(|| anyhow!("the active workspace has no visible worktree"))?;
        Ok(worktree.read(cx).abs_path().to_string_lossy().into_owned())
    }

    fn read_active_worktree_file(&self, path: &str, cx: &mut App) -> Result<String> {
        let relative_path = Path::new(path);
        if path.is_empty()
            || !relative_path
                .components()
                .all(|component| matches!(component, Component::Normal(_)))
        {
            return Err(anyhow!(
                "extension panel file path must be a relative path without traversal"
            ));
        }

        let (_, workspace) = self.first_workspace(cx)?;
        let worktree = workspace
            .read(cx)
            .visible_worktrees(cx)
            .next()
            .ok_or_else(|| anyhow!("the active workspace has no visible worktree"))?;
        let file_path = worktree.read(cx).abs_path().join(relative_path);
        std::fs::read_to_string(&file_path)
            .map_err(|error| anyhow!("failed to read {}: {error}", file_path.display()))
    }
}
