use gpui2::{
    elements::div,
    style::{StyleHelpers, Styleable},
    Element, IntoElement, ParentElement, ViewContext,
};

use crate::{
    h_stack, theme, token, v_stack, DisclosureControlVisibility, Icon, IconAsset, InteractionState,
    Label, LabelColor, LabelSize, ToggleState,
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

#[derive(Element, Clone)]
pub struct ListItem {
    label: Label,
    left_icon: Option<IconAsset>,
    indent_level: u32,
    state: InteractionState,
    disclosure_control_style: DisclosureControlVisibility,
    toggle: Option<ToggleState>,
}

pub fn list_item(label: Label) -> ListItem {
    ListItem {
        label,
        indent_level: 0,
        left_icon: None,
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

    pub fn left_icon(mut self, left_icon: Option<IconAsset>) -> Self {
        self.left_icon = left_icon;
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

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        let token = token();
        let mut disclosure_control = match self.toggle {
            Some(ToggleState::NotToggled) => Some(div().child(Icon::new(IconAsset::ChevronRight))),
            Some(ToggleState::Toggled) => Some(div().child(Icon::new(IconAsset::ChevronDown))),
            None => Some(div()),
        };

        match self.disclosure_control_style {
            DisclosureControlVisibility::OnHover => {
                disclosure_control =
                    disclosure_control.map(|c| div().absolute().neg_left_5().child(c));
            }
            DisclosureControlVisibility::Always => {}
        }

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
                    .children(disclosure_control)
                    .children(self.left_icon.map(Icon::new))
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
