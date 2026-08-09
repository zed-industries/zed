pub mod board_detect;
mod board_picker;
pub mod board_registry;
pub mod build_pipeline;

use std::path::Path;
use std::sync::Arc;

use board_detect::{BoardIdentity, GlobalBoardMonitor, UnverifiedChipDetected};
use board_registry::{avrdude_defaults, board_kind_from_display_name};
use build_pipeline::{BuildTarget, build_and_flash};
use gpui::{Action, App, AssetSource, Context, SharedString, Subscription, WeakEntity, actions};
use notifications::status_toast::StatusToast;
use ui::prelude::*;
use util::ResultExt;
use workspace::{ItemHandle, StatusItemView, Workspace};

actions!(
    citadel_build,
    [
        /// Builds the current project's cpp/ and rust/ sources and flashes the result to the connected board.
        BuildAndUpload
    ]
);

pub fn init(cx: &mut App) {
    board_detect::init(cx);

    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else { return };

        let indicator = cx.new(|cx| BoardIndicator::new(workspace, cx));
        workspace.status_bar().update(cx, |status_bar, cx| {
            status_bar.add_right_item(indicator, window, cx);
        });

        workspace.register_action(|workspace, _: &BuildAndUpload, _window, cx| {
            start_build_and_upload(workspace, cx);
        });
    })
    .detach();
}

/// Shows a dismissible error toast in `workspace`. Duplicated (not shared)
/// from `citadel_new_project::new_project`'s helper of the same name and
/// shape, per this crate's Global Constraints against cross-crate UI
/// coupling for a single small helper.
fn show_error_toast_in_workspace(
    workspace: &mut Workspace,
    message: impl Into<SharedString>,
    cx: &mut Context<Workspace>,
) {
    let toast = StatusToast::new(message, cx, |this, _| {
        this.icon(
            Icon::new(IconName::XCircle)
                .size(IconSize::Small)
                .color(Color::Error),
        )
        .dismiss_button(true)
    });
    workspace.toggle_status_toast(toast, cx);
}

fn show_success_toast_in_workspace(
    workspace: &mut Workspace,
    message: impl Into<SharedString>,
    cx: &mut Context<Workspace>,
) {
    let toast = StatusToast::new(message, cx, |this, _| {
        this.icon(
            Icon::new(IconName::Check)
                .size(IconSize::Small)
                .color(Color::Success),
        )
        .dismiss_button(true)
    });
    workspace.toggle_status_toast(toast, cx);
}

fn start_build_and_upload(workspace: &mut Workspace, cx: &mut Context<Workspace>) {
    let Some(monitor) = cx
        .try_global::<GlobalBoardMonitor>()
        .map(|global| global.0.clone())
    else {
        show_error_toast_in_workspace(workspace, "No board detected. Connect a board first.", cx);
        return;
    };

    let Some(detected) = monitor.read(cx).detected.clone() else {
        show_error_toast_in_workspace(workspace, "No board detected. Connect a board first.", cx);
        return;
    };

    let Some(mmcu) = detected.mmcu else {
        show_error_toast_in_workspace(
            workspace,
            "Could not read the chip signature yet. Wait a moment and try again.",
            cx,
        );
        return;
    };

    let board_kind = match &detected.identity {
        BoardIdentity::Known(name) => board_kind_from_display_name(name),
        BoardIdentity::Unknown => None,
    };
    let Some(board_kind) = board_kind else {
        show_error_toast_in_workspace(
            workspace,
            "Unknown board. Click the board indicator to identify it first.",
            cx,
        );
        return;
    };

    let Some(worktree) = workspace.project().read(cx).visible_worktrees(cx).next() else {
        show_error_toast_in_workspace(workspace, "Open a project folder first.", cx);
        return;
    };
    let project_root = worktree.read(cx).abs_path().to_path_buf();

    if !project_root.join("rust").is_dir() || !project_root.join("cpp").is_dir() {
        show_error_toast_in_workspace(
            workspace,
            "Not a Citadel project: missing a rust/ or cpp/ folder.",
            cx,
        );
        return;
    }

    let (programmer, baud) = avrdude_defaults(board_kind);
    let core_dir = paths::data_dir()
        .join("citadel_build")
        .join("arduino-core-1.8.8");
    let target = BuildTarget {
        project_root,
        core_source_dir: core_dir.clone(),
        core_cache_dir: core_dir.clone(),
        mmcu: mmcu.to_string(),
        port_name: detected.port_name.clone(),
        avrdude_programmer: programmer.to_string(),
        avrdude_baud: baud,
    };
    let asset_source = cx.asset_source().clone();

    cx.spawn(async move |workspace, cx| {
        let extract_result = cx
            .background_spawn({
                let core_dir = core_dir.clone();
                async move { extract_core_sources_if_needed(asset_source.as_ref(), &core_dir) }
            })
            .await;

        if let Err(error) = extract_result {
            workspace
                .update(cx, |workspace, cx| {
                    show_error_toast_in_workspace(workspace, error.to_string(), cx);
                })
                .log_err();
            return;
        }

        let build_result = cx.background_spawn(build_and_flash(target)).await;

        workspace
            .update(cx, |workspace, cx| match build_result {
                Ok(hex_path) => show_success_toast_in_workspace(
                    workspace,
                    format!("Build and upload succeeded: {}", hex_path.display()),
                    cx,
                ),
                Err(error) => show_error_toast_in_workspace(workspace, error.to_string(), cx),
            })
            .log_err();
    })
    .detach();
}

/// Extracts the embedded ArduinoCore-avr sources into `dest_dir`, unless
/// they're already there. `dest_dir` is paid for once (per Citadel
/// install/data-dir), not once per build.
fn extract_core_sources_if_needed(
    asset_source: &dyn AssetSource,
    dest_dir: &Path,
) -> anyhow::Result<()> {
    if dest_dir
        .join("cores")
        .join("arduino")
        .join("main.cpp")
        .exists()
    {
        return Ok(());
    }

    const ASSET_DIR: &str = "arduino-core/ArduinoCore-avr";
    let asset_prefix = format!("{ASSET_DIR}/");

    for asset_path in asset_source.list(ASSET_DIR)? {
        let Some(relative_path) = asset_path.strip_prefix(&asset_prefix) else {
            continue;
        };
        let Some(contents) = asset_source.load(&asset_path)? else {
            continue;
        };

        let dest_path = dest_dir.join(relative_path);
        if let Some(parent) = dest_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&dest_path, contents.as_ref())?;
    }

    Ok(())
}

/// Status-bar item showing the currently connected board (or an
/// unknown-board prompt) plus a "Build and Upload" button. Board detection
/// is workspace-global rather than tab-scoped, so this renders the same
/// regardless of the active pane item.
pub struct BoardIndicator {
    workspace: WeakEntity<Workspace>,
    _subscriptions: Vec<Subscription>,
}

impl BoardIndicator {
    fn new(workspace: &Workspace, cx: &mut Context<Self>) -> Self {
        let monitor = cx.global::<GlobalBoardMonitor>().0.clone();
        let observe_subscription = cx.observe(&monitor, |_, _, cx| cx.notify());
        let subscribe_subscription = cx.subscribe(
            &monitor,
            |this, _monitor, _event: &UnverifiedChipDetected, cx| {
                this.show_unverified_chip_toast(cx);
            },
        );

        Self {
            workspace: workspace.weak_handle(),
            _subscriptions: vec![observe_subscription, subscribe_subscription],
        }
    }

    fn show_unverified_chip_toast(&self, cx: &mut App) {
        self.workspace
            .update(cx, |workspace, cx| {
                show_error_toast_in_workspace(
                    workspace,
                    "Connected chip does not match a verified part signature. Builds may not work correctly.",
                    cx,
                );
            })
            .log_err();
    }
}

impl Render for BoardIndicator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(monitor) = cx.try_global::<GlobalBoardMonitor>() else {
            return div();
        };
        let Some(detected) = monitor.0.read(cx).detected.clone() else {
            return div();
        };

        let board_label = match &detected.identity {
            BoardIdentity::Known(name) => format!("Board: {} ({})", name, detected.port_name),
            BoardIdentity::Unknown => {
                format!("Unknown board ({}) — click to identify", detected.port_name)
            }
        };

        let workspace = self.workspace.clone();
        let vid_pid = detected.vid_pid;

        h_flex()
            .gap_1()
            .child(
                Button::new("citadel-board-indicator", board_label)
                    .label_size(LabelSize::Small)
                    .on_click(move |_, window, cx| {
                        let monitor = cx.global::<GlobalBoardMonitor>().0.clone();
                        let on_picked: Arc<dyn Fn(&str, &mut App) + Send + Sync> =
                            Arc::new(move |name, cx| {
                                let name = name.to_string();
                                monitor.update(cx, |monitor, cx| {
                                    if let Some(detected) = monitor.detected.as_mut() {
                                        detected.identity = BoardIdentity::Known(name);
                                    }
                                    cx.notify();
                                });
                            });
                        board_picker::BoardPicker::toggle(
                            vid_pid, on_picked, &workspace, window, cx,
                        );
                    }),
            )
            .child(
                Button::new("citadel-build-and-upload", "Build and Upload")
                    .label_size(LabelSize::Small)
                    .start_icon(Icon::new(IconName::PlayFilled))
                    .on_click(|_, window, cx| {
                        window.dispatch_action(BuildAndUpload.boxed_clone(), cx);
                    }),
            )
    }
}

impl StatusItemView for BoardIndicator {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    fn hide_setting(&self, _: &App) -> Option<workspace::HideStatusItem> {
        None
    }
}
