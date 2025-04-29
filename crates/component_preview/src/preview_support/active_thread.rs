use languages::LanguageRegistry;
use project::Project;
use std::{cell::RefCell, rc::Rc, sync::Arc};
use util::ResultExt;

use agent::{ActiveThread, Thread, ThreadStore};
use assistant_tool::ToolWorkingSet;
use gpui::{AppContext, AsyncApp, Entity, Task, WeakEntity};
use prompt_store::PromptBuilder;
use ui::{App, Window};
use workspace::Workspace;

pub async fn load_preview_thread_store(
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    cx: &mut AsyncApp,
) -> Task<anyhow::Result<Entity<ThreadStore>>> {
    cx.spawn(async move |cx| {
        workspace
            .update(cx, |_, cx| {
                ThreadStore::load(
                    project.clone(),
                    cx.new(|_| ToolWorkingSet::default()),
                    Arc::new(PromptBuilder::new(None).unwrap()),
                    None,
                    cx,
                )
            })?
            .await
    })
}

pub fn static_active_thread(
    workspace: WeakEntity<Workspace>,
    language_registry: Arc<LanguageRegistry>,
    thread_store: Entity<ThreadStore>,
    window: &mut Window,
    cx: &mut App,
) -> Entity<ActiveThread> {
    let thread = thread_store.update(cx, |thread_store, cx| thread_store.create_thread(cx));

    cx.new(|cx| {
        ActiveThread::new(
            thread,
            thread_store,
            language_registry,
            workspace.clone(),
            window,
            cx,
        )
    })
}
