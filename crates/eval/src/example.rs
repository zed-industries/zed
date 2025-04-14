use agent::{RequestKind, ThreadEvent, ThreadStore};
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::ToolWorkingSet;
use client::proto::LspWorkProgress;
use collections::HashMap;
use dap::DapRegistry;
use futures::channel::{mpsc, oneshot};
use futures::{FutureExt, StreamExt as _};
use gpui::{App, AsyncApp, Entity, Task};
use handlebars::Handlebars;
use language::{DiagnosticSeverity, OffsetRangeExt};
use language_model::{
    LanguageModel, LanguageModelRequest, LanguageModelRequestMessage, MessageContent, Role,
    StopReason, TokenUsage,
};
use project::{LspStore, Project, ProjectPath};
use serde::{Deserialize, Serialize};
use std::fmt::Write as _;
use std::fs::File;
use std::io::Write as _;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{
    fs,
    path::{Path, PathBuf},
};
use unindent::Unindent as _;
use util::ResultExt as _;
use util::command::new_smol_command;
use util::serde::default_true;

use crate::AgentAppState;

pub const EXAMPLES_DIR: &str = "./crates/eval/examples";
pub const REPOS_DIR: &str = "./crates/eval/repos";
pub const WORKTREES_DIR: &str = "./crates/eval/worktrees";

#[derive(Clone, Debug, Deserialize)]
pub struct ExampleBase {
    pub url: String,
    pub revision: String,
    pub language_extension: Option<String>,
    pub insert_id: Option<String>,
    #[serde(default = "default_true")]
    pub require_lsp: bool,
}

#[derive(Clone, Debug)]
pub struct Example {
    pub name: String,
    /// Content of `base.toml`
    pub base: ExampleBase,
    /// Content of `prompt.md`
    pub prompt: String,
    /// Content of `criteria.md`
    pub criteria: String,
    /// Markdown log file to append to
    pub log_file: Arc<Mutex<File>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RunOutput {
    pub repository_diff: String,
    pub diagnostics: String,
    pub response_count: usize,
    pub token_usage: TokenUsage,
    pub tool_use_counts: HashMap<Arc<str>, u32>,
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
    pub fn load_from_directory(dir_path: &Path, run_dir: &Path) -> Result<Self> {
        let name = dir_path.file_name().unwrap().to_string_lossy().to_string();
        let base_path = dir_path.join("base.toml");
        let prompt_path = dir_path.join("prompt.md");
        let criteria_path = dir_path.join("criteria.md");

        let log_file_path = run_dir.join(format!(
            "{}.md",
            dir_path.file_name().unwrap().to_str().unwrap()
        ));
        let log_file = Arc::new(Mutex::new(File::create(&log_file_path).unwrap()));
        println!("{}> Logging to {:?}", name, log_file_path);

        Ok(Example {
            name,
            base: toml::from_str(&fs::read_to_string(&base_path)?)?,
            prompt: fs::read_to_string(prompt_path.clone())?,
            criteria: fs::read_to_string(criteria_path.clone())?,
            log_file,
        })
    }

    pub fn worktree_path(&self) -> PathBuf {
        Path::new(WORKTREES_DIR)
            .canonicalize()
            .context(format!("No such directory {WORKTREES_DIR}"))
            .unwrap()
            .join(&self.name)
    }

    /// Set up the example by checking out the specified Git revision
    pub async fn setup(&self) -> Result<()> {
        let repo_path = repo_path_for_url(&self.base.url);

        println!("{}> Fetching", self.name);

        run_git(
            &repo_path,
            &["fetch", "--depth", "1", "origin", &self.base.revision],
        )
        .await?;

        let worktree_path = self.worktree_path();

        if worktree_path.is_dir() {
            println!("{}> Resetting existing worktree", self.name);

            // TODO: consider including "-x" to remove ignored files. The downside of this is that
            // it will also remove build artifacts, and so prevent incremental reuse there.
            run_git(&worktree_path, &["clean", "--force", "-d"]).await?;
            run_git(&worktree_path, &["reset", "--hard", "HEAD"]).await?;
            run_git(&worktree_path, &["checkout", &self.base.revision]).await?;
        } else {
            println!("{}> Creating worktree", self.name);

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

        cx.spawn(async move |cx| {
            let worktree = worktree.await?;

            // Wait for worktree scan to finish before choosing a file to open.
            worktree
                .update(cx, |worktree, _cx| {
                    worktree.as_local().unwrap().scan_complete()
                })?
                .await;

            let lsp_open_handle_and_store = if this.base.require_lsp {
                let language_extension = this.base.language_extension.as_deref().context(
                    "language_extension field is required in base.toml when `require_lsp == true`",
                )?;

                // Open a file that matches the language to cause LSP to start.
                let language_file = worktree.read_with(cx, |worktree, _cx| {
                    worktree
                        .files(false, 0)
                        .find_map(|e| {
                            if e.path.clone().extension().and_then(|ext| ext.to_str())
                                == Some(language_extension)
                            {
                                Some(ProjectPath {
                                    worktree_id: worktree.id(),
                                    path: e.path.clone(),
                                })
                            } else {
                                None
                            }
                        })
                        .context("Failed to find a file for example language")
                })??;

                let open_language_file_buffer_task = project.update(cx, |project, cx| {
                    project.open_buffer(language_file.clone(), cx)
                })?;

                let language_file_buffer = open_language_file_buffer_task.await?;

                let (lsp_open_handle, lsp_store) = project.update(cx, |project, cx| {
                    (
                        project.register_buffer_with_language_servers(&language_file_buffer, cx),
                        project.lsp_store().clone(),
                    )
                })?;

                // TODO: remove this once the diagnostics tool waits for new diagnostics
                cx.background_executor().timer(Duration::new(5, 0)).await;
                wait_for_lang_server(&lsp_store, this.name.clone(), cx).await?;

                lsp_store.update(cx, |lsp_store, cx| {
                    lsp_open_handle.update(cx, |buffer, cx| {
                        buffer.update(cx, |buffer, cx| {
                            let has_language_server = lsp_store
                                .language_servers_for_local_buffer(buffer, cx)
                                .next()
                                .is_some();
                            if has_language_server {
                                Ok(())
                            } else {
                                Err(anyhow!(
                                    "`{:?}` was opened to cause the language server to start, \
                                    but no language servers are registered for its buffer. \
                                    Set `require_lsp = false` in `base.toml` to skip this.",
                                    language_file
                                ))
                            }
                        })
                    })
                })??;

                Some((lsp_open_handle, lsp_store))
            } else {
                None
            };

            if std::env::var("ZED_EVAL_SETUP_ONLY").is_ok() {
                return Err(anyhow!("Setup only mode"));
            }

            let thread_store = thread_store.await;
            let thread =
                thread_store.update(cx, |thread_store, cx| thread_store.create_thread(cx))?;

            {
                let mut log_file = this.log_file.lock().unwrap();
                writeln!(&mut log_file, "👤 USER:").log_err();
                writeln!(&mut log_file, "{}", this.prompt).log_err();
                writeln!(&mut log_file, "🤖 ASSISTANT:").log_err();
                log_file.flush().log_err();
            }

            let tool_use_counts: Arc<Mutex<HashMap<Arc<str>, u32>>> =
                Mutex::new(HashMap::default()).into();

            let (tx, rx) = oneshot::channel();
            let mut tx = Some(tx);

            let subscription = cx.subscribe(&thread, {
                let log_file = this.log_file.clone();
                let name = this.name.clone();
                let tool_use_counts = tool_use_counts.clone();
                move |thread, event: &ThreadEvent, cx| {
                    let mut log_file = log_file.lock().unwrap();

                    match event {
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
                            write!(&mut log_file, "{}", chunk).log_err();
                        }
                        ThreadEvent::StreamedAssistantThinking(_, chunk) => {
                            write!(&mut log_file, "{}", chunk).log_err();
                        }
                        ThreadEvent::UsePendingTools { tool_uses } => {
                            writeln!(&mut log_file, "\n\nUSING TOOLS:").log_err();
                            for tool_use in tool_uses {
                                writeln!(&mut log_file, "{}: {}", tool_use.name, tool_use.input)
                                    .log_err();
                            }
                        }
                        ThreadEvent::ToolFinished {
                            tool_use_id,
                            pending_tool_use,
                            ..
                        } => {
                            if let Some(tool_use) = pending_tool_use {
                                let message = format!("TOOL FINISHED: {}", tool_use.name);
                                println!("{name}> {message}");
                                writeln!(&mut log_file, "\n{}", message).log_err();
                            }
                            if let Some(tool_result) = thread.read(cx).tool_result(tool_use_id) {
                                writeln!(&mut log_file, "\n{}\n", tool_result.content).log_err();
                                let mut tool_use_counts = tool_use_counts.lock().unwrap();
                                *tool_use_counts
                                    .entry(tool_result.tool_name.clone())
                                    .or_insert(0) += 1;
                            }
                        }
                        _ => {}
                    }

                    log_file.flush().log_err();
                }
            })?;

            thread.update(cx, |thread, cx| {
                let context = vec![];
                thread.insert_user_message(this.prompt.clone(), context, None, cx);
                thread.send_to_model(model, RequestKind::Chat, cx);
            })?;

            rx.await??;

            if let Some((_, lsp_store)) = lsp_open_handle_and_store.as_ref() {
                wait_for_lang_server(lsp_store, this.name.clone(), cx).await?;
            }

            let repository_diff = this.repository_diff().await?;
            let diagnostics = cx
                .update(move |cx| {
                    cx.spawn(async move |cx| query_lsp_diagnostics(project, cx).await)
                })?
                .await?;

            drop(subscription);
            drop(lsp_open_handle_and_store);

            thread.update(cx, |thread, _cx| {
                let response_count = thread
                    .messages()
                    .filter(|message| message.role == language_model::Role::Assistant)
                    .count();
                RunOutput {
                    repository_diff,
                    diagnostics,
                    response_count,
                    token_usage: thread.cumulative_token_usage(),
                    tool_use_counts: tool_use_counts.lock().unwrap().clone(),
                }
            })
        })
    }

    pub async fn judge(
        &mut self,
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

        let mut log_file = self.log_file.lock().unwrap();

        writeln!(&mut log_file, "\n\n").log_err();
        writeln!(&mut log_file, "========================================").log_err();
        writeln!(&mut log_file, "              JUDGE OUTPUT              ").log_err();
        writeln!(&mut log_file, "========================================").log_err();
        writeln!(&mut log_file, "\n{}", &response).log_err();

        parse_judge_output(&response)
    }

    pub async fn repository_diff(&self) -> Result<String> {
        let worktree_path = self.worktree_path();
        run_git(&worktree_path, &["add", "-N"]).await?;
        run_git(&worktree_path, &["diff"]).await
    }
}

fn wait_for_lang_server(
    lsp_store: &Entity<LspStore>,
    name: String,
    cx: &mut AsyncApp,
) -> Task<Result<()>> {
    if cx
        .update(|cx| !has_pending_lang_server_work(lsp_store, cx))
        .unwrap()
        || std::env::var("ZED_EVAL_SKIP_LS_WAIT").is_ok()
    {
        return Task::ready(anyhow::Ok(()));
    }

    println!("{}> ⏵ Waiting for language server", name);

    let (mut tx, mut rx) = mpsc::channel(1);

    let subscription =
        cx.subscribe(&lsp_store, {
            let name = name.clone();
            move |lsp_store, event, cx| {
                match event {
                    project::LspStoreEvent::LanguageServerUpdate {
                        message:
                            client::proto::update_language_server::Variant::WorkProgress(
                                LspWorkProgress {
                                    message: Some(message),
                                    ..
                                },
                            ),
                        ..
                    } => println!("{name}> ⟲ {message}"),
                    _ => {}
                }

                if !has_pending_lang_server_work(&lsp_store, cx) {
                    tx.try_send(()).ok();
                }
            }
        });

    cx.spawn(async move |cx| {
        let timeout = cx.background_executor().timer(Duration::new(60 * 5, 0));
        let result = futures::select! {
            _ = rx.next() => {
                println!("{}> ⚑ Language server idle", name);
                anyhow::Ok(())
            },
            _ = timeout.fuse() => {
                Err(anyhow!("LSP wait timed out after 5 minutes"))
            }
        };
        drop(subscription);
        result
    })
}

fn has_pending_lang_server_work(lsp_store: &Entity<LspStore>, cx: &App) -> bool {
    lsp_store
        .read(cx)
        .language_server_statuses()
        .any(|(_, status)| !status.pending_work.is_empty())
}

async fn query_lsp_diagnostics(project: Entity<Project>, cx: &mut AsyncApp) -> Result<String> {
    let paths_with_diagnostics = project.update(cx, |project, cx| {
        project
            .diagnostic_summaries(true, cx)
            .filter(|(_, _, summary)| summary.error_count > 0 || summary.warning_count > 0)
            .map(|(project_path, _, _)| project_path)
            .collect::<Vec<_>>()
    })?;

    let mut output = String::new();
    for project_path in paths_with_diagnostics {
        let buffer = project
            .update(cx, |project, cx| project.open_buffer(project_path, cx))?
            .await?;
        let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;

        for (_, group) in snapshot.diagnostic_groups(None) {
            let entry = &group.entries[group.primary_ix];
            let range = entry.range.to_point(&snapshot);
            let severity = match entry.diagnostic.severity {
                DiagnosticSeverity::ERROR => "error",
                DiagnosticSeverity::WARNING => "warning",
                _ => continue,
            };

            writeln!(
                output,
                "{} at line {}: {}",
                severity,
                range.start.row + 1,
                entry.diagnostic.message
            )?;
        }
    }
    anyhow::Ok(output)
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

#[cfg(test)]
mod test {
    use super::*;

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
}
