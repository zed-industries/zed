#![cfg_attr(target_family = "wasm", no_main)]

//! Demonstrates posting, replacing, dismissing, and responding to system notifications.

use gpui::{
    App, Bounds, Context, Div, SharedString, Stateful, SystemNotification,
    SystemNotificationAction, SystemNotificationResponse, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};
use gpui_platform::application;

const NOTIFICATION_TAG: &str = "gpui-system-notification-example";

struct SystemNotificationExample {
    revision: usize,
    status: SharedString,
}

impl Render for SystemNotificationExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .gap_4()
            .p_8()
            .bg(rgb(0x18181b))
            .text_color(rgb(0xf4f4f5))
            .child(div().text_2xl().child("GPUI system notifications"))
            .child(div().text_sm().text_color(rgb(0xa1a1aa)).child(
                "Post repeatedly to replace the notification with the same tag. Click the \
                         notification body or an action button to send a response back to GPUI.",
            ))
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(button("show", "Show or replace").on_click(cx.listener(
                        |this, _, _, cx| {
                            this.revision += 1;
                            let revision = this.revision;
                            cx.show_system_notification(SystemNotification {
                                tag: NOTIFICATION_TAG.into(),
                                title: format!("Example notification {revision}").into(),
                                body: "This notification was posted by the GPUI example.".into(),
                                actions: vec![
                                    SystemNotificationAction {
                                        id: "open".into(),
                                        label: "Open".into(),
                                    },
                                    SystemNotificationAction {
                                        id: "snooze".into(),
                                        label: "Snooze".into(),
                                    },
                                ],
                            });
                            this.status = format!("Posted notification revision {revision}").into();
                            cx.notify();
                        },
                    )))
                    .child(
                        button("dismiss", "Dismiss").on_click(cx.listener(|this, _, _, cx| {
                            cx.dismiss_system_notification(NOTIFICATION_TAG);
                            this.status = "Dismissed the notification".into();
                            cx.notify();
                        })),
                    ),
            )
            .child(
                div()
                    .mt_2()
                    .p_4()
                    .rounded_md()
                    .bg(rgb(0x27272a))
                    .child(self.status.clone()),
            )
            .when(cfg!(target_os = "macos"), |this| {
                this.child(div().mt_2().text_xs().text_color(rgb(0x71717a)).child(
                    "macOS only delivers notifications when this example runs from an app bundle.",
                ))
            })
    }
}

fn button(id: &'static str, label: &'static str) -> Stateful<Div> {
    div()
        .id(id)
        .px_4()
        .py_2()
        .rounded_md()
        .bg(rgb(0x3f3f46))
        .hover(|style| style.bg(rgb(0x52525b)))
        .active(|style| style.bg(rgb(0x71717a)))
        .cursor_pointer()
        .child(label)
}

fn run_example() {
    application().run(|cx: &mut App| {
        cx.set_app_identity("dev.zed.gpui.system-notifications", "GPUI Notifications");

        let view = cx.new(|_| SystemNotificationExample {
            revision: 0,
            status: "No notification posted yet".into(),
        });
        cx.on_system_notification_response({
            let view = view.clone();
            move |response, cx| {
                let SystemNotificationResponse { tag, action_id } = response;
                view.update(cx, |this, cx| {
                    this.status = match action_id {
                        Some(action_id) => {
                            format!("Received action '{action_id}' for tag '{tag}'").into()
                        }
                        None => format!("Notification body clicked for tag '{tag}'").into(),
                    };
                    cx.notify();
                });
            }
        });

        let bounds = Bounds::centered(None, size(px(560.), px(360.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("System Notifications Example".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            move |_, _| view,
        )
        .expect("failed to open system notifications example window");
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
