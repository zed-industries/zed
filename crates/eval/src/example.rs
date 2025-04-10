use agent::{Thread, ThreadEvent, ThreadStore};
use anyhow::{Result, anyhow};
use assistant_tool::ToolWorkingSet;
use dap::DapRegistry;
use futures::channel::oneshot;
use gpui::{App, AppContext, Entity, Task};
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
    pub rubric: String,
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
            rubric: fs::read_to_string(rubric_path)?,
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
        &self,
        app_state: Arc<AgentAppState>,
        cx: &mut App,
    ) -> impl 'static + Future<Output = Result<()>> + use<> {
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

        let tools = Arc::new(ToolWorkingSet::default());
        let thread_store = cx.new(|cx| {
            ThreadStore::new(project.clone(), tools, app_state.prompt_builder.clone(), cx)
        });

        let thread = thread_store.update(cx, |thread_store, cx| thread_store.create_thread(cx));

        let (tx, rx) = oneshot::channel();
        let mut tx = Some(tx);

        let subscription = cx.subscribe(
            &thread,
            move |thread, event: &ThreadEvent, cx| match event {
                ThreadEvent::DoneStreaming => {
                    if let Some(tx) = tx.take() {
                        _ = tx.send(Ok(()));
                    }
                }
                ThreadEvent::ShowError(thread_error) => {
                    if let Some(tx) = tx.take() {
                        _ = tx.send(Err(anyhow!(thread_error.clone())));
                    }
                }
                ThreadEvent::ToolFinished {
                    tool_use_id,
                    pending_tool_use,
                } => todo!(),
                _ => {}
            },
        );

        // let (system_prompt_context, load_error) = thread.read(cx).load_system_prompt_context(cx)?;

        // thread.update(cx, |thread, cx| {
        //     let context = vec![];
        //     thread.insert_user_message(self.prompt.clone(), context, None, cx);
        //     thread.set_system_prompt_context(system_prompt_context);
        //     thread.send_to_model(model, RequestKind::Chat, cx);
        // });

        async move {
            rx.await??;
            drop(subscription);
            Ok(())
        }
    }
}
