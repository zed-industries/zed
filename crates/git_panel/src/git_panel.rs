mod git_panel_settings;

use std::sync::Arc;

use anyhow::Context;
use db::kvp::KEY_VALUE_STORE;
use editor::Editor;
use file_icons::FileIcons;
use gpui::{
    actions, impl_actions, Action, AppContext, AssetSource, AsyncWindowContext, EventEmitter,
    FocusHandle, FocusableView, InteractiveElement, IntoElement, KeyContext, Pixels, Render,
    Styled, Subscription, Task, View, ViewContext, VisualContext, WeakView, WindowContext,
};

use git_panel_settings::{GitPanelDockPosition, GitPanelSettings};
use project::Fs;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use theme::ThemeSettings;
use util::{ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    ui::{v_flex, IconName},
    Workspace,
};

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct Open {
    change_selection: bool,
}

impl_actions!(outline_panel, [Open]);

actions!(
    outline_panel,
    [
        CollapseAllEntries,
        CollapseSelectedEntry,
        CopyPath,
        CopyRelativePath,
        ExpandAllEntries,
        ExpandSelectedEntry,
        FoldDirectory,
        ToggleActiveEditorPin,
        RevealInFileManager,
        SelectParent,
        ToggleFocus,
        UnfoldDirectory,
    ]
);

const OUTLINE_PANEL_KEY: &str = "GitPanel";

pub struct GitPanel {
    fs: Arc<dyn Fs>,
    width: Option<Pixels>,
    active: bool,
    focus_handle: FocusHandle,
    pending_serialization: Task<Option<()>>,
    _subscriptions: Vec<Subscription>,
    filter_editor: View<Editor>,
}

#[derive(Debug)]
pub enum Event {
    Focus,
}

#[derive(Serialize, Deserialize)]
struct SerializedOutlinePanel {
    width: Option<Pixels>,
    active: Option<bool>,
}

pub fn init_settings(cx: &mut AppContext) {
    GitPanelSettings::register(cx);
}

pub fn init(assets: impl AssetSource, cx: &mut AppContext) {
    init_settings(cx);
    file_icons::init(assets, cx);

    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, cx| {
            workspace.toggle_panel_focus::<GitPanel>(cx);
        });
    })
    .detach();
}

impl GitPanel {
    pub async fn load(
        workspace: WeakView<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> anyhow::Result<View<Self>> {
        let serialized_panel = cx
            .background_executor()
            .spawn(async move { KEY_VALUE_STORE.read_kvp(OUTLINE_PANEL_KEY) })
            .await
            .context("loading git panel")
            .log_err()
            .flatten()
            .map(|panel| serde_json::from_str::<SerializedOutlinePanel>(&panel))
            .transpose()
            .log_err()
            .flatten();

        workspace.update(&mut cx, |workspace, cx| {
            let panel = Self::new(workspace, cx);
            if let Some(serialized_panel) = serialized_panel {
                panel.update(cx, |panel, cx| {
                    panel.width = serialized_panel.width.map(|px| px.round());
                    panel.active = serialized_panel.active.unwrap_or(false);
                    cx.notify();
                });
            }
            panel
        })
    }

    fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        let git_panel = cx.new_view(|cx| {
            let filter_editor = cx.new_view(|cx| {
                let mut editor = Editor::single_line(cx);
                editor.set_placeholder_text("Filter...", cx);
                editor
            });

            let focus_handle = cx.focus_handle();

            let icons_subscription = cx.observe_global::<FileIcons>(|_, cx| {
                cx.notify();
            });

            let mut git_panel_settings = *GitPanelSettings::get_global(cx);
            let mut current_theme = ThemeSettings::get_global(cx).clone();
            let settings_subscription = cx.observe_global::<SettingsStore>(move |git_panel, cx| {
                let new_settings = GitPanelSettings::get_global(cx);
                let new_theme = ThemeSettings::get_global(cx);
                if &current_theme != new_theme {
                    git_panel_settings = *new_settings;
                    current_theme = new_theme.clone();
                } else if &git_panel_settings != new_settings {
                    git_panel_settings = *new_settings;
                    cx.notify();
                }
            });

            let git_panel = Self {
                active: false,
                fs: workspace.app_state().fs.clone(),
                focus_handle,
                filter_editor,
                width: None,
                pending_serialization: Task::ready(None),
                _subscriptions: vec![settings_subscription, icons_subscription],
            };
            git_panel
        });

        git_panel
    }

    fn serialize(&mut self, cx: &mut ViewContext<Self>) {
        let width = self.width;
        let active = Some(self.active);
        self.pending_serialization = cx.background_executor().spawn(
            async move {
                KEY_VALUE_STORE
                    .write_kvp(
                        OUTLINE_PANEL_KEY.into(),
                        serde_json::to_string(&SerializedOutlinePanel { width, active })?,
                    )
                    .await?;
                anyhow::Ok(())
            }
            .log_err(),
        );
    }

    fn dispatch_context(&self, cx: &ViewContext<Self>) -> KeyContext {
        let mut dispatch_context = KeyContext::new_with_defaults();
        dispatch_context.add("GitPanel");
        dispatch_context.add("menu");
        let identifier = if self.filter_editor.focus_handle(cx).is_focused(cx) {
            "editing"
        } else {
            "not_editing"
        };
        dispatch_context.add(identifier);
        dispatch_context
    }
}

impl Panel for GitPanel {
    fn persistent_name() -> &'static str {
        "Outline Panel"
    }

    fn position(&self, cx: &WindowContext) -> DockPosition {
        match GitPanelSettings::get_global(cx).dock {
            GitPanelDockPosition::Left => DockPosition::Left,
            GitPanelDockPosition::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
        settings::update_settings_file::<GitPanelSettings>(
            self.fs.clone(),
            cx,
            move |settings, _| {
                let dock = match position {
                    DockPosition::Left | DockPosition::Bottom => GitPanelDockPosition::Left,
                    DockPosition::Right => GitPanelDockPosition::Right,
                };
                settings.dock = Some(dock);
            },
        );
    }

    fn size(&self, cx: &WindowContext) -> Pixels {
        self.width
            .unwrap_or_else(|| GitPanelSettings::get_global(cx).default_width)
    }

    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
    }

    fn icon(&self, cx: &WindowContext) -> Option<IconName> {
        GitPanelSettings::get_global(cx)
            .button
            .then_some(IconName::AiGoogle)
    }

    fn icon_tooltip(&self, _: &WindowContext) -> Option<&'static str> {
        Some("Git Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn starts_open(&self, _: &WindowContext) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        cx.spawn(|outline_panel, mut cx| async move {
            outline_panel
                .update(&mut cx, |outline_panel, cx| {
                    outline_panel.active = active;

                    outline_panel.serialize(cx);
                })
                .ok();
        })
        .detach()
    }
}

impl FocusableView for GitPanel {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.filter_editor.focus_handle(cx).clone()
    }
}

impl EventEmitter<Event> for GitPanel {}

impl EventEmitter<PanelEvent> for GitPanel {}

impl Render for GitPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let outline_panel = v_flex()
            .id("git-panel")
            .size_full()
            .relative()
            .key_context(self.dispatch_context(cx))
            .track_focus(&self.focus_handle);

        outline_panel
    }
}
