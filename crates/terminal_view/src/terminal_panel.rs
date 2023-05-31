use std::sync::Arc;

use crate::TerminalView;
use db::kvp::KEY_VALUE_STORE;
use gpui::{
    actions, anyhow::Result, elements::*, serde_json, Action, AppContext, AsyncAppContext, Entity,
    Subscription, Task, View, ViewContext, ViewHandle, WeakViewHandle, WindowContext,
};
use project::Fs;
use serde::{Deserialize, Serialize};
use settings::SettingsStore;
use terminal::{TerminalDockPosition, TerminalSettings};
use util::{ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel},
    item::Item,
    pane, DraggedItem, Pane, Workspace,
};

const TERMINAL_PANEL_KEY: &'static str = "TerminalPanel";

actions!(terminal_panel, [ToggleFocus]);

pub fn init(cx: &mut AppContext) {
    cx.add_action(TerminalPanel::new_terminal);
}

pub enum Event {
    Close,
    DockPositionChanged,
    ZoomIn,
    ZoomOut,
    Focus,
}

pub struct TerminalPanel {
    pane: ViewHandle<Pane>,
    fs: Arc<dyn Fs>,
    workspace: WeakViewHandle<Workspace>,
    width: Option<f32>,
    height: Option<f32>,
    pending_serialization: Task<Option<()>>,
    _subscriptions: Vec<Subscription>,
}

impl TerminalPanel {
    fn new(workspace: &Workspace, cx: &mut ViewContext<Self>) -> Self {
        let weak_self = cx.weak_handle();
        let pane = cx.add_view(|cx| {
            let window_id = cx.window_id();
            let mut pane = Pane::new(
                workspace.weak_handle(),
                workspace.app_state().background_actions,
                Default::default(),
                cx,
            );
            pane.set_can_split(false, cx);
            pane.set_can_navigate(false, cx);
            pane.on_can_drop(move |drag_and_drop, cx| {
                drag_and_drop
                    .currently_dragged::<DraggedItem>(window_id)
                    .map_or(false, |(_, item)| {
                        item.handle.act_as::<TerminalView>(cx).is_some()
                    })
            });
            pane.set_render_tab_bar_buttons(cx, move |pane, cx| {
                let this = weak_self.clone();
                Flex::row()
                    .with_child(Pane::render_tab_bar_button(
                        0,
                        "icons/plus_12.svg",
                        false,
                        Some((
                            "New Terminal".into(),
                            Some(Box::new(workspace::NewTerminal)),
                        )),
                        cx,
                        move |_, cx| {
                            let this = this.clone();
                            cx.window_context().defer(move |cx| {
                                if let Some(this) = this.upgrade(cx) {
                                    this.update(cx, |this, cx| {
                                        this.add_terminal(cx);
                                    });
                                }
                            })
                        },
                        None,
                    ))
                    .with_child(Pane::render_tab_bar_button(
                        1,
                        if pane.is_zoomed() {
                            "icons/minimize_8.svg"
                        } else {
                            "icons/maximize_8.svg"
                        },
                        pane.is_zoomed(),
                        Some(("Toggle Zoom".into(), Some(Box::new(workspace::ToggleZoom)))),
                        cx,
                        move |pane, cx| pane.toggle_zoom(&Default::default(), cx),
                        None,
                    ))
                    .into_any()
            });
            let buffer_search_bar = cx.add_view(search::BufferSearchBar::new);
            pane.toolbar()
                .update(cx, |toolbar, cx| toolbar.add_item(buffer_search_bar, cx));
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
        cx.observe_global::<SettingsStore, _>(move |this, cx| {
            let new_dock_position = this.position(cx);
            if new_dock_position != old_dock_position {
                old_dock_position = new_dock_position;
                cx.emit(Event::DockPositionChanged);
            }
        })
        .detach();
        this
    }

    pub fn load(
        workspace: WeakViewHandle<Workspace>,
        cx: AsyncAppContext,
    ) -> Task<Result<ViewHandle<Self>>> {
        cx.spawn(|mut cx| async move {
            let serialized_panel = if let Some(panel) = cx
                .background()
                .spawn(async move { KEY_VALUE_STORE.read_kvp(TERMINAL_PANEL_KEY) })
                .await
                .log_err()
                .flatten()
            {
                Some(serde_json::from_str::<SerializedTerminalPanel>(&panel)?)
            } else {
                None
            };
            let (panel, pane, items) = workspace.update(&mut cx, |workspace, cx| {
                let panel = cx.add_view(|cx| TerminalPanel::new(workspace, cx));
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

            let items = futures::future::join_all(items).await;
            workspace.update(&mut cx, |workspace, cx| {
                let active_item_id = serialized_panel
                    .as_ref()
                    .and_then(|panel| panel.active_item_id);
                let mut active_ix = None;
                for item in items {
                    if let Some(item) = item.log_err() {
                        let item_id = item.id();
                        Pane::add_item(workspace, &pane, Box::new(item), false, false, None, cx);
                        if Some(item_id) == active_item_id {
                            active_ix = Some(pane.read(cx).items_len() - 1);
                        }
                    }
                }

                if let Some(active_ix) = active_ix {
                    pane.update(cx, |pane, cx| {
                        pane.activate_item(active_ix, false, false, cx)
                    });
                }
            })?;

            Ok(panel)
        })
    }

    fn handle_pane_event(
        &mut self,
        _pane: ViewHandle<Pane>,
        event: &pane::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            pane::Event::ActivateItem { .. } => self.serialize(cx),
            pane::Event::RemoveItem { .. } => self.serialize(cx),
            pane::Event::Remove => cx.emit(Event::Close),
            pane::Event::ZoomIn => cx.emit(Event::ZoomIn),
            pane::Event::ZoomOut => cx.emit(Event::ZoomOut),
            pane::Event::Focus => cx.emit(Event::Focus),
            _ => {}
        }
    }

    fn new_terminal(
        workspace: &mut Workspace,
        _: &workspace::NewTerminal,
        cx: &mut ViewContext<Workspace>,
    ) {
        let Some(this) = workspace.focus_panel::<Self>(cx) else {
            return;
        };

        this.update(cx, |this, cx| this.add_terminal(cx))
    }

    fn add_terminal(&mut self, cx: &mut ViewContext<Self>) {
        let workspace = self.workspace.clone();
        cx.spawn(|this, mut cx| async move {
            let pane = this.read_with(&cx, |this, _| this.pane.clone())?;
            workspace.update(&mut cx, |workspace, cx| {
                let working_directory_strategy = settings::get::<TerminalSettings>(cx)
                    .working_directory
                    .clone();
                let working_directory =
                    crate::get_working_directory(workspace, cx, working_directory_strategy);
                let window_id = cx.window_id();
                if let Some(terminal) = workspace.project().update(cx, |project, cx| {
                    project
                        .create_terminal(working_directory, window_id, cx)
                        .log_err()
                }) {
                    let terminal =
                        Box::new(cx.add_view(|cx| {
                            TerminalView::new(terminal, workspace.database_id(), cx)
                        }));
                    let focus = pane.read(cx).has_focus();
                    Pane::add_item(workspace, &pane, terminal, true, focus, None, cx);
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
            .map(|item| item.id())
            .collect::<Vec<_>>();
        let active_item_id = self.pane.read(cx).active_item().map(|item| item.id());
        let height = self.height;
        let width = self.width;
        self.pending_serialization = cx.background().spawn(
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

impl Entity for TerminalPanel {
    type Event = Event;
}

impl View for TerminalPanel {
    fn ui_name() -> &'static str {
        "TerminalPanel"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> gpui::AnyElement<Self> {
        ChildView::new(&self.pane, cx).into_any()
    }

    fn focus_in(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        if cx.is_self_focused() {
            cx.focus(&self.pane);
        }
    }
}

impl Panel for TerminalPanel {
    fn position(&self, cx: &WindowContext) -> DockPosition {
        match settings::get::<TerminalSettings>(cx).dock {
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

    fn size(&self, cx: &WindowContext) -> f32 {
        let settings = settings::get::<TerminalSettings>(cx);
        match self.position(cx) {
            DockPosition::Left | DockPosition::Right => {
                self.width.unwrap_or_else(|| settings.default_width)
            }
            DockPosition::Bottom => self.height.unwrap_or_else(|| settings.default_height),
        }
    }

    fn set_size(&mut self, size: f32, cx: &mut ViewContext<Self>) {
        match self.position(cx) {
            DockPosition::Left | DockPosition::Right => self.width = Some(size),
            DockPosition::Bottom => self.height = Some(size),
        }
        self.serialize(cx);
        cx.notify();
    }

    fn should_zoom_in_on_event(event: &Event) -> bool {
        matches!(event, Event::ZoomIn)
    }

    fn should_zoom_out_on_event(event: &Event) -> bool {
        matches!(event, Event::ZoomOut)
    }

    fn is_zoomed(&self, cx: &WindowContext) -> bool {
        self.pane.read(cx).is_zoomed()
    }

    fn set_zoomed(&mut self, zoomed: bool, cx: &mut ViewContext<Self>) {
        self.pane.update(cx, |pane, cx| pane.set_zoomed(zoomed, cx));
    }

    fn set_active(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        if active && self.pane.read(cx).items_len() == 0 {
            self.add_terminal(cx)
        }
    }

    fn icon_path(&self) -> &'static str {
        "icons/terminal_12.svg"
    }

    fn icon_tooltip(&self) -> (String, Option<Box<dyn Action>>) {
        ("Terminal Panel".into(), Some(Box::new(ToggleFocus)))
    }

    fn icon_label(&self, cx: &WindowContext) -> Option<String> {
        let count = self.pane.read(cx).items_len();
        if count == 0 {
            None
        } else {
            Some(count.to_string())
        }
    }

    fn should_change_position_on_event(event: &Self::Event) -> bool {
        matches!(event, Event::DockPositionChanged)
    }

    fn should_activate_on_event(_: &Self::Event) -> bool {
        false
    }

    fn should_close_on_event(event: &Event) -> bool {
        matches!(event, Event::Close)
    }

    fn has_focus(&self, cx: &WindowContext) -> bool {
        self.pane.read(cx).has_focus()
    }

    fn is_focus_event(event: &Self::Event) -> bool {
        matches!(event, Event::Focus)
    }
}

#[derive(Serialize, Deserialize)]
struct SerializedTerminalPanel {
    items: Vec<usize>,
    active_item_id: Option<usize>,
    width: Option<f32>,
    height: Option<f32>,
}
