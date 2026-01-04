use gpui::{
    App, Application, Bounds, Context, Div, ElementId, FocusHandle, KeyBinding, SharedString,
    Stateful, Window, WindowBounds, WindowOptions, actions, div, prelude::*, px, size,
};

actions!(example, [Tab, TabPrev, Quit]);

struct Example {
    focus_handle: FocusHandle,
    items: Vec<(FocusHandle, &'static str)>,
    message: SharedString,
}

impl Example {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let items = vec![
            (
                cx.focus_handle().tab_index(1).tab_stop(true),
                "Button with .focus() - always shows border when focused",
            ),
            (
                cx.focus_handle().tab_index(2).tab_stop(true),
                "Button with .focus_visible() - only shows border with keyboard",
            ),
            (
                cx.focus_handle().tab_index(3).tab_stop(true),
                "Button with both .focus() and .focus_visible()",
            ),
        ];

        let focus_handle = cx.focus_handle();
        window.focus(&focus_handle, cx);

        Self {
            focus_handle,
            items,
            message: SharedString::from(
                "Try clicking vs tabbing! Click shows no border, Tab shows border.",
            ),
        }
    }

    fn on_tab(&mut self, _: &Tab, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_next(cx);
        self.message = SharedString::from("Pressed Tab - focus-visible border should appear!");
    }

    fn on_tab_prev(&mut self, _: &TabPrev, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_prev(cx);
        self.message =
            SharedString::from("Pressed Shift-Tab - focus-visible border should appear!");
    }

    fn on_quit(&mut self, _: &Quit, _window: &mut Window, cx: &mut Context<Self>) {
        cx.quit();
    }
}

impl Render for Example {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        fn button_base(id: impl Into<ElementId>, label: &'static str) -> Stateful<Div> {
            div()
                .id(id)
                .h_16()
                .w_full()
                .flex()
                .justify_center()
                .items_center()
                .bg(gpui::rgb(0x2563eb))
                .text_color(gpui::white())
                .rounded_md()
                .cursor_pointer()
                .hover(|style| style.bg(gpui::rgb(0x1d4ed8)))
                .child(label)
        }

        div()
            .id("app")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::on_tab))
            .on_action(cx.listener(Self::on_tab_prev))
            .on_action(cx.listener(Self::on_quit))
            .size_full()
            .flex()
            .flex_col()
            .p_8()
            .gap_6()
            .bg(gpui::rgb(0xf3f4f6))
            .child(
                div()
                    .text_2xl()
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(gpui::rgb(0x111827))
                    .child("CSS focus-visible Demo"),
            )
            .child(
                div()
                    .p_4()
                    .rounded_md()
                    .bg(gpui::rgb(0xdbeafe))
                    .text_color(gpui::rgb(0x1e3a8a))
                    .child(self.message.clone()),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .text_color(gpui::rgb(0x374151))
                                    .child("1. Regular .focus() - always visible:"),
                            )
                            .child(
                                button_base("button1", self.items[0].1)
                                    .track_focus(&self.items[0].0)
                                    .focus(|style| {
                                        style.border_4().border_color(gpui::rgb(0xfbbf24))
                                    })
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.message =
                                            "Clicked button 1 - focus border is visible!".into();
                                        cx.notify();
                                    })),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .text_color(gpui::rgb(0x374151))
                                    .child("2. New .focus_visible() - only keyboard:"),
                            )
                            .child(
                                button_base("button2", self.items[1].1)
                                    .track_focus(&self.items[1].0)
                                    .focus_visible(|style| {
                                        style.border_4().border_color(gpui::rgb(0x10b981))
                                    })
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.message =
                                            "Clicked button 2 - no border! Try Tab instead.".into();
                                        cx.notify();
                                    })),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .text_color(gpui::rgb(0x374151))
                                    .child(
                                        "3. Both .focus() (yellow) and .focus_visible() (green):",
                                    ),
                            )
                            .child(
                                button_base("button3", self.items[2].1)
                                    .track_focus(&self.items[2].0)
                                    .focus(|style| {
                                        style.border_4().border_color(gpui::rgb(0xfbbf24))
                                    })
                                    .focus_visible(|style| {
                                        style.border_4().border_color(gpui::rgb(0x10b981))
                                    })
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.message =
                                            "Clicked button 3 - yellow border. Tab shows green!"
                                                .into();
                                        cx.notify();
                                    })),
                            ),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.bind_keys([
            KeyBinding::new("tab", Tab, None),
            KeyBinding::new("shift-tab", TabPrev, None),
            KeyBinding::new("cmd-q", Quit, None),
        ]);

        let bounds = Bounds::centered(None, size(px(800.), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| Example::new(window, cx)),
        )
        .unwrap();

        cx.activate(true);
    });
}
