#![cfg_attr(target_family = "wasm", no_main)]

use gpui::{
    App, Bounds, Context, SharedString, SystemNotification, SystemNotificationId,
    SystemNotificationPermission, SystemNotificationPriority, Task, Window, WindowBounds,
    WindowOptions, div, prelude::*, px, rgb, size,
};
use gpui_platform::application;

struct SystemNotificationsExample {
    status: SharedString,
    counter: usize,
}

impl SystemNotificationsExample {
    fn new() -> Self {
        Self {
            status: "Ready".into(),
            counter: 0,
        }
    }

    fn notification_id() -> SystemNotificationId {
        SystemNotificationId("gpui.example.system-notification".into())
    }

    fn check_permission(&mut self, cx: &mut Context<Self>) {
        let task = cx.system_notification_permission();
        self.await_permission("permission", task, cx);
    }

    fn request_permission(&mut self, cx: &mut Context<Self>) {
        let task = cx.request_system_notification_permission();
        self.await_permission("request", task, cx);
    }

    fn show_notification(&mut self, cx: &mut Context<Self>) {
        self.counter += 1;
        let notification = SystemNotification {
            id: Self::notification_id(),
            title: "GPUI system notification".into(),
            body: Some(format!("Shown from the GPUI example {} time(s).", self.counter).into()),
            priority: SystemNotificationPriority::Normal,
        };

        let task = cx.show_system_notification(notification);
        self.await_result("show/update", task, cx);
    }

    fn remove_notification(&mut self, cx: &mut Context<Self>) {
        let task = cx.remove_system_notification(Self::notification_id());
        self.await_result("remove", task, cx);
    }

    fn await_permission(
        &mut self,
        label: &'static str,
        task: Task<anyhow::Result<SystemNotificationPermission>>,
        cx: &mut Context<Self>,
    ) {
        self.status = format!("{label}: waiting").into();
        cx.notify();
        cx.spawn(async move |this, cx| {
            let status = match task.await {
                Ok(permission) => format!("{label}: {permission:?}"),
                Err(error) => format!("{label} failed: {error:#}"),
            };
            this.update(cx, |this, cx| {
                this.status = status.into();
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    fn await_result(
        &mut self,
        label: &'static str,
        task: Task<anyhow::Result<()>>,
        cx: &mut Context<Self>,
    ) {
        self.status = format!("{label}: waiting").into();
        cx.notify();
        cx.spawn(async move |this, cx| {
            let status = match task.await {
                Ok(()) => format!("{label}: ok"),
                Err(error) => format!("{label} failed: {error:#}"),
            };
            this.update(cx, |this, cx| {
                this.status = status.into();
                cx.notify();
            })
            .ok();
        })
        .detach();
    }
}

impl Render for SystemNotificationsExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .size_full()
            .p_6()
            .bg(rgb(0xf7f8f3))
            .text_color(rgb(0x1f2933))
            .child(div().text_2xl().child("System notifications"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_2()
                    .child(
                        div()
                            .id("check-permission")
                            .px_3()
                            .py_2()
                            .rounded_sm()
                            .bg(rgb(0xe1e7dd))
                            .hover(|style| style.bg(rgb(0xd5dfcf)))
                            .child("Check permission")
                            .on_click(cx.listener(|this, _, _, cx| this.check_permission(cx))),
                    )
                    .child(
                        div()
                            .id("request-permission")
                            .px_3()
                            .py_2()
                            .rounded_sm()
                            .bg(rgb(0xf0dfb8))
                            .hover(|style| style.bg(rgb(0xe8d39f)))
                            .child("Request permission")
                            .on_click(cx.listener(|this, _, _, cx| this.request_permission(cx))),
                    )
                    .child(
                        div()
                            .id("show-notification")
                            .px_3()
                            .py_2()
                            .rounded_sm()
                            .bg(rgb(0x1d6f9f))
                            .text_color(rgb(0xffffff))
                            .hover(|style| style.bg(rgb(0x155c86)))
                            .child("Show/update")
                            .on_click(cx.listener(|this, _, _, cx| this.show_notification(cx))),
                    )
                    .child(
                        div()
                            .id("remove-notification")
                            .px_3()
                            .py_2()
                            .rounded_sm()
                            .bg(rgb(0xf2c8bd))
                            .hover(|style| style.bg(rgb(0xeeb7aa)))
                            .child("Remove")
                            .on_click(cx.listener(|this, _, _, cx| this.remove_notification(cx))),
                    ),
            )
            .child(
                div()
                    .id("status")
                    .p_3()
                    .rounded_sm()
                    .bg(rgb(0xffffff))
                    .text_color(rgb(0x314154))
                    .child(self.status.clone()),
            )
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(520.), px(240.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| SystemNotificationsExample::new()),
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
