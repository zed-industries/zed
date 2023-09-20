use crate::prelude::{InteractionState, ToggleState};
use crate::theme::theme;
use crate::ui::{icon, IconAsset, Label};
use gpui2::geometry::rems;
use gpui2::style::{StyleHelpers, Styleable};
use gpui2::{elements::div, IntoElement};
use gpui2::{Element, ParentElement, ViewContext};

#[derive(Element)]
pub struct ListItem {
    label: Label,
    left_icon: Option<IconAsset>,
    indent_level: u32,
    state: InteractionState,
    toggle: Option<ToggleState>,
}

pub fn list_item(label: Label) -> ListItem {
    ListItem {
        label,
        indent_level: 0,
        left_icon: None,
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

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div()
            .fill(theme.middle.base.default.background)
            .hover()
            .fill(theme.middle.base.hovered.background)
            .active()
            .fill(theme.middle.base.pressed.background)
            .relative()
            .child(
                div()
                    .h_7()
                    .px_2()
                    // .ml(rems(0.75 * self.indent_level as f32))
                    .children((0..self.indent_level).map(|_| {
                        div().w(rems(0.75)).h_full().flex().justify_center().child(
                            div()
                                .w_px()
                                .h_full()
                                .fill(theme.middle.base.default.border)
                                .hover()
                                .fill(theme.middle.warning.default.border)
                                .active()
                                .fill(theme.middle.negative.default.border),
                        )
                    }))
                    .flex()
                    .gap_2()
                    .items_center()
                    .children(match self.toggle {
                        Some(ToggleState::NotToggled) => Some(icon(IconAsset::ChevronRight)),
                        Some(ToggleState::Toggled) => Some(icon(IconAsset::ChevronDown)),
                        None => None,
                    })
                    .children(self.left_icon.map(|i| icon(i)))
                    .child(self.label.clone()),
            )
    }
}
