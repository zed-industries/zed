use agent::{ThreadEvent, ThreadStore};
use anyhow::{Context, Result, anyhow};
use assistant_tool::ToolWorkingSet;
use client::proto::LspWorkProgress;
use futures::channel::mpsc;
use futures::{FutureExt as _, StreamExt as _, select_biased};
use gpui::{App, AppContext as _, AsyncApp, Entity, Task};
use handlebars::Handlebars;
use language::{Buffer, DiagnosticSeverity, OffsetRangeExt as _};
use language_model::{
    LanguageModel, LanguageModelCompletionEvent, LanguageModelRequest, LanguageModelRequestMessage,
    MessageContent, Role, StopReason, TokenUsage,
};
use project::{Project, ProjectPath};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fmt::Write as _;
use std::fs;
use std::fs::File;
use std::io::Write as _;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use unindent::Unindent as _;
use util::ResultExt as _;
use util::command::new_smol_command;
use util::markdown::MarkdownString;

use crate::thread::{Assertions, EvalThread, FailedAssertion, ThreadContext};
use crate::{AgentAppState, ToolMetrics};

pub const REPOS_DIR: &str = "./crates/eval/repos";
pub const WORKTREES_DIR: &str = "./crates/eval/worktrees";

pub const THREAD_EVENT_TIMEOUT: Duration = Duration::from_secs(60 * 2);

pub const ZED_REPO_URL: &str = "https://github.com/zed-industries/zed.git";

#[derive(Clone)]
pub struct ThreadInstance {
    pub thread: Rc<dyn EvalThread>,
    pub name: String,
    pub run_directory: PathBuf,
    pub log_prefix: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RunOutput {
    pub repository_diff: String,
    pub ran_diagnostics_check: bool,
    pub diagnostics_before: Option<String>,
    pub diagnostics_after: Option<String>,
    pub response_count: usize,
    pub token_usage: TokenUsage,
    pub tool_metrics: ToolMetrics,
    pub last_request: LanguageModelRequest,
    pub assertions: Assertions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgeResponse {
    pub analysis: String,
    pub score: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgeOutput {
    pub thread: Option<JudgeResponse>,
    pub diff: Option<JudgeResponse>,
}

#[derive(Debug, Clone, Serialize)]
pub struct JudgeDiffInput {
    pub repository_diff: String,
    pub ran_diagnostics_check: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics_after: Option<String>,
    pub criteria: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct JudgeThreadInput {
    pub messages: String,
    pub criteria: String,
}

impl ThreadInstance {
    pub fn new(thread: Rc<dyn EvalThread>, run_directory: &Path, repetition_number: u32) -> Self {
        let name = if repetition_number > 0 {
            format!("{}-{}", thread.meta().name, repetition_number)
        } else {
            thread.meta().name.to_string()
        };

        let run_directory = run_directory.join(&name).to_path_buf();

        Self {
            name,
            thread,
            log_prefix: String::new(),
            run_directory,
        }
    }

    pub fn repo_url(&self) -> String {
        self.thread.meta().url
    }

    pub fn revision(&self) -> String {
        self.thread.meta().revision
    }

    pub fn set_log_prefix_style(&mut self, color: &str, name_width: usize) {
        self.log_prefix = format!(
            "{}{:<width$}\x1b[0m | ",
            color,
            self.name,
            width = name_width
        );
    }

    pub async fn setup(&mut self) -> Result<()> {
        let meta = self.thread.meta();
        let repo_path = repo_path_for_url(&meta.url);

        let revision_exists = run_git(
            &repo_path,
            &["rev-parse", &format!("{}^{{commit}}", meta.revision)],
        )
        .await
        .is_ok();

        if !revision_exists {
            println!("{}Fetching revision {}", self.log_prefix, &meta.revision);
            run_git(
                &repo_path,
                &["fetch", "--depth", "1", "origin", &meta.revision],
            )
            .await?;
        }

        let worktree_path = self.worktree_path();

        if worktree_path.is_dir() {
            println!("{}Resetting existing worktree", self.log_prefix);

            // TODO: consider including "-x" to remove ignored files. The downside of this is that
            // it will also remove build artifacts, and so prevent incremental reuse there.
            run_git(&worktree_path, &["clean", "--force", "-d"]).await?;
            run_git(&worktree_path, &["reset", "--hard", "HEAD"]).await?;
            run_git(&worktree_path, &["checkout", &meta.revision]).await?;
        } else {
            println!("{}Creating worktree", self.log_prefix);

            let worktree_path_string = worktree_path.to_string_lossy().to_string();

            run_git(
                &repo_path,
                &[
                    "worktree",
                    "add",
                    "-f",
                    &worktree_path_string,
                    &meta.revision,
                ],
            )
            .await?;
        }

        if meta.url == ZED_REPO_URL {
            std::fs::write(worktree_path.join(".rules"), std::fs::read(".rules")?)?;
        }

        std::fs::create_dir_all(&self.run_directory)?;

        Ok(())
    }

    pub fn worktree_path(&self) -> PathBuf {
        Path::new(WORKTREES_DIR)
            .canonicalize()
            .context(format!("No such directory {WORKTREES_DIR}"))
            .unwrap()
            .join(&self.name)
            .join(self.thread.meta().repo_name())
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
            app_state.fs.clone(),
            None,
            cx,
        );

        let worktree_path = self.worktree_path();
        let worktree = project.update(cx, |project, cx| {
            project.create_worktree(&worktree_path, true, cx)
        });

        let tools = cx.new(|_| ToolWorkingSet::default());
        let thread_store =
            ThreadStore::load(project.clone(), tools, app_state.prompt_builder.clone(), cx);
        let meta = self.thread.meta();
        let this = self.clone();

        cx.spawn(async move |cx| {
            let worktree = worktree.await?;

            // Wait for worktree scan to finish before choosing a file to open.
            worktree
                .update(cx, |worktree, _cx| {
                    worktree.as_local().unwrap().scan_complete()
                })?
                .await;

            let lsp = if let Some(language_server) = &meta.language_server {
                // Open a file that matches the language to cause LSP to start.
                let language_file = worktree.read_with(cx, |worktree, _cx| {
                    worktree
                        .files(false, 0)
                        .find_map(|e| {
                            if e.path.clone().extension().and_then(|ext| ext.to_str())
                                == Some(&language_server.file_extension)
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

                let lsp_open_handle = project.update(cx, |project, cx| {
                    project.register_buffer_with_language_servers(&language_file_buffer, cx)
                })?;

                wait_for_lang_server(&project, &language_file_buffer, this.log_prefix.clone(), cx).await?;

                let diagnostics_before = query_lsp_diagnostics(project.clone(), cx).await?;
                if diagnostics_before.is_some() && language_server.allow_preexisting_diagnostics {
                    return Err(anyhow!("Example has pre-existing diagnostics. If you want to run this example regardless, set `allow_preexisting_diagnostics` to `true` in `base.toml`"));
                }

                Some((lsp_open_handle, language_file_buffer, language_server, diagnostics_before))
            } else {
                None
            };


            if std::env::var("ZED_EVAL_SETUP_ONLY").is_ok() {
                return Err(anyhow!("Setup only mode"));
            }

            let thread_store = thread_store.await?;
            let thread =
                thread_store.update(cx, |thread_store, cx| thread_store.create_thread(cx))?;
            let last_request = Rc::new(RefCell::new(None));

            thread.update(cx, |thread, _cx| {
                let mut request_count = 0;
                let example_dir_path = this.run_directory.clone();

                let last_request = Rc::clone(&last_request);
                thread.set_request_callback(move |request, response_events| {
                    *last_request.borrow_mut() = Some(request.clone());

                    request_count += 1;
                    let messages_file_path = example_dir_path.join(format!("{request_count}.messages.md"));
                    let last_messages_file_path = example_dir_path.join("last.messages.md");
                    let request_markdown = RequestMarkdown::new(request);
                    let response_events_markdown = response_events_to_markdown(response_events);

                    let messages = format!("{}\n\n{}", request_markdown.messages, response_events_markdown);
                    fs::write(messages_file_path, messages.clone()).expect("failed to write messages file");
                    fs::write(last_messages_file_path, messages).expect("failed to write last messages file");

                    if request_count == 1 {
                        let tools_file_path = example_dir_path.join("tools.md");
                        fs::write(tools_file_path, request_markdown.tools).expect("failed to write tools file");
                    }
                });
            })?;

            let tool_metrics = Arc::new(Mutex::new(ToolMetrics::default()));

            let subscription = cx.subscribe(&thread, {
                let log_prefix = this.log_prefix.clone();
                let tool_metrics = tool_metrics.clone();
                let thread = thread.clone();

                move |_thread, event: &ThreadEvent, cx| {
                match event {
                    ThreadEvent::Stopped(reason) => match reason {
                        Ok(StopReason::EndTurn) => {
                            if std::env::var("ZED_EVAL_DEBUG").is_ok() {
                                println!("{}StopReason: End turn", log_prefix);
                            }
                        }
                        Ok(StopReason::ToolUse) => {
                            if std::env::var("ZED_EVAL_DEBUG").is_ok() {
                                println!("{}StopReason: Tool use", log_prefix);
                            }
                        }
                        Ok(StopReason::MaxTokens) | Err(_) => {
                            if std::env::var("ZED_EVAL_DEBUG").is_ok() {
                                println!("{}Event: {:#?}", log_prefix, event);
                            }
                        }
                    },
                    ThreadEvent::StreamedAssistantText(_, _)| ThreadEvent::StreamedAssistantThinking(_, _) | ThreadEvent::UsePendingTools { .. } => {
                    }
                    ThreadEvent::ToolFinished {
                        tool_use_id,
                        pending_tool_use,
                        ..
                    } => {
                        thread.update(cx, |thread, _cx| {
                            if let Some(tool_use) = pending_tool_use {
                                let mut tool_metrics = tool_metrics.lock().unwrap();
                                if let Some(tool_result) = thread.tool_result(&tool_use_id) {
                                    let message = if tool_result.is_error {
                                        format!("TOOL FAILED: {}", tool_use.name)
                                    } else {
                                        format!("TOOL FINISHED: {}", tool_use.name)
                                    };
                                    println!("{log_prefix}{message}");
                                    tool_metrics.insert(tool_result.tool_name.clone(), !tool_result.is_error);
                                } else {
                                    let message = format!("TOOL FINISHED WITHOUT RESULT: {}", tool_use.name);
                                    println!("{log_prefix}{message}");
                                    tool_metrics.insert(tool_use.name.clone(), true);
                                }
                            }
                        });
                    }
                    ThreadEvent::ToolConfirmationNeeded => {
                        panic!("{}Bug: Tool confirmation should not be required in eval", log_prefix);
                    },
                    ThreadEvent::StreamedCompletion |
                    ThreadEvent::MessageAdded(_) |
                    ThreadEvent::MessageEdited(_) |
                    ThreadEvent::MessageDeleted(_) |
                    ThreadEvent::SummaryChanged |
                    ThreadEvent::SummaryGenerated |
                    ThreadEvent::CheckpointChanged |
                    ThreadEvent::UsageUpdated(_) |
                    ThreadEvent::ShowError(_) => {
                        if std::env::var("ZED_EVAL_DEBUG").is_ok() {
                            println!("{}Event: {:#?}", log_prefix, event);
                        }
                    }
                }
            }});

            let mut thread_cx = ThreadContext::new(meta.clone(), this.log_prefix.clone(), thread.clone(), model.clone(), cx.clone());

            let assertions = select_biased! {
                result = this.thread.conversation(&mut thread_cx).fuse() => {
                    if let Err(err) = result {
                        if !err.is::<FailedAssertion>() {
                            return Err(err);
                        }
                    }

                    thread_cx.report()
                },
                _ = cx.background_executor().timer(THREAD_EVENT_TIMEOUT).fuse() => {
                    return Err(anyhow!("Agentic loop stalled - waited {:?} without any events", THREAD_EVENT_TIMEOUT));
                }
            };

            println!("{}Stopped", this.log_prefix);

            println!("{}Getting repository diff", this.log_prefix);
            let repository_diff = this.repository_diff().await?;

            let repository_diff_path = this.run_directory.join("patch.diff");
            let mut repository_diff_output_file = File::create(&repository_diff_path)?;
            writeln!(&mut repository_diff_output_file, "{}", &repository_diff).log_err();

            let (diagnostics_before, diagnostics_after) = if let Some((_, language_file_buffer, _, diagnostics_before)) = lsp {
                wait_for_lang_server(&project, &language_file_buffer, this.log_prefix.clone(), cx).await?;

                println!("{}Getting diagnostics", this.log_prefix);
                let diagnostics_after = cx
                    .update(move |cx| {
                        cx.spawn(async move |cx| query_lsp_diagnostics(project, cx).await)
                    })?
                    .await?;
                println!("{}Got diagnostics", this.log_prefix);

                (diagnostics_before, diagnostics_after)
            } else {
                (None, None)
            };

            let Some(last_request) = last_request.borrow_mut().take() else {
                return Err(anyhow!("No requests ran."));
            };

            drop(subscription);

            if let Some(diagnostics_before) = &diagnostics_before {
                fs::write(this.run_directory.join("diagnostics_before.txt"), diagnostics_before)?;
            }

            if let Some(diagnostics_after) = &diagnostics_after {
                fs::write(this.run_directory.join("diagnostics_after.txt"), diagnostics_after)?;
            }


            thread.update(cx, |thread, _cx| {
                let response_count = thread
                    .messages()
                    .filter(|message| message.role == language_model::Role::Assistant)
                    .count();
                RunOutput {
                    repository_diff: "".to_string(),
                    ran_diagnostics_check: meta.language_server.is_some(),
                    diagnostics_before,
                    diagnostics_after,
                    response_count,
                    token_usage: thread.cumulative_token_usage(),
                    tool_metrics: tool_metrics.lock().unwrap().clone(),
                    last_request,
                    assertions,
                }
            })
        })
    }

    async fn repository_diff(&self) -> Result<String> {
        let worktree_path = self.worktree_path();
        run_git(&worktree_path, &["add", "."]).await?;
        let mut diff_args = vec!["diff", "--staged"];
        if self.thread.meta().url == ZED_REPO_URL {
            diff_args.push(":(exclude).rules");
        }
        run_git(&worktree_path, &diff_args).await
    }

    pub async fn judge(
        &self,
        model: Arc<dyn LanguageModel>,
        run_output: &RunOutput,
        judge_number: u32,
        cx: &AsyncApp,
    ) -> Result<JudgeOutput> {
        let mut output_file = File::create(
            self.run_directory
                .join(format!("judge_{}.md", judge_number)),
        )
        .expect("failed to create judge.md");

        println!("{}Running judge #{judge_number}", self.log_prefix);

        let diff_task = self.judge_diff(model.clone(), &run_output, judge_number, cx);
        let thread_task = self.judge_thread(model.clone(), &run_output, judge_number, cx);

        let (diff_result, thread_result) = futures::join!(diff_task, thread_task);

        let (diff_response, diff_output) = diff_result?;
        let (thread_response, thread_output) = thread_result?;

        writeln!(
            &mut output_file,
            "# Judgment\n\n## Thread\n\n{thread_response}\n\n## Diff\n\n{diff_response}",
        )
        .log_err();

        Ok(JudgeOutput {
            thread: thread_output,
            diff: diff_output,
        })
    }

    async fn judge_diff(
        &self,
        model: Arc<dyn LanguageModel>,
        run_output: &RunOutput,
        judge_number: u32,
        cx: &AsyncApp,
    ) -> Result<(String, Option<JudgeResponse>)> {
        let diff_criteria = self.thread.diff_criteria();
        if diff_criteria.is_empty() {
            let msg = "No diff criteria specified.".to_string();
            return Ok((msg, None));
        }

        let judge_diff_prompt = include_str!("judge_diff_prompt.hbs");
        let judge_diff_prompt_name = "judge_diff_prompt";
        let mut hbs = Handlebars::new();
        hbs.register_template_string(judge_diff_prompt_name, judge_diff_prompt)?;

        let diff_prompt = hbs.render(
            judge_diff_prompt_name,
            &JudgeDiffInput {
                repository_diff: run_output.repository_diff.clone(),
                ran_diagnostics_check: run_output.ran_diagnostics_check,
                diagnostics_before: run_output.diagnostics_before.clone(),
                diagnostics_after: run_output.diagnostics_after.clone(),
                criteria: diff_criteria,
            },
        )?;

        let request = LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text(diff_prompt)],
                cache: false,
            }],
            temperature: None,
            tools: Vec::new(),
            stop: Vec::new(),
        };

        let diff_response = send_language_model_request(model, request, cx).await?;
        let diff_output = JudgeResponse::parse(&diff_response)?;

        println!(
            "{}Judge #{judge_number} - Diff score: {}",
            self.log_prefix, diff_output.score
        );

        Ok((diff_response, Some(diff_output)))
    }

    async fn judge_thread(
        &self,
        model: Arc<dyn LanguageModel>,
        run_output: &RunOutput,
        judge_number: u32,
        cx: &AsyncApp,
    ) -> Result<(String, Option<JudgeResponse>)> {
        let thread_criteria = self.thread.thread_criteria();
        if thread_criteria.is_empty() {
            let msg = "There were no criteria specified for this thread, so this example was not judged on its thread.".to_string();
            return Ok((msg, None));
        }

        let judge_thread_prompt = include_str!("judge_thread_prompt.hbs");
        let judge_thread_prompt_name = "judge_thread_prompt";
        let mut hbs = Handlebars::new();
        hbs.register_template_string(judge_thread_prompt_name, judge_thread_prompt)?;

        let request_markdown = RequestMarkdown::new(&run_output.last_request);
        let thread_prompt = hbs.render(
            judge_thread_prompt_name,
            &JudgeThreadInput {
                messages: request_markdown.messages,
                criteria: thread_criteria,
            },
        )?;

        let request = LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text(thread_prompt)],
                cache: false,
            }],
            temperature: None,
            tools: Vec::new(),
            stop: Vec::new(),
        };

        let thread_response = send_language_model_request(model, request, cx).await?;
        let thread_output = JudgeResponse::parse(&thread_response)?;

        println!(
            "{}Judge #{judge_number} - Thread score: {}",
            self.log_prefix, thread_output.score
        );

        Ok((thread_response, Some(thread_output)))
    }
}

pub fn wait_for_lang_server(
    project: &Entity<Project>,
    buffer: &Entity<Buffer>,
    log_prefix: String,
    cx: &mut AsyncApp,
) -> Task<Result<()>> {
    if std::env::var("ZED_EVAL_SKIP_LS").is_ok() {
        return Task::ready(Ok(()));
    }

    println!("{}⏵ Waiting for language server", log_prefix);

    let (mut tx, mut rx) = mpsc::channel(1);

    let lsp_store = project
        .update(cx, |project, _| project.lsp_store())
        .unwrap();

    let has_lang_server = buffer
        .update(cx, |buffer, cx| {
            lsp_store.update(cx, |lsp_store, cx| {
                lsp_store
                    .language_servers_for_local_buffer(&buffer, cx)
                    .next()
                    .is_some()
            })
        })
        .unwrap_or(false);

    if has_lang_server {
        project
            .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
            .unwrap()
            .detach();
    }

    let subscriptions =
        [
            cx.subscribe(&lsp_store, {
                let log_prefix = log_prefix.clone();
                move |_, event, _| match event {
                    project::LspStoreEvent::LanguageServerUpdate {
                        message:
                            client::proto::update_language_server::Variant::WorkProgress(
                                LspWorkProgress {
                                    message: Some(message),
                                    ..
                                },
                            ),
                        ..
                    } => println!("{}⟲ {message}", log_prefix),
                    _ => {}
                }
            }),
            cx.subscribe(&project, {
                let buffer = buffer.clone();
                move |project, event, cx| match event {
                    project::Event::LanguageServerAdded(_, _, _) => {
                        let buffer = buffer.clone();
                        project
                            .update(cx, |project, cx| project.save_buffer(buffer, cx))
                            .detach();
                    }
                    project::Event::DiskBasedDiagnosticsFinished { .. } => {
                        tx.try_send(()).ok();
                    }
                    _ => {}
                }
            }),
        ];

    cx.spawn(async move |cx| {
        let timeout = cx.background_executor().timer(Duration::new(60 * 5, 0));
        let result = futures::select! {
            _ = rx.next() => {
                println!("{}⚑ Language server idle", log_prefix);
                anyhow::Ok(())
            },
            _ = timeout.fuse() => {
                Err(anyhow!("LSP wait timed out after 5 minutes"))
            }
        };
        drop(subscriptions);
        result
    })
}

pub async fn query_lsp_diagnostics(
    project: Entity<Project>,
    cx: &mut AsyncApp,
) -> Result<Option<String>> {
    let paths_with_diagnostics = project.update(cx, |project, cx| {
        project
            .diagnostic_summaries(true, cx)
            .filter(|(_, _, summary)| summary.error_count > 0 || summary.warning_count > 0)
            .map(|(project_path, _, _)| project_path)
            .collect::<Vec<_>>()
    })?;

    if paths_with_diagnostics.is_empty() {
        return Ok(None);
    }

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
    anyhow::Ok(Some(output))
}

impl JudgeResponse {
    fn parse(response: &str) -> Result<Self> {
        let analysis = get_tag("analysis", response)?.to_string();
        let score = get_tag("score", response)?
            .parse()
            .context("error parsing score")?;

        Ok(Self { analysis, score })
    }
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

pub struct RequestMarkdown {
    pub tools: String,
    pub messages: String,
}

impl RequestMarkdown {
    pub fn new(request: &LanguageModelRequest) -> Self {
        let mut tools = String::new();
        let mut messages = String::new();
        let mut assistant_message_number: u32 = 1;

        // Print the tools
        if !request.tools.is_empty() {
            for tool in &request.tools {
                write!(&mut tools, "# {}\n\n", tool.name).unwrap();
                write!(&mut tools, "{}\n\n", tool.description).unwrap();
                write!(
                    &mut tools,
                    "{}\n",
                    MarkdownString::code_block("json", &format!("{:#}", tool.input_schema))
                )
                .unwrap();
            }
        }

        // Print the messages
        for message in &request.messages {
            match message.role {
                Role::System => messages.push_str("# ⚙️ SYSTEM\n\n"),
                Role::User => messages.push_str("# 👤 USER\n\n"),
                Role::Assistant => {
                    messages.push_str(&format!("# 🤖 ASSISTANT {assistant_message_number}\n\n"));
                    assistant_message_number += 1;
                }
            };

            for content in &message.content {
                match content {
                    MessageContent::Text(text) => {
                        messages.push_str(text);
                        messages.push_str("\n\n");
                    }
                    MessageContent::Image(_) => {
                        messages.push_str("[IMAGE DATA]\n\n");
                    }
                    MessageContent::Thinking { text, signature } => {
                        messages.push_str("**Thinking**:\n\n");
                        if let Some(sig) = signature {
                            messages.push_str(&format!("Signature: {}\n\n", sig));
                        }
                        messages.push_str(text);
                        messages.push_str("\n");
                    }
                    MessageContent::RedactedThinking(items) => {
                        messages.push_str(&format!(
                            "**Redacted Thinking**: {} item(s)\n\n",
                            items.len()
                        ));
                    }
                    MessageContent::ToolUse(tool_use) => {
                        messages.push_str(&format!(
                            "**Tool Use**: {} (ID: {})\n",
                            tool_use.name, tool_use.id
                        ));
                        messages.push_str(&format!(
                            "{}\n",
                            MarkdownString::code_block("json", &format!("{:#}", tool_use.input))
                        ));
                    }
                    MessageContent::ToolResult(tool_result) => {
                        messages.push_str(&format!(
                            "**Tool Result**: {} (ID: {})\n\n",
                            tool_result.tool_name, tool_result.tool_use_id
                        ));
                        if tool_result.is_error {
                            messages.push_str("**ERROR:**\n");
                        }
                        messages.push_str(&format!("{}\n\n", tool_result.content));
                    }
                }
            }
        }

        Self { tools, messages }
    }
}

pub fn response_events_to_markdown(
    response_events: &[std::result::Result<LanguageModelCompletionEvent, String>],
) -> String {
    let mut response = String::new();
    // Print the response events if any
    response.push_str("# Response\n\n");
    let mut text_buffer = String::new();
    let mut thinking_buffer = String::new();

    let flush_buffers =
        |output: &mut String, text_buffer: &mut String, thinking_buffer: &mut String| {
            if !text_buffer.is_empty() {
                output.push_str(&format!("**Text**:\n{}\n\n", text_buffer));
                text_buffer.clear();
            }
            if !thinking_buffer.is_empty() {
                output.push_str(&format!("**Thinking**:\n{}\n\n", thinking_buffer));
                thinking_buffer.clear();
            }
        };

    for event in response_events {
        match event {
            Ok(LanguageModelCompletionEvent::Text(text)) => {
                text_buffer.push_str(text);
            }
            Ok(LanguageModelCompletionEvent::Thinking { text, .. }) => {
                thinking_buffer.push_str(text);
            }
            Ok(LanguageModelCompletionEvent::Stop(reason)) => {
                flush_buffers(&mut response, &mut text_buffer, &mut thinking_buffer);
                response.push_str(&format!("**Stop**: {:?}\n\n", reason));
            }
            Ok(LanguageModelCompletionEvent::ToolUse(tool_use)) => {
                flush_buffers(&mut response, &mut text_buffer, &mut thinking_buffer);
                response.push_str(&format!(
                    "**Tool Use**: {} (ID: {})\n",
                    tool_use.name, tool_use.id
                ));
                response.push_str(&format!(
                    "{}\n",
                    MarkdownString::code_block("json", &format!("{:#}", tool_use.input))
                ));
            }
            Ok(
                LanguageModelCompletionEvent::UsageUpdate(_)
                | LanguageModelCompletionEvent::StartMessage { .. },
            ) => {}
            Err(error) => {
                flush_buffers(&mut response, &mut text_buffer, &mut thinking_buffer);
                response.push_str(&format!("**Error**: {}\n\n", error));
            }
        }
    }

    flush_buffers(&mut response, &mut text_buffer, &mut thinking_buffer);

    response
}

#[cfg(test)]
mod test {
    use super::*;
    use handlebars::Handlebars;

    #[test]
    fn test_parse_judge_output() {
        let response = r#"
            <analysis>The model did a good job but there were still compilations errors.</analysis>
            <score>3</score>
        "#
        .unindent();

        let output = JudgeResponse::parse(&response).unwrap();
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

        let output = JudgeResponse::parse(&response).unwrap();
        assert_eq!(output.analysis, "Failed to compile:\n- Error 1\n- Error 2");
        assert_eq!(output.score, 1);
    }

    #[test]
    fn test_judge_prompt_with_diagnostics() {
        // Case 1: Both diagnostics before and after are present
        let input = JudgeDiffInput {
            repository_diff: "diff content goes here".to_string(),
            ran_diagnostics_check: true,
            diagnostics_before: Some("Error at line 10: variable not found".to_string()),
            diagnostics_after: Some("Error at line 15: missing semicolon".to_string()),
            criteria: "Fix all bugs".to_string(),
        };

        let rendered = templates().render(JUDGE_PROMPT_NAME, &input).unwrap();

        let expected_diagnostics_section = r#"
            Take into account the diagnostics before and after applying the change:

            <diagnostics_before>
            Error at line 10: variable not found
            </diagnostics_before>

            <diagnostics_after>
            Error at line 15: missing semicolon
            </diagnostics_after>
            "#
        .unindent();

        assert!(rendered.contains(&expected_diagnostics_section));
    }

    #[test]
    fn test_judge_prompt_with_empty_diagnostics() {
        // Case 2: Diagnostics check run but no diagnostics found
        let input = JudgeDiffInput {
            repository_diff: "diff content goes here".to_string(),
            ran_diagnostics_check: true,
            diagnostics_before: None,
            diagnostics_after: None,
            criteria: "Fix all bugs".to_string(),
        };

        let rendered = templates().render(JUDGE_PROMPT_NAME, &input).unwrap();

        let expected_diagnostics_section = r#"
            Take into account the diagnostics before and after applying the change:

            <diagnostics_before>
            No diagnostics before applying the edits.
            </diagnostics_before>

            <diagnostics_after>
            No diagnostics after applying the edits.
            </diagnostics_after>
            "#
        .unindent();

        assert!(rendered.contains(&expected_diagnostics_section));
    }

    #[test]
    fn test_judge_prompt_with_mixed_diagnostics() {
        let templates = templates();

        // Case 3: Before diagnostics present, after diagnostics absent
        let input = JudgeDiffInput {
            repository_diff: "diff content goes here".to_string(),
            ran_diagnostics_check: true,
            diagnostics_before: Some("Error at line 10: variable not found".to_string()),
            diagnostics_after: None,
            criteria: "Fix all bugs".to_string(),
        };

        let rendered = templates.render(JUDGE_PROMPT_NAME, &input).unwrap();

        let expected_diagnostics_section = r#"
            Take into account the diagnostics before and after applying the change:

            <diagnostics_before>
            Error at line 10: variable not found
            </diagnostics_before>

            <diagnostics_after>
            No diagnostics after applying the edits.
            </diagnostics_after>
            "#
        .unindent();

        assert!(rendered.contains(&expected_diagnostics_section));

        // Case 4: Before diagnostics absent, after diagnostics present
        let input = JudgeDiffInput {
            repository_diff: "diff content goes here".to_string(),
            ran_diagnostics_check: true,
            diagnostics_before: None,
            diagnostics_after: Some("Error at line 15: missing semicolon".to_string()),
            criteria: "Fix all bugs".to_string(),
        };

        let rendered = templates.render(JUDGE_PROMPT_NAME, &input).unwrap();

        let expected_diagnostics_section = r#"
            Take into account the diagnostics before and after applying the change:

            <diagnostics_before>
            No diagnostics before applying the edits.
            </diagnostics_before>

            <diagnostics_after>
            Error at line 15: missing semicolon
            </diagnostics_after>
            "#
        .unindent();

        assert!(rendered.contains(&expected_diagnostics_section));
    }

    #[test]
    fn test_judge_prompt_without_diagnostics() {
        let templates = templates();

        // Case 5: No diagnostics check run
        let input = JudgeDiffInput {
            repository_diff: "diff content goes here".to_string(),
            ran_diagnostics_check: false,
            diagnostics_before: None,
            diagnostics_after: None,
            criteria: "Fix all bugs".to_string(),
        };

        let rendered = templates.render(JUDGE_PROMPT_NAME, &input).unwrap();

        // Check for the message when no diagnostics were performed
        let diagnostics_message = "No diagnostic checks were performed.";

        assert!(rendered.contains(diagnostics_message));
        assert!(!rendered.contains("<diagnostics_before>"));
        assert!(!rendered.contains("<diagnostics_after>"));
    }

    const JUDGE_PROMPT_NAME: &str = "judge_prompt";

    fn templates() -> Handlebars<'static> {
        let mut judge_prompt = include_str!("judge_diff_prompt.hbs").to_string();
        language::LineEnding::normalize(&mut judge_prompt);
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string(JUDGE_PROMPT_NAME, judge_prompt)
            .unwrap();
        handlebars
    }
}
