use std::rc::Rc;

use gpui::{
    px, AnyElement, AnyView, ClickEvent, Div, ImageSource, MouseButton, MouseDownEvent, Pixels,
    Stateful,
};
use smallvec::SmallVec;

use crate::prelude::*;
use crate::{Avatar, Disclosure, Icon, IconElement, IconSize};

#[derive(IntoElement)]
pub struct ListItem {
    id: ElementId,
    selected: bool,
    // TODO: Reintroduce this
    // disclosure_control_style: DisclosureControlVisibility,
    indent_level: usize,
    indent_step_size: Pixels,
    left_slot: Option<AnyElement>,
    toggle: Option<bool>,
    inset: bool,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
    on_toggle: Option<Rc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
    tooltip: Option<Box<dyn Fn(&mut WindowContext) -> AnyView + 'static>>,
    on_secondary_mouse_down: Option<Rc<dyn Fn(&MouseDownEvent, &mut WindowContext) + 'static>>,
    children: SmallVec<[AnyElement; 2]>,
}

impl ListItem {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            selected: false,
            indent_level: 0,
            indent_step_size: px(12.),
            left_slot: None,
            toggle: None,
            inset: false,
            on_click: None,
            on_secondary_mouse_down: None,
            on_toggle: None,
            tooltip: None,
            children: SmallVec::new(),
        }
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn on_secondary_mouse_down(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.on_secondary_mouse_down = Some(Rc::new(handler));
        self
    }

    pub fn tooltip(mut self, tooltip: impl Fn(&mut WindowContext) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Box::new(tooltip));
        self
    }

    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    pub fn indent_level(mut self, indent_level: usize) -> Self {
        self.indent_level = indent_level;
        self
    }

    pub fn indent_step_size(mut self, indent_step_size: Pixels) -> Self {
        self.indent_step_size = indent_step_size;
        self
    }

    pub fn toggle(mut self, toggle: impl Into<Option<bool>>) -> Self {
        self.toggle = toggle.into();
        self
    }

    pub fn on_toggle(
        mut self,
        on_toggle: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.on_toggle = Some(Rc::new(on_toggle));
        self
    }

    pub fn left_child(mut self, left_content: impl IntoElement) -> Self {
        self.left_slot = Some(left_content.into_any_element());
        self
    }

    pub fn left_icon(mut self, left_icon: Icon) -> Self {
        self.left_slot = Some(
            IconElement::new(left_icon)
                .size(IconSize::Small)
                .color(Color::Muted)
                .into_any_element(),
        );
        self
    }

    pub fn left_avatar(mut self, left_avatar: impl Into<ImageSource>) -> Self {
        self.left_slot = Some(Avatar::source(left_avatar.into()).into_any_element());
        self
    }
}

impl Selectable for ListItem {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl ParentElement for ListItem {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.children
    }
}

impl RenderOnce for ListItem {
    type Rendered = Stateful<Div>;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        div()
            .id(self.id)
            .relative()
            // TODO: Add focus state
            // .when(self.state == InteractionState::Focused, |this| {
            //     this.border()
            //         .border_color(cx.theme().colors().border_focused)
            // })
            .when(self.inset, |this| this.rounded_md())
            .hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
            .active(|style| style.bg(cx.theme().colors().ghost_element_active))
            .when(self.selected, |this| {
                this.bg(cx.theme().colors().ghost_element_selected)
            })
            .when_some(self.on_click, |this, on_click| {
                this.cursor_pointer().on_click(move |event, cx| {
                    // HACK: GPUI currently fires `on_click` with any mouse button,
                    // but we only care about the left button.
                    if event.down.button == MouseButton::Left {
                        (on_click)(event, cx)
                    }
                })
            })
            .when_some(self.on_secondary_mouse_down, |this, on_mouse_down| {
                this.on_mouse_down(MouseButton::Right, move |event, cx| {
                    (on_mouse_down)(event, cx)
                })
            })
            .when_some(self.tooltip, |this, tooltip| this.tooltip(tooltip))
            .child(
                div()
                    .when(self.inset, |this| this.px_2())
                    .ml(self.indent_level as f32 * self.indent_step_size)
                    .flex()
                    .gap_1()
                    .items_center()
                    .relative()
                    .children(
                        self.toggle
                            .map(|is_open| Disclosure::new(is_open).on_toggle(self.on_toggle)),
                    )
                    .children(self.left_slot)
                    .children(self.children),
            )
    }
}
