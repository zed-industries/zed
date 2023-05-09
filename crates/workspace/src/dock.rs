use crate::{StatusItemView, Workspace};
use gpui::{
    elements::*, impl_actions, platform::CursorStyle, platform::MouseButton, AnyViewHandle,
    AppContext, Entity, Subscription, View, ViewContext, ViewHandle, WeakViewHandle, WindowContext,
};
use serde::Deserialize;
use settings::Settings;
use std::rc::Rc;

pub trait Panel: View {
    fn position(&self, cx: &WindowContext) -> DockPosition;
    fn position_is_valid(&self, position: DockPosition) -> bool;
    fn icon_path(&self) -> &'static str;
    fn icon_tooltip(&self) -> String;
    fn icon_label(&self, _: &AppContext) -> Option<String> {
        None
    }
    fn should_change_position_on_event(&self, _: &Self::Event, _: &AppContext) -> bool;
    fn should_activate_on_event(&self, _: &Self::Event, _: &AppContext) -> bool;
    fn should_close_on_event(&self, _: &Self::Event, _: &AppContext) -> bool;
}

pub trait PanelHandle {
    fn id(&self) -> usize;
    fn position(&self, cx: &WindowContext) -> DockPosition;
    fn position_is_valid(&self, position: DockPosition, cx: &WindowContext) -> bool;
    fn icon_path(&self, cx: &WindowContext) -> &'static str;
    fn icon_tooltip(&self, cx: &WindowContext) -> String;
    fn icon_label(&self, cx: &WindowContext) -> Option<String>;
    fn is_focused(&self, cx: &WindowContext) -> bool;
    fn as_any(&self) -> &AnyViewHandle;
}

impl<T> PanelHandle for ViewHandle<T>
where
    T: Panel,
{
    fn id(&self) -> usize {
        self.id()
    }

    fn position(&self, cx: &WindowContext) -> DockPosition {
        self.read(cx).position(cx)
    }

    fn position_is_valid(&self, position: DockPosition, cx: &WindowContext) -> bool {
        self.read(cx).position_is_valid(position)
    }

    fn icon_path(&self, cx: &WindowContext) -> &'static str {
        self.read(cx).icon_path()
    }

    fn icon_tooltip(&self, cx: &WindowContext) -> String {
        self.read(cx).icon_tooltip()
    }

    fn icon_label(&self, cx: &WindowContext) -> Option<String> {
        self.read(cx).icon_label(cx)
    }

    fn is_focused(&self, cx: &WindowContext) -> bool {
        ViewHandle::is_focused(self, cx)
    }

    fn as_any(&self) -> &AnyViewHandle {
        self
    }
}

impl From<&dyn PanelHandle> for AnyViewHandle {
    fn from(val: &dyn PanelHandle) -> Self {
        val.as_any().clone()
    }
}

pub enum Event {
    Close,
}

pub struct Dock {
    position: DockPosition,
    items: Vec<Item>,
    is_open: bool,
    active_item_ix: usize,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum DockPosition {
    Left,
    Bottom,
    Right,
}

impl DockPosition {
    fn to_resizable_side(self) -> Side {
        match self {
            Self::Left => Side::Right,
            Self::Bottom => Side::Top,
            Self::Right => Side::Left,
        }
    }
}

struct Item {
    view: Rc<dyn PanelHandle>,
    _subscriptions: [Subscription; 2],
}

pub struct PanelButtons {
    dock: ViewHandle<Dock>,
    workspace: WeakViewHandle<Workspace>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct TogglePanel {
    pub dock_position: DockPosition,
    pub item_index: usize,
}

impl_actions!(workspace, [TogglePanel]);

impl Dock {
    pub fn new(position: DockPosition) -> Self {
        Self {
            position,
            items: Default::default(),
            active_item_ix: 0,
            is_open: false,
        }
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn active_item_ix(&self) -> usize {
        self.active_item_ix
    }

    pub fn set_open(&mut self, open: bool, cx: &mut ViewContext<Self>) {
        if open != self.is_open {
            self.is_open = open;
            cx.notify();
        }
    }

    pub fn toggle_open(&mut self, cx: &mut ViewContext<Self>) {
        if self.is_open {}
        self.is_open = !self.is_open;
        cx.notify();
    }

    pub fn add_panel<T: Panel>(&mut self, view: ViewHandle<T>, cx: &mut ViewContext<Self>) {
        let subscriptions = [
            cx.observe(&view, |_, _, cx| cx.notify()),
            cx.subscribe(&view, |this, view, event, cx| {
                if view.read(cx).should_activate_on_event(event, cx) {
                    if let Some(ix) = this
                        .items
                        .iter()
                        .position(|item| item.view.id() == view.id())
                    {
                        this.activate_item(ix, cx);
                    }
                } else if view.read(cx).should_close_on_event(event, cx) {
                    cx.emit(Event::Close);
                }
            }),
        ];

        self.items.push(Item {
            view: Rc::new(view),
            _subscriptions: subscriptions,
        });
        cx.notify()
    }

    pub fn activate_item(&mut self, item_ix: usize, cx: &mut ViewContext<Self>) {
        self.active_item_ix = item_ix;
        cx.notify();
    }

    pub fn toggle_item(&mut self, item_ix: usize, cx: &mut ViewContext<Self>) {
        if self.active_item_ix == item_ix {
            self.is_open = false;
        } else {
            self.active_item_ix = item_ix;
        }
        cx.notify();
    }

    pub fn active_item(&self) -> Option<&Rc<dyn PanelHandle>> {
        if self.is_open {
            self.items.get(self.active_item_ix).map(|item| &item.view)
        } else {
            None
        }
    }
}

impl Entity for Dock {
    type Event = Event;
}

impl View for Dock {
    fn ui_name() -> &'static str {
        "Dock"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        if let Some(active_item) = self.active_item() {
            enum ResizeHandleTag {}
            let style = &cx.global::<Settings>().theme.workspace.dock;
            ChildView::new(active_item.as_any(), cx)
                .contained()
                .with_style(style.container)
                .with_resize_handle::<ResizeHandleTag>(
                    self.position as usize,
                    self.position.to_resizable_side(),
                    4.,
                    style.initial_size,
                    cx,
                )
                .into_any()
        } else {
            Empty::new().into_any()
        }
    }
}

impl PanelButtons {
    pub fn new(
        dock: ViewHandle<Dock>,
        workspace: WeakViewHandle<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        cx.observe(&dock, |_, _, cx| cx.notify()).detach();
        Self { dock, workspace }
    }
}

impl Entity for PanelButtons {
    type Event = ();
}

impl View for PanelButtons {
    fn ui_name() -> &'static str {
        "PanelButtons"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let theme = &cx.global::<Settings>().theme;
        let tooltip_style = theme.tooltip.clone();
        let theme = &theme.workspace.status_bar.panel_buttons;
        let item_style = theme.button.clone();
        let dock = self.dock.read(cx);
        let active_ix = dock.active_item_ix;
        let is_open = dock.is_open;
        let dock_position = dock.position;
        let group_style = match dock_position {
            DockPosition::Left => theme.group_left,
            DockPosition::Bottom => theme.group_bottom,
            DockPosition::Right => theme.group_right,
        };

        let items = dock
            .items
            .iter()
            .map(|item| item.view.clone())
            .collect::<Vec<_>>();
        Flex::row()
            .with_children(items.into_iter().enumerate().map(|(ix, view)| {
                let action = TogglePanel {
                    dock_position,
                    item_index: ix,
                };
                MouseEventHandler::<Self, _>::new(ix, cx, |state, cx| {
                    let is_active = is_open && ix == active_ix;
                    let style = item_style.style_for(state, is_active);
                    Flex::row()
                        .with_child(
                            Svg::new(view.icon_path(cx))
                                .with_color(style.icon_color)
                                .constrained()
                                .with_width(style.icon_size)
                                .aligned(),
                        )
                        .with_children(if let Some(label) = view.icon_label(cx) {
                            Some(
                                Label::new(label, style.label.text.clone())
                                    .contained()
                                    .with_style(style.label.container)
                                    .aligned(),
                            )
                        } else {
                            None
                        })
                        .constrained()
                        .with_height(style.icon_size)
                        .contained()
                        .with_style(style.container)
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, {
                    let action = action.clone();
                    move |_, this, cx| {
                        if let Some(workspace) = this.workspace.upgrade(cx) {
                            let action = action.clone();
                            cx.window_context().defer(move |cx| {
                                workspace.update(cx, |workspace, cx| {
                                    workspace.toggle_panel(&action, cx)
                                });
                            });
                        }
                    }
                })
                .with_tooltip::<Self>(
                    ix,
                    view.icon_tooltip(cx),
                    Some(Box::new(action)),
                    tooltip_style.clone(),
                    cx,
                )
            }))
            .contained()
            .with_style(group_style)
            .into_any()
    }
}

impl StatusItemView for PanelButtons {
    fn set_active_pane_item(
        &mut self,
        _: Option<&dyn crate::ItemHandle>,
        _: &mut ViewContext<Self>,
    ) {
    }
}
