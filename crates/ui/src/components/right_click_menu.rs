#![allow(missing_docs)]

use std::{cell::RefCell, rc::Rc};

use gpui::{
    anchored, deferred, div, px, AnchorCorner, AnyElement, Bounds, DismissEvent, DispatchPhase,
    Element, ElementId, GlobalElementId, Hitbox, InteractiveElement, IntoElement, LayoutId,
    ManagedView, MouseButton, MouseDownEvent, ParentElement, Pixels, Point, View, VisualContext,
    WindowContext,
};

pub struct RightClickMenu<M: ManagedView> {
    id: ElementId,
    child_builder: Option<Box<dyn FnOnce(bool) -> AnyElement + 'static>>,
    menu_builder: Option<Rc<dyn Fn(&mut WindowContext) -> View<M> + 'static>>,
    anchor: Option<AnchorCorner>,
    attach: Option<AnchorCorner>,
}

impl<M: ManagedView> RightClickMenu<M> {
    pub fn menu(mut self, f: impl Fn(&mut WindowContext) -> View<M> + 'static) -> Self {
        self.menu_builder = Some(Rc::new(f));
        self
    }

    pub fn trigger<E: IntoElement + 'static>(mut self, e: E) -> Self {
        self.child_builder = Some(Box::new(move |_| e.into_any_element()));
        self
    }

    /// anchor defines which corner of the menu to anchor to the attachment point
    /// (by default the cursor position, but see attach)
    pub fn anchor(mut self, anchor: AnchorCorner) -> Self {
        self.anchor = Some(anchor);
        self
    }

    /// attach defines which corner of the handle to attach the menu's anchor to
    pub fn attach(mut self, attach: AnchorCorner) -> Self {
        self.attach = Some(attach);
        self
    }

    fn with_element_state<R>(
        &mut self,
        global_id: &GlobalElementId,
        cx: &mut WindowContext,
        f: impl FnOnce(&mut Self, &mut MenuHandleElementState<M>, &mut WindowContext) -> R,
    ) -> R {
        cx.with_optional_element_state::<MenuHandleElementState<M>, _>(
            Some(global_id),
            |element_state, cx| {
                let mut element_state = element_state.unwrap().unwrap_or_default();
                let result = f(self, &mut element_state, cx);
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
    menu: Rc<RefCell<Option<View<M>>>>,
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

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        self.with_element_state(id.unwrap(), cx, |this, element_state, cx| {
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

                menu_layout_id = Some(element.request_layout(cx));
                element
            });

            let mut child_element = this
                .child_builder
                .take()
                .map(|child_builder| (child_builder)(element_state.menu.borrow().is_some()));

            let child_layout_id = child_element
                .as_mut()
                .map(|child_element| child_element.request_layout(cx));

            let layout_id = cx.request_layout(
                gpui::Style::default(),
                menu_layout_id.into_iter().chain(child_layout_id),
            );

            (
                layout_id,
                RequestLayoutState {
                    child_element,
                    child_layout_id,
                    menu_element,
                },
            )
        })
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> PrepaintState {
        let hitbox = cx.insert_hitbox(bounds, false);

        if let Some(child) = request_layout.child_element.as_mut() {
            child.prepaint(cx);
        }

        if let Some(menu) = request_layout.menu_element.as_mut() {
            menu.prepaint(cx);
        }

        PrepaintState {
            hitbox,
            child_bounds: request_layout
                .child_layout_id
                .map(|layout_id| cx.layout_bounds(layout_id)),
        }
    }

    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        _bounds: Bounds<gpui::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint_state: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        self.with_element_state(id.unwrap(), cx, |this, element_state, cx| {
            if let Some(mut child) = request_layout.child_element.take() {
                child.paint(cx);
            }

            if let Some(mut menu) = request_layout.menu_element.take() {
                menu.paint(cx);
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
            cx.on_mouse_event(move |event: &MouseDownEvent, phase, cx| {
                if phase == DispatchPhase::Bubble
                    && event.button == MouseButton::Right
                    && hitbox_id.is_hovered(cx)
                {
                    cx.stop_propagation();
                    cx.prevent_default();

                    let new_menu = (builder)(cx);
                    let menu2 = menu.clone();
                    let previous_focus_handle = cx.focused();

                    cx.subscribe(&new_menu, move |modal, _: &DismissEvent, cx| {
                        if modal.focus_handle(cx).contains_focused(cx) {
                            if let Some(previous_focus_handle) = previous_focus_handle.as_ref() {
                                cx.focus(previous_focus_handle);
                            }
                        }
                        *menu2.borrow_mut() = None;
                        cx.refresh();
                    })
                    .detach();
                    cx.focus_view(&new_menu);
                    *menu.borrow_mut() = Some(new_menu);
                    *position.borrow_mut() = if let Some(child_bounds) = child_bounds {
                        if let Some(attach) = attach {
                            attach.corner(child_bounds)
                        } else {
                            cx.mouse_position()
                        }
                    } else {
                        cx.mouse_position()
                    };
                    cx.refresh();
                }
            });
        })
    }
}

impl<M: ManagedView> IntoElement for RightClickMenu<M> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
