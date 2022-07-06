use crate::{ItemHandle, Pane};
use gpui::{
    elements::*, platform::CursorStyle, Action, AnyViewHandle, AppContext, ElementBox, Entity,
    MutableAppContext, RenderContext, View, ViewContext, ViewHandle, WeakViewHandle,
};
use settings::Settings;

pub trait ToolbarItemView: View {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn crate::ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) -> ToolbarItemLocation;

    fn location_for_event(
        &self,
        _event: &Self::Event,
        current_location: ToolbarItemLocation,
        _cx: &AppContext,
    ) -> ToolbarItemLocation {
        current_location
    }
}

trait ToolbarItemViewHandle {
    fn id(&self) -> usize;
    fn to_any(&self) -> AnyViewHandle;
    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut MutableAppContext,
    ) -> ToolbarItemLocation;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ToolbarItemLocation {
    Hidden,
    PrimaryLeft { flex: Option<(f32, bool)> },
    PrimaryRight { flex: Option<(f32, bool)> },
    Secondary,
}

pub struct Toolbar {
    active_pane_item: Option<Box<dyn ItemHandle>>,
    pane: WeakViewHandle<Pane>,
    items: Vec<(Box<dyn ToolbarItemViewHandle>, ToolbarItemLocation)>,
}

impl Entity for Toolbar {
    type Event = ();
}

impl View for Toolbar {
    fn ui_name() -> &'static str {
        "Toolbar"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = &cx.global::<Settings>().theme.workspace.toolbar;

        let mut primary_left_items = Vec::new();
        let mut primary_right_items = Vec::new();
        let mut secondary_item = None;
        let spacing = theme.item_spacing;

        for (item, position) in &self.items {
            match *position {
                ToolbarItemLocation::Hidden => {}
                ToolbarItemLocation::PrimaryLeft { flex } => {
                    let left_item = ChildView::new(item.as_ref())
                        .aligned()
                        .contained()
                        .with_margin_right(spacing);
                    if let Some((flex, expanded)) = flex {
                        primary_left_items.push(left_item.flex(flex, expanded).boxed());
                    } else {
                        primary_left_items.push(left_item.boxed());
                    }
                }
                ToolbarItemLocation::PrimaryRight { flex } => {
                    let right_item = ChildView::new(item.as_ref())
                        .aligned()
                        .contained()
                        .with_margin_left(spacing)
                        .flex_float();
                    if let Some((flex, expanded)) = flex {
                        primary_right_items.push(right_item.flex(flex, expanded).boxed());
                    } else {
                        primary_right_items.push(right_item.boxed());
                    }
                }
                ToolbarItemLocation::Secondary => {
                    secondary_item = Some(
                        ChildView::new(item.as_ref())
                            .constrained()
                            .with_height(theme.height)
                            .boxed(),
                    );
                }
            }
        }

        let pane = self.pane.clone();
        let mut enable_go_backward = false;
        let mut enable_go_forward = false;
        if let Some(pane) = pane.upgrade(cx) {
            let pane = pane.read(cx);
            enable_go_backward = pane.can_navigate_backward();
            enable_go_forward = pane.can_navigate_forward();
        }

        let container_style = theme.container;
        let height = theme.height;
        let button_style = theme.nav_button;

        Flex::column()
            .with_child(
                Flex::row()
                    .with_child(nav_button(
                        "icons/arrow-left.svg",
                        button_style,
                        enable_go_backward,
                        spacing,
                        super::GoBack {
                            pane: Some(pane.clone()),
                        },
                        cx,
                    ))
                    .with_child(nav_button(
                        "icons/arrow-right.svg",
                        button_style,
                        enable_go_forward,
                        spacing,
                        super::GoForward {
                            pane: Some(pane.clone()),
                        },
                        cx,
                    ))
                    .with_children(primary_left_items)
                    .with_children(primary_right_items)
                    .constrained()
                    .with_height(height)
                    .boxed(),
            )
            .with_children(secondary_item)
            .contained()
            .with_style(container_style)
            .boxed()
    }
}

fn nav_button<A: Action + Clone>(
    svg_path: &'static str,
    style: theme::Interactive<theme::IconButton>,
    enabled: bool,
    spacing: f32,
    action: A,
    cx: &mut RenderContext<Toolbar>,
) -> ElementBox {
    MouseEventHandler::new::<A, _, _>(0, cx, |state, _| {
        let style = if enabled {
            style.style_for(state, false)
        } else {
            style.disabled_style()
        };
        Svg::new(svg_path)
            .with_color(style.color)
            .constrained()
            .with_width(style.icon_width)
            .aligned()
            .contained()
            .with_style(style.container)
            .constrained()
            .with_width(style.button_width)
            .with_height(style.button_width)
            .boxed()
    })
    .with_cursor_style(if enabled {
        CursorStyle::PointingHand
    } else {
        CursorStyle::default()
    })
    .on_mouse_down(move |_, cx| cx.dispatch_action(action.clone()))
    .contained()
    .with_margin_right(spacing)
    .boxed()
}

impl Toolbar {
    pub fn new(pane: WeakViewHandle<Pane>) -> Self {
        Self {
            active_pane_item: None,
            pane,
            items: Default::default(),
        }
    }

    pub fn add_item<T>(&mut self, item: ViewHandle<T>, cx: &mut ViewContext<Self>)
    where
        T: 'static + ToolbarItemView,
    {
        let location = item.set_active_pane_item(self.active_pane_item.as_deref(), cx);
        cx.subscribe(&item, |this, item, event, cx| {
            if let Some((_, current_location)) =
                this.items.iter_mut().find(|(i, _)| i.id() == item.id())
            {
                let new_location = item
                    .read(cx)
                    .location_for_event(event, *current_location, cx);
                if new_location != *current_location {
                    *current_location = new_location;
                    cx.notify();
                }
            }
        })
        .detach();
        self.items.push((Box::new(item), location));
        cx.notify();
    }

    pub fn set_active_pane_item(
        &mut self,
        pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) {
        self.active_pane_item = pane_item.map(|item| item.boxed_clone());
        for (toolbar_item, current_location) in self.items.iter_mut() {
            let new_location = toolbar_item.set_active_pane_item(pane_item, cx);
            if new_location != *current_location {
                *current_location = new_location;
                cx.notify();
            }
        }
    }

    pub fn item_of_type<T: ToolbarItemView>(&self) -> Option<ViewHandle<T>> {
        self.items
            .iter()
            .find_map(|(item, _)| item.to_any().downcast())
    }
}

impl<T: ToolbarItemView> ToolbarItemViewHandle for ViewHandle<T> {
    fn id(&self) -> usize {
        self.id()
    }

    fn to_any(&self) -> AnyViewHandle {
        self.into()
    }

    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut MutableAppContext,
    ) -> ToolbarItemLocation {
        self.update(cx, |this, cx| {
            this.set_active_pane_item(active_pane_item, cx)
        })
    }
}

impl Into<AnyViewHandle> for &dyn ToolbarItemViewHandle {
    fn into(self) -> AnyViewHandle {
        self.to_any()
    }
}
