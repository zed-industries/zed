use crate::stdio_agent_server::{StdioAgentServer, find_bin_in_path};
use crate::{AgentServerCommand, AgentServerVersion};
use anyhow::{Context as _, Result};
use gpui::{AsyncApp, Entity};
use project::Project;
use settings::SettingsStore;

use crate::AllAgentServersSettings;

#[derive(Clone)]
pub struct Gemini;

const ACP_ARG: &str = "--experimental-acp";

impl StdioAgentServer for Gemini {
    fn name(&self) -> &'static str {
        "Gemini"
    }

    fn empty_state_headline(&self) -> &'static str {
        "Welcome to Gemini"
    }

    fn empty_state_message(&self) -> &'static str {
        "Ask questions, edit files, run commands.\nBe specific for the best results."
    }

    fn supports_always_allow(&self) -> bool {
        true
    }

    fn logo(&self) -> ui::IconName {
        ui::IconName::AiGemini
    }

    async fn command(
        &self,
        project: &Entity<Project>,
        cx: &mut AsyncApp,
    ) -> Result<AgentServerCommand> {
        let custom_command = cx.read_global(|settings: &SettingsStore, _| {
            let settings = settings.get::<AllAgentServersSettings>(None);
            settings
                .gemini
                .as_ref()
                .map(|gemini_settings| AgentServerCommand {
                    path: gemini_settings.command.path.clone(),
                    args: gemini_settings
                        .command
                        .args
                        .iter()
                        .cloned()
                        .chain(std::iter::once(ACP_ARG.into()))
                        .collect(),
                    env: gemini_settings.command.env.clone(),
                })
        })?;

        if let Some(custom_command) = custom_command {
            return Ok(custom_command);
        }

        if let Some(path) = find_bin_in_path("gemini", project, cx).await {
            return Ok(AgentServerCommand {
                path,
                args: vec![ACP_ARG.into()],
                env: None,
            });
        }

        let (fs, node_runtime) = project.update(cx, |project, _| {
            (project.fs().clone(), project.node_runtime().cloned())
        })?;
        let node_runtime = node_runtime.context("gemini not found on path")?;

        let directory = ::paths::agent_servers_dir().join("gemini");
        fs.create_dir(&directory).await?;
        node_runtime
            .npm_install_packages(&directory, &[("@google/gemini-cli", "latest")])
            .await?;
        let path = directory.join("node_modules/.bin/gemini");

        Ok(AgentServerCommand {
            path,
            args: vec![ACP_ARG.into()],
            env: None,
        })
    }

    async fn version(&self, command: &AgentServerCommand) -> Result<AgentServerVersion> {
        let version_fut = util::command::new_smol_command(&command.path)
            .args(command.args.iter())
            .arg("--version")
            .kill_on_drop(true)
            .output();

        let help_fut = util::command::new_smol_command(&command.path)
            .args(command.args.iter())
            .arg("--help")
            .kill_on_drop(true)
            .output();

        let (version_output, help_output) = futures::future::join(version_fut, help_fut).await;

        let current_version = String::from_utf8(version_output?.stdout)?;
        let supported = String::from_utf8(help_output?.stdout)?.contains(ACP_ARG);

        if supported {
            Ok(AgentServerVersion::Supported)
        } else {
            Ok(AgentServerVersion::Unsupported {
                error_message: format!(
                    "Your installed version of Gemini {} doesn't support the Agentic Coding Protocol (ACP).",
                    current_version
                ).into(),
                upgrade_message: "Upgrade Gemini to Latest".into(),
                upgrade_command: "npm install -g @google/gemini-cli@latest".into(),
            })
        }
    }
}

#[cfg(test)]
mod test {
    use std::{path::Path, time::Duration};

    use acp_thread::{
        AcpThread, AgentThreadEntry, ToolCall, ToolCallConfirmation, ToolCallContent,
        ToolCallStatus,
    };
    use agentic_coding_protocol as acp;
    use anyhow::Result;
    use futures::{FutureExt, StreamExt, channel::mpsc, select};
    use gpui::{AsyncApp, Entity, TestAppContext};
    use indoc::indoc;
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    use crate::{AgentServer, AgentServerCommand, AgentServerVersion, StdioAgentServer};

    pub async fn gemini_acp_thread(
        project: Entity<Project>,
        current_dir: impl AsRef<Path>,
        cx: &mut TestAppContext,
    ) -> Entity<AcpThread> {
        #[derive(Clone)]
        struct DevGemini;

        impl StdioAgentServer for DevGemini {
            async fn command(
                &self,
                _project: &Entity<Project>,
                _cx: &mut AsyncApp,
            ) -> Result<AgentServerCommand> {
                let cli_path = Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../../../gemini-cli/packages/cli")
                    .to_string_lossy()
                    .to_string();

                Ok(AgentServerCommand {
                    path: "node".into(),
                    args: vec![cli_path, "--experimental-acp".into()],
                    env: None,
                })
            }

            async fn version(&self, _command: &AgentServerCommand) -> Result<AgentServerVersion> {
                Ok(AgentServerVersion::Supported)
            }

            fn logo(&self) -> ui::IconName {
                ui::IconName::AiGemini
            }

            fn name(&self) -> &'static str {
                "test"
            }

            fn empty_state_headline(&self) -> &'static str {
                "test"
            }

            fn empty_state_message(&self) -> &'static str {
                "test"
            }

            fn supports_always_allow(&self) -> bool {
                true
            }
        }

        let thread = cx
            .update(|cx| AgentServer::new_thread(&DevGemini, current_dir.as_ref(), &project, cx))
            .await
            .unwrap();

        thread
            .update(cx, |thread, _| thread.initialize())
            .await
            .unwrap();
        thread
    }

    fn init_test(cx: &mut TestAppContext) {
        env_logger::try_init().ok();
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            Project::init_settings(cx);
            language::init(cx);
        });
    }

    #[gpui::test]
    #[cfg_attr(not(feature = "gemini"), ignore)]
    async fn test_gemini_basic(cx: &mut TestAppContext) {
        init_test(cx);

        cx.executor().allow_parking();

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let thread = gemini_acp_thread(project.clone(), "/private/tmp", cx).await;
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

    #[gpui::test]
    #[cfg_attr(not(feature = "gemini"), ignore)]
    async fn test_gemini_path_mentions(cx: &mut TestAppContext) {
        init_test(cx);

        cx.executor().allow_parking();
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
        let thread = gemini_acp_thread(project.clone(), tempdir.path(), cx).await;
        thread
            .update(cx, |thread, cx| {
                thread.send(
                    acp::SendUserMessageParams {
                        chunks: vec![
                            acp::UserMessageChunk::Text {
                                text: "Read the file ".into(),
                            },
                            acp::UserMessageChunk::Path {
                                path: Path::new("foo.rs").into(),
                            },
                            acp::UserMessageChunk::Text {
                                text: " and tell me what the content of the println! is".into(),
                            },
                        ],
                    },
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

    #[gpui::test]
    #[cfg_attr(not(feature = "gemini"), ignore)]
    async fn test_gemini_tool_call(cx: &mut TestAppContext) {
        init_test(cx);

        cx.executor().allow_parking();

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/private/tmp"),
            json!({"foo": "Lorem ipsum dolor", "bar": "bar", "baz": "baz"}),
        )
        .await;
        let project = Project::test(fs, [path!("/private/tmp").as_ref()], cx).await;
        let thread = gemini_acp_thread(project.clone(), "/private/tmp", cx).await;
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
            assert!(matches!(
                &thread.entries()[2],
                AgentThreadEntry::ToolCall(ToolCall {
                    status: ToolCallStatus::Allowed { .. },
                    ..
                })
            ));

            assert!(matches!(
                thread.entries()[3],
                AgentThreadEntry::AssistantMessage(_)
            ));
        });
    }

    #[gpui::test]
    #[cfg_attr(not(feature = "gemini"), ignore)]
    async fn test_gemini_tool_call_with_confirmation(cx: &mut TestAppContext) {
        init_test(cx);

        cx.executor().allow_parking();

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [path!("/private/tmp").as_ref()], cx).await;
        let thread = gemini_acp_thread(project.clone(), "/private/tmp", cx).await;
        let full_turn = thread.update(cx, |thread, cx| {
            thread.send_raw(r#"Run `echo "Hello, world!"`"#, cx)
        });

        run_until_first_tool_call(&thread, cx).await;

        let tool_call_id = thread.read_with(cx, |thread, _cx| {
            let AgentThreadEntry::ToolCall(ToolCall {
                id,
                status:
                    ToolCallStatus::WaitingForConfirmation {
                        confirmation: ToolCallConfirmation::Execute { root_command, .. },
                        ..
                    },
                ..
            }) = &thread.entries()[2]
            else {
                panic!();
            };

            assert_eq!(root_command, "echo");

            *id
        });

        thread.update(cx, |thread, cx| {
            thread.authorize_tool_call(tool_call_id, acp::ToolCallConfirmationOutcome::Allow, cx);

            assert!(matches!(
                &thread.entries()[2],
                AgentThreadEntry::ToolCall(ToolCall {
                    status: ToolCallStatus::Allowed { .. },
                    ..
                })
            ));
        });

        full_turn.await.unwrap();

        thread.read_with(cx, |thread, cx| {
            let AgentThreadEntry::ToolCall(ToolCall {
                content: Some(ToolCallContent::Markdown { markdown }),
                status: ToolCallStatus::Allowed { .. },
                ..
            }) = &thread.entries()[2]
            else {
                panic!();
            };

            markdown.read_with(cx, |md, _cx| {
                assert!(
                    md.source().contains("Hello, world!"),
                    r#"Expected '{}' to contain "Hello, world!""#,
                    md.source()
                );
            });
        });
    }

    #[gpui::test]
    #[cfg_attr(not(feature = "gemini"), ignore)]
    async fn test_gemini_cancel(cx: &mut TestAppContext) {
        init_test(cx);

        cx.executor().allow_parking();

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [path!("/private/tmp").as_ref()], cx).await;
        let thread = gemini_acp_thread(project.clone(), "/private/tmp", cx).await;
        let full_turn = thread.update(cx, |thread, cx| {
            thread.send_raw(r#"Run `echo "Hello, world!"`"#, cx)
        });

        let first_tool_call_ix = run_until_first_tool_call(&thread, cx).await;

        thread.read_with(cx, |thread, _cx| {
            let AgentThreadEntry::ToolCall(ToolCall {
                id,
                status:
                    ToolCallStatus::WaitingForConfirmation {
                        confirmation: ToolCallConfirmation::Execute { root_command, .. },
                        ..
                    },
                ..
            }) = &thread.entries()[first_tool_call_ix]
            else {
                panic!("{:?}", thread.entries()[1]);
            };

            assert_eq!(root_command, "echo");

            *id
        });

        thread
            .update(cx, |thread, cx| thread.cancel(cx))
            .await
            .unwrap();
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

    async fn run_until_first_tool_call(
        thread: &Entity<AcpThread>,
        cx: &mut TestAppContext,
    ) -> usize {
        let (mut tx, mut rx) = mpsc::channel::<usize>(1);

        let subscription = cx.update(|cx| {
            cx.subscribe(thread, move |thread, _, cx| {
                for (ix, entry) in thread.read(cx).entries().iter().enumerate() {
                    if matches!(entry, AgentThreadEntry::ToolCall(_)) {
                        return tx.try_send(ix).unwrap();
                    }
                }
            })
        });

        select! {
            _ =  cx.executor().timer(Duration::from_secs(10)).fuse() => {
                panic!("Timeout waiting for tool call")
            }
            ix = rx.next().fuse() => {
                drop(subscription);
                ix.unwrap()
            }
        }
    }
}
