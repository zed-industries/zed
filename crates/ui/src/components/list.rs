use gpui2::{
    elements::div,
    style::{StyleHelpers, Styleable},
    Element, IntoElement, ParentElement, ViewContext,
};

use crate::{
    h_stack, theme, token, v_stack, Avatar, DisclosureControlVisibility, Icon, IconAsset,
    IconColor, IconSize, InteractionState, Label, LabelColor, LabelSize, SystemColor, ToggleState,
};

#[derive(Element, Clone, Copy)]
pub struct ListSectionHeader {
    label: &'static str,
    left_icon: Option<IconAsset>,
    state: InteractionState,
    toggle: Option<ToggleState>,
}

impl ListSectionHeader {
    pub fn new(label: &'static str) -> Self {
        Self {
            label,
            left_icon: None,
            state: InteractionState::default(),
            toggle: None,
        }
    }

    pub fn set_toggle(mut self, toggle: ToggleState) -> Self {
        self.toggle = Some(toggle);
        self
    }

    pub fn left_icon(mut self, left_icon: Option<IconAsset>) -> Self {
        self.left_icon = left_icon;
        self
    }

    pub fn state(mut self, state: InteractionState) -> Self {
        self.state = state;
        self
    }

    fn disclosure_control(&self) -> Icon {
        Icon::new(if let Some(ToggleState::Toggled) = self.toggle {
            IconAsset::ChevronDown
        } else {
            IconAsset::ChevronRight
        })
        .color(IconColor::Muted)
        .size(IconSize::Small)
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        let system_color = SystemColor::new();
        let token = token();

        h_stack()
            .flex_1()
            .w_full()
            .fill(system_color.transparent)
            .hover()
            .fill(token.state_hover_background)
            .active()
            .fill(token.state_active_background)
            .relative()
            .py_1()
            .child(
                div()
                    .h_6()
                    .px_2()
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
                                Icon::new(i).color(IconColor::Muted).size(IconSize::Small)
                            }))
                            .child(
                                Label::new(self.label.clone())
                                    .color(LabelColor::Muted)
                                    .size(LabelSize::Small),
                            ),
                    )
                    .children(self.toggle.map(|_| self.disclosure_control())),
            )
    }
}

#[derive(Clone)]
pub enum LeftContent {
    Icon(IconAsset),
    Avatar(&'static str),
}

#[derive(Default, PartialEq, Copy, Clone)]
pub enum ListItemSize {
    #[default]
    Small,
    Medium,
}

#[derive(Element, Clone)]
pub struct ListItem {
    disclosure_control_style: DisclosureControlVisibility,
    indent_level: u32,
    label: Label,
    left_content: Option<LeftContent>,
    size: ListItemSize,
    state: InteractionState,
    toggle: Option<ToggleState>,
}

impl ListItem {
    pub fn new(label: Label) -> Self {
        Self {
            disclosure_control_style: DisclosureControlVisibility::default(),
            indent_level: 0,
            label,
            left_content: None,
            size: ListItemSize::default(),
            state: InteractionState::default(),
            toggle: None,
        }
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

    pub fn left_icon(mut self, left_icon: IconAsset) -> Self {
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

    pub fn size(mut self, size: ListItemSize) -> Self {
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

    fn disclosure_control<V: 'static>(
        &mut self,
        cx: &mut ViewContext<V>,
    ) -> Option<impl IntoElement<V>> {
        let theme = theme(cx);
        let token = token();

        let disclosure_control_icon = Icon::new(if let Some(ToggleState::Toggled) = self.toggle {
            IconAsset::ChevronDown
        } else {
            IconAsset::ChevronRight
        })
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

        let left_content = match self.left_content {
            Some(LeftContent::Icon(i)) => Some(h_stack().child(Icon::new(i).size(IconSize::Small))),
            Some(LeftContent::Avatar(src)) => Some(h_stack().child(Avatar::new(src))),
            None => None,
        };

        let sized_item = match self.size {
            ListItemSize::Small => div().h_6(),
            ListItemSize::Medium => div().h_7(),
        };

        div()
            .fill(system_color.transparent)
            .hover()
            .fill(token.state_hover_background)
            .active()
            .fill(token.state_active_background)
            .relative()
            .py_1()
            .child(
                sized_item
                    .px_2()
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

#[derive(Element)]
pub struct List {
    header: Option<ListSectionHeader>,
    items: Vec<ListItem>,
    empty_message: &'static str,
    toggle: Option<ToggleState>,
    // footer: Option<ListSectionFooter>,
}

impl List {
    pub fn new(items: Vec<ListItem>) -> Self {
        Self {
            header: None,
            items,
            empty_message: "No items",
            toggle: None,
        }
    }

    pub fn header(mut self, header: ListSectionHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn empty_message(mut self, empty_message: &'static str) -> Self {
        self.empty_message = empty_message;
        self
    }

    pub fn set_toggle(mut self, toggle: ToggleState) -> Self {
        self.toggle = Some(toggle);
        self
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        let token = token();

        let disclosure_control = match self.toggle {
            Some(ToggleState::NotToggled) => Some(Icon::new(IconAsset::ChevronRight)),
            Some(ToggleState::Toggled) => Some(Icon::new(IconAsset::ChevronDown)),
            None => None,
        };

        let list_items = if self.toggle == Some(ToggleState::Toggled) {
            div().children(self.items.iter().cloned())
        } else {
            div()
        };

        v_stack()
            .py_1()
            .children(self.header)
            .children(
                self.items
                    .is_empty()
                    .then(|| Label::new(self.empty_message).color(LabelColor::Muted)),
            )
            .child(list_items)
    }
}
