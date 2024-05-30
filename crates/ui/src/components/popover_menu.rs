use std::{cell::RefCell, rc::Rc};

use gpui::{
    anchored, deferred, div, point, prelude::FluentBuilder, px, AnchorCorner, AnyElement, Bounds,
    DismissEvent, DispatchPhase, Element, ElementId, GlobalElementId, HitboxId, InteractiveElement,
    IntoElement, LayoutId, ManagedView, MouseDownEvent, ParentElement, Pixels, Point, View,
    VisualContext, WindowContext,
};

use crate::prelude::*;

pub trait PopoverTrigger: IntoElement + Clickable + Selectable + 'static {}

impl<T: IntoElement + Clickable + Selectable + 'static> PopoverTrigger for T {}

pub struct PopoverMenuHandle<M>(Rc<RefCell<Option<PopoverMenuHandleState<M>>>>);

impl<M> Clone for PopoverMenuHandle<M> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<M> Default for PopoverMenuHandle<M> {
    fn default() -> Self {
        Self(Rc::default())
    }
}

struct PopoverMenuHandleState<M> {
    menu_builder: Rc<dyn Fn(&mut WindowContext) -> Option<View<M>>>,
    menu: Rc<RefCell<Option<View<M>>>>,
}

impl<M: ManagedView> PopoverMenuHandle<M> {
    pub fn show(&self, cx: &mut WindowContext) {
        if let Some(state) = self.0.borrow().as_ref() {
            show_menu(&state.menu_builder, &state.menu, cx);
        }
    }

    pub fn hide(&self, cx: &mut WindowContext) {
        if let Some(state) = self.0.borrow().as_ref() {
            if let Some(menu) = state.menu.borrow().as_ref() {
                menu.update(cx, |_, cx| cx.emit(DismissEvent));
            }
        }
    }

    pub fn toggle(&self, cx: &mut WindowContext) {
        if let Some(state) = self.0.borrow().as_ref() {
            if state.menu.borrow().is_some() {
                self.hide(cx);
            } else {
                self.show(cx);
            }
        }
    }
}

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
    trigger_handle: Option<PopoverMenuHandle<M>>,
}

impl<M: ManagedView> PopoverMenu<M> {
    pub fn menu(mut self, f: impl Fn(&mut WindowContext) -> Option<View<M>> + 'static) -> Self {
        self.menu_builder = Some(Rc::new(f));
        self
    }

    pub fn with_handle(mut self, handle: PopoverMenuHandle<M>) -> Self {
        self.trigger_handle = Some(handle);
        self
    }

    pub fn trigger<T: PopoverTrigger>(mut self, t: T) -> Self {
        self.child_builder = Some(Box::new(|menu, builder| {
            let open = menu.borrow().is_some();
            t.selected(open)
                .when_some(builder, |el, builder| {
                    el.on_click(move |_, cx| show_menu(&builder, &menu, cx))
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
}

fn show_menu<M: ManagedView>(
    builder: &Rc<dyn Fn(&mut WindowContext) -> Option<View<M>>>,
    menu: &Rc<RefCell<Option<View<M>>>>,
    cx: &mut WindowContext,
) {
    let Some(new_menu) = (builder)(cx) else {
        return;
    };
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
    cx.refresh();
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
        trigger_handle: None,
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
    type RequestLayoutState = PopoverMenuFrameState;
    type PrepaintState = Option<HitboxId>;

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        cx.with_element_state(
            global_id.unwrap(),
            |element_state: Option<PopoverMenuElementState<M>>, cx| {
                let element_state = element_state.unwrap_or_default();
                let mut menu_layout_id = None;

                let menu_element = element_state.menu.borrow_mut().as_mut().map(|menu| {
                    let mut anchored = anchored().snap_to_window().anchor(self.anchor);
                    if let Some(child_bounds) = element_state.child_bounds {
                        anchored = anchored.position(
                            self.resolved_attach().corner(child_bounds) + self.resolved_offset(cx),
                        );
                    }
                    let mut element = deferred(anchored.child(div().occlude().child(menu.clone())))
                        .with_priority(1)
                        .into_any();

                    menu_layout_id = Some(element.request_layout(cx));
                    element
                });

                let mut child_element = self.child_builder.take().map(|child_builder| {
                    (child_builder)(element_state.menu.clone(), self.menu_builder.clone())
                });

                if let Some(trigger_handle) = self.trigger_handle.take() {
                    if let Some(menu_builder) = self.menu_builder.clone() {
                        *trigger_handle.0.borrow_mut() = Some(PopoverMenuHandleState {
                            menu_builder,
                            menu: element_state.menu.clone(),
                        });
                    }
                }

                let child_layout_id = child_element
                    .as_mut()
                    .map(|child_element| child_element.request_layout(cx));

                let layout_id = cx.request_layout(
                    gpui::Style::default(),
                    menu_layout_id.into_iter().chain(child_layout_id),
                );

                (
                    (
                        layout_id,
                        PopoverMenuFrameState {
                            child_element,
                            child_layout_id,
                            menu_element,
                        },
                    ),
                    element_state,
                )
            },
        )
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        _bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> Option<HitboxId> {
        if let Some(child) = request_layout.child_element.as_mut() {
            child.prepaint(cx);
        }

        if let Some(menu) = request_layout.menu_element.as_mut() {
            menu.prepaint(cx);
        }

        let hitbox_id = request_layout.child_layout_id.map(|layout_id| {
            let bounds = cx.layout_bounds(layout_id);
            cx.with_element_state(global_id.unwrap(), |element_state, _cx| {
                let mut element_state: PopoverMenuElementState<M> = element_state.unwrap();
                element_state.child_bounds = Some(bounds);
                ((), element_state)
            });

            cx.insert_hitbox(bounds, false).id
        });

        hitbox_id
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _: Bounds<gpui::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        child_hitbox: &mut Option<HitboxId>,
        cx: &mut WindowContext,
    ) {
        if let Some(mut child) = request_layout.child_element.take() {
            child.paint(cx);
        }

        if let Some(mut menu) = request_layout.menu_element.take() {
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
    }
}

impl<M: ManagedView> IntoElement for PopoverMenu<M> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
