use gpui::ClickEvent;
use ui::prelude::*;

// todo: This can easily get moved to the ui crate so it can be used elsewhere
#[derive(IntoElement, RegisterComponent)]
pub struct Callout {
    title: SharedString,
    message: Option<SharedString>,
    icon: Icon,
    cta_label: SharedString,
    cta_action: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
}

impl Callout {
    pub fn single_line(
        title: SharedString,
        icon: Icon,
        cta_label: SharedString,
        cta_action: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    ) -> Self {
        Self {
            title,
            message: None,
            icon,
            cta_label,
            cta_action,
        }
    }

    pub fn multi_line(
        title: SharedString,
        message: SharedString,
        icon: Icon,
        cta_label: SharedString,
        cta_action: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    ) -> Self {
        Self {
            title,
            message: Some(message),
            icon,
            cta_label,
            cta_action,
        }
    }
}

impl RenderOnce for Callout {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let line_height = window.line_height();

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
        ComponentScope::Agent
    }

    fn sort_name() -> &'static str {
        "AgentCallout"
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
                    "Reaching Free tier prompt limit soon".into(),
                    Icon::new(IconName::Warning)
                        .color(Color::Warning)
                        .size(IconSize::Small),
                    "Upgrade".into(),
                    Box::new(|_, _, _| {}),
                )
                .into_any_element(),
            )
            .width(px(580.)),
            single_example(
                "Multi Line",
                Callout::multi_line(
                    "Thread reached the token limit".into(),
                    "Start a new thread from a summary to continue the conversation.".into(),
                    Icon::new(IconName::X)
                        .color(Color::Error)
                        .size(IconSize::Small),
                    "Start New Thread".into(),
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
