use crate::AgentPanel;
use acp_thread::{AcpThread, AcpThreadEvent, SubmissionResponse, ThreadStatus};
use agent_client_protocol::schema::{v1 as acp_v1, v2 as acp_v2};
use anyhow::{Context as _, Result, bail, ensure};
use editor::Editor;
use gpui::{App, Context, Entity, EntityId, Global, Subscription, WeakEntity, Window, actions};
use serde::Deserialize;
use serde_json::json;
use std::{
    collections::{HashMap, VecDeque},
    rc::Rc,
};
use util::ResultExt as _;
use uuid::Uuid;
use workspace::{Toast, Workspace, notifications::NotificationId};

const REVIEW_META_KEY: &str = "zed-pi-live/review";

actions!(
    agent,
    [
        /// Comment on selected file lines and queue it for the active pi-live session.
        CommentWithPiLive,
    ]
);

struct ReviewJob {
    editor: WeakEntity<Editor>,
    comment_id: usize,
    discussion_id: String,
    submission_id: String,
    text: String,
}

struct ReviewQueue {
    thread: Entity<AcpThread>,
    pending: VecDeque<ReviewJob>,
    running: bool,
    _subscription: Subscription,
}

#[derive(Default)]
struct ReviewQueues(HashMap<EntityId, ReviewQueue>);
impl Global for ReviewQueues {}

pub fn init(cx: &mut App) {
    cx.set_global(ReviewQueues::default());
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
        editor.show_inline_review(Rc::new(move |comment, _, cx| {
            let text = format!("Review comment on {}:{}-{}\n\nSelected code (editor snapshot):\n{}\n\nComment:\n{}",
                comment.path.display(), comment.lines.start + 1, comment.lines.end, comment.excerpt, comment.comment);
            enqueue(thread.clone(), ReviewJob {
                editor: target_editor.clone(),
                comment_id: comment.id,
                discussion_id: comment.discussion_id,
                submission_id: Uuid::new_v4().to_string(),
                text,
            }, cx);
        }), window, cx)
    })
}

fn enqueue(thread: Entity<AcpThread>, job: ReviewJob, cx: &mut App) {
    let id = thread.entity_id();
    if !cx.global::<ReviewQueues>().0.contains_key(&id) {
        let subscription = cx.subscribe(&thread, move |_, event, cx| {
            if matches!(event, AcpThreadEvent::StatusChanged) {
                cx.defer(move |cx| pump(id, cx));
            }
        });
        cx.global_mut::<ReviewQueues>().0.insert(
            id,
            ReviewQueue {
                thread,
                pending: VecDeque::new(),
                running: false,
                _subscription: subscription,
            },
        );
    }
    let submission_id = job.submission_id.clone();
    job.editor
        .update(cx, |editor, cx| {
            editor.set_inline_review_status(
                job.comment_id,
                "Queued".into(),
                Some(Rc::new(move |cx| cancel_queued(id, &submission_id, cx))),
                cx,
            )
        })
        .log_err();
    if let Some(queue) = cx.global_mut::<ReviewQueues>().0.get_mut(&id) {
        queue.pending.push_back(job);
    }
    pump(id, cx);
}

fn cancel_queued(id: EntityId, submission_id: &str, cx: &mut App) {
    let cancelled = cx
        .global_mut::<ReviewQueues>()
        .0
        .get_mut(&id)
        .and_then(|queue| {
            let index = queue
                .pending
                .iter()
                .position(|job| job.submission_id == submission_id)?;
            queue.pending.remove(index)
        });
    if let Some(job) = cancelled {
        set_status(&job, "Cancelled".into(), cx);
        pump(id, cx);
    }
}

fn set_status(job: &ReviewJob, status: String, cx: &mut App) {
    job.editor
        .update(cx, |editor, cx| {
            editor.set_inline_review_status(job.comment_id, status, None, cx)
        })
        .log_err();
}

fn pump(id: EntityId, cx: &mut App) {
    loop {
        let Some(queue) = cx.global::<ReviewQueues>().0.get(&id) else {
            return;
        };
        if queue.running {
            return;
        }
        if queue.pending.is_empty() {
            cx.global_mut::<ReviewQueues>().0.remove(&id);
            return;
        }
        if queue.thread.read(cx).status() != ThreadStatus::Idle {
            return;
        }
        let thread = queue.thread.clone();
        let job = cx
            .global_mut::<ReviewQueues>()
            .0
            .get_mut(&id)
            .and_then(|queue| queue.pending.pop_front());
        let Some(job) = job else { return };
        if job.editor.upgrade().is_none() {
            continue;
        }
        if let Some(queue) = cx.global_mut::<ReviewQueues>().0.get_mut(&id) {
            queue.running = true;
        }
        set_status(&job, "Running".into(), cx);
        let content = review_content(&job.discussion_id, &job.submission_id, &job.text);
        let submission = thread.update(cx, |thread, cx| thread.send(vec![content], cx));
        cx.spawn(async move |cx| {
            let result = match submission.await {
                Ok(Some(SubmissionResponse::LegacyCompleted(response))) => correlated_reply(response, &job.discussion_id, &job.submission_id),
                Ok(None) => Ok(None),
                Ok(_) => Err(anyhow::anyhow!("No completed inline reply received")),
                Err(error) => Err(error),
            };
            cx.update(|cx| {
                let completed = matches!(&result, Ok(Some(_)));
                match result {
                    Ok(Some(text)) => { job.editor.update(cx, |editor, cx| editor.set_inline_review_reply(job.comment_id, "Pi".into(), text, cx)).log_err(); }
                    Ok(None) => set_status(&job, "Cancelled".into(), cx),
                    Err(error) => set_status(&job, format!("Failed: {error}. Check the transcript before retrying; this comment was not resent."), cx),
                }
                let not_sent = if let Some(queue) = cx.global_mut::<ReviewQueues>().0.get_mut(&id) {
                    queue.running = false;
                    if completed { VecDeque::new() } else { std::mem::take(&mut queue.pending) }
                } else { VecDeque::new() };
                for job in not_sent {
                    set_status(&job, "Not sent: an earlier review failed or was stopped. Check the transcript before posting again.".into(), cx);
                }
                pump(id, cx);
            });
            drop(thread);
        }).detach();
        return;
    }
}

fn review_content(discussion_id: &str, submission_id: &str, text: &str) -> acp_v2::ContentBlock {
    let mut meta = serde_json::Map::new();
    meta.insert(
        REVIEW_META_KEY.into(),
        json!({"version": 1, "threadId": discussion_id, "submissionId": submission_id}),
    );
    acp_v2::ContentBlock::Text(
        acp_v2::TextContent::new(format!("[Zed discussion {discussion_id}]\n\n{text}")).meta(meta),
    )
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
    discussion_id: &str,
    submission_id: &str,
) -> Result<Option<String>> {
    if response.stop_reason == acp_v1::StopReason::Cancelled {
        return Ok(None);
    }
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
        reply.version == 1
            && reply.thread_id == discussion_id
            && reply.submission_id == submission_id,
        "Inline-review reply belongs to a different submission or protocol version"
    );
    Ok(Some(reply.text))
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

    async fn queue_fixture(
        cx: &mut gpui::TestAppContext,
    ) -> (
        Entity<AcpThread>,
        Entity<Editor>,
        acp_thread::StubAgentConnection,
    ) {
        use acp_thread::AgentConnection as _;
        crate::test_support::init_test(cx);
        cx.update(init);
        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(util::path!("/project"), json!({"file.txt": "text"}))
            .await;
        let project = project::Project::test(fs, [util::path!("/project").as_ref()], cx).await;
        let connection = acp_thread::StubAgentConnection::new();
        let thread = cx
            .update(|cx| {
                Rc::new(connection.clone()).new_session(
                    project.clone(),
                    project.read(cx).default_path_list(cx),
                    cx,
                )
            })
            .await
            .unwrap();
        let window = cx.add_window(|window, cx| Editor::single_line(window, cx));
        let editor = window.update(cx, |_, _, cx| cx.entity()).unwrap();
        (thread, editor, connection)
    }

    fn job(editor: &Entity<Editor>, discussion: &str, submission: &str) -> ReviewJob {
        ReviewJob {
            editor: editor.downgrade(),
            comment_id: 0,
            discussion_id: discussion.into(),
            submission_id: submission.into(),
            text: submission.into(),
        }
    }

    #[gpui::test]
    async fn queued_reviews_preserve_fifo_and_cancellation_does_not_interrupt(
        cx: &mut gpui::TestAppContext,
    ) {
        let (thread, editor, connection) = queue_fixture(cx).await;
        let first = connection.defer_next_prompt_response();
        cx.update(|cx| {
            enqueue(thread.clone(), job(&editor, "discussion-a", "first"), cx);
            enqueue(thread.clone(), job(&editor, "discussion-b", "second"), cx);
            enqueue(
                thread.clone(),
                job(&editor, "discussion-a", "follow-up"),
                cx,
            );
            enqueue(
                thread.clone(),
                job(&editor, "discussion-c", "cancel-me"),
                cx,
            );
            cancel_queued(thread.entity_id(), "cancel-me", cx);
            cancel_queued(thread.entity_id(), "first", cx);
        });
        cx.run_until_parked();
        cx.update(|cx| {
            assert_eq!(thread.read(cx).entries().len(), 1);
            let queue = &cx.global::<ReviewQueues>().0[&thread.entity_id()];
            assert_eq!(
                queue
                    .pending
                    .iter()
                    .map(|job| job.submission_id.as_str())
                    .collect::<Vec<_>>(),
                vec!["second", "follow-up"]
            );
        });
        assert!(connection.take_cancel_calls().is_empty());
        let second = connection.defer_next_prompt_response();
        first.send(Ok(response("discussion-a", "first"))).unwrap();
        cx.run_until_parked();
        cx.update(|cx| assert_eq!(thread.read(cx).entries().len(), 2));
        let third = connection.defer_next_prompt_response();
        second.send(Ok(response("discussion-b", "second"))).unwrap();
        cx.run_until_parked();
        cx.update(|cx| assert_eq!(thread.read(cx).entries().len(), 3));
        third
            .send(Ok(response("discussion-a", "follow-up")))
            .unwrap();
        cx.run_until_parked();
        cx.update(|cx| assert!(cx.global::<ReviewQueues>().0.is_empty()));
        assert!(connection.take_cancel_calls().is_empty());
    }

    #[gpui::test]
    async fn failed_review_does_not_dispatch_remaining_queue_or_retry(
        cx: &mut gpui::TestAppContext,
    ) {
        let (thread, editor, connection) = queue_fixture(cx).await;
        let first = connection.defer_next_prompt_response();
        cx.update(|cx| {
            enqueue(thread.clone(), job(&editor, "discussion", "first"), cx);
            enqueue(thread.clone(), job(&editor, "discussion", "second"), cx);
        });
        cx.run_until_parked();
        first
            .send(Err(anyhow::anyhow!("connection lost; delivery uncertain")))
            .unwrap();
        cx.run_until_parked();
        cx.update(|cx| {
            assert_eq!(thread.read(cx).entries().len(), 1);
            assert!(cx.global::<ReviewQueues>().0.is_empty());
        });
        assert!(connection.take_cancel_calls().is_empty());
    }

    #[test]
    fn discussion_is_visible_to_pi_and_matches_wire_metadata() {
        let acp_v2::ContentBlock::Text(content) =
            review_content("discussion-a", "submission-b", "Why?")
        else {
            panic!("expected text")
        };
        assert_eq!(content.text, "[Zed discussion discussion-a]\n\nWhy?");
        assert_eq!(
            content.meta.unwrap()[REVIEW_META_KEY],
            json!({"version": 1, "threadId": "discussion-a", "submissionId": "submission-b"})
        );
    }

    #[test]
    fn replies_require_exact_correlation() {
        assert_eq!(
            correlated_reply(response("a", "b"), "a", "b").unwrap(),
            Some("reply".into())
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
        assert_eq!(correlated_reply(cancelled, "a", "b").unwrap(), None);
    }
}
