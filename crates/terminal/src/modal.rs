use gpui::{ViewContext, ViewHandle};
use workspace::Workspace;

use crate::{DeployModal, Event, Terminal};

pub fn deploy_modal(workspace: &mut Workspace, _: &DeployModal, cx: &mut ViewContext<Workspace>) {
    if let Some(stored_terminal) = cx.default_global::<Option<ViewHandle<Terminal>>>().clone() {
        workspace.toggle_modal(cx, |_, _| stored_terminal);
    } else {
        let project = workspace.project().read(cx);
        let abs_path = project
            .active_entry()
            .and_then(|entry_id| project.worktree_for_entry(entry_id, cx))
            .and_then(|worktree_handle| worktree_handle.read(cx).as_local())
            .map(|wt| wt.abs_path().to_path_buf());

        let displaced_modal = workspace.toggle_modal(cx, |_, cx| {
            let this = cx.add_view(|cx| Terminal::new(cx, abs_path, true));
            cx.subscribe(&this, on_event).detach();
            this
        });
        cx.set_global(displaced_modal);
    }
}

pub fn on_event(
    workspace: &mut Workspace,
    _: ViewHandle<Terminal>,
    event: &Event,
    cx: &mut ViewContext<Workspace>,
) {
    // Dismiss the modal if the terminal quit
    if let Event::CloseTerminal = event {
        cx.set_global::<Option<ViewHandle<Terminal>>>(None);
        if workspace
            .modal()
            .cloned()
            .and_then(|modal| modal.downcast::<Terminal>())
            .is_some()
        {
            workspace.dismiss_modal(cx)
        }
    }
}
