use acp_thread::AcpThread;
use anyhow::Result;
use context_server::{
    listener::{McpServerTool, ToolResponse},
    types::{ToolAnnotations, ToolResponseContent},
};
use gpui::{AsyncApp, WeakEntity};

use crate::tools::ReadToolParams;

#[derive(Clone)]
pub struct ReadTool {
    thread_rx: watch::Receiver<WeakEntity<AcpThread>>,
}

impl ReadTool {
    pub fn new(thread_rx: watch::Receiver<WeakEntity<AcpThread>>) -> Self {
        Self { thread_rx }
    }
}

impl McpServerTool for ReadTool {
    type Input = ReadToolParams;
    type Output = ();

    const NAME: &'static str = "Read";

    fn annotations(&self) -> ToolAnnotations {
        ToolAnnotations {
            title: Some("Read file".to_string()),
            read_only_hint: Some(true),
            destructive_hint: Some(false),
            open_world_hint: Some(false),
            idempotent_hint: None,
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

        let content = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(input.abs_path, input.offset, input.limit, false, cx)
            })?
            .await?;

        Ok(ToolResponse {
            content: vec![ToolResponseContent::Text { text: content }],
            structured_content: (),
        })
    }
}
