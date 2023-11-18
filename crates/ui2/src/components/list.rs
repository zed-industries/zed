use gpui::{div, Action};

use crate::settings::user_settings;
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

#[derive(Component)]
pub struct ListHeader {
    label: SharedString,
    left_icon: Option<Icon>,
    meta: Option<ListHeaderMeta>,
    variant: ListItemVariant,
    toggle: Toggle,
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

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
        let disclosure_control = disclosure_control(self.toggle);

        let meta = match self.meta {
            Some(ListHeaderMeta::Tools(icons)) => div().child(
                h_stack()
                    .gap_2()
                    .items_center()
                    .children(icons.into_iter().map(|i| {
                        IconElement::new(i)
                            .color(TextColor::Muted)
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
            // TODO: Add focus state
            // .when(self.state == InteractionState::Focused, |this| {
            //     this.border()
            //         .border_color(cx.theme().colors().border_focused)
            // })
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
                                            .color(TextColor::Muted)
                                            .size(IconSize::Small)
                                    }))
                                    .child(Label::new(self.label.clone()).color(TextColor::Muted)),
                            )
                            .child(disclosure_control),
                    )
                    .child(meta),
            )
    }
}

#[derive(Component, Clone)]
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

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
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
                                .color(TextColor::Muted)
                                .size(IconSize::Small)
                        }))
                        .child(Label::new(self.label.clone()).color(TextColor::Muted)),
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

#[derive(Component, Clone)]
pub enum ListItem {
    Entry(ListEntry),
    Separator(ListSeparator),
    Header(ListSubHeader),
}

impl From<ListEntry> for ListItem {
    fn from(entry: ListEntry) -> Self {
        Self::Entry(entry)
    }
}

impl From<ListSeparator> for ListItem {
    fn from(entry: ListSeparator) -> Self {
        Self::Separator(entry)
    }
}

impl From<ListSubHeader> for ListItem {
    fn from(entry: ListSubHeader) -> Self {
        Self::Header(entry)
    }
}

impl ListItem {
    fn render<V: 'static>(self, view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
        match self {
            ListItem::Entry(entry) => div().child(entry.render(view, cx)),
            ListItem::Separator(separator) => div().child(separator.render(view, cx)),
            ListItem::Header(header) => div().child(header.render(view, cx)),
        }
    }

    pub fn new(label: Label) -> Self {
        Self::Entry(ListEntry::new(label))
    }

    pub fn as_entry(&mut self) -> Option<&mut ListEntry> {
        if let Self::Entry(entry) = self {
            Some(entry)
        } else {
            None
        }
    }
}

#[derive(Component)]
pub struct ListEntry {
    disabled: bool,
    // TODO: Reintroduce this
    // disclosure_control_style: DisclosureControlVisibility,
    indent_level: u32,
    label: Label,
    left_slot: Option<GraphicSlot>,
    overflow: OverflowStyle,
    size: ListEntrySize,
    toggle: Toggle,
    variant: ListItemVariant,
    on_click: Option<Box<dyn Action>>,
}

impl Clone for ListEntry {
    fn clone(&self) -> Self {
        Self {
            disabled: self.disabled,
            // TODO: Reintroduce this
            // disclosure_control_style: DisclosureControlVisibility,
            indent_level: self.indent_level,
            label: self.label.clone(),
            left_slot: self.left_slot.clone(),
            overflow: self.overflow,
            size: self.size,
            toggle: self.toggle,
            variant: self.variant,
            on_click: self.on_click.as_ref().map(|opt| opt.boxed_clone()),
        }
    }
}

impl ListEntry {
    pub fn new(label: Label) -> Self {
        Self {
            disabled: false,
            indent_level: 0,
            label,
            left_slot: None,
            overflow: OverflowStyle::Hidden,
            size: ListEntrySize::default(),
            toggle: Toggle::NotToggleable,
            variant: ListItemVariant::default(),
            on_click: Default::default(),
        }
    }

    pub fn action(mut self, action: impl Into<Box<dyn Action>>) -> Self {
        self.on_click = Some(action.into());
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

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
        let settings = user_settings(cx);

        let left_content = match self.left_slot.clone() {
            Some(GraphicSlot::Icon(i)) => Some(
                h_stack().child(
                    IconElement::new(i)
                        .size(IconSize::Small)
                        .color(TextColor::Muted),
                ),
            ),
            Some(GraphicSlot::Avatar(src)) => Some(h_stack().child(Avatar::new(src))),
            Some(GraphicSlot::PublicActor(src)) => Some(h_stack().child(Avatar::new(src))),
            None => None,
        };

        let sized_item = match self.size {
            ListEntrySize::Small => div().h_6(),
            ListEntrySize::Medium => div().h_7(),
        };
        div()
            .relative()
            .hover(|mut style| {
                style.background = Some(cx.theme().colors().editor_background.into());
                style
            })
            .on_mouse_down(gpui::MouseButton::Left, {
                let action = self.on_click.map(|action| action.boxed_clone());

                move |entry: &mut V, event, cx| {
                    if let Some(action) = action.as_ref() {
                        cx.dispatch_action(action.boxed_clone());
                    }
                }
            })
            .group("")
            .bg(cx.theme().colors().surface_background)
            // TODO: Add focus state
            // .when(self.state == InteractionState::Focused, |this| {
            //     this.border()
            //         .border_color(cx.theme().colors().border_focused)
            // })
            .child(
                sized_item
                    .when(self.variant == ListItemVariant::Inset, |this| this.px_2())
                    // .ml(rems(0.75 * self.indent_level as f32))
                    .children((0..self.indent_level).map(|_| {
                        div()
                            .w(*settings.list_indent_depth)
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
                    .child(self.label),
            )
    }
}

#[derive(Clone, Component)]
pub struct ListSeparator;

impl ListSeparator {
    pub fn new() -> Self {
        Self
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
        div().h_px().w_full().bg(cx.theme().colors().border_variant)
    }
}

#[derive(Component)]
pub struct List {
    items: Vec<ListItem>,
    /// Message to display when the list is empty
    /// Defaults to "No items"
    empty_message: SharedString,
    header: Option<ListHeader>,
    toggle: Toggle,
}

impl List {
    pub fn new(items: Vec<ListItem>) -> Self {
        Self {
            items,
            empty_message: "No items".into(),
            header: None,
            toggle: Toggle::NotToggleable,
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

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
        let list_content = match (self.items.is_empty(), self.toggle) {
            (false, _) => div().children(self.items),
            (true, Toggle::Toggled(false)) => div(),
            (true, _) => {
                div().child(Label::new(self.empty_message.clone()).color(TextColor::Muted))
            }
        };

        v_stack()
            .w_full()
            .py_1()
            .children(self.header.map(|header| header))
            .child(list_content)
    }
}
