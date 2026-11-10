use crate::AgentPanel;
use acp_thread::SubmissionResponse;
use agent_client_protocol::schema::{v1 as acp_v1, v2 as acp_v2};
use anyhow::{Context as _, Result, bail, ensure};
use editor::Editor;
use gpui::{App, Context, Window, actions};
use serde::Deserialize;
use serde_json::json;
use std::rc::Rc;
use util::ResultExt as _;
use uuid::Uuid;
use workspace::{Toast, Workspace, notifications::NotificationId};

const REVIEW_META_KEY: &str = "zed-pi-live/review";

actions!(
    agent,
    [
        /// Comment on selected file lines and send immediately to the active pi-live session.
        CommentWithPiLive,
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(
            |workspace: &mut Workspace, _: &CommentWithPiLive, window, cx| {
                if let Err(error) = show_comment(workspace, window, cx) {
                    workspace.show_toast(
                        Toast::new(
                            NotificationId::unique::<CommentWithPiLive>(),
                            error.to_string(),
                        ),
                        cx,
                    );
                }
            },
        );
    })
    .detach();
}

fn show_comment(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> Result<()> {
    let editor = workspace
        .active_item_as::<Editor>(cx)
        .context("Open a regular file and select the lines to review")?;
    let panel = workspace
        .panel::<AgentPanel>(cx)
        .context("Open the Agent Panel and attach pi-live first")?;
    let thread = panel
        .read(cx)
        .active_agent_thread(cx)
        .context("Start and attach a pi-live thread first")?;
    ensure!(
        thread.read(cx).connection().agent_id().as_ref() == "pi-live",
        "Select an attached pi-live thread in the Agent Panel first"
    );
    let target_editor = editor.downgrade();
    editor.update(cx, |editor, cx| {
        editor.show_inline_review(Rc::new(move |comment, window, cx| {
            let thread_id = Uuid::new_v4().to_string();
            let submission_id = Uuid::new_v4().to_string();
            let text = format!("Review comment on {}:{}-{}\n\nSelected code (editor snapshot):\n{}\n\nComment:\n{}",
                comment.path.display(), comment.lines.start + 1, comment.lines.end, comment.excerpt, comment.comment);
            let mut meta = serde_json::Map::new();
            meta.insert(REVIEW_META_KEY.into(), json!({ "version": 1, "threadId": thread_id, "submissionId": submission_id }));
            let content = acp_v2::ContentBlock::Text(acp_v2::TextContent::new(text).meta(meta));
            let submission = thread.update(cx, |thread, cx| thread.send(vec![content], cx));
            let target_editor = target_editor.clone();
            let thread = thread.clone();
            window.spawn(cx, async move |cx| {
                let result = match submission.await {
                    Ok(Some(SubmissionResponse::LegacyCompleted(response))) => correlated_reply(response, &thread_id, &submission_id),
                    Ok(_) => Err(anyhow::anyhow!("No completed inline reply received")),
                    Err(error) => Err(error),
                };
                let reply = match result {
                    Ok(text) => format!("Pi\n{text}"),
                    Err(error) => format!("Reply unavailable: {error}\nCheck the agent transcript before retrying; this comment was not resent."),
                };
                target_editor.update_in(cx, |editor, window, cx| editor.set_inline_review_reply(comment.id, reply, window, cx)).log_err();
                drop(thread);
            }).detach();
        }), window, cx)
    })
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReviewReply {
    version: u32,
    thread_id: String,
    submission_id: String,
    text: String,
}

fn correlated_reply(
    response: acp_v1::PromptResponse,
    thread_id: &str,
    submission_id: &str,
) -> Result<String> {
    if response.stop_reason != acp_v1::StopReason::EndTurn {
        bail!(
            "Pi did not complete the review ({:?})",
            response.stop_reason
        );
    }
    let meta = response
        .meta
        .and_then(|mut meta| meta.remove(REVIEW_META_KEY))
        .context("Pi did not return inline-review metadata; rebuild and reload the bridge")?;
    let reply: ReviewReply = serde_json::from_value(meta).context("Invalid inline-review reply")?;
    ensure!(
        reply.version == 1 && reply.thread_id == thread_id && reply.submission_id == submission_id,
        "Inline-review reply belongs to a different submission or protocol version"
    );
    Ok(reply.text)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn response(thread: &str, submission: &str) -> acp_v1::PromptResponse {
        let mut meta = serde_json::Map::new();
        meta.insert(
            REVIEW_META_KEY.into(),
            json!({"version": 1, "threadId": thread, "submissionId": submission, "text": "reply"}),
        );
        acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn).meta(meta)
    }

    #[test]
    fn replies_require_exact_correlation() {
        assert_eq!(
            correlated_reply(response("a", "b"), "a", "b").unwrap(),
            "reply"
        );
        assert!(correlated_reply(response("a", "b"), "other", "b").is_err());
        assert!(correlated_reply(response("a", "b"), "a", "other").is_err());
        assert!(
            correlated_reply(
                acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn),
                "a",
                "b"
            )
            .is_err()
        );
        let mut cancelled = response("a", "b");
        cancelled.stop_reason = acp_v1::StopReason::Cancelled;
        assert!(correlated_reply(cancelled, "a", "b").is_err());
    }
}
