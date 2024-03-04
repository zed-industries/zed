use std::{cell::RefCell, rc::Rc};

use gpui::{
    overlay, AnchorCorner, AnyElement, Bounds, DismissEvent, DispatchPhase, Element,
    ElementContext, ElementId, InteractiveBounds, IntoElement, LayoutId, ManagedView, MouseButton,
    MouseDownEvent, ParentElement, Pixels, Point, View, VisualContext, WindowContext,
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

pub struct MenuHandleState<M> {
    menu: Rc<RefCell<Option<View<M>>>>,
    position: Rc<RefCell<Point<Pixels>>>,
    child_layout_id: Option<LayoutId>,
    child_element: Option<AnyElement>,
    menu_element: Option<AnyElement>,
}

impl<M: ManagedView> Element for RightClickMenu<M> {
    type State = MenuHandleState<M>;

    fn request_layout(
        &mut self,
        element_state: Option<Self::State>,
        cx: &mut ElementContext,
    ) -> (gpui::LayoutId, Self::State) {
        let (menu, position) = if let Some(element_state) = element_state {
            (element_state.menu, element_state.position)
        } else {
            (Rc::default(), Rc::default())
        };

        let mut menu_layout_id = None;

        let menu_element = menu.borrow_mut().as_mut().map(|menu| {
            let mut overlay = overlay().snap_to_window();
            if let Some(anchor) = self.anchor {
                overlay = overlay.anchor(anchor);
            }
            overlay = overlay.position(*position.borrow());

            let mut element = overlay.child(menu.clone()).into_any();
            menu_layout_id = Some(element.request_layout(cx));
            element
        });

        let mut child_element = self
            .child_builder
            .take()
            .map(|child_builder| (child_builder)(menu.borrow().is_some()));

        let child_layout_id = child_element
            .as_mut()
            .map(|child_element| child_element.request_layout(cx));

        let layout_id = cx.request_layout(
            &gpui::Style::default(),
            menu_layout_id.into_iter().chain(child_layout_id),
        );

        (
            layout_id,
            MenuHandleState {
                menu,
                position,
                child_element,
                child_layout_id,
                menu_element,
            },
        )
    }

    fn paint(
        &mut self,
        bounds: Bounds<gpui::Pixels>,
        element_state: &mut Self::State,
        cx: &mut ElementContext,
    ) {
        if let Some(mut child) = element_state.child_element.take() {
            child.paint(cx);
        }

        if let Some(mut menu) = element_state.menu_element.take() {
            menu.paint(cx);
            return;
        }

        let Some(builder) = self.menu_builder.take() else {
            return;
        };
        let menu = element_state.menu.clone();
        let position = element_state.position.clone();
        let attach = self.attach;
        let child_layout_id = element_state.child_layout_id;
        let child_bounds = cx.layout_bounds(child_layout_id.unwrap());

        let interactive_bounds = InteractiveBounds {
            bounds: bounds.intersect(&cx.content_mask().bounds),
            stacking_order: cx.stacking_order().clone(),
        };
        cx.on_mouse_event(move |event: &MouseDownEvent, phase, cx| {
            if phase == DispatchPhase::Bubble
                && event.button == MouseButton::Right
                && interactive_bounds.visibly_contains(&event.position, cx)
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

                *position.borrow_mut() =
                    if let Some(attach) = attach.filter(|_| child_layout_id.is_some()) {
                        attach.corner(child_bounds)
                    } else {
                        cx.mouse_position()
                    };
                cx.refresh();
            }
        });
    }
}

impl<M: ManagedView> IntoElement for RightClickMenu<M> {
    type Element = Self;

    fn element_id(&self) -> Option<gpui::ElementId> {
        Some(self.id.clone())
    }

    fn into_element(self) -> Self::Element {
        self
    }
}
