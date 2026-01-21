use std::rc::Rc;

use db::kvp::KEY_VALUE_STORE;
use gpui::{App, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Window};
use ui::{Button, Color, Icon, IconName, IconSize, Label, SharedString, h_flex, prelude::*};
use util::ResultExt;
use workspace::{ToastAction, ToastView, Workspace};

const WSL_WELCOME_SHOWN: &str = "wsl_welcome_shown";

pub(crate) fn maybe_show_wsl_welcome(
    workspace: Entity<Workspace>,
    _window: &mut Window,
    cx: &mut App,
) {
    if KEY_VALUE_STORE
        .read_kvp(WSL_WELCOME_SHOWN)
        .log_err()
        .flatten()
        .is_some()
    {
        return;
    }

    let toast = WslWelcomeToast::new(cx);
    workspace.update(cx, |workspace, cx| {
        workspace.toggle_status_toast(toast, cx);
    });

    cx.spawn(async |_| {
        KEY_VALUE_STORE
            .write_kvp(WSL_WELCOME_SHOWN.to_string(), "true".to_string())
            .await
    })
    .detach_and_log_err(cx);
}

#[derive(RegisterComponent)]
struct WslWelcomeToast {
    action: Option<ToastAction>,
    this_handle: Entity<Self>,
    focus_handle: FocusHandle,
}

impl WslWelcomeToast {
    fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let focus_handle = cx.focus_handle();
            let this_handle = cx.entity();
            let action = ToastAction::new(
                SharedString::from("Install CLI in WSL"),
                Some(Rc::new(move |window, cx| {
                    this_handle.update(cx, |_, cx| {
                        cx.emit(DismissEvent);
                    });
                    run_install_wsl(window, cx);
                })),
            );
            Self {
                action: Some(action),
                this_handle,
                focus_handle,
            }
        })
    }
}

impl Render for WslWelcomeToast {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .id("wsl-welcome-toast")
            .elevation_3(cx)
            .gap_2()
            .py_1p5()
            .pl_2p5()
            .pr_1p5()
            .flex_none()
            .bg(cx.theme().colors().surface_background)
            .shadow_lg()
            .child(Icon::new(IconName::Linux).size(IconSize::Small))
            .child(Label::new("WSL connected. Install the CLI for terminal workflows."))
            .when_some(self.action.as_ref(), |this, action| {
                this.child(
                    Button::new(action.id.clone(), action.label.clone())
                        .color(Color::Muted)
                        .when_some(action.on_click.clone(), |el, handler| {
                            el.on_click(move |_event, window, cx| handler(window, cx))
                        }),
                )
            })
    }
}

impl ToastView for WslWelcomeToast {
    fn action(&self) -> Option<ToastAction> {
        self.action.clone()
    }
}

impl Focusable for WslWelcomeToast {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for WslWelcomeToast {}

fn run_install_wsl(_window: &mut Window, cx: &mut App) {
    let cli_path = match util::get_zed_cli_path() {
        Ok(path) => path,
        Err(err) => {
            log::error!("Failed to locate zed CLI: {err}");
            return;
        }
    };

    cx.background_spawn(async move {
        let status = smol::process::Command::new(cli_path)
            .arg("--install-wsl")
            .status()
            .await?;
        if !status.success() {
            log::error!("zed --install-wsl exited with status {}", status);
        }
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}
