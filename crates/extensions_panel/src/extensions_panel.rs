mod extension;
mod extensions_panel_settings;
use anyhow::{anyhow, Result};
use client::{ErrorCode, ErrorExt};
use db::kvp::KEY_VALUE_STORE;
use extensions_panel_settings::{ExtensionsPanelDockPosition, ExtensionsPanelSettings};
use gpui::{
    actions, div, overlay, px, uniform_list, Action, AppContext, AssetSource, AsyncWindowContext,
    ClipboardItem, DismissEvent, Div, EventEmitter, FocusHandle, FocusableView, InteractiveElement,
    KeyContext, Model, MouseButton, MouseDownEvent, ParentElement, Pixels, Point, PromptLevel,
    Render, Stateful, Styled, Subscription, Task, UniformListScrollHandle, View, ViewContext,
    VisualContext as _, WeakView, WindowContext,
};
use project::{
    repository::GitFileStatus, Entry, EntryKind, Fs, Project, ProjectEntryId, ProjectPath,
    Worktree, WorktreeId,
};
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::{cmp::Ordering, ffi::OsStr, ops::Range, path::Path, sync::Arc};
use ui::{prelude::*, v_flex, ContextMenu, Icon, KeyBinding, Label, ListItem};
use util::{maybe, ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    notifications::DetachAndPromptErr,
    Workspace,
};

#[derive(Debug)]
pub enum Event {
    OpenedEntry { focus_opened_item: bool },
    SplitEntry {},
    Focus,
}

const EXTENSIONS_PANEL_KEY: &'static str = "ExtensionsPanel";
pub struct ExtensionsPanel {
    fs: Arc<dyn Fs>,
    list: UniformListScrollHandle,
    focus_handle: FocusHandle,

    selection: Option<Selection>,
    context_menu: Option<(View<ContextMenu>, Point<Pixels>, Subscription)>,
    workspace: WeakView<Workspace>,
    width: Option<Pixels>,
    pending_serialization: Task<Option<()>>,
}

actions!(
    extensions_panel,
    [
        ExpandSelectedEntry,
        CollapseSelectedEntry,
        CollapseAllEntries,
        NewDirectory,
        NewFile,
        Copy,
        CopyPath,
        CopyRelativePath,
        RevealInFinder,
        OpenInTerminal,
        Cut,
        Paste,
        Delete,
        Rename,
        Open,
        ToggleFocus,
        NewSearchInDirectory,
    ]
);

pub fn init_settings(cx: &mut AppContext) {
    ExtensionsPanelSettings::register(cx);
}

pub fn init(cx: &mut AppContext) {
    init_settings(cx);

    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, cx| {
            workspace.toggle_panel_focus::<ExtensionsPanel>(cx);
        });
    })
    .detach();
}

#[derive(Serialize, Deserialize)]
struct SerializedExtensionsPanel {
    width: Option<Pixels>,
}

impl ExtensionsPanel {
    fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        let extensions_panel = cx.new_view(|cx: &mut ViewContext<Self>| {
            let focus_handle = cx.focus_handle();

            cx.on_focus(&focus_handle, Self::focus_in).detach();
            let mut this = Self {
                fs: workspace.app_state().fs.clone(),
                list: UniformListScrollHandle::new(),
                focus_handle,
                selection: None,
                context_menu: None,
                workspace: workspace.weak_handle(),
                width: None,
                pending_serialization: Task::ready(None),
            };

            this
        });
        extensions_panel
    }
    pub async fn load(
        workspace: WeakView<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<View<Self>> {
        let serialized_panel = cx
            .background_executor()
            .spawn(async move { KEY_VALUE_STORE.read_kvp(EXTENSIONS_PANEL_KEY) })
            .await
            .map_err(|e| anyhow!("Failed to load extensions panel: {}", e))
            .log_err()
            .flatten()
            .map(|panel| serde_json::from_str::<SerializedExtensionsPanel>(&panel))
            .transpose()
            .log_err()
            .flatten();

        workspace.update(&mut cx, |workspace, cx| {
            let panel = ExtensionsPanel::new(workspace, cx);
            if let Some(serialized_panel) = serialized_panel {
                panel.update(cx, |panel, cx| {
                    panel.width = serialized_panel.width;
                    cx.notify();
                });
            }
            panel
        })
    }
    fn serialize(&mut self, cx: &mut ViewContext<Self>) {
        let width = self.width;
        self.pending_serialization = cx.background_executor().spawn(
            async move {
                KEY_VALUE_STORE
                    .write_kvp(
                        EXTENSIONS_PANEL_KEY.into(),
                        serde_json::to_string(&SerializedExtensionsPanel { width })?,
                    )
                    .await?;
                anyhow::Ok(())
            }
            .log_err(),
        );
    }

    fn focus_in(&mut self, cx: &mut ViewContext<Self>) {
        if !self.focus_handle.contains_focused(cx) {
            cx.emit(Event::Focus);
        }
    }

    fn render_entry(&self, details: Entry, cx: &mut ViewContext<Self>) -> Div {
        div().child(div().child(Label::new(details.name).render(cx)))
    }
}

impl EventEmitter<Event> for ExtensionsPanel {}

impl EventEmitter<PanelEvent> for ExtensionsPanel {}

impl Panel for ExtensionsPanel {
    fn position(&self, cx: &WindowContext) -> DockPosition {
        match ExtensionsPanelSettings::get_global(cx).dock {
            ExtensionsPanelDockPosition::Left => DockPosition::Left,
            ExtensionsPanelDockPosition::Right => DockPosition::Right,
        }
    }
    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
        settings::update_settings_file::<ExtensionsPanelSettings>(
            self.fs.clone(),
            cx,
            move |settings| {
                let dock = match position {
                    DockPosition::Left | DockPosition::Bottom => ExtensionsPanelDockPosition::Left,
                    DockPosition::Right => ExtensionsPanelDockPosition::Right,
                };
                settings.dock = Some(dock);
            },
        );
    }

    fn size(&self, cx: &WindowContext) -> Pixels {
        self.width
            .unwrap_or_else(|| ExtensionsPanelSettings::get_global(cx).default_width)
    }

    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
    }

    fn icon(&self, _: &WindowContext) -> Option<ui::IconName> {
        Some(ui::IconName::Extensions)
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        Some("Extensions Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn persistent_name() -> &'static str {
        "Extensions Panel"
    }

    fn starts_open(&self, cx: &WindowContext) -> bool {
        true
        // self.project.read(cx).visible_worktrees(cx).any(|tree| {
        //     tree.read(cx)
        //         .root_entry()
        //         .map_or(false, |entry| entry.is_dir())
        // })
    }
}

impl Render for ExtensionsPanel {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .id("empty-extensions_panel")
            .size_full()
            .p_4()
            .track_focus(&self.focus_handle)
            .child(
                Button::new("open_extensions", "Open a extension")
                    .style(ButtonStyle::Filled)
                    .full_width()
                    // .key_binding(KeyBinding::for_action(&workspace::Open, cx))
                    .on_click(cx.listener(|this, _, cx| {
                        // this.workspace
                        //     .update(cx, |workspace, cx| workspace.open(&workspace::Open, cx))
                        //     .log_err();
                    })),
            )
    }
}

impl FocusableView for ExtensionsPanel {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}
