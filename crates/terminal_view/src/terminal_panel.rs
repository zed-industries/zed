use std::{path::PathBuf, sync::Arc};

use crate::TerminalView;
use db::kvp::KEY_VALUE_STORE;
use gpui::{
    actions, div, serde_json, AppContext, AsyncWindowContext, Entity, EventEmitter, ExternalPaths,
    FocusHandle, FocusableView, IntoElement, ParentElement, Pixels, Render, Styled, Subscription,
    Task, View, ViewContext, VisualContext, WeakView, WindowContext,
};
use project::Fs;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use terminal::terminal_settings::{TerminalDockPosition, TerminalSettings};
use ui::{h_stack, ButtonCommon, Clickable, IconButton, IconSize, Selectable, Tooltip};
use util::{ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    item::Item,
    pane,
    ui::Icon,
    Pane, Workspace,
};

use anyhow::Result;

const TERMINAL_PANEL_KEY: &'static str = "TerminalPanel";

actions!(terminal_panel, [ToggleFocus]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(
        |workspace: &mut Workspace, _: &mut ViewContext<Workspace>| {
            workspace.register_action(TerminalPanel::new_terminal);
            workspace.register_action(TerminalPanel::open_terminal);
            workspace.register_action(|workspace, _: &ToggleFocus, cx| {
                workspace.toggle_panel_focus::<TerminalPanel>(cx);
            });
        },
    )
    .detach();
}

pub struct TerminalPanel {
    pane: View<Pane>,
    fs: Arc<dyn Fs>,
    workspace: WeakView<Workspace>,
    width: Option<Pixels>,
    height: Option<Pixels>,
    pending_serialization: Task<Option<()>>,
    _subscriptions: Vec<Subscription>,
}

impl TerminalPanel {
    fn new(workspace: &Workspace, cx: &mut ViewContext<Self>) -> Self {
        let terminal_panel = cx.view().clone();
        let pane = cx.new_view(|cx| {
            let mut pane = Pane::new(
                workspace.weak_handle(),
                workspace.project().clone(),
                Default::default(),
                Some(Arc::new(|a, cx| {
                    if let Some(tab) = a.downcast_ref::<workspace::pane::DraggedTab>() {
                        if let Some(item) = tab.pane.read(cx).item_for_index(tab.ix) {
                            return item.downcast::<TerminalView>().is_some();
                        }
                    }
                    if a.downcast_ref::<ExternalPaths>().is_some() {
                        return true;
                    }

                    false
                })),
                cx,
            );
            pane.set_can_split(false, cx);
            pane.set_can_navigate(false, cx);
            pane.display_nav_history_buttons(false);
            pane.set_render_tab_bar_buttons(cx, move |pane, cx| {
                h_stack()
                    .gap_2()
                    .child(
                        IconButton::new("plus", Icon::Plus)
                            .icon_size(IconSize::Small)
                            .on_click(cx.listener_for(&terminal_panel, |terminal_panel, _, cx| {
                                terminal_panel.add_terminal(None, cx);
                            }))
                            .tooltip(|cx| Tooltip::text("New Terminal", cx)),
                    )
                    .child({
                        let zoomed = pane.is_zoomed();
                        IconButton::new("toggle_zoom", Icon::Maximize)
                            .icon_size(IconSize::Small)
                            .selected(zoomed)
                            .selected_icon(Icon::Minimize)
                            .on_click(cx.listener(|pane, _, cx| {
                                pane.toggle_zoom(&workspace::ToggleZoom, cx);
                            }))
                            .tooltip(move |cx| {
                                Tooltip::text(if zoomed { "Zoom Out" } else { "Zoom In" }, cx)
                            })
                    })
                    .into_any_element()
            });
            // let buffer_search_bar = cx.build_view(search::BufferSearchBar::new);
            // pane.toolbar()
            //     .update(cx, |toolbar, cx| toolbar.add_item(buffer_search_bar, cx));
            pane
        });
        let subscriptions = vec![
            cx.observe(&pane, |_, _, cx| cx.notify()),
            cx.subscribe(&pane, Self::handle_pane_event),
        ];
        let this = Self {
            pane,
            fs: workspace.app_state().fs.clone(),
            workspace: workspace.weak_handle(),
            pending_serialization: Task::ready(None),
            width: None,
            height: None,
            _subscriptions: subscriptions,
        };
        let mut old_dock_position = this.position(cx);
        cx.observe_global::<SettingsStore>(move |this, cx| {
            let new_dock_position = this.position(cx);
            if new_dock_position != old_dock_position {
                old_dock_position = new_dock_position;
                cx.emit(PanelEvent::ChangePosition);
            }
        })
        .detach();
        this
    }

    pub async fn load(
        workspace: WeakView<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<View<Self>> {
        let serialized_panel = cx
            .background_executor()
            .spawn(async move { KEY_VALUE_STORE.read_kvp(TERMINAL_PANEL_KEY) })
            .await
            .log_err()
            .flatten()
            .map(|panel| serde_json::from_str::<SerializedTerminalPanel>(&panel))
            .transpose()
            .log_err()
            .flatten();

        let (panel, pane, items) = workspace.update(&mut cx, |workspace, cx| {
            let panel = cx.new_view(|cx| TerminalPanel::new(workspace, cx));
            let items = if let Some(serialized_panel) = serialized_panel.as_ref() {
                panel.update(cx, |panel, cx| {
                    cx.notify();
                    panel.height = serialized_panel.height;
                    panel.width = serialized_panel.width;
                    panel.pane.update(cx, |_, cx| {
                        serialized_panel
                            .items
                            .iter()
                            .map(|item_id| {
                                TerminalView::deserialize(
                                    workspace.project().clone(),
                                    workspace.weak_handle(),
                                    workspace.database_id(),
                                    *item_id,
                                    cx,
                                )
                            })
                            .collect::<Vec<_>>()
                    })
                })
            } else {
                Default::default()
            };
            let pane = panel.read(cx).pane.clone();
            (panel, pane, items)
        })?;

        let pane = pane.downgrade();
        let items = futures::future::join_all(items).await;
        pane.update(&mut cx, |pane, cx| {
            let active_item_id = serialized_panel
                .as_ref()
                .and_then(|panel| panel.active_item_id);
            let mut active_ix = None;
            for item in items {
                if let Some(item) = item.log_err() {
                    let item_id = item.entity_id().as_u64();
                    pane.add_item(Box::new(item), false, false, None, cx);
                    if Some(item_id) == active_item_id {
                        active_ix = Some(pane.items_len() - 1);
                    }
                }
            }

            if let Some(active_ix) = active_ix {
                pane.activate_item(active_ix, false, false, cx)
            }
        })?;

        Ok(panel)
    }

    fn handle_pane_event(
        &mut self,
        _pane: View<Pane>,
        event: &pane::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            pane::Event::ActivateItem { .. } => self.serialize(cx),
            pane::Event::RemoveItem { .. } => self.serialize(cx),
            pane::Event::Remove => cx.emit(PanelEvent::Close),
            pane::Event::ZoomIn => cx.emit(PanelEvent::ZoomIn),
            pane::Event::ZoomOut => cx.emit(PanelEvent::ZoomOut),
            pane::Event::Focus => cx.emit(PanelEvent::Focus),

            pane::Event::AddItem { item } => {
                if let Some(workspace) = self.workspace.upgrade() {
                    let pane = self.pane.clone();
                    workspace.update(cx, |workspace, cx| item.added_to_pane(workspace, pane, cx))
                }
            }

            _ => {}
        }
    }

    pub fn open_terminal(
        workspace: &mut Workspace,
        action: &workspace::OpenTerminal,
        cx: &mut ViewContext<Workspace>,
    ) {
        let Some(this) = workspace.focus_panel::<Self>(cx) else {
            return;
        };

        this.update(cx, |this, cx| {
            this.add_terminal(Some(action.working_directory.clone()), cx)
        })
    }

    ///Create a new Terminal in the current working directory or the user's home directory
    fn new_terminal(
        workspace: &mut Workspace,
        _: &workspace::NewTerminal,
        cx: &mut ViewContext<Workspace>,
    ) {
        let Some(this) = workspace.focus_panel::<Self>(cx) else {
            return;
        };

        this.update(cx, |this, cx| this.add_terminal(None, cx))
    }

    fn add_terminal(&mut self, working_directory: Option<PathBuf>, cx: &mut ViewContext<Self>) {
        let workspace = self.workspace.clone();
        cx.spawn(|this, mut cx| async move {
            let pane = this.update(&mut cx, |this, _| this.pane.clone())?;
            workspace.update(&mut cx, |workspace, cx| {
                let working_directory = if let Some(working_directory) = working_directory {
                    Some(working_directory)
                } else {
                    let working_directory_strategy =
                        TerminalSettings::get_global(cx).working_directory.clone();
                    crate::get_working_directory(workspace, cx, working_directory_strategy)
                };

                let window = cx.window_handle();
                if let Some(terminal) = workspace.project().update(cx, |project, cx| {
                    project
                        .create_terminal(working_directory, window, cx)
                        .log_err()
                }) {
                    let terminal = Box::new(cx.new_view(|cx| {
                        TerminalView::new(
                            terminal,
                            workspace.weak_handle(),
                            workspace.database_id(),
                            cx,
                        )
                    }));
                    pane.update(cx, |pane, cx| {
                        let focus = pane.has_focus(cx);
                        pane.add_item(terminal, true, focus, None, cx);
                    });
                }
            })?;
            this.update(&mut cx, |this, cx| this.serialize(cx))?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn serialize(&mut self, cx: &mut ViewContext<Self>) {
        let items = self
            .pane
            .read(cx)
            .items()
            .map(|item| item.item_id().as_u64())
            .collect::<Vec<_>>();
        let active_item_id = self
            .pane
            .read(cx)
            .active_item()
            .map(|item| item.item_id().as_u64());
        let height = self.height;
        let width = self.width;
        self.pending_serialization = cx.background_executor().spawn(
            async move {
                KEY_VALUE_STORE
                    .write_kvp(
                        TERMINAL_PANEL_KEY.into(),
                        serde_json::to_string(&SerializedTerminalPanel {
                            items,
                            active_item_id,
                            height,
                            width,
                        })?,
                    )
                    .await?;
                anyhow::Ok(())
            }
            .log_err(),
        );
    }
}

impl EventEmitter<PanelEvent> for TerminalPanel {}

impl Render for TerminalPanel {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().size_full().child(self.pane.clone())
    }
}

impl FocusableView for TerminalPanel {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.pane.focus_handle(cx)
    }
}

impl Panel for TerminalPanel {
    fn position(&self, cx: &WindowContext) -> DockPosition {
        match TerminalSettings::get_global(cx).dock {
            TerminalDockPosition::Left => DockPosition::Left,
            TerminalDockPosition::Bottom => DockPosition::Bottom,
            TerminalDockPosition::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, _: DockPosition) -> bool {
        true
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
        settings::update_settings_file::<TerminalSettings>(self.fs.clone(), cx, move |settings| {
            let dock = match position {
                DockPosition::Left => TerminalDockPosition::Left,
                DockPosition::Bottom => TerminalDockPosition::Bottom,
                DockPosition::Right => TerminalDockPosition::Right,
            };
            settings.dock = Some(dock);
        });
    }

    fn size(&self, cx: &WindowContext) -> Pixels {
        let settings = TerminalSettings::get_global(cx);
        match self.position(cx) {
            DockPosition::Left | DockPosition::Right => {
                self.width.unwrap_or_else(|| settings.default_width)
            }
            DockPosition::Bottom => self.height.unwrap_or_else(|| settings.default_height),
        }
    }

    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>) {
        match self.position(cx) {
            DockPosition::Left | DockPosition::Right => self.width = size,
            DockPosition::Bottom => self.height = size,
        }
        self.serialize(cx);
        cx.notify();
    }

    fn is_zoomed(&self, cx: &WindowContext) -> bool {
        self.pane.read(cx).is_zoomed()
    }

    fn set_zoomed(&mut self, zoomed: bool, cx: &mut ViewContext<Self>) {
        self.pane.update(cx, |pane, cx| pane.set_zoomed(zoomed, cx));
    }

    fn set_active(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        if active && self.pane.read(cx).items_len() == 0 {
            self.add_terminal(None, cx)
        }
    }

    fn icon_label(&self, cx: &WindowContext) -> Option<String> {
        let count = self.pane.read(cx).items_len();
        if count == 0 {
            None
        } else {
            Some(count.to_string())
        }
    }

    fn persistent_name() -> &'static str {
        "TerminalPanel"
    }

    // todo!()
    // fn icon_tooltip(&self) -> (String, Option<Box<dyn Action>>) {
    //     ("Terminal Panel".into(), Some(Box::new(ToggleFocus)))
    // }

    fn icon(&self, _cx: &WindowContext) -> Option<Icon> {
        Some(Icon::Terminal)
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        Some("Terminal Panel")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }
}

#[derive(Serialize, Deserialize)]
struct SerializedTerminalPanel {
    items: Vec<u64>,
    active_item_id: Option<u64>,
    width: Option<Pixels>,
    height: Option<Pixels>,
}
