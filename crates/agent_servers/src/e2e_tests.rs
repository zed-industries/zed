use crate::{AgentServer, AgentServerDelegate};
use acp_thread::{
    AcpThread, AcpThreadEvent, AgentThreadEntry, ForegroundActivity, SubmissionResponse,
    ToolCallStatus,
};
use agent_client_protocol::schema::v2 as acp_v2;
use anyhow::{Context as _, Result, anyhow};
use client::RefreshLlmTokenListener;
use futures::{FutureExt, StreamExt, channel::mpsc, channel::oneshot, select};
use gpui::AppContext;
use gpui::{Entity, Task, TestAppContext};
use indoc::indoc;
use project::{FakeFs, Project};
#[cfg(test)]
use settings::Settings;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use util::path;
use util::path_list::PathList;

pub async fn test_basic<T, F>(server: F, cx: &mut TestAppContext)
where
    T: AgentServer + 'static,
    F: AsyncFn(&Arc<dyn fs::Fs>, &mut TestAppContext) -> T,
{
    let fs = init_test(cx).await as Arc<dyn fs::Fs>;
    let project = Project::test(fs.clone(), [], cx).await;
    let thread = new_test_thread(server(&fs, cx).await, project.clone(), "/private/tmp", cx).await;

    send_and_wait_for_turn(&thread, vec!["Hello from Zed!".into()], cx)
        .await
        .unwrap();

    thread.read_with(cx, |thread, _| {
        assert!(
            thread.entries().len() >= 2,
            "Expected at least 2 entries. Got: {:?}",
            thread.entries()
        );
        assert!(matches!(
            thread.entries()[0],
            AgentThreadEntry::UserMessage(_)
        ));
        assert!(matches!(
            thread.entries()[1],
            AgentThreadEntry::AssistantMessage(_)
        ));
    });
}

pub async fn test_path_mentions<T, F>(server: F, cx: &mut TestAppContext)
where
    T: AgentServer + 'static,
    F: AsyncFn(&Arc<dyn fs::Fs>, &mut TestAppContext) -> T,
{
    let fs = init_test(cx).await as _;

    let tempdir = tempfile::tempdir().unwrap();
    std::fs::write(
        tempdir.path().join("foo.rs"),
        indoc! {"
            fn main() {
                println!(\"Hello, world!\");
            }
        "},
    )
    .expect("failed to write file");
    let project = Project::example([tempdir.path()], &mut cx.to_async()).await;
    let thread = new_test_thread(server(&fs, cx).await, project.clone(), tempdir.path(), cx).await;
    send_and_wait_for_turn(
        &thread,
        vec![
            "Read the file ".into(),
            acp_v2::ContentBlock::ResourceLink(acp_v2::ResourceLink::new("foo.rs", "foo.rs")),
            " and tell me what the content of the println! is".into(),
        ],
        cx,
    )
    .await
    .unwrap();

    thread.read_with(cx, |thread, cx| {
        assert!(matches!(
            thread.entries()[0],
            AgentThreadEntry::UserMessage(_)
        ));
        let assistant_message = &thread
            .entries()
            .iter()
            .rev()
            .find_map(|entry| match entry {
                AgentThreadEntry::AssistantMessage(msg) => Some(msg),
                _ => None,
            })
            .unwrap();

        assert!(
            assistant_message.to_markdown(cx).contains("Hello, world!"),
            "unexpected assistant message: {:?}",
            assistant_message.to_markdown(cx)
        );
    });

    drop(tempdir);
}

pub async fn test_tool_call<T, F>(server: F, cx: &mut TestAppContext)
where
    T: AgentServer + 'static,
    F: AsyncFn(&Arc<dyn fs::Fs>, &mut TestAppContext) -> T,
{
    let fs = init_test(cx).await as _;

    let tempdir = tempfile::tempdir().unwrap();
    let foo_path = tempdir.path().join("foo");
    std::fs::write(&foo_path, "Lorem ipsum dolor").expect("failed to write file");

    let project = Project::example([tempdir.path()], &mut cx.to_async()).await;
    let thread = new_test_thread(server(&fs, cx).await, project.clone(), "/private/tmp", cx).await;

    send_and_wait_for_turn(
        &thread,
        vec![format!("Read {} and tell me what you see.", foo_path.display()).into()],
        cx,
    )
    .await
    .unwrap();
    thread.read_with(cx, |thread, _cx| {
        assert!(thread.entries().iter().any(|entry| {
            matches!(
                entry,
                AgentThreadEntry::ToolCall(call) if matches!(call.status(), ToolCallStatus::Pending
                        | ToolCallStatus::InProgress
                        | ToolCallStatus::Completed)
            )
        }));
        assert!(
            thread
                .entries()
                .iter()
                .any(|entry| { matches!(entry, AgentThreadEntry::AssistantMessage(_)) })
        );
    });

    drop(tempdir);
}

pub async fn test_tool_call_with_permission<T, F>(
    server: F,
    allow_option_id: acp_v2::PermissionOptionId,
    cx: &mut TestAppContext,
) where
    T: AgentServer + 'static,
    F: AsyncFn(&Arc<dyn fs::Fs>, &mut TestAppContext) -> T,
{
    let fs = init_test(cx).await as Arc<dyn fs::Fs>;
    let project = Project::test(fs.clone(), [path!("/private/tmp").as_ref()], cx).await;
    let thread = new_test_thread(server(&fs, cx).await, project.clone(), "/private/tmp", cx).await;
    let full_turn = send_and_wait_for_turn(
        &thread,
        vec![r#"Run exactly `touch hello.txt && echo "Hello, world!" | tee hello.txt` in the terminal."#.into()],
        cx,
    );

    run_until_first_tool_call(
        &thread,
        |entry| {
            matches!(
                entry,
                AgentThreadEntry::ToolCall(call) if call.status() == ToolCallStatus::WaitingForConfirmation
            )
        },
        cx,
    )
    .await;

    let tool_call_id = thread.read_with(cx, |thread, cx| {
        let AgentThreadEntry::ToolCall(tool_call) = &thread
            .entries()
            .iter()
            .find(|entry| matches!(entry, AgentThreadEntry::ToolCall(_)))
            .unwrap()
        else {
            panic!();
        };
        assert_eq!(tool_call.status(), ToolCallStatus::WaitingForConfirmation);

        let label = tool_call.label.read(cx).source();
        assert!(label.contains("touch"), "Got: {}", label);

        tool_call.id.clone()
    });

    thread.update(cx, |thread, cx| {
        thread.authorize_tool_call(
            tool_call_id,
            acp_thread::SelectedPermissionOutcome::new(
                allow_option_id,
                acp_v2::PermissionOptionKind::AllowOnce,
            ),
            cx,
        );

        assert!(thread.entries().iter().any(|entry| matches!(
            entry,
            AgentThreadEntry::ToolCall(call) if matches!(call.status(), ToolCallStatus::Pending
                    | ToolCallStatus::InProgress
                    | ToolCallStatus::Completed)
        )));
    });

    full_turn.await.unwrap();

    thread.read_with(cx, |thread, cx| {
        let AgentThreadEntry::ToolCall(tool_call) = thread
            .entries()
            .iter()
            .find(|entry| matches!(entry, AgentThreadEntry::ToolCall(_)))
            .unwrap()
        else {
            panic!();
        };
        assert!(matches!(
            tool_call.status(),
            ToolCallStatus::Pending | ToolCallStatus::InProgress | ToolCallStatus::Completed
        ));

        assert!(
            tool_call
                .content()
                .iter()
                .any(|content| content.to_markdown(cx).contains("Hello")),
            "Expected content to contain 'Hello'"
        );
    });
}

pub async fn test_cancel<T, F>(server: F, cx: &mut TestAppContext)
where
    T: AgentServer + 'static,
    F: AsyncFn(&Arc<dyn fs::Fs>, &mut TestAppContext) -> T,
{
    let fs = init_test(cx).await as Arc<dyn fs::Fs>;

    let project = Project::test(fs.clone(), [path!("/private/tmp").as_ref()], cx).await;
    let thread = new_test_thread(server(&fs, cx).await, project.clone(), "/private/tmp", cx).await;
    let full_turn = send_and_wait_for_turn(
        &thread,
        vec![r#"Run exactly `touch hello.txt && echo "Hello, world!" | tee hello.txt` in the terminal."#.into()],
        cx,
    );

    let first_tool_call_ix = run_until_first_tool_call(
        &thread,
        |entry| {
            matches!(
                entry,
                AgentThreadEntry::ToolCall(call) if call.status() == ToolCallStatus::WaitingForConfirmation
            )
        },
        cx,
    )
    .await;

    thread.read_with(cx, |thread, cx| {
        let AgentThreadEntry::ToolCall(tool_call) = &thread.entries()[first_tool_call_ix] else {
            panic!("{:?}", thread.entries()[1]);
        };
        assert_eq!(tool_call.status(), ToolCallStatus::WaitingForConfirmation);

        let label = tool_call.label.read(cx).source();
        assert!(label.contains("touch"), "Got: {}", label);

        tool_call.id.clone()
    });

    thread
        .update(cx, |thread, cx| thread.cancel(cx))
        .await
        .expect("cancellation succeeds");
    full_turn.await.unwrap();
    thread.read_with(cx, |thread, _cx| {
        let AgentThreadEntry::ToolCall(tool_call) = &thread.entries()[first_tool_call_ix] else {
            panic!();
        };
        assert_eq!(tool_call.status(), ToolCallStatus::Canceled);
    });

    send_and_wait_for_turn(
        &thread,
        vec!["Stop running and say goodbye to me.".into()],
        cx,
    )
    .await
    .unwrap();
    thread.read_with(cx, |thread, _| {
        assert!(matches!(
            &thread.entries().last().unwrap(),
            AgentThreadEntry::AssistantMessage(..),
        ))
    });
}

pub async fn test_thread_drop<T, F>(server: F, cx: &mut TestAppContext)
where
    T: AgentServer + 'static,
    F: AsyncFn(&Arc<dyn fs::Fs>, &mut TestAppContext) -> T,
{
    let fs = init_test(cx).await as Arc<dyn fs::Fs>;
    let project = Project::test(fs.clone(), [], cx).await;
    let thread = new_test_thread(server(&fs, cx).await, project.clone(), "/private/tmp", cx).await;

    send_and_wait_for_turn(&thread, vec!["Hello from test!".into()], cx)
        .await
        .unwrap();

    thread.read_with(cx, |thread, _| {
        assert!(thread.entries().len() >= 2, "Expected at least 2 entries");
    });

    let weak_thread = thread.downgrade();
    drop(thread);

    cx.executor().run_until_parked();
    assert!(!weak_thread.is_upgradable());
}

#[macro_export]
macro_rules! common_e2e_tests {
    ($server:expr, allow_option_id = $allow_option_id:expr) => {
        mod common_e2e {
            use super::*;

            #[::gpui::test]
            #[cfg_attr(not(feature = "e2e"), ignore)]
            async fn basic(cx: &mut ::gpui::TestAppContext) {
                $crate::e2e_tests::test_basic($server, cx).await;
            }

            #[::gpui::test]
            #[cfg_attr(not(feature = "e2e"), ignore)]
            async fn path_mentions(cx: &mut ::gpui::TestAppContext) {
                $crate::e2e_tests::test_path_mentions($server, cx).await;
            }

            #[::gpui::test]
            #[cfg_attr(not(feature = "e2e"), ignore)]
            async fn tool_call(cx: &mut ::gpui::TestAppContext) {
                $crate::e2e_tests::test_tool_call($server, cx).await;
            }

            #[::gpui::test]
            #[cfg_attr(not(feature = "e2e"), ignore)]
            async fn tool_call_with_permission(cx: &mut ::gpui::TestAppContext) {
                $crate::e2e_tests::test_tool_call_with_permission(
                    $server,
                    ::agent_client_protocol::schema::v2::PermissionOptionId::new($allow_option_id),
                    cx,
                )
                .await;
            }

            #[::gpui::test]
            #[cfg_attr(not(feature = "e2e"), ignore)]
            async fn cancel(cx: &mut ::gpui::TestAppContext) {
                $crate::e2e_tests::test_cancel($server, cx).await;
            }

            #[::gpui::test]
            #[cfg_attr(not(feature = "e2e"), ignore)]
            async fn thread_drop(cx: &mut ::gpui::TestAppContext) {
                $crate::e2e_tests::test_thread_drop($server, cx).await;
            }
        }
    };
}
pub use common_e2e_tests;

// Helpers

pub fn send_and_wait_for_turn(
    thread: &Entity<AcpThread>,
    content: Vec<acp_v2::ContentBlock>,
    cx: &mut TestAppContext,
) -> Task<Result<()>> {
    cx.update(|cx| {
        // These tests send sequential turns; otherwise a predecessor's stop could win this wait.
        if thread.read(cx).foreground_activity() != ForegroundActivity::Idle
            || thread.read(cx).has_unsettled_submissions()
        {
            return Task::ready(Err(anyhow!(
                "send_and_wait_for_turn requires an idle thread with no unsettled submissions"
            )));
        }
        let generation = thread.read(cx).activity_generation();
        let (sender, receiver) = oneshot::channel();
        let mut sender = Some(sender);
        // Native acceptance can precede completion, and completion can precede polling the receipt.
        let subscription = cx.subscribe(thread, move |_, event, _| {
            if let AcpThreadEvent::Stopped {
                activity_generation,
                stop_reason,
                ..
            } = event
                && *activity_generation > generation
                && let Some(sender) = sender.take()
            {
                let result = match stop_reason {
                    Some(acp_v2::StopReason::Error(reason)) => Err(match &reason.error {
                        Some(error) => anyhow::Error::new((**error).clone()),
                        None => anyhow!("Agent stopped with an error"),
                    }),
                    _ => Ok(()),
                };
                if sender.send(result).is_err() {
                    log::debug!("Turn completion waiter was dropped");
                }
            }
        });
        let submission = thread.update(cx, |thread, cx| thread.send(content, cx));
        cx.spawn(async move |_| {
            let result = match submission.await? {
                Some(SubmissionResponse::Accepted(_)) => receiver
                    .await
                    .context("Thread dropped before the turn stopped")?,
                Some(SubmissionResponse::LegacyCompleted(_)) | None => Ok(()),
            };
            drop(subscription);
            result
        })
    })
}

pub async fn init_test(cx: &mut TestAppContext) -> Arc<FakeFs> {
    env_logger::try_init().ok();

    cx.update(|cx| {
        let settings_store = settings::SettingsStore::test(cx);
        cx.set_global(settings_store);
        gpui_tokio::init(cx);
        let http_client = reqwest_client::ReqwestClient::user_agent("agent tests").unwrap();
        cx.set_http_client(Arc::new(http_client));
        let client = client::Client::production(cx);
        let user_store = cx.new(|cx| client::UserStore::new(client.clone(), cx));
        language_model::init(cx);
        RefreshLlmTokenListener::register(client.clone(), user_store, cx);

        #[cfg(test)]
        project::agent_server_store::AllAgentServersSettings::override_global(
            project::agent_server_store::AllAgentServersSettings(collections::HashMap::default()),
            cx,
        );
    });

    cx.executor().allow_parking();

    FakeFs::new(cx.executor())
}

pub async fn new_test_thread(
    server: impl AgentServer + 'static,
    project: Entity<Project>,
    current_dir: impl AsRef<Path>,
    cx: &mut TestAppContext,
) -> Entity<AcpThread> {
    let store = project.read_with(cx, |project, _| project.agent_server_store().clone());
    let delegate = AgentServerDelegate::new(store, None, None);

    let connection = cx
        .update(|cx| server.connect(delegate, project.clone(), cx))
        .await
        .unwrap();

    cx.update(|cx| {
        connection.new_session(project.clone(), PathList::new(&[current_dir.as_ref()]), cx)
    })
    .await
    .unwrap()
}

pub async fn run_until_first_tool_call(
    thread: &Entity<AcpThread>,
    wait_until: impl Fn(&AgentThreadEntry) -> bool + 'static,
    cx: &mut TestAppContext,
) -> usize {
    let (mut tx, mut rx) = mpsc::channel::<usize>(1);

    let subscription = cx.update(|cx| {
        cx.subscribe(thread, move |thread, _, cx| {
            for (ix, entry) in thread.read(cx).entries().iter().enumerate() {
                if wait_until(entry) {
                    return tx.try_send(ix).unwrap();
                }
            }
        })
    });

    select! {
        _ = futures::FutureExt::fuse(cx.background_executor.timer(Duration::from_secs(20))) => {
            panic!("Timeout waiting for tool call")
        }
        ix = rx.next().fuse() => {
            drop(subscription);
            ix.unwrap()
        }
    }
}

pub fn get_zed_path() -> PathBuf {
    let mut zed_path = std::env::current_exe().unwrap();

    while zed_path
        .file_name()
        .is_none_or(|name| name.to_string_lossy() != "debug")
    {
        if !zed_path.pop() {
            panic!("Could not find target directory");
        }
    }

    zed_path.push("zed");

    if !zed_path.exists() {
        panic!("\n🚨 Run `cargo build` at least once before running e2e tests\n\n");
    }

    zed_path
}

#[cfg(test)]
mod tests {
    use super::*;
    use acp_thread::{AgentConnection, StubAgentConnection};
    use agent_client_protocol::schema::v1 as acp_v1;
    use std::rc::Rc;

    async fn stub_thread(
        receipt_submissions: bool,
        cx: &mut TestAppContext,
    ) -> (Rc<StubAgentConnection>, Entity<AcpThread>) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
        });
        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let connection =
            Rc::new(StubAgentConnection::new().with_receipt_submissions(receipt_submissions));
        let thread = cx
            .update(|cx| {
                connection.clone().new_session(
                    project,
                    PathList::new(&[Path::new("/private/tmp")]),
                    cx,
                )
            })
            .await
            .expect("stub session");
        (connection, thread)
    }

    #[gpui::test]
    async fn test_e2e_helper_preserves_v1_completion(cx: &mut TestAppContext) {
        let (connection, thread) = stub_thread(false, cx).await;
        let response = connection.defer_next_prompt_response();
        let mut completion = send_and_wait_for_turn(&thread, vec!["hello".into()], cx);
        cx.run_until_parked();
        assert!((&mut completion).now_or_never().is_none());
        response
            .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
            .expect("prompt response");
        completion.await.expect("v1 completion");
    }

    #[gpui::test]
    async fn test_e2e_helper_preserves_stop_before_receipt(cx: &mut TestAppContext) {
        let (connection, thread) = stub_thread(true, cx).await;
        let receipt = connection.defer_next_receipt_response();
        let completion = send_and_wait_for_turn(&thread, vec!["fail".into()], cx);
        send_and_wait_for_turn(&thread, vec!["overlapping".into()], cx)
            .await
            .expect_err("an unsettled receipt must reject overlapping prompts");
        thread.update(cx, |thread, cx| {
            cx.emit(AcpThreadEvent::Stopped {
                activity_generation: thread.activity_generation(),
                activity_duration: None,
                stop_reason: Some(acp_v2::StopReason::EndTurn),
            });
            thread
                .update_session_state(
                    acp_v2::StateUpdate::Running(acp_v2::RunningStateUpdate::new()),
                    cx,
                )
                .expect("running");
            thread
                .update_session_state(
                    acp_v2::StateUpdate::Idle(
                        acp_v2::IdleStateUpdate::new()
                            .stop_reason(acp_v2::StopReason::Error(acp_v2::ErrorStopReason::new())),
                    ),
                    cx,
                )
                .expect("stopped");
        });
        cx.run_until_parked();
        receipt
            .send(Ok(acp_v2::PromptResponse::new("accepted")))
            .expect("receipt");
        assert_eq!(
            completion
                .await
                .expect_err("error without details")
                .to_string(),
            "Agent stopped with an error"
        );
    }
}
