use std::sync::Arc;

use client::{Client, UserStore};
use gpui::{Entity, IntoElement, ParentElement};
use ui::{Checkbox, prelude::*};

use crate::ZedAiOnboarding;

pub struct EditPredictionOnboarding {
    user_store: Entity<UserStore>,
    client: Arc<Client>,
    copilot_is_configured: bool,
    continue_with_zed_ai: Arc<dyn Fn(&mut Window, &mut App)>,
    continue_with_copilot: Arc<dyn Fn(&mut Window, &mut App)>,
    data_collection_opted_in: bool,
    data_collection_expanded: bool,
}

impl EditPredictionOnboarding {
    pub fn new(
        user_store: Entity<UserStore>,
        client: Arc<Client>,
        copilot_is_configured: bool,
        continue_with_zed_ai: Arc<dyn Fn(&mut Window, &mut App)>,
        continue_with_copilot: Arc<dyn Fn(&mut Window, &mut App)>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            user_store,
            copilot_is_configured,
            client,
            continue_with_zed_ai,
            continue_with_copilot,
            data_collection_expanded: false,
            data_collection_opted_in: false,
        }
    }

    fn render_data_collection_explanation(&self, cx: &Context<Self>) -> impl IntoElement {
        fn label_item(label_text: impl Into<SharedString>) -> impl Element {
            Label::new(label_text).color(Color::Muted).into_element()
        }

        fn info_item(label_text: impl Into<SharedString>) -> impl Element {
            h_flex()
                .items_start()
                .gap_2()
                .child(
                    div()
                        .mt_1p5()
                        .child(Icon::new(IconName::Check).size(IconSize::XSmall)),
                )
                .child(div().w_full().child(label_item(label_text)))
        }

        fn multiline_info_item<E1: Into<SharedString>, E2: IntoElement>(
            first_line: E1,
            second_line: E2,
        ) -> impl Element {
            v_flex()
                .child(info_item(first_line))
                .child(div().pl_5().child(second_line))
        }

        v_flex()
            .mt_2()
            .p_2()
            .rounded_sm()
            .bg(cx.theme().colors().editor_background.opacity(0.5))
            .border_1()
            .border_color(cx.theme().colors().border_variant)
            .child(
                div().child(
                    Label::new("To improve edit predictions, please consider contributing to our open dataset based on your interactions within open source repositories.")
                        .mb_1()
                )
            )
            .child(info_item(
                "We collect data exclusively from open source projects.",
            ))
            .child(info_item(
                "Zed automatically detects if your project is open source.",
            ))
            .child(info_item("Toggle participation at any time via the status bar menu."))
            .child(multiline_info_item(
                "If turned on, this setting applies for all open source repositories",
                label_item("you open in Zed.")
            ))
            .child(multiline_info_item(
                "Files with sensitive data, like `.env`, are excluded by default",
                h_flex()
                    .w_full()
                    .flex_wrap()
                    .child(label_item("via the"))
                    .child(
                        Button::new("doc-link", "disabled_globs"),
                    )
                    .child(label_item("setting.")),
            ))
    }
}

impl Render for EditPredictionOnboarding {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let (description_label, button_label) = if self.copilot_is_configured {
            (
                "Alternatively, you can continue to use GitHub Copilt as that's already set up.",
                "Use Copilot",
            )
        } else {
            (
                "Alternatively, you can use GitHub Copilot as your edit prediction provider.",
                "Configure Copilot",
            )
        };

        let github_copilot = v_flex().gap_1().child(Label::new(description_label)).child(
            Button::new("configure-copilot", button_label)
                .full_width()
                .style(ButtonStyle::Outlined)
                .on_click({
                    let callback = self.continue_with_copilot.clone();
                    move |_, window, cx| callback(window, cx)
                }),
        );

        v_flex()
            .gap_2()
            .child(ZedAiOnboarding::new(
                self.client.clone(),
                &self.user_store,
                self.continue_with_zed_ai.clone(),
                cx,
            ))
            .child(
                v_flex()
                    .child(
                        h_flex()
                            .flex_wrap()
                            .child(
                                Checkbox::new(
                                    "training-data-checkbox",
                                    self.data_collection_opted_in.into(),
                                )
                                .label("Contribute to the open dataset when editing open source.")
                                .fill(),
                            )
                            .child(
                                Button::new("learn-more", "Learn More")
                                    .icon(if self.data_collection_expanded {
                                        IconName::ChevronUp
                                    } else {
                                        IconName::ChevronDown
                                    })
                                    .icon_size(IconSize::Indicator)
                                    .icon_color(Color::Muted),
                            ),
                    )
                    .when(self.data_collection_expanded, |element| {
                        element.child(self.render_data_collection_explanation(cx))
                    }),
            )
            .child(ui::Divider::horizontal())
            .child(github_copilot)
    }
}
