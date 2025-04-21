use agent::{ThreadEvent, ThreadStore};
use anyhow::{Context, Result, anyhow};
use assistant_tool::ToolWorkingSet;
use futures::channel::mpsc;
use futures::{FutureExt as _, StreamExt as _, select_biased};
use gpui::{App, AppContext as _, Task};
use language_model::{LanguageModel, StopReason};
use project::{Project, ProjectPath};
use std::cell::RefCell;
use std::fs;
use std::fs::File;
use std::io::Write as _;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use util::ResultExt as _;

use crate::thread::EvalThread;
use crate::{
    AgentAppState, RequestMarkdown, RunOutput, THREAD_EVENT_TIMEOUT, ToolMetrics, WORKTREES_DIR,
    ZED_REPO_URL, query_lsp_diagnostics, repo_path_for_url, response_events_to_markdown, run_git,
    wait_for_lang_server,
};

#[derive(Clone)]
pub struct ThreadInstance {
    name: String,
    thread: Rc<dyn EvalThread>,
    pub run_directory: PathBuf,
    pub log_prefix: String,
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

    pub fn repo_url(&self) -> &'static str {
        self.thread.meta().url
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
        let repo_path = repo_path_for_url(meta.url);

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
                &["fetch", "--depth", "1", "origin", meta.revision],
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
            run_git(&worktree_path, &["checkout", meta.revision]).await?;
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
                    meta.revision,
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
                                == Some(language_server.file_extension)
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

            let (thread_event_tx, mut thread_event_rx) = mpsc::unbounded();

            let subscription = cx.subscribe(&thread, move |_thread, event: &ThreadEvent, _cx| {
                thread_event_tx.unbounded_send(event.clone()).log_err();
            });

            let event_handler_task = cx.spawn({
                let log_prefix = this.log_prefix.clone();
                let tool_metrics = tool_metrics.clone();
                let thread = thread.downgrade();
                async move |cx| {
                    loop {
                        let event = select_biased! {
                            event = thread_event_rx.next() => event,
                            _ = cx.background_executor().timer(THREAD_EVENT_TIMEOUT).fuse() => {
                                return Err(anyhow!("Agentic loop stalled - waited {:?} without any events", THREAD_EVENT_TIMEOUT));
                            }
                        };
                        let Some(event) = event else {
                            return Err(anyhow!("ThreadEvent channel ended early"));
                        };

                        match event {
                            ThreadEvent::Stopped(reason) => match reason {
                                Ok(StopReason::EndTurn) => {
                                    return Ok(());
                                }
                                Ok(StopReason::MaxTokens) => {
                                    return Err(anyhow!("Exceeded maximum tokens"));
                                }
                                Ok(StopReason::ToolUse) => {
                                    if std::env::var("ZED_EVAL_DEBUG").is_ok() {
                                        println!("{}StopReason: Tool use", log_prefix);
                                    }
                                }
                                Err(error) => {
                                    return Err(anyhow!(error.clone()));
                                }
                            },
                            ThreadEvent::ShowError(thread_error) => {
                                break Err(anyhow!(thread_error.clone()));
                            }
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
                                })?;
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
                            ThreadEvent::UsageUpdated(_) => {
                                if std::env::var("ZED_EVAL_DEBUG").is_ok() {
                                    println!("{}Event: {:#?}", log_prefix, event);
                                }
                            }
                        }
                    }
                }
            });

            thread.update(cx, |thread, cx| {
                // let context = vec![];
                // todo!
                // thread.insert_user_message(this.prompt.clone(), context, None, cx);
                thread.send_to_model(model, cx);
            })?;

            event_handler_task.await?;

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
}
