use crate::headless_assistant::{
    authenticate_model_provider, find_model, HeadlessAppState, HeadlessAssistant,
};
use anyhow::anyhow;
use assistant2::RequestKind;
use collections::HashMap;
use gpui::{App, Task};
use language_model::LanguageModelRegistry;
use serde::Deserialize;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

pub struct Eval {
    pub repo_path: PathBuf,
    pub system_prompt: Option<String>,
    pub user_query: String,
    pub model_name: String,
    pub editor_model_name: String,
}

#[derive(Debug)]
pub struct EvalOutput {
    pub diff: String,
    pub last_message: String,
    pub elapsed_time: Duration,
    pub assistant_response_count: usize,
    pub tool_use_counts: HashMap<Arc<str>, u32>,
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
        system_prompt: Option<String>,
        model_name: String,
        editor_model_name: String,
    ) -> anyhow::Result<Self> {
        let user_query = std::fs::read_to_string(eval_path.join("prompt.txt"))?;
        let setup_contents = std::fs::read_to_string(eval_path.join("setup.json"))?;
        let setup = serde_json_lenient::from_str_lenient::<EvalSetup>(&setup_contents)?;

        checkout_repo(&setup, repo_path)?;

        Ok(Eval {
            repo_path: repo_path.to_path_buf(),
            system_prompt,
            user_query,
            model_name,
            editor_model_name,
        })
    }

    /// Runs the eval. Note that this cannot be run concurrently because
    /// LanguageModelRegistry.active_model is global state.
    pub fn run(
        &self,
        app_state: Arc<HeadlessAppState>,
        cx: &mut App,
    ) -> Task<anyhow::Result<EvalOutput>> {
        let model = match find_model(&self.model_name, cx) {
            Ok(model) => model,
            Err(err) => return Task::ready(Err(err)),
        };

        let editor_model = match find_model(&self.editor_model_name, cx) {
            Ok(editor_model) => editor_model,
            Err(err) => return Task::ready(Err(err)),
        };

        LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
            registry.set_active_model(Some(model.clone()), cx);
            registry.set_editor_model(Some(editor_model.clone()), cx);
        });

        let model_provider_id = model.provider_id();
        let editor_model_provider_id = editor_model.provider_id();
        let repo_path = self.repo_path.clone();
        let system_prompt = self.system_prompt.clone();
        let user_query = self.user_query.clone();

        cx.spawn(move |mut cx| async move {
            cx.update(|cx| authenticate_model_provider(model_provider_id.clone(), cx))?
                .await?;

            cx.update(|cx| authenticate_model_provider(editor_model_provider_id.clone(), cx))?
                .await?;

            let (assistant, done_rx) =
                cx.update(|cx| HeadlessAssistant::new(app_state.clone(), cx))??;

            let _worktree = assistant
                .update(&mut cx, |assistant, cx| {
                    assistant.project.update(cx, |project, cx| {
                        project.create_worktree(&repo_path, true, cx)
                    })
                })?
                .await?;

            let start_time = std::time::SystemTime::now();

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
                    thread.send_to_model(model, RequestKind::Chat, cx);
                });
            })?;

            done_rx.recv().await??;

            let elapsed_time = start_time.elapsed()?;

            let diff = repo_diff(&repo_path)?;

            assistant.update(&mut cx, |assistant, cx| {
                let thread = assistant.thread.read(cx);
                let last_message = thread.messages().last().unwrap();
                if last_message.role != language_model::Role::Assistant {
                    return Err(anyhow!("Last message is not from assistant"));
                }
                let assistant_response_count = thread
                    .messages()
                    .filter(|message| message.role == language_model::Role::Assistant)
                    .count();
                Ok(EvalOutput {
                    diff,
                    last_message: last_message.text.clone(),
                    elapsed_time,
                    assistant_response_count,
                    tool_use_counts: assistant.tool_use_counts.clone(),
                })
            })?
        })
    }
}

fn checkout_repo(eval_setup: &EvalSetup, repo_path: &Path) -> anyhow::Result<()> {
    if !repo_path.exists() {
        if let Some(parent) = repo_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut child = std::process::Command::new("git")
            .arg("clone")
            .arg(&eval_setup.url)
            .arg(repo_path)
            .spawn()?;
        let exit_status = child.wait()?;
        if !exit_status.success() {
            return Err(anyhow!(
                "git clone exited with failure status {exit_status}"
            ));
        }
    }

    let output = std::process::Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .current_dir(repo_path)
        .output()?;
    if !output.status.success() {
        return Err(anyhow!(
            "`git remote get-url origin` exited with failure status {}",
            output.status
        ));
    }
    let stdout = String::from_utf8(output.stdout)?;
    let actual_origin = stdout.trim();
    if actual_origin != eval_setup.url {
        return Err(anyhow!(
            "remote origin {} does not match expected origin {}",
            actual_origin,
            eval_setup.url
        ));
    }

    let mut child = std::process::Command::new("git")
        .arg("reset")
        .arg("--hard")
        .arg("HEAD")
        .current_dir(repo_path)
        .spawn()?;
    let exit_status = child.wait()?;
    if !exit_status.success() {
        return Err(anyhow!(
            "`git reset --hard` exited with failure status {exit_status}"
        ));
    }

    let mut child = std::process::Command::new("git")
        .arg("checkout")
        .arg(&eval_setup.base_sha)
        .current_dir(repo_path)
        .spawn()?;
    let exit_status = child.wait()?;
    if !exit_status.success() {
        return Err(anyhow!(
            "`git checkout` exited with failure status {exit_status}"
        ));
    }

    Ok(())
}

fn repo_diff(repo_path: &Path) -> anyhow::Result<String> {
    // Run git diff in repo_path
    let output = std::process::Command::new("git")
        .arg("diff")
        .current_dir(repo_path)
        .output()?;
    if !output.status.success() {
        return Err(anyhow!(
            "`git diff` exited with failure status {}",
            output.status
        ));
    }
    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout)
}
