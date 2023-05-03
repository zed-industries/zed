mod collab_titlebar_item;
mod collaborator_list_popover;
mod contact_finder;
mod contact_list;
mod contact_notification;
mod contacts_popover;
mod face_pile;
mod incoming_call_notification;
mod notifications;
mod project_shared_notification;
mod sharing_status_indicator;

use anyhow::anyhow;
use call::ActiveCall;
pub use collab_titlebar_item::{CollabTitlebarItem, ToggleContactsMenu};
use gpui::{actions, AppContext, Task};
use std::sync::Arc;
use workspace::{AppState, JoinProject, ToggleFollow, Workspace};

actions!(collab, [ToggleScreenSharing]);

pub fn init(app_state: Arc<AppState>, cx: &mut AppContext) {
    collab_titlebar_item::init(cx);
    contact_notification::init(cx);
    contact_list::init(cx);
    contact_finder::init(cx);
    contacts_popover::init(cx);
    incoming_call_notification::init(cx);
    project_shared_notification::init(cx);
    sharing_status_indicator::init(cx);

    cx.add_global_action(toggle_screen_sharing);
    cx.add_global_action(move |action: &JoinProject, cx| {
        join_project(action, app_state.clone(), cx);
    });
}

pub fn toggle_screen_sharing(_: &ToggleScreenSharing, cx: &mut AppContext) {
    if let Some(room) = ActiveCall::global(cx).read(cx).room().cloned() {
        let toggle_screen_sharing = room.update(cx, |room, cx| {
            if room.is_screen_sharing() {
                Task::ready(room.unshare_screen(cx))
            } else {
                room.share_screen(cx)
            }
        });
        toggle_screen_sharing.detach_and_log_err(cx);
    }
}

fn join_project(action: &JoinProject, app_state: Arc<AppState>, cx: &mut AppContext) {
    let project_id = action.project_id;
    let follow_user_id = action.follow_user_id;
    cx.spawn(|mut cx| async move {
        let existing_workspace = cx
            .window_ids()
            .into_iter()
            .filter_map(|window_id| cx.root_view(window_id)?.clone().downcast::<Workspace>())
            .find(|workspace| {
                cx.read_window(workspace.window_id(), |cx| {
                    workspace.read(cx).project().read(cx).remote_id() == Some(project_id)
                })
                .unwrap_or(false)
            });

        let workspace = if let Some(existing_workspace) = existing_workspace {
            existing_workspace.downgrade()
        } else {
            let active_call = cx.read(ActiveCall::global);
            let room = active_call
                .read_with(&cx, |call, _| call.room().cloned())
                .ok_or_else(|| anyhow!("not in a call"))?;
            let project = room
                .update(&mut cx, |room, cx| {
                    room.join_project(
                        project_id,
                        app_state.languages.clone(),
                        app_state.fs.clone(),
                        cx,
                    )
                })
                .await?;

            let (_, workspace) = cx.add_window(
                (app_state.build_window_options)(None, None, cx.platform().as_ref()),
                |cx| {
                    let mut workspace = Workspace::new(
                        Default::default(),
                        0,
                        project,
                        app_state.dock_default_item_factory,
                        app_state.background_actions,
                        cx,
                    );
                    (app_state.initialize_workspace)(&mut workspace, &app_state, cx);
                    workspace
                },
            );
            workspace.downgrade()
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
                    if !workspace.is_being_followed(follow_peer_id) {
                        workspace
                            .toggle_follow(&ToggleFollow(follow_peer_id), cx)
                            .map(|follow| follow.detach_and_log_err(cx));
                    }
                }
            }
        })?;

        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}
