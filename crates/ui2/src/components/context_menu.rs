use std::cell::RefCell;
use std::rc::Rc;

use crate::{prelude::*, v_stack, List};
use crate::{ListItem, ListSeparator, ListSubHeader};
use gpui::{
    overlay, px, Action, AnchorCorner, AnyElement, AppContext, Bounds, ClickEvent, DispatchPhase,
    Div, EventEmitter, FocusHandle, FocusableView, LayoutId, ManagedView, Manager, MouseButton,
    MouseDownEvent, Pixels, Point, Render, RenderOnce, View, VisualContext,
};

pub enum ContextMenuItem {
    Separator(ListSeparator),
    Header(ListSubHeader),
    Entry(ListItem, Rc<dyn Fn(&ClickEvent, &mut WindowContext)>),
}

pub struct ContextMenu {
    items: Vec<ContextMenuItem>,
    focus_handle: FocusHandle,
}

impl FocusableView for ContextMenu {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<Manager> for ContextMenu {}

impl ContextMenu {
    pub fn build(
        cx: &mut WindowContext,
        f: impl FnOnce(Self, &mut WindowContext) -> Self,
    ) -> View<Self> {
        // let handle = cx.view().downgrade();
        cx.build_view(|cx| {
            f(
                Self {
                    items: Default::default(),
                    focus_handle: cx.focus_handle(),
                },
                cx,
            )
        })
    }

    pub fn header(mut self, title: impl Into<SharedString>) -> Self {
        self.items
            .push(ContextMenuItem::Header(ListSubHeader::new(title)));
        self
    }

    pub fn separator(mut self) -> Self {
        self.items.push(ContextMenuItem::Separator(ListSeparator));
        self
    }

    pub fn entry(
        mut self,
        view: ListItem,
        on_click: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.items
            .push(ContextMenuItem::Entry(view, Rc::new(on_click)));
        self
    }

    pub fn action(self, view: ListItem, action: Box<dyn Action>) -> Self {
        // todo: add the keybindings to the list entry
        self.entry(view, move |_, cx| cx.dispatch_action(action.boxed_clone()))
    }

    pub fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        // todo!()
        cx.emit(Manager::Dismiss);
    }

    pub fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(Manager::Dismiss);
    }
}

impl Render for ContextMenu {
    type Element = Div;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        div().elevation_2(cx).flex().flex_row().child(
            v_stack()
                .min_w(px(200.))
                .track_focus(&self.focus_handle)
                .on_mouse_down_out(
                    cx.listener(|this: &mut Self, _, cx| this.cancel(&Default::default(), cx)),
                )
                // .on_action(ContextMenu::select_first)
                // .on_action(ContextMenu::select_last)
                // .on_action(ContextMenu::select_next)
                // .on_action(ContextMenu::select_prev)
                .on_action(cx.listener(ContextMenu::confirm))
                .on_action(cx.listener(ContextMenu::cancel))
                .flex_none()
                // .bg(cx.theme().colors().elevated_surface_background)
                // .border()
                // .border_color(cx.theme().colors().border)
                .child(
                    List::new().children(self.items.iter().map(|item| match item {
                        ContextMenuItem::Separator(separator) => {
                            separator.clone().render_into_any()
                        }
                        ContextMenuItem::Header(header) => header.clone().render_into_any(),
                        ContextMenuItem::Entry(entry, callback) => {
                            let callback = callback.clone();
                            let dismiss = cx.listener(|_, _, cx| cx.emit(Manager::Dismiss));

                            entry
                                .clone()
                                .on_click(move |event, cx| {
                                    callback(event, cx);
                                    dismiss(event, cx)
                                })
                                .render_into_any()
                        }
                    })),
                ),
        )
    }
}

pub struct MenuHandle<M: ManagedView> {
    id: ElementId,
    child_builder: Option<Box<dyn FnOnce(bool) -> AnyElement + 'static>>,
    menu_builder: Option<Rc<dyn Fn(&mut WindowContext) -> View<M> + 'static>>,
    anchor: Option<AnchorCorner>,
    attach: Option<AnchorCorner>,
}

impl<M: ManagedView> MenuHandle<M> {
    pub fn menu(mut self, f: impl Fn(&mut WindowContext) -> View<M> + 'static) -> Self {
        self.menu_builder = Some(Rc::new(f));
        self
    }

    pub fn child<R: RenderOnce>(mut self, f: impl FnOnce(bool) -> R + 'static) -> Self {
        self.child_builder = Some(Box::new(|b| f(b).render_once().into_any()));
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

pub fn menu_handle<M: ManagedView>(id: impl Into<ElementId>) -> MenuHandle<M> {
    MenuHandle {
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
impl<M: ManagedView> Element for MenuHandle<M> {
    type State = MenuHandleState<M>;

    fn layout(
        &mut self,
        element_state: Option<Self::State>,
        cx: &mut WindowContext,
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
            menu_layout_id = Some(element.layout(cx));
            element
        });

        let mut child_element = self
            .child_builder
            .take()
            .map(|child_builder| (child_builder)(menu.borrow().is_some()));

        let child_layout_id = child_element
            .as_mut()
            .map(|child_element| child_element.layout(cx));

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
        self,
        bounds: Bounds<gpui::Pixels>,
        element_state: &mut Self::State,
        cx: &mut WindowContext,
    ) {
        if let Some(child) = element_state.child_element.take() {
            child.paint(cx);
        }

        if let Some(menu) = element_state.menu_element.take() {
            menu.paint(cx);
            return;
        }

        let Some(builder) = self.menu_builder else {
            return;
        };
        let menu = element_state.menu.clone();
        let position = element_state.position.clone();
        let attach = self.attach.clone();
        let child_layout_id = element_state.child_layout_id.clone();

        cx.on_mouse_event(move |event: &MouseDownEvent, phase, cx| {
            if phase == DispatchPhase::Bubble
                && event.button == MouseButton::Right
                && bounds.contains_point(&event.position)
            {
                cx.stop_propagation();
                cx.prevent_default();

                let new_menu = (builder)(cx);
                let menu2 = menu.clone();
                cx.subscribe(&new_menu, move |modal, e, cx| match e {
                    &Manager::Dismiss => {
                        *menu2.borrow_mut() = None;
                        cx.notify();
                    }
                })
                .detach();
                cx.focus_view(&new_menu);
                *menu.borrow_mut() = Some(new_menu);

                *position.borrow_mut() = if attach.is_some() && child_layout_id.is_some() {
                    attach
                        .unwrap()
                        .corner(cx.layout_bounds(child_layout_id.unwrap()))
                } else {
                    cx.mouse_position()
                };
                cx.notify();
            }
        });
    }
}

impl<M: ManagedView> RenderOnce for MenuHandle<M> {
    type Element = Self;

    fn element_id(&self) -> Option<gpui::ElementId> {
        Some(self.id.clone())
    }

    fn render_once(self) -> Self::Element {
        self
    }
}
