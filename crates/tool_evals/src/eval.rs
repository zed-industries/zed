use crate::headless_assistant::{
    authenticate_model_provider, find_model, HeadlessAppState, HeadlessAssistant,
};
use assistant2::{Message, RequestKind};
use gpui::{App, Task};
use language_model::{LanguageModelProviderId, LanguageModelRegistry};
use serde::Deserialize;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub struct Eval {
    pub repo_path: PathBuf,
    pub system_prompt: Option<String>,
    pub user_query: String,
    pub provider_id: LanguageModelProviderId,
    pub model_name: String,
}

#[derive(Deserialize)]
struct EvalSetup {
    pub url: String,
    pub base_sha: String,
}

impl Eval {
    /// Loads the eval from a path (typically in `evaluation_data`). Clones and checks out the repo
    /// if necessary.
    pub fn load(
        eval_path: &Path,
        repo_path: &Path,
        provider_id: LanguageModelProviderId,
        model_name: String,
    ) -> anyhow::Result<Self> {
        let user_query = std::fs::read_to_string(eval_path.join("prompt.txt"))?;
        let setup_contents = std::fs::read_to_string(eval_path.join("setup.json"))?;
        let setup = serde_json_lenient::from_str_lenient::<EvalSetup>(&setup_contents)?;

        setup.checkout_repo(repo_path)?;

        Ok(Eval {
            repo_path: repo_path.to_path_buf(),
            system_prompt: None,
            user_query,
            provider_id,
            model_name,
        })
    }

    /// Runs the eval. Note that this cannot be run concurrently because
    /// LanguageModelRegistry.active_model is global state.
    pub fn run(
        &self,
        app_state: Arc<HeadlessAppState>,
        cx: &mut App,
    ) -> Task<anyhow::Result<Vec<Message>>> {
        let model = match find_model(&self.model_name, cx) {
            Ok(model) => model,
            Err(err) => return Task::ready(Err(err)),
        };

        LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
            registry.set_active_model(Some(model.clone()), cx);
        });

        let authenticate_task = authenticate_model_provider(self.provider_id.clone(), cx);

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

impl EvalSetup {
    fn checkout_repo(&self, repo_path: &Path) -> anyhow::Result<()> {
        if !repo_path.exists() {
            if let Some(parent) = repo_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut child = std::process::Command::new("git")
                .arg("clone")
                .arg(&self.url)
                .arg(repo_path)
                .spawn()?;
            let exit_status = child.wait().unwrap();
            if !exit_status.success() {
                panic!("git clone exited with failure status {exit_status}");
            }
        }

        let mut child = std::process::Command::new("git")
            .arg("checkout")
            .arg(&self.base_sha)
            .current_dir(repo_path)
            .spawn()?;
        let exit_status = child.wait().unwrap();
        if !exit_status.success() {
            panic!("git checkout exited with failure status {exit_status}");
        }

        Ok(())
    }
}
