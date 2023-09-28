use gpui2::elements::div::Div;
use gpui2::{Hsla, WindowContext};

use crate::prelude::*;
use crate::{
    h_stack, theme, token, v_stack, Avatar, DisclosureControlVisibility, Icon, IconColor,
    IconElement, IconSize, InteractionState, Label, LabelColor, LabelSize, SystemColor,
    ToggleState,
};

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub enum ListItemVariant {
    /// The list item extends to the far left and right of the list.
    #[default]
    FullWidth,
    Inset,
}

#[derive(Element, Clone, Copy)]
pub struct ListHeader {
    label: &'static str,
    left_icon: Option<Icon>,
    variant: ListItemVariant,
    state: InteractionState,
    toggleable: Toggleable,
}

impl ListHeader {
    pub fn new(label: &'static str) -> Self {
        Self {
            label,
            left_icon: None,
            variant: ListItemVariant::default(),
            state: InteractionState::default(),
            toggleable: Toggleable::default(),
        }
    }

    pub fn set_toggle(mut self, toggle: ToggleState) -> Self {
        self.toggleable = toggle.into();
        self
    }

    pub fn set_toggleable(mut self, toggleable: Toggleable) -> Self {
        self.toggleable = toggleable;
        self
    }

    pub fn left_icon(mut self, left_icon: Option<Icon>) -> Self {
        self.left_icon = left_icon;
        self
    }

    pub fn state(mut self, state: InteractionState) -> Self {
        self.state = state;
        self
    }

    fn disclosure_control<V: 'static>(&self) -> Div<V> {
        let is_toggleable = self.toggleable != Toggleable::NotToggleable;
        let is_toggled = Toggleable::is_toggled(&self.toggleable);

        match (is_toggleable, is_toggled) {
            (false, _) => div(),
            (_, true) => div().child(IconElement::new(Icon::ChevronRight).color(IconColor::Muted)),
            (_, false) => div().child(IconElement::new(Icon::ChevronDown).size(IconSize::Small)),
        }
    }

    fn background_color(&self, cx: &WindowContext) -> Hsla {
        let theme = theme(cx);
        let system_color = SystemColor::new();

        match self.state {
            InteractionState::Hovered => theme.lowest.base.hovered.background,
            InteractionState::Active => theme.lowest.base.pressed.background,
            InteractionState::Enabled => theme.lowest.on.default.background,
            _ => system_color.transparent,
        }
    }

    fn label_color(&self) -> LabelColor {
        match self.state {
            InteractionState::Disabled => LabelColor::Disabled,
            _ => Default::default(),
        }
    }

    fn icon_color(&self) -> IconColor {
        match self.state {
            InteractionState::Disabled => IconColor::Disabled,
            _ => Default::default(),
        }
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        let token = token();
        let system_color = SystemColor::new();
        let background_color = self.background_color(cx);

        let is_toggleable = self.toggleable != Toggleable::NotToggleable;
        let is_toggled = Toggleable::is_toggled(&self.toggleable);

        let disclosure_control = self.disclosure_control();

        h_stack()
            .flex_1()
            .w_full()
            .fill(background_color)
            .when(self.state == InteractionState::Focused, |this| {
                this.border()
                    .border_color(theme.lowest.accent.default.border)
            })
            .relative()
            .py_1()
            .child(
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
                                    .color(IconColor::Muted)
                                    .size(IconSize::Small)
                            }))
                            .child(
                                Label::new(self.label.clone())
                                    .color(LabelColor::Muted)
                                    .size(LabelSize::Small),
                            ),
                    )
                    .child(disclosure_control),
            )
    }
}

#[derive(Element, Clone, Copy)]
pub struct ListSubHeader {
    label: &'static str,
    left_icon: Option<Icon>,
    variant: ListItemVariant,
}

impl ListSubHeader {
    pub fn new(label: &'static str) -> Self {
        Self {
            label,
            left_icon: None,
            variant: ListItemVariant::default(),
        }
    }

    pub fn left_icon(mut self, left_icon: Option<Icon>) -> Self {
        self.left_icon = left_icon;
        self
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        let token = token();

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
                                .color(IconColor::Muted)
                                .size(IconSize::Small)
                        }))
                        .child(
                            Label::new(self.label.clone())
                                .color(LabelColor::Muted)
                                .size(LabelSize::Small),
                        ),
                ),
        )
    }
}

#[derive(Clone)]
pub enum LeftContent {
    Icon(Icon),
    Avatar(&'static str),
}

#[derive(Default, PartialEq, Copy, Clone)]
pub enum ListEntrySize {
    #[default]
    Small,
    Medium,
}

#[derive(Clone, Element)]
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
    fn render<V: 'static>(&mut self, v: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        match self {
            ListItem::Entry(entry) => div().child(entry.render(v, cx)),
            ListItem::Separator(separator) => div().child(separator.render(v, cx)),
            ListItem::Header(header) => div().child(header.render(v, cx)),
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

#[derive(Element, Clone)]
pub struct ListEntry {
    disclosure_control_style: DisclosureControlVisibility,
    indent_level: u32,
    label: Label,
    left_content: Option<LeftContent>,
    variant: ListItemVariant,
    size: ListEntrySize,
    state: InteractionState,
    toggle: Option<ToggleState>,
}

impl ListEntry {
    pub fn new(label: Label) -> Self {
        Self {
            disclosure_control_style: DisclosureControlVisibility::default(),
            indent_level: 0,
            label,
            variant: ListItemVariant::default(),
            left_content: None,
            size: ListEntrySize::default(),
            state: InteractionState::default(),
            toggle: None,
        }
    }
    pub fn variant(mut self, variant: ListItemVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn indent_level(mut self, indent_level: u32) -> Self {
        self.indent_level = indent_level;
        self
    }

    pub fn set_toggle(mut self, toggle: ToggleState) -> Self {
        self.toggle = Some(toggle);
        self
    }

    pub fn left_content(mut self, left_content: LeftContent) -> Self {
        self.left_content = Some(left_content);
        self
    }

    pub fn left_icon(mut self, left_icon: Icon) -> Self {
        self.left_content = Some(LeftContent::Icon(left_icon));
        self
    }

    pub fn left_avatar(mut self, left_avatar: &'static str) -> Self {
        self.left_content = Some(LeftContent::Avatar(left_avatar));
        self
    }

    pub fn state(mut self, state: InteractionState) -> Self {
        self.state = state;
        self
    }

    pub fn size(mut self, size: ListEntrySize) -> Self {
        self.size = size;
        self
    }

    pub fn disclosure_control_style(
        mut self,
        disclosure_control_style: DisclosureControlVisibility,
    ) -> Self {
        self.disclosure_control_style = disclosure_control_style;
        self
    }

    fn background_color(&self, cx: &WindowContext) -> Hsla {
        let theme = theme(cx);
        let system_color = SystemColor::new();

        match self.state {
            InteractionState::Hovered => theme.lowest.base.hovered.background,
            InteractionState::Active => theme.lowest.base.pressed.background,
            InteractionState::Enabled => theme.lowest.on.default.background,
            _ => system_color.transparent,
        }
    }

    fn label_color(&self) -> LabelColor {
        match self.state {
            InteractionState::Disabled => LabelColor::Disabled,
            _ => Default::default(),
        }
    }

    fn icon_color(&self) -> IconColor {
        match self.state {
            InteractionState::Disabled => IconColor::Disabled,
            _ => Default::default(),
        }
    }

    fn disclosure_control<V: 'static>(
        &mut self,
        cx: &mut ViewContext<V>,
    ) -> Option<impl IntoElement<V>> {
        let theme = theme(cx);
        let token = token();

        let disclosure_control_icon = if let Some(ToggleState::Toggled) = self.toggle {
            IconElement::new(Icon::ChevronDown)
        } else {
            IconElement::new(Icon::ChevronRight)
        }
        .color(IconColor::Muted)
        .size(IconSize::Small);

        match (self.toggle, self.disclosure_control_style) {
            (Some(_), DisclosureControlVisibility::OnHover) => {
                Some(div().absolute().neg_left_5().child(disclosure_control_icon))
            }
            (Some(_), DisclosureControlVisibility::Always) => {
                Some(div().child(disclosure_control_icon))
            }
            (None, _) => None,
        }
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        let token = token();
        let system_color = SystemColor::new();
        let background_color = self.background_color(cx);

        let left_content = match self.left_content {
            Some(LeftContent::Icon(i)) => {
                Some(h_stack().child(IconElement::new(i).size(IconSize::Small)))
            }
            Some(LeftContent::Avatar(src)) => Some(h_stack().child(Avatar::new(src))),
            None => None,
        };

        let sized_item = match self.size {
            ListEntrySize::Small => div().h_6(),
            ListEntrySize::Medium => div().h_7(),
        };

        div()
            .fill(background_color)
            .when(self.state == InteractionState::Focused, |this| {
                this.border()
                    .border_color(theme.lowest.accent.default.border)
            })
            .relative()
            .py_1()
            .child(
                sized_item
                    .when(self.variant == ListItemVariant::Inset, |this| this.px_2())
                    // .ml(rems(0.75 * self.indent_level as f32))
                    .children((0..self.indent_level).map(|_| {
                        div()
                            .w(token.list_indent_depth)
                            .h_full()
                            .flex()
                            .justify_center()
                            .child(h_stack().child(div().w_px().h_full()).child(
                                div().w_px().h_full().fill(theme.middle.base.default.border),
                            ))
                    }))
                    .flex()
                    .gap_1()
                    .items_center()
                    .relative()
                    .children(self.disclosure_control(cx))
                    .children(left_content)
                    .child(self.label.clone()),
            )
    }
}

#[derive(Clone, Default, Element)]
pub struct ListSeparator;

impl ListSeparator {
    pub fn new() -> Self {
        Self::default()
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div().h_px().w_full().fill(theme.lowest.base.default.border)
    }
}

#[derive(Element)]
pub struct List {
    items: Vec<ListItem>,
    empty_message: &'static str,
    header: Option<ListHeader>,
    toggleable: Toggleable,
}

impl List {
    pub fn new(items: Vec<ListItem>) -> Self {
        Self {
            items,
            empty_message: "No items",
            header: None,
            toggleable: Toggleable::default(),
        }
    }

    pub fn empty_message(mut self, empty_message: &'static str) -> Self {
        self.empty_message = empty_message;
        self
    }

    pub fn header(mut self, header: ListHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn set_toggle(mut self, toggle: ToggleState) -> Self {
        self.toggleable = toggle.into();
        self
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        let token = token();
        let is_toggleable = self.toggleable != Toggleable::NotToggleable;
        let is_toggled = Toggleable::is_toggled(&self.toggleable);

        let disclosure_control = if is_toggleable {
            IconElement::new(Icon::ChevronRight)
        } else {
            IconElement::new(Icon::ChevronDown)
        };

        let list_content = match (self.items.is_empty(), is_toggled) {
            (_, false) => div(),
            (false, _) => div().children(self.items.iter().cloned()),
            (true, _) => div().child(Label::new(self.empty_message).color(LabelColor::Muted)),
        };

        v_stack()
            .py_1()
            .children(
                self.header
                    .clone()
                    .map(|header| header.set_toggleable(self.toggleable)),
            )
            .child(list_content)
    }
}
