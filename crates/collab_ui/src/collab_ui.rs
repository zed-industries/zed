mod collab_titlebar_item;
mod contact_finder;
mod contact_list;
mod contact_notification;
mod contacts_popover;
mod incoming_call_notification;
mod notifications;
mod project_shared_notification;

use call::ActiveCall;
pub use collab_titlebar_item::{CollabTitlebarItem, ToggleCollaborationMenu};
use gpui::MutableAppContext;
use project::Project;
use std::sync::Arc;
use workspace::{AppState, JoinProject, ToggleFollow, Workspace};

pub fn init(app_state: Arc<AppState>, cx: &mut MutableAppContext) {
    collab_titlebar_item::init(cx);
    contact_notification::init(cx);
    contact_list::init(cx);
    contact_finder::init(cx);
    contacts_popover::init(cx);
    incoming_call_notification::init(cx);
    project_shared_notification::init(cx);

    cx.add_global_action(move |action: &JoinProject, cx| {
        let project_id = action.project_id;
        let follow_user_id = action.follow_user_id;
        let app_state = app_state.clone();
        cx.spawn(|mut cx| async move {
            let existing_workspace = cx.update(|cx| {
                cx.window_ids()
                    .filter_map(|window_id| cx.root_view::<Workspace>(window_id))
                    .find(|workspace| {
                        workspace.read(cx).project().read(cx).remote_id() == Some(project_id)
                    })
            });

            let workspace = if let Some(existing_workspace) = existing_workspace {
                existing_workspace
            } else {
                let project = Project::remote(
                    project_id,
                    app_state.client.clone(),
                    app_state.user_store.clone(),
                    app_state.languages.clone(),
                    app_state.fs.clone(),
                    cx.clone(),
                )
                .await?;

                let (_, workspace) = cx.add_window((app_state.build_window_options)(), |cx| {
                    let mut workspace = Workspace::new(
                        Default::default(),
                        0,
                        project,
                        app_state.dock_default_item_factory,
                        cx,
                    );
                    (app_state.initialize_workspace)(&mut workspace, &app_state, cx);
                    workspace
                });
                workspace
            };

            cx.activate_window(workspace.window_id());
            cx.platform().activate(true);

            workspace.update(&mut cx, |workspace, cx| {
                if let Some(room) = ActiveCall::global(cx).read(cx).room().cloned() {
                    let follow_peer_id = room
                        .read(cx)
                        .remote_participants()
                        .iter()
                        .find(|(_, participant)| participant.user.id == follow_user_id)
                        .map(|(_, p)| p.peer_id)
                        .or_else(|| {
                            // If we couldn't follow the given user, follow the host instead.
                            let collaborator = workspace
                                .project()
                                .read(cx)
                                .collaborators()
                                .values()
                                .find(|collaborator| collaborator.replica_id == 0)?;
                            Some(collaborator.peer_id)
                        });

                    if let Some(follow_peer_id) = follow_peer_id {
                        if !workspace.is_following(follow_peer_id) {
                            workspace
                                .toggle_follow(&ToggleFollow(follow_peer_id), cx)
                                .map(|follow| follow.detach_and_log_err(cx));
                        }
                    }
                }
            });

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    });
}
