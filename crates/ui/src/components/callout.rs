use gpui::{AnyElement, Hsla};

use crate::prelude::*;

/// A callout component for displaying important information that requires user attention.
///
/// # Usage Example
///
/// ```
/// use ui::{Callout};
///
/// Callout::new()
///     .icon(Icon::new(IconName::Warning).color(Color::Warning))
///     .title(Label::new("Be aware of your subscription!"))
///     .description(Label::new("Your subscription is about to expire. Renew now!"))
///     .primary_action(Button::new("renew", "Renew Now"))
///     .secondary_action(Button::new("remind", "Remind Me Later"))
/// ```
///
#[derive(IntoElement, RegisterComponent)]
pub struct Callout {
    icon: Option<Icon>,
    title: Option<SharedString>,
    description: Option<SharedString>,
    primary_action: Option<AnyElement>,
    secondary_action: Option<AnyElement>,
    tertiary_action: Option<AnyElement>,
    line_height: Option<Pixels>,
    bg_color: Option<Hsla>,
}

impl Callout {
    /// Creates a new `Callout` component with default styling.
    pub fn new() -> Self {
        Self {
            icon: None,
            title: None,
            description: None,
            primary_action: None,
            secondary_action: None,
            tertiary_action: None,
            line_height: None,
            bg_color: None,
        }
    }

    /// Sets the icon to display in the callout.
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets the title of the callout.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the description of the callout.
    /// The description can be single or multi-line text.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the primary call-to-action button.
    pub fn primary_action(mut self, action: impl IntoElement) -> Self {
        self.primary_action = Some(action.into_any_element());
        self
    }

    /// Sets an optional secondary call-to-action button.
    pub fn secondary_action(mut self, action: impl IntoElement) -> Self {
        self.secondary_action = Some(action.into_any_element());
        self
    }

    /// Sets an optional tertiary call-to-action button.
    pub fn tertiary_action(mut self, action: impl IntoElement) -> Self {
        self.tertiary_action = Some(action.into_any_element());
        self
    }

    /// Sets a custom line height for the callout content.
    pub fn line_height(mut self, line_height: Pixels) -> Self {
        self.line_height = Some(line_height);
        self
    }

    /// Sets a custom background color for the callout content.
    pub fn bg_color(mut self, color: Hsla) -> Self {
        self.bg_color = Some(color);
        self
    }
}

impl RenderOnce for Callout {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let line_height = self.line_height.unwrap_or(window.line_height());
        let bg_color = self
            .bg_color
            .unwrap_or(cx.theme().colors().panel_background);
        let has_actions = self.primary_action.is_some()
            || self.secondary_action.is_some()
            || self.tertiary_action.is_some();

        h_flex()
            .p_2()
            .gap_2()
            .items_start()
            .bg(bg_color)
            .overflow_x_hidden()
            .when_some(self.icon, |this, icon| {
                this.child(h_flex().h(line_height).justify_center().child(icon))
            })
            .child(
                v_flex()
                    .min_w_0()
                    .w_full()
                    .child(
                        h_flex()
                            .h(line_height)
                            .w_full()
                            .gap_1()
                            .justify_between()
                            .when_some(self.title, |this, title| {
                                this.child(h_flex().child(Label::new(title).size(LabelSize::Small)))
                            })
                            .when(has_actions, |this| {
                                this.child(
                                    h_flex()
                                        .gap_0p5()
                                        .when_some(self.tertiary_action, |this, action| {
                                            this.child(action)
                                        })
                                        .when_some(self.secondary_action, |this, action| {
                                            this.child(action)
                                        })
                                        .when_some(self.primary_action, |this, action| {
                                            this.child(action)
                                        }),
                                )
                            }),
                    )
                    .when_some(self.description, |this, description| {
                        this.child(
                            div()
                                .w_full()
                                .flex_1()
                                .text_ui_sm(cx)
                                .text_color(cx.theme().colors().text_muted)
                                .child(description),
                        )
                    }),
            )
    }
}

impl Component for Callout {
    fn scope() -> ComponentScope {
        ComponentScope::DataDisplay
    }

    fn description() -> Option<&'static str> {
        Some(
            "Used to display a callout for situations where the user needs to know some information, and likely make a decision. This might be a thread running out of tokens, or running out of prompts on a plan and needing to upgrade.",
        )
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        let callout_examples = vec![
            single_example(
                "Simple with Title Only",
                Callout::new()
                    .icon(
                        Icon::new(IconName::Info)
                            .color(Color::Accent)
                            .size(IconSize::Small),
                    )
                    .title("System maintenance scheduled for tonight")
                    .primary_action(Button::new("got-it", "Got it").label_size(LabelSize::Small))
                    .into_any_element(),
            )
            .width(px(580.)),
            single_example(
                "With Title and Description",
                Callout::new()
                    .icon(
                        Icon::new(IconName::Warning)
                            .color(Color::Warning)
                            .size(IconSize::Small),
                    )
                    .title("Your settings contain deprecated values")
                    .description(
                        "We'll backup your current settings and update them to the new format.",
                    )
                    .primary_action(
                        Button::new("update", "Backup & Update").label_size(LabelSize::Small),
                    )
                    .secondary_action(
                        Button::new("dismiss", "Dismiss").label_size(LabelSize::Small),
                    )
                    .into_any_element(),
            )
            .width(px(580.)),
            single_example(
                "Error with Multiple Actions",
                Callout::new()
                    .icon(
                        Icon::new(IconName::X)
                            .color(Color::Error)
                            .size(IconSize::Small),
                    )
                    .title("Thread reached the token limit")
                    .description("Start a new thread from a summary to continue the conversation.")
                    .primary_action(
                        Button::new("new-thread", "Start New Thread").label_size(LabelSize::Small),
                    )
                    .secondary_action(
                        Button::new("view-summary", "View Summary").label_size(LabelSize::Small),
                    )
                    .into_any_element(),
            )
            .width(px(580.)),
            single_example(
                "Multi-line Description",
                Callout::new()
                    .icon(
                        Icon::new(IconName::Sparkle)
                            .color(Color::Accent)
                            .size(IconSize::Small),
                    )
                    .title("Upgrade to Pro")
                    .description("• Unlimited threads\n• Priority support\n• Advanced analytics")
                    .primary_action(
                        Button::new("upgrade", "Upgrade Now").label_size(LabelSize::Small),
                    )
                    .secondary_action(
                        Button::new("learn-more", "Learn More").label_size(LabelSize::Small),
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
