// This example demonstrates how to create a macOS agent-style application using GPUI.
// An agent app runs in the background without a Dock icon (LSUIElement behavior).
//
// ## What is LSUIElement?
// LSUIElement (also known as ActivationPolicy::Accessory) makes your app:
// - Run without appearing in the Dock
// - Not show in Cmd+Tab app switcher
// - Perfect for menu bar apps, background utilities, or system agents
//
// ## How to run this example:
// ```
// cargo run --example macos_agent
// ```
//
// Note: This example only works on macOS. On other platforms, it will behave like a normal window.

use gpui::{
    App, Application, Bounds, Context, KeyBinding, SharedString, Window, WindowBounds,
    WindowOptions, actions, div, prelude::*, px, rgb, size,
};

// Define a Quit action for the quit button
actions!(macos_agent, [Quit]);

struct AgentWindow {
    title: SharedString,
}

impl Render for AgentWindow {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(0xf5f5f5))
            .size_full()
            .p_4()
            .gap_3()
            .child(
                // Header
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(rgb(0x333333))
                            .child(self.title.clone()),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x666666))
                            .child("This window is running as an LSUIElement (Accessory) app"),
                    ),
            )
            .child(
                // Info box
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .p_3()
                    .bg(rgb(0xe3f2fd))
                    .border_1()
                    .border_color(rgb(0x90caf9))
                    .rounded_md()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(rgb(0x1565c0))
                            .child("ℹ️ LSUIElement Characteristics:"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x1976d2))
                            .child("• No Dock icon visible"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x1976d2))
                            .child("• Not shown in Cmd+Tab switcher"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x1976d2))
                            .child("• Perfect for menu bar apps and background utilities"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x1976d2))
                            .child("• Window can still be shown and interacted with"),
                    ),
            )
            .child(
                // Instructions
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .p_3()
                    .bg(rgb(0xfff3e0))
                    .border_1()
                    .border_color(rgb(0xffb74d))
                    .rounded_md()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(rgb(0xe65100))
                            .child("💡 Try this:"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0xef6c00))
                            .child("1. Look at your Dock - this app won't appear there"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0xef6c00))
                            .child("2. Press Cmd+Tab - this app won't be in the switcher"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0xef6c00))
                            .child("3. Use the Quit button below to close the app"),
                    ),
            )
            .child(
                // Quit button
                div().flex().justify_center().mt_2().child(
                    div()
                        .id("quit-button")
                        .flex()
                        .px_4()
                        .py_2()
                        .bg(rgb(0xd32f2f))
                        .hover(|style| style.bg(rgb(0xc62828)))
                        .active(|style| style.bg(rgb(0xb71c1c)))
                        .text_color(rgb(0xffffff))
                        .rounded_md()
                        .cursor_pointer()
                        .child("Quit Application (Cmd+Q)")
                        .on_click(|_, _, cx| {
                            cx.quit();
                        }),
                ),
            )
    }
}

fn main() {
    // Create the application
    let app = Application::new();

    // On macOS, set the activation policy to Accessory (LSUIElement)
    // This makes the app run without a Dock icon
    #[cfg(target_os = "macos")]
    let app = app.with_activation_policy(gpui::ActivationPolicy::Accessory);

    app.run(|cx: &mut App| {
        // Set up the quit action handler
        cx.on_action(|_: &Quit, cx| {
            cx.quit();
        });

        // Bind Cmd+Q to quit
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

        // Create a centered window
        let bounds = Bounds::centered(None, size(px(500.0), px(450.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("macOS Agent Demo".into()),
                    ..Default::default()
                }),
                focus: true,
                show: true,
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| AgentWindow {
                    title: "macOS Agent Application".into(),
                })
            },
        )
        .unwrap();

        // Activate the application
        // Even though we're an accessory app, we can still show windows
        cx.activate(true);
    });
}
