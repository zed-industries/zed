use languages::LanguageRegistry;
use project::Project;
use std::sync::Arc;

use agent::{ActiveThread, MessageSegment, ThreadStore};
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
                    None,
                    Arc::new(PromptBuilder::new(None).unwrap()),
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
    thread.update(cx, |thread, cx| {
        thread.insert_assistant_message(vec![
            MessageSegment::Text("I'll help you fix the lifetime error in your `cx.spawn` call. When working with async operations in GPUI, there are specific patterns to follow for proper lifetime management.".to_string()),
            MessageSegment::Text("\n\nLet's look at what's happening in your code:".to_string()),
            MessageSegment::Text("\n\n---\n\nLet's check the current state of the active_thread.rs file to understand what might have changed:".to_string()),
            MessageSegment::Text("\n\n---\n\nLooking at the implementation of `load_preview_thread_store` and understanding GPUI's async patterns, here's the issue:".to_string()),
            MessageSegment::Text("\n\n1. `load_preview_thread_store` returns a `Task<anyhow::Result<Entity<ThreadStore>>>`, which means it's already a task".to_string()),
            MessageSegment::Text("\n2. When you call this function inside another `spawn` call, you're nesting tasks incorrectly".to_string()),
            MessageSegment::Text("\n3. The `this` parameter you're trying to use in your closure has the wrong context".to_string()),
            MessageSegment::Text("\n\nHere's the correct way to implement this:".to_string()),
            MessageSegment::Text("\n\n---\n\nThe problem is in how you're setting up the async closure and trying to reference variables like `window` and `language_registry` that aren't accessible in that scope.".to_string()),
            MessageSegment::Text("\n\nHere's how to fix it:".to_string()),
        ], cx);
    });
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
