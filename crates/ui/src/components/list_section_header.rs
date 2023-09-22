use crate::prelude::{InteractionState, ToggleState};
use crate::theme::theme;
use crate::tokens::token;
use crate::{icon, label, IconAsset, LabelColor, LabelSize};
use gpui2::style::{StyleHelpers, Styleable};
use gpui2::{elements::div, IntoElement};
use gpui2::{Element, ParentElement, ViewContext};

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
            Some(ToggleState::NotToggled) => Some(div().child(icon(IconAsset::ChevronRight))),
            Some(ToggleState::Toggled) => Some(div().child(icon(IconAsset::ChevronDown))),
            None => Some(div()),
        };

        div()
            .flex()
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
                            .children(self.left_icon.map(|i| icon(i)))
                            .child(
                                label(self.label.clone())
                                    .color(LabelColor::Muted)
                                    .size(LabelSize::Small),
                            ),
                    )
                    .children(disclosure_control),
            )
    }
}
