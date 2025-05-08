use ui::prelude::*;

#[derive(IntoElement)]
pub struct ProfileModalHeader {
    label: SharedString,
    icon: Option<IconName>,
}

impl ProfileModalHeader {
    pub fn new(label: impl Into<SharedString>, icon: Option<IconName>) -> Self {
        Self {
            label: label.into(),
            icon,
        }
    }
}

impl RenderOnce for ProfileModalHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let mut container = h_flex()
            .w_full()
            .px(DynamicSpacing::Base12.rems(cx))
            .pt(DynamicSpacing::Base08.rems(cx))
            .pb(DynamicSpacing::Base04.rems(cx))
            .rounded_t_sm()
            .gap_1p5();

        if let Some(icon) = self.icon {
            container = container.child(Icon::new(icon).size(IconSize::XSmall).color(Color::Muted));
        }

        container.child(
            h_flex().gap_1().overflow_x_hidden().child(
                div()
                    .max_w_96()
                    .overflow_x_hidden()
                    .text_ellipsis()
                    .child(Headline::new(self.label).size(HeadlineSize::XSmall)),
            ),
        )
    }
}
