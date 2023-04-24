use gpui::{
    elements::*,
    geometry::vector::Vector2F,
    impl_internal_actions,
    keymap_matcher::KeymapContext,
    platform::{CursorStyle, MouseButton},
    Action, AnyViewHandle, AppContext, Axis, Entity, MouseState, SizeConstraint, Subscription,
    View, ViewContext,
};
use menu::*;
use settings::Settings;
use std::{any::TypeId, borrow::Cow, time::Duration};

#[derive(Copy, Clone, PartialEq)]
struct Clicked;

impl_internal_actions!(context_menu, [Clicked]);

pub fn init(cx: &mut AppContext) {
    cx.add_action(ContextMenu::select_first);
    cx.add_action(ContextMenu::select_last);
    cx.add_action(ContextMenu::select_next);
    cx.add_action(ContextMenu::select_prev);
    cx.add_action(ContextMenu::clicked);
    cx.add_action(ContextMenu::confirm);
    cx.add_action(ContextMenu::cancel);
}

pub type StaticItem = Box<dyn Fn(&mut AppContext) -> AnyElement<ContextMenu>>;

type ContextMenuItemBuilder =
    Box<dyn Fn(&mut MouseState, &theme::ContextMenuItem) -> AnyElement<ContextMenu>>;

pub enum ContextMenuItemLabel {
    String(Cow<'static, str>),
    Element(ContextMenuItemBuilder),
}

pub enum ContextMenuAction {
    ParentAction {
        action: Box<dyn Action>,
    },
    ViewAction {
        action: Box<dyn Action>,
        for_view: usize,
    },
}

impl ContextMenuAction {
    fn id(&self) -> TypeId {
        match self {
            ContextMenuAction::ParentAction { action } => action.id(),
            ContextMenuAction::ViewAction { action, .. } => action.id(),
        }
    }
}

pub enum ContextMenuItem {
    Item {
        label: ContextMenuItemLabel,
        action: ContextMenuAction,
    },
    Static(StaticItem),
    Separator,
}

impl ContextMenuItem {
    pub fn element_item(label: ContextMenuItemBuilder, action: impl 'static + Action) -> Self {
        Self::Item {
            label: ContextMenuItemLabel::Element(label),
            action: ContextMenuAction::ParentAction {
                action: Box::new(action),
            },
        }
    }

    pub fn item(label: impl Into<Cow<'static, str>>, action: impl 'static + Action) -> Self {
        Self::Item {
            label: ContextMenuItemLabel::String(label.into()),
            action: ContextMenuAction::ParentAction {
                action: Box::new(action),
            },
        }
    }

    pub fn item_for_view(
        label: impl Into<Cow<'static, str>>,
        view_id: usize,
        action: impl 'static + Action,
    ) -> Self {
        Self::Item {
            label: ContextMenuItemLabel::String(label.into()),
            action: ContextMenuAction::ViewAction {
                action: Box::new(action),
                for_view: view_id,
            },
        }
    }

    pub fn separator() -> Self {
        Self::Separator
    }

    fn is_action(&self) -> bool {
        matches!(self, Self::Item { .. })
    }

    fn action_id(&self) -> Option<TypeId> {
        match self {
            ContextMenuItem::Item { action, .. } => Some(action.id()),
            ContextMenuItem::Static(..) | ContextMenuItem::Separator => None,
        }
    }
}

pub struct ContextMenu {
    show_count: usize,
    anchor_position: Vector2F,
    anchor_corner: AnchorCorner,
    position_mode: OverlayPositionMode,
    items: Vec<ContextMenuItem>,
    selected_index: Option<usize>,
    visible: bool,
    previously_focused_view_id: Option<usize>,
    clicked: bool,
    parent_view_id: usize,
    _actions_observation: Subscription,
}

impl Entity for ContextMenu {
    type Event = ();
}

impl View for ContextMenu {
    fn ui_name() -> &'static str {
        "ContextMenu"
    }

    fn keymap_context(&self, _: &AppContext) -> KeymapContext {
        let mut cx = Self::default_keymap_context();
        cx.add_identifier("menu");
        cx
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        if !self.visible {
            return Empty::new().into_any();
        }

        // Render the menu once at minimum width.
        let mut collapsed_menu = self.render_menu_for_measurement(cx);
        let expanded_menu =
            self.render_menu(cx)
                .constrained()
                .dynamically(move |constraint, view, cx| {
                    SizeConstraint::strict_along(
                        Axis::Horizontal,
                        collapsed_menu.layout(constraint, view, cx).0.x(),
                    )
                });

        Overlay::new(expanded_menu)
            .with_hoverable(true)
            .with_fit_mode(OverlayFitMode::SnapToWindow)
            .with_anchor_position(self.anchor_position)
            .with_anchor_corner(self.anchor_corner)
            .with_position_mode(self.position_mode)
            .into_any()
    }

    fn focus_out(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        self.reset(cx);
    }
}

impl ContextMenu {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let parent_view_id = cx.parent().unwrap();

        Self {
            show_count: 0,
            anchor_position: Default::default(),
            anchor_corner: AnchorCorner::TopLeft,
            position_mode: OverlayPositionMode::Window,
            items: Default::default(),
            selected_index: Default::default(),
            visible: Default::default(),
            previously_focused_view_id: Default::default(),
            clicked: false,
            parent_view_id,
            _actions_observation: cx.observe_actions(Self::action_dispatched),
        }
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    fn action_dispatched(&mut self, action_id: TypeId, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self
            .items
            .iter()
            .position(|item| item.action_id() == Some(action_id))
        {
            if self.clicked {
                self.cancel(&Default::default(), cx);
            } else {
                self.selected_index = Some(ix);
                cx.notify();
                cx.spawn(|this, mut cx| async move {
                    cx.background().timer(Duration::from_millis(50)).await;
                    this.update(&mut cx, |this, cx| this.cancel(&Default::default(), cx))
                })
                .detach_and_log_err(cx);
            }
        }
    }

    fn clicked(&mut self, _: &Clicked, _: &mut ViewContext<Self>) {
        self.clicked = true;
    }

    fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.selected_index {
            if let Some(ContextMenuItem::Item { action, .. }) = self.items.get(ix) {
                match action {
                    ContextMenuAction::ParentAction { action } => {
                        cx.dispatch_any_action(action.boxed_clone())
                    }
                    ContextMenuAction::ViewAction { action, for_view } => {
                        let window_id = cx.window_id();
                        cx.dispatch_any_action_at(window_id, *for_view, action.boxed_clone())
                    }
                };
                self.reset(cx);
            }
        }
    }

    fn cancel(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        self.reset(cx);
        let show_count = self.show_count;
        cx.defer(move |this, cx| {
            if cx.handle().is_focused(cx) && this.show_count == show_count {
                let window_id = cx.window_id();
                (**cx).focus(window_id, this.previously_focused_view_id.take());
            }
        });
    }

    fn reset(&mut self, cx: &mut ViewContext<Self>) {
        self.items.clear();
        self.visible = false;
        self.selected_index.take();
        self.clicked = false;
        cx.notify();
    }

    fn select_first(&mut self, _: &SelectFirst, cx: &mut ViewContext<Self>) {
        self.selected_index = self.items.iter().position(|item| item.is_action());
        cx.notify();
    }

    fn select_last(&mut self, _: &SelectLast, cx: &mut ViewContext<Self>) {
        for (ix, item) in self.items.iter().enumerate().rev() {
            if item.is_action() {
                self.selected_index = Some(ix);
                cx.notify();
                break;
            }
        }
    }

    fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.selected_index {
            for (ix, item) in self.items.iter().enumerate().skip(ix + 1) {
                if item.is_action() {
                    self.selected_index = Some(ix);
                    cx.notify();
                    break;
                }
            }
        } else {
            self.select_first(&Default::default(), cx);
        }
    }

    fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.selected_index {
            for (ix, item) in self.items.iter().enumerate().take(ix).rev() {
                if item.is_action() {
                    self.selected_index = Some(ix);
                    cx.notify();
                    break;
                }
            }
        } else {
            self.select_last(&Default::default(), cx);
        }
    }

    pub fn show(
        &mut self,
        anchor_position: Vector2F,
        anchor_corner: AnchorCorner,
        items: Vec<ContextMenuItem>,
        cx: &mut ViewContext<Self>,
    ) {
        let mut items = items.into_iter().peekable();
        if items.peek().is_some() {
            self.items = items.collect();
            self.anchor_position = anchor_position;
            self.anchor_corner = anchor_corner;
            self.visible = true;
            self.show_count += 1;
            if !cx.is_self_focused() {
                self.previously_focused_view_id = cx.focused_view_id();
            }
            cx.focus_self();
        } else {
            self.visible = false;
        }
        cx.notify();
    }

    pub fn set_position_mode(&mut self, mode: OverlayPositionMode) {
        self.position_mode = mode;
    }

    fn render_menu_for_measurement(&self, cx: &mut ViewContext<Self>) -> impl Element<ContextMenu> {
        let style = cx.global::<Settings>().theme.context_menu.clone();
        Flex::row()
            .with_child(
                Flex::column().with_children(self.items.iter().enumerate().map(|(ix, item)| {
                    match item {
                        ContextMenuItem::Item { label, .. } => {
                            let style = style.item.style_for(
                                &mut Default::default(),
                                Some(ix) == self.selected_index,
                            );

                            match label {
                                ContextMenuItemLabel::String(label) => {
                                    Label::new(label.to_string(), style.label.clone())
                                        .contained()
                                        .with_style(style.container)
                                        .into_any()
                                }
                                ContextMenuItemLabel::Element(element) => {
                                    element(&mut Default::default(), style)
                                }
                            }
                        }

                        ContextMenuItem::Static(f) => f(cx),

                        ContextMenuItem::Separator => Empty::new()
                            .collapsed()
                            .contained()
                            .with_style(style.separator)
                            .constrained()
                            .with_height(1.)
                            .into_any(),
                    }
                })),
            )
            .with_child(
                Flex::column()
                    .with_children(self.items.iter().enumerate().map(|(ix, item)| {
                        match item {
                            ContextMenuItem::Item { action, .. } => {
                                let style = style.item.style_for(
                                    &mut Default::default(),
                                    Some(ix) == self.selected_index,
                                );
                                let (action, view_id) = match action {
                                    ContextMenuAction::ParentAction { action } => {
                                        (action.boxed_clone(), self.parent_view_id)
                                    }
                                    ContextMenuAction::ViewAction { action, for_view } => {
                                        (action.boxed_clone(), *for_view)
                                    }
                                };

                                KeystrokeLabel::new(
                                    view_id,
                                    action.boxed_clone(),
                                    style.keystroke.container,
                                    style.keystroke.text.clone(),
                                )
                                .into_any()
                            }

                            ContextMenuItem::Static(_) => Empty::new().into_any(),

                            ContextMenuItem::Separator => Empty::new()
                                .collapsed()
                                .constrained()
                                .with_height(1.)
                                .contained()
                                .with_style(style.separator)
                                .into_any(),
                        }
                    }))
                    .contained()
                    .with_margin_left(style.keystroke_margin),
            )
            .contained()
            .with_style(style.container)
    }

    fn render_menu(&self, cx: &mut ViewContext<Self>) -> impl Element<ContextMenu> {
        enum Menu {}
        enum MenuItem {}

        let style = cx.global::<Settings>().theme.context_menu.clone();

        MouseEventHandler::<Menu, ContextMenu>::new(0, cx, |_, cx| {
            Flex::column()
                .with_children(self.items.iter().enumerate().map(|(ix, item)| {
                    match item {
                        ContextMenuItem::Item { label, action } => {
                            let (action, view_id) = match action {
                                ContextMenuAction::ParentAction { action } => {
                                    (action.boxed_clone(), self.parent_view_id)
                                }
                                ContextMenuAction::ViewAction { action, for_view } => {
                                    (action.boxed_clone(), *for_view)
                                }
                            };

                            MouseEventHandler::<MenuItem, ContextMenu>::new(ix, cx, |state, _| {
                                let style =
                                    style.item.style_for(state, Some(ix) == self.selected_index);

                                Flex::row()
                                    .with_child(match label {
                                        ContextMenuItemLabel::String(label) => {
                                            Label::new(label.clone(), style.label.clone())
                                                .contained()
                                                .into_any()
                                        }
                                        ContextMenuItemLabel::Element(element) => {
                                            element(state, style)
                                        }
                                    })
                                    .with_child({
                                        KeystrokeLabel::new(
                                            view_id,
                                            action.boxed_clone(),
                                            style.keystroke.container,
                                            style.keystroke.text.clone(),
                                        )
                                        .flex_float()
                                    })
                                    .contained()
                                    .with_style(style.container)
                            })
                            .with_cursor_style(CursorStyle::PointingHand)
                            .on_up(MouseButton::Left, |_, _, _| {}) // Capture these events
                            .on_down(MouseButton::Left, |_, _, _| {}) // Capture these events
                            .on_click(MouseButton::Left, move |_, _, cx| {
                                cx.dispatch_action(Clicked);
                                let window_id = cx.window_id();
                                cx.dispatch_any_action_at(window_id, view_id, action.boxed_clone());
                            })
                            .on_drag(MouseButton::Left, |_, _, _| {})
                            .into_any()
                        }

                        ContextMenuItem::Static(f) => f(cx),

                        ContextMenuItem::Separator => Empty::new()
                            .constrained()
                            .with_height(1.)
                            .contained()
                            .with_style(style.separator)
                            .into_any(),
                    }
                }))
                .contained()
                .with_style(style.container)
        })
        .on_down_out(MouseButton::Left, |_, _, cx| cx.dispatch_action(Cancel))
        .on_down_out(MouseButton::Right, |_, _, cx| cx.dispatch_action(Cancel))
    }
}
