use gpui::{AnyElement, ClickEvent, prelude::*};

use crate::{ButtonLike, CommonAnimationExt, Tooltip, prelude::*};

/// A button component displayed in the title bar to show auto-update status.
#[derive(IntoElement, RegisterComponent)]
pub struct UpdateButton {
    icon: IconName,
    icon_animate: bool,
    icon_color: Option<Color>,
    message: SharedString,
    tooltip: Option<SharedString>,
    show_dismiss: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_dismiss: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl UpdateButton {
    pub fn new(icon: IconName, message: impl Into<SharedString>) -> Self {
        Self {
            icon,
            icon_animate: false,
            icon_color: None,
            message: message.into(),
            tooltip: None,
            show_dismiss: false,
            on_click: None,
            on_dismiss: None,
        }
    }

    /// Sets whether the icon should have a rotation animation (for progress states).
    pub fn icon_animate(mut self, animate: bool) -> Self {
        self.icon_animate = animate;
        self
    }

    /// Sets the icon color (e.g., for warning/error states).
    pub fn icon_color(mut self, color: impl Into<Option<Color>>) -> Self {
        self.icon_color = color.into();
        self
    }

    /// Sets the tooltip text shown on hover.
    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Shows a dismiss button on the right side.
    pub fn with_dismiss(mut self) -> Self {
        self.show_dismiss = true;
        self
    }

    /// Sets the click handler for the main button area.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Sets the click handler for the dismiss button.
    pub fn on_dismiss(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_dismiss = Some(Box::new(handler));
        self
    }

    pub fn checking() -> Self {
        Self::new(IconName::ArrowCircle, "Checking for Zed updates…").icon_animate(true)
    }

    pub fn downloading(version: impl Into<SharedString>) -> Self {
        Self::new(IconName::Download, "Downloading Zed update…").tooltip(version)
    }

    pub fn installing(version: impl Into<SharedString>) -> Self {
        Self::new(IconName::ArrowCircle, "Installing Zed update…")
            .icon_animate(true)
            .tooltip(version)
    }

    pub fn updated(version: impl Into<SharedString>) -> Self {
        Self::new(IconName::Download, "Click to restart and update Zed")
            .tooltip(version)
            .with_dismiss()
    }

    pub fn errored(error: impl Into<SharedString>) -> Self {
        Self::new(IconName::Warning, "Failed to update Zed")
            .icon_color(Color::Warning)
            .tooltip(error)
            .with_dismiss()
    }
}

impl RenderOnce for UpdateButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let border_color = cx.theme().colors().border;

        let icon = Icon::new(self.icon)
            .size(IconSize::XSmall)
            .when_some(self.icon_color, |this, color| this.color(color));
        let icon_element = if self.icon_animate {
            icon.with_rotate_animation(3).into_any_element()
        } else {
            icon.into_any_element()
        };

        let tooltip = self.tooltip.clone();

        h_flex()
            .mr_2()
            .rounded_sm()
            .border_1()
            .border_color(border_color)
            .child(
                ButtonLike::new("update-button")
                    .child(
                        h_flex()
                            .h_full()
                            .gap_1()
                            .child(icon_element)
                            .child(Label::new(self.message).size(LabelSize::Small)),
                    )
                    .when_some(tooltip, |this, tooltip| {
                        this.tooltip(Tooltip::text(tooltip))
                    })
                    .when_some(self.on_click, |this, handler| this.on_click(handler)),
            )
            .when(self.show_dismiss, |this| {
                this.child(
                    div().border_l_1().border_color(border_color).child(
                        IconButton::new("dismiss-update-button", IconName::Close)
                            .icon_size(IconSize::Indicator)
                            .when_some(self.on_dismiss, |this, handler| this.on_click(handler))
                            .tooltip(Tooltip::text("Dismiss")),
                    ),
                )
            })
    }
}

impl Component for UpdateButton {
    fn scope() -> ComponentScope {
        ComponentScope::Collaboration
    }

    fn name() -> &'static str {
        "UpdateButton"
    }

    fn description() -> Option<&'static str> {
        Some(
            "A button component displayed in the title bar to show auto-update status and allow users to restart Zed.",
        )
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        let version = "1.99.0";

        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group_with_title(
                        "Progress States",
                        vec![
                            single_example("Checking", UpdateButton::checking().into_any_element()),
                            single_example(
                                "Downloading",
                                UpdateButton::downloading(version).into_any_element(),
                            ),
                            single_example(
                                "Installing",
                                UpdateButton::installing(version).into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Actionable States",
                        vec![
                            single_example(
                                "Ready to Update",
                                UpdateButton::updated(version).into_any_element(),
                            ),
                            single_example(
                                "Error",
                                UpdateButton::errored("Network timeout").into_any_element(),
                            ),
                        ],
                    ),
                ])
                .into_any_element(),
        )
    }
}
