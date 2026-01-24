use agent::ContextServerRegistry;
use agent_client_protocol as acp;
use anyhow::{Context as _, Result, anyhow, bail};
use client::proto::LspWorkProgress;
use futures::channel::mpsc;
use futures::future::Shared;
use futures::{FutureExt as _, StreamExt as _, future};
use gpui::{App, AppContext as _, AsyncApp, Entity, Task};
use handlebars::Handlebars;
use language::{Buffer, DiagnosticSeverity, OffsetRangeExt as _};
use language_model::{
    LanguageModel, LanguageModelCompletionEvent, LanguageModelRegistry, LanguageModelRequest,
    LanguageModelRequestMessage, LanguageModelToolResultContent, MessageContent, Role, TokenUsage,
};
use project::{DiagnosticSummary, Project, ProjectPath, lsp_store::OpenLspBufferHandle};
use prompt_store::{ProjectContext, WorktreeContext};
use rand::{distr, prelude::*};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Write as _,
    fs::{self, File},
    io::Write as _,
    path::{Path, PathBuf},
    rc::Rc,
    sync::{Arc, Mutex},
    time::Duration,
};
use unindent::Unindent as _;
use util::{ResultExt as _, command::new_smol_command, markdown::MarkdownCodeBlock};

use crate::{
    AgentAppState, ToolMetrics,
    assertions::{AssertionsReport, RanAssertion, RanAssertionResult},
    example::{Example, ExampleContext, FailedAssertion, JudgeAssertion},
};

pub const ZED_REPO_URL: &str = "https://github.com/zed-industries/zed.git";

#[derive(Clone)]
pub struct ExampleInstance {
    pub thread: Rc<dyn Example>,
    pub name: String,
    pub run_directory: PathBuf,
    pub log_prefix: String,
    /// The repetition number for this example (0-based)
    /// When running multiple repetitions of the same example, each instance is assigned a unique repetition number.
    /// This affects the worktree path and log prefix to avoid clobbering results between runs.
    pub repetition: usize,
    pub repo_path: PathBuf,
    /// Path to the directory containing the requests and responses for the agentic loop
    worktrees_dir: PathBuf,
}

#[derive(Debug, Serialize, Clone)]
pub struct RunOutput {
    pub repository_diff: String,
    pub diagnostic_summary_before: DiagnosticSummary,
    pub diagnostic_summary_after: DiagnosticSummary,
    pub diagnostics_before: Option<String>,
    pub diagnostics_after: Option<String>,
    pub token_usage: TokenUsage,
    pub tool_metrics: ToolMetrics,
    pub thread_markdown: String,
    pub programmatic_assertions: AssertionsReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgeDiffInput {
    pub repository_diff: String,
    pub assertion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgeThreadInput {
    pub messages: String,
    pub assertion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgeOutput {
    pub thread: AssertionsReport,
    pub diff: AssertionsReport,
}

impl ExampleInstance {
    pub fn new(
        thread: Rc<dyn Example>,
        repos_dir: &Path,
        run_dir: &Path,
        worktrees_dir: &Path,
        repetition: usize,
    ) -> Self {
        let name = thread.meta().name;
        let run_directory = run_dir.join(&name).join(repetition.to_string());

        let repo_path = repo_path_for_url(repos_dir, &thread.meta().url);

        Self {
            name,
            thread,
            log_prefix: String::new(),
            run_directory,
            repetition,
            repo_path,
            worktrees_dir: worktrees_dir.to_path_buf(),
        }
    }

    pub fn repo_url(&self) -> String {
        self.thread.meta().url
    }

    pub fn revision(&self) -> String {
        self.thread.meta().revision
    }

    pub fn worktree_name(&self) -> String {
        format!("{}-{}", self.name, self.repetition)
    }

    pub fn set_log_prefix_style(&mut self, color: &str, name_width: usize) {
        self.log_prefix = format!(
            "{}{:<width$}\x1b[0m | ",
            color,
            self.worktree_name(),
            width = name_width
        );
    }

    /// Set up the example by checking out the specified Git revision
    pub async fn fetch(&mut self) -> Result<()> {
        let meta = self.thread.meta();

        let revision_exists = run_git(
            &self.repo_path,
            &["rev-parse", &format!("{}^{{commit}}", &meta.revision)],
        )
        .await
        .is_ok();

        if !revision_exists {
            println!("{}Fetching revision {}", self.log_prefix, &meta.revision);
            run_git(
                &self.repo_path,
                &["fetch", "--depth", "1", "origin", &meta.revision],
            )
            .await?;
        }
        Ok(())
    }

    /// Set up the example by checking out the specified Git revision
    pub async fn setup(&mut self) -> Result<()> {
        let worktree_path = self.worktree_path();
        let meta = self.thread.meta();
        if worktree_path.is_dir() {
            println!("{}Resetting existing worktree", self.log_prefix);

            // TODO: consider including "-x" to remove ignored files. The downside of this is that
            // it will also remove build artifacts, and so prevent incremental reuse there.
            run_git(&worktree_path, &["clean", "--force", "-d"]).await?;
            run_git(&worktree_path, &["reset", "--hard", "HEAD"]).await?;
            run_git(&worktree_path, &["checkout", &meta.revision]).await?;
        } else {
            println!("{}Creating worktree", self.log_prefix);

            let worktree_path_string = worktree_path.to_string_lossy().into_owned();

            run_git(
                &self.repo_path,
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
        self.worktrees_dir
            .join(self.worktree_name())
            .join(self.thread.meta().repo_name())
    }

    pub fn run(&self, app_state: Arc<AgentAppState>, cx: &mut App) -> Task<Result<RunOutput>> {
        let project = Project::local(
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            None,
            project::LocalProjectFlags {
                init_worktree_trust: false,
                ..Default::default()
            },
            cx,
        );

        let worktree = project.update(cx, |project, cx| {
            project.create_worktree(self.worktree_path(), true, cx)
        });

        let meta = self.thread.meta();
        let this = self.clone();

        cx.spawn(async move |cx| {
            let worktree = worktree.await?;

            // Wait for worktree scan to finish before choosing a file to open.
            worktree
                .update(cx, |worktree, _cx| {
                    worktree.as_local().unwrap().scan_complete()
                })
                .await;

            struct LanguageServerState {
                _lsp_open_handle: OpenLspBufferHandle,
                language_file_buffer: Entity<Buffer>,
            }

            let mut diagnostics_before = None;
            let mut diagnostic_summary_before = DiagnosticSummary::default();

            let lsp = if let Some(language_server) = &meta.language_server {
                // Open a file that matches the language to cause LSP to start.
                let language_file = worktree
                    .read_with(cx, |worktree, _cx| {
                        worktree
                            .files(false, 0)
                            .find_map(|e| {
                                if e.path.clone().extension()
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
                    })?;

                let open_language_file_buffer_task = project.update(cx, |project, cx| {
                    project.open_buffer(language_file.clone(), cx)
                });

                let language_file_buffer = open_language_file_buffer_task.await?;

                let lsp_open_handle = project.update(cx, |project, cx| {
                    project.register_buffer_with_language_servers(&language_file_buffer, cx)
                });

                wait_for_lang_server(&project, &language_file_buffer, this.log_prefix.clone(), cx).await?;

                diagnostic_summary_before = project.read_with(cx, |project, cx| {
                    project.diagnostic_summary(false, cx)
                });

                diagnostics_before = query_lsp_diagnostics(project.clone(), cx).await?;
                if diagnostics_before.is_some() && language_server.allow_preexisting_diagnostics {
                    anyhow::bail!("Example has pre-existing diagnostics. If you want to run this example regardless, set `allow_preexisting_diagnostics` to `true` in `base.toml`");
                }

                Some(LanguageServerState {
                    _lsp_open_handle: lsp_open_handle,
                    language_file_buffer,
                })
            } else {
                None
            };

            anyhow::ensure!(std::env::var("ZED_EVAL_SETUP_ONLY").is_err(), "Setup only mode");

            let last_diff_file_path = this.run_directory.join("last.diff");

            // Write an empty "last.diff" so that it can be opened in Zed for convenient view of the
            // history using undo/redo.
            std::fs::write(&last_diff_file_path, "")?;

            let thread = cx.update(|cx| {
                //todo: Do we want to load rules files here?
                let worktrees = project.read(cx).visible_worktrees(cx).map(|worktree| {
                    let root_name = worktree.read(cx).root_name_str().into();
                    let abs_path = worktree.read(cx).abs_path();

                    WorktreeContext {
                        root_name,
                        abs_path,
                        rules_file: None,
                    }
                }).collect::<Vec<_>>();
                let project_context = cx.new(|_cx| ProjectContext::new(worktrees, vec![]));
                let context_server_registry = cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));

                let thread = if let Some(json) = &meta.existing_thread_json {
                    let session_id = acp::SessionId::new(
                        rand::rng()
                            .sample_iter(&distr::Alphanumeric)
                            .take(7)
                            .map(char::from)
                            .collect::<String>(),
                    );

                    let db_thread = agent::DbThread::from_json(json.as_bytes()).expect("Can't read serialized thread");
                    cx.new(|cx| agent::Thread::from_db(session_id, db_thread, project.clone(), project_context, context_server_registry, agent::Templates::new(), cx))
                } else {
                    cx.new(|cx| agent::Thread::new(project.clone(), project_context, context_server_registry, agent::Templates::new(), None, cx))
                };

                thread.update(cx, |thread, cx| {
                    thread.add_default_tools(Rc::new(EvalThreadEnvironment {
                        project: project.clone(),
                    }), cx);
                    thread.set_profile(meta.profile_id.clone(), cx);
                    thread.set_model(
                        LanguageModelInterceptor::new(
                            LanguageModelRegistry::read_global(cx).default_model().expect("Missing model").model.clone(),
                            this.run_directory.clone(),
                            last_diff_file_path.clone(),
                            this.run_directory.join("last.messages.json"),
                            this.worktree_path(),
                            this.repo_url(),
                        ),
                        cx,
                    );
                });

                thread
            });

            let mut example_cx = ExampleContext::new(
                meta.clone(),
                this.log_prefix.clone(),
                thread.clone(),
                cx.clone(),
            );
            let result = this.thread.conversation(&mut example_cx).await;

            if let Err(err) = result
                && !err.is::<FailedAssertion>() {
                    return Err(err);
                }

            println!("{}Stopped", this.log_prefix);

            println!("{}Getting repository diff", this.log_prefix);
            let repository_diff = Self::repository_diff(this.worktree_path(), &this.repo_url()).await?;

            std::fs::write(last_diff_file_path, &repository_diff)?;


            let mut diagnostics_after = None;
            let mut diagnostic_summary_after = Default::default();

            if let Some(language_server_state) = lsp {
                wait_for_lang_server(&project, &language_server_state.language_file_buffer, this.log_prefix.clone(), cx).await?;

                println!("{}Getting diagnostics", this.log_prefix);
                diagnostics_after = cx
                    .update(|cx| {
                        let project = project.clone();
                        cx.spawn(async move |cx| query_lsp_diagnostics(project, cx).await)
                    })
                    .await?;
                println!("{}Got diagnostics", this.log_prefix);

                diagnostic_summary_after = project.read_with(cx, |project, cx| {
                    project.diagnostic_summary(false, cx)
                });

            }

            if let Some(diagnostics_before) = &diagnostics_before {
                fs::write(this.run_directory.join("diagnostics_before.txt"), diagnostics_before)?;
            }

            if let Some(diagnostics_after) = &diagnostics_after {
                fs::write(this.run_directory.join("diagnostics_after.txt"), diagnostics_after)?;
            }

            Ok(thread.update(cx, |thread, _cx| {
                RunOutput {
                    repository_diff,
                    diagnostic_summary_before,
                    diagnostic_summary_after,
                    diagnostics_before,
                    diagnostics_after,
                    token_usage: thread.latest_request_token_usage().unwrap(),
                    tool_metrics: example_cx.tool_metrics.lock().unwrap().clone(),
                    thread_markdown: thread.to_markdown(),
                    programmatic_assertions: example_cx.assertions,
                }
            }))
        })
    }

    async fn repository_diff(repository_path: PathBuf, repository_url: &str) -> Result<String> {
        run_git(&repository_path, &["add", "."]).await?;
        let mut diff_args = vec!["diff", "--staged"];
        if repository_url == ZED_REPO_URL {
            diff_args.push(":(exclude).rules");
        }
        run_git(&repository_path, &diff_args).await
    }

    pub async fn judge(
        &self,
        model: Arc<dyn LanguageModel>,
        run_output: &RunOutput,
        cx: &AsyncApp,
    ) -> JudgeOutput {
        let mut output_file =
            File::create(self.run_directory.join("judge.md")).expect("failed to create judge.md");

        let diff_task = self.judge_diff(model.clone(), run_output, cx);
        let thread_task = self.judge_thread(model.clone(), run_output, cx);

        let (diff_result, thread_result) = futures::join!(diff_task, thread_task);

        let (diff_response, diff_output) = diff_result;
        let (thread_response, thread_output) = thread_result;

        writeln!(
            &mut output_file,
            "# Judgment\n\n## Thread\n\n{thread_response}\n\n## Diff\n\n{diff_response}",
        )
        .log_err();

        JudgeOutput {
            thread: thread_output,
            diff: diff_output,
        }
    }

    async fn judge_diff(
        &self,
        model: Arc<dyn LanguageModel>,
        run_output: &RunOutput,
        cx: &AsyncApp,
    ) -> (String, AssertionsReport) {
        let diff_assertions = self.thread.diff_assertions();

        if diff_assertions.is_empty() {
            return (
                "No diff assertions".to_string(),
                AssertionsReport::default(),
            );
        }

        println!("{}Running diff judge", self.log_prefix);

        let judge_diff_prompt = include_str!("judge_diff_prompt.hbs");
        let judge_diff_prompt_name = "judge_diff_prompt";
        let mut hbs = Handlebars::new();
        hbs.register_template_string(judge_diff_prompt_name, judge_diff_prompt)
            .unwrap();

        let to_prompt = |assertion: String| {
            hbs.render(
                judge_diff_prompt_name,
                &JudgeDiffInput {
                    repository_diff: run_output.repository_diff.clone(),
                    assertion,
                },
            )
            .unwrap()
        };

        let (responses, report) = self
            .judge_assertions(model, diff_assertions, to_prompt, cx)
            .await;

        println!(
            "{}Judge - Diff score: {}%",
            self.log_prefix,
            report.passed_percentage()
        );

        (responses, report)
    }

    async fn judge_thread(
        &self,
        model: Arc<dyn LanguageModel>,
        run_output: &RunOutput,
        cx: &AsyncApp,
    ) -> (String, AssertionsReport) {
        let thread_assertions = self.thread.thread_assertions();

        if thread_assertions.is_empty() {
            return (
                "No thread assertions".to_string(),
                AssertionsReport::default(),
            );
        }

        let judge_thread_prompt = include_str!("judge_thread_prompt.hbs");
        let judge_thread_prompt_name = "judge_thread_prompt";
        let mut hbs = Handlebars::new();
        hbs.register_template_string(judge_thread_prompt_name, judge_thread_prompt)
            .unwrap();

        let complete_messages = &run_output.thread_markdown;
        let to_prompt = |assertion: String| {
            hbs.render(
                judge_thread_prompt_name,
                &JudgeThreadInput {
                    messages: complete_messages.clone(),
                    assertion,
                },
            )
            .unwrap()
        };

        let (responses, report) = self
            .judge_assertions(model, thread_assertions, to_prompt, cx)
            .await;

        println!(
            "{}Judge - Thread score: {}%",
            self.log_prefix,
            report.passed_percentage()
        );

        (responses, report)
    }

    async fn judge_assertions(
        &self,
        model: Arc<dyn LanguageModel>,
        assertions: Vec<JudgeAssertion>,
        to_prompt: impl Fn(String) -> String,
        cx: &AsyncApp,
    ) -> (String, AssertionsReport) {
        let assertions = assertions.into_iter().map(|assertion| {
            let request = LanguageModelRequest {
                thread_id: None,
                prompt_id: None,
                intent: None,
                messages: vec![LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![MessageContent::Text(to_prompt(assertion.description))],
                    cache: false,
                    reasoning_details: None,
                }],
                temperature: None,
                tools: Vec::new(),
                tool_choice: None,
                stop: Vec::new(),
                thinking_allowed: true,
            };

            let model = model.clone();
            let log_prefix = self.log_prefix.clone();
            async move {
                let response = send_language_model_request(model, request, cx).await;

                let (response, result) = match response {
                    Ok(response) => (
                        response.clone(),
                        parse_assertion_result(&response).map_err(|err| err.to_string()),
                    ),
                    Err(err) => (err.to_string(), Err(err.to_string())),
                };

                if result.is_ok() {
                    println!("{}✅ {}", log_prefix, assertion.id);
                } else {
                    println!("{}❌ {}", log_prefix, assertion.id);
                }

                (
                    response,
                    RanAssertion {
                        id: assertion.id,
                        result,
                    },
                )
            }
        });

        let mut responses = String::new();
        let mut report = AssertionsReport::default();

        for (response, assertion) in future::join_all(assertions).await {
            writeln!(&mut responses, "# {}", assertion.id).unwrap();
            writeln!(&mut responses, "{}\n\n", response).unwrap();
            report.ran.push(assertion);
        }

        (responses, report)
    }
}

struct EvalThreadEnvironment {
    project: Entity<Project>,
}

struct EvalTerminalHandle {
    terminal: Entity<acp_thread::Terminal>,
}

impl agent::TerminalHandle for EvalTerminalHandle {
    fn id(&self, cx: &AsyncApp) -> Result<acp::TerminalId> {
        Ok(self.terminal.read_with(cx, |term, _cx| term.id().clone()))
    }

    fn wait_for_exit(&self, cx: &AsyncApp) -> Result<Shared<Task<acp::TerminalExitStatus>>> {
        Ok(self
            .terminal
            .read_with(cx, |term, _cx| term.wait_for_exit()))
    }

    fn current_output(&self, cx: &AsyncApp) -> Result<acp::TerminalOutputResponse> {
        Ok(self
            .terminal
            .read_with(cx, |term, cx| term.current_output(cx)))
    }

    fn kill(&self, cx: &AsyncApp) -> Result<()> {
        cx.update(|cx| {
            self.terminal.update(cx, |terminal, cx| {
                terminal.kill(cx);
            });
        });
        Ok(())
    }

    fn was_stopped_by_user(&self, cx: &AsyncApp) -> Result<bool> {
        Ok(self
            .terminal
            .read_with(cx, |term, _cx| term.was_stopped_by_user()))
    }
}

impl agent::ThreadEnvironment for EvalThreadEnvironment {
    fn create_terminal(
        &self,
        command: String,
        cwd: Option<PathBuf>,
        output_byte_limit: Option<u64>,
        cx: &mut AsyncApp,
    ) -> Task<Result<Rc<dyn agent::TerminalHandle>>> {
        let project = self.project.clone();
        cx.spawn(async move |cx| {
            let language_registry =
                project.read_with(cx, |project, _cx| project.languages().clone());
            let id = acp::TerminalId::new(uuid::Uuid::new_v4().to_string());
            let terminal =
                acp_thread::create_terminal_entity(command, &[], vec![], cwd.clone(), &project, cx)
                    .await?;
            let terminal = cx.new(|cx| {
                acp_thread::Terminal::new(
                    id,
                    "",
                    cwd,
                    output_byte_limit.map(|limit| limit as usize),
                    terminal,
                    language_registry,
                    cx,
                )
            });
            Ok(Rc::new(EvalTerminalHandle { terminal }) as Rc<dyn agent::TerminalHandle>)
        })
    }
}

struct LanguageModelInterceptor {
    model: Arc<dyn LanguageModel>,
    request_count: Arc<Mutex<usize>>,
    previous_diff: Arc<Mutex<String>>,
    example_output_dir: PathBuf,
    last_diff_file_path: PathBuf,
    messages_json_file_path: PathBuf,
    repository_path: PathBuf,
    repository_url: String,
}

impl LanguageModelInterceptor {
    fn new(
        model: Arc<dyn LanguageModel>,
        example_output_dir: PathBuf,
        last_diff_file_path: PathBuf,
        messages_json_file_path: PathBuf,
        repository_path: PathBuf,
        repository_url: String,
    ) -> Arc<Self> {
        Arc::new(Self {
            model,
            request_count: Arc::new(Mutex::new(0)),
            previous_diff: Arc::new(Mutex::new("".to_string())),
            example_output_dir,
            last_diff_file_path,
            messages_json_file_path,
            repository_path,
            repository_url,
        })
    }
}

impl language_model::LanguageModel for LanguageModelInterceptor {
    fn id(&self) -> language_model::LanguageModelId {
        self.model.id()
    }

    fn name(&self) -> language_model::LanguageModelName {
        self.model.name()
    }

    fn provider_id(&self) -> language_model::LanguageModelProviderId {
        self.model.provider_id()
    }

    fn provider_name(&self) -> language_model::LanguageModelProviderName {
        self.model.provider_name()
    }

    fn telemetry_id(&self) -> String {
        self.model.telemetry_id()
    }

    fn supports_images(&self) -> bool {
        self.model.supports_images()
    }

    fn supports_tools(&self) -> bool {
        self.model.supports_tools()
    }

    fn supports_tool_choice(&self, choice: language_model::LanguageModelToolChoice) -> bool {
        self.model.supports_tool_choice(choice)
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> future::BoxFuture<'static, Result<u64>> {
        self.model.count_tokens(request, cx)
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> future::BoxFuture<
        'static,
        Result<
            futures::stream::BoxStream<
                'static,
                Result<LanguageModelCompletionEvent, language_model::LanguageModelCompletionError>,
            >,
            language_model::LanguageModelCompletionError,
        >,
    > {
        let stream = self.model.stream_completion(request.clone(), cx);
        let request_count = self.request_count.clone();
        let previous_diff = self.previous_diff.clone();
        let example_output_dir = self.example_output_dir.clone();
        let last_diff_file_path = self.last_diff_file_path.clone();
        let messages_json_file_path = self.messages_json_file_path.clone();
        let repository_path = self.repository_path.clone();
        let repository_url = self.repository_url.clone();

        Box::pin(async move {
            let stream = stream.await?;

            let response_events = Arc::new(Mutex::new(Vec::new()));
            let request_clone = request.clone();

            let wrapped_stream = stream.then(move |event| {
                let response_events = response_events.clone();
                let request = request_clone.clone();
                let request_count = request_count.clone();
                let previous_diff = previous_diff.clone();
                let example_output_dir = example_output_dir.clone();
                let last_diff_file_path = last_diff_file_path.clone();
                let messages_json_file_path = messages_json_file_path.clone();
                let repository_path = repository_path.clone();
                let repository_url = repository_url.clone();

                async move {
                    let event_result = match &event {
                        Ok(ev) => Ok(ev.clone()),
                        Err(err) => Err(err.to_string()),
                    };
                    response_events.lock().unwrap().push(event_result);

                    let should_execute = matches!(
                        &event,
                        Ok(LanguageModelCompletionEvent::Stop { .. }) | Err(_)
                    );

                    if should_execute {
                        let current_request_count = {
                            let mut count = request_count.lock().unwrap();
                            *count += 1;
                            *count
                        };

                        let messages_file_path =
                            example_output_dir.join(format!("{current_request_count}.messages.md"));
                        let diff_file_path =
                            example_output_dir.join(format!("{current_request_count}.diff"));
                        let last_messages_file_path = example_output_dir.join("last.messages.md");

                        let collected_events = response_events.lock().unwrap().clone();
                        let request_markdown = RequestMarkdown::new(&request);
                        let response_events_markdown =
                            response_events_to_markdown(&collected_events);
                        let dialog = ThreadDialog::new(&request, &collected_events);
                        let dialog_json =
                            serde_json::to_string_pretty(&dialog.to_combined_request())
                                .unwrap_or_default();

                        let messages = format!(
                            "{}\n\n{}",
                            request_markdown.messages, response_events_markdown
                        );
                        fs::write(&messages_file_path, messages.clone())
                            .expect("failed to write messages file");
                        fs::write(&last_messages_file_path, messages)
                            .expect("failed to write last messages file");
                        fs::write(&messages_json_file_path, dialog_json)
                            .expect("failed to write last.messages.json");

                        // Get repository diff
                        let diff_result =
                            ExampleInstance::repository_diff(repository_path, &repository_url)
                                .await;

                        match diff_result {
                            Ok(diff) => {
                                let prev_diff = previous_diff.lock().unwrap().clone();
                                if diff != prev_diff {
                                    fs::write(&diff_file_path, &diff)
                                        .expect("failed to write diff file");
                                    fs::write(&last_diff_file_path, &diff)
                                        .expect("failed to write last diff file");
                                    *previous_diff.lock().unwrap() = diff;
                                }
                            }
                            Err(err) => {
                                let error_message = format!("{err:?}");
                                fs::write(&diff_file_path, &error_message)
                                    .expect("failed to write diff error to file");
                                fs::write(&last_diff_file_path, &error_message)
                                    .expect("failed to write last diff file");
                            }
                        }

                        if current_request_count == 1 {
                            let tools_file_path = example_output_dir.join("tools.md");
                            fs::write(tools_file_path, request_markdown.tools)
                                .expect("failed to write tools file");
                        }
                    }

                    event
                }
            });

            Ok(Box::pin(wrapped_stream)
                as futures::stream::BoxStream<
                    'static,
                    Result<
                        LanguageModelCompletionEvent,
                        language_model::LanguageModelCompletionError,
                    >,
                >)
        })
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

    let lsp_store = project.read_with(cx, |project, _| project.lsp_store());

    let has_lang_server = buffer.update(cx, |buffer, cx| {
        lsp_store.update(cx, |lsp_store, cx| {
            lsp_store
                .running_language_servers_for_local_buffer(buffer, cx)
                .next()
                .is_some()
        })
    });

    if has_lang_server {
        project
            .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
            .detach();
    }

    let subscriptions =
        [
            cx.subscribe(&lsp_store, {
                let log_prefix = log_prefix.clone();
                move |_, event, _| {
                    if let project::LspStoreEvent::LanguageServerUpdate {
                        message:
                            client::proto::update_language_server::Variant::WorkProgress(
                                LspWorkProgress {
                                    message: Some(message),
                                    ..
                                },
                            ),
                        ..
                    } = event
                    {
                        println!("{}⟲ {message}", log_prefix)
                    }
                }
            }),
            cx.subscribe(project, {
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
                anyhow::bail!("LSP wait timed out after 5 minutes");
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
    });

    if paths_with_diagnostics.is_empty() {
        return Ok(None);
    }

    let mut output = String::new();
    for project_path in paths_with_diagnostics {
        let buffer = project
            .update(cx, |project, cx| project.open_buffer(project_path, cx))
            .await?;
        let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());

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

fn parse_assertion_result(response: &str) -> Result<RanAssertionResult> {
    let analysis = get_tag("analysis", response)?;
    let passed = match get_tag("passed", response)?.to_lowercase().as_str() {
        "true" => true,
        "false" => false,
        value @ _ => bail!("invalid judge `passed` tag: {value}"),
    };
    Ok(RanAssertionResult {
        analysis: Some(analysis),
        passed,
    })
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

pub fn repo_path_for_url(repos_dir: &Path, repo_url: &str) -> PathBuf {
    let repo_name = repo_url
        .trim_start_matches("https://")
        .replace(|c: char| !c.is_alphanumeric(), "-");
    Path::new(repos_dir).join(repo_name)
}

pub async fn run_git(repo_path: &Path, args: &[&str]) -> Result<String> {
    let output = new_smol_command("git")
        .current_dir(repo_path)
        .args(args)
        .output()
        .await?;

    anyhow::ensure!(
        output.status.success(),
        "`git {}` within `{}` failed with status: {}\nstderr:\n{}\nstdout:\n{}",
        args.join(" "),
        repo_path.display(),
        output.status,
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout),
    );
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

fn push_role(role: &Role, buf: &mut String, assistant_message_number: &mut u32) {
    match role {
        Role::System => buf.push_str("# ⚙️ SYSTEM\n\n"),
        Role::User => buf.push_str("# 👤 USER\n\n"),
        Role::Assistant => {
            buf.push_str(&format!("# 🤖 ASSISTANT {assistant_message_number}\n\n"));
            *assistant_message_number = *assistant_message_number + 1;
        }
    }
}

pub async fn send_language_model_request(
    model: Arc<dyn LanguageModel>,
    request: LanguageModelRequest,
    cx: &AsyncApp,
) -> anyhow::Result<String> {
    match model.stream_completion_text(request, cx).await {
        Ok(mut stream) => {
            let mut full_response = String::new();
            while let Some(chunk_result) = stream.stream.next().await {
                match chunk_result {
                    Ok(chunk_str) => {
                        full_response.push_str(&chunk_str);
                    }
                    Err(err) => {
                        anyhow::bail!("Error receiving response from language model: {err}");
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
                writeln!(
                    &mut tools,
                    "{}",
                    MarkdownCodeBlock {
                        tag: "json",
                        text: &format!("{:#}", tool.input_schema)
                    }
                )
                .unwrap();
            }
        }

        // Print the messages
        for message in &request.messages {
            push_role(&message.role, &mut messages, &mut assistant_message_number);

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
                            MarkdownCodeBlock {
                                tag: "json",
                                text: &format!("{:#}", tool_use.input)
                            }
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

                        match &tool_result.content {
                            LanguageModelToolResultContent::Text(text) => {
                                writeln!(messages, "{text}\n").ok();
                            }
                            LanguageModelToolResultContent::Image(image) => {
                                writeln!(messages, "![Image](data:base64,{})\n", image.source).ok();
                            }
                        }

                        if let Some(output) = tool_result.output.as_ref() {
                            writeln!(
                                messages,
                                "**Debug Output**:\n\n```json\n{}\n```\n",
                                serde_json::to_string_pretty(output).unwrap()
                            )
                            .unwrap();
                        }
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
            Ok(LanguageModelCompletionEvent::RedactedThinking { .. }) => {}
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
                    MarkdownCodeBlock {
                        tag: "json",
                        text: &format!("{:#}", tool_use.input)
                    }
                ));
            }
            Ok(
                LanguageModelCompletionEvent::UsageUpdate(_)
                | LanguageModelCompletionEvent::StartMessage { .. }
                | LanguageModelCompletionEvent::Queued { .. }
                | LanguageModelCompletionEvent::Started
                | LanguageModelCompletionEvent::ReasoningDetails(_),
            ) => {}
            Ok(LanguageModelCompletionEvent::ToolUseJsonParseError {
                json_parse_error, ..
            }) => {
                flush_buffers(&mut response, &mut text_buffer, &mut thinking_buffer);
                response.push_str(&format!(
                    "**Error**: parse error in tool use JSON: {}\n\n",
                    json_parse_error
                ));
            }
            Err(error) => {
                flush_buffers(&mut response, &mut text_buffer, &mut thinking_buffer);
                response.push_str(&format!("**Error**: {}\n\n", error));
            }
        }
    }

    flush_buffers(&mut response, &mut text_buffer, &mut thinking_buffer);

    response
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ThreadDialog {
    pub request: LanguageModelRequest,
    pub response_events: Vec<std::result::Result<LanguageModelCompletionEvent, String>>,
}

impl ThreadDialog {
    pub fn new(
        request: &LanguageModelRequest,
        response_events: &[std::result::Result<LanguageModelCompletionEvent, String>],
    ) -> Self {
        Self {
            request: request.clone(),
            response_events: response_events.to_vec(),
        }
    }

    /// Represents all request and response messages in a unified format.
    ///
    /// Specifically, it appends the assistant's response (derived from response events)
    /// as a new message to existing messages in the request.
    pub fn to_combined_request(&self) -> LanguageModelRequest {
        let mut request = self.request.clone();
        if let Some(assistant_message) = self.response_events_to_message() {
            request.messages.push(assistant_message);
        }
        request
    }
    fn response_events_to_message(&self) -> Option<LanguageModelRequestMessage> {
        let response_events = &self.response_events;
        let mut content: Vec<MessageContent> = Vec::new();
        let mut current_text = String::new();

        let flush_text = |text: &mut String, content: &mut Vec<MessageContent>| {
            if !text.is_empty() {
                content.push(MessageContent::Text(std::mem::take(text)));
            }
        };

        for event in response_events {
            match event {
                Ok(LanguageModelCompletionEvent::Text(text)) => {
                    current_text.push_str(text);
                }

                Ok(LanguageModelCompletionEvent::ToolUse(tool_use)) => {
                    flush_text(&mut current_text, &mut content);
                    if tool_use.is_input_complete {
                        content.push(MessageContent::ToolUse(tool_use.clone()));
                    }
                }
                Ok(LanguageModelCompletionEvent::Thinking { text, signature }) => {
                    flush_text(&mut current_text, &mut content);
                    content.push(MessageContent::Thinking {
                        text: text.clone(),
                        signature: signature.clone(),
                    });
                }

                // Skip these
                Ok(LanguageModelCompletionEvent::UsageUpdate(_))
                | Ok(LanguageModelCompletionEvent::RedactedThinking { .. })
                | Ok(LanguageModelCompletionEvent::StartMessage { .. })
                | Ok(LanguageModelCompletionEvent::ReasoningDetails(_))
                | Ok(LanguageModelCompletionEvent::Stop(_))
                | Ok(LanguageModelCompletionEvent::Queued { .. })
                | Ok(LanguageModelCompletionEvent::Started) => {}

                Ok(LanguageModelCompletionEvent::ToolUseJsonParseError {
                    json_parse_error,
                    ..
                }) => {
                    flush_text(&mut current_text, &mut content);
                    content.push(MessageContent::Text(format!(
                        "ERROR: parse error in tool use JSON: {}",
                        json_parse_error
                    )));
                }

                Err(error) => {
                    flush_text(&mut current_text, &mut content);
                    content.push(MessageContent::Text(format!("ERROR: {}", error)));
                }
            }
        }

        flush_text(&mut current_text, &mut content);

        if !content.is_empty() {
            Some(LanguageModelRequestMessage {
                role: Role::Assistant,
                content,
                cache: false,
                reasoning_details: None,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_judge_output() {
        let response = r#"
            <analysis>The model did a good job but there were still compilations errors.</analysis>
            <passed>true</passed>
        "#
        .unindent();

        let output = parse_assertion_result(&response).unwrap();
        assert_eq!(
            output.analysis,
            Some("The model did a good job but there were still compilations errors.".into())
        );
        assert!(output.passed);

        let response = r#"
            Text around ignored

            <analysis>
                Failed to compile:
                - Error 1
                - Error 2
            </analysis>

            <passed>false</passed>
        "#
        .unindent();

        let output = parse_assertion_result(&response).unwrap();
        assert_eq!(
            output.analysis,
            Some("Failed to compile:\n- Error 1\n- Error 2".into())
        );
        assert!(!output.passed);
    }
}
