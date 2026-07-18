use gpui::{AnyElement, AnyView, ClickEvent, prelude::*};

use crate::{ButtonLike, CircularProgress, CommonAnimationExt, Tooltip, prelude::*};

const LOAD_CIRCLE_GLYPH_VIEWBOX: f32 = 16.0;
const LOAD_CIRCLE_GLYPH_STROKE_WIDTH: f32 = 1.2;
const LOAD_CIRCLE_GLYPH_RADIUS: f32 = 5.0;

/// A button component displayed in the title bar to show auto-update status.
#[derive(IntoElement, RegisterComponent)]
pub struct UpdateButton {
    icon: IconName,
    icon_animate: bool,
    icon_color: Option<Color>,
    message: SharedString,
    tooltip: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView + 'static>>,
    disabled: bool,
    show_dismiss: bool,
    progress: Option<f32>,
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
            disabled: false,
            show_dismiss: false,
            progress: None,
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
        self.tooltip = Some(Box::new(Tooltip::text(tooltip.into())));
        self
    }

    /// Sets a tooltip builder invoked on every render, so the tooltip can
    /// display content that changes while it stays visible.
    pub fn tooltip_fn(
        mut self,
        tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static,
    ) -> Self {
        self.tooltip = Some(Box::new(tooltip));
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn progress(mut self, progress: impl Into<Option<f32>>) -> Self {
        self.progress = progress.into();
        self
    }

    pub fn checking() -> Self {
        Self::new(IconName::LoadCircle, "Checking for Zed Updates…")
            .icon_animate(true)
            .disabled(true)
    }

    pub fn downloading(progress: Option<f32>) -> Self {
        Self::new(IconName::Download, "Downloading Zed Update…")
            .progress(progress)
            .disabled(true)
    }

    pub fn installing(version: impl Into<SharedString>) -> Self {
        Self::new(IconName::LoadCircle, "Installing Zed Update…")
            .icon_animate(true)
            .tooltip(version)
            .disabled(true)
    }

    pub fn updated(version: impl Into<SharedString>) -> Self {
        Self::new(IconName::Download, "Restart to Update")
            .tooltip(version)
            .with_dismiss()
    }

    pub fn errored(error: impl Into<SharedString>) -> Self {
        Self::new(IconName::Warning, "Failed to Update")
            .icon_color(Color::Warning)
            .tooltip(error)
            .with_dismiss()
    }

    pub fn version_tooltip_message(version: impl std::fmt::Display) -> String {
        format!("Update to Version: {version}")
    }

    pub fn downloading_tooltip_message(
        version: impl std::fmt::Display,
        progress: Option<f32>,
    ) -> String {
        let message = Self::version_tooltip_message(version);
        match progress {
            Some(progress) => format!(
                "{message} ({:.0}% downloaded)",
                progress.clamp(0.0, 1.0) * 100.0
            ),
            None => message,
        }
    }
}

impl RenderOnce for UpdateButton {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let border_color = if self.disabled {
            cx.theme().colors().border
        } else {
            cx.theme().colors().text.opacity(0.15)
        };

        let icon_element = if let Some(progress) = self.progress {
            let progress = progress.clamp(0.0, 1.0);
            let icon_box = IconSize::XSmall.rems().to_pixels(window.rem_size());
            let progress_color = Color::Default.color(cx);
            CircularProgress::new(progress, 1.0, icon_box, cx)
                .stroke_width(
                    icon_box * (LOAD_CIRCLE_GLYPH_STROKE_WIDTH / LOAD_CIRCLE_GLYPH_VIEWBOX),
                )
                .radius(icon_box * (LOAD_CIRCLE_GLYPH_RADIUS / LOAD_CIRCLE_GLYPH_VIEWBOX))
                .bg_color(progress_color.opacity(0.2))
                .progress_color(progress_color)
                .into_any_element()
        } else {
            let icon = Icon::new(self.icon)
                .size(IconSize::XSmall)
                .when_some(self.icon_color, |this, color| this.color(color));
            if self.icon_animate {
                icon.with_rotate_animation(2).into_any_element()
            } else {
                icon.into_any_element()
            }
        };

        let tooltip = self.tooltip;

        let button_id = ElementId::Name(self.message.clone());
        let dismiss_button_id = ElementId::Name(format!("dismiss-{}", self.message).into());

        let label_row = h_flex()
            .h_full()
            .gap_1()
            .child(icon_element)
            .child(Label::new(self.message).size(LabelSize::Small));

        h_flex()
            .mr_2()
            .rounded_sm()
            .border_1()
            .border_color(border_color)
            .child(
                ButtonLike::new(button_id)
                    .child(label_row)
                    .when_some(tooltip, |this, tooltip| this.tooltip(tooltip))
                    .disabled(self.disabled)
                    .when_some(self.on_click, |this, handler| this.on_click(handler)),
            )
            .when(self.show_dismiss, |this| {
                this.child(
                    div().border_l_1().border_color(border_color).child(
                        IconButton::new(dismiss_button_id, IconName::Close)
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

    fn description() -> &'static str {
        "A button component displayed in the title bar to \
        show auto-update status and allow users to restart Zed."
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> AnyElement {
        let version = "1.3.0+stable.2025051";

        v_flex()
            .gap_6()
            .children(vec![
                example_group_with_title(
                    "Progress States",
                    vec![
                        single_example("Checking", UpdateButton::checking().into_any_element()),
                        single_example(
                            "Downloading",
                            UpdateButton::downloading(Some(0.45))
                                .tooltip(UpdateButton::downloading_tooltip_message(
                                    version,
                                    Some(0.45),
                                ))
                                .into_any_element(),
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
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Render, TestAppContext, point, px};
    use std::cell::Cell;
    use std::rc::Rc;
    use std::time::Duration;

    struct TestTooltip {
        rendered: Rc<Cell<u32>>,
    }

    impl Render for TestTooltip {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            self.rendered.set(self.rendered.get() + 1);
            div().child("tooltip")
        }
    }

    struct PreviewLikeButtons {
        tooltip_built: Rc<Cell<bool>>,
        tooltip_rendered: Rc<Cell<u32>>,
    }

    impl Render for PreviewLikeButtons {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let tooltip_built = self.tooltip_built.clone();
            let tooltip_rendered = self.tooltip_rendered.clone();
            crate::v_flex()
                .size_full()
                .child(UpdateButton::checking())
                .child(
                    UpdateButton::downloading(Some(0.5)).tooltip_fn(move |_, cx| {
                        tooltip_built.set(true);
                        let rendered = tooltip_rendered.clone();
                        cx.new(|_| TestTooltip { rendered }).into()
                    }),
                )
                .child(UpdateButton::updated("Update to Version: 1.0.0"))
        }
    }

    #[gpui::test]
    async fn test_downloading_tooltip_shows_in_preview_like_layout(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
        });
        let tooltip_built = Rc::new(Cell::new(false));
        let tooltip_rendered = Rc::new(Cell::new(0));
        let (_view, cx) = cx.add_window_view({
            let tooltip_built = tooltip_built.clone();
            let tooltip_rendered = tooltip_rendered.clone();
            |_, _| PreviewLikeButtons {
                tooltip_built,
                tooltip_rendered,
            }
        });

        cx.simulate_mouse_move(point(px(30.), px(30.)), None, gpui::Modifiers::default());
        cx.run_until_parked();
        cx.simulate_mouse_move(point(px(31.), px(30.)), None, gpui::Modifiers::default());
        cx.run_until_parked();

        cx.executor().advance_clock(Duration::from_millis(600));
        cx.run_until_parked();

        assert!(tooltip_built.get(), "tooltip should have been built");

        tooltip_rendered.set(0);
        cx.update(|window, _| window.refresh());
        cx.run_until_parked();
        assert!(
            tooltip_rendered.get() > 0,
            "tooltip should still be rendered after another frame"
        );
    }
}
