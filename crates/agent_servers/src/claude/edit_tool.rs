use acp_thread::AcpThread;
use anyhow::Result;
use context_server::{
    listener::{McpServerTool, ToolResponse},
    types::{ToolAnnotations, ToolResponseContent},
};
use gpui::{AsyncApp, WeakEntity};
use language::unified_diff;
use util::markdown::MarkdownCodeBlock;

use crate::tools::EditToolParams;

#[derive(Clone)]
pub struct EditTool {
    thread_rx: watch::Receiver<WeakEntity<AcpThread>>,
}

impl EditTool {
    pub fn new(thread_rx: watch::Receiver<WeakEntity<AcpThread>>) -> Self {
        Self { thread_rx }
    }
}

impl McpServerTool for EditTool {
    type Input = EditToolParams;
    type Output = ();

    const NAME: &'static str = "Edit";

    fn annotations(&self) -> ToolAnnotations {
        ToolAnnotations {
            title: Some("Edit file".to_string()),
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

        let content = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(input.abs_path.clone(), None, None, true, cx)
            })?
            .await?;

        let (new_content, diff) = cx
            .background_executor()
            .spawn(async move {
                let new_content = content.replace(&input.old_text, &input.new_text);
                if new_content == content {
                    return Err(anyhow::anyhow!("Failed to find `old_text`",));
                }
                let diff = unified_diff(&content, &new_content);

                Ok((new_content, diff))
            })
            .await?;

        thread
            .update(cx, |thread, cx| {
                thread.write_text_file(input.abs_path, new_content, cx)
            })?
            .await?;

        Ok(ToolResponse {
            content: vec![ToolResponseContent::Text {
                text: MarkdownCodeBlock {
                    tag: "diff",
                    text: diff.as_str().trim_end_matches('\n'),
                }
                .to_string(),
            }],
            structured_content: (),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use acp_thread::{AgentConnection, StubAgentConnection};
    use gpui::{Entity, TestAppContext};
    use indoc::indoc;
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    use super::*;

    #[gpui::test]
    async fn old_text_not_found(cx: &mut TestAppContext) {
        let (_thread, tool) = init_test(cx).await;

        let result = tool
            .run(
                EditToolParams {
                    abs_path: path!("/root/file.txt").into(),
                    old_text: "hi".into(),
                    new_text: "bye".into(),
                },
                &mut cx.to_async(),
            )
            .await;

        assert_eq!(result.unwrap_err().to_string(), "Failed to find `old_text`");
    }

    #[gpui::test]
    async fn found_and_replaced(cx: &mut TestAppContext) {
        let (_thread, tool) = init_test(cx).await;

        let result = tool
            .run(
                EditToolParams {
                    abs_path: path!("/root/file.txt").into(),
                    old_text: "hello".into(),
                    new_text: "hi".into(),
                },
                &mut cx.to_async(),
            )
            .await;

        assert_eq!(
            result.unwrap().content[0].text().unwrap(),
            indoc! {
                r"
                ```diff
                @@ -1,1 +1,1 @@
                -hello
                +hi
                ```
                "
            }
        );
    }

    async fn init_test(cx: &mut TestAppContext) -> (Entity<AcpThread>, EditTool) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
        });

        let connection = Rc::new(StubAgentConnection::new());
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "file.txt": "hello"
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let (mut thread_tx, thread_rx) = watch::channel(WeakEntity::new_invalid());

        let thread = cx
            .update(|cx| connection.new_thread(project, path!("/test").as_ref(), cx))
            .await
            .unwrap();

        thread_tx.send(thread.downgrade()).unwrap();

        (thread, EditTool::new(thread_rx))
    }
}
