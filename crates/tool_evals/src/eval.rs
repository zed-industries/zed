use crate::headless_assistant::{HeadlessAppState, HeadlessAssistant};
use anyhow::anyhow;
use assistant2::RequestKind;
use collections::HashMap;
use gpui::{App, Task};
use language_model::LanguageModel;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

pub struct Eval {
    pub repo_path: PathBuf,
    pub system_prompt: Option<String>,
    pub user_query: String,
}

#[derive(Debug, Serialize)]
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
    ) -> anyhow::Result<Self> {
        let user_query = std::fs::read_to_string(eval_path.join("prompt.txt"))?;
        let setup_contents = std::fs::read_to_string(eval_path.join("setup.json"))?;
        let setup = serde_json_lenient::from_str_lenient::<EvalSetup>(&setup_contents)?;

        checkout_repo(&setup, repo_path)?;

        Ok(Eval {
            repo_path: repo_path.to_path_buf(),
            system_prompt,
            user_query,
        })
    }

    pub fn run(
        &self,
        app_state: Arc<HeadlessAppState>,
        model: Arc<dyn LanguageModel>,
        cx: &mut App,
    ) -> Task<anyhow::Result<EvalOutput>> {
        let repo_path = self.repo_path.clone();
        let system_prompt = self.system_prompt.clone();
        let user_query = self.user_query.clone();

        cx.spawn(move |mut cx| async move {
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

impl EvalOutput {
    // Method to save the output to a directory
    pub fn save_to_directory(
        &self,
        eval_name: &str,
        output_dir: &Path,
        eval_output_value: String,
    ) -> anyhow::Result<PathBuf> {
        // Create the output directory if it doesn't exist
        let eval_output_dir = output_dir.join(eval_name);
        fs::create_dir_all(&eval_output_dir)?;

        // Save the diff to a file
        let diff_path = eval_output_dir.join("diff.patch");
        let mut diff_file = fs::File::create(&diff_path)?;
        diff_file.write_all(self.diff.as_bytes())?;

        // Save the last message to a file
        let message_path = eval_output_dir.join("assistant_response.txt");
        let mut message_file = fs::File::create(&message_path)?;
        message_file.write_all(self.last_message.as_bytes())?;

        // Current metrics for this run
        let current_metrics = serde_json::json!({
            "elapsed_time_ms": self.elapsed_time.as_millis(),
            "assistant_response_count": self.assistant_response_count,
            "tool_use_counts": self.tool_use_counts,
            "eval_output_value": eval_output_value,
        });

        // Get current timestamp in milliseconds
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis()
            .to_string();

        // Path to metrics file
        let metrics_path = eval_output_dir.join("metrics.json");

        // Load existing metrics if the file exists, or create a new object
        let mut historical_metrics = if metrics_path.exists() {
            let metrics_content = fs::read_to_string(&metrics_path)?;
            serde_json::from_str::<serde_json::Value>(&metrics_content)
                .unwrap_or_else(|_| serde_json::json!({}))
        } else {
            serde_json::json!({})
        };

        // Add new run with timestamp as key
        if let serde_json::Value::Object(ref mut map) = historical_metrics {
            map.insert(timestamp, current_metrics);
        }

        // Write updated metrics back to file
        let metrics_json = serde_json::to_string_pretty(&historical_metrics)?;
        let mut metrics_file = fs::File::create(&metrics_path)?;
        metrics_file.write_all(metrics_json.as_bytes())?;

        Ok(eval_output_dir)
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
