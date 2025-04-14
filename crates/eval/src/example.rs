use agent::{RequestKind, ThreadEvent, ThreadStore};
use anyhow::{Result, anyhow};
use assistant_tool::ToolWorkingSet;
use dap::DapRegistry;
use futures::channel::oneshot;
use gpui::{App, Task};
use language_model::{LanguageModel, StopReason};
use project::Project;
use serde::Deserialize;
use std::process::Command;
use std::sync::Arc;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::AgentAppState;

#[derive(Debug, Deserialize)]
pub struct ExampleBase {
    pub path: PathBuf,
    pub revision: String,
}

#[derive(Debug)]
pub struct Example {
    pub base: ExampleBase,

    /// Content of the prompt.md file
    pub prompt: String,

    /// Content of the rubric.md file
    pub _rubric: String,
}

impl Example {
    /// Load an example from a directory containing base.toml, prompt.md, and rubric.md
    pub fn load_from_directory<P: AsRef<Path>>(dir_path: P) -> Result<Self> {
        let base_path = dir_path.as_ref().join("base.toml");
        let prompt_path = dir_path.as_ref().join("prompt.md");
        let rubric_path = dir_path.as_ref().join("rubric.md");

        let mut base: ExampleBase = toml::from_str(&fs::read_to_string(&base_path)?)?;
        base.path = base.path.canonicalize()?;

        Ok(Example {
            base,
            prompt: fs::read_to_string(prompt_path)?,
            _rubric: fs::read_to_string(rubric_path)?,
        })
    }

    /// Set up the example by checking out the specified Git revision
    pub fn setup(&self) -> Result<()> {
        // Check if the directory exists
        let path = Path::new(&self.base.path);
        anyhow::ensure!(path.exists(), "Path does not exist: {:?}", self.base.path);

        // Change to the project directory and checkout the specified revision
        let output = Command::new("git")
            .current_dir(&self.base.path)
            .arg("checkout")
            .arg(&self.base.revision)
            .output()?;
        anyhow::ensure!(
            output.status.success(),
            "Failed to checkout revision {}: {}",
            self.base.revision,
            String::from_utf8_lossy(&output.stderr),
        );

        Ok(())
    }

    pub fn run(
        self,
        model: Arc<dyn LanguageModel>,
        app_state: Arc<AgentAppState>,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let project = Project::local(
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            Arc::new(DapRegistry::default()),
            app_state.fs.clone(),
            None,
            cx,
        );

        let worktree = project.update(cx, |project, cx| {
            project.create_worktree(self.base.path, true, cx)
        });

        let tools = Arc::new(ToolWorkingSet::default());
        let thread_store =
            ThreadStore::load(project.clone(), tools, app_state.prompt_builder.clone(), cx);

        println!("USER:");
        println!("{}", self.prompt);
        println!("ASSISTANT:");
        cx.spawn(async move |cx| {
            worktree.await?;
            let thread_store = thread_store.await;
            let thread =
                thread_store.update(cx, |thread_store, cx| thread_store.create_thread(cx))?;

            let (tx, rx) = oneshot::channel();
            let mut tx = Some(tx);

            let _subscription =
                cx.subscribe(
                    &thread,
                    move |thread, event: &ThreadEvent, cx| match event {
                        ThreadEvent::Stopped(reason) => match reason {
                            Ok(StopReason::EndTurn) => {
                                if let Some(tx) = tx.take() {
                                    tx.send(Ok(())).ok();
                                }
                            }
                            Ok(StopReason::MaxTokens) => {
                                if let Some(tx) = tx.take() {
                                    tx.send(Err(anyhow!("Exceeded maximum tokens"))).ok();
                                }
                            }
                            Ok(StopReason::ToolUse) => {}
                            Err(error) => {
                                if let Some(tx) = tx.take() {
                                    tx.send(Err(anyhow!(error.clone()))).ok();
                                }
                            }
                        },
                        ThreadEvent::ShowError(thread_error) => {
                            if let Some(tx) = tx.take() {
                                tx.send(Err(anyhow!(thread_error.clone()))).ok();
                            }
                        }
                        ThreadEvent::StreamedAssistantText(_, chunk) => {
                            print!("{}", chunk);
                        }
                        ThreadEvent::StreamedAssistantThinking(_, chunk) => {
                            print!("{}", chunk);
                        }
                        ThreadEvent::UsePendingTools { tool_uses } => {
                            println!("\n\nUSING TOOLS:");
                            for tool_use in tool_uses {
                                println!("{}: {}", tool_use.name, tool_use.input);
                            }
                        }
                        ThreadEvent::ToolFinished {
                            tool_use_id,
                            pending_tool_use,
                            ..
                        } => {
                            if let Some(tool_use) = pending_tool_use {
                                println!("\nTOOL FINISHED: {}", tool_use.name);
                            }
                            if let Some(tool_result) = thread.read(cx).tool_result(tool_use_id) {
                                println!("\n{}\n", tool_result.content);
                            }
                        }
                        _ => {}
                    },
                )?;

            thread.update(cx, |thread, cx| {
                let context = vec![];
                thread.insert_user_message(self.prompt.clone(), context, None, cx);
                thread.send_to_model(model, RequestKind::Chat, cx);
            })?;

            rx.await??;

            Ok(())
        })
    }
}
