use crate::prelude::InteractionState;
use crate::theme::theme;
use crate::ui::{icon, IconAsset, Label};
use gpui2::geometry::rems;
use gpui2::style::StyleHelpers;
use gpui2::{elements::div, IntoElement};
use gpui2::{Element, ParentElement, ViewContext};

#[derive(Element)]
pub struct ListItem {
    label: Label,
    left_icon: Option<IconAsset>,
    indent_level: f32,
    state: InteractionState,
}

pub fn list_item(label: Label) -> ListItem {
    ListItem {
        label,
        indent_level: 0.0,
        left_icon: None,
        state: InteractionState::default(),
    }
}

impl ListItem {
    pub fn indent_level(mut self, indent_level: f32) -> Self {
        self.indent_level = indent_level;
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

        let mut el = div()
            .h_7()
            .px_2()
            .ml(rems(0.75 * self.indent_level.clone()))
            .flex()
            .gap_2()
            .items_center();

        if self.left_icon.is_some() {
            el = el.child(icon(self.left_icon.clone().unwrap()))
        }

        el.child(self.label.clone())
    }
}
