use crate::{AgentServer, AgentServerDelegate};
use acp_thread::{AcpThread, AgentThreadEntry, ToolCall, ToolCallStatus};
use agent_client_protocol as acp;
use futures::{FutureExt, StreamExt, channel::mpsc, select};
use gpui::{AppContext, Entity, TestAppContext};
use indoc::indoc;
#[cfg(test)]
use project::agent_server_store::BuiltinAgentServerSettings;
use project::{FakeFs, Project, agent_server_store::AllAgentServersSettings};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use util::path;

pub async fn test_basic<T, F>(server: F, cx: &mut TestAppContext)
where
    T: AgentServer + 'static,
    F: AsyncFn(&Arc<dyn fs::Fs>, &Entity<Project>, &mut TestAppContext) -> T,
{
    let fs = init_test(cx).await as Arc<dyn fs::Fs>;
    let project = Project::test(fs.clone(), [], cx).await;
    let thread = new_test_thread(
        server(&fs, &project, cx).await,
        project.clone(),
        "/private/tmp",
        cx,
    )
    .await;

    thread
        .update(cx, |thread, cx| thread.send_raw("Hello from Zed!", cx))
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
    F: AsyncFn(&Arc<dyn fs::Fs>, &Entity<Project>, &mut TestAppContext) -> T,
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
    let thread = new_test_thread(
        server(&fs, &project, cx).await,
        project.clone(),
        tempdir.path(),
        cx,
    )
    .await;
    thread
        .update(cx, |thread, cx| {
            thread.send(
                vec![
                    acp::ContentBlock::Text(acp::TextContent {
                        text: "Read the file ".into(),
                        annotations: None,
                        meta: None,
                    }),
                    acp::ContentBlock::ResourceLink(acp::ResourceLink {
                        uri: "foo.rs".into(),
                        name: "foo.rs".into(),
                        annotations: None,
                        description: None,
                        mime_type: None,
                        size: None,
                        title: None,
                        meta: None,
                    }),
                    acp::ContentBlock::Text(acp::TextContent {
                        text: " and tell me what the content of the println! is".into(),
                        annotations: None,
                        meta: None,
                    }),
                ],
                cx,
            )
        })
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
    F: AsyncFn(&Arc<dyn fs::Fs>, &Entity<Project>, &mut TestAppContext) -> T,
{
    let fs = init_test(cx).await as _;

    let tempdir = tempfile::tempdir().unwrap();
    let foo_path = tempdir.path().join("foo");
    std::fs::write(&foo_path, "Lorem ipsum dolor").expect("failed to write file");

    let project = Project::example([tempdir.path()], &mut cx.to_async()).await;
    let thread = new_test_thread(
        server(&fs, &project, cx).await,
        project.clone(),
        "/private/tmp",
        cx,
    )
    .await;

    thread
        .update(cx, |thread, cx| {
            thread.send_raw(
                &format!("Read {} and tell me what you see.", foo_path.display()),
                cx,
            )
        })
        .await
        .unwrap();
    thread.read_with(cx, |thread, _cx| {
        assert!(thread.entries().iter().any(|entry| {
            matches!(
                entry,
                AgentThreadEntry::ToolCall(ToolCall {
                    status: ToolCallStatus::Pending
                        | ToolCallStatus::InProgress
                        | ToolCallStatus::Completed,
                    ..
                })
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
    allow_option_id: acp::PermissionOptionId,
    cx: &mut TestAppContext,
) where
    T: AgentServer + 'static,
    F: AsyncFn(&Arc<dyn fs::Fs>, &Entity<Project>, &mut TestAppContext) -> T,
{
    let fs = init_test(cx).await as Arc<dyn fs::Fs>;
    let project = Project::test(fs.clone(), [path!("/private/tmp").as_ref()], cx).await;
    let thread = new_test_thread(
        server(&fs, &project, cx).await,
        project.clone(),
        "/private/tmp",
        cx,
    )
    .await;
    let full_turn = thread.update(cx, |thread, cx| {
        thread.send_raw(
            r#"Run exactly `touch hello.txt && echo "Hello, world!" | tee hello.txt` in the terminal."#,
            cx,
        )
    });

    run_until_first_tool_call(
        &thread,
        |entry| {
            matches!(
                entry,
                AgentThreadEntry::ToolCall(ToolCall {
                    status: ToolCallStatus::WaitingForConfirmation { .. },
                    ..
                })
            )
        },
        cx,
    )
    .await;

    let tool_call_id = thread.read_with(cx, |thread, cx| {
        let AgentThreadEntry::ToolCall(ToolCall {
            id,
            label,
            status: ToolCallStatus::WaitingForConfirmation { .. },
            ..
        }) = &thread
            .entries()
            .iter()
            .find(|entry| matches!(entry, AgentThreadEntry::ToolCall(_)))
            .unwrap()
        else {
            panic!();
        };

        let label = label.read(cx).source();
        assert!(label.contains("touch"), "Got: {}", label);

        id.clone()
    });

    thread.update(cx, |thread, cx| {
        thread.authorize_tool_call(
            tool_call_id,
            allow_option_id,
            acp::PermissionOptionKind::AllowOnce,
            cx,
        );

        assert!(thread.entries().iter().any(|entry| matches!(
            entry,
            AgentThreadEntry::ToolCall(ToolCall {
                status: ToolCallStatus::Pending
                    | ToolCallStatus::InProgress
                    | ToolCallStatus::Completed,
                ..
            })
        )));
    });

    full_turn.await.unwrap();

    thread.read_with(cx, |thread, cx| {
        let AgentThreadEntry::ToolCall(ToolCall {
            content,
            status: ToolCallStatus::Pending
                | ToolCallStatus::InProgress
                | ToolCallStatus::Completed,
            ..
        }) = thread
            .entries()
            .iter()
            .find(|entry| matches!(entry, AgentThreadEntry::ToolCall(_)))
            .unwrap()
        else {
            panic!();
        };

        assert!(
            content.iter().any(|c| c.to_markdown(cx).contains("Hello")),
            "Expected content to contain 'Hello'"
        );
    });
}

pub async fn test_cancel<T, F>(server: F, cx: &mut TestAppContext)
where
    T: AgentServer + 'static,
    F: AsyncFn(&Arc<dyn fs::Fs>, &Entity<Project>, &mut TestAppContext) -> T,
{
    let fs = init_test(cx).await as Arc<dyn fs::Fs>;

    let project = Project::test(fs.clone(), [path!("/private/tmp").as_ref()], cx).await;
    let thread = new_test_thread(
        server(&fs, &project, cx).await,
        project.clone(),
        "/private/tmp",
        cx,
    )
    .await;
    let _ = thread.update(cx, |thread, cx| {
        thread.send_raw(
            r#"Run exactly `touch hello.txt && echo "Hello, world!" | tee hello.txt` in the terminal."#,
            cx,
        )
    });

    let first_tool_call_ix = run_until_first_tool_call(
        &thread,
        |entry| {
            matches!(
                entry,
                AgentThreadEntry::ToolCall(ToolCall {
                    status: ToolCallStatus::WaitingForConfirmation { .. },
                    ..
                })
            )
        },
        cx,
    )
    .await;

    thread.read_with(cx, |thread, cx| {
        let AgentThreadEntry::ToolCall(ToolCall {
            id,
            label,
            status: ToolCallStatus::WaitingForConfirmation { .. },
            ..
        }) = &thread.entries()[first_tool_call_ix]
        else {
            panic!("{:?}", thread.entries()[1]);
        };

        let label = label.read(cx).source();
        assert!(label.contains("touch"), "Got: {}", label);

        id.clone()
    });

    thread.update(cx, |thread, cx| thread.cancel(cx)).await;
    thread.read_with(cx, |thread, _cx| {
        let AgentThreadEntry::ToolCall(ToolCall {
            status: ToolCallStatus::Canceled,
            ..
        }) = &thread.entries()[first_tool_call_ix]
        else {
            panic!();
        };
    });

    thread
        .update(cx, |thread, cx| {
            thread.send_raw(r#"Stop running and say goodbye to me."#, cx)
        })
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
    F: AsyncFn(&Arc<dyn fs::Fs>, &Entity<Project>, &mut TestAppContext) -> T,
{
    let fs = init_test(cx).await as Arc<dyn fs::Fs>;
    let project = Project::test(fs.clone(), [], cx).await;
    let thread = new_test_thread(
        server(&fs, &project, cx).await,
        project.clone(),
        "/private/tmp",
        cx,
    )
    .await;

    thread
        .update(cx, |thread, cx| thread.send_raw("Hello from test!", cx))
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
                    ::agent_client_protocol::PermissionOptionId($allow_option_id.into()),
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

pub async fn init_test(cx: &mut TestAppContext) -> Arc<FakeFs> {
    use settings::Settings;

    env_logger::try_init().ok();

    cx.update(|cx| {
        let settings_store = settings::SettingsStore::test(cx);
        cx.set_global(settings_store);
        Project::init_settings(cx);
        language::init(cx);
        gpui_tokio::init(cx);
        let http_client = reqwest_client::ReqwestClient::user_agent("agent tests").unwrap();
        cx.set_http_client(Arc::new(http_client));
        client::init_settings(cx);
        let client = client::Client::production(cx);
        let user_store = cx.new(|cx| client::UserStore::new(client.clone(), cx));
        language_model::init(client.clone(), cx);
        language_models::init(user_store, client, cx);
        agent_settings::init(cx);
        AllAgentServersSettings::register(cx);

        #[cfg(test)]
        AllAgentServersSettings::override_global(
            AllAgentServersSettings {
                claude: Some(BuiltinAgentServerSettings {
                    path: Some("claude-code-acp".into()),
                    args: None,
                    env: None,
                    ignore_system_version: None,
                    default_mode: None,
                }),
                gemini: Some(crate::gemini::tests::local_command().into()),
                custom: collections::HashMap::default(),
            },
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
    let delegate = AgentServerDelegate::new(store, project.clone(), None, None);

    let (connection, _) = cx
        .update(|cx| server.connect(Some(current_dir.as_ref()), delegate, cx))
        .await
        .unwrap();

    cx.update(|cx| connection.new_thread(project.clone(), current_dir.as_ref(), cx))
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
        // We have to use a smol timer here because
        // cx.background_executor().timer isn't real in the test context
        _ = futures::FutureExt::fuse(smol::Timer::after(Duration::from_secs(20))) => {
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
