use std::{cell::RefCell, rc::Rc};

use gpui::{
    AnyElement, AnyView, App, Bounds, Corner, DismissEvent, DispatchPhase, Element, ElementId,
    Entity, Focusable as _, GlobalElementId, HitboxId, InteractiveElement, IntoElement, LayoutId,
    Length, ManagedView, MouseDownEvent, ParentElement, Pixels, Point, Style, Window, anchored,
    deferred, div, point, prelude::FluentBuilder, px, size,
};

use crate::prelude::*;

pub trait PopoverTrigger: IntoElement + Clickable + Toggleable + 'static {}

impl<T: IntoElement + Clickable + Toggleable + 'static> PopoverTrigger for T {}

impl<T: Clickable> Clickable for gpui::AnimationElement<T>
where
    T: Clickable + 'static,
{
    fn on_click(
        self,
        handler: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.map_element(|e| e.on_click(handler))
    }

    fn cursor_style(self, cursor_style: gpui::CursorStyle) -> Self {
        self.map_element(|e| e.cursor_style(cursor_style))
    }
}

impl<T: Toggleable> Toggleable for gpui::AnimationElement<T>
where
    T: Toggleable + 'static,
{
    fn toggle_state(self, selected: bool) -> Self {
        self.map_element(|e| e.toggle_state(selected))
    }
}

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
    menu_builder: Rc<dyn Fn(&mut Window, &mut App) -> Option<Entity<M>>>,
    menu: Rc<RefCell<Option<Entity<M>>>>,
    on_open: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
}

impl<M: ManagedView> PopoverMenuHandle<M> {
    pub fn show(&self, window: &mut Window, cx: &mut App) {
        if let Some(state) = self.0.borrow().as_ref() {
            show_menu(
                &state.menu_builder,
                &state.menu,
                state.on_open.clone(),
                window,
                cx,
            );
        }
    }

    pub fn hide(&self, cx: &mut App) {
        if let Some(state) = self.0.borrow().as_ref() {
            if let Some(menu) = state.menu.borrow().as_ref() {
                menu.update(cx, |_, cx| cx.emit(DismissEvent));
            }
        }
    }

    pub fn toggle(&self, window: &mut Window, cx: &mut App) {
        if let Some(state) = self.0.borrow().as_ref() {
            if state.menu.borrow().is_some() {
                self.hide(cx);
            } else {
                self.show(window, cx);
            }
        }
    }

    pub fn is_deployed(&self) -> bool {
        self.0
            .borrow()
            .as_ref()
            .map_or(false, |state| state.menu.borrow().as_ref().is_some())
    }

    pub fn is_focused(&self, window: &Window, cx: &App) -> bool {
        self.0.borrow().as_ref().map_or(false, |state| {
            state
                .menu
                .borrow()
                .as_ref()
                .map_or(false, |model| model.focus_handle(cx).is_focused(window))
        })
    }
}

pub struct PopoverMenu<M: ManagedView> {
    id: ElementId,
    child_builder: Option<
        Box<
            dyn FnOnce(
                    Rc<RefCell<Option<Entity<M>>>>,
                    Option<Rc<dyn Fn(&mut Window, &mut App) -> Option<Entity<M>> + 'static>>,
                ) -> AnyElement
                + 'static,
        >,
    >,
    menu_builder: Option<Rc<dyn Fn(&mut Window, &mut App) -> Option<Entity<M>> + 'static>>,
    anchor: Corner,
    attach: Option<Corner>,
    offset: Option<Point<Pixels>>,
    trigger_handle: Option<PopoverMenuHandle<M>>,
    on_open: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    full_width: bool,
}

impl<M: ManagedView> PopoverMenu<M> {
    /// Returns a new [`PopoverMenu`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            child_builder: None,
            menu_builder: None,
            anchor: Corner::TopLeft,
            attach: None,
            offset: None,
            trigger_handle: None,
            on_open: None,
            full_width: false,
        }
    }

    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }

    pub fn menu(
        mut self,
        f: impl Fn(&mut Window, &mut App) -> Option<Entity<M>> + 'static,
    ) -> Self {
        self.menu_builder = Some(Rc::new(f));
        self
    }

    pub fn with_handle(mut self, handle: PopoverMenuHandle<M>) -> Self {
        self.trigger_handle = Some(handle);
        self
    }

    pub fn trigger<T: PopoverTrigger>(mut self, t: T) -> Self {
        let on_open = self.on_open.clone();
        self.child_builder = Some(Box::new(move |menu, builder| {
            let open = menu.borrow().is_some();
            t.toggle_state(open)
                .when_some(builder, |el, builder| {
                    el.on_click(move |_event, window, cx| {
                        show_menu(&builder, &menu, on_open.clone(), window, cx)
                    })
                })
                .into_any_element()
        }));
        self
    }

    /// This method prevents the trigger button tooltip from being seen when the menu is open.
    pub fn trigger_with_tooltip<T: PopoverTrigger + ButtonCommon>(
        mut self,
        t: T,
        tooltip_builder: impl Fn(&mut Window, &mut App) -> AnyView + 'static,
    ) -> Self {
        let on_open = self.on_open.clone();
        self.child_builder = Some(Box::new(move |menu, builder| {
            let open = menu.borrow().is_some();
            t.toggle_state(open)
                .when_some(builder, |el, builder| {
                    el.on_click(move |_, window, cx| {
                        show_menu(&builder, &menu, on_open.clone(), window, cx)
                    })
                    .when(!open, |t| {
                        t.tooltip(move |window, cx| tooltip_builder(window, cx))
                    })
                })
                .into_any_element()
        }));
        self
    }

    /// Defines which corner of the menu to anchor to the attachment point.
    /// By default, it uses the cursor position. Also see the `attach` method.
    pub fn anchor(mut self, anchor: Corner) -> Self {
        self.anchor = anchor;
        self
    }

    /// Defines which corner of the handle to attach the menu's anchor to.
    pub fn attach(mut self, attach: Corner) -> Self {
        self.attach = Some(attach);
        self
    }

    /// Offsets the position of the content by that many pixels.
    pub fn offset(mut self, offset: Point<Pixels>) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Attaches something upon opening the menu.
    pub fn on_open(mut self, on_open: Rc<dyn Fn(&mut Window, &mut App)>) -> Self {
        self.on_open = Some(on_open);
        self
    }

    fn resolved_attach(&self) -> Corner {
        self.attach.unwrap_or(match self.anchor {
            Corner::TopLeft => Corner::BottomLeft,
            Corner::TopRight => Corner::BottomRight,
            Corner::BottomLeft => Corner::TopLeft,
            Corner::BottomRight => Corner::TopRight,
        })
    }

    fn resolved_offset(&self, window: &mut Window) -> Point<Pixels> {
        self.offset.unwrap_or_else(|| {
            // Default offset = 4px padding + 1px border
            let offset = rems_from_px(5.) * window.rem_size();
            match self.anchor {
                Corner::TopRight | Corner::BottomRight => point(offset, px(0.)),
                Corner::TopLeft | Corner::BottomLeft => point(-offset, px(0.)),
            }
        })
    }
}

fn show_menu<M: ManagedView>(
    builder: &Rc<dyn Fn(&mut Window, &mut App) -> Option<Entity<M>>>,
    menu: &Rc<RefCell<Option<Entity<M>>>>,
    on_open: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    window: &mut Window,
    cx: &mut App,
) {
    let Some(new_menu) = (builder)(window, cx) else {
        return;
    };
    let menu2 = menu.clone();
    let previous_focus_handle = window.focused(cx);

    window
        .subscribe(&new_menu, cx, move |modal, _: &DismissEvent, window, cx| {
            if modal.focus_handle(cx).contains_focused(window, cx) {
                if let Some(previous_focus_handle) = previous_focus_handle.as_ref() {
                    window.focus(previous_focus_handle);
                }
            }
            *menu2.borrow_mut() = None;
            window.refresh();
        })
        .detach();
    window.focus(&new_menu.focus_handle(cx));
    *menu.borrow_mut() = Some(new_menu);
    window.refresh();

    if let Some(on_open) = on_open {
        on_open(window, cx);
    }
}

pub struct PopoverMenuElementState<M> {
    menu: Rc<RefCell<Option<Entity<M>>>>,
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

pub struct PopoverMenuFrameState<M: ManagedView> {
    child_layout_id: Option<LayoutId>,
    child_element: Option<AnyElement>,
    menu_element: Option<AnyElement>,
    menu_handle: Rc<RefCell<Option<Entity<M>>>>,
}

impl<M: ManagedView> Element for PopoverMenu<M> {
    type RequestLayoutState = PopoverMenuFrameState<M>;
    type PrepaintState = Option<HitboxId>;

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        window.with_element_state(
            global_id.unwrap(),
            |element_state: Option<PopoverMenuElementState<M>>, window| {
                let element_state = element_state.unwrap_or_default();
                let mut menu_layout_id = None;

                let menu_element = element_state.menu.borrow_mut().as_mut().map(|menu| {
                    let offset = self.resolved_offset(window);
                    let mut anchored = anchored()
                        .snap_to_window_with_margin(px(8.))
                        .anchor(self.anchor)
                        .offset(offset);
                    if let Some(child_bounds) = element_state.child_bounds {
                        anchored =
                            anchored.position(child_bounds.corner(self.resolved_attach()) + offset);
                    }
                    let mut element = deferred(anchored.child(div().occlude().child(menu.clone())))
                        .with_priority(1)
                        .into_any();

                    menu_layout_id = Some(element.request_layout(window, cx));
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
                            on_open: self.on_open.clone(),
                        });
                    }
                }

                let child_layout_id = child_element
                    .as_mut()
                    .map(|child_element| child_element.request_layout(window, cx));

                let mut style = Style::default();
                if self.full_width {
                    style.size = size(relative(1.).into(), Length::Auto);
                }

                let layout_id = window.request_layout(
                    style,
                    menu_layout_id.into_iter().chain(child_layout_id),
                    cx,
                );

                (
                    (
                        layout_id,
                        PopoverMenuFrameState {
                            child_element,
                            child_layout_id,
                            menu_element,
                            menu_handle: element_state.menu.clone(),
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
        _inspector_id: Option<&gpui::InspectorElementId>,
        _bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<HitboxId> {
        if let Some(child) = request_layout.child_element.as_mut() {
            child.prepaint(window, cx);
        }

        if let Some(menu) = request_layout.menu_element.as_mut() {
            menu.prepaint(window, cx);
        }

        request_layout.child_layout_id.map(|layout_id| {
            let bounds = window.layout_bounds(layout_id);
            window.with_element_state(global_id.unwrap(), |element_state, _cx| {
                let mut element_state: PopoverMenuElementState<M> = element_state.unwrap();
                element_state.child_bounds = Some(bounds);
                ((), element_state)
            });

            window.insert_hitbox(bounds, false).id
        })
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        _: Bounds<gpui::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        child_hitbox: &mut Option<HitboxId>,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(mut child) = request_layout.child_element.take() {
            child.paint(window, cx);
        }

        if let Some(mut menu) = request_layout.menu_element.take() {
            menu.paint(window, cx);

            if let Some(child_hitbox) = *child_hitbox {
                let menu_handle = request_layout.menu_handle.clone();
                // Mouse-downing outside the menu dismisses it, so we don't
                // want a click on the toggle to re-open it.
                window.on_mouse_event(move |_: &MouseDownEvent, phase, window, cx| {
                    if phase == DispatchPhase::Bubble && child_hitbox.is_hovered(window) {
                        if let Some(menu) = menu_handle.borrow().as_ref() {
                            menu.update(cx, |_, cx| {
                                cx.emit(DismissEvent);
                            });
                        }
                        cx.stop_propagation();
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
