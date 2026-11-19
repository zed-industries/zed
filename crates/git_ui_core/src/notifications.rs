use editor::Editor;
use gpui::{App, Context, Entity, SharedString};
use language::Buffer;
use notifications::status_toast::StatusToast;
use proto::RpcError;
use ui::prelude::*;
use workspace::Workspace;

pub fn open_output(
    operation: impl Into<SharedString>,
    workspace: &mut Workspace,
    output: &str,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let operation = operation.into();

    let plain_text = terminal::strip_ansi_text(output.as_bytes());

    let buffer = cx.new(|cx| Buffer::local(plain_text.as_str(), cx));
    buffer.update(cx, |buffer, cx| {
        buffer.set_capability(language::Capability::ReadOnly, cx);
    });
    let editor = cx.new(|cx| {
        let mut editor = Editor::for_buffer(buffer, None, window, cx);
        editor.buffer().update(cx, |buffer, cx| {
            buffer.set_title(format!("Output from git {operation}"), cx);
        });
        editor.set_read_only(true);
        editor
    });

    workspace.add_item_to_center(Box::new(editor), window, cx);
}

fn is_non_fast_forward_error(error_message: &str) -> bool {
    (error_message.contains("[rejected]")
        && (error_message.contains("(non-fast-forward)")
            || error_message.contains("(fetch first)")))
        || (error_message.contains("Updates were rejected")
            && (error_message.contains("behind") || error_message.contains("remote")))
}

fn git_error_toast_title(action: &str, message: &str) -> String {
    if action == "push" && is_non_fast_forward_error(message) {
        "Push rejected: fetch and integrate remote changes before pushing again".to_string()
    } else {
        format!("git {action} failed")
    }
}

pub fn show_error_toast(
    workspace: Entity<Workspace>,
    action: impl Into<SharedString>,
    e: anyhow::Error,
    cx: &mut App,
) {
    let action = action.into();
    let message = format_git_error_toast_message(&e);
    if message
        .matches(git::repository::REMOTE_CANCELLED_BY_USER)
        .next()
        .is_some()
    { // Hide the cancelled by user message
    } else {
        cx.defer(move |cx| {
            workspace.update(cx, |workspace, cx| {
                let workspace_weak = cx.weak_entity();
                let title = git_error_toast_title(&action, &message);
                let toast = StatusToast::new(title, cx, |this, _cx| {
                    this.icon(
                        Icon::new(IconName::XCircle)
                            .size(IconSize::Small)
                            .color(Color::Error),
                    )
                    .action("View Log", move |window, cx| {
                        let message = message.clone();
                        let action = action.clone();
                        workspace_weak
                            .update(cx, move |workspace, cx| {
                                open_output(action, workspace, &message, window, cx)
                            })
                            .ok();
                    })
                });
                workspace.toggle_status_toast(toast, cx)
            });
        });
    }
}

fn rpc_error_raw_message_from_chain(error: &anyhow::Error) -> Option<&str> {
    error
        .chain()
        .find_map(|cause| cause.downcast_ref::<RpcError>().map(RpcError::raw_message))
}

fn format_git_error_toast_message(error: &anyhow::Error) -> String {
    if let Some(message) = rpc_error_raw_message_from_chain(error) {
        message.trim().to_string()
    } else {
        error.to_string().trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_non_fast_forward_error_standard_format() {
        let error = "error: failed to push some refs to 'origin'\n\
                     To https://github.com/user/repo.git\n\
                     ! [rejected]        main -> main (non-fast-forward)";
        assert!(is_non_fast_forward_error(error));
    }

    #[test]
    fn test_is_non_fast_forward_error_behind_variant() {
        let error = "Updates were rejected because the tip of your current branch is behind\n\
                     its remote counterpart.";
        assert!(is_non_fast_forward_error(error));
    }

    #[test]
    fn test_is_non_fast_forward_error_remote_variant() {
        let error = "Updates were rejected because the remote contains work that you do\n\
                     not have locally.";
        assert!(is_non_fast_forward_error(error));
    }

    #[test]
    fn test_is_non_fast_forward_error_combined_patterns() {
        let error = "! [rejected]        main -> main (non-fast-forward)\n\
                     error: failed to push some refs to 'https://github.com/user/repo.git'\n\
                     hint: Updates were rejected because the remote contains work that you do\n\
                     hint: not have locally.";
        assert!(is_non_fast_forward_error(error));
    }

    #[test]
    fn test_is_non_fast_forward_error_false_positives() {
        assert!(!is_non_fast_forward_error("error: failed to push"));

        assert!(!is_non_fast_forward_error("fatal: Authentication failed"));

        assert!(!is_non_fast_forward_error(
            "error: RPC failed; curl 56 OpenSSL"
        ));

        assert!(!is_non_fast_forward_error("[rejected]"));
        assert!(!is_non_fast_forward_error("Updates were rejected"));
        assert!(!is_non_fast_forward_error("(non-fast-forward)"));

        assert!(!is_non_fast_forward_error("fatal: could not read Username"));
    }

    #[test]
    fn test_is_non_fast_forward_error_edge_cases() {
        assert!(!is_non_fast_forward_error(""));

        assert!(!is_non_fast_forward_error("   "));
        assert!(!is_non_fast_forward_error("\n\n"));

        assert!(!is_non_fast_forward_error("random error message"));

        assert!(!is_non_fast_forward_error(
            "! [REJECTED] main -> main (NON-FAST-FORWARD)"
        ));
    }

    #[test]
    fn test_is_non_fast_forward_error_multiline_real_world() {
        let error = "
            To https://github.com/user/repo.git
             ! [rejected]        feature-branch -> feature-branch (non-fast-forward)
            error: failed to push some refs to 'https://github.com/user/repo.git'
            hint: Updates were rejected because the remote contains work that you do
            hint: not have locally. This is usually caused by another repository pushing
            hint: to the same ref. You may want to first integrate the remote changes
            hint: (e.g., 'git pull ...') before pushing again.
            hint: See the 'Note about fast-forwards' in 'git push --help' for details.
        ";
        assert!(is_non_fast_forward_error(error));
    }

    #[test]
    fn test_format_git_error_toast_message_prefers_raw_rpc_message() {
        let rpc_error = RpcError::from_proto(
            &proto::Error {
                message:
                    "Your local changes to the following files would be overwritten by merge\n"
                        .to_string(),
                code: proto::ErrorCode::Internal as i32,
                tags: Default::default(),
            },
            "Pull",
        );

        let message = format_git_error_toast_message(&rpc_error);
        assert_eq!(
            message,
            "Your local changes to the following files would be overwritten by merge"
        );
    }

    #[test]
    fn test_format_git_error_toast_message_prefers_raw_rpc_message_when_wrapped() {
        let rpc_error = RpcError::from_proto(
            &proto::Error {
                message:
                    "Your local changes to the following files would be overwritten by merge\n"
                        .to_string(),
                code: proto::ErrorCode::Internal as i32,
                tags: Default::default(),
            },
            "Pull",
        );
        let wrapped = rpc_error.context("sending pull request");

        let message = format_git_error_toast_message(&wrapped);
        assert_eq!(
            message,
            "Your local changes to the following files would be overwritten by merge"
        );
    }
}
