use gpui2::{
    elements::div,
    style::{StyleHelpers, Styleable},
    Element, IntoElement, ParentElement, ViewContext,
};

use crate::{
    h_stack, theme, token, v_stack, Avatar, DisclosureControlVisibility, Icon, IconAsset,
    InteractionState, Label, LabelColor, LabelSize, ToggleState,
};

#[derive(Element, Clone, Copy)]
pub struct ListSectionHeader {
    label: &'static str,
    left_icon: Option<IconAsset>,
    state: InteractionState,
    toggle: Option<ToggleState>,
}

pub fn list_section_header(label: &'static str) -> ListSectionHeader {
    ListSectionHeader {
        label,
        left_icon: None,
        state: InteractionState::default(),
        toggle: None,
    }
}

impl ListSectionHeader {
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

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        let token = token();

        let disclosure_control = match self.toggle {
            Some(ToggleState::NotToggled) => Some(div().child(Icon::new(IconAsset::ChevronRight))),
            Some(ToggleState::Toggled) => Some(div().child(Icon::new(IconAsset::ChevronDown))),
            None => Some(div()),
        };

        h_stack()
            .flex_1()
            .w_full()
            .fill(theme.middle.base.default.background)
            .hover()
            .fill(theme.middle.base.hovered.background)
            .active()
            .fill(theme.middle.base.pressed.background)
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
                            .children(self.left_icon.map(Icon::new))
                            .child(
                                Label::new(self.label.clone())
                                    .color(LabelColor::Muted)
                                    .size(LabelSize::Small),
                            ),
                    )
                    .children(disclosure_control),
            )
    }
}

#[derive(Clone)]
pub enum LeftContent {
    Icon(IconAsset),
    Avatar(&'static str),
}

#[derive(Element, Clone)]
pub struct ListItem {
    label: Label,
    left_content: Option<LeftContent>,
    indent_level: u32,
    state: InteractionState,
    disclosure_control_style: DisclosureControlVisibility,
    toggle: Option<ToggleState>,
}

pub fn list_item(label: Label) -> ListItem {
    ListItem {
        label,
        indent_level: 0,
        left_content: None,
        disclosure_control_style: DisclosureControlVisibility::default(),
        state: InteractionState::default(),
        toggle: None,
    }
}

impl ListItem {
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
        });

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

        let left_content = match self.left_content {
            Some(LeftContent::Icon(i)) => Some(div().child(Icon::new(i))),
            Some(LeftContent::Avatar(src)) => Some(div().child(Avatar::new(src))),
            None => None,
        };

        div()
            .fill(theme.middle.base.default.background)
            .hover()
            .fill(theme.middle.base.hovered.background)
            .active()
            .fill(theme.middle.base.pressed.background)
            .relative()
            .py_1()
            .child(
                div()
                    .h_6()
                    .px_2()
                    // .ml(rems(0.75 * self.indent_level as f32))
                    .children((0..self.indent_level).map(|_| {
                        div()
                            .w(token.list_indent_depth)
                            .h_full()
                            .flex()
                            .justify_center()
                            .child(
                                div()
                                    .ml_px()
                                    .w_px()
                                    .h_full()
                                    .fill(theme.middle.base.default.border),
                            )
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

pub fn list(items: Vec<ListItem>) -> List {
    List {
        header: None,
        items,
        empty_message: "No items",
        toggle: None,
    }
}

impl List {
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

        v_stack()
            .py_1()
            .children(self.header)
            .children(
                self.items
                    .is_empty()
                    .then(|| Label::new(self.empty_message).color(LabelColor::Muted)),
            )
            .children(self.items.iter().cloned())
    }
}
