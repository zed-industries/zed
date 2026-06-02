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
    fn new(status: SharedString) -> Self {
        Self { status, counter: 0 }
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
        let status = initial_status();
        let bounds = Bounds::centered(None, size(px(520.), px(240.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| SystemNotificationsExample::new(status)),
        )
        .unwrap();
        cx.activate(true);
    });
}

#[cfg(target_os = "windows")]
fn initial_status() -> SharedString {
    match windows_notification_example::setup() {
        Ok(shortcut_path) => format!(
            "Ready (registered Windows toast shortcut at {})",
            shortcut_path.display()
        )
        .into(),
        Err(error) => format!("Windows toast setup failed: {error:#}").into(),
    }
}

#[cfg(not(target_os = "windows"))]
fn initial_status() -> SharedString {
    "Ready".into()
}

#[cfg(target_os = "windows")]
mod windows_notification_example {
    use std::path::PathBuf;

    use anyhow::{Context as _, Result};
    use windows::{
        Win32::{
            Foundation::PROPERTYKEY,
            System::Com::{
                CLSCTX_INPROC_SERVER, CoCreateInstance, CoTaskMemFree, IPersistFile,
                StructuredStorage::PROPVARIANT,
            },
            UI::Shell::{
                FOLDERID_Programs, IShellLinkW, KNOWN_FOLDER_FLAG,
                PropertiesSystem::IPropertyStore, SHGetKnownFolderPath,
                SetCurrentProcessExplicitAppUserModelID, ShellLink,
            },
        },
        core::{GUID, HSTRING, Interface},
    };

    const APP_USER_MODEL_ID: &str = "dev.gpui.SystemNotificationsExample";
    const SHORTCUT_NAME: &str = "GPUI System Notifications Example.lnk";
    const PKEY_APP_USER_MODEL_ID: PROPERTYKEY = PROPERTYKEY {
        fmtid: GUID::from_u128(0x9f4c2855_9f79_4b39_a8d0_e1d42de1d5f3),
        pid: 5,
    };

    pub(super) fn setup() -> Result<PathBuf> {
        let app_user_model_id = HSTRING::from(APP_USER_MODEL_ID);
        unsafe {
            SetCurrentProcessExplicitAppUserModelID(&app_user_model_id)
                .context("setting process AppUserModelID")?;
        }

        let shortcut_path = shortcut_path().context("resolving Start Menu shortcut path")?;
        create_shortcut(&shortcut_path).context("creating Start Menu shortcut")?;
        Ok(shortcut_path)
    }

    fn shortcut_path() -> Result<PathBuf> {
        let programs_path =
            unsafe { SHGetKnownFolderPath(&FOLDERID_Programs, KNOWN_FOLDER_FLAG(0), None)? };
        let programs_path_string = unsafe { programs_path.to_string() };
        unsafe {
            CoTaskMemFree(Some(programs_path.0 as _));
        }

        let mut path = PathBuf::from(programs_path_string?);
        path.push(SHORTCUT_NAME);
        Ok(path)
    }

    fn create_shortcut(shortcut_path: &PathBuf) -> Result<()> {
        if let Some(parent) = shortcut_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating shortcut directory {}", parent.display()))?;
        }

        unsafe {
            let link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;
            let exe_path = HSTRING::from(std::env::current_exe()?.as_os_str());
            link.SetPath(&exe_path)?;
            link.SetDescription(&HSTRING::from("GPUI system notifications example"))?;

            let store: IPropertyStore = link.cast()?;
            let app_user_model_id = PROPVARIANT::from(APP_USER_MODEL_ID);
            store.SetValue(&PKEY_APP_USER_MODEL_ID, &app_user_model_id)?;
            store.Commit()?;

            let persist_file: IPersistFile = link.cast()?;
            persist_file.Save(&HSTRING::from(shortcut_path.as_os_str()), true)?;
        }

        Ok(())
    }
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
