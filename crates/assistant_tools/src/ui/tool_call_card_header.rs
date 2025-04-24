use gpui::{Animation, AnimationExt, App, IntoElement, pulsating_between};
use std::time::Duration;
use ui::{Tooltip, prelude::*};

/// A reusable header component for tool call cards.
#[derive(IntoElement)]
pub struct ToolCallCardHeader {
    icon: IconName,
    primary_text: SharedString,
    secondary_text: Option<SharedString>,
    is_loading: bool,
    error: Option<String>,
}

impl ToolCallCardHeader {
    pub fn new(icon: IconName, primary_text: impl Into<SharedString>) -> Self {
        Self {
            icon,
            primary_text: primary_text.into(),
            secondary_text: None,
            is_loading: false,
            error: None,
        }
    }

    pub fn with_secondary_text(mut self, text: impl Into<SharedString>) -> Self {
        self.secondary_text = Some(text.into());
        self
    }

    pub fn loading(mut self) -> Self {
        self.is_loading = true;
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }
}

impl RenderOnce for ToolCallCardHeader {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let font_size = rems(0.8125);
        let secondary_text = self.secondary_text;

        h_flex()
            .id("tool-label-container")
            .gap_1p5()
            .max_w_full()
            .overflow_x_scroll()
            .opacity(0.8)
            .child(
                h_flex().h(window.line_height()).justify_center().child(
                    Icon::new(self.icon)
                        .size(IconSize::XSmall)
                        .color(Color::Muted),
                ),
            )
            .child(
                h_flex()
                    .h(window.line_height())
                    .gap_1p5()
                    .text_size(font_size)
                    .map(|this| {
                        if let Some(error) = &self.error {
                            this.child(format!("{} failed", self.primary_text)).child(
                                IconButton::new("error_info", IconName::Warning)
                                    .shape(ui::IconButtonShape::Square)
                                    .icon_size(IconSize::XSmall)
                                    .icon_color(Color::Warning)
                                    .tooltip(Tooltip::text(error.clone())),
                            )
                        } else {
                            this.child(self.primary_text.clone())
                        }
                    })
                    .when_some(secondary_text, |this, secondary_text| {
                        this.child(
                            div()
                                .size(px(3.))
                                .rounded_full()
                                .bg(cx.theme().colors().text),
                        )
                        .child(div().text_size(font_size).child(secondary_text.clone()))
                    })
                    .with_animation(
                        "loading-label",
                        Animation::new(Duration::from_secs(2))
                            .repeat()
                            .with_easing(pulsating_between(0.6, 1.)),
                        move |this, delta| {
                            if self.is_loading {
                                this.opacity(delta)
                            } else {
                                this
                            }
                        },
                    ),
            )
    }
}
