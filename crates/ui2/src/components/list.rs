use std::rc::Rc;

use gpui::{
    div, px, AnyElement, ClickEvent, Div, ImageSource, IntoElement, MouseButton, MouseDownEvent,
    Pixels, Stateful, StatefulInteractiveElement,
};
use smallvec::SmallVec;

use crate::{
    disclosure_control, h_stack, v_stack, Avatar, Icon, IconButton, IconElement, IconSize, Label,
    Toggle,
};
use crate::{prelude::*, GraphicSlot};

pub enum ListHeaderMeta {
    Tools(Vec<IconButton>),
    // TODO: This should be a button
    Button(Label),
    Text(Label),
}

#[derive(IntoElement)]
pub struct ListHeader {
    label: SharedString,
    left_icon: Option<Icon>,
    meta: Option<ListHeaderMeta>,
    toggle: Toggle,
    inset: bool,
}

impl ListHeader {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            left_icon: None,
            meta: None,
            inset: false,
            toggle: Toggle::NotToggleable,
        }
    }

    pub fn toggle(mut self, toggle: Toggle) -> Self {
        self.toggle = toggle;
        self
    }

    pub fn left_icon(mut self, left_icon: Option<Icon>) -> Self {
        self.left_icon = left_icon;
        self
    }

    pub fn right_button(self, button: IconButton) -> Self {
        self.meta(Some(ListHeaderMeta::Tools(vec![button])))
    }

    pub fn meta(mut self, meta: Option<ListHeaderMeta>) -> Self {
        self.meta = meta;
        self
    }
}

impl RenderOnce for ListHeader {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        let disclosure_control = disclosure_control(self.toggle);

        let meta = match self.meta {
            Some(ListHeaderMeta::Tools(icons)) => div().child(
                h_stack()
                    .gap_2()
                    .items_center()
                    .children(icons.into_iter().map(|i| i.color(Color::Muted))),
            ),
            Some(ListHeaderMeta::Button(label)) => div().child(label),
            Some(ListHeaderMeta::Text(label)) => div().child(label),
            None => div(),
        };

        h_stack()
            .w_full()
            .bg(cx.theme().colors().surface_background)
            .relative()
            .child(
                div()
                    .h_5()
                    .when(self.inset, |this| this.px_2())
                    .flex()
                    .flex_1()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .gap_1()
                    .child(
                        h_stack()
                            .gap_1()
                            .child(
                                div()
                                    .flex()
                                    .gap_1()
                                    .items_center()
                                    .children(self.left_icon.map(|i| {
                                        IconElement::new(i)
                                            .color(Color::Muted)
                                            .size(IconSize::Small)
                                    }))
                                    .child(Label::new(self.label.clone()).color(Color::Muted)),
                            )
                            .child(disclosure_control),
                    )
                    .child(meta),
            )
    }
}

#[derive(IntoElement, Clone)]
pub struct ListSubHeader {
    label: SharedString,
    left_icon: Option<Icon>,
    inset: bool,
}

impl ListSubHeader {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            left_icon: None,
            inset: false,
        }
    }

    pub fn left_icon(mut self, left_icon: Option<Icon>) -> Self {
        self.left_icon = left_icon;
        self
    }
}

impl RenderOnce for ListSubHeader {
    type Rendered = Div;

    fn render(self, _cx: &mut WindowContext) -> Self::Rendered {
        h_stack().flex_1().w_full().relative().py_1().child(
            div()
                .h_6()
                .when(self.inset, |this| this.px_2())
                .flex()
                .flex_1()
                .w_full()
                .gap_1()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .gap_1()
                        .items_center()
                        .children(self.left_icon.map(|i| {
                            IconElement::new(i)
                                .color(Color::Muted)
                                .size(IconSize::Small)
                        }))
                        .child(Label::new(self.label.clone()).color(Color::Muted)),
                ),
        )
    }
}

#[derive(IntoElement)]
pub struct ListItem {
    id: ElementId,
    selected: bool,
    // TODO: Reintroduce this
    // disclosure_control_style: DisclosureControlVisibility,
    indent_level: usize,
    indent_step_size: Pixels,
    left_slot: Option<GraphicSlot>,
    toggle: Toggle,
    inset: bool,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
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
            toggle: Toggle::NotToggleable,
            inset: false,
            on_click: None,
            on_secondary_mouse_down: None,
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

    pub fn toggle(mut self, toggle: Toggle) -> Self {
        self.toggle = toggle;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn left_content(mut self, left_content: GraphicSlot) -> Self {
        self.left_slot = Some(left_content);
        self
    }

    pub fn left_icon(mut self, left_icon: Icon) -> Self {
        self.left_slot = Some(GraphicSlot::Icon(left_icon));
        self
    }

    pub fn left_avatar(mut self, left_avatar: impl Into<ImageSource>) -> Self {
        self.left_slot = Some(GraphicSlot::Avatar(left_avatar.into()));
        self
    }
}

impl RenderOnce for ListItem {
    type Rendered = Stateful<Div>;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        let left_content = match self.left_slot.clone() {
            Some(GraphicSlot::Icon(i)) => Some(
                h_stack().child(
                    IconElement::new(i)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                ),
            ),
            Some(GraphicSlot::Avatar(src)) => Some(h_stack().child(Avatar::source(src))),
            Some(GraphicSlot::PublicActor(src)) => Some(h_stack().child(Avatar::uri(src))),
            None => None,
        };

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
            .when_some(self.on_click.clone(), |this, on_click| {
                this.on_click(move |event, cx| {
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
            .child(
                div()
                    .when(self.inset, |this| this.px_2())
                    .ml(self.indent_level as f32 * self.indent_step_size)
                    .flex()
                    .gap_1()
                    .items_center()
                    .relative()
                    .child(disclosure_control(self.toggle))
                    .children(left_content)
                    .children(self.children)
                    // HACK: We need to attach the `on_click` handler to the child element in order to have the click
                    // event actually fire.
                    // Once this is fixed in GPUI we can remove this and rely on the `on_click` handler set above on the
                    // outer `div`.
                    .id("on_click_hack")
                    .when_some(self.on_click, |this, on_click| {
                        this.on_click(move |event, cx| {
                            // HACK: GPUI currently fires `on_click` with any mouse button,
                            // but we only care about the left button.
                            if event.down.button == MouseButton::Left {
                                (on_click)(event, cx)
                            }
                        })
                    }),
            )
    }
}

impl ParentElement for ListItem {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.children
    }
}

#[derive(IntoElement, Clone)]
pub struct ListSeparator;

impl ListSeparator {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for ListSeparator {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        div().h_px().w_full().bg(cx.theme().colors().border_variant)
    }
}

#[derive(IntoElement)]
pub struct List {
    /// Message to display when the list is empty
    /// Defaults to "No items"
    empty_message: SharedString,
    header: Option<ListHeader>,
    toggle: Toggle,
    children: SmallVec<[AnyElement; 2]>,
}

impl RenderOnce for List {
    type Rendered = Div;

    fn render(self, _cx: &mut WindowContext) -> Self::Rendered {
        v_stack()
            .w_full()
            .py_1()
            .children(self.header.map(|header| header))
            .map(|this| match (self.children.is_empty(), self.toggle) {
                (false, _) => this.children(self.children),
                (true, Toggle::Toggled(false)) => this,
                (true, _) => this.child(Label::new(self.empty_message.clone()).color(Color::Muted)),
            })
    }
}

impl List {
    pub fn new() -> Self {
        Self {
            empty_message: "No items".into(),
            header: None,
            toggle: Toggle::NotToggleable,
            children: SmallVec::new(),
        }
    }

    pub fn empty_message(mut self, empty_message: impl Into<SharedString>) -> Self {
        self.empty_message = empty_message.into();
        self
    }

    pub fn header(mut self, header: ListHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn toggle(mut self, toggle: Toggle) -> Self {
        self.toggle = toggle;
        self
    }
}

impl ParentElement for List {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.children
    }
}
