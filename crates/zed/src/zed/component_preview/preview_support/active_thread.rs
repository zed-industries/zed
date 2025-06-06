use agent::{ActiveThread, ContextStore, MessageSegment, TextThreadStore, ThreadStore};
use anyhow::{Result, anyhow};
use assistant_tool::ToolWorkingSet;
use gpui::{AppContext, AsyncApp, Entity, Task, WeakEntity};
use indoc::indoc;
use languages::LanguageRegistry;
use project::Project;
use prompt_store::PromptBuilder;
use std::sync::Arc;
use ui::{App, Window};
use workspace::Workspace;

pub fn load_preview_thread_store(
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    cx: &mut AsyncApp,
) -> Task<Result<Entity<ThreadStore>>> {
    workspace
        .update(cx, |_, cx| {
            ThreadStore::load(
                project.clone(),
                cx.new(|_| ToolWorkingSet::default()),
                None,
                Arc::new(PromptBuilder::new(None).unwrap()),
                cx,
            )
        })
        .unwrap_or(Task::ready(Err(anyhow!("workspace dropped"))))
}

pub fn load_preview_text_thread_store(
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    cx: &mut AsyncApp,
) -> Task<Result<Entity<TextThreadStore>>> {
    workspace
        .update(cx, |_, cx| {
            TextThreadStore::new(
                project.clone(),
                Arc::new(PromptBuilder::new(None).unwrap()),
                Default::default(),
                cx,
            )
        })
        .unwrap_or(Task::ready(Err(anyhow!("workspace dropped"))))
}

pub fn static_active_thread(
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    language_registry: Arc<LanguageRegistry>,
    thread_store: Entity<ThreadStore>,
    text_thread_store: Entity<TextThreadStore>,
    window: &mut Window,
    cx: &mut App,
) -> Entity<ActiveThread> {
    let context_store =
        cx.new(|_| ContextStore::new(project.downgrade(), Some(thread_store.downgrade())));

    let thread = thread_store.update(cx, |thread_store, cx| thread_store.create_thread(cx));
    thread.update(cx, |thread, cx| {
        thread.insert_assistant_message(vec![
            MessageSegment::Text(indoc! {"
                I'll help you fix the lifetime error in your `cx.spawn` call. When working with async operations in GPUI, there are specific patterns to follow for proper lifetime management.

                Let's look at what's happening in your code:

                ---

                Let's check the current state of the active_thread.rs file to understand what might have changed:

                ---

                Looking at the implementation of `load_preview_thread_store` and understanding GPUI's async patterns, here's the issue:

                1. `load_preview_thread_store` returns a `Task<anyhow::Result<Entity<ThreadStore>>>`, which means it's already a task.
                2. When you call this function inside another `spawn` call, you're nesting tasks incorrectly.

                Here's the correct way to implement this:

                ---

                The problem is in how you're setting up the async closure and trying to reference variables like `window` and `language_registry` that aren't accessible in that scope.

                Here's how to fix it:
            "}.to_string()),
        ], cx);
    });
    cx.new(|cx| {
        ActiveThread::new(
            thread,
            thread_store,
            text_thread_store,
            context_store,
            language_registry,
            workspace.clone(),
            window,
            cx,
        )
    })
}
