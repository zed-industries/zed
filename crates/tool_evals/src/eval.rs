use crate::headless_assistant::{authenticate_model_provider, find_model, HeadlessAssistant};
use anyhow::anyhow;
use assistant2::{Message, RequestKind, Thread, ThreadEvent, ThreadStore};
use assistant_tool::ToolWorkingSet;
use client::Client;
use git::GitHostingProviderRegistry;
use gpui::{prelude::*, App, Entity, Subscription, Task};
use language::LanguageRegistry;
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelProviderId, LanguageModelRegistry,
};
use project::{Project, RealFs};
use prompt_store::PromptBuilder;
use settings::SettingsStore;
use smol::channel;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use workspace::AppState;

pub struct Eval {
    pub repo_path: PathBuf,
    pub system_prompt: Option<String>,
    pub user_query: String,
    pub provider_id: String,
    pub model_name: String,
}

impl Eval {
    /// Runs the eval. Note that this cannot be run concurrently because
    /// LanguageModelRegistry.active_model is global state.
    pub fn run(
        &self,
        app_state: Arc<AppState>,
        cx: &mut App,
    ) -> Task<anyhow::Result<Vec<Message>>> {
        let model = match find_model(&self.model_name, cx) {
            Ok(model) => model,
            Err(err) => return Task::ready(Err(err)),
        };

        LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
            registry.set_active_model(Some(model.clone()), cx);
        });

        let provider_id = LanguageModelProviderId(self.provider_id.clone().into());
        let authenticate_task = authenticate_model_provider(provider_id, cx);

        let repo_path = self.repo_path.clone();
        let system_prompt = self.system_prompt.clone();
        let user_query = self.user_query.clone();

        cx.spawn(move |mut cx| async move {
            authenticate_task.await?;

            let (assistant, done_rx) =
                cx.update(|cx| HeadlessAssistant::new(app_state.clone(), cx))??;

            let _worktree = assistant
                .update(&mut cx, |assistant, cx| {
                    assistant.project.update(cx, |project, cx| {
                        project.create_worktree(&repo_path, true, cx)
                    })
                })?
                .await?;

            assistant.update(&mut cx, |assistant, cx| {
                assistant.thread.update(cx, |thread, cx| {
                    let context = vec![];
                    if let Some(system_prompt) = system_prompt {
                        thread.insert_message(
                            language_model::Role::System,
                            system_prompt.clone(),
                            cx,
                        );
                    }
                    thread.insert_user_message(user_query.clone(), context, cx);
                    thread.send_to_model(model, RequestKind::Chat, true, cx);
                });
            })?;

            done_rx.recv().await??;

            assistant.update(&mut cx, |assistant, cx| {
                assistant
                    .thread
                    .read(cx)
                    .messages()
                    .cloned()
                    .collect::<Vec<_>>()
            })
        })
    }
}
