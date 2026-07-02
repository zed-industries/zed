use db::kvp::KeyValueStore;
use gpui::{SharedString, Window};
use project::{Project, WorktreeId};
use std::cell::Cell;
use std::rc::Rc;
use ui::Tooltip;
use ui::prelude::*;
use util::ResultExt;
use workspace::notifications::NotificationId;
use workspace::notifications::simple_message_notification::MessageNotification;
use workspace::{Workspace, WorkspaceId};

const FLATPAK_SANDBOX_NOTICE_DISMISSED_KEY: &str = "flatpak_sandbox_notice_dismissed";

/// Documentation covering how to enable (or disable) the Flatpak host transport.
const FLATPAK_DOCS_URL: &str =
    "https://zed.dev/docs/linux#flatpak-sandbox-is-preventing-host-access";

/// Returns the key used to remember the user's "Don't Show Again" choice.
fn dismissed_key(workspace_id: WorkspaceId) -> String {
    format!(
        "{}_{}",
        FLATPAK_SANDBOX_NOTICE_DISMISSED_KEY,
        i64::from(workspace_id)
    )
}

/// Reports that the current project is open inside a Flatpak sandbox and suggests next
/// steps for the user to get to a usable state.
pub fn maybe_notify(
    workspace: &mut Workspace,
    worktree_id: WorktreeId,
    project: &gpui::Entity<Project>,
    already_notified: &Rc<Cell<bool>>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    if already_notified.get() {
        return;
    }

    // This notice only makes sense from inside the sandbox, and only for a
    // project that is currently local to the sandbox.
    let Some(metadata) = util::flatpak::CURRENT_SANDBOX_METADATA.as_ref() else {
        return;
    };
    if !project.read(cx).is_local() {
        return;
    }

    let Some(worktree) = project.read(cx).worktree_for_id(worktree_id, cx) else {
        return;
    };
    let worktree = worktree.read(cx);
    if !worktree.is_local() {
        return;
    }

    let Some(workspace_id) = workspace.database_id() else {
        // Workspaces hasn't been persisted yet, delay until a later notification
        return;
    };

    let key_for_dismiss = dismissed_key(workspace_id);
    let already_dismissed = KeyValueStore::global(cx)
        .read_kvp(&key_for_dismiss)
        .ok()
        .flatten()
        .is_some();

    already_notified.set(true);
    if already_dismissed {
        return;
    }

    let can_spawn_on_host = metadata.can_spawn_on_host();
    let project_path = worktree.abs_path().to_string_lossy().to_string();
    let worktree_name = worktree.root_name_str().to_string();

    cx.on_next_frame(window, move |workspace, _window, cx| {
        struct FlatpakSandboxNotice;

        let notification_id = NotificationId::unique::<FlatpakSandboxNotice>();

        workspace.show_notification(notification_id, cx, |cx| {
            cx.new(move |cx| {
                let message: SharedString =
                    format!("{worktree_name} is currently open in Zed's Flatpak sandbox.").into();
                let tooltip_text: SharedString = project_path.clone().into();
                MessageNotification::new_from_builder(cx, move |_window, _cx| {
                    div()
                        .id("flatpak-sandbox-notice-message")
                        .child(Label::new(message.clone()))
                        .tooltip(Tooltip::text(tooltip_text.clone()))
                        .into_any_element()
                })
                .primary_message("Reopen on Local Host")
                .primary_icon(IconName::Screen)
                .primary_icon_color(Color::Success)
                .primary_disabled(!can_spawn_on_host)
                .when(!can_spawn_on_host, |this| {
                    this.primary_tooltip("Flatpak sandbox is preventing host access")
                })
                .primary_on_click(move |window, cx| {
                    window.dispatch_action(Box::new(zed_actions::ReopenAsLocal), cx);
                })
                .more_info_message("Learn More")
                .more_info_url(FLATPAK_DOCS_URL)
                .secondary_message("Don't Show Again")
                .secondary_icon(IconName::Close)
                .secondary_icon_color(Color::Error)
                .secondary_on_click(move |_window, cx| {
                    let key = key_for_dismiss.clone();
                    let kvp = KeyValueStore::global(cx);
                    cx.background_spawn(async move {
                        kvp.write_kvp(key, "dismissed".to_string()).await.log_err();
                    })
                    .detach();
                })
            })
        });
    });
}
