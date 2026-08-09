use gpui::{App, actions};
use workspace::Workspace;

mod new_project;
mod scaffold;

actions!(
    citadel_new_project,
    [
        /// Scaffolds a new Citadel project into an empty folder and opens it.
        NewProject
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _action: &NewProject, window, cx| {
            new_project::new_project(workspace.weak_handle(), window, cx);
        });
    })
    .detach();
}
