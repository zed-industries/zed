use crate::persistence::model::DockData;
use crate::{DraggedDock, Event, ModalLayer, Pane};
use crate::{Workspace, status_bar::StatusItemView};
use anyhow::Context as _;
use client::proto;
use gpui::{
    Action, AnyView, App, Axis, Context, Corner, Entity, EntityId, EventEmitter, FocusHandle,
    Focusable, IntoElement, KeyContext, MouseButton, MouseDownEvent, MouseUpEvent, ParentElement,
    Render, SharedString, StyleRefinement, Styled, Subscription, WeakEntity, Window, deferred, div,
    px,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::SettingsStore;
use std::sync::Arc;
use ui::{ContextMenu, Divider, DividerColor, IconButton, Tooltip, h_flex};
use ui::{prelude::*, right_click_menu};

pub(crate) const RESIZE_HANDLE_SIZE: Pixels = Pixels(6.);

pub enum PanelEvent {
    ZoomIn,
    ZoomOut,
    Activate,
    Close,
}

pub use proto::PanelId;

pub trait Panel: Focusable + EventEmitter<PanelEvent> + Render + Sized {
    fn persistent_name() -> &'static str;
    fn position(&self, window: &Window, cx: &App) -> DockPosition;
    fn position_is_valid(&self, position: DockPosition) -> bool;
    fn set_position(&mut self, position: DockPosition, window: &mut Window, cx: &mut Context<Self>);
    fn size(&self, window: &Window, cx: &App) -> Pixels;
    fn set_size(&mut self, size: Option<Pixels>, window: &mut Window, cx: &mut Context<Self>);
    fn icon(&self, window: &Window, cx: &App) -> Option<ui::IconName>;
    fn icon_tooltip(&self, window: &Window, cx: &App) -> Option<&'static str>;
    fn toggle_action(&self) -> Box<dyn Action>;
    fn icon_label(&self, _window: &Window, _: &App) -> Option<String> {
        None
    }
    fn is_zoomed(&self, _window: &Window, _cx: &App) -> bool {
        false
    }
    fn starts_open(&self, _window: &Window, _cx: &App) -> bool {
        false
    }
    fn set_zoomed(&mut self, _zoomed: bool, _window: &mut Window, _cx: &mut Context<Self>) {}
    fn set_active(&mut self, _active: bool, _window: &mut Window, _cx: &mut Context<Self>) {}
    fn pane(&self) -> Option<Entity<Pane>> {
        None
    }
    fn remote_id() -> Option<proto::PanelId> {
        None
    }
    fn activation_priority(&self) -> u32;
    fn enabled(&self, _cx: &App) -> bool {
        true
    }
}

pub trait PanelHandle: Send + Sync {
    fn panel_id(&self) -> EntityId;
    fn persistent_name(&self) -> &'static str;
    fn position(&self, window: &Window, cx: &App) -> DockPosition;
    fn position_is_valid(&self, position: DockPosition, cx: &App) -> bool;
    fn set_position(&self, position: DockPosition, window: &mut Window, cx: &mut App);
    fn is_zoomed(&self, window: &Window, cx: &App) -> bool;
    fn set_zoomed(&self, zoomed: bool, window: &mut Window, cx: &mut App);
    fn set_active(&self, active: bool, window: &mut Window, cx: &mut App);
    fn remote_id(&self) -> Option<proto::PanelId>;
    fn pane(&self, cx: &App) -> Option<Entity<Pane>>;
    fn size(&self, window: &Window, cx: &App) -> Pixels;
    fn set_size(&self, size: Option<Pixels>, window: &mut Window, cx: &mut App);
    fn icon(&self, window: &Window, cx: &App) -> Option<ui::IconName>;
    fn icon_tooltip(&self, window: &Window, cx: &App) -> Option<&'static str>;
    fn toggle_action(&self, window: &Window, cx: &App) -> Box<dyn Action>;
    fn icon_label(&self, window: &Window, cx: &App) -> Option<String>;
    fn panel_focus_handle(&self, cx: &App) -> FocusHandle;
    fn to_any(&self) -> AnyView;
    fn activation_priority(&self, cx: &App) -> u32;
    fn enabled(&self, cx: &App) -> bool;
    fn move_to_next_position(&self, window: &mut Window, cx: &mut App) {
        let current_position = self.position(window, cx);
        let next_position = [
            DockPosition::Left,
            DockPosition::Bottom,
            DockPosition::Right,
        ]
        .into_iter()
        .filter(|position| self.position_is_valid(*position, cx))
        .skip_while(|valid_position| *valid_position != current_position)
        .nth(1)
        .unwrap_or(DockPosition::Left);

        self.set_position(next_position, window, cx);
    }
}

impl<T> PanelHandle for Entity<T>
where
    T: Panel,
{
    fn panel_id(&self) -> EntityId {
        Entity::entity_id(self)
    }

    fn persistent_name(&self) -> &'static str {
        T::persistent_name()
    }

    fn position(&self, window: &Window, cx: &App) -> DockPosition {
        self.read(cx).position(window, cx)
    }

    fn position_is_valid(&self, position: DockPosition, cx: &App) -> bool {
        self.read(cx).position_is_valid(position)
    }

    fn set_position(&self, position: DockPosition, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| this.set_position(position, window, cx))
    }

    fn is_zoomed(&self, window: &Window, cx: &App) -> bool {
        self.read(cx).is_zoomed(window, cx)
    }

    fn set_zoomed(&self, zoomed: bool, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| this.set_zoomed(zoomed, window, cx))
    }

    fn set_active(&self, active: bool, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| this.set_active(active, window, cx))
    }

    fn pane(&self, cx: &App) -> Option<Entity<Pane>> {
        self.read(cx).pane()
    }

    fn remote_id(&self) -> Option<PanelId> {
        T::remote_id()
    }

    fn size(&self, window: &Window, cx: &App) -> Pixels {
        self.read(cx).size(window, cx)
    }

    fn set_size(&self, size: Option<Pixels>, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| this.set_size(size, window, cx))
    }

    fn icon(&self, window: &Window, cx: &App) -> Option<ui::IconName> {
        self.read(cx).icon(window, cx)
    }

    fn icon_tooltip(&self, window: &Window, cx: &App) -> Option<&'static str> {
        self.read(cx).icon_tooltip(window, cx)
    }

    fn toggle_action(&self, _: &Window, cx: &App) -> Box<dyn Action> {
        self.read(cx).toggle_action()
    }

    fn icon_label(&self, window: &Window, cx: &App) -> Option<String> {
        self.read(cx).icon_label(window, cx)
    }

    fn to_any(&self) -> AnyView {
        self.clone().into()
    }

    fn panel_focus_handle(&self, cx: &App) -> FocusHandle {
        self.read(cx).focus_handle(cx).clone()
    }

    fn activation_priority(&self, cx: &App) -> u32 {
        self.read(cx).activation_priority()
    }

    fn enabled(&self, cx: &App) -> bool {
        self.read(cx).enabled(cx)
    }
}

impl From<&dyn PanelHandle> for AnyView {
    fn from(val: &dyn PanelHandle) -> Self {
        val.to_any()
    }
}

/// A container with a fixed [`DockPosition`] adjacent to a certain widown edge.
/// Can contain multiple panels and show/hide itself with all contents.
pub struct Dock {
    position: DockPosition,
    panel_entries: Vec<PanelEntry>,
    workspace: WeakEntity<Workspace>,
    is_open: bool,
    active_panel_index: Option<usize>,
    focus_handle: FocusHandle,
    pub(crate) serialized_dock: Option<DockData>,
    zoom_layer_open: bool,
    modal_layer: Entity<ModalLayer>,
    _subscriptions: [Subscription; 2],
}

impl Focusable for Dock {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DockPosition {
    Left,
    Bottom,
    Right,
}

impl DockPosition {
    fn label(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Bottom => "bottom",
            Self::Right => "right",
        }
    }

    pub fn axis(&self) -> Axis {
        match self {
            Self::Left | Self::Right => Axis::Horizontal,
            Self::Bottom => Axis::Vertical,
        }
    }
}

struct PanelEntry {
    panel: Arc<dyn PanelHandle>,
    _subscriptions: [Subscription; 3],
}

pub struct PanelButtons {
    dock: Entity<Dock>,
}

impl Dock {
    pub fn new(
        position: DockPosition,
        modal_layer: Entity<ModalLayer>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let focus_handle = cx.focus_handle();
        let workspace = cx.entity().clone();
        let dock = cx.new(|cx| {
            let focus_subscription =
                cx.on_focus(&focus_handle, window, |dock: &mut Dock, window, cx| {
                    if let Some(active_entry) = dock.active_panel_entry() {
                        active_entry.panel.panel_focus_handle(cx).focus(window)
                    }
                });
            let zoom_subscription = cx.subscribe(&workspace, |dock, workspace, e: &Event, cx| {
                if matches!(e, Event::ZoomChanged) {
                    let is_zoomed = workspace.read(cx).zoomed.is_some();
                    dock.zoom_layer_open = is_zoomed;
                }
            });
            Self {
                position,
                workspace: workspace.downgrade(),
                panel_entries: Default::default(),
                active_panel_index: None,
                is_open: false,
                focus_handle: focus_handle.clone(),
                _subscriptions: [focus_subscription, zoom_subscription],
                serialized_dock: None,
                zoom_layer_open: false,
                modal_layer,
            }
        });

        cx.on_focus_in(&focus_handle, window, {
            let dock = dock.downgrade();
            move |workspace, window, cx| {
                let Some(dock) = dock.upgrade() else {
                    return;
                };
                let Some(panel) = dock.read(cx).active_panel() else {
                    return;
                };
                if panel.is_zoomed(window, cx) {
                    workspace.zoomed = Some(panel.to_any().downgrade());
                    workspace.zoomed_position = Some(position);
                } else {
                    workspace.zoomed = None;
                    workspace.zoomed_position = None;
                }
                cx.emit(Event::ZoomChanged);
                workspace.dismiss_zoomed_items_to_reveal(Some(position), window, cx);
                workspace.update_active_view_for_followers(window, cx)
            }
        })
        .detach();

        cx.observe_in(&dock, window, move |workspace, dock, window, cx| {
            if dock.read(cx).is_open() {
                if let Some(panel) = dock.read(cx).active_panel() {
                    if panel.is_zoomed(window, cx) {
                        workspace.zoomed = Some(panel.to_any().downgrade());
                        workspace.zoomed_position = Some(position);
                        cx.emit(Event::ZoomChanged);
                        return;
                    }
                }
            }
            if workspace.zoomed_position == Some(position) {
                workspace.zoomed = None;
                workspace.zoomed_position = None;
                cx.emit(Event::ZoomChanged);
            }
        })
        .detach();

        dock
    }

    pub fn position(&self) -> DockPosition {
        self.position
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    fn resizable(&self, cx: &App) -> bool {
        !(self.zoom_layer_open || self.modal_layer.read(cx).has_active_modal())
    }

    pub fn panel<T: Panel>(&self) -> Option<Entity<T>> {
        self.panel_entries
            .iter()
            .find_map(|entry| entry.panel.to_any().clone().downcast().ok())
    }

    pub fn panel_index_for_type<T: Panel>(&self) -> Option<usize> {
        self.panel_entries
            .iter()
            .position(|entry| entry.panel.to_any().downcast::<T>().is_ok())
    }

    pub fn panel_index_for_persistent_name(&self, ui_name: &str, _cx: &App) -> Option<usize> {
        self.panel_entries
            .iter()
            .position(|entry| entry.panel.persistent_name() == ui_name)
    }

    pub fn panel_index_for_proto_id(&self, panel_id: PanelId) -> Option<usize> {
        self.panel_entries
            .iter()
            .position(|entry| entry.panel.remote_id() == Some(panel_id))
    }

    pub fn first_enabled_panel_idx(&mut self, cx: &mut Context<Self>) -> anyhow::Result<usize> {
        self.panel_entries
            .iter()
            .position(|entry| entry.panel.enabled(cx))
            .with_context(|| {
                format!(
                    "Couldn't find any enabled panel for the {} dock.",
                    self.position.label()
                )
            })
    }

    fn active_panel_entry(&self) -> Option<&PanelEntry> {
        self.active_panel_index
            .and_then(|index| self.panel_entries.get(index))
    }

    pub fn active_panel_index(&self) -> Option<usize> {
        self.active_panel_index
    }

    pub fn set_open(&mut self, open: bool, window: &mut Window, cx: &mut Context<Self>) {
        if open != self.is_open {
            self.is_open = open;
            if let Some(active_panel) = self.active_panel_entry() {
                active_panel.panel.set_active(open, window, cx);
            }

            cx.notify();
        }
    }

    pub fn set_panel_zoomed(
        &mut self,
        panel: &AnyView,
        zoomed: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        for entry in &mut self.panel_entries {
            if entry.panel.panel_id() == panel.entity_id() {
                if zoomed != entry.panel.is_zoomed(window, cx) {
                    entry.panel.set_zoomed(zoomed, window, cx);
                }
            } else if entry.panel.is_zoomed(window, cx) {
                entry.panel.set_zoomed(false, window, cx);
            }
        }

        self.workspace
            .update(cx, |workspace, cx| {
                workspace.serialize_workspace(window, cx);
            })
            .ok();
        cx.notify();
    }

    pub fn zoom_out(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        for entry in &mut self.panel_entries {
            if entry.panel.is_zoomed(window, cx) {
                entry.panel.set_zoomed(false, window, cx);
            }
        }
    }

    pub(crate) fn add_panel<T: Panel>(
        &mut self,
        panel: Entity<T>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> usize {
        let subscriptions = [
            cx.observe(&panel, |_, _, cx| cx.notify()),
            cx.observe_global_in::<SettingsStore>(window, {
                let workspace = workspace.clone();
                let panel = panel.clone();

                move |this, window, cx| {
                    let new_position = panel.read(cx).position(window, cx);
                    if new_position == this.position {
                        return;
                    }

                    let Ok(new_dock) = workspace.update(cx, |workspace, cx| {
                        if panel.is_zoomed(window, cx) {
                            workspace.zoomed_position = Some(new_position);
                        }
                        match new_position {
                            DockPosition::Left => &workspace.left_dock,
                            DockPosition::Bottom => &workspace.bottom_dock,
                            DockPosition::Right => &workspace.right_dock,
                        }
                        .clone()
                    }) else {
                        return;
                    };

                    let was_visible = this.is_open()
                        && this.visible_panel().map_or(false, |active_panel| {
                            active_panel.panel_id() == Entity::entity_id(&panel)
                        });

                    this.remove_panel(&panel, window, cx);

                    new_dock.update(cx, |new_dock, cx| {
                        new_dock.remove_panel(&panel, window, cx);
                        let index =
                            new_dock.add_panel(panel.clone(), workspace.clone(), window, cx);
                        if was_visible {
                            new_dock.set_open(true, window, cx);
                            new_dock.activate_panel(index, window, cx);
                        }
                    });
                }
            }),
            cx.subscribe_in(
                &panel,
                window,
                move |this, panel, event, window, cx| match event {
                    PanelEvent::ZoomIn => {
                        this.set_panel_zoomed(&panel.to_any(), true, window, cx);
                        if !PanelHandle::panel_focus_handle(panel, cx).contains_focused(window, cx)
                        {
                            window.focus(&panel.focus_handle(cx));
                        }
                        workspace
                            .update(cx, |workspace, cx| {
                                workspace.zoomed = Some(panel.downgrade().into());
                                workspace.zoomed_position =
                                    Some(panel.read(cx).position(window, cx));
                                cx.emit(Event::ZoomChanged);
                            })
                            .ok();
                    }
                    PanelEvent::ZoomOut => {
                        this.set_panel_zoomed(&panel.to_any(), false, window, cx);
                        workspace
                            .update(cx, |workspace, cx| {
                                if workspace.zoomed_position == Some(this.position) {
                                    workspace.zoomed = None;
                                    workspace.zoomed_position = None;
                                    cx.emit(Event::ZoomChanged);
                                }
                                cx.notify();
                            })
                            .ok();
                    }
                    PanelEvent::Activate => {
                        if let Some(ix) = this
                            .panel_entries
                            .iter()
                            .position(|entry| entry.panel.panel_id() == Entity::entity_id(panel))
                        {
                            this.set_open(true, window, cx);
                            this.activate_panel(ix, window, cx);
                            window.focus(&panel.read(cx).focus_handle(cx));
                        }
                    }
                    PanelEvent::Close => {
                        if this
                            .visible_panel()
                            .map_or(false, |p| p.panel_id() == Entity::entity_id(panel))
                        {
                            this.set_open(false, window, cx);
                        }
                    }
                },
            ),
        ];

        let index = match self
            .panel_entries
            .binary_search_by_key(&panel.read(cx).activation_priority(), |entry| {
                entry.panel.activation_priority(cx)
            }) {
            Ok(ix) => ix,
            Err(ix) => ix,
        };
        if let Some(active_index) = self.active_panel_index.as_mut() {
            if *active_index >= index {
                *active_index += 1;
            }
        }
        self.panel_entries.insert(
            index,
            PanelEntry {
                panel: Arc::new(panel.clone()),
                _subscriptions: subscriptions,
            },
        );

        self.restore_state(window, cx);
        if panel.read(cx).starts_open(window, cx) {
            self.activate_panel(index, window, cx);
            self.set_open(true, window, cx);
        }

        cx.notify();
        index
    }

    pub fn restore_state(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if let Some(serialized) = self.serialized_dock.clone() {
            if let Some(active_panel) = serialized.active_panel.filter(|_| serialized.visible) {
                if let Some(idx) = self.panel_index_for_persistent_name(active_panel.as_str(), cx) {
                    self.activate_panel(idx, window, cx);
                }
            }

            if serialized.zoom {
                if let Some(panel) = self.active_panel() {
                    panel.set_zoomed(true, window, cx)
                }
            }
            self.set_open(serialized.visible, window, cx);
            return true;
        }
        false
    }

    pub fn remove_panel<T: Panel>(
        &mut self,
        panel: &Entity<T>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(panel_ix) = self
            .panel_entries
            .iter()
            .position(|entry| entry.panel.panel_id() == Entity::entity_id(panel))
        {
            if let Some(active_panel_index) = self.active_panel_index.as_mut() {
                match panel_ix.cmp(active_panel_index) {
                    std::cmp::Ordering::Less => {
                        *active_panel_index -= 1;
                    }
                    std::cmp::Ordering::Equal => {
                        self.active_panel_index = None;
                        self.set_open(false, window, cx);
                    }
                    std::cmp::Ordering::Greater => {}
                }
            }
            self.panel_entries.remove(panel_ix);
            cx.notify();
        }
    }

    pub fn panels_len(&self) -> usize {
        self.panel_entries.len()
    }

    pub fn activate_panel(&mut self, panel_ix: usize, window: &mut Window, cx: &mut Context<Self>) {
        if Some(panel_ix) != self.active_panel_index {
            if let Some(active_panel) = self.active_panel_entry() {
                active_panel.panel.set_active(false, window, cx);
            }

            self.active_panel_index = Some(panel_ix);
            if let Some(active_panel) = self.active_panel_entry() {
                active_panel.panel.set_active(true, window, cx);
            }

            cx.notify();
        }
    }

    pub fn visible_panel(&self) -> Option<&Arc<dyn PanelHandle>> {
        let entry = self.visible_entry()?;
        Some(&entry.panel)
    }

    pub fn active_panel(&self) -> Option<&Arc<dyn PanelHandle>> {
        let panel_entry = self.active_panel_entry()?;
        Some(&panel_entry.panel)
    }

    fn visible_entry(&self) -> Option<&PanelEntry> {
        if self.is_open {
            self.active_panel_entry()
        } else {
            None
        }
    }

    pub fn zoomed_panel(&self, window: &Window, cx: &App) -> Option<Arc<dyn PanelHandle>> {
        let entry = self.visible_entry()?;
        if entry.panel.is_zoomed(window, cx) {
            Some(entry.panel.clone())
        } else {
            None
        }
    }

    pub fn panel_size(&self, panel: &dyn PanelHandle, window: &Window, cx: &App) -> Option<Pixels> {
        self.panel_entries
            .iter()
            .find(|entry| entry.panel.panel_id() == panel.panel_id())
            .map(|entry| entry.panel.size(window, cx))
    }

    pub fn active_panel_size(&self, window: &Window, cx: &App) -> Option<Pixels> {
        if self.is_open {
            self.active_panel_entry()
                .map(|entry| entry.panel.size(window, cx))
        } else {
            None
        }
    }

    pub fn resize_active_panel(
        &mut self,
        size: Option<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(entry) = self.active_panel_entry() {
            let size = size.map(|size| size.max(RESIZE_HANDLE_SIZE).round());

            entry.panel.set_size(size, window, cx);
            cx.notify();
        }
    }

    pub fn resize_all_panels(
        &mut self,
        size: Option<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        for entry in &mut self.panel_entries {
            let size = size.map(|size| size.max(RESIZE_HANDLE_SIZE).round());
            entry.panel.set_size(size, window, cx);
        }
        cx.notify();
    }

    pub fn toggle_action(&self) -> Box<dyn Action> {
        match self.position {
            DockPosition::Left => crate::ToggleLeftDock.boxed_clone(),
            DockPosition::Bottom => crate::ToggleBottomDock.boxed_clone(),
            DockPosition::Right => crate::ToggleRightDock.boxed_clone(),
        }
    }

    fn dispatch_context() -> KeyContext {
        let mut dispatch_context = KeyContext::new_with_defaults();
        dispatch_context.add("Dock");

        dispatch_context
    }

    pub fn clamp_panel_size(&mut self, max_size: Pixels, window: &mut Window, cx: &mut App) {
        let max_size = px((max_size.0 - RESIZE_HANDLE_SIZE.0).abs());
        for panel in self.panel_entries.iter().map(|entry| &entry.panel) {
            if panel.size(window, cx) > max_size {
                panel.set_size(Some(max_size.max(RESIZE_HANDLE_SIZE)), window, cx);
            }
        }
    }
}

impl Render for Dock {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dispatch_context = Self::dispatch_context();
        if let Some(entry) = self.visible_entry() {
            let size = entry.panel.size(window, cx);

            let position = self.position;
            let create_resize_handle = || {
                let handle = div()
                    .id("resize-handle")
                    .on_drag(DraggedDock(position), |dock, _, _, cx| {
                        cx.stop_propagation();
                        cx.new(|_| dock.clone())
                    })
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|_, _: &MouseDownEvent, _, cx| {
                            cx.stop_propagation();
                        }),
                    )
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|dock, e: &MouseUpEvent, window, cx| {
                            if e.click_count == 2 {
                                dock.resize_active_panel(None, window, cx);
                                dock.workspace
                                    .update(cx, |workspace, cx| {
                                        workspace.serialize_workspace(window, cx);
                                    })
                                    .ok();
                                cx.stop_propagation();
                            }
                        }),
                    )
                    .occlude();
                match self.position() {
                    DockPosition::Left => deferred(
                        handle
                            .absolute()
                            .right(-RESIZE_HANDLE_SIZE / 2.)
                            .top(px(0.))
                            .h_full()
                            .w(RESIZE_HANDLE_SIZE)
                            .cursor_col_resize(),
                    ),
                    DockPosition::Bottom => deferred(
                        handle
                            .absolute()
                            .top(-RESIZE_HANDLE_SIZE / 2.)
                            .left(px(0.))
                            .w_full()
                            .h(RESIZE_HANDLE_SIZE)
                            .cursor_row_resize(),
                    ),
                    DockPosition::Right => deferred(
                        handle
                            .absolute()
                            .top(px(0.))
                            .left(-RESIZE_HANDLE_SIZE / 2.)
                            .h_full()
                            .w(RESIZE_HANDLE_SIZE)
                            .cursor_col_resize(),
                    ),
                }
            };

            div()
                .key_context(dispatch_context)
                .track_focus(&self.focus_handle(cx))
                .flex()
                .bg(cx.theme().colors().panel_background)
                .border_color(cx.theme().colors().border)
                .overflow_hidden()
                .map(|this| match self.position().axis() {
                    Axis::Horizontal => this.w(size).h_full().flex_row(),
                    Axis::Vertical => this.h(size).w_full().flex_col(),
                })
                .map(|this| match self.position() {
                    DockPosition::Left => this.border_r_1(),
                    DockPosition::Right => this.border_l_1(),
                    DockPosition::Bottom => this.border_t_1(),
                })
                .child(
                    div()
                        .map(|this| match self.position().axis() {
                            Axis::Horizontal => this.min_w(size).h_full(),
                            Axis::Vertical => this.min_h(size).w_full(),
                        })
                        .child(
                            entry
                                .panel
                                .to_any()
                                .cached(StyleRefinement::default().v_flex().size_full()),
                        ),
                )
                .when(self.resizable(cx), |this| {
                    this.child(create_resize_handle())
                })
        } else {
            div()
                .key_context(dispatch_context)
                .track_focus(&self.focus_handle(cx))
        }
    }
}

impl PanelButtons {
    pub fn new(dock: Entity<Dock>, cx: &mut Context<Self>) -> Self {
        cx.observe(&dock, |_, _, cx| cx.notify()).detach();
        Self { dock }
    }
}

impl Render for PanelButtons {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dock = self.dock.read(cx);
        let active_index = dock.active_panel_index;
        let is_open = dock.is_open;
        let dock_position = dock.position;

        let (menu_anchor, menu_attach) = match dock.position {
            DockPosition::Left => (Corner::BottomLeft, Corner::TopLeft),
            DockPosition::Bottom | DockPosition::Right => (Corner::BottomRight, Corner::TopRight),
        };

        let buttons: Vec<_> = dock
            .panel_entries
            .iter()
            .enumerate()
            .filter_map(|(i, entry)| {
                let icon = entry.panel.icon(window, cx)?;
                let icon_tooltip = entry.panel.icon_tooltip(window, cx)?;
                let name = entry.panel.persistent_name();
                let panel = entry.panel.clone();

                let is_active_button = Some(i) == active_index && is_open;
                let (action, tooltip) = if is_active_button {
                    let action = dock.toggle_action();

                    let tooltip: SharedString =
                        format!("Close {} dock", dock.position.label()).into();

                    (action, tooltip)
                } else {
                    let action = entry.panel.toggle_action(window, cx);

                    (action, icon_tooltip.into())
                };

                Some(
                    right_click_menu(name)
                        .menu(move |window, cx| {
                            const POSITIONS: [DockPosition; 3] = [
                                DockPosition::Left,
                                DockPosition::Right,
                                DockPosition::Bottom,
                            ];

                            ContextMenu::build(window, cx, |mut menu, _, cx| {
                                for position in POSITIONS {
                                    if position != dock_position
                                        && panel.position_is_valid(position, cx)
                                    {
                                        let panel = panel.clone();
                                        menu = menu.entry(
                                            format!("Dock {}", position.label()),
                                            None,
                                            move |window, cx| {
                                                panel.set_position(position, window, cx);
                                            },
                                        )
                                    }
                                }
                                menu
                            })
                        })
                        .anchor(menu_anchor)
                        .attach(menu_attach)
                        .trigger(move |is_active| {
                            IconButton::new(name, icon)
                                .icon_size(IconSize::Small)
                                .toggle_state(is_active_button)
                                .on_click({
                                    let action = action.boxed_clone();
                                    move |_, window, cx| {
                                        window.dispatch_action(action.boxed_clone(), cx)
                                    }
                                })
                                .when(!is_active, |this| {
                                    this.tooltip(move |window, cx| {
                                        Tooltip::for_action(tooltip.clone(), &*action, window, cx)
                                    })
                                })
                        }),
                )
            })
            .collect();

        let has_buttons = !buttons.is_empty();
        h_flex()
            .gap_1()
            .children(buttons)
            .when(has_buttons && dock.position == DockPosition::Left, |this| {
                this.child(Divider::vertical().color(DividerColor::Border))
            })
    }
}

impl StatusItemView for PanelButtons {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn crate::ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        // Nothing to do, panel buttons don't depend on the active center item
    }
}

#[cfg(any(test, feature = "test-support"))]
pub mod test {
    use super::*;
    use gpui::{App, Context, Window, actions, div};

    pub struct TestPanel {
        pub position: DockPosition,
        pub zoomed: bool,
        pub active: bool,
        pub focus_handle: FocusHandle,
        pub size: Pixels,
    }
    actions!(test, [ToggleTestPanel]);

    impl EventEmitter<PanelEvent> for TestPanel {}

    impl TestPanel {
        pub fn new(position: DockPosition, cx: &mut App) -> Self {
            Self {
                position,
                zoomed: false,
                active: false,
                focus_handle: cx.focus_handle(),
                size: px(300.),
            }
        }
    }

    impl Render for TestPanel {
        fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div().id("test").track_focus(&self.focus_handle(cx))
        }
    }

    impl Panel for TestPanel {
        fn persistent_name() -> &'static str {
            "TestPanel"
        }

        fn position(&self, _window: &Window, _: &App) -> super::DockPosition {
            self.position
        }

        fn position_is_valid(&self, _: super::DockPosition) -> bool {
            true
        }

        fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
            self.position = position;
            cx.update_global::<SettingsStore, _>(|_, _| {});
        }

        fn size(&self, _window: &Window, _: &App) -> Pixels {
            self.size
        }

        fn set_size(&mut self, size: Option<Pixels>, _window: &mut Window, _: &mut Context<Self>) {
            self.size = size.unwrap_or(px(300.));
        }

        fn icon(&self, _window: &Window, _: &App) -> Option<ui::IconName> {
            None
        }

        fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
            None
        }

        fn toggle_action(&self) -> Box<dyn Action> {
            ToggleTestPanel.boxed_clone()
        }

        fn is_zoomed(&self, _window: &Window, _: &App) -> bool {
            self.zoomed
        }

        fn set_zoomed(&mut self, zoomed: bool, _window: &mut Window, _cx: &mut Context<Self>) {
            self.zoomed = zoomed;
        }

        fn set_active(&mut self, active: bool, _window: &mut Window, _cx: &mut Context<Self>) {
            self.active = active;
        }

        fn activation_priority(&self) -> u32 {
            100
        }
    }

    impl Focusable for TestPanel {
        fn focus_handle(&self, _cx: &App) -> FocusHandle {
            self.focus_handle.clone()
        }
    }
}
