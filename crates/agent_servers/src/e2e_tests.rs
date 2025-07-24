use std::{path::Path, sync::Arc, time::Duration};

use crate::{AgentServer, AgentServerSettings, AllAgentServersSettings};
use acp_thread::{AcpThread, AgentThreadEntry, ToolCall, ToolCallStatus};
use agent_client_protocol as acp;

use futures::{FutureExt, StreamExt, channel::mpsc, select};
use gpui::{Entity, TestAppContext};
use indoc::indoc;
use project::{FakeFs, Project};
use serde_json::json;
use settings::{Settings, SettingsStore};
use util::path;

pub async fn test_basic(server: impl AgentServer + 'static, cx: &mut TestAppContext) {
    let fs = init_test(cx).await;
    let project = Project::test(fs, [], cx).await;
    let thread = new_test_thread(server, project.clone(), "/private/tmp", cx).await;

    thread
        .update(cx, |thread, cx| thread.send_raw("Hello from Zed!", cx))
        .await
        .unwrap();

    thread.read_with(cx, |thread, _| {
        assert_eq!(thread.entries().len(), 2);
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

pub async fn test_path_mentions(server: impl AgentServer + 'static, cx: &mut TestAppContext) {
    let _fs = init_test(cx).await;

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
    let thread = new_test_thread(server, project.clone(), tempdir.path(), cx).await;
    thread
        .update(cx, |thread, cx| {
            thread.send(
                vec![
                    acp::ContentBlock::Text(acp::TextContent {
                        text: "Read the file ".into(),
                        annotations: None,
                    }),
                    acp::ContentBlock::ResourceLink(acp::ResourceLink {
                        uri: "foo.rs".into(),
                        name: "foo.rs".into(),
                        annotations: None,
                        description: None,
                        mime_type: None,
                        size: None,
                        title: None,
                    }),
                    acp::ContentBlock::Text(acp::TextContent {
                        text: " and tell me what the content of the println! is".into(),
                        annotations: None,
                    }),
                ],
                cx,
            )
        })
        .await
        .unwrap();

    thread.read_with(cx, |thread, cx| {
        assert_eq!(thread.entries().len(), 3);
        assert!(matches!(
            thread.entries()[0],
            AgentThreadEntry::UserMessage(_)
        ));
        assert!(matches!(thread.entries()[1], AgentThreadEntry::ToolCall(_)));
        let AgentThreadEntry::AssistantMessage(assistant_message) = &thread.entries()[2] else {
            panic!("Expected AssistantMessage")
        };
        assert!(
            assistant_message.to_markdown(cx).contains("Hello, world!"),
            "unexpected assistant message: {:?}",
            assistant_message.to_markdown(cx)
        );
    });
}

pub async fn test_tool_call(server: impl AgentServer + 'static, cx: &mut TestAppContext) {
    let fs = init_test(cx).await;
    fs.insert_tree(
        path!("/private/tmp"),
        json!({"foo": "Lorem ipsum dolor", "bar": "bar", "baz": "baz"}),
    )
    .await;
    let project = Project::test(fs, [path!("/private/tmp").as_ref()], cx).await;
    let thread = new_test_thread(server, project.clone(), "/private/tmp", cx).await;

    thread
        .update(cx, |thread, cx| {
            thread.send_raw(
                "Read the '/private/tmp/foo' file and tell me what you see.",
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
                    status: ToolCallStatus::Allowed { .. },
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
}

pub async fn test_tool_call_with_confirmation(
    server: impl AgentServer + 'static,
    cx: &mut TestAppContext,
) {
    let fs = init_test(cx).await;
    let project = Project::test(fs, [path!("/private/tmp").as_ref()], cx).await;
    let thread = new_test_thread(server, project.clone(), "/private/tmp", cx).await;
    let full_turn = thread.update(cx, |thread, cx| {
        thread.send_raw(
            r#"Run `touch hello.txt && echo "Hello, world!" | tee hello.txt`"#,
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

    let tool_call_id = thread.read_with(cx, |thread, _cx| {
        let AgentThreadEntry::ToolCall(ToolCall {
            id,
            content,
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

        assert!(content.iter().any(|c| c.to_markdown(_cx).contains("touch")));

        id.clone()
    });

    thread.update(cx, |thread, cx| {
        thread.authorize_tool_call(
            tool_call_id,
            acp::PermissionOptionId("0".into()),
            acp::PermissionOptionKind::AllowOnce,
            cx,
        );

        assert!(thread.entries().iter().any(|entry| matches!(
            entry,
            AgentThreadEntry::ToolCall(ToolCall {
                status: ToolCallStatus::Allowed { .. },
                ..
            })
        )));
    });

    full_turn.await.unwrap();

    thread.read_with(cx, |thread, cx| {
        let AgentThreadEntry::ToolCall(ToolCall {
            content,
            status: ToolCallStatus::Allowed { .. },
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

pub async fn test_cancel(server: impl AgentServer + 'static, cx: &mut TestAppContext) {
    let fs = init_test(cx).await;

    let project = Project::test(fs, [path!("/private/tmp").as_ref()], cx).await;
    let thread = new_test_thread(server, project.clone(), "/private/tmp", cx).await;
    let full_turn = thread.update(cx, |thread, cx| {
        thread.send_raw(
            r#"Run `touch hello.txt && echo "Hello, world!" >> hello.txt`"#,
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

    thread.read_with(cx, |thread, _cx| {
        let AgentThreadEntry::ToolCall(ToolCall {
            id,
            content,
            status: ToolCallStatus::WaitingForConfirmation { .. },
            ..
        }) = &thread.entries()[first_tool_call_ix]
        else {
            panic!("{:?}", thread.entries()[1]);
        };

        assert!(content.iter().any(|c| c.to_markdown(_cx).contains("touch")));

        id.clone()
    });

    let _ = thread.update(cx, |thread, cx| thread.cancel(cx));
    full_turn.await.unwrap();
    thread.read_with(cx, |thread, _| {
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

#[macro_export]
macro_rules! common_e2e_tests {
    ($server:expr) => {
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
            async fn tool_call_with_confirmation(cx: &mut ::gpui::TestAppContext) {
                $crate::e2e_tests::test_tool_call_with_confirmation($server, cx).await;
            }

            #[::gpui::test]
            #[cfg_attr(not(feature = "e2e"), ignore)]
            async fn cancel(cx: &mut ::gpui::TestAppContext) {
                $crate::e2e_tests::test_cancel($server, cx).await;
            }
        }
    };
}

// Helpers

pub async fn init_test(cx: &mut TestAppContext) -> Arc<FakeFs> {
    env_logger::try_init().ok();

    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        Project::init_settings(cx);
        language::init(cx);
        crate::settings::init(cx);

        crate::AllAgentServersSettings::override_global(
            AllAgentServersSettings {
                claude: Some(AgentServerSettings {
                    command: crate::claude::tests::local_command(),
                }),
                gemini: Some(AgentServerSettings {
                    command: crate::gemini::tests::local_command(),
                }),
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
    let connection = cx
        .update(|cx| server.connect(current_dir.as_ref(), &project, cx))
        .await
        .unwrap();

    let thread = connection
        .new_thread(project.clone(), current_dir.as_ref(), &mut cx.to_async())
        .await
        .unwrap();

    thread
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
