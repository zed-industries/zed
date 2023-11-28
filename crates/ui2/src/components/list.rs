use gpui::{
    div, px, AnyElement, ClickEvent, Div, IntoElement, Stateful, StatefulInteractiveElement,
};
use smallvec::SmallVec;
use std::rc::Rc;

use crate::{
    disclosure_control, h_stack, v_stack, Avatar, Icon, IconElement, IconSize, Label, Toggle,
};
use crate::{prelude::*, GraphicSlot};

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub enum ListItemVariant {
    /// The list item extends to the far left and right of the list.
    FullWidth,
    #[default]
    Inset,
}

pub enum ListHeaderMeta {
    // TODO: These should be IconButtons
    Tools(Vec<Icon>),
    // TODO: This should be a button
    Button(Label),
    Text(Label),
}

#[derive(IntoElement)]
pub struct ListHeader {
    label: SharedString,
    left_icon: Option<Icon>,
    meta: Option<ListHeaderMeta>,
    variant: ListItemVariant,
    toggle: Toggle,
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
                    .children(icons.into_iter().map(|i| {
                        IconElement::new(i)
                            .color(Color::Muted)
                            .size(IconSize::Small)
                    })),
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
                    .when(self.variant == ListItemVariant::Inset, |this| this.px_2())
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

impl ListHeader {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            left_icon: None,
            meta: None,
            variant: ListItemVariant::default(),
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

    pub fn meta(mut self, meta: Option<ListHeaderMeta>) -> Self {
        self.meta = meta;
        self
    }

    // before_ship!("delete")
    // fn render<V: 'static>(self,  cx: &mut WindowContext) -> impl Element<V> {
    //     let disclosure_control = disclosure_control(self.toggle);

    //     let meta = match self.meta {
    //         Some(ListHeaderMeta::Tools(icons)) => div().child(
    //             h_stack()
    //                 .gap_2()
    //                 .items_center()
    //                 .children(icons.into_iter().map(|i| {
    //                     IconElement::new(i)
    //                         .color(TextColor::Muted)
    //                         .size(IconSize::Small)
    //                 })),
    //         ),
    //         Some(ListHeaderMeta::Button(label)) => div().child(label),
    //         Some(ListHeaderMeta::Text(label)) => div().child(label),
    //         None => div(),
    //     };

    //     h_stack()
    //         .w_full()
    //         .bg(cx.theme().colors().surface_background)
    //         // TODO: Add focus state
    //         // .when(self.state == InteractionState::Focused, |this| {
    //         //     this.border()
    //         //         .border_color(cx.theme().colors().border_focused)
    //         // })
    //         .relative()
    //         .child(
    //             div()
    //                 .h_5()
    //                 .when(self.variant == ListItemVariant::Inset, |this| this.px_2())
    //                 .flex()
    //                 .flex_1()
    //                 .items_center()
    //                 .justify_between()
    //                 .w_full()
    //                 .gap_1()
    //                 .child(
    //                     h_stack()
    //                         .gap_1()
    //                         .child(
    //                             div()
    //                                 .flex()
    //                                 .gap_1()
    //                                 .items_center()
    //                                 .children(self.left_icon.map(|i| {
    //                                     IconElement::new(i)
    //                                         .color(TextColor::Muted)
    //                                         .size(IconSize::Small)
    //                                 }))
    //                                 .child(Label::new(self.label.clone()).color(TextColor::Muted)),
    //                         )
    //                         .child(disclosure_control),
    //                 )
    //                 .child(meta),
    //         )
    // }
}

#[derive(IntoElement, Clone)]
pub struct ListSubHeader {
    label: SharedString,
    left_icon: Option<Icon>,
    variant: ListItemVariant,
}

impl ListSubHeader {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            left_icon: None,
            variant: ListItemVariant::default(),
        }
    }

    pub fn left_icon(mut self, left_icon: Option<Icon>) -> Self {
        self.left_icon = left_icon;
        self
    }
}

impl RenderOnce for ListSubHeader {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        h_stack().flex_1().w_full().relative().py_1().child(
            div()
                .h_6()
                .when(self.variant == ListItemVariant::Inset, |this| this.px_2())
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

#[derive(Default, PartialEq, Copy, Clone)]
pub enum ListEntrySize {
    #[default]
    Small,
    Medium,
}

#[derive(IntoElement)]
pub struct ListItem {
    id: ElementId,
    disabled: bool,
    // TODO: Reintroduce this
    // disclosure_control_style: DisclosureControlVisibility,
    indent_level: u32,
    left_slot: Option<GraphicSlot>,
    size: ListEntrySize,
    toggle: Toggle,
    variant: ListItemVariant,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
    children: SmallVec<[AnyElement; 2]>,
}

impl ListItem {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            disabled: false,
            indent_level: 0,
            left_slot: None,
            size: ListEntrySize::default(),
            toggle: Toggle::NotToggleable,
            variant: ListItemVariant::default(),
            on_click: Default::default(),
            children: SmallVec::new(),
        }
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn variant(mut self, variant: ListItemVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn indent_level(mut self, indent_level: u32) -> Self {
        self.indent_level = indent_level;
        self
    }

    pub fn toggle(mut self, toggle: Toggle) -> Self {
        self.toggle = toggle;
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

    pub fn left_avatar(mut self, left_avatar: impl Into<SharedString>) -> Self {
        self.left_slot = Some(GraphicSlot::Avatar(left_avatar.into()));
        self
    }

    pub fn size(mut self, size: ListEntrySize) -> Self {
        self.size = size;
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
            Some(GraphicSlot::Avatar(src)) => Some(h_stack().child(Avatar::uri(src))),
            Some(GraphicSlot::PublicActor(src)) => Some(h_stack().child(Avatar::uri(src))),
            None => None,
        };

        let sized_item = match self.size {
            ListEntrySize::Small => div().h_6(),
            ListEntrySize::Medium => div().h_7(),
        };
        div()
            .id(self.id)
            .relative()
            .hover(|mut style| {
                style.background = Some(cx.theme().colors().editor_background.into());
                style
            })
            .on_click({
                let on_click = self.on_click.clone();
                move |event, cx| {
                    if let Some(on_click) = &on_click {
                        (on_click)(event, cx)
                    }
                }
            })
            // TODO: Add focus state
            // .when(self.state == InteractionState::Focused, |this| {
            //     this.border()
            //         .border_color(cx.theme().colors().border_focused)
            // })
            .hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
            .active(|style| style.bg(cx.theme().colors().ghost_element_active))
            .child(
                sized_item
                    .when(self.variant == ListItemVariant::Inset, |this| this.px_2())
                    // .ml(rems(0.75 * self.indent_level as f32))
                    .children((0..self.indent_level).map(|_| {
                        div()
                            .w(px(4.))
                            .h_full()
                            .flex()
                            .justify_center()
                            .group_hover("", |style| style.bg(cx.theme().colors().border_focused))
                            .child(
                                h_stack()
                                    .child(div().w_px().h_full())
                                    .child(div().w_px().h_full().bg(cx.theme().colors().border)),
                            )
                    }))
                    .flex()
                    .gap_1()
                    .items_center()
                    .relative()
                    .child(disclosure_control(self.toggle))
                    .children(left_content)
                    .children(self.children),
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

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        let list_content = match (self.children.is_empty(), self.toggle) {
            (false, _) => div().children(self.children),
            (true, Toggle::Toggled(false)) => div(),
            (true, _) => div().child(Label::new(self.empty_message.clone()).color(Color::Muted)),
        };

        v_stack()
            .w_full()
            .py_1()
            .children(self.header.map(|header| header))
            .child(list_content)
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
