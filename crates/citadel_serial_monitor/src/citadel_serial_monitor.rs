mod plot_parser;
pub mod serial_connection;
mod serial_monitor_panel;
mod serial_plotter_window;

use gpui::{App, Context, SharedString, actions};
use notifications::status_toast::StatusToast;
use ui::prelude::*;
use workspace::Workspace;

actions!(
    citadel_serial_monitor,
    [
        /// Toggles focus on the Serial Monitor dock panel.
        ToggleFocus
    ]
);

pub fn init(cx: &mut App) {
    serial_connection::init(cx);
}

/// Shows a dismissible error toast in `workspace`. Duplicated (not shared)
/// from `citadel_new_project::new_project`/`citadel_build`'s helper of the
/// same name and shape, per the established convention against cross-crate
/// UI coupling for a single small helper.
pub(crate) fn show_error_toast_in_workspace(
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
