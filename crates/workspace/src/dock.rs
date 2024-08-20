use crate::persistence::model::DockData;
use crate::{status_bar::StatusItemView, Workspace};
use crate::{DraggedDock, Event, Pane};
use client::proto;
use gpui::{
    deferred, div, px, Action, AnchorCorner, AnyView, AppContext, Axis, Entity, EntityId,
    EventEmitter, FocusHandle, FocusableView, IntoElement, KeyContext, MouseButton, MouseDownEvent,
    MouseUpEvent, ParentElement, Render, SharedString, StyleRefinement, Styled, Subscription, View,
    ViewContext, VisualContext, WeakView, WindowContext,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::SettingsStore;
use std::sync::Arc;
use ui::{h_flex, ContextMenu, IconButton, Tooltip};
use ui::{prelude::*, right_click_menu};

const RESIZE_HANDLE_SIZE: Pixels = Pixels(6.);

pub enum PanelEvent {
    ZoomIn,
    ZoomOut,
    Activate,
    Close,
}

pub use proto::PanelId;

pub trait Panel: FocusableView + EventEmitter<PanelEvent> {
    fn persistent_name() -> &'static str;
    fn position(&self, cx: &WindowContext) -> DockPosition;
    fn position_is_valid(&self, position: DockPosition) -> bool;
    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>);
    fn size(&self, cx: &WindowContext) -> Pixels;
    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>);
    fn icon(&self, cx: &WindowContext) -> Option<ui::IconName>;
    fn icon_tooltip(&self, cx: &WindowContext) -> Option<&'static str>;
    fn toggle_action(&self) -> Box<dyn Action>;
    fn icon_label(&self, _: &WindowContext) -> Option<String> {
        None
    }
    fn is_zoomed(&self, _cx: &WindowContext) -> bool {
        false
    }
    fn starts_open(&self, _cx: &WindowContext) -> bool {
        false
    }
    fn set_zoomed(&mut self, _zoomed: bool, _cx: &mut ViewContext<Self>) {}
    fn set_active(&mut self, _active: bool, _cx: &mut ViewContext<Self>) {}
    fn pane(&self) -> Option<View<Pane>> {
        None
    }
    fn remote_id() -> Option<proto::PanelId> {
        None
    }
}

pub trait PanelHandle: Send + Sync {
    fn panel_id(&self) -> EntityId;
    fn persistent_name(&self) -> &'static str;
    fn position(&self, cx: &WindowContext) -> DockPosition;
    fn position_is_valid(&self, position: DockPosition, cx: &WindowContext) -> bool;
    fn set_position(&self, position: DockPosition, cx: &mut WindowContext);
    fn is_zoomed(&self, cx: &WindowContext) -> bool;
    fn set_zoomed(&self, zoomed: bool, cx: &mut WindowContext);
    fn set_active(&self, active: bool, cx: &mut WindowContext);
    fn remote_id(&self) -> Option<proto::PanelId>;
    fn pane(&self, cx: &WindowContext) -> Option<View<Pane>>;
    fn size(&self, cx: &WindowContext) -> Pixels;
    fn set_size(&self, size: Option<Pixels>, cx: &mut WindowContext);
    fn icon(&self, cx: &WindowContext) -> Option<ui::IconName>;
    fn icon_tooltip(&self, cx: &WindowContext) -> Option<&'static str>;
    fn toggle_action(&self, cx: &WindowContext) -> Box<dyn Action>;
    fn icon_label(&self, cx: &WindowContext) -> Option<String>;
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle;
    fn to_any(&self) -> AnyView;
}

impl<T> PanelHandle for View<T>
where
    T: Panel,
{
    fn panel_id(&self) -> EntityId {
        Entity::entity_id(self)
    }

    fn persistent_name(&self) -> &'static str {
        T::persistent_name()
    }

    fn position(&self, cx: &WindowContext) -> DockPosition {
        self.read(cx).position(cx)
    }

    fn position_is_valid(&self, position: DockPosition, cx: &WindowContext) -> bool {
        self.read(cx).position_is_valid(position)
    }

    fn set_position(&self, position: DockPosition, cx: &mut WindowContext) {
        self.update(cx, |this, cx| this.set_position(position, cx))
    }

    fn is_zoomed(&self, cx: &WindowContext) -> bool {
        self.read(cx).is_zoomed(cx)
    }

    fn set_zoomed(&self, zoomed: bool, cx: &mut WindowContext) {
        self.update(cx, |this, cx| this.set_zoomed(zoomed, cx))
    }

    fn set_active(&self, active: bool, cx: &mut WindowContext) {
        self.update(cx, |this, cx| this.set_active(active, cx))
    }

    fn pane(&self, cx: &WindowContext) -> Option<View<Pane>> {
        self.read(cx).pane()
    }

    fn remote_id(&self) -> Option<PanelId> {
        T::remote_id()
    }

    fn size(&self, cx: &WindowContext) -> Pixels {
        self.read(cx).size(cx)
    }

    fn set_size(&self, size: Option<Pixels>, cx: &mut WindowContext) {
        self.update(cx, |this, cx| this.set_size(size, cx))
    }

    fn icon(&self, cx: &WindowContext) -> Option<ui::IconName> {
        self.read(cx).icon(cx)
    }

    fn icon_tooltip(&self, cx: &WindowContext) -> Option<&'static str> {
        self.read(cx).icon_tooltip(cx)
    }

    fn toggle_action(&self, cx: &WindowContext) -> Box<dyn Action> {
        self.read(cx).toggle_action()
    }

    fn icon_label(&self, cx: &WindowContext) -> Option<String> {
        self.read(cx).icon_label(cx)
    }

    fn to_any(&self) -> AnyView {
        self.clone().into()
    }

    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.read(cx).focus_handle(cx).clone()
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
    is_open: bool,
    active_panel_index: usize,
    focus_handle: FocusHandle,
    pub(crate) serialized_dock: Option<DockData>,
    resizeable: bool,
    _subscriptions: [Subscription; 2],
}

impl FocusableView for Dock {
    fn focus_handle(&self, _: &AppContext) -> FocusHandle {
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
    fn to_label(&self) -> &'static str {
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
    dock: View<Dock>,
}

impl Dock {
    pub fn new(position: DockPosition, cx: &mut ViewContext<Workspace>) -> View<Self> {
        let focus_handle = cx.focus_handle();
        let workspace = cx.view().clone();
        let dock = cx.new_view(|cx: &mut ViewContext<Self>| {
            let focus_subscription = cx.on_focus(&focus_handle, |dock, cx| {
                if let Some(active_entry) = dock.panel_entries.get(dock.active_panel_index) {
                    active_entry.panel.focus_handle(cx).focus(cx)
                }
            });
            let zoom_subscription = cx.subscribe(&workspace, |dock, workspace, e: &Event, cx| {
                if matches!(e, Event::ZoomChanged) {
                    let is_zoomed = workspace.read(cx).zoomed.is_some();
                    dock.resizeable = !is_zoomed;
                }
            });
            Self {
                position,
                panel_entries: Default::default(),
                active_panel_index: 0,
                is_open: false,
                focus_handle: focus_handle.clone(),
                _subscriptions: [focus_subscription, zoom_subscription],
                serialized_dock: None,
                resizeable: true,
            }
        });

        cx.on_focus_in(&focus_handle, {
            let dock = dock.downgrade();
            move |workspace, cx| {
                let Some(dock) = dock.upgrade() else {
                    return;
                };
                let Some(panel) = dock.read(cx).active_panel() else {
                    return;
                };
                if panel.is_zoomed(cx) {
                    workspace.zoomed = Some(panel.to_any().downgrade());
                    workspace.zoomed_position = Some(position);
                } else {
                    workspace.zoomed = None;
                    workspace.zoomed_position = None;
                }
                cx.emit(Event::ZoomChanged);
                workspace.dismiss_zoomed_items_to_reveal(Some(position), cx);
                workspace.update_active_view_for_followers(cx)
            }
        })
        .detach();

        cx.observe(&dock, move |workspace, dock, cx| {
            if dock.read(cx).is_open() {
                if let Some(panel) = dock.read(cx).active_panel() {
                    if panel.is_zoomed(cx) {
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

    pub fn panel<T: Panel>(&self) -> Option<View<T>> {
        self.panel_entries
            .iter()
            .find_map(|entry| entry.panel.to_any().clone().downcast().ok())
    }

    pub fn panel_index_for_type<T: Panel>(&self) -> Option<usize> {
        self.panel_entries
            .iter()
            .position(|entry| entry.panel.to_any().downcast::<T>().is_ok())
    }

    pub fn panel_index_for_persistent_name(
        &self,
        ui_name: &str,
        _cx: &AppContext,
    ) -> Option<usize> {
        self.panel_entries
            .iter()
            .position(|entry| entry.panel.persistent_name() == ui_name)
    }

    pub fn panel_index_for_proto_id(&self, panel_id: PanelId) -> Option<usize> {
        self.panel_entries
            .iter()
            .position(|entry| entry.panel.remote_id() == Some(panel_id))
    }

    pub fn active_panel_index(&self) -> usize {
        self.active_panel_index
    }

    pub(crate) fn set_open(&mut self, open: bool, cx: &mut ViewContext<Self>) {
        if open != self.is_open {
            self.is_open = open;
            if let Some(active_panel) = self.panel_entries.get(self.active_panel_index) {
                active_panel.panel.set_active(open, cx);
            }

            cx.notify();
        }
    }

    pub fn set_panel_zoomed(&mut self, panel: &AnyView, zoomed: bool, cx: &mut ViewContext<Self>) {
        for entry in &mut self.panel_entries {
            if entry.panel.panel_id() == panel.entity_id() {
                if zoomed != entry.panel.is_zoomed(cx) {
                    entry.panel.set_zoomed(zoomed, cx);
                }
            } else if entry.panel.is_zoomed(cx) {
                entry.panel.set_zoomed(false, cx);
            }
        }

        cx.notify();
    }

    pub fn zoom_out(&mut self, cx: &mut ViewContext<Self>) {
        for entry in &mut self.panel_entries {
            if entry.panel.is_zoomed(cx) {
                entry.panel.set_zoomed(false, cx);
            }
        }
    }

    pub(crate) fn add_panel<T: Panel>(
        &mut self,
        panel: View<T>,
        workspace: WeakView<Workspace>,
        cx: &mut ViewContext<Self>,
    ) {
        let subscriptions = [
            cx.observe(&panel, |_, _, cx| cx.notify()),
            cx.observe_global::<SettingsStore>({
                let workspace = workspace.clone();
                let panel = panel.clone();

                move |this, cx| {
                    let new_position = panel.read(cx).position(cx);
                    if new_position == this.position {
                        return;
                    }

                    let Ok(new_dock) = workspace.update(cx, |workspace, cx| {
                        if panel.is_zoomed(cx) {
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

                    this.remove_panel(&panel, cx);

                    new_dock.update(cx, |new_dock, cx| {
                        new_dock.remove_panel(&panel, cx);
                        new_dock.add_panel(panel.clone(), workspace.clone(), cx);
                        if was_visible {
                            new_dock.set_open(true, cx);
                            new_dock.activate_panel(new_dock.panels_len() - 1, cx);
                        }
                    });
                }
            }),
            cx.subscribe(&panel, move |this, panel, event, cx| match event {
                PanelEvent::ZoomIn => {
                    this.set_panel_zoomed(&panel.to_any(), true, cx);
                    if !panel.focus_handle(cx).contains_focused(cx) {
                        cx.focus_view(&panel);
                    }
                    workspace
                        .update(cx, |workspace, cx| {
                            workspace.zoomed = Some(panel.downgrade().into());
                            workspace.zoomed_position = Some(panel.read(cx).position(cx));
                            cx.emit(Event::ZoomChanged);
                        })
                        .ok();
                }
                PanelEvent::ZoomOut => {
                    this.set_panel_zoomed(&panel.to_any(), false, cx);
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
                        .position(|entry| entry.panel.panel_id() == Entity::entity_id(&panel))
                    {
                        this.set_open(true, cx);
                        this.activate_panel(ix, cx);
                        cx.focus_view(&panel);
                    }
                }
                PanelEvent::Close => {
                    if this
                        .visible_panel()
                        .map_or(false, |p| p.panel_id() == Entity::entity_id(&panel))
                    {
                        this.set_open(false, cx);
                    }
                }
            }),
        ];

        self.panel_entries.push(PanelEntry {
            panel: Arc::new(panel.clone()),
            _subscriptions: subscriptions,
        });

        if !self.restore_state(cx) && panel.read(cx).starts_open(cx) {
            self.activate_panel(self.panel_entries.len() - 1, cx);
            self.set_open(true, cx);
        }

        cx.notify()
    }

    pub fn restore_state(&mut self, cx: &mut ViewContext<Self>) -> bool {
        if let Some(serialized) = self.serialized_dock.clone() {
            if let Some(active_panel) = serialized.active_panel {
                if let Some(idx) = self.panel_index_for_persistent_name(active_panel.as_str(), cx) {
                    self.activate_panel(idx, cx);
                }
            }

            if serialized.zoom {
                if let Some(panel) = self.active_panel() {
                    panel.set_zoomed(true, cx)
                }
            }
            self.set_open(serialized.visible, cx);
            return true;
        }
        return false;
    }

    pub fn remove_panel<T: Panel>(&mut self, panel: &View<T>, cx: &mut ViewContext<Self>) {
        if let Some(panel_ix) = self
            .panel_entries
            .iter()
            .position(|entry| entry.panel.panel_id() == Entity::entity_id(panel))
        {
            if panel_ix == self.active_panel_index {
                self.active_panel_index = 0;
                self.set_open(false, cx);
            } else if panel_ix < self.active_panel_index {
                self.active_panel_index -= 1;
            }
            self.panel_entries.remove(panel_ix);
            cx.notify();
        }
    }

    pub fn panels_len(&self) -> usize {
        self.panel_entries.len()
    }

    pub fn activate_panel(&mut self, panel_ix: usize, cx: &mut ViewContext<Self>) {
        if panel_ix != self.active_panel_index {
            if let Some(active_panel) = self.panel_entries.get(self.active_panel_index) {
                active_panel.panel.set_active(false, cx);
            }

            self.active_panel_index = panel_ix;
            if let Some(active_panel) = self.panel_entries.get(self.active_panel_index) {
                active_panel.panel.set_active(true, cx);
            }

            cx.notify();
        }
    }

    pub fn visible_panel(&self) -> Option<&Arc<dyn PanelHandle>> {
        let entry = self.visible_entry()?;
        Some(&entry.panel)
    }

    pub fn active_panel(&self) -> Option<&Arc<dyn PanelHandle>> {
        Some(&self.panel_entries.get(self.active_panel_index)?.panel)
    }

    fn visible_entry(&self) -> Option<&PanelEntry> {
        if self.is_open {
            self.panel_entries.get(self.active_panel_index)
        } else {
            None
        }
    }

    pub fn zoomed_panel(&self, cx: &WindowContext) -> Option<Arc<dyn PanelHandle>> {
        let entry = self.visible_entry()?;
        if entry.panel.is_zoomed(cx) {
            Some(entry.panel.clone())
        } else {
            None
        }
    }

    pub fn panel_size(&self, panel: &dyn PanelHandle, cx: &WindowContext) -> Option<Pixels> {
        self.panel_entries
            .iter()
            .find(|entry| entry.panel.panel_id() == panel.panel_id())
            .map(|entry| entry.panel.size(cx))
    }

    pub fn active_panel_size(&self, cx: &WindowContext) -> Option<Pixels> {
        if self.is_open {
            self.panel_entries
                .get(self.active_panel_index)
                .map(|entry| entry.panel.size(cx))
        } else {
            None
        }
    }

    pub fn resize_active_panel(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>) {
        if let Some(entry) = self.panel_entries.get_mut(self.active_panel_index) {
            let size = size.map(|size| size.max(RESIZE_HANDLE_SIZE).round());
            entry.panel.set_size(size, cx);
            cx.notify();
        }
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
}

impl Render for Dock {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let dispatch_context = Self::dispatch_context();
        if let Some(entry) = self.visible_entry() {
            let size = entry.panel.size(cx);

            let position = self.position;
            let create_resize_handle = || {
                let handle = div()
                    .id("resize-handle")
                    .on_drag(DraggedDock(position), |dock, cx| {
                        cx.stop_propagation();
                        cx.new_view(|_| dock.clone())
                    })
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|_, _: &MouseDownEvent, cx| {
                            cx.stop_propagation();
                        }),
                    )
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|v, e: &MouseUpEvent, cx| {
                            if e.click_count == 2 {
                                v.resize_active_panel(None, cx);
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
                .track_focus(&self.focus_handle)
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
                .when(self.resizeable, |this| this.child(create_resize_handle()))
        } else {
            div()
                .key_context(dispatch_context)
                .track_focus(&self.focus_handle)
        }
    }
}

impl PanelButtons {
    pub fn new(dock: View<Dock>, cx: &mut ViewContext<Self>) -> Self {
        cx.observe(&dock, |_, _, cx| cx.notify()).detach();
        Self { dock }
    }
}

impl Render for PanelButtons {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let dock = self.dock.read(cx);
        let active_index = dock.active_panel_index;
        let is_open = dock.is_open;
        let dock_position = dock.position;

        let (menu_anchor, menu_attach) = match dock.position {
            DockPosition::Left => (AnchorCorner::BottomLeft, AnchorCorner::TopLeft),
            DockPosition::Bottom | DockPosition::Right => {
                (AnchorCorner::BottomRight, AnchorCorner::TopRight)
            }
        };

        let buttons = dock
            .panel_entries
            .iter()
            .enumerate()
            .filter_map(|(i, entry)| {
                let icon = entry.panel.icon(cx)?;
                let icon_tooltip = entry.panel.icon_tooltip(cx)?;
                let name = entry.panel.persistent_name();
                let panel = entry.panel.clone();

                let is_active_button = i == active_index && is_open;
                let (action, tooltip) = if is_active_button {
                    let action = dock.toggle_action();

                    let tooltip: SharedString =
                        format!("Close {} dock", dock.position.to_label()).into();

                    (action, tooltip)
                } else {
                    let action = entry.panel.toggle_action(cx);

                    (action, icon_tooltip.into())
                };

                Some(
                    right_click_menu(name)
                        .menu(move |cx| {
                            const POSITIONS: [DockPosition; 3] = [
                                DockPosition::Left,
                                DockPosition::Right,
                                DockPosition::Bottom,
                            ];

                            ContextMenu::build(cx, |mut menu, cx| {
                                for position in POSITIONS {
                                    if position != dock_position
                                        && panel.position_is_valid(position, cx)
                                    {
                                        let panel = panel.clone();
                                        menu = menu.entry(
                                            format!("Dock {}", position.to_label()),
                                            None,
                                            move |cx| {
                                                panel.set_position(position, cx);
                                            },
                                        )
                                    }
                                }
                                menu
                            })
                        })
                        .anchor(menu_anchor)
                        .attach(menu_attach)
                        .trigger(
                            IconButton::new(name, icon)
                                .icon_size(IconSize::Small)
                                .selected(is_active_button)
                                .on_click({
                                    let action = action.boxed_clone();
                                    move |_, cx| cx.dispatch_action(action.boxed_clone())
                                })
                                .tooltip(move |cx| {
                                    Tooltip::for_action(tooltip.clone(), &*action, cx)
                                }),
                        ),
                )
            });

        h_flex().gap_0p5().children(buttons)
    }
}

impl StatusItemView for PanelButtons {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn crate::ItemHandle>,
        _cx: &mut ViewContext<Self>,
    ) {
        // Nothing to do, panel buttons don't depend on the active center item
    }
}

#[cfg(any(test, feature = "test-support"))]
pub mod test {
    use super::*;
    use gpui::{actions, div, ViewContext, WindowContext};

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
        pub fn new(position: DockPosition, cx: &mut WindowContext) -> Self {
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
        fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
            div().id("test").track_focus(&self.focus_handle)
        }
    }

    impl Panel for TestPanel {
        fn persistent_name() -> &'static str {
            "TestPanel"
        }

        fn position(&self, _: &gpui::WindowContext) -> super::DockPosition {
            self.position
        }

        fn position_is_valid(&self, _: super::DockPosition) -> bool {
            true
        }

        fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
            self.position = position;
            cx.update_global::<SettingsStore, _>(|_, _| {});
        }

        fn size(&self, _: &WindowContext) -> Pixels {
            self.size
        }

        fn set_size(&mut self, size: Option<Pixels>, _: &mut ViewContext<Self>) {
            self.size = size.unwrap_or(px(300.));
        }

        fn icon(&self, _: &WindowContext) -> Option<ui::IconName> {
            None
        }

        fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
            None
        }

        fn toggle_action(&self) -> Box<dyn Action> {
            ToggleTestPanel.boxed_clone()
        }

        fn is_zoomed(&self, _: &WindowContext) -> bool {
            self.zoomed
        }

        fn set_zoomed(&mut self, zoomed: bool, _cx: &mut ViewContext<Self>) {
            self.zoomed = zoomed;
        }

        fn set_active(&mut self, active: bool, _cx: &mut ViewContext<Self>) {
            self.active = active;
        }
    }

    impl FocusableView for TestPanel {
        fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
            self.focus_handle.clone()
        }
    }
}
