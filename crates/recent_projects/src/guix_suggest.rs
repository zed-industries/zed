use db::kvp::KEY_VALUE_STORE;
use gpui::{SharedString, WeakEntity, Window};
use project::{Project, WorktreeId};
use std::sync::LazyLock;
use ui::prelude::*;
use util::rel_path::RelPath;
use workspace::Workspace;
use workspace::notifications::NotificationId;
use workspace::notifications::simple_message_notification::MessageNotification;
use worktree::UpdatedEntriesSet;

const GUIX_CONTAINER_SUGGEST_KEY: &str = "guix_container_suggest_dismissed";

fn manifest_path() -> &'static RelPath {
    static PATH: LazyLock<&'static RelPath> =
        LazyLock::new(|| RelPath::unix("manifest.scm").expect("valid path"));
    *PATH
}

fn project_guix_key(project_path: &str) -> String {
    format!("{}_{}", GUIX_CONTAINER_SUGGEST_KEY, project_path)
}

pub fn suggest_on_worktree_updated(
    worktree_id: WorktreeId,
    updated_entries: &UpdatedEntriesSet,
    project: &gpui::Entity<Project>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let manifest_updated = updated_entries
        .iter()
        .any(|(path, _, _)| path.as_ref() == manifest_path());

    if !manifest_updated {
        return;
    }

    let Some(worktree) = project.read(cx).worktree_for_id(worktree_id, cx) else {
        return;
    };

    let worktree = worktree.read(cx);

    if !worktree.is_local() {
        return;
    }

    let abs_path = worktree.abs_path();
    let project_path = abs_path.to_string_lossy().to_string();
    let manifest_path = abs_path.join("manifest.scm");
    let key_for_dismiss = project_guix_key(&project_path);

    let already_dismissed = KEY_VALUE_STORE
        .read_kvp(&key_for_dismiss)
        .ok()
        .flatten()
        .is_some();

    if already_dismissed {
        return;
    }

    cx.on_next_frame(window, move |workspace, _window, cx| {
        let workspace_weak: WeakEntity<Workspace> = workspace.weak_handle();
        struct GuixContainerSuggestionNotification;

        let notification_id = NotificationId::composite::<GuixContainerSuggestionNotification>(
            SharedString::from(project_path.clone()),
        );

        workspace.show_notification(notification_id, cx, |cx| {
            cx.new(move |cx| {
                MessageNotification::new(
                    "This project contains a Guix manifest. Would you like to re-open it in a container?",
                    cx,
                )
                .primary_message("Yes, Open in Container")
                .primary_icon(IconName::Check)
                .primary_icon_color(Color::Success)
                .primary_on_click({
                    move |window, cx| {
                        window.dispatch_action(Box::new(zed_actions::OpenGuixContainer), cx);
                    }
                })
                .more_info_message("Open manifest.scm")
                .more_info_on_click({
                    let workspace_weak = workspace_weak.clone();
                    let manifest_path = manifest_path.clone();
                    move |window, cx| {
                        let Some(workspace) = workspace_weak.upgrade() else {
                            return;
                        };

                        workspace.update(cx, |_workspace, cx| {
                            let manifest_path = manifest_path.clone();
                            cx.spawn_in(window, async move |workspace, cx| {
                                workspace
                                    .update_in(cx, |workspace, window, cx| {
                                        workspace.open_abs_path(
                                            manifest_path.clone(),
                                            Default::default(),
                                            window,
                                            cx,
                                        )
                                    })?
                                    .await
                            })
                            .detach();
                        });
                    }
                })
                .secondary_message("Don't Show Again")
                .secondary_icon(IconName::Close)
                .secondary_icon_color(Color::Error)
                .secondary_on_click({
                    move |_window, cx| {
                        let key = key_for_dismiss.clone();
                        db::write_and_log(cx, move || {
                            KEY_VALUE_STORE.write_kvp(key, "dismissed".to_string())
                        });
                    }
                })
            })
        });
    });
}
