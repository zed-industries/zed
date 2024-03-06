use std::{cell::RefCell, rc::Rc};

use gpui::{
    overlay, point, prelude::FluentBuilder, px, rems, AnchorCorner, AnyElement, Bounds,
    DismissEvent, DispatchPhase, Element, ElementContext, ElementId, InteractiveBounds,
    IntoElement, LayoutId, ManagedView, MouseDownEvent, ParentElement, Pixels, Point, View,
    VisualContext, WindowContext,
};

use crate::{Clickable, Selectable};

pub trait PopoverTrigger: IntoElement + Clickable + Selectable + 'static {}

impl<T: IntoElement + Clickable + Selectable + 'static> PopoverTrigger for T {}

pub struct PopoverMenu<M: ManagedView> {
    id: ElementId,
    child_builder: Option<
        Box<
            dyn FnOnce(
                    Rc<RefCell<Option<View<M>>>>,
                    Option<Rc<dyn Fn(&mut WindowContext) -> Option<View<M>> + 'static>>,
                ) -> AnyElement
                + 'static,
        >,
    >,
    menu_builder: Option<Rc<dyn Fn(&mut WindowContext) -> Option<View<M>> + 'static>>,
    anchor: AnchorCorner,
    attach: Option<AnchorCorner>,
    offset: Option<Point<Pixels>>,
}

impl<M: ManagedView> PopoverMenu<M> {
    pub fn menu(mut self, f: impl Fn(&mut WindowContext) -> Option<View<M>> + 'static) -> Self {
        self.menu_builder = Some(Rc::new(f));
        self
    }

    pub fn trigger<T: PopoverTrigger>(mut self, t: T) -> Self {
        self.child_builder = Some(Box::new(|menu, builder| {
            let open = menu.borrow().is_some();
            t.selected(open)
                .when_some(builder, |el, builder| {
                    el.on_click({
                        move |_, cx| {
                            let Some(new_menu) = (builder)(cx) else {
                                return;
                            };
                            let menu2 = menu.clone();
                            let previous_focus_handle = cx.focused();

                            cx.subscribe(&new_menu, move |modal, _: &DismissEvent, cx| {
                                if modal.focus_handle(cx).contains_focused(cx) {
                                    if let Some(previous_focus_handle) =
                                        previous_focus_handle.as_ref()
                                    {
                                        cx.focus(previous_focus_handle);
                                    }
                                }
                                *menu2.borrow_mut() = None;
                                cx.refresh();
                            })
                            .detach();
                            cx.focus_view(&new_menu);
                            *menu.borrow_mut() = Some(new_menu);
                        }
                    })
                })
                .into_any_element()
        }));
        self
    }

    /// anchor defines which corner of the menu to anchor to the attachment point
    /// (by default the cursor position, but see attach)
    pub fn anchor(mut self, anchor: AnchorCorner) -> Self {
        self.anchor = anchor;
        self
    }

    /// attach defines which corner of the handle to attach the menu's anchor to
    pub fn attach(mut self, attach: AnchorCorner) -> Self {
        self.attach = Some(attach);
        self
    }

    /// offset offsets the position of the content by that many pixels.
    pub fn offset(mut self, offset: Point<Pixels>) -> Self {
        self.offset = Some(offset);
        self
    }

    fn resolved_attach(&self) -> AnchorCorner {
        self.attach.unwrap_or_else(|| match self.anchor {
            AnchorCorner::TopLeft => AnchorCorner::BottomLeft,
            AnchorCorner::TopRight => AnchorCorner::BottomRight,
            AnchorCorner::BottomLeft => AnchorCorner::TopLeft,
            AnchorCorner::BottomRight => AnchorCorner::TopRight,
        })
    }

    fn resolved_offset(&self, cx: &WindowContext) -> Point<Pixels> {
        self.offset.unwrap_or_else(|| {
            // Default offset = 4px padding + 1px border
            let offset = rems(5. / 16.) * cx.rem_size();
            match self.anchor {
                AnchorCorner::TopRight | AnchorCorner::BottomRight => point(offset, px(0.)),
                AnchorCorner::TopLeft | AnchorCorner::BottomLeft => point(-offset, px(0.)),
            }
        })
    }
}

/// Creates a [`PopoverMenu`]
pub fn popover_menu<M: ManagedView>(id: impl Into<ElementId>) -> PopoverMenu<M> {
    PopoverMenu {
        id: id.into(),
        child_builder: None,
        menu_builder: None,
        anchor: AnchorCorner::TopLeft,
        attach: None,
        offset: None,
    }
}

pub struct PopoverMenuState<M> {
    child_layout_id: Option<LayoutId>,
    child_element: Option<AnyElement>,
    child_bounds: Option<Bounds<Pixels>>,
    menu_element: Option<AnyElement>,
    menu: Rc<RefCell<Option<View<M>>>>,
}

impl<M: ManagedView> Element for PopoverMenu<M> {
    type State = PopoverMenuState<M>;

    fn request_layout(
        &mut self,
        element_state: Option<Self::State>,
        cx: &mut ElementContext,
    ) -> (gpui::LayoutId, Self::State) {
        let mut menu_layout_id = None;

        let (menu, child_bounds) = if let Some(element_state) = element_state {
            (element_state.menu, element_state.child_bounds)
        } else {
            (Rc::default(), None)
        };

        let menu_element = menu.borrow_mut().as_mut().map(|menu| {
            let mut overlay = overlay().snap_to_window().anchor(self.anchor);

            if let Some(child_bounds) = child_bounds {
                overlay = overlay.position(
                    self.resolved_attach().corner(child_bounds) + self.resolved_offset(cx),
                );
            }

            let mut element = overlay.child(menu.clone()).into_any();
            menu_layout_id = Some(element.request_layout(cx));
            element
        });

        let mut child_element = self
            .child_builder
            .take()
            .map(|child_builder| (child_builder)(menu.clone(), self.menu_builder.clone()));

        let child_layout_id = child_element
            .as_mut()
            .map(|child_element| child_element.request_layout(cx));

        let layout_id = cx.request_layout(
            &gpui::Style::default(),
            menu_layout_id.into_iter().chain(child_layout_id),
        );

        (
            layout_id,
            PopoverMenuState {
                menu,
                child_element,
                child_layout_id,
                menu_element,
                child_bounds,
            },
        )
    }

    fn paint(
        &mut self,
        _: Bounds<gpui::Pixels>,
        element_state: &mut Self::State,
        cx: &mut ElementContext,
    ) {
        if let Some(mut child) = element_state.child_element.take() {
            child.paint(cx);
        }

        if let Some(child_layout_id) = element_state.child_layout_id.take() {
            element_state.child_bounds = Some(cx.layout_bounds(child_layout_id));
        }

        if let Some(mut menu) = element_state.menu_element.take() {
            menu.paint(cx);

            if let Some(child_bounds) = element_state.child_bounds {
                let interactive_bounds = InteractiveBounds {
                    bounds: child_bounds,
                    stacking_order: cx.stacking_order().clone(),
                };

                // Mouse-downing outside the menu dismisses it, so we don't
                // want a click on the toggle to re-open it.
                cx.on_mouse_event(move |e: &MouseDownEvent, phase, cx| {
                    if phase == DispatchPhase::Bubble
                        && interactive_bounds.visibly_contains(&e.position, cx)
                    {
                        cx.stop_propagation()
                    }
                })
            }
        }
    }
}

impl<M: ManagedView> IntoElement for PopoverMenu<M> {
    type Element = Self;

    fn element_id(&self) -> Option<gpui::ElementId> {
        Some(self.id.clone())
    }

    fn into_element(self) -> Self::Element {
        self
    }
}
