use crate::{AgentTool, Template, Templates, TerminalTool, TerminalToolInput};
use Role::*;
use anyhow::{Context as _, Result};
use client::{Client, RefreshLlmTokenListener, UserStore};
use futures::{FutureExt as _, StreamExt};
use gpui::{AppContext as _, AsyncApp, TestAppContext};
use http_client::StatusCode;
use language_model::{
    LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage, MessageContent, Role,
    SelectedModel,
};
use prompt_store::{ProjectContext, WorktreeContext};
use rand::prelude::*;
use reqwest_client::ReqwestClient;
use settings::SettingsStore;
use std::{
    fmt::{self, Display},
    path::Path,
    str::FromStr,
    sync::Arc,
    time::Duration,
};

#[derive(Clone)]
struct EvalInput {
    conversation: Vec<LanguageModelRequestMessage>,
    assertion: CommandAssertion,
}

impl EvalInput {
    fn new(conversation: Vec<LanguageModelRequestMessage>, assertion: CommandAssertion) -> Self {
        Self {
            conversation,
            assertion,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct EvalAssertionOutcome {
    score: usize,
    message: Option<String>,
}

type AssertionFn = Arc<dyn Fn(&TerminalToolInput) -> EvalAssertionOutcome + Send + Sync + 'static>;

#[derive(Clone)]
struct CommandAssertion {
    description: &'static str,
    check: AssertionFn,
}

impl CommandAssertion {
    fn new(
        description: &'static str,
        check: impl Fn(&TerminalToolInput) -> EvalAssertionOutcome + Send + Sync + 'static,
    ) -> Self {
        Self {
            description,
            check: Arc::new(check),
        }
    }

    /// Passes when the command is a git command and every git subcommand that
    /// could block on a pty (pager or editor) is guarded with the appropriate
    /// environment variable or flag.
    ///
    /// This is intentionally permissive about *which* git subcommand the model
    /// chooses — for an indirect prompt like "combine my last 3 commits", the
    /// model is free to first investigate with `git log` or jump straight to
    /// `git rebase -i`. Either is fine, as long as whatever it picks won't
    /// hang on a pager or editor.
    fn git_pty_safe(description: &'static str) -> Self {
        Self::new(description, |input| {
            let cmd = input.command.as_str();
            let words: Vec<&str> = cmd.split_whitespace().collect();

            if !words.contains(&"git") {
                return EvalAssertionOutcome {
                    score: 0,
                    message: Some(format!("Expected a `git` command, got: {cmd}")),
                };
            }

            // Subcommands that pipe their output through a pager by default,
            // and so will hang on `less` unless one of these escape hatches is
            // present somewhere in the command:
            const PAGER_SUBCMDS: &[&str] = &["log", "diff", "show", "blame"];
            const PAGER_GUARDS: &[&str] = &["--no-pager", "GIT_PAGER=cat", "PAGER=cat"];

            // Subcommands that may invoke an interactive editor and so will
            // hang unless one of these escape hatches is present:
            const EDITOR_SUBCMDS: &[&str] = &["rebase", "commit", "merge", "tag"];
            const EDITOR_GUARDS: &[&str] =
                &["GIT_EDITOR=true", "GIT_EDITOR=:", "EDITOR=true", "EDITOR=:"];

            let has_pager_guard = PAGER_GUARDS.iter().any(|guard| cmd.contains(guard));
            let has_editor_guard = EDITOR_GUARDS.iter().any(|guard| cmd.contains(guard));

            for subcmd in PAGER_SUBCMDS {
                if words.contains(subcmd) && !has_pager_guard {
                    return EvalAssertionOutcome {
                        score: 0,
                        message: Some(format!(
                            "`git {subcmd}` is missing a pager guard \
                             (one of {PAGER_GUARDS:?}). Command: {cmd}"
                        )),
                    };
                }
            }

            for subcmd in EDITOR_SUBCMDS {
                if words.contains(subcmd) && !has_editor_guard {
                    return EvalAssertionOutcome {
                        score: 0,
                        message: Some(format!(
                            "`git {subcmd}` is missing an editor guard \
                             (one of {EDITOR_GUARDS:?}). Command: {cmd}"
                        )),
                    };
                }
            }

            EvalAssertionOutcome {
                score: 100,
                message: None,
            }
        })
    }
}

struct EvalOutput {
    tool_input: TerminalToolInput,
    assertion: EvalAssertionOutcome,
    assertion_description: &'static str,
}

impl Display for EvalOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Score: {}", self.assertion.score)?;
        writeln!(f, "Assertion: {}", self.assertion_description)?;
        if let Some(message) = self.assertion.message.as_ref() {
            writeln!(f, "Message: {}", message)?;
        }
        writeln!(f, "Tool input: {:#?}", self.tool_input)?;
        Ok(())
    }
}

struct TerminalToolTest {
    model: Arc<dyn LanguageModel>,
    model_thinking_effort: Option<String>,
}

impl TerminalToolTest {
    async fn new(cx: &mut TestAppContext) -> Self {
        cx.executor().allow_parking();

        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);

            gpui_tokio::init(cx);
            let http_client = Arc::new(ReqwestClient::user_agent("agent tests").unwrap());
            cx.set_http_client(http_client);
            let client = Client::production(cx);
            let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
            language_model::init(cx);
            RefreshLlmTokenListener::register(client.clone(), user_store.clone(), cx);
            language_models::init(user_store, client, cx);
        });

        let agent_model = SelectedModel::from_str(
            &std::env::var("ZED_AGENT_MODEL")
                .unwrap_or("anthropic/claude-sonnet-4-6-latest".into()),
        )
        .unwrap();

        let authenticate_provider_tasks = cx.update(|cx| {
            LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
                registry
                    .providers()
                    .iter()
                    .map(|p| p.authenticate(cx))
                    .collect::<Vec<_>>()
            })
        });

        let model = cx
            .update(|cx| {
                cx.spawn(async move |cx| {
                    futures::future::join_all(authenticate_provider_tasks).await;
                    load_model(&agent_model, cx).await.unwrap()
                })
            })
            .await;

        let model_thinking_effort = model
            .default_effort_level()
            .map(|effort_level| effort_level.value.to_string());

        Self {
            model,
            model_thinking_effort,
        }
    }

    async fn eval(&self, mut eval: EvalInput, cx: &mut TestAppContext) -> Result<EvalOutput> {
        eval.conversation
            .last_mut()
            .context("Conversation must not be empty")?
            .cache = true;

        let tools = crate::built_in_tools().collect::<Vec<_>>();

        let system_prompt = {
            let worktrees = vec![WorktreeContext {
                root_name: "root".to_string(),
                abs_path: Path::new("/path/to/root").into(),
                rules_file: None,
            }];
            let project_context = ProjectContext::new(worktrees, Vec::default());
            let tool_names = tools
                .iter()
                .map(|tool| tool.name.clone().into())
                .collect::<Vec<_>>();
            let template = crate::SystemPromptTemplate {
                project: &project_context,
                available_tools: tool_names,
                model_name: None,
                date: chrono::Local::now().format("%Y-%m-%d").to_string(),
                user_agents_md: None,
            };
            template.render(&Templates::new())?
        };

        let has_system_prompt = eval
            .conversation
            .first()
            .is_some_and(|msg| msg.role == Role::System);
        let messages = if has_system_prompt {
            eval.conversation
        } else {
            [LanguageModelRequestMessage {
                role: Role::System,
                content: vec![MessageContent::Text(system_prompt)],
                cache: true,
                reasoning_details: None,
            }]
            .into_iter()
            .chain(eval.conversation)
            .collect::<Vec<_>>()
        };

        let request = LanguageModelRequest {
            messages,
            tools,
            thinking_allowed: true,
            thinking_effort: self.model_thinking_effort.clone(),
            ..Default::default()
        };

        let tool_input =
            retry_on_rate_limit(async || extract_tool_use(&self.model, request.clone(), cx).await)
                .await?;

        let assertion = (eval.assertion.check)(&tool_input);
        Ok(EvalOutput {
            tool_input,
            assertion,
            assertion_description: eval.assertion.description,
        })
    }
}

async fn load_model(
    selected_model: &SelectedModel,
    cx: &mut AsyncApp,
) -> Result<Arc<dyn LanguageModel>> {
    cx.update(|cx| {
        let registry = LanguageModelRegistry::read_global(cx);
        let provider = registry
            .provider(&selected_model.provider)
            .expect("Provider not found");
        provider.authenticate(cx)
    })
    .await?;
    Ok(cx.update(|cx| {
        let models = LanguageModelRegistry::read_global(cx);
        models
            .available_models(cx)
            .find(|model| {
                model.provider_id() == selected_model.provider && model.id() == selected_model.model
            })
            .unwrap_or_else(|| panic!("Model {} not found", selected_model.model.0))
    }))
}

/// Stream the model completion and extract the first complete tool use whose
/// name matches `TerminalTool::NAME`, parsed as `TerminalToolInput`.
async fn extract_tool_use(
    model: &Arc<dyn LanguageModel>,
    request: LanguageModelRequest,
    cx: &mut TestAppContext,
) -> Result<TerminalToolInput> {
    let model = model.clone();
    let events = cx
        .update(|cx| {
            let async_cx = cx.to_async();
            cx.foreground_executor()
                .spawn(async move { model.stream_completion(request, &async_cx).await })
        })
        .await
        .map_err(|err| anyhow::anyhow!("completion error: {}", err))?;

    let mut streamed_text = String::new();
    let mut stop_reason = None;
    let mut parse_errors = Vec::new();

    let mut events = events.fuse();
    while let Some(event) = events.next().await {
        match event {
            Ok(LanguageModelCompletionEvent::ToolUse(tool_use))
                if tool_use.is_input_complete && tool_use.name.as_ref() == TerminalTool::NAME =>
            {
                let input: TerminalToolInput = serde_json::from_value(tool_use.input)
                    .context("Failed to parse tool input as TerminalToolInput")?;
                return Ok(input);
            }
            Ok(LanguageModelCompletionEvent::Text(text)) => {
                if streamed_text.len() < 2_000 {
                    streamed_text.push_str(&text);
                }
            }
            Ok(LanguageModelCompletionEvent::Stop(reason)) => {
                stop_reason = Some(reason);
            }
            Ok(LanguageModelCompletionEvent::ToolUseJsonParseError {
                tool_name,
                raw_input,
                json_parse_error,
                ..
            }) if tool_name.as_ref() == TerminalTool::NAME => {
                parse_errors.push(format!("{json_parse_error}\nRaw input:\n{raw_input:?}"));
            }
            Err(err) => {
                return Err(anyhow::anyhow!("completion error: {}", err));
            }
            _ => {}
        }
    }

    let streamed_text = streamed_text.trim();
    let streamed_text_suffix = if streamed_text.is_empty() {
        String::new()
    } else {
        format!("\nStreamed text:\n{streamed_text}")
    };
    let stop_reason_suffix = stop_reason
        .map(|reason| format!("\nStop reason: {reason:?}"))
        .unwrap_or_default();
    let parse_errors_suffix = if parse_errors.is_empty() {
        String::new()
    } else {
        format!("\nTool parse errors:\n{}", parse_errors.join("\n"))
    };

    anyhow::bail!(
        "Stream ended without a terminal tool use{stop_reason_suffix}{parse_errors_suffix}{streamed_text_suffix}"
    )
}

async fn retry_on_rate_limit<R>(mut request: impl AsyncFnMut() -> Result<R>) -> Result<R> {
    const MAX_RETRIES: usize = 20;
    let mut attempt = 0;

    loop {
        attempt += 1;
        let response = request().await;

        if attempt >= MAX_RETRIES {
            return response;
        }

        let retry_delay = match &response {
            Ok(_) => None,
            Err(err) => match err.downcast_ref::<LanguageModelCompletionError>() {
                Some(err) => match &err {
                    LanguageModelCompletionError::RateLimitExceeded { retry_after, .. }
                    | LanguageModelCompletionError::ServerOverloaded { retry_after, .. } => {
                        Some(retry_after.unwrap_or(Duration::from_secs(5)))
                    }
                    LanguageModelCompletionError::UpstreamProviderError {
                        status,
                        retry_after,
                        ..
                    } => {
                        let should_retry = matches!(
                            *status,
                            StatusCode::TOO_MANY_REQUESTS | StatusCode::SERVICE_UNAVAILABLE
                        ) || status.as_u16() == 529;

                        if should_retry {
                            Some(retry_after.unwrap_or(Duration::from_secs(5)))
                        } else {
                            None
                        }
                    }
                    LanguageModelCompletionError::ApiReadResponseError { .. }
                    | LanguageModelCompletionError::ApiInternalServerError { .. }
                    | LanguageModelCompletionError::HttpSend { .. } => {
                        Some(Duration::from_secs(2_u64.pow((attempt - 1) as u32).min(30)))
                    }
                    _ => None,
                },
                _ => None,
            },
        };

        if let Some(retry_after) = retry_delay {
            let jitter = retry_after.mul_f64(rand::rng().random_range(0.0..1.0));
            eprintln!("Attempt #{attempt}: Retry after {retry_after:?} + jitter of {jitter:?}");
            #[allow(clippy::disallowed_methods)]
            async_io::Timer::after(retry_after + jitter).await;
        } else {
            return response;
        }
    }
}

fn run_eval(eval: EvalInput) -> eval_utils::EvalOutput<()> {
    super::run_gpui_eval(
        |cx| {
            async move {
                let test = TerminalToolTest::new(cx).await;
                let result = test.eval(eval, cx).await;
                drop(test);
                cx.run_until_parked();
                result
            }
            .boxed_local()
        },
        |output| {
            if output.assertion.score < 80 {
                eval_utils::OutcomeKind::Failed
            } else {
                eval_utils::OutcomeKind::Passed
            }
        },
    )
}

fn message(
    role: Role,
    contents: impl IntoIterator<Item = MessageContent>,
) -> LanguageModelRequestMessage {
    LanguageModelRequestMessage {
        role,
        content: contents.into_iter().collect(),
        cache: false,
        reasoning_details: None,
    }
}

fn text(text: impl Into<String>) -> MessageContent {
    MessageContent::Text(text.into())
}

#[test]
#[cfg_attr(not(feature = "unit-eval"), ignore)]
fn eval_git_log_uses_no_pager() {
    eval_utils::eval(100, 0.95, eval_utils::NoProcessor, move || {
        run_eval(EvalInput::new(
            vec![message(
                User,
                [text(indoc::indoc! {"
                    Use the terminal tool to show me the most recent 3 commits
                    on the current branch (subject lines only is fine).
                "})],
            )],
            CommandAssertion::git_pty_safe(
                "`git log`-style prompt produces a pty-safe git command",
            ),
        ))
    });
}

#[test]
#[cfg_attr(not(feature = "unit-eval"), ignore)]
fn eval_git_rebase_sets_git_editor() {
    eval_utils::eval(100, 0.95, eval_utils::NoProcessor, move || {
        run_eval(EvalInput::new(
            vec![message(
                User,
                [text(indoc::indoc! {"
                    Use the terminal tool to rebase the current branch onto
                    `origin/main`.
                "})],
            )],
            CommandAssertion::git_pty_safe("`git rebase` prompt produces a pty-safe git command"),
        ))
    });
}

#[test]
#[cfg_attr(not(feature = "unit-eval"), ignore)]
fn eval_git_rebase_implied_sets_git_editor() {
    eval_utils::eval(100, 0.95, eval_utils::NoProcessor, move || {
        run_eval(EvalInput::new(
            vec![message(
                User,
                [text(indoc::indoc! {"
                    My branch has 3 small commits that I'd like to combine
                    into a single clean commit before merging. Help me do
                    that with the terminal tool.
                "})],
            )],
            CommandAssertion::git_pty_safe("indirect prompt produces a pty-safe git command"),
        ))
    });
}
