use crate::{StatusItemView, Workspace};
use gpui::{
    elements::*, impl_actions, platform::CursorStyle, platform::MouseButton, AnyViewHandle,
    AppContext, Entity, Subscription, View, ViewContext, ViewHandle, WeakViewHandle, WindowContext,
};
use serde::Deserialize;
use settings::Settings;
use std::rc::Rc;

pub trait SidebarItem: View {
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

pub trait SidebarItemHandle {
    fn id(&self) -> usize;
    fn should_show_badge(&self, cx: &WindowContext) -> bool;
    fn is_focused(&self, cx: &WindowContext) -> bool;
    fn as_any(&self) -> &AnyViewHandle;
}

impl<T> SidebarItemHandle for ViewHandle<T>
where
    T: SidebarItem,
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

impl From<&dyn SidebarItemHandle> for AnyViewHandle {
    fn from(val: &dyn SidebarItemHandle) -> Self {
        val.as_any().clone()
    }
}

pub struct Sidebar {
    sidebar_side: SidebarSide,
    items: Vec<Item>,
    is_open: bool,
    active_item_ix: usize,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum SidebarSide {
    Left,
    Right,
}

impl SidebarSide {
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
    view: Rc<dyn SidebarItemHandle>,
    _subscriptions: [Subscription; 2],
}

pub struct SidebarButtons {
    sidebar: ViewHandle<Sidebar>,
    workspace: WeakViewHandle<Workspace>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ToggleSidebarItem {
    pub sidebar_side: SidebarSide,
    pub item_index: usize,
}

impl_actions!(workspace, [ToggleSidebarItem]);

impl Sidebar {
    pub fn new(sidebar_side: SidebarSide) -> Self {
        Self {
            sidebar_side,
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

    pub fn add_item<T: SidebarItem>(
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
        cx.reparent(&view);
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

    pub fn active_item(&self) -> Option<&Rc<dyn SidebarItemHandle>> {
        if self.is_open {
            self.items.get(self.active_item_ix).map(|item| &item.view)
        } else {
            None
        }
    }
}

impl Entity for Sidebar {
    type Event = ();
}

impl View for Sidebar {
    fn ui_name() -> &'static str {
        "Sidebar"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        if let Some(active_item) = self.active_item() {
            enum ResizeHandleTag {}
            let style = &cx.global::<Settings>().theme.workspace.sidebar;
            ChildView::new(active_item.as_any(), cx)
                .contained()
                .with_style(style.container)
                .with_resize_handle::<ResizeHandleTag>(
                    self.sidebar_side as usize,
                    self.sidebar_side.to_resizable_side(),
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

impl SidebarButtons {
    pub fn new(
        sidebar: ViewHandle<Sidebar>,
        workspace: WeakViewHandle<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        cx.observe(&sidebar, |_, _, cx| cx.notify()).detach();
        Self { sidebar, workspace }
    }
}

impl Entity for SidebarButtons {
    type Event = ();
}

impl View for SidebarButtons {
    fn ui_name() -> &'static str {
        "SidebarToggleButton"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let theme = &cx.global::<Settings>().theme;
        let tooltip_style = theme.tooltip.clone();
        let theme = &theme.workspace.status_bar.sidebar_buttons;
        let sidebar = self.sidebar.read(cx);
        let item_style = theme.item.clone();
        let badge_style = theme.badge;
        let active_ix = sidebar.active_item_ix;
        let is_open = sidebar.is_open;
        let sidebar_side = sidebar.sidebar_side;
        let group_style = match sidebar_side {
            SidebarSide::Left => theme.group_left,
            SidebarSide::Right => theme.group_right,
        };

        #[allow(clippy::needless_collect)]
        let items = sidebar
            .items
            .iter()
            .map(|item| (item.icon_path, item.tooltip.clone(), item.view.clone()))
            .collect::<Vec<_>>();

        Flex::row()
            .with_children(items.into_iter().enumerate().map(
                |(ix, (icon_path, tooltip, item_view))| {
                    let action = ToggleSidebarItem {
                        sidebar_side,
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
                                        workspace.toggle_sidebar_item(&action, cx)
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

impl StatusItemView for SidebarButtons {
    fn set_active_pane_item(
        &mut self,
        _: Option<&dyn crate::ItemHandle>,
        _: &mut ViewContext<Self>,
    ) {
    }
}
