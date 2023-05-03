use crate::{ItemHandle, Pane};
use gpui::{
    elements::*, platform::CursorStyle, platform::MouseButton, Action, AnyElement, AnyViewHandle,
    AppContext, Entity, View, ViewContext, ViewHandle, WeakViewHandle, WindowContext,
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

    fn pane_focus_update(&mut self, _pane_focused: bool, _cx: &mut ViewContext<Self>) {}
}

trait ToolbarItemViewHandle {
    fn id(&self) -> usize;
    fn as_any(&self) -> &AnyViewHandle;
    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut WindowContext,
    ) -> ToolbarItemLocation;
    fn pane_focus_update(&mut self, pane_focused: bool, cx: &mut WindowContext);
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
    hidden: bool,
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

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let theme = &cx.global::<Settings>().theme.workspace.toolbar;

        let mut primary_left_items = Vec::new();
        let mut primary_right_items = Vec::new();
        let mut secondary_item = None;
        let spacing = theme.item_spacing;

        for (item, position) in &self.items {
            match *position {
                ToolbarItemLocation::Hidden => {}

                ToolbarItemLocation::PrimaryLeft { flex } => {
                    let left_item = ChildView::new(item.as_any(), cx)
                        .aligned()
                        .contained()
                        .with_margin_right(spacing);
                    if let Some((flex, expanded)) = flex {
                        primary_left_items.push(left_item.flex(flex, expanded).into_any());
                    } else {
                        primary_left_items.push(left_item.into_any());
                    }
                }

                ToolbarItemLocation::PrimaryRight { flex } => {
                    let right_item = ChildView::new(item.as_any(), cx)
                        .aligned()
                        .contained()
                        .with_margin_left(spacing)
                        .flex_float();
                    if let Some((flex, expanded)) = flex {
                        primary_right_items.push(right_item.flex(flex, expanded).into_any());
                    } else {
                        primary_right_items.push(right_item.into_any());
                    }
                }

                ToolbarItemLocation::Secondary => {
                    secondary_item = Some(
                        ChildView::new(item.as_any(), cx)
                            .constrained()
                            .with_height(theme.height)
                            .into_any(),
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
        let tooltip_style = cx.global::<Settings>().theme.tooltip.clone();

        Flex::column()
            .with_child(
                Flex::row()
                    .with_child(nav_button(
                        "icons/arrow_left_16.svg",
                        button_style,
                        tooltip_style.clone(),
                        enable_go_backward,
                        spacing,
                        {
                            let pane = pane.clone();
                            move |toolbar, cx| {
                                if let Some(workspace) = toolbar
                                    .pane
                                    .upgrade(cx)
                                    .and_then(|pane| pane.read(cx).workspace().upgrade(cx))
                                {
                                    let pane = pane.clone();
                                    cx.window_context().defer(move |cx| {
                                        workspace.update(cx, |workspace, cx| {
                                            Pane::go_back(workspace, Some(pane.clone()), cx)
                                                .detach_and_log_err(cx);
                                        });
                                    })
                                }
                            }
                        },
                        super::GoBack { pane: None },
                        "Go Back",
                        cx,
                    ))
                    .with_child(nav_button(
                        "icons/arrow_right_16.svg",
                        button_style,
                        tooltip_style,
                        enable_go_forward,
                        spacing,
                        {
                            let pane = pane.clone();
                            move |toolbar, cx| {
                                if let Some(workspace) = toolbar
                                    .pane
                                    .upgrade(cx)
                                    .and_then(|pane| pane.read(cx).workspace().upgrade(cx))
                                {
                                    let pane = pane.clone();
                                    cx.window_context().defer(move |cx| {
                                        workspace.update(cx, |workspace, cx| {
                                            Pane::go_forward(workspace, Some(pane.clone()), cx)
                                                .detach_and_log_err(cx);
                                        });
                                    });
                                }
                            }
                        },
                        super::GoForward { pane: None },
                        "Go Forward",
                        cx,
                    ))
                    .with_children(primary_left_items)
                    .with_children(primary_right_items)
                    .constrained()
                    .with_height(height),
            )
            .with_children(secondary_item)
            .contained()
            .with_style(container_style)
            .into_any_named("toolbar")
    }
}

#[allow(clippy::too_many_arguments)]
fn nav_button<A: Action, F: 'static + Fn(&mut Toolbar, &mut ViewContext<Toolbar>)>(
    svg_path: &'static str,
    style: theme::Interactive<theme::IconButton>,
    tooltip_style: TooltipStyle,
    enabled: bool,
    spacing: f32,
    on_click: F,
    tooltip_action: A,
    action_name: &str,
    cx: &mut ViewContext<Toolbar>,
) -> AnyElement<Toolbar> {
    MouseEventHandler::<A, _>::new(0, cx, |state, _| {
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
            .aligned()
    })
    .with_cursor_style(if enabled {
        CursorStyle::PointingHand
    } else {
        CursorStyle::default()
    })
    .on_click(MouseButton::Left, move |_, toolbar, cx| {
        on_click(toolbar, cx)
    })
    .with_tooltip::<A>(
        0,
        action_name.to_string(),
        Some(Box::new(tooltip_action)),
        tooltip_style,
        cx,
    )
    .contained()
    .with_margin_right(spacing)
    .into_any_named("nav button")
}

impl Toolbar {
    pub fn new(pane: WeakViewHandle<Pane>) -> Self {
        Self {
            active_pane_item: None,
            pane,
            items: Default::default(),
            hidden: false,
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
        self.hidden = self
            .active_pane_item
            .as_ref()
            .map(|item| !item.show_toolbar(cx))
            .unwrap_or(false);

        for (toolbar_item, current_location) in self.items.iter_mut() {
            let new_location = toolbar_item.set_active_pane_item(pane_item, cx);
            if new_location != *current_location {
                *current_location = new_location;
                cx.notify();
            }
        }
    }

    pub fn pane_focus_update(&mut self, pane_focused: bool, cx: &mut ViewContext<Self>) {
        for (toolbar_item, _) in self.items.iter_mut() {
            toolbar_item.pane_focus_update(pane_focused, cx);
        }
    }

    pub fn item_of_type<T: ToolbarItemView>(&self) -> Option<ViewHandle<T>> {
        self.items
            .iter()
            .find_map(|(item, _)| item.as_any().clone().downcast())
    }

    pub fn hidden(&self) -> bool {
        self.hidden
    }
}

impl<T: ToolbarItemView> ToolbarItemViewHandle for ViewHandle<T> {
    fn id(&self) -> usize {
        self.id()
    }

    fn as_any(&self) -> &AnyViewHandle {
        self
    }

    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut WindowContext,
    ) -> ToolbarItemLocation {
        self.update(cx, |this, cx| {
            this.set_active_pane_item(active_pane_item, cx)
        })
    }

    fn pane_focus_update(&mut self, pane_focused: bool, cx: &mut WindowContext) {
        self.update(cx, |this, cx| {
            this.pane_focus_update(pane_focused, cx);
            cx.notify();
        });
    }
}

impl From<&dyn ToolbarItemViewHandle> for AnyViewHandle {
    fn from(val: &dyn ToolbarItemViewHandle) -> Self {
        val.as_any().clone()
    }
}
