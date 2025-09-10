use gpui::AnyElement;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderPosition {
    Top,
    Bottom,
}

/// A callout component for displaying important information that requires user attention.
///
/// # Usage Example
///
/// ```
/// use ui::{Callout};
///
/// Callout::new()
///     .severity(Severity::Warning)
///     .icon(IconName::Warning)
///     .title(Label::new("Be aware of your subscription!"))
///     .description(Label::new("Your subscription is about to expire. Renew now!"))
///     .actions_slot(Button::new("renew", "Renew Now"))
/// ```
///
#[derive(IntoElement, RegisterComponent)]
pub struct Callout {
    severity: Severity,
    icon: Option<IconName>,
    title: Option<SharedString>,
    description: Option<SharedString>,
    actions_slot: Option<AnyElement>,
    dismiss_action: Option<AnyElement>,
    line_height: Option<Pixels>,
    border_position: BorderPosition,
}

impl Callout {
    /// Creates a new `Callout` component with default styling.
    pub fn new() -> Self {
        Self {
            severity: Severity::Info,
            icon: None,
            title: None,
            description: None,
            actions_slot: None,
            dismiss_action: None,
            line_height: None,
            border_position: BorderPosition::Top,
        }
    }

    /// Sets the severity of the callout.
    pub fn severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    /// Sets the icon to display in the callout.
    pub fn icon(mut self, icon: IconName) -> Self {
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
    pub fn actions_slot(mut self, action: impl IntoElement) -> Self {
        self.actions_slot = Some(action.into_any_element());
        self
    }

    /// Sets an optional dismiss button, which is usually an icon button with a close icon.
    /// This button is always rendered as the last one to the far right.
    pub fn dismiss_action(mut self, action: impl IntoElement) -> Self {
        self.dismiss_action = Some(action.into_any_element());
        self
    }

    /// Sets a custom line height for the callout content.
    pub fn line_height(mut self, line_height: Pixels) -> Self {
        self.line_height = Some(line_height);
        self
    }

    /// Sets the border position in the callout.
    pub fn border_position(mut self, border_position: BorderPosition) -> Self {
        self.border_position = border_position;
        self
    }
}

impl RenderOnce for Callout {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let line_height = self.line_height.unwrap_or(window.line_height());

        let has_actions = self.actions_slot.is_some() || self.dismiss_action.is_some();

        let (icon, icon_color, bg_color) = match self.severity {
            Severity::Info => (
                IconName::Info,
                Color::Muted,
                cx.theme().colors().panel_background.opacity(0.),
            ),
            Severity::Success => (
                IconName::Check,
                Color::Success,
                cx.theme().status().success.opacity(0.1),
            ),
            Severity::Warning => (
                IconName::Warning,
                Color::Warning,
                cx.theme().status().warning_background.opacity(0.2),
            ),
            Severity::Error => (
                IconName::XCircle,
                Color::Error,
                cx.theme().status().error.opacity(0.08),
            ),
        };

        h_flex()
            .min_w_0()
            .w_full()
            .p_2()
            .gap_2()
            .items_start()
            .map(|this| match self.border_position {
                BorderPosition::Top => this.border_t_1(),
                BorderPosition::Bottom => this.border_b_1(),
            })
            .border_color(cx.theme().colors().border)
            .bg(bg_color)
            .overflow_x_hidden()
            .when(self.icon.is_some(), |this| {
                this.child(
                    h_flex()
                        .h(line_height)
                        .justify_center()
                        .child(Icon::new(icon).size(IconSize::Small).color(icon_color)),
                )
            })
            .child(
                v_flex()
                    .min_w_0()
                    .w_full()
                    .child(
                        h_flex()
                            .min_h(line_height)
                            .w_full()
                            .gap_1()
                            .justify_between()
                            .flex_wrap()
                            .when_some(self.title, |this, title| {
                                this.child(h_flex().child(Label::new(title).size(LabelSize::Small)))
                            })
                            .when(has_actions, |this| {
                                this.child(
                                    h_flex()
                                        .gap_0p5()
                                        .when_some(self.actions_slot, |this, action| {
                                            this.child(action)
                                        })
                                        .when_some(self.dismiss_action, |this, action| {
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
        let single_action = || Button::new("got-it", "Got it").label_size(LabelSize::Small);
        let multiple_actions = || {
            h_flex()
                .gap_0p5()
                .child(Button::new("update", "Backup & Update").label_size(LabelSize::Small))
                .child(Button::new("dismiss", "Dismiss").label_size(LabelSize::Small))
        };

        let basic_examples = vec![
            single_example(
                "Simple with Title Only",
                Callout::new()
                    .icon(IconName::Info)
                    .title("System maintenance scheduled for tonight")
                    .actions_slot(single_action())
                    .into_any_element(),
            )
            .width(px(580.)),
            single_example(
                "With Title and Description",
                Callout::new()
                    .icon(IconName::Warning)
                    .title("Your settings contain deprecated values")
                    .description(
                        "We'll backup your current settings and update them to the new format.",
                    )
                    .actions_slot(single_action())
                    .into_any_element(),
            )
            .width(px(580.)),
            single_example(
                "Error with Multiple Actions",
                Callout::new()
                    .icon(IconName::Close)
                    .title("Thread reached the token limit")
                    .description("Start a new thread from a summary to continue the conversation.")
                    .actions_slot(multiple_actions())
                    .into_any_element(),
            )
            .width(px(580.)),
            single_example(
                "Multi-line Description",
                Callout::new()
                    .icon(IconName::Sparkle)
                    .title("Upgrade to Pro")
                    .description("• Unlimited threads\n• Priority support\n• Advanced analytics")
                    .actions_slot(multiple_actions())
                    .into_any_element(),
            )
            .width(px(580.)),
        ];

        let severity_examples = vec![
            single_example(
                "Info",
                Callout::new()
                    .icon(IconName::Info)
                    .title("System maintenance scheduled for tonight")
                    .actions_slot(single_action())
                    .into_any_element(),
            ),
            single_example(
                "Warning",
                Callout::new()
                    .severity(Severity::Warning)
                    .icon(IconName::Triangle)
                    .title("System maintenance scheduled for tonight")
                    .actions_slot(single_action())
                    .into_any_element(),
            ),
            single_example(
                "Error",
                Callout::new()
                    .severity(Severity::Error)
                    .icon(IconName::XCircle)
                    .title("System maintenance scheduled for tonight")
                    .actions_slot(single_action())
                    .into_any_element(),
            ),
            single_example(
                "Success",
                Callout::new()
                    .severity(Severity::Success)
                    .icon(IconName::Check)
                    .title("System maintenance scheduled for tonight")
                    .actions_slot(single_action())
                    .into_any_element(),
            ),
        ];

        Some(
            v_flex()
                .gap_4()
                .child(example_group(basic_examples).vertical())
                .child(example_group_with_title("Severity", severity_examples).vertical())
                .into_any_element(),
        )
    }
}
