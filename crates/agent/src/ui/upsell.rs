use gpui::{App, ClickEvent, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window};
use theme::ActiveTheme;
use ui::{Button, ButtonCommon, ButtonStyle, Checkbox, Clickable, Color, LabelCommon, Label, h_flex, v_flex, ToggleState};

/// A component that displays an upsell message with a call-to-action button
/// 
/// # Example
/// ```
/// let upsell = Upsell::new(
///     "Upgrade to Zed Pro",
///     "Get unlimited access to AI features and more",
///     "Upgrade Now",
///     Box::new(|_, _window, cx| {
///         cx.open_url("https://zed.dev/pricing");
///     }),
///     Box::new(|_, _window, cx| {
///         // Handle dismiss
///     }),
///     Box::new(|checked, window, cx| {
///         // Handle don't show again
///     }),
/// );
/// ```
pub struct Upsell {
    title: SharedString,
    message: SharedString,
    cta_text: SharedString,
    on_click: Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>,
    on_dismiss: Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>,
    on_dont_show_again: Box<dyn Fn(bool, &mut Window, &mut App)>,
}

impl Upsell {
    /// Create a new upsell component
    pub fn new(
        title: impl Into<SharedString>,
        message: impl Into<SharedString>,
        cta_text: impl Into<SharedString>,
        on_click: Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>,
        on_dismiss: Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>,
        on_dont_show_again: Box<dyn Fn(bool, &mut Window, &mut App)>,
    ) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            cta_text: cta_text.into(),
            on_click,
            on_dismiss,
            on_dont_show_again,
        }
    }
}

impl RenderOnce for Upsell {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .w_full()
            .p_4()
            .gap_3()
            .bg(cx.theme().colors().surface_background)
            .rounded_md()
            .border_1()
            .border_color(cx.theme().colors().border)
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        Label::new(self.title)
                            .size(ui::LabelSize::Large)
                            .weight(gpui::FontWeight::BOLD)
                    )
                    .child(
                        Label::new(self.message)
                            .color(Color::Muted)
                    )
            )
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .items_center()
                    .child(
                        h_flex()
                            .items_center()
                            .gap_1()
                            .child(
                                Checkbox::new("dont-show-again", ToggleState::Unselected)
                                    .on_click(move |_, window, cx| {
                                        (self.on_dont_show_again)(true, window, cx);
                                    })
                            )
                            .child(
                                Label::new("Don't show again")
                                    .color(Color::Muted)
                                    .size(ui::LabelSize::Small)
                            )
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Button::new("dismiss-button", "Dismiss")
                                    .style(ButtonStyle::Subtle)
                                    .on_click(self.on_dismiss)
                            )
                            .child(
                                Button::new("cta-button", self.cta_text)
                                    .style(ButtonStyle::Filled)
                                    .on_click(self.on_click)
                            )
                    )
            )
    }
}