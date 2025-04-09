use crate::prelude::*;
use gpui::{AnyElement, IntoElement, ParentElement, Styled};

/// Severity levels that determine the style of the banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Info,
    Success,
    Warning,
    Error,
}

/// Banners provide informative and brief messages without interrupting the user.
/// This component offers four severity levels that can be used depending on the message.
///
/// # Usage Example
///
/// ```
/// use ui::{Banner};
///
///    Banner::new()
///     .severity(Severity::Info)
///     .children(Label::new("This is an informational message"))
///     .action_slot(
///         Button::new("learn-more", "Learn More")
///             .icon(IconName::ArrowUpRight)
///             .icon_size(IconSize::XSmall)
///             .icon_position(IconPosition::End),
///     )
/// ```
#[derive(IntoElement, RegisterComponent)]
pub struct Banner {
    severity: Severity,
    children: Option<AnyElement>,
    icon: Option<(IconName, Option<Color>)>,
    action_slot: Option<AnyElement>,
}

impl Banner {
    /// Creates a new `Banner` component with default styling.
    pub fn new() -> Self {
        Self {
            severity: Severity::Info,
            children: None,
            icon: None,
            action_slot: None,
        }
    }

    /// Sets the severity of the banner.
    pub fn severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    /// Sets an icon to display in the banner with an optional color.
    pub fn icon(mut self, icon: IconName, color: Option<impl Into<Color>>) -> Self {
        self.icon = Some((icon, color.map(|c| c.into())));
        self
    }

    /// A slot for actions, such as CTA or dismissal buttons.
    pub fn action_slot(mut self, element: impl IntoElement) -> Self {
        self.action_slot = Some(element.into_any_element());
        self
    }

    /// A general container for the banner's main content.
    pub fn children(mut self, element: impl IntoElement) -> Self {
        self.children = Some(element.into_any_element());
        self
    }
}

impl RenderOnce for Banner {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let base = h_flex()
            .py_0p5()
            .rounded_sm()
            .flex_wrap()
            .justify_between()
            .border_1();

        let (icon, icon_color, bg_color, border_color) = match self.severity {
            Severity::Info => (
                IconName::Info,
                Color::Muted,
                cx.theme().status().info_background.opacity(0.5),
                cx.theme().colors().border_variant,
            ),
            Severity::Success => (
                IconName::Check,
                Color::Success,
                cx.theme().status().success.opacity(0.1),
                cx.theme().status().success.opacity(0.2),
            ),
            Severity::Warning => (
                IconName::Warning,
                Color::Warning,
                cx.theme().status().warning_background.opacity(0.5),
                cx.theme().status().warning_border.opacity(0.4),
            ),
            Severity::Error => (
                IconName::XCircle,
                Color::Error,
                cx.theme().status().error.opacity(0.1),
                cx.theme().status().error.opacity(0.2),
            ),
        };

        let mut container = base.bg(bg_color).border_color(border_color);

        let mut content_area = h_flex().id("content_area").gap_1p5().overflow_x_scroll();

        if self.icon.is_none() {
            content_area =
                content_area.child(Icon::new(icon).size(IconSize::XSmall).color(icon_color));
        }

        if let Some(children) = self.children {
            content_area = content_area.child(children);
        }

        if let Some(action_slot) = self.action_slot {
            container = container
                .pl_2()
                .pr_0p5()
                .gap_2()
                .child(content_area)
                .child(action_slot);
        } else {
            container = container.px_2().child(div().w_full().child(content_area));
        }

        container
    }
}

impl Component for Banner {
    fn scope() -> ComponentScope {
        ComponentScope::Notification
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        let severity_examples = vec![
            single_example(
                "Default",
                Banner::new()
                    .children(Label::new("This is a default banner with no customization"))
                    .into_any_element(),
            ),
            single_example(
                "Info",
                Banner::new()
                    .severity(Severity::Info)
                    .children(Label::new("This is an informational message"))
                    .action_slot(
                        Button::new("learn-more", "Learn More")
                            .icon(IconName::ArrowUpRight)
                            .icon_size(IconSize::XSmall)
                            .icon_position(IconPosition::End),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Success",
                Banner::new()
                    .severity(Severity::Success)
                    .children(Label::new("Operation completed successfully"))
                    .action_slot(Button::new("dismiss", "Dismiss"))
                    .into_any_element(),
            ),
            single_example(
                "Warning",
                Banner::new()
                    .severity(Severity::Warning)
                    .children(Label::new("Your settings file uses deprecated settings"))
                    .action_slot(Button::new("update", "Update Settings"))
                    .into_any_element(),
            ),
            single_example(
                "Error",
                Banner::new()
                    .severity(Severity::Error)
                    .children(Label::new("Connection error: unable to connect to server"))
                    .action_slot(Button::new("reconnect", "Retry"))
                    .into_any_element(),
            ),
        ];

        Some(
            example_group(severity_examples)
                .vertical()
                .into_any_element(),
        )
    }
}
