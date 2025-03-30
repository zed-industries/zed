use crate::headless_assistant::{HeadlessAppState, HeadlessAssistant};
use anyhow::anyhow;
use assistant2::RequestKind;
use collections::HashMap;
use gpui::{App, Task};
use language_model::{LanguageModel, TokenUsage};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use util::command::new_smol_command;

pub struct Eval {
    pub name: String,
    pub path: PathBuf,
    pub repo_path: PathBuf,
    pub eval_setup: EvalSetup,
    pub user_prompt: String,
}

#[derive(Debug, Serialize)]
pub struct EvalOutput {
    pub diff: String,
    pub last_message: String,
    pub elapsed_time: Duration,
    pub assistant_response_count: usize,
    pub tool_use_counts: HashMap<Arc<str>, u32>,
    pub token_usage: TokenUsage,
}

#[derive(Deserialize)]
pub struct EvalSetup {
    pub url: String,
    pub base_sha: String,
}

impl Eval {
    /// Loads the eval from a path (typically in `evaluation_data`). Clones and checks out the repo
    /// if necessary.
    pub async fn load(name: String, path: PathBuf, repos_dir: &Path) -> anyhow::Result<Self> {
        let prompt_path = path.join("prompt.txt");
        let user_prompt = smol::unblock(|| std::fs::read_to_string(prompt_path)).await?;
        let setup_path = path.join("setup.json");
        let setup_contents = smol::unblock(|| std::fs::read_to_string(setup_path)).await?;
        let eval_setup = serde_json_lenient::from_str_lenient::<EvalSetup>(&setup_contents)?;
        let repo_path = repos_dir.join(repo_dir_name(&eval_setup.url));
        Ok(Eval {
            name,
            path,
            repo_path,
            eval_setup,
            user_prompt,
        })
    }

    pub fn run(
        self,
        app_state: Arc<HeadlessAppState>,
        model: Arc<dyn LanguageModel>,
        cx: &mut App,
    ) -> Task<anyhow::Result<EvalOutput>> {
        cx.spawn(async move |cx| {
            checkout_repo(&self.eval_setup, &self.repo_path).await?;

            let (assistant, done_rx) =
                cx.update(|cx| HeadlessAssistant::new(app_state.clone(), cx))??;

            let _worktree = assistant
                .update(cx, |assistant, cx| {
                    assistant.project.update(cx, |project, cx| {
                        project.create_worktree(&self.repo_path, true, cx)
                    })
                })?
                .await?;

            let start_time = std::time::SystemTime::now();

            let (system_prompt_context, load_error) = cx
                .update(|cx| {
                    assistant
                        .read(cx)
                        .thread
                        .read(cx)
                        .load_system_prompt_context(cx)
                })?
                .await;

            if let Some(load_error) = load_error {
                return Err(anyhow!("{:?}", load_error));
            };

            assistant.update(cx, |assistant, cx| {
                assistant.thread.update(cx, |thread, cx| {
                    let context = vec![];
                    thread.insert_user_message(self.user_prompt.clone(), context, None, cx);
                    thread.set_system_prompt_context(system_prompt_context);
                    thread.send_to_model(model, RequestKind::Chat, cx);
                });
            })?;

            done_rx.recv().await??;

            let elapsed_time = start_time.elapsed()?;

            let diff = query_git(&self.repo_path, vec!["diff"]).await?;

            assistant.update(cx, |assistant, cx| {
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
                    last_message: last_message.to_string(),
                    elapsed_time,
                    assistant_response_count,
                    tool_use_counts: assistant.tool_use_counts.clone(),
                    token_usage: thread.cumulative_token_usage(),
                })
            })?
        })
    }
}

impl EvalOutput {
    // Method to save the output to a directory
    pub fn save_to_directory(
        &self,
        output_dir: &Path,
        eval_output_value: String,
    ) -> anyhow::Result<()> {
        // Create the output directory if it doesn't exist
        fs::create_dir_all(&output_dir)?;

        // Save the diff to a file
        let diff_path = output_dir.join("diff.patch");
        let mut diff_file = fs::File::create(&diff_path)?;
        diff_file.write_all(self.diff.as_bytes())?;

        // Save the last message to a file
        let message_path = output_dir.join("assistant_response.txt");
        let mut message_file = fs::File::create(&message_path)?;
        message_file.write_all(self.last_message.as_bytes())?;

        // Current metrics for this run
        let current_metrics = serde_json::json!({
            "elapsed_time_ms": self.elapsed_time.as_millis(),
            "assistant_response_count": self.assistant_response_count,
            "tool_use_counts": self.tool_use_counts,
            "token_usage": self.token_usage,
            "eval_output_value": eval_output_value,
        });

        // Get current timestamp in milliseconds
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis()
            .to_string();

        // Path to metrics file
        let metrics_path = output_dir.join("metrics.json");

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

        Ok(())
    }
}

fn repo_dir_name(url: &str) -> String {
    url.trim_start_matches("https://")
        .replace(|c: char| !c.is_alphanumeric(), "_")
}

async fn checkout_repo(eval_setup: &EvalSetup, repo_path: &Path) -> anyhow::Result<()> {
    if !repo_path.exists() {
        smol::unblock({
            let repo_path = repo_path.to_path_buf();
            || std::fs::create_dir_all(repo_path)
        })
        .await?;
        run_git(repo_path, vec!["init"]).await?;
        run_git(repo_path, vec!["remote", "add", "origin", &eval_setup.url]).await?;
    } else {
        let actual_origin = query_git(repo_path, vec!["remote", "get-url", "origin"]).await?;
        if actual_origin != eval_setup.url {
            return Err(anyhow!(
                "remote origin {} does not match expected origin {}",
                actual_origin,
                eval_setup.url
            ));
        }

        // TODO: consider including "-x" to remove ignored files. The downside of this is that it will
        // also remove build artifacts, and so prevent incremental reuse there.
        run_git(repo_path, vec!["clean", "--force", "-d"]).await?;
        run_git(repo_path, vec!["reset", "--hard", "HEAD"]).await?;
    }

    run_git(
        repo_path,
        vec!["fetch", "--depth", "1", "origin", &eval_setup.base_sha],
    )
    .await?;
    run_git(repo_path, vec!["checkout", &eval_setup.base_sha]).await?;

    Ok(())
}

async fn run_git(repo_path: &Path, args: Vec<&str>) -> anyhow::Result<()> {
    let exit_status = new_smol_command("git")
        .current_dir(repo_path)
        .args(args.clone())
        .status()
        .await?;
    if exit_status.success() {
        Ok(())
    } else {
        Err(anyhow!(
            "`git {}` failed with {}",
            args.join(" "),
            exit_status,
        ))
    }
}

async fn query_git(repo_path: &Path, args: Vec<&str>) -> anyhow::Result<String> {
    let output = new_smol_command("git")
        .current_dir(repo_path)
        .args(args.clone())
        .output()
        .await?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    } else {
        Err(anyhow!(
            "`git {}` failed with {}",
            args.join(" "),
            output.status
        ))
    }
}
