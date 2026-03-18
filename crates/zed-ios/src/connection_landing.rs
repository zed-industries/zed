use gpui::{
    App, Context, IntoElement, Render, SharedString, Window, WindowOptions,
    div, prelude::*,
};
use theme::ActiveTheme;
use ui::{Vector, VectorName, h_flex, v_flex, rems_from_px, Headline, Label, LabelCommon, LabelSize, Color};

/// Landing screen shown on iPad launch. Lists saved SSH hosts and provides
/// an "Add Host" entry point. This replaces the desktop welcome page — the
/// thin client has no local filesystem, so the first thing a user does is
/// pick a remote host.
pub struct ConnectionLanding {
    focus_handle: gpui::FocusHandle,
}

impl ConnectionLanding {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }

    /// Open the connection landing screen in a new window.
    pub fn open(cx: &mut App) -> anyhow::Result<()> {
        cx.open_window(WindowOptions::default(), |_window, cx| {
            cx.new(Self::new)
        })?;
        Ok(())
    }
}

impl Render for ConnectionLanding {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors();

        div()
            .id("connection-landing")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(colors.background)
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_6()
            // Header — logo + welcome
            .child(
                v_flex()
                    .items_center()
                    .gap_4()
                    .child(
                        h_flex()
                            .justify_center()
                            .gap_4()
                            .child(Vector::square(VectorName::ZedLogo, rems_from_px(45.)))
                            .child(
                                v_flex()
                                    .child(Headline::new("Welcome to Zed"))
                                    .child(
                                        Label::new("The editor for what's next")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted)
                                            .italic(),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .text_color(colors.text_muted)
                            .child(SharedString::from(
                                "Connect to a remote host to start editing",
                            )),
                    ),
            )
            // Hosts section
            .child(
                div()
                    .w_96()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .text_color(colors.text_muted)
                            .child(SharedString::from("SAVED HOSTS")),
                    )
                    .child(
                        div()
                            .rounded_lg()
                            .border_1()
                            .border_color(colors.border)
                            .bg(colors.surface_background)
                            .p_4()
                            .child(
                                div()
                                    .text_color(colors.text_muted)
                                    .text_center()
                                    .child(SharedString::from("No saved hosts yet")),
                            ),
                    ),
            )
            // Add host button
            .child(
                div()
                    .id("add-host")
                    .w_96()
                    .rounded_lg()
                    .border_1()
                    .border_color(colors.border)
                    .bg(colors.surface_background)
                    .px_4()
                    .py_3()
                    .cursor_pointer()
                    .hover(|style| style.bg(colors.ghost_element_hover))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_color(colors.text)
                                    .child(SharedString::from("+ Connect SSH Server")),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(colors.text_muted)
                                    .child(SharedString::from("ssh user@hostname -p port")),
                            ),
                    ),
            )
    }
}

impl gpui::Focusable for ConnectionLanding {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}
