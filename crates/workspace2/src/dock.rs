use crate::{status_bar::StatusItemView, Workspace};
use crate::{DockClickReset, DockDragState};
use gpui::{
    div, px, Action, AnchorCorner, AnyView, AppContext, Axis, ClickEvent, Div, Entity, EntityId,
    EventEmitter, FocusHandle, FocusableView, IntoElement, MouseButton, ParentElement, Render,
    SharedString, Styled, Subscription, View, ViewContext, VisualContext, WeakView, WindowContext,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ui::{h_stack, ContextMenu, IconButton, Tooltip};
use ui::{prelude::*, right_click_menu};

pub enum PanelEvent {
    ChangePosition,
    ZoomIn,
    ZoomOut,
    Activate,
    Close,
    Focus,
}

pub trait Panel: FocusableView + EventEmitter<PanelEvent> {
    fn persistent_name() -> &'static str;
    fn position(&self, cx: &WindowContext) -> DockPosition;
    fn position_is_valid(&self, position: DockPosition) -> bool;
    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>);
    fn size(&self, cx: &WindowContext) -> f32;
    fn set_size(&mut self, size: Option<f32>, cx: &mut ViewContext<Self>);
    // todo!("We should have a icon tooltip method, rather than using persistant_name")
    fn icon(&self, cx: &WindowContext) -> Option<ui::Icon>;
    fn toggle_action(&self) -> Box<dyn Action>;
    fn icon_label(&self, _: &WindowContext) -> Option<String> {
        None
    }
    fn is_zoomed(&self, _cx: &WindowContext) -> bool {
        false
    }
    fn set_zoomed(&mut self, _zoomed: bool, _cx: &mut ViewContext<Self>) {}
    fn set_active(&mut self, _active: bool, _cx: &mut ViewContext<Self>) {}
}

pub trait PanelHandle: Send + Sync {
    fn entity_id(&self) -> EntityId;
    fn persistent_name(&self) -> &'static str;
    fn position(&self, cx: &WindowContext) -> DockPosition;
    fn position_is_valid(&self, position: DockPosition, cx: &WindowContext) -> bool;
    fn set_position(&self, position: DockPosition, cx: &mut WindowContext);
    fn is_zoomed(&self, cx: &WindowContext) -> bool;
    fn set_zoomed(&self, zoomed: bool, cx: &mut WindowContext);
    fn set_active(&self, active: bool, cx: &mut WindowContext);
    fn size(&self, cx: &WindowContext) -> f32;
    fn set_size(&self, size: Option<f32>, cx: &mut WindowContext);
    fn icon(&self, cx: &WindowContext) -> Option<ui::Icon>;
    fn toggle_action(&self, cx: &WindowContext) -> Box<dyn Action>;
    fn icon_label(&self, cx: &WindowContext) -> Option<String>;
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle;
    fn to_any(&self) -> AnyView;
}

impl<T> PanelHandle for View<T>
where
    T: Panel,
{
    fn entity_id(&self) -> EntityId {
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

    fn size(&self, cx: &WindowContext) -> f32 {
        self.read(cx).size(cx)
    }

    fn set_size(&self, size: Option<f32>, cx: &mut WindowContext) {
        self.update(cx, |this, cx| this.set_size(size, cx))
    }

    fn icon(&self, cx: &WindowContext) -> Option<ui::Icon> {
        self.read(cx).icon(cx)
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

pub struct Dock {
    position: DockPosition,
    panel_entries: Vec<PanelEntry>,
    is_open: bool,
    active_panel_index: usize,
    focus_handle: FocusHandle,
    focus_subscription: Subscription,
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

    // todo!()
    // fn to_resize_handle_side(self) -> HandleSide {
    //     match self {
    //         Self::Left => HandleSide::Right,
    //         Self::Bottom => HandleSide::Top,
    //         Self::Right => HandleSide::Left,
    //     }
    // }

    pub fn axis(&self) -> Axis {
        match self {
            Self::Left | Self::Right => Axis::Horizontal,
            Self::Bottom => Axis::Vertical,
        }
    }
}

struct PanelEntry {
    panel: Arc<dyn PanelHandle>,
    // todo!()
    // context_menu: View<ContextMenu>,
    _subscriptions: [Subscription; 2],
}

pub struct PanelButtons {
    dock: View<Dock>,
    workspace: WeakView<Workspace>,
}

impl Dock {
    pub fn new(position: DockPosition, cx: &mut ViewContext<'_, Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let focus_subscription = cx.on_focus(&focus_handle, |dock, cx| {
            if let Some(active_entry) = dock.panel_entries.get(dock.active_panel_index) {
                active_entry.panel.focus_handle(cx).focus(cx)
            }
        });
        Self {
            position,
            panel_entries: Default::default(),
            active_panel_index: 0,
            is_open: false,
            focus_handle,
            focus_subscription,
        }
    }

    pub fn position(&self) -> DockPosition {
        self.position
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    // todo!()
    //     pub fn has_focus(&self, cx: &WindowContext) -> bool {
    //         self.visible_panel()
    //             .map_or(false, |panel| panel.has_focus(cx))
    //     }

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
            if entry.panel.entity_id() == panel.entity_id() {
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
            cx.subscribe(&panel, move |this, panel, event, cx| match event {
                PanelEvent::ChangePosition => {
                    let new_position = panel.read(cx).position(cx);

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
                            active_panel.entity_id() == Entity::entity_id(&panel)
                        });

                    this.remove_panel(&panel, cx);

                    new_dock.update(cx, |new_dock, cx| {
                        new_dock.add_panel(panel.clone(), workspace.clone(), cx);
                        if was_visible {
                            new_dock.set_open(true, cx);
                            new_dock.activate_panel(this.panels_len() - 1, cx);
                        }
                    });
                }
                PanelEvent::ZoomIn => {
                    this.set_panel_zoomed(&panel.to_any(), true, cx);
                    if !panel.focus_handle(cx).contains_focused(cx) {
                        cx.focus_view(&panel);
                    }
                    workspace
                        .update(cx, |workspace, cx| {
                            workspace.zoomed = Some(panel.downgrade().into());
                            workspace.zoomed_position = Some(panel.read(cx).position(cx));
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
                            }
                            cx.notify();
                        })
                        .ok();
                }
                PanelEvent::Activate => {
                    if let Some(ix) = this
                        .panel_entries
                        .iter()
                        .position(|entry| entry.panel.entity_id() == Entity::entity_id(&panel))
                    {
                        this.set_open(true, cx);
                        this.activate_panel(ix, cx);
                        cx.focus_view(&panel);
                    }
                }
                PanelEvent::Close => {
                    if this
                        .visible_panel()
                        .map_or(false, |p| p.entity_id() == Entity::entity_id(&panel))
                    {
                        this.set_open(false, cx);
                    }
                }
                PanelEvent::Focus => {}
            }),
        ];

        // todo!()
        // let dock_view_id = cx.view_id();
        self.panel_entries.push(PanelEntry {
            panel: Arc::new(panel),
            // todo!()
            // context_menu: cx.add_view(|cx| {
            //     let mut menu = ContextMenu::new(dock_view_id, cx);
            //     menu.set_position_mode(OverlayPositionMode::Local);
            //     menu
            // }),
            _subscriptions: subscriptions,
        });
        cx.notify()
    }

    pub fn remove_panel<T: Panel>(&mut self, panel: &View<T>, cx: &mut ViewContext<Self>) {
        if let Some(panel_ix) = self
            .panel_entries
            .iter()
            .position(|entry| entry.panel.entity_id() == Entity::entity_id(panel))
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

    pub fn panel_size(&self, panel: &dyn PanelHandle, cx: &WindowContext) -> Option<f32> {
        self.panel_entries
            .iter()
            .find(|entry| entry.panel.entity_id() == panel.entity_id())
            .map(|entry| entry.panel.size(cx))
    }

    pub fn active_panel_size(&self, cx: &WindowContext) -> Option<f32> {
        if self.is_open {
            self.panel_entries
                .get(self.active_panel_index)
                .map(|entry| entry.panel.size(cx))
        } else {
            None
        }
    }

    pub fn resize_active_panel(&mut self, size: Option<f32>, cx: &mut ViewContext<Self>) {
        if let Some(entry) = self.panel_entries.get_mut(self.active_panel_index) {
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
}

impl Render for Dock {
    type Element = Div;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        if let Some(entry) = self.visible_entry() {
            let size = entry.panel.size(cx);

            let mut pre_resize_handle = None;
            let mut post_resize_handle = None;
            let position = self.position;
            let handler = div()
                .id("resize-handle")
                .bg(gpui::red())
                .on_mouse_down(gpui::MouseButton::Left, move |_, cx| {
                    cx.update_global(|drag: &mut DockDragState, cx| drag.0 = Some(position))
                })
                .on_click(cx.listener(|v, e: &ClickEvent, cx| {
                    if e.down.button == MouseButton::Left {
                        cx.update_global(|state: &mut DockClickReset, cx| {
                            if state.0.is_some() {
                                state.0 = None;
                                v.resize_active_panel(None, cx)
                            } else {
                                let double_click = cx.double_click_interval();
                                let timer = cx.background_executor().timer(double_click);
                                state.0 = Some(cx.spawn(|_, mut cx| async move {
                                    timer.await;
                                    cx.update_global(|state: &mut DockClickReset, cx| {
                                        state.0 = None;
                                    })
                                    .ok();
                                }));
                            }
                        })
                    }
                }));

            match self.position() {
                DockPosition::Left => {
                    post_resize_handle = Some(handler.w_2().h_full().cursor_col_resize())
                }
                DockPosition::Bottom => {
                    pre_resize_handle = Some(handler.w_full().h_2().cursor_row_resize())
                }
                DockPosition::Right => {
                    pre_resize_handle = Some(handler.w_full().h_1().cursor_col_resize())
                }
            }

            div()
                .border_color(cx.theme().colors().border)
                .map(|this| match self.position().axis() {
                    Axis::Horizontal => this.w(px(size)).h_full(),
                    Axis::Vertical => this.h(px(size)).w_full(),
                })
                .map(|this| match self.position() {
                    DockPosition::Left => this.border_r(),
                    DockPosition::Right => this.border_l(),
                    DockPosition::Bottom => this.border_t(),
                })
                .children(pre_resize_handle)
                .child(entry.panel.to_any())
                .children(post_resize_handle)
        } else {
            div()
        }
    }
}

impl PanelButtons {
    pub fn new(
        dock: View<Dock>,
        workspace: WeakView<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        cx.observe(&dock, |_, _, cx| cx.notify()).detach();
        Self { dock, workspace }
    }
}

// impl Render for PanelButtons {
//     type Element = ();

//     fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
//         todo!("")
//     }

//     fn ui_name() -> &'static str {
//         "PanelButtons"
//     }

//     fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
//         let theme = &settings::get::<ThemeSettings>(cx).theme;
//         let tooltip_style = theme.tooltip.clone();
//         let theme = &theme.workspace.status_bar.panel_buttons;
//         let button_style = theme.button.clone();
//         let dock = self.dock.read(cx);
//         let active_ix = dock.active_panel_index;
//         let is_open = dock.is_open;
//         let dock_position = dock.position;
//         let group_style = match dock_position {
//             DockPosition::Left => theme.group_left,
//             DockPosition::Bottom => theme.group_bottom,
//             DockPosition::Right => theme.group_right,
//         };
//         let menu_corner = match dock_position {
//             DockPosition::Left => AnchorCorner::BottomLeft,
//             DockPosition::Bottom | DockPosition::Right => AnchorCorner::BottomRight,
//         };

//         let panels = dock
//             .panel_entries
//             .iter()
//             .map(|item| (item.panel.clone(), item.context_menu.clone()))
//             .collect::<Vec<_>>();
//         Flex::row()
//             .with_children(panels.into_iter().enumerate().filter_map(
//                 |(panel_ix, (view, context_menu))| {
//                     let icon_path = view.icon_path(cx)?;
//                     let is_active = is_open && panel_ix == active_ix;
//                     let (tooltip, tooltip_action) = if is_active {
//                         (
//                             format!("Close {} dock", dock_position.to_label()),
//                             Some(match dock_position {
//                                 DockPosition::Left => crate::ToggleLeftDock.boxed_clone(),
//                                 DockPosition::Bottom => crate::ToggleBottomDock.boxed_clone(),
//                                 DockPosition::Right => crate::ToggleRightDock.boxed_clone(),
//                             }),
//                         )
//                     } else {
//                         view.icon_tooltip(cx)
//                     };
//                     Some(
//                         Stack::new()
//                             .with_child(
//                                 MouseEventHandler::new::<Self, _>(panel_ix, cx, |state, cx| {
//                                     let style = button_style.in_state(is_active);

//                                     let style = style.style_for(state);
//                                     Flex::row()
//                                         .with_child(
//                                             Svg::new(icon_path)
//                                                 .with_color(style.icon_color)
//                                                 .constrained()
//                                                 .with_width(style.icon_size)
//                                                 .aligned(),
//                                         )
//                                         .with_children(if let Some(label) = view.icon_label(cx) {
//                                             Some(
//                                                 Label::new(label, style.label.text.clone())
//                                                     .contained()
//                                                     .with_style(style.label.container)
//                                                     .aligned(),
//                                             )
//                                         } else {
//                                             None
//                                         })
//                                         .constrained()
//                                         .with_height(style.icon_size)
//                                         .contained()
//                                         .with_style(style.container)
//                                 })
//                                 .with_cursor_style(CursorStyle::PointingHand)
//                                 .on_click(MouseButton::Left, {
//                                     let tooltip_action =
//                                         tooltip_action.as_ref().map(|action| action.boxed_clone());
//                                     move |_, this, cx| {
//                                         if let Some(tooltip_action) = &tooltip_action {
//                                             let window = cx.window();
//                                             let view_id = this.workspace.id();
//                                             let tooltip_action = tooltip_action.boxed_clone();
//                                             cx.spawn(|_, mut cx| async move {
//                                                 window.dispatch_action(
//                                                     view_id,
//                                                     &*tooltip_action,
//                                                     &mut cx,
//                                                 );
//                                             })
//                                             .detach();
//                                         }
//                                     }
//                                 })
//                                 .on_click(MouseButton::Right, {
//                                     let view = view.clone();
//                                     let menu = context_menu.clone();
//                                     move |_, _, cx| {
//                                         const POSITIONS: [DockPosition; 3] = [
//                                             DockPosition::Left,
//                                             DockPosition::Right,
//                                             DockPosition::Bottom,
//                                         ];

//                                         menu.update(cx, |menu, cx| {
//                                             let items = POSITIONS
//                                                 .into_iter()
//                                                 .filter(|position| {
//                                                     *position != dock_position
//                                                         && view.position_is_valid(*position, cx)
//                                                 })
//                                                 .map(|position| {
//                                                     let view = view.clone();
//                                                     ContextMenuItem::handler(
//                                                         format!("Dock {}", position.to_label()),
//                                                         move |cx| view.set_position(position, cx),
//                                                     )
//                                                 })
//                                                 .collect();
//                                             menu.show(Default::default(), menu_corner, items, cx);
//                                         })
//                                     }
//                                 })
//                                 .with_tooltip::<Self>(
//                                     panel_ix,
//                                     tooltip,
//                                     tooltip_action,
//                                     tooltip_style.clone(),
//                                     cx,
//                                 ),
//                             )
//                             .with_child(ChildView::new(&context_menu, cx)),
//                     )
//                 },
//             ))
//             .contained()
//             .with_style(group_style)
//             .into_any()
//     }
// }

// here be kittens
impl Render for PanelButtons {
    type Element = Div;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        // todo!()
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

                    (action, name.into())
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
                                        menu = menu.entry(position.to_label(), move |cx| {
                                            panel.set_position(position, cx);
                                        })
                                    }
                                }
                                menu
                            })
                        })
                        .anchor(menu_anchor)
                        .attach(menu_attach)
                        .trigger(
                            IconButton::new(name, icon)
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

        h_stack().gap_0p5().children(buttons)
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
    use gpui::{actions, div, Div, ViewContext, WindowContext};

    pub struct TestPanel {
        pub position: DockPosition,
        pub zoomed: bool,
        pub active: bool,
        pub focus_handle: FocusHandle,
        pub size: f32,
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
                size: 300.,
            }
        }
    }

    impl Render for TestPanel {
        type Element = Div;

        fn render(&mut self, _cx: &mut ViewContext<Self>) -> Self::Element {
            div()
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
            cx.emit(PanelEvent::ChangePosition);
        }

        fn size(&self, _: &WindowContext) -> f32 {
            self.size
        }

        fn set_size(&mut self, size: Option<f32>, _: &mut ViewContext<Self>) {
            self.size = size.unwrap_or(300.);
        }

        fn icon(&self, _: &WindowContext) -> Option<ui::Icon> {
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
