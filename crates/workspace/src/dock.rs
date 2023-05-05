use crate::{StatusItemView, Workspace};
use gpui::{
    elements::*, impl_actions, platform::CursorStyle, platform::MouseButton, AnyViewHandle,
    AppContext, Entity, Subscription, View, ViewContext, ViewHandle, WeakViewHandle, WindowContext,
};
use serde::Deserialize;
use settings::Settings;
use std::rc::Rc;

pub trait DockItem: View {
    fn should_activate_item_on_event(&self, _: &Self::Event, _: &AppContext) -> bool {
        false
    }
    fn should_show_badge(&self, _: &AppContext) -> bool {
        false
    }
    fn contains_focused_view(&self, _: &AppContext) -> bool {
        false
    }
}

pub trait DockItemHandle {
    fn id(&self) -> usize;
    fn should_show_badge(&self, cx: &WindowContext) -> bool;
    fn is_focused(&self, cx: &WindowContext) -> bool;
    fn as_any(&self) -> &AnyViewHandle;
}

impl<T> DockItemHandle for ViewHandle<T>
where
    T: DockItem,
{
    fn id(&self) -> usize {
        self.id()
    }

    fn should_show_badge(&self, cx: &WindowContext) -> bool {
        self.read(cx).should_show_badge(cx)
    }

    fn is_focused(&self, cx: &WindowContext) -> bool {
        ViewHandle::is_focused(self, cx) || self.read(cx).contains_focused_view(cx)
    }

    fn as_any(&self) -> &AnyViewHandle {
        self
    }
}

impl From<&dyn DockItemHandle> for AnyViewHandle {
    fn from(val: &dyn DockItemHandle) -> Self {
        val.as_any().clone()
    }
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
    Right,
}

impl DockPosition {
    fn to_resizable_side(self) -> Side {
        match self {
            Self::Left => Side::Right,
            Self::Right => Side::Left,
        }
    }
}

struct Item {
    icon_path: &'static str,
    tooltip: String,
    view: Rc<dyn DockItemHandle>,
    _subscriptions: [Subscription; 2],
}

pub struct PanelButtons {
    dock: ViewHandle<Dock>,
    workspace: WeakViewHandle<Workspace>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ToggleDockItem {
    pub dock_position: DockPosition,
    pub item_index: usize,
}

impl_actions!(workspace, [ToggleDockItem]);

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

    pub fn add_item<T: DockItem>(
        &mut self,
        icon_path: &'static str,
        tooltip: String,
        view: ViewHandle<T>,
        cx: &mut ViewContext<Self>,
    ) {
        let subscriptions = [
            cx.observe(&view, |_, _, cx| cx.notify()),
            cx.subscribe(&view, |this, view, event, cx| {
                if view.read(cx).should_activate_item_on_event(event, cx) {
                    if let Some(ix) = this
                        .items
                        .iter()
                        .position(|item| item.view.id() == view.id())
                    {
                        this.activate_item(ix, cx);
                    }
                }
            }),
        ];

        self.items.push(Item {
            icon_path,
            tooltip,
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

    pub fn active_item(&self) -> Option<&Rc<dyn DockItemHandle>> {
        if self.is_open {
            self.items.get(self.active_item_ix).map(|item| &item.view)
        } else {
            None
        }
    }
}

impl Entity for Dock {
    type Event = ();
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
        "DockToggleButton"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let theme = &cx.global::<Settings>().theme;
        let tooltip_style = theme.tooltip.clone();
        let theme = &theme.workspace.status_bar.panel_buttons;
        let dock = self.dock.read(cx);
        let item_style = theme.item.clone();
        let badge_style = theme.badge;
        let active_ix = dock.active_item_ix;
        let is_open = dock.is_open;
        let dock_position = dock.position;
        let group_style = match dock_position {
            DockPosition::Left => theme.group_left,
            DockPosition::Right => theme.group_right,
        };

        #[allow(clippy::needless_collect)]
        let items = dock
            .items
            .iter()
            .map(|item| (item.icon_path, item.tooltip.clone(), item.view.clone()))
            .collect::<Vec<_>>();

        Flex::row()
            .with_children(items.into_iter().enumerate().map(
                |(ix, (icon_path, tooltip, item_view))| {
                    let action = ToggleDockItem {
                        dock_position,
                        item_index: ix,
                    };
                    MouseEventHandler::<Self, _>::new(ix, cx, |state, cx| {
                        let is_active = is_open && ix == active_ix;
                        let style = item_style.style_for(state, is_active);
                        Stack::new()
                            .with_child(Svg::new(icon_path).with_color(style.icon_color))
                            .with_children(if !is_active && item_view.should_show_badge(cx) {
                                Some(
                                    Empty::new()
                                        .collapsed()
                                        .contained()
                                        .with_style(badge_style)
                                        .aligned()
                                        .bottom()
                                        .right(),
                                )
                            } else {
                                None
                            })
                            .constrained()
                            .with_width(style.icon_size)
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
                        tooltip,
                        Some(Box::new(action)),
                        tooltip_style.clone(),
                        cx,
                    )
                },
            ))
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
