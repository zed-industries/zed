use std::cell::RefCell;
use std::rc::Rc;

use crate::{h_stack, prelude::*, ListItemVariant};
use crate::{v_stack, Label, List, ListEntry, ListItem, ListSeparator, ListSubHeader};
use gpui::{
    overlay, px, Action, AnyElement, Bounds, DispatchPhase, Div, EventEmitter, FocusHandle,
    Focusable, FocusableView, LayoutId, MouseButton, MouseDownEvent, Overlay, Render, View,
};
use smallvec::SmallVec;

pub enum ContextMenuItem {
    Header(SharedString),
    Entry(Label, Box<dyn gpui::Action>),
    Separator,
}

impl Clone for ContextMenuItem {
    fn clone(&self) -> Self {
        match self {
            ContextMenuItem::Header(name) => ContextMenuItem::Header(name.clone()),
            ContextMenuItem::Entry(label, action) => {
                ContextMenuItem::Entry(label.clone(), action.boxed_clone())
            }
            ContextMenuItem::Separator => ContextMenuItem::Separator,
        }
    }
}
impl ContextMenuItem {
    fn to_list_item(self) -> ListItem {
        match self {
            ContextMenuItem::Header(label) => ListSubHeader::new(label).into(),
            ContextMenuItem::Entry(label, action) => ListEntry::new(label)
                .variant(ListItemVariant::Inset)
                .action(action)
                .into(),
            ContextMenuItem::Separator => ListSeparator::new().into(),
        }
    }

    pub fn header(label: impl Into<SharedString>) -> Self {
        Self::Header(label.into())
    }

    pub fn separator() -> Self {
        Self::Separator
    }

    pub fn entry(label: Label, action: impl Action) -> Self {
        Self::Entry(label, Box::new(action))
    }
}

pub struct ContextMenu {
    items: Vec<ListItem>,
    focus_handle: FocusHandle,
}

pub enum MenuEvent {
    Dismissed,
}

impl EventEmitter<MenuEvent> for ContextMenu {}
impl FocusableView for ContextMenu {
    fn focus_handle(&self, cx: &gpui::AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ContextMenu {
    pub fn new(cx: &mut WindowContext) -> Self {
        Self {
            items: Default::default(),
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn header(mut self, title: impl Into<SharedString>) -> Self {
        self.items.push(ListItem::Header(ListSubHeader::new(title)));
        self
    }

    pub fn separator(mut self) -> Self {
        self.items.push(ListItem::Separator(ListSeparator));
        self
    }

    pub fn entry(mut self, label: Label, action: Box<dyn Action>) -> Self {
        self.items.push(ListEntry::new(label).action(action).into());
        self
    }

    pub fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        // todo!()
        cx.emit(MenuEvent::Dismissed);
    }

    pub fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(MenuEvent::Dismissed);
    }
}

impl Render for ContextMenu {
    type Element = Overlay<Self>;
    // todo!()
    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        overlay().child(
            div().elevation_2(cx).flex().flex_row().child(
                v_stack()
                    .min_w(px(200.))
                    .track_focus(&self.focus_handle)
                    .on_mouse_down_out(|this: &mut Self, _, cx| {
                        this.cancel(&Default::default(), cx)
                    })
                    // .on_action(ContextMenu::select_first)
                    // .on_action(ContextMenu::select_last)
                    // .on_action(ContextMenu::select_next)
                    // .on_action(ContextMenu::select_prev)
                    .on_action(ContextMenu::confirm)
                    .on_action(ContextMenu::cancel)
                    .flex_none()
                    // .bg(cx.theme().colors().elevated_surface_background)
                    // .border()
                    // .border_color(cx.theme().colors().border)
                    .child(List::new(self.items.clone())),
            ),
        )
    }
}

pub struct MenuHandle<V: 'static> {
    id: ElementId,
    children: SmallVec<[AnyElement<V>; 2]>,
    builder: Rc<dyn Fn(&mut V, &mut ViewContext<V>) -> View<ContextMenu> + 'static>,
}

impl<V: 'static> ParentComponent<V> for MenuHandle<V> {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<V>; 2]> {
        &mut self.children
    }
}

impl<V: 'static> MenuHandle<V> {
    pub fn new(
        id: impl Into<ElementId>,
        builder: impl Fn(&mut V, &mut ViewContext<V>) -> View<ContextMenu> + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            children: SmallVec::new(),
            builder: Rc::new(builder),
        }
    }
}

pub struct MenuHandleState<V> {
    menu: Rc<RefCell<Option<View<ContextMenu>>>>,
    menu_element: Option<AnyElement<V>>,
}
impl<V: 'static> Element<V> for MenuHandle<V> {
    type ElementState = MenuHandleState<V>;

    fn element_id(&self) -> Option<gpui::ElementId> {
        Some(self.id.clone())
    }

    fn layout(
        &mut self,
        view_state: &mut V,
        element_state: Option<Self::ElementState>,
        cx: &mut crate::ViewContext<V>,
    ) -> (gpui::LayoutId, Self::ElementState) {
        let mut child_layout_ids = self
            .children
            .iter_mut()
            .map(|child| child.layout(view_state, cx))
            .collect::<SmallVec<[LayoutId; 2]>>();

        let menu = if let Some(element_state) = element_state {
            element_state.menu
        } else {
            Rc::new(RefCell::new(None))
        };

        let menu_element = menu.borrow_mut().as_mut().map(|menu| {
            let mut view = menu.clone().render();
            child_layout_ids.push(view.layout(view_state, cx));
            view
        });

        let layout_id = cx.request_layout(&gpui::Style::default(), child_layout_ids.into_iter());

        (layout_id, MenuHandleState { menu, menu_element })
    }

    fn paint(
        &mut self,
        bounds: Bounds<gpui::Pixels>,
        view_state: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut crate::ViewContext<V>,
    ) {
        for child in &mut self.children {
            child.paint(view_state, cx);
        }

        if let Some(menu) = element_state.menu_element.as_mut() {
            menu.paint(view_state, cx);
            return;
        }

        let menu = element_state.menu.clone();
        let builder = self.builder.clone();
        cx.on_mouse_event(move |view_state, event: &MouseDownEvent, phase, cx| {
            if phase == DispatchPhase::Bubble
                && event.button == MouseButton::Right
                && bounds.contains_point(&event.position)
            {
                cx.stop_propagation();
                cx.prevent_default();

                let new_menu = (builder)(view_state, cx);
                let menu2 = menu.clone();
                cx.subscribe(&new_menu, move |this, modal, e, cx| match e {
                    MenuEvent::Dismissed => {
                        *menu2.borrow_mut() = None;
                        cx.notify();
                    }
                })
                .detach();
                *menu.borrow_mut() = Some(new_menu);
                cx.notify();
            }
        });
    }
}

impl<V: 'static> Component<V> for MenuHandle<V> {
    fn render(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::story::Story;
    use gpui::{action, Div, Render, VisualContext};

    pub struct ContextMenuStory;

    impl Render for ContextMenuStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            #[action]
            struct PrintCurrentDate {}

            Story::container(cx)
                .child(Story::title_for::<_, ContextMenu>(cx))
                .on_action(|_, _: &PrintCurrentDate, _| {
                    if let Ok(unix_time) = std::time::UNIX_EPOCH.elapsed() {
                        println!("Current Unix time is {:?}", unix_time.as_secs());
                    }
                })
                .child(
                    MenuHandle::new("test", move |_, cx| {
                        cx.build_view(|cx| {
                            ContextMenu::new(cx)
                                .header("Section header")
                                .separator()
                                .entry(
                                    Label::new("Print current time"),
                                    PrintCurrentDate {}.boxed_clone(),
                                )
                        })
                    })
                    .child(Label::new("RIGHT CLICK ME")),
                )
        }
    }
}
