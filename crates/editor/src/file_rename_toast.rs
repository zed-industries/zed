use std::sync::Arc;

use gpui::{App, Entity};
use notifications::status_toast::StatusToast;
use project::{FileRename, Project};
use ui::prelude::*;
use workspace::Workspace;

/// Tells the user that undoing a rename left its files under their new names,
/// and offers to move them back.
///
/// The move is not part of the undo. A workspace edit becomes one buffer
/// transaction per file it touched, and those are undone independently, so
/// there is no single point at which the edit is "undone" and the files can
/// safely follow. Asking is the honest way out of that.
///
/// Redo has no counterpart. Once the files have been moved back the record of
/// where the edit put them is dropped, so redoing the text leaves the files
/// where the user just asked for them to be. Offering to move them forward
/// again would mean tracking a second direction, which is more than this is
/// trying to do.
pub fn show_files_left_renamed_toast(
    workspace: &Entity<Workspace>,
    project: Entity<Project>,
    renames: Arc<Vec<FileRename>>,
    window: &mut Window,
    cx: &mut App,
) {
    let message = match renames.as_slice() {
        [rename] => format!(
            "Undo reverted the text. `{}` is still named `{}`.",
            rename.old_path.path.as_unix_str(),
            rename.new_path.path.as_unix_str()
        ),
        renames => format!(
            "Undo reverted the text. {} files are still under their new names.",
            renames.len()
        ),
    };

    let project = project.downgrade();
    let status_toast = StatusToast::new(message, cx, move |this, _cx| {
        this.icon(
            Icon::new(IconName::Undo)
                .size(IconSize::Small)
                .color(Color::Muted),
        )
        .action("Move back", move |_window, cx| {
            if let Some(project) = project.upgrade() {
                project
                    .update(cx, |project, cx| {
                        project.move_renamed_files_back(renames.clone(), cx)
                    })
                    .detach_and_log_err(cx);
            }
        })
        .dismiss_button(true)
        // The default is to disappear after ten seconds. This one offers to
        // repair a state the user did not ask for, so it waits to be answered.
        .auto_dismiss(false)
    });

    workspace.update(cx, |workspace, cx| {
        workspace.toggle_status_toast(status_toast, cx);
    });
    let _ = window;
}
