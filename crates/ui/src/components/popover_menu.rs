use std::{cell::RefCell, rc::Rc};

use gpui::{
    anchored, deferred, div, point, prelude::FluentBuilder, px, AnchorCorner, AnyElement, Bounds,
    DismissEvent, DispatchPhase, Element, ElementContext, ElementId, HitboxId, InteractiveElement,
    IntoElement, LayoutId, ManagedView, MouseDownEvent, ParentElement, Pixels, Point, View,
    VisualContext, WindowContext,
};

use crate::prelude::*;

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
            let offset = rems_from_px(5.) * cx.rem_size();
            match self.anchor {
                AnchorCorner::TopRight | AnchorCorner::BottomRight => point(offset, px(0.)),
                AnchorCorner::TopLeft | AnchorCorner::BottomLeft => point(-offset, px(0.)),
            }
        })
    }

    fn with_element_state<R>(
        &mut self,
        cx: &mut ElementContext,
        f: impl FnOnce(&mut Self, &mut PopoverMenuElementState<M>, &mut ElementContext) -> R,
    ) -> R {
        cx.with_element_state::<PopoverMenuElementState<M>, _>(
            Some(self.id.clone()),
            |element_state, cx| {
                let mut element_state = element_state.unwrap().unwrap_or_default();
                let result = f(self, &mut element_state, cx);
                (result, Some(element_state))
            },
        )
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

pub struct PopoverMenuElementState<M> {
    menu: Rc<RefCell<Option<View<M>>>>,
    child_bounds: Option<Bounds<Pixels>>,
}

impl<M> Clone for PopoverMenuElementState<M> {
    fn clone(&self) -> Self {
        Self {
            menu: Rc::clone(&self.menu),
            child_bounds: self.child_bounds,
        }
    }
}

impl<M> Default for PopoverMenuElementState<M> {
    fn default() -> Self {
        Self {
            menu: Rc::default(),
            child_bounds: None,
        }
    }
}

pub struct PopoverMenuFrameState {
    child_layout_id: Option<LayoutId>,
    child_element: Option<AnyElement>,
    menu_element: Option<AnyElement>,
}

impl<M: ManagedView> Element for PopoverMenu<M> {
    type BeforeLayout = PopoverMenuFrameState;
    type AfterLayout = Option<HitboxId>;

    fn before_layout(&mut self, cx: &mut ElementContext) -> (gpui::LayoutId, Self::BeforeLayout) {
        self.with_element_state(cx, |this, element_state, cx| {
            let mut menu_layout_id = None;

            let menu_element = element_state.menu.borrow_mut().as_mut().map(|menu| {
                let mut anchored = anchored().snap_to_window().anchor(this.anchor);
                if let Some(child_bounds) = element_state.child_bounds {
                    anchored = anchored.position(
                        this.resolved_attach().corner(child_bounds) + this.resolved_offset(cx),
                    );
                }
                let mut element =
                    deferred(anchored.child(div().occlude().child(menu.clone()))).into_any();

                menu_layout_id = Some(element.before_layout(cx));
                element
            });

            let mut child_element = this.child_builder.take().map(|child_builder| {
                (child_builder)(element_state.menu.clone(), this.menu_builder.clone())
            });

            let child_layout_id = child_element
                .as_mut()
                .map(|child_element| child_element.before_layout(cx));

            let layout_id = cx.request_layout(
                &gpui::Style::default(),
                menu_layout_id.into_iter().chain(child_layout_id),
            );

            (
                layout_id,
                PopoverMenuFrameState {
                    child_element,
                    child_layout_id,
                    menu_element,
                },
            )
        })
    }

    fn after_layout(
        &mut self,
        _bounds: Bounds<Pixels>,
        before_layout: &mut Self::BeforeLayout,
        cx: &mut ElementContext,
    ) -> Option<HitboxId> {
        self.with_element_state(cx, |_this, element_state, cx| {
            if let Some(child) = before_layout.child_element.as_mut() {
                child.after_layout(cx);
            }

            if let Some(menu) = before_layout.menu_element.as_mut() {
                menu.after_layout(cx);
            }

            before_layout.child_layout_id.map(|layout_id| {
                let bounds = cx.layout_bounds(layout_id);
                element_state.child_bounds = Some(bounds);
                cx.insert_hitbox(bounds, false).id
            })
        })
    }

    fn paint(
        &mut self,
        _: Bounds<gpui::Pixels>,
        before_layout: &mut Self::BeforeLayout,
        child_hitbox: &mut Option<HitboxId>,
        cx: &mut ElementContext,
    ) {
        self.with_element_state(cx, |_this, _element_state, cx| {
            if let Some(mut child) = before_layout.child_element.take() {
                child.paint(cx);
            }

            if let Some(mut menu) = before_layout.menu_element.take() {
                menu.paint(cx);

                if let Some(child_hitbox) = *child_hitbox {
                    // Mouse-downing outside the menu dismisses it, so we don't
                    // want a click on the toggle to re-open it.
                    cx.on_mouse_event(move |_: &MouseDownEvent, phase, cx| {
                        if phase == DispatchPhase::Bubble && child_hitbox.is_hovered(cx) {
                            cx.stop_propagation()
                        }
                    })
                }
            }
        })
    }
}

impl<M: ManagedView> IntoElement for PopoverMenu<M> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
