use std::sync::Arc;

use agent::{ActiveThread, Thread, ThreadStore};
use assistant_tool::ToolWorkingSet;
use gpui::{AppContext, WeakEntity};
use prompt_store::PromptBuilder;
use ui::App;
use workspace::Workspace;

fn static_active_thread(
    weak_workspace: WeakEntity<Workspace>,
    cx: &mut App,
) -> anyhow::Result<ActiveThread> {
    if let Some(workspace) = weak_workspace.upgrade() {
        let project = workspace.read(cx).project().clone();
        let tools = cx.new(|_| ToolWorkingSet::default());
        let prompt_builder = Arc::new(PromptBuilder::new(None)?);
        // let system_prompt = cx.new(|_| SystemPrompt::default());

        let thread_store = cx.new(|cx| {
            Ok(ThreadStore::new(
                project.clone(),
                tools.clone(),
                prompt_builder.clone(),
                None,
                cx,
            ))
        });

        let thread = Thread::new(project.clone(), tools, prompt_builder, system_prompt, cx);
        Ok(ActiveThread::new(thread))
    } else {
        anyhow::bail!("Workspace is no longer available")
    }
}
