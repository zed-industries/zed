use gpui::ClickEvent;

use crate::prelude::*;

#[derive(IntoElement, RegisterComponent)]
pub struct Callout {
    title: SharedString,
    message: Option<SharedString>,
    icon: Icon,
    cta_label: SharedString,
    cta_action: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    line_height: Option<Pixels>,
}

impl Callout {
    pub fn single_line(
        title: impl Into<SharedString>,
        icon: Icon,
        cta_label: impl Into<SharedString>,
        cta_action: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    ) -> Self {
        Self {
            title: title.into(),
            message: None,
            icon,
            cta_label: cta_label.into(),
            cta_action,
            line_height: None,
        }
    }

    pub fn multi_line(
        title: impl Into<SharedString>,
        message: impl Into<SharedString>,
        icon: Icon,
        cta_label: impl Into<SharedString>,
        cta_action: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    ) -> Self {
        Self {
            title: title.into(),
            message: Some(message.into()),
            icon,
            cta_label: cta_label.into(),
            cta_action,
            line_height: None,
        }
    }

    pub fn line_height(mut self, line_height: Pixels) -> Self {
        self.line_height = Some(line_height);
        self
    }
}

impl RenderOnce for Callout {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let line_height = self.line_height.unwrap_or(window.line_height());

        h_flex()
            .p_2()
            .gap_2()
            .w_full()
            .items_center()
            .justify_between()
            .bg(cx.theme().colors().panel_background)
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .overflow_x_hidden()
            .child(
                h_flex()
                    .flex_shrink()
                    .overflow_hidden()
                    .gap_2()
                    .items_start()
                    .child(
                        h_flex()
                            .h(line_height)
                            .items_center()
                            .justify_center()
                            .child(self.icon),
                    )
                    .child(
                        v_flex()
                            .flex_shrink()
                            .overflow_hidden()
                            .child(
                                h_flex()
                                    .h(line_height)
                                    .items_center()
                                    .child(Label::new(self.title).size(LabelSize::Small)),
                            )
                            .when_some(self.message, |this, message| {
                                this.child(
                                    div()
                                        .w_full()
                                        .flex_1()
                                        .child(message)
                                        .text_ui_sm(cx)
                                        .text_color(cx.theme().colors().text_muted),
                                )
                            }),
                    ),
            )
            .child(
                div().flex_none().child(
                    Button::new("cta", self.cta_label)
                        .on_click(self.cta_action)
                        .style(ButtonStyle::Filled)
                        .label_size(LabelSize::Small),
                ),
            )
    }
}

impl Component for Callout {
    fn scope() -> ComponentScope {
        ComponentScope::Notification
    }

    fn description() -> Option<&'static str> {
        Some(
            "Used to display a callout for situations where the user needs to know some information, and likely make a decision. This might be a thread running out of tokens, or running out of prompts on a plan and needing to upgrade.",
        )
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        let callout_examples = vec![
            single_example(
                "Single Line",
                Callout::single_line(
                    "Your settings contain deprecated values, please update them.",
                    Icon::new(IconName::Warning)
                        .color(Color::Warning)
                        .size(IconSize::Small),
                    "Backup & Update",
                    Box::new(|_, _, _| {}),
                )
                .into_any_element(),
            )
            .width(px(580.)),
            single_example(
                "Multi Line",
                Callout::multi_line(
                    "Thread reached the token limit",
                    "Start a new thread from a summary to continue the conversation.",
                    Icon::new(IconName::X)
                        .color(Color::Error)
                        .size(IconSize::Small),
                    "Start New Thread",
                    Box::new(|_, _, _| {}),
                )
                .into_any_element(),
            )
            .width(px(580.)),
        ];

        Some(
            example_group(callout_examples)
                .vertical()
                .into_any_element(),
        )
    }
}
