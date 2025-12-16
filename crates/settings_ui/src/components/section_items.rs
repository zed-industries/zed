use gpui::{IntoElement, ParentElement, Styled};
use ui::{Divider, DividerColor, prelude::*};

#[derive(IntoElement)]
pub struct SettingsSectionHeader {
    icon: Option<IconName>,
    label: SharedString,
    no_padding: bool,
}

impl SettingsSectionHeader {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            no_padding: false,
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn no_padding(mut self, no_padding: bool) -> Self {
        self.no_padding = no_padding;
        self
    }
}

impl RenderOnce for SettingsSectionHeader {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let label = Label::new(self.label)
            .size(LabelSize::Small)
            .color(Color::Muted)
            .buffer_font(cx);

        v_flex()
            .w_full()
            .when(!self.no_padding, |this| this.px_8())
            .gap_1p5()
            .map(|this| {
                if self.icon.is_some() {
                    this.child(
                        h_flex()
                            .gap_1p5()
                            .child(Icon::new(self.icon.unwrap()).color(Color::Muted))
                            .child(label),
                    )
                } else {
                    this.child(label)
                }
            })
            .child(Divider::horizontal().color(DividerColor::BorderFaded))
    }
}
