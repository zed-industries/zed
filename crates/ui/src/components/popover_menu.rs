use std::{cell::RefCell, rc::Rc};

use gpui::{
    Anchor, AnyElement, AnyView, App, Bounds, DismissEvent, DispatchPhase, Element, ElementId,
    Entity, Focusable as _, GlobalElementId, HitboxBehavior, HitboxId, InteractiveElement,
    IntoElement, LayoutId, Length, ManagedView, MouseDownEvent, ParentElement, Pixels, Point,
    Style, Window, anchored, deferred, div, point, prelude::FluentBuilder, px, size,
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
        if let Some(state) = self.0.borrow().as_ref()
            && let Some(menu) = state.menu.borrow().as_ref()
        {
            menu.update(cx, |_, cx| cx.emit(DismissEvent));
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
            .is_some_and(|state| state.menu.borrow().as_ref().is_some())
    }

    pub fn is_focused(&self, window: &Window, cx: &App) -> bool {
        self.0.borrow().as_ref().is_some_and(|state| {
            state
                .menu
                .borrow()
                .as_ref()
                .is_some_and(|model| model.focus_handle(cx).is_focused(window))
        })
    }

    pub fn refresh_menu(
        &self,
        window: &mut Window,
        cx: &mut App,
        new_menu_builder: Rc<dyn Fn(&mut Window, &mut App) -> Option<Entity<M>>>,
    ) {
        let show_menu = if let Some(state) = self.0.borrow_mut().as_mut() {
            state.menu_builder = new_menu_builder;
            state.menu.borrow().is_some()
        } else {
            false
        };

        if show_menu {
            self.show(window, cx);
        }
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
    anchor: Anchor,
    attach: Option<Anchor>,
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
            anchor: Anchor::TopLeft,
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
    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// Defines which corner of the handle to attach the menu's anchor to.
    pub fn attach(mut self, attach: Anchor) -> Self {
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

    fn resolved_attach(&self) -> Anchor {
        self.attach
            .unwrap_or(self.attach.unwrap_or(match self.anchor {
                Anchor::TopLeft => Anchor::BottomLeft,
                Anchor::TopCenter => Anchor::BottomCenter,
                Anchor::TopRight => Anchor::BottomRight,
                Anchor::BottomLeft => Anchor::TopLeft,
                Anchor::BottomCenter => Anchor::TopCenter,
                Anchor::BottomRight => Anchor::TopRight,
                Anchor::LeftCenter => Anchor::LeftCenter,
                Anchor::RightCenter => Anchor::RightCenter,
            }))
    }

    fn resolved_offset(&self, window: &mut Window) -> Point<Pixels> {
        self.offset.unwrap_or_else(|| {
            // Default offset = 4px padding + 1px border
            let offset = rems_from_px(5_f32) * window.rem_size();
            match self.anchor {
                Anchor::TopRight | Anchor::BottomRight | Anchor::RightCenter => {
                    point(offset, px(0.))
                }
                Anchor::TopLeft | Anchor::BottomLeft | Anchor::LeftCenter => point(-offset, px(0.)),
                Anchor::TopCenter | Anchor::BottomCenter => point(px(0.), px(0.)),
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
    let previous_focus_handle = window.focused(cx);
    let Some(new_menu) = (builder)(window, cx) else {
        return;
    };
    let weak_menu = Rc::downgrade(menu);

    window
        .subscribe(&new_menu, cx, move |modal, _: &DismissEvent, window, cx| {
            let Some(menu) = weak_menu.upgrade() else {
                return;
            };
            if modal.focus_handle(cx).contains_focused(window, cx)
                && let Some(previous_focus_handle) = previous_focus_handle.as_ref()
            {
                window.focus(previous_focus_handle, cx);
            }
            *menu.borrow_mut() = None;
            window.refresh();
        })
        .detach();

    // Since menus are rendered in a deferred fashion, their focus handles are
    // not linked in the dispatch tree until after the deferred draw callback
    // runs. We need to wait for that to happen before focusing it, so that
    // calling `contains_focused` on the parent's focus handle returns `true`
    // when the menu is focused. This prevents the pane's tab bar buttons from
    // flickering when opening popover menus.
    let focus_handle = new_menu.focus_handle(cx);
    window.on_next_frame(move |window, _cx| {
        window.on_next_frame(move |window, cx| {
            window.focus(&focus_handle, cx);
        });
    });
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

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
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

                if let Some(trigger_handle) = self.trigger_handle.take()
                    && let Some(menu_builder) = self.menu_builder.clone()
                {
                    *trigger_handle.0.borrow_mut() = Some(PopoverMenuHandleState {
                        menu_builder,
                        menu: element_state.menu.clone(),
                        on_open: self.on_open.clone(),
                    });
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

            window.insert_hitbox(bounds, HitboxBehavior::Normal).id
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ContextMenu, right_click_menu};
    use gpui::{Modifiers, MouseButton, TestAppContext, VisualTestContext, WeakEntity};
    use menu::{SelectChild, SelectFirst};

    #[gpui::test]
    fn test_menus_are_released_when_their_element_is_removed(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
        });
        let (host, cx) = cx.add_window_view(|_, _| MenuHost {
            visible: true,
            opened_menu: Rc::default(),
            opened_submenu: Rc::default(),
        });

        for (selector, button) in [
            ("POPOVER_MENU", MouseButton::Left),
            ("RIGHT_CLICK_MENU", MouseButton::Right),
        ] {
            for submenu_depth in 0..=2 {
                set_visible(&host, true, cx);
                let position = cx
                    .debug_bounds(selector)
                    .expect("menu trigger should be rendered")
                    .center();
                cx.simulate_mouse_down(position, button, Modifiers::default());
                cx.simulate_mouse_up(position, button, Modifiers::default());
                cx.run_until_parked();
                let menu = host
                    .update(cx, |host, _| host.opened_menu.take())
                    .expect("menu should open");
                assert!(menu.upgrade().is_some(), "{selector} should stay open");

                let mut menus = vec![menu];
                for _ in 0..submenu_depth {
                    menus
                        .last()
                        .expect("parent menu should exist")
                        .update_in(cx, |menu, window, cx| {
                            menu.select_first(&SelectFirst, window, cx);
                            menu.select_submenu_child(&SelectChild, window, cx);
                        })
                        .expect("parent menu should remain alive");
                    cx.run_until_parked();
                    let submenu = host
                        .update(cx, |host, _| host.opened_submenu.take())
                        .expect("submenu should open");
                    assert!(submenu.upgrade().is_some());
                    menus.push(submenu);
                }

                set_visible(&host, false, cx);
                for menu in menus {
                    menu.assert_released();
                }
            }
        }
    }

    fn set_visible(host: &Entity<MenuHost>, visible: bool, cx: &mut VisualTestContext) {
        host.update(cx, |host, cx| {
            host.visible = visible;
            cx.notify();
        });
        cx.run_until_parked();
        cx.update(|window, _| window.refresh());
        cx.run_until_parked();
    }

    struct MenuHost {
        visible: bool,
        opened_menu: Rc<RefCell<Option<WeakEntity<ContextMenu>>>>,
        opened_submenu: Rc<RefCell<Option<WeakEntity<ContextMenu>>>>,
    }

    impl Render for MenuHost {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let opened_menu = self.opened_menu.clone();
            let opened_submenu = self.opened_submenu.clone();
            let build_menu = move |window: &mut Window, cx: &mut App| {
                let menu = ContextMenu::build(window, cx, |menu, _, _| {
                    menu.header("Menu").submenu("Submenu", {
                        let opened_submenu = opened_submenu.clone();
                        move |menu, _, cx| {
                            opened_submenu.replace(Some(cx.weak_entity()));
                            menu.submenu("Nested", {
                                let opened_submenu = opened_submenu.clone();
                                move |menu, _, cx| {
                                    opened_submenu.replace(Some(cx.weak_entity()));
                                    menu.header("Child")
                                }
                            })
                        }
                    })
                });
                opened_menu.replace(Some(menu.downgrade()));
                menu
            };
            v_flex().size_full().when(self.visible, |this| {
                this.child(
                    div().debug_selector(|| "POPOVER_MENU".to_owned()).child(
                        PopoverMenu::new("popover-menu")
                            .menu({
                                let build_menu = build_menu.clone();
                                move |window, cx| Some(build_menu(window, cx))
                            })
                            .trigger(Button::new("popover-menu-trigger", "Popover")),
                    ),
                )
                .child(
                    div()
                        .debug_selector(|| "RIGHT_CLICK_MENU".to_owned())
                        .child(
                            right_click_menu("right-click-menu")
                                .menu(build_menu)
                                .trigger(|_, _, _| Label::new("Right Click")),
                        ),
                )
            })
        }
    }
}
