use crate::prelude::{DisclosureControlVisibility, InteractionState, ToggleState};
use crate::theme::theme;
use crate::tokens::token;
use crate::{icon, IconAsset, Label};
use gpui2::style::{StyleHelpers, Styleable};
use gpui2::{elements::div, IntoElement};
use gpui2::{Element, ParentElement, ViewContext};

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
            Some(ToggleState::NotToggled) => Some(div().child(icon(IconAsset::ChevronRight))),
            Some(ToggleState::Toggled) => Some(div().child(icon(IconAsset::ChevronDown))),
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
                    .children(self.left_icon.map(|i| icon(i)))
                    .child(self.label.clone()),
            )
    }
}
