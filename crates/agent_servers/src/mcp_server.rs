use acp_thread::AcpThread;
use agent_client_protocol as acp;
use anyhow::Result;
use context_server::listener::{McpServerTool, ToolResponse};
use context_server::types::{
    Implementation, InitializeParams, InitializeResponse, ProtocolVersion, ServerCapabilities,
    ToolsCapabilities, requests,
};
use futures::channel::oneshot;
use gpui::{App, AsyncApp, Task, WeakEntity};
use indoc::indoc;

pub struct ZedMcpServer {
    server: context_server::listener::McpServer,
}

pub const SERVER_NAME: &str = "zed";

impl ZedMcpServer {
    pub async fn new(
        thread_rx: watch::Receiver<WeakEntity<AcpThread>>,
        cx: &AsyncApp,
    ) -> Result<Self> {
        let mut mcp_server = context_server::listener::McpServer::new(cx).await?;
        mcp_server.handle_request::<requests::Initialize>(Self::handle_initialize);

        mcp_server.add_tool(RequestPermissionTool {
            thread_rx: thread_rx.clone(),
        });
        mcp_server.add_tool(ReadTextFileTool {
            thread_rx: thread_rx.clone(),
        });
        mcp_server.add_tool(WriteTextFileTool {
            thread_rx: thread_rx.clone(),
        });

        Ok(Self { server: mcp_server })
    }

    pub fn server_config(&self) -> Result<acp::McpServerConfig> {
        #[cfg(not(test))]
        let zed_path = anyhow::Context::context(
            std::env::current_exe(),
            "finding current executable path for use in mcp_server",
        )?;

        #[cfg(test)]
        let zed_path = crate::e2e_tests::get_zed_path();

        Ok(acp::McpServerConfig {
            command: zed_path,
            args: vec![
                "--nc".into(),
                self.server.socket_path().display().to_string(),
            ],
            env: None,
        })
    }

    fn handle_initialize(_: InitializeParams, cx: &App) -> Task<Result<InitializeResponse>> {
        cx.foreground_executor().spawn(async move {
            Ok(InitializeResponse {
                protocol_version: ProtocolVersion("2025-06-18".into()),
                capabilities: ServerCapabilities {
                    experimental: None,
                    logging: None,
                    completions: None,
                    prompts: None,
                    resources: None,
                    tools: Some(ToolsCapabilities {
                        list_changed: Some(false),
                    }),
                },
                server_info: Implementation {
                    name: SERVER_NAME.into(),
                    version: "0.1.0".into(),
                },
                meta: None,
            })
        })
    }
}

// Tools

#[derive(Clone)]
pub struct RequestPermissionTool {
    thread_rx: watch::Receiver<WeakEntity<AcpThread>>,
}

impl McpServerTool for RequestPermissionTool {
    type Input = acp::RequestPermissionArguments;
    type Output = acp::RequestPermissionOutput;

    const NAME: &'static str = "Confirmation";

    fn description(&self) -> &'static str {
        indoc! {"
            Request permission for tool calls.

            This tool is meant to be called programmatically by the agent loop, not the LLM.
        "}
    }

    async fn run(
        &self,
        input: Self::Input,
        cx: &mut AsyncApp,
    ) -> Result<ToolResponse<Self::Output>> {
        let mut thread_rx = self.thread_rx.clone();
        let Some(thread) = thread_rx.recv().await?.upgrade() else {
            anyhow::bail!("Thread closed");
        };

        let result = thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_permission(input.tool_call, input.options, cx)
            })?
            .await;

        let outcome = match result {
            Ok(option_id) => acp::RequestPermissionOutcome::Selected { option_id },
            Err(oneshot::Canceled) => acp::RequestPermissionOutcome::Canceled,
        };

        Ok(ToolResponse {
            content: vec![],
            structured_content: acp::RequestPermissionOutput { outcome },
        })
    }
}

#[derive(Clone)]
pub struct ReadTextFileTool {
    thread_rx: watch::Receiver<WeakEntity<AcpThread>>,
}

impl McpServerTool for ReadTextFileTool {
    type Input = acp::ReadTextFileArguments;
    type Output = acp::ReadTextFileOutput;

    const NAME: &'static str = "Read";

    fn description(&self) -> &'static str {
        "Reads the content of the given file in the project including unsaved changes."
    }

    async fn run(
        &self,
        input: Self::Input,
        cx: &mut AsyncApp,
    ) -> Result<ToolResponse<Self::Output>> {
        let mut thread_rx = self.thread_rx.clone();
        let Some(thread) = thread_rx.recv().await?.upgrade() else {
            anyhow::bail!("Thread closed");
        };

        let content = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(input.path, input.line, input.limit, false, cx)
            })?
            .await?;

        Ok(ToolResponse {
            content: vec![],
            structured_content: acp::ReadTextFileOutput { content },
        })
    }
}

#[derive(Clone)]
pub struct WriteTextFileTool {
    thread_rx: watch::Receiver<WeakEntity<AcpThread>>,
}

impl McpServerTool for WriteTextFileTool {
    type Input = acp::WriteTextFileArguments;
    type Output = ();

    const NAME: &'static str = "Write";

    fn description(&self) -> &'static str {
        "Write to a file replacing its contents"
    }

    async fn run(
        &self,
        input: Self::Input,
        cx: &mut AsyncApp,
    ) -> Result<ToolResponse<Self::Output>> {
        let mut thread_rx = self.thread_rx.clone();
        let Some(thread) = thread_rx.recv().await?.upgrade() else {
            anyhow::bail!("Thread closed");
        };

        thread
            .update(cx, |thread, cx| {
                thread.write_text_file(input.path, input.content, cx)
            })?
            .await?;

        Ok(ToolResponse {
            content: vec![],
            structured_content: (),
        })
    }
}
