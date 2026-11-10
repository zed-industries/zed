mod storage;
use crate::AgentPanel;
use acp_thread::{AcpThread, AcpThreadEvent, SubmissionResponse, ThreadStatus};
use agent_client_protocol::schema::{v1 as acp_v1, v2 as acp_v2};
use anyhow::{Context as _, Result, bail, ensure};
use editor::{Editor, InlineReviewComment, SavedInlineReview, SplittableEditor};
use futures::{
    FutureExt as _,
    future::{LocalBoxFuture, Shared},
};
use gpui::{App, Context, Entity, EntityId, Global, Subscription, WeakEntity, Window, actions};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use std::{
    collections::{HashMap, VecDeque},
    rc::Rc,
};
use storage::ReviewStorage;
use util::ResultExt as _;
use workspace::{Toast, Workspace, notifications::NotificationId};

const REVIEW_META_KEY: &str = "zed-pi-live/review";

actions!(
    agent,
    [
        /// Comment on selected file or diff-side lines and queue it for the active pi-live session.
        CommentWithPiLive,
        /// Restore saved pi discussions for the current file without submitting prompts.
        RestorePiReviews,
    ]
);

#[derive(Clone)]
struct ReviewJob {
    editor: WeakEntity<Editor>,
    workspace: Option<WeakEntity<Workspace>>,
    comment_id: usize,
    discussion_id: String,
    submission_id: String,
    text: String,
    storage: ReviewStorage,
    saved: Shared<LocalBoxFuture<'static, Result<(), Arc<anyhow::Error>>>>,
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
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        if let Some(window) = window {
            cx.subscribe_in(
                &cx.entity(),
                window,
                |workspace, _, event: &workspace::Event, window, cx| {
                    if !matches!(event, workspace::Event::ActiveItemChanged) {
                        return;
                    }
                    let has_bridge = workspace
                        .panel::<AgentPanel>(cx)
                        .and_then(|panel| panel.read(cx).active_agent_thread(cx))
                        .is_some_and(|thread| {
                            thread.read(cx).connection().agent_id().as_ref() == "pi-live"
                        });
                    let reviewable_file = active_review_editor(workspace, cx)
                        .is_some_and(|editor| editor.read(cx).inline_review_file_path(cx).is_ok());
                    if has_bridge && reviewable_file {
                        if let Err(error) = load_reviews(workspace, false, window, cx) {
                            workspace.show_toast(
                                Toast::new(
                                    NotificationId::unique::<RestorePiReviews>(),
                                    error.to_string(),
                                ),
                                cx,
                            );
                        }
                    }
                },
            )
            .detach();
        }
        workspace.register_action(
            |workspace: &mut Workspace, _: &RestorePiReviews, window, cx| {
                if let Err(error) = load_reviews(workspace, false, window, cx) {
                    workspace.show_toast(
                        Toast::new(
                            NotificationId::unique::<RestorePiReviews>(),
                            error.to_string(),
                        ),
                        cx,
                    );
                }
            },
        );
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

fn active_review_editor(workspace: &Workspace, cx: &App) -> Option<Entity<Editor>> {
    workspace
        .active_item_as::<SplittableEditor>(cx)
        .map(|editor| editor.read(cx).focused_editor().clone())
        .or_else(|| workspace.active_item_as::<Editor>(cx))
}

fn show_comment(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> Result<()> {
    load_reviews(workspace, true, window, cx)
}

fn load_reviews(
    workspace: &mut Workspace,
    open_comment: bool,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> Result<()> {
    let editor = active_review_editor(workspace, cx)
        .context("Open a file or Git diff and select the lines to review")?;
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
    let path = editor.read(cx).inline_review_file_path(cx)?;
    let original_selections = editor
        .read(cx)
        .selections
        .disjoint_anchor_ranges()
        .collect::<Vec<_>>();
    let storage = ReviewStorage::for_thread(&thread, cx)?;
    let loaded = storage.load(&path, cx);
    let target_editor = editor.downgrade();
    let target_workspace = cx.entity().downgrade();
    let on_submit = Rc::new(
        move |comment: InlineReviewComment, _: &mut Window, cx: &mut App| {
            let text = review_prompt(&comment);
            let record = SavedInlineReview {
                version: if comment.diff.is_some() { 2 } else { 1 },
                discussion_id: comment.discussion_id.clone(),
                submission_id: comment.submission_id.clone(),
                path: comment.path,
                start_line: comment.lines.start,
                end_line: comment.lines.end,
                excerpt: comment.excerpt,
                diff: comment.diff,
                before: comment.before,
                after: comment.after,
                comment: comment.comment,
                created_at: chrono::Utc::now().timestamp_micros(),
                outcome: "pending".into(),
                reply: None,
                error: None,
            };
            let saved = storage
                .create(record, cx)
                .map(|result| result.map_err(Arc::new))
                .boxed_local()
                .shared();
            enqueue(
                thread.clone(),
                ReviewJob {
                    editor: target_editor.clone(),
                    comment_id: comment.id,
                    discussion_id: comment.discussion_id,
                    submission_id: comment.submission_id,
                    text,
                    storage: storage.clone(),
                    workspace: Some(target_workspace.clone()),
                    saved,
                },
                cx,
            );
        },
    );
    let editor = editor.downgrade();
    cx.spawn_in(window, async move |workspace, cx| {
        let loaded = loaded.await;
        workspace
            .update_in(cx, |workspace, window, cx| {
                let result = (|| -> Result<()> {
                    let reviews = loaded?;
                    let editor = editor.upgrade().context("Review editor was closed")?;
                    ensure!(
                        active_review_editor(workspace, cx).as_ref() == Some(&editor),
                        "Active file changed while loading reviews; run the action again"
                    );
                    if open_comment {
                        ensure!(
                            editor
                                .read(cx)
                                .selections
                                .disjoint_anchor_ranges()
                                .collect::<Vec<_>>()
                                == original_selections,
                            "Selection changed while loading reviews; run the action again"
                        );
                    }
                    let mut failures = Vec::new();
                    let focused = window.focused(cx);
                    let restored = editor.update(cx, |editor, cx| -> Result<()> {
                        ensure!(
                            editor.inline_review_file_path(cx)? == path,
                            "File path changed while loading reviews"
                        );
                        for review in reviews {
                            if let Err(error) =
                                editor.restore_inline_review(review, on_submit.clone(), window, cx)
                            {
                                failures.push(error.to_string());
                            }
                        }
                        if open_comment {
                            editor.show_inline_review(on_submit, window, cx)?;
                        }
                        Ok(())
                    });
                    if !open_comment || restored.is_err() {
                        if let Some(focused) = focused {
                            window.focus(&focused, cx);
                        }
                    }
                    restored?;
                    if let Some(first) = failures.first() {
                        bail!(
                            "{} saved review(s) could not be attached. {first}",
                            failures.len()
                        );
                    }
                    Ok(())
                })();
                if let Err(error) = result {
                    workspace.show_toast(
                        Toast::new(
                            NotificationId::unique::<RestorePiReviews>(),
                            error.to_string(),
                        ),
                        cx,
                    );
                }
            })
            .log_err();
    })
    .detach();
    Ok(())
}

fn review_prompt(comment: &InlineReviewComment) -> String {
    let source = comment.diff.as_ref().map(|diff| format!(
        "{}; base content SHA-256 {}. Line numbers refer to the selected side, not necessarily the current file. Historical code may no longer exist on disk.",
        diff.description(), diff.base_sha256,
    )).unwrap_or_else(|| "current file (editor snapshot)".into());
    format!(
        "Review comment on {}:{}-{}\nSource: {source}\n\nSelected code:\n{}\n\nComment:\n{}",
        comment.path.display(),
        comment.lines.start + 1,
        comment.lines.end,
        comment.excerpt,
        comment.comment
    )
}

fn enqueue(thread: Entity<AcpThread>, job: ReviewJob, cx: &mut App) {
    let id = thread.entity_id();
    let saving_job = job.clone();
    cx.spawn(async move |cx| {
        if let Err(error) = saving_job.saved.clone().await {
            cx.update(|cx| {
                report_storage_failure(
                    &saving_job,
                    format!(
                        "Could not confirm the comment was saved; it was not dispatched: {error}"
                    ),
                    cx,
                )
            });
        }
        cx.update(|cx| pump(id, cx));
    })
    .detach();
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
        persist_outcome(job, "cancelled", None, None, cx);
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
        let Some(saved) = queue.pending.front().and_then(|job| job.saved.peek()) else {
            return;
        };
        let save_error = saved.as_ref().err().cloned();
        if queue.thread.read(cx).status() != ThreadStatus::Idle && save_error.is_none() {
            return;
        }
        let thread = queue.thread.clone();
        let job = cx
            .global_mut::<ReviewQueues>()
            .0
            .get_mut(&id)
            .and_then(|queue| queue.pending.pop_front());
        let Some(job) = job else { return };
        if let Some(error) = save_error {
            settle_job(
                id,
                job,
                Err(anyhow::anyhow!(
                    "Comment was not dispatched because storage failed: {error}"
                )),
                cx,
            );
            return;
        }
        if job.editor.upgrade().is_none() {
            persist_outcome(job, "not_sent", None, None, cx);
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
                Ok(Some(SubmissionResponse::LegacyCompleted(response))) => {
                    correlated_reply(response, &job.discussion_id, &job.submission_id)
                }
                Ok(None) => Ok(None),
                Ok(_) => Err(anyhow::anyhow!("No completed inline reply received")),
                Err(error) => Err(error),
            };
            cx.update(|cx| settle_job(id, job, result, cx));
            drop(thread);
        })
        .detach();
        return;
    }
}

fn report_storage_failure(job: &ReviewJob, message: String, cx: &mut App) {
    log::error!("{message}");
    if job.editor.upgrade().is_some() {
        set_status(job, message.clone(), cx);
    }
    if let Some(workspace) = &job.workspace {
        workspace
            .update(cx, |workspace, cx| {
                workspace.show_toast(
                    Toast::new(NotificationId::unique::<RestorePiReviews>(), message),
                    cx,
                );
            })
            .log_err();
    }
}

fn persist_outcome(
    job: ReviewJob,
    outcome: &'static str,
    reply: Option<String>,
    error: Option<String>,
    cx: &mut App,
) {
    cx.spawn(async move |cx| {
        if let Err(error) = job.saved.clone().await {
            log::error!("Outcome was not written because the initial save failed: {error}");
            return;
        }
        let saved = cx.update(|cx| job.storage.finish(&job.submission_id, outcome, reply, error, cx));
        if let Err(error) = saved.await {
            cx.update(|cx| report_storage_failure(&job, format!("Could not confirm the review outcome was saved: {error}. Check pi's transcript; nothing was resent."), cx));
        }
    }).detach();
}

fn settle_job(id: EntityId, job: ReviewJob, result: Result<Option<String>>, cx: &mut App) {
    let completed = matches!(&result, Ok(Some(_)));
    match result {
        Ok(Some(text)) => {
            if let Some(editor) = job.editor.upgrade() {
                editor.update(cx, |editor, cx| {
                    editor.set_inline_review_reply(job.comment_id, "Pi".into(), text.clone(), cx)
                });
            }
            persist_outcome(job, "answered", Some(text), None, cx);
        }
        Ok(None) => {
            set_status(&job, "Cancelled".into(), cx);
            persist_outcome(job, "cancelled", None, None, cx);
        }
        Err(error) => {
            set_status(
                &job,
                format!(
                    "Failed: {error}. Check the transcript before retrying; this comment was not resent."
                ),
                cx,
            );
            persist_outcome(job, "failed", None, Some(error.to_string()), cx);
        }
    }
    let not_sent = if let Some(queue) = cx.global_mut::<ReviewQueues>().0.get_mut(&id) {
        queue.running = false;
        if completed {
            VecDeque::new()
        } else {
            std::mem::take(&mut queue.pending)
        }
    } else {
        VecDeque::new()
    };
    for job in not_sent {
        set_status(&job, "Not sent: an earlier review failed or was stopped. Check the transcript before posting again.".into(), cx);
        persist_outcome(job, "not_sent", None, None, cx);
    }
    pump(id, cx);
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
            workspace: None,
            storage: ReviewStorage {
                request: Rc::new(|_, _| gpui::Task::ready(Ok(json!({"version": 1})))),
            },
            saved: futures::future::ready(Ok(())).boxed_local().shared(),
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
        });
        cx.run_until_parked();
        cx.update(|cx| {
            cancel_queued(thread.entity_id(), "first", cx);
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

    #[gpui::test]
    async fn saving_gates_dispatch_and_cancellation_waits_for_initial_save(
        cx: &mut gpui::TestAppContext,
    ) {
        let (thread, editor, connection) = queue_fixture(cx).await;
        let (saved, saving) = futures::channel::oneshot::channel::<()>();
        let outcomes = Rc::new(std::cell::RefCell::new(Vec::new()));
        let mut pending = job(&editor, "discussion", "saving");
        pending.saved = async move {
            saving
                .await
                .map_err(|error| Arc::new(anyhow::anyhow!(error)))
        }
        .boxed_local()
        .shared();
        pending.storage = ReviewStorage {
            request: Rc::new({
                let outcomes = outcomes.clone();
                move |request, _| {
                    outcomes.borrow_mut().push(request);
                    gpui::Task::ready(Ok(json!({"version": 1})))
                }
            }),
        };
        cx.update(|cx| enqueue(thread.clone(), pending, cx));
        cx.run_until_parked();
        cx.update(|cx| {
            assert!(thread.read(cx).entries().is_empty());
            cancel_queued(thread.entity_id(), "saving", cx);
        });
        cx.run_until_parked();
        assert!(outcomes.borrow().is_empty());
        saved.send(()).unwrap();
        cx.run_until_parked();
        assert_eq!(
            outcomes.borrow().as_slice(),
            &[json!({"operation": "finish", "submissionId": "saving", "outcome": "cancelled"})]
        );
        cx.update(|cx| assert!(thread.read(cx).entries().is_empty()));
        assert!(connection.take_cancel_calls().is_empty());
    }

    #[gpui::test]
    async fn storage_failure_never_dispatches_or_retries(cx: &mut gpui::TestAppContext) {
        let (thread, editor, connection) = queue_fixture(cx).await;
        let mut failed = job(&editor, "discussion", "failed-save");
        failed.saved = futures::future::ready(Err(Arc::new(anyhow::anyhow!("disk full"))))
            .boxed_local()
            .shared();
        cx.update(|cx| {
            enqueue(thread.clone(), failed, cx);
            enqueue(thread.clone(), job(&editor, "discussion", "not-sent"), cx);
        });
        cx.run_until_parked();
        cx.update(|cx| {
            assert!(thread.read(cx).entries().is_empty());
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
    fn diff_review_prompt_preserves_side_comparison_and_historical_line_numbers() {
        let comment = InlineReviewComment {
            id: 0,
            discussion_id: "discussion".into(),
            submission_id: "submission".into(),
            path: "/project/deleted.txt".into(),
            lines: 7..9,
            excerpt: "old first\nold second".into(),
            before: String::new(),
            after: String::new(),
            comment: "Why was this removed?".into(),
            diff: Some(
                serde_json::from_value(
                    json!({"comparison": "staged", "side": "old", "baseSha256": "a".repeat(64)}),
                )
                .unwrap(),
            ),
        };
        let prompt = review_prompt(&comment);
        assert!(prompt.contains("/project/deleted.txt:8-9"));
        assert!(prompt.contains("Source: staged diff (HEAD → index), old side"));
        assert!(prompt.contains("Historical code may no longer exist on disk"));
        assert!(prompt.contains("old first\nold second"));
        assert!(prompt.contains(&"a".repeat(64)));
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
