use std::{cell::RefCell, rc::Rc};

use gpui::{
    AnyElement, App, Bounds, Corner, DismissEvent, DispatchPhase, Element, ElementId, Entity,
    Focusable as _, GlobalElementId, Hitbox, InteractiveElement, IntoElement, LayoutId,
    ManagedView, MouseButton, MouseDownEvent, ParentElement, Pixels, Point, Window, anchored,
    deferred, div, px,
};

pub struct RightClickMenu<M: ManagedView> {
    id: ElementId,
    child_builder: Option<Box<dyn FnOnce(bool) -> AnyElement + 'static>>,
    menu_builder: Option<Rc<dyn Fn(&mut Window, &mut App) -> Entity<M> + 'static>>,
    anchor: Option<Corner>,
    attach: Option<Corner>,
}

impl<M: ManagedView> RightClickMenu<M> {
    pub fn menu(mut self, f: impl Fn(&mut Window, &mut App) -> Entity<M> + 'static) -> Self {
        self.menu_builder = Some(Rc::new(f));
        self
    }

    pub fn trigger<F, E>(mut self, e: F) -> Self
    where
        F: FnOnce(bool) -> E + 'static,
        E: IntoElement + 'static,
    {
        self.child_builder = Some(Box::new(move |is_menu_active| {
            e(is_menu_active).into_any_element()
        }));
        self
    }

    /// anchor defines which corner of the menu to anchor to the attachment point
    /// (by default the cursor position, but see attach)
    pub fn anchor(mut self, anchor: Corner) -> Self {
        self.anchor = Some(anchor);
        self
    }

    /// attach defines which corner of the handle to attach the menu's anchor to
    pub fn attach(mut self, attach: Corner) -> Self {
        self.attach = Some(attach);
        self
    }

    fn with_element_state<R>(
        &mut self,
        global_id: &GlobalElementId,
        window: &mut Window,
        cx: &mut App,
        f: impl FnOnce(&mut Self, &mut MenuHandleElementState<M>, &mut Window, &mut App) -> R,
    ) -> R {
        window.with_optional_element_state::<MenuHandleElementState<M>, _>(
            Some(global_id),
            |element_state, window| {
                let mut element_state = element_state.unwrap().unwrap_or_default();
                let result = f(self, &mut element_state, window, cx);
                (result, Some(element_state))
            },
        )
    }
}

/// Creates a [`RightClickMenu`]
pub fn right_click_menu<M: ManagedView>(id: impl Into<ElementId>) -> RightClickMenu<M> {
    RightClickMenu {
        id: id.into(),
        child_builder: None,
        menu_builder: None,
        anchor: None,
        attach: None,
    }
}

pub struct MenuHandleElementState<M> {
    menu: Rc<RefCell<Option<Entity<M>>>>,
    position: Rc<RefCell<Point<Pixels>>>,
}

impl<M> Clone for MenuHandleElementState<M> {
    fn clone(&self) -> Self {
        Self {
            menu: Rc::clone(&self.menu),
            position: Rc::clone(&self.position),
        }
    }
}

impl<M> Default for MenuHandleElementState<M> {
    fn default() -> Self {
        Self {
            menu: Rc::default(),
            position: Rc::default(),
        }
    }
}

pub struct RequestLayoutState {
    child_layout_id: Option<LayoutId>,
    child_element: Option<AnyElement>,
    menu_element: Option<AnyElement>,
}

pub struct PrepaintState {
    hitbox: Hitbox,
    child_bounds: Option<Bounds<Pixels>>,
}

impl<M: ManagedView> Element for RightClickMenu<M> {
    type RequestLayoutState = RequestLayoutState;
    type PrepaintState = PrepaintState;
    type DebugState = ();

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        _debug_state: &mut Option<Self::DebugState>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        self.with_element_state(
            id.unwrap(),
            window,
            cx,
            |this, element_state, window, cx| {
                let mut menu_layout_id = None;

                let menu_element = element_state.menu.borrow_mut().as_mut().map(|menu| {
                    let mut anchored = anchored().snap_to_window_with_margin(px(8.));
                    if let Some(anchor) = this.anchor {
                        anchored = anchored.anchor(anchor);
                    }
                    anchored = anchored.position(*element_state.position.borrow());

                    let mut element = deferred(anchored.child(div().occlude().child(menu.clone())))
                        .with_priority(1)
                        .into_any();

                    menu_layout_id = Some(element.request_layout(window, cx));
                    element
                });

                let mut child_element = this
                    .child_builder
                    .take()
                    .map(|child_builder| (child_builder)(element_state.menu.borrow().is_some()));

                let child_layout_id = child_element
                    .as_mut()
                    .map(|child_element| child_element.request_layout(window, cx));

                let layout_id = window.request_layout(
                    gpui::Style::default(),
                    menu_layout_id.into_iter().chain(child_layout_id),
                    cx,
                );

                (
                    layout_id,
                    RequestLayoutState {
                        child_element,
                        child_layout_id,
                        menu_element,
                    },
                )
            },
        )
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        _debug_state: &mut Option<Self::DebugState>,
        window: &mut Window,
        cx: &mut App,
    ) -> PrepaintState {
        let hitbox = window.insert_hitbox(bounds, false);

        if let Some(child) = request_layout.child_element.as_mut() {
            child.prepaint(window, cx);
        }

        if let Some(menu) = request_layout.menu_element.as_mut() {
            menu.prepaint(window, cx);
        }

        PrepaintState {
            hitbox,
            child_bounds: request_layout
                .child_layout_id
                .map(|layout_id| window.layout_bounds(layout_id)),
        }
    }

    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        _bounds: Bounds<gpui::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint_state: &mut Self::PrepaintState,
        _debug_state: &mut Option<Self::DebugState>,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.with_element_state(
            id.unwrap(),
            window,
            cx,
            |this, element_state, window, cx| {
                if let Some(mut child) = request_layout.child_element.take() {
                    child.paint(window, cx);
                }

                if let Some(mut menu) = request_layout.menu_element.take() {
                    menu.paint(window, cx);
                    return;
                }

                let Some(builder) = this.menu_builder.take() else {
                    return;
                };

                let attach = this.attach;
                let menu = element_state.menu.clone();
                let position = element_state.position.clone();
                let child_bounds = prepaint_state.child_bounds;

                let hitbox_id = prepaint_state.hitbox.id;
                window.on_mouse_event(move |event: &MouseDownEvent, phase, window, cx| {
                    if phase == DispatchPhase::Bubble
                        && event.button == MouseButton::Right
                        && hitbox_id.is_hovered(window)
                    {
                        cx.stop_propagation();
                        window.prevent_default();

                        let new_menu = (builder)(window, cx);
                        let menu2 = menu.clone();
                        let previous_focus_handle = window.focused(cx);

                        window
                            .subscribe(&new_menu, cx, move |modal, _: &DismissEvent, window, cx| {
                                if modal.focus_handle(cx).contains_focused(window, cx) {
                                    if let Some(previous_focus_handle) =
                                        previous_focus_handle.as_ref()
                                    {
                                        window.focus(previous_focus_handle);
                                    }
                                }
                                *menu2.borrow_mut() = None;
                                window.refresh();
                            })
                            .detach();
                        window.focus(&new_menu.focus_handle(cx));
                        *menu.borrow_mut() = Some(new_menu);
                        *position.borrow_mut() = if let Some(child_bounds) = child_bounds {
                            if let Some(attach) = attach {
                                child_bounds.corner(attach)
                            } else {
                                window.mouse_position()
                            }
                        } else {
                            window.mouse_position()
                        };
                        window.refresh();
                    }
                });
            },
        )
    }
}

impl<M: ManagedView> IntoElement for RightClickMenu<M> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
