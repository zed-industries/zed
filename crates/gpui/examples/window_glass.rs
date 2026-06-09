#![cfg_attr(target_family = "wasm", no_main)]

use gpui::{
    App, Bounds, Context, FontWeight, SharedString, Window, WindowBackgroundAppearance,
    WindowBounds, WindowOptions, div, hsla, prelude::*, px, size,
};
use gpui_platform::application;

/// A small mail client demonstrating [`Styled::glass`].
///
/// The window uses a system glass background, so the wallpaper shows through
/// the translucent sidebar. Following Apple's Liquid Glass guidance, only the
/// navigation layer (title bar + sidebar) is translucent; the message list is
/// opaque so the content stays crisp.
///
/// The sidebar is marked `.glass()`: its rounded selected/hover rows only blend
/// RGB and preserve the sidebar's alpha, so their anti-aliased corners stay
/// clean instead of punching through the glass. Remove `.glass()` to see those
/// rounded edges turn jagged against the wallpaper behind the window.
struct MailApp {
    selected_mailbox: usize,
    selected_message: usize,
}

struct Mailbox {
    name: &'static str,
    unread: u32,
}

struct Message {
    sender: &'static str,
    subject: &'static str,
    preview: &'static str,
    time: &'static str,
    unread: bool,
}

const MAILBOXES: [Mailbox; 5] = [
    Mailbox {
        name: "Inbox",
        unread: 4,
    },
    Mailbox {
        name: "Flagged",
        unread: 1,
    },
    Mailbox {
        name: "Sent",
        unread: 0,
    },
    Mailbox {
        name: "Drafts",
        unread: 2,
    },
    Mailbox {
        name: "Trash",
        unread: 0,
    },
];

const MESSAGES: [Message; 5] = [
    Message {
        sender: "Lina Park",
        subject: "Design review notes",
        preview: "Thanks for the mockups — a couple of small tweaks on the sidebar spacing and we're good to ship.",
        time: "9:41 AM",
        unread: true,
    },
    Message {
        sender: "GitHub",
        subject: "[zed] PR #12345 approved",
        preview: "build-and-test passed · 2 approvals · ready to merge into main.",
        time: "8:02 AM",
        unread: true,
    },
    Message {
        sender: "Marcus Lee",
        subject: "Lunch on Friday?",
        preview: "There's a new ramen place near the office — want to try it this week?",
        time: "Yesterday",
        unread: false,
    },
    Message {
        sender: "Figma",
        subject: "Your weekly summary",
        preview: "3 files updated · 8 comments · 2 new prototypes shared with you.",
        time: "Tuesday",
        unread: false,
    },
    Message {
        sender: "Aria Chen",
        subject: "Re: Q3 roadmap",
        preview: "Pushed the milestones back a week and updated the dates in the doc.",
        time: "Mon",
        unread: false,
    },
];

impl Render for MailApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_mailbox = self.selected_mailbox;
        let selected_message = self.selected_message;

        // Navigation surfaces are translucent (reveal the glass); the message
        // list is opaque.
        let chrome_bg = hsla(240. / 360., 0.12, 0.97, 0.8);
        let chrome_border = hsla(0., 0., 0., 0.04);
        let list_bg = hsla(0., 0., 1., 1.);
        let accent = hsla(211. / 360., 1., 0.52, 1.);
        let ink = hsla(240. / 360., 0.03, 0.12, 1.);
        let secondary = hsla(240. / 360., 0.03, 0.44, 1.);
        let muted = hsla(240. / 360., 0.03, 0.62, 1.);
        let hairline = hsla(240. / 360., 0.05, 0.94, 1.);
        let highlight = hsla(0., 0., 0., 0.06);
        let hover = hsla(0., 0., 0., 0.04);
        let row_hover = hsla(0., 0., 0., 0.025);
        let selected = hsla(211. / 360., 1., 0.52, 0.08);

        let sidebar = div()
            .flex()
            .flex_col()
            .w(px(200.))
            .flex_shrink_0()
            .h_full()
            .px_2()
            .pt_3()
            .gap(px(1.))
            .bg(chrome_bg)
            .border_r(px(1.))
            .border_color(chrome_border)
            // The whole sidebar is glass content.
            .glass()
            .child(
                div()
                    .px_2()
                    .pb_1p5()
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(muted)
                    .child("Mailboxes"),
            )
            .children(MAILBOXES.iter().enumerate().map(|(ix, mailbox)| {
                let is_selected = ix == selected_mailbox;
                div()
                    .id(ix)
                    .flex()
                    .items_center()
                    .h(px(30.))
                    .px_2p5()
                    .rounded(px(7.))
                    .text_sm()
                    // macOS-style: faint neutral highlight + tinted text, not a
                    // heavy solid fill.
                    .when(is_selected, |row| row.bg(highlight))
                    .when(!is_selected, |row| row.hover(|row| row.bg(hover)))
                    .child(
                        div()
                            .flex_1()
                            .text_color(if is_selected { accent } else { ink })
                            .when(is_selected, |t| t.font_weight(FontWeight::MEDIUM))
                            .child(mailbox.name),
                    )
                    .when(mailbox.unread > 0, |row| {
                        row.child(
                            div()
                                .text_xs()
                                .text_color(muted)
                                .child(SharedString::from(mailbox.unread.to_string())),
                        )
                    })
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.selected_mailbox = ix;
                        cx.notify();
                    }))
            }));

        let list = div()
            .flex()
            .flex_col()
            .flex_1()
            // Let the list shrink to its flex width instead of being widened by
            // its content (flexbox `min-width: auto`), otherwise a long row
            // pushes the list past the viewport and clips the right edge.
            .min_w_0()
            .h_full()
            .bg(list_bg)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.))
                    .px_5()
                    .py_4()
                    .border_b_1()
                    .border_color(hairline)
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::BOLD)
                            .text_color(ink)
                            .child(MAILBOXES[selected_mailbox].name),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(muted)
                            .child(format!("{} messages", MESSAGES.len())),
                    ),
            )
            .child(div().flex().flex_col().flex_1().min_h_0().children(
                MESSAGES.iter().enumerate().map(|(ix, message)| {
                    let is_selected = ix == selected_message;
                    div()
                        .id(("message", ix))
                        .flex()
                        .flex_col()
                        .gap(px(3.))
                        .px_5()
                        .py_3()
                        .border_b_1()
                        .border_color(hairline)
                        .when(is_selected, |row| row.bg(selected))
                        .when(!is_selected, |row| row.hover(|row| row.bg(row_hover)))
                        .child(
                            div()
                                .flex()
                                .items_baseline()
                                .gap_2()
                                .child(
                                    div()
                                        .flex_1()
                                        .text_sm()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(ink)
                                        .child(message.sender),
                                )
                                .when(message.unread, |r| {
                                    r.child(div().size(px(7.)).rounded_full().bg(accent))
                                })
                                .child(div().text_xs().text_color(muted).child(message.time)),
                        )
                        .child(div().text_sm().text_color(ink).child(message.subject))
                        .child(
                            div()
                                .text_xs()
                                .text_color(secondary)
                                // Clamp the preview to one line. Unlike
                                // `truncate` (which is nowrap and stretches the
                                // row to the full text width, clipping the time
                                // and right edge), `line_clamp` wraps by width
                                // so the row never grows wider than the list.
                                .line_clamp(1)
                                .child(message.preview),
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.selected_message = ix;
                            cx.notify();
                        }))
                }),
            ));

        div()
            .flex()
            .flex_col()
            .size_full()
            .text_color(ink)
            .child(
                // Title bar: translucent navigation chrome.
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .h(px(44.))
                    .px_5()
                    .bg(chrome_bg)
                    .border_b(px(1.))
                    .border_color(chrome_border)
                    // Mark the chrome as glass content so children (the Compose
                    // button) inherit it: their fills blend RGB but preserve the
                    // chrome's alpha, so the black button reads as "black glass".
                    .glass()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Mailbox"),
                    )
                    .child(
                        // Black button — inherits the chrome's glass content,
                        // so it reads as "black glass" (blends RGB but keeps the
                        // chrome's alpha) instead of a flat black block.
                        div()
                            .id("compose")
                            .flex()
                            .items_center()
                            .h(px(26.))
                            .px_3()
                            .rounded(px(6.))
                            .bg(hsla(0., 0., 0., 1.))
                            .text_color(hsla(0., 0., 1., 1.))
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .child("Compose"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .min_h_0()
                    .child(sidebar)
                    .child(list),
            )
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(760.), px(680.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_background: WindowBackgroundAppearance::Blurred,
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| MailApp {
                    selected_mailbox: 0,
                    selected_message: 0,
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    run_example();
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    gpui_platform::web_init();
    run_example();
}
