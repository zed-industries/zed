use agent::{RequestKind, ThreadEvent, ThreadStore};
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::ToolWorkingSet;
use dap::DapRegistry;
use futures::StreamExt as _;
use futures::channel::oneshot;
use gpui::{App, AsyncApp, Task};
use handlebars::Handlebars;
use language_model::{
    LanguageModel, LanguageModelRequest, LanguageModelRequestMessage, MessageContent, Role,
    StopReason, TokenUsage,
};
use project::Project;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{
    fs,
    path::{Path, PathBuf},
};
use unindent::Unindent as _;
use util::command::new_smol_command;

use crate::AgentAppState;

pub const EXAMPLES_DIR: &str = "./crates/eval/examples";
pub const REPOS_DIR: &str = "./crates/eval/repos";
pub const WORKTREES_DIR: &str = "./crates/eval/worktrees";

#[derive(Clone, Debug, Deserialize)]
pub struct ExampleBase {
    pub url: String,
    pub revision: String,
    pub language: String,
    pub insert_id: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Example {
    pub path: PathBuf,

    pub base: ExampleBase,

    /// Content of the prompt.md file
    pub prompt: String,

    /// Content of the criteria.md file
    pub criteria: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RunOutput {
    pub repository_diff: String,
    pub response_count: usize,
    pub token_usage: TokenUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgeInput {
    pub repository_diff: String,
    pub criteria: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgeOutput {
    pub analysis: String,
    pub score: u32,
}

impl Example {
    /// Load an example from a directory containing base.toml, prompt.md, and criteria.md
    pub fn load_from_directory(dir_path: &Path) -> Result<Self> {
        let base_path = dir_path.join("base.toml");
        let prompt_path = dir_path.join("prompt.md");
        let criteria_path = dir_path.join("criteria.md");

        Ok(Example {
            path: dir_path.to_path_buf(),
            base: toml::from_str(&fs::read_to_string(&base_path)?)?,
            prompt: fs::read_to_string(prompt_path.clone())?,
            criteria: fs::read_to_string(criteria_path.clone())?,
        })
    }

    pub fn name(&self) -> String {
        self.path.file_name().unwrap().to_string_lossy().to_string()
    }

    pub fn worktree_path(&self) -> PathBuf {
        Path::new(WORKTREES_DIR)
            .canonicalize()
            .context(format!("No such directory {WORKTREES_DIR}"))
            .unwrap()
            .join(self.name())
    }

    /// Set up the example by checking out the specified Git revision
    pub async fn setup(&self) -> Result<()> {
        let repo_path = repo_path_for_url(&self.base.url);

        run_git(
            &repo_path,
            &["fetch", "--depth", "1", "origin", &self.base.revision],
        )
        .await?;

        let worktree_path = self.worktree_path();

        if worktree_path.is_dir() {
            // TODO: consider including "-x" to remove ignored files. The downside of this is that
            // it will also remove build artifacts, and so prevent incremental reuse there.
            run_git(&worktree_path, &["clean", "--force", "-d"]).await?;
            run_git(&worktree_path, &["reset", "--hard", "HEAD"]).await?;
            run_git(&worktree_path, &["checkout", &self.base.revision]).await?;
        } else {
            let worktree_path_string = worktree_path.to_string_lossy().to_string();

            run_git(
                &repo_path,
                &[
                    "worktree",
                    "add",
                    "-f",
                    &worktree_path_string,
                    &self.base.revision,
                ],
            )
            .await?;
        }

        Ok(())
    }

    pub fn run(
        &self,
        model: Arc<dyn LanguageModel>,
        app_state: Arc<AgentAppState>,
        cx: &mut App,
    ) -> Task<Result<RunOutput>> {
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

        let worktree_path = self.worktree_path();
        let worktree = project.update(cx, |project, cx| {
            project.create_worktree(&worktree_path, true, cx)
        });

        let tools = Arc::new(ToolWorkingSet::default());
        let thread_store =
            ThreadStore::load(project.clone(), tools, app_state.prompt_builder.clone(), cx);
        let this = self.clone();

        println!("USER:");
        println!("{}", self.prompt);
        println!("ASSISTANT:");
        cx.spawn(async move |cx| {
            worktree.await?;
            let thread_store = thread_store.await;
            let thread =
                thread_store.update(cx, |thread_store, cx| thread_store.create_thread(cx))?;

            let (tx, rx) = oneshot::channel();
            let mut tx = Some(tx);

            let _subscription =
                cx.subscribe(
                    &thread,
                    move |thread, event: &ThreadEvent, cx| match event {
                        ThreadEvent::Stopped(reason) => match reason {
                            Ok(StopReason::EndTurn) => {
                                if let Some(tx) = tx.take() {
                                    tx.send(Ok(())).ok();
                                }
                            }
                            Ok(StopReason::MaxTokens) => {
                                if let Some(tx) = tx.take() {
                                    tx.send(Err(anyhow!("Exceeded maximum tokens"))).ok();
                                }
                            }
                            Ok(StopReason::ToolUse) => {}
                            Err(error) => {
                                if let Some(tx) = tx.take() {
                                    tx.send(Err(anyhow!(error.clone()))).ok();
                                }
                            }
                        },
                        ThreadEvent::ShowError(thread_error) => {
                            if let Some(tx) = tx.take() {
                                tx.send(Err(anyhow!(thread_error.clone()))).ok();
                            }
                        }
                        ThreadEvent::StreamedAssistantText(_, chunk) => {
                            print!("{}", chunk);
                        }
                        ThreadEvent::StreamedAssistantThinking(_, chunk) => {
                            print!("{}", chunk);
                        }
                        ThreadEvent::UsePendingTools { tool_uses } => {
                            println!("\n\nUSING TOOLS:");
                            for tool_use in tool_uses {
                                println!("{}: {}", tool_use.name, tool_use.input);
                            }
                        }
                        ThreadEvent::ToolFinished {
                            tool_use_id,
                            pending_tool_use,
                            ..
                        } => {
                            if let Some(tool_use) = pending_tool_use {
                                println!("\nTOOL FINISHED: {}", tool_use.name);
                            }
                            if let Some(tool_result) = thread.read(cx).tool_result(tool_use_id) {
                                println!("\n{}\n", tool_result.content);
                            }
                        }
                        _ => {}
                    },
                )?;

            thread.update(cx, |thread, cx| {
                let context = vec![];
                thread.insert_user_message(this.prompt.clone(), context, None, cx);
                thread.send_to_model(model, RequestKind::Chat, cx);
            })?;

            rx.await??;

            let repository_diff = this.repository_diff().await?;

            thread.update(cx, |thread, _cx| {
                let response_count = thread
                    .messages()
                    .filter(|message| message.role == language_model::Role::Assistant)
                    .count();
                RunOutput {
                    repository_diff,
                    response_count,
                    token_usage: thread.cumulative_token_usage(),
                }
            })
        })
    }

    pub async fn judge(
        &self,
        model: Arc<dyn LanguageModel>,
        repository_diff: String,
        cx: &AsyncApp,
    ) -> Result<JudgeOutput> {
        let judge_prompt = include_str!("judge_prompt.hbs");
        let judge_prompt_name = "judge_prompt";
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string(judge_prompt_name, judge_prompt)?;
        let prompt = handlebars.render(
            judge_prompt_name,
            &JudgeInput {
                repository_diff,
                criteria: self.criteria.clone(),
            },
        )?;

        let request = LanguageModelRequest {
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text(prompt)],
                cache: false,
            }],
            temperature: None,
            tools: Vec::new(),
            stop: Vec::new(),
        };

        let response = send_language_model_request(model, request, cx).await?;

        parse_judge_output(&response)
    }

    pub async fn repository_diff(&self) -> Result<String> {
        let worktree_path = self.worktree_path();
        run_git(&worktree_path, &["add", "-N"]).await?;
        run_git(&worktree_path, &["diff"]).await
    }
}

fn parse_judge_output(response: &str) -> Result<JudgeOutput> {
    let analysis = get_tag("analysis", response)?.to_string();
    let score = get_tag("score", response)?
        .parse()
        .context("error parsing score")?;

    Ok(JudgeOutput { analysis, score })
}

fn get_tag(name: &'static str, response: &str) -> Result<String> {
    let start_tag = format!("<{}>", name);
    let end_tag = format!("</{}>", name);

    let start_ix = response
        .find(&start_tag)
        .context(format!("{} start tag not found", name))?;
    let content_start_ix = start_ix + start_tag.len();

    let end_ix = content_start_ix
        + response[content_start_ix..]
            .find(&end_tag)
            .context(format!("{} end tag not found", name))?;

    let content = response[content_start_ix..end_ix].trim().unindent();

    anyhow::Ok(content)
}

pub fn repo_path_for_url(repo_url: &str) -> PathBuf {
    let repo_name = repo_url
        .trim_start_matches("https://")
        .replace(|c: char| !c.is_alphanumeric(), "-");
    Path::new(REPOS_DIR)
        .canonicalize()
        .context(format!("No such directory {REPOS_DIR}"))
        .unwrap()
        .join(repo_name)
}

#[cfg(test)]
#[test]
fn test_parse_judge_output() {
    let response = r#"
        <analysis>The model did a good job but there were still compilations errors.</analysis>
        <score>3</score>
    "#
    .unindent();

    let output = parse_judge_output(&response).unwrap();
    assert_eq!(
        output.analysis,
        "The model did a good job but there were still compilations errors."
    );
    assert_eq!(output.score, 3);

    let response = r#"
        Text around ignored

        <analysis>
            Failed to compile:
            - Error 1
            - Error 2
        </analysis>

        <score>1</score>
    "#
    .unindent();

    let output = parse_judge_output(&response).unwrap();
    assert_eq!(output.analysis, "Failed to compile:\n- Error 1\n- Error 2");
    assert_eq!(output.score, 1);
}

pub async fn run_git(repo_path: &Path, args: &[&str]) -> Result<String> {
    let output = new_smol_command("git")
        .current_dir(repo_path)
        .args(args)
        .output()
        .await?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    } else {
        Err(anyhow!(
            "`git {}` within `{}` failed with status: {}\nstderr:\n{}\nstdout:\n{}",
            args.join(" "),
            repo_path.display(),
            output.status,
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout),
        ))
    }
}

pub async fn send_language_model_request(
    model: Arc<dyn LanguageModel>,
    request: LanguageModelRequest,
    cx: &AsyncApp,
) -> anyhow::Result<String> {
    match model.stream_completion_text(request, &cx).await {
        Ok(mut stream) => {
            let mut full_response = String::new();
            while let Some(chunk_result) = stream.stream.next().await {
                match chunk_result {
                    Ok(chunk_str) => {
                        print!("{}", &chunk_str);
                        full_response.push_str(&chunk_str);
                    }
                    Err(err) => {
                        return Err(anyhow!(
                            "Error receiving response from language model: {err}"
                        ));
                    }
                }
            }
            Ok(full_response)
        }
        Err(err) => Err(anyhow!(
            "Failed to get response from language model. Error was: {err}"
        )),
    }
}
