use acp_thread::AcpThread;
use anyhow::Result;
use context_server::{
    listener::{McpServerTool, ToolResponse},
    types::ToolAnnotations,
};
use gpui::{AsyncApp, WeakEntity};

use crate::tools::WriteToolParams;

#[derive(Clone)]
pub struct WriteTool {
    thread_rx: watch::Receiver<WeakEntity<AcpThread>>,
}

impl WriteTool {
    pub fn new(thread_rx: watch::Receiver<WeakEntity<AcpThread>>) -> Self {
        Self { thread_rx }
    }
}

impl McpServerTool for WriteTool {
    type Input = WriteToolParams;
    type Output = ();

    const NAME: &'static str = "Write";

    fn annotations(&self) -> ToolAnnotations {
        ToolAnnotations {
            title: Some("Write file".to_string()),
            read_only_hint: Some(false),
            destructive_hint: Some(false),
            open_world_hint: Some(false),
            idempotent_hint: Some(false),
        }
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
                thread.write_text_file(input.abs_path, input.content, cx)
            })?
            .await?;

        Ok(ToolResponse {
            content: vec![],
            structured_content: (),
        })
    }
}
