use crate::tools::streaming_edit_file_tool::*;
use crate::{
    AgentTool, ContextServerRegistry, EditFileTool, GrepTool, GrepToolInput, ListDirectoryTool,
    ListDirectoryToolInput, ReadFileTool, ReadFileToolInput, StreamingEditFileTool, Template,
    Templates, Thread, ToolCallEventStream, ToolInput,
};
use Role::*;
use anyhow::{Context as _, Result};
use client::{Client, RefreshLlmTokenListener, UserStore};
use fs::FakeFs;
use futures::{FutureExt, StreamExt, future::LocalBoxFuture};
use gpui::{AppContext as _, AsyncApp, Entity, TestAppContext, UpdateGlobal as _};
use http_client::StatusCode;
use language::language_settings::FormatOnSave;
use language_model::{
    LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage,
    LanguageModelRequestTool, LanguageModelToolResult, LanguageModelToolResultContent,
    LanguageModelToolSchemaFormat, LanguageModelToolUse, LanguageModelToolUseId, MessageContent,
    Role, SelectedModel,
};
use project::Project;
use prompt_store::{ProjectContext, WorktreeContext};
use rand::prelude::*;
use reqwest_client::ReqwestClient;
use serde::Serialize;
use serde_json::json;
use settings::SettingsStore;
use std::{
    fmt::{self, Display},
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
    time::Duration,
};
use util::path;

#[derive(Serialize)]
struct DiffJudgeTemplate {
    diff: String,
    assertions: &'static str,
}

impl Template for DiffJudgeTemplate {
    const TEMPLATE_NAME: &'static str = "diff_judge.hbs";
}

#[derive(Clone)]
struct EvalInput {
    conversation: Vec<LanguageModelRequestMessage>,
    input_file_path: PathBuf,
    input_content: Option<String>,
    assertion: EvalAssertion,
}

impl EvalInput {
    fn new(
        conversation: Vec<LanguageModelRequestMessage>,
        input_file_path: impl Into<PathBuf>,
        input_content: Option<String>,
        assertion: EvalAssertion,
    ) -> Self {
        EvalInput {
            conversation,
            input_file_path: input_file_path.into(),
            input_content,
            assertion,
        }
    }
}

#[derive(Clone)]
struct EvalSample {
    text_before: String,
    text_after: String,
    tool_input: StreamingEditFileToolInput,
    diff: String,
}

trait AssertionFn: 'static + Send + Sync {
    fn assert<'a>(
        &'a self,
        sample: &'a EvalSample,
        judge_model: Arc<dyn LanguageModel>,
        cx: &'a mut TestAppContext,
    ) -> LocalBoxFuture<'a, Result<EvalAssertionOutcome>>;
}

impl<F> AssertionFn for F
where
    F: 'static
        + Send
        + Sync
        + AsyncFn(
            &EvalSample,
            Arc<dyn LanguageModel>,
            &mut TestAppContext,
        ) -> Result<EvalAssertionOutcome>,
{
    fn assert<'a>(
        &'a self,
        sample: &'a EvalSample,
        judge_model: Arc<dyn LanguageModel>,
        cx: &'a mut TestAppContext,
    ) -> LocalBoxFuture<'a, Result<EvalAssertionOutcome>> {
        (self)(sample, judge_model, cx).boxed_local()
    }
}

#[derive(Clone)]
struct EvalAssertion(Arc<dyn AssertionFn>);

impl EvalAssertion {
    fn new<F>(f: F) -> Self
    where
        F: 'static
            + Send
            + Sync
            + AsyncFn(
                &EvalSample,
                Arc<dyn LanguageModel>,
                &mut TestAppContext,
            ) -> Result<EvalAssertionOutcome>,
    {
        EvalAssertion(Arc::new(f))
    }

    fn assert_eq(expected: impl Into<String>) -> Self {
        let expected = expected.into();
        Self::new(async move |sample, _judge, _cx| {
            Ok(EvalAssertionOutcome {
                score: if strip_empty_lines(&sample.text_after) == strip_empty_lines(&expected) {
                    100
                } else {
                    0
                },
                message: None,
            })
        })
    }

    fn assert_diff_any(expected_diffs: Vec<impl Into<String>>) -> Self {
        let expected_diffs: Vec<String> = expected_diffs.into_iter().map(Into::into).collect();
        Self::new(async move |sample, _judge, _cx| {
            let matches = expected_diffs.iter().any(|possible_diff| {
                language::apply_diff_patch(&sample.text_before, possible_diff)
                    .map(|expected| {
                        strip_empty_lines(&expected) == strip_empty_lines(&sample.text_after)
                    })
                    .unwrap_or(false)
            });

            Ok(EvalAssertionOutcome {
                score: if matches { 100 } else { 0 },
                message: None,
            })
        })
    }

    fn judge_diff(assertions: &'static str) -> Self {
        Self::new(async move |sample, judge, cx| {
            let prompt = DiffJudgeTemplate {
                diff: sample.diff.clone(),
                assertions,
            }
            .render(&Templates::new())
            .context("Failed to render diff judge template")?;

            let request = LanguageModelRequest {
                messages: vec![LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![prompt.into()],
                    cache: false,
                    reasoning_details: None,
                }],
                thinking_allowed: true,
                thinking_effort: judge
                    .default_effort_level()
                    .map(|effort_level| effort_level.value.to_string()),
                ..Default::default()
            };
            let mut response = retry_on_rate_limit(async || {
                Ok(judge
                    .stream_completion_text(request.clone(), &cx.to_async())
                    .await?)
            })
            .await?;
            let mut output = String::new();
            while let Some(chunk) = response.stream.next().await {
                let chunk = chunk?;
                output.push_str(&chunk);
            }

            let re = regex::Regex::new(r"<score>(\d+)</score>")
                .context("Failed to compile score regex")?;
            if let Some(captures) = re.captures(&output)
                && let Some(score_match) = captures.get(1)
            {
                let score = score_match.as_str().parse().unwrap_or(0);
                return Ok(EvalAssertionOutcome {
                    score,
                    message: Some(output),
                });
            }

            anyhow::bail!("No score found in response. Raw output: {output}");
        })
    }

    async fn run(
        &self,
        input: &EvalSample,
        judge_model: Arc<dyn LanguageModel>,
        cx: &mut TestAppContext,
    ) -> Result<EvalAssertionOutcome> {
        self.0.assert(input, judge_model, cx).await
    }
}

#[derive(Clone)]
struct StreamingEditEvalOutput {
    sample: EvalSample,
    assertion: EvalAssertionOutcome,
}

impl Display for StreamingEditEvalOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Score: {:?}", self.assertion.score)?;
        if let Some(message) = self.assertion.message.as_ref() {
            writeln!(f, "Message: {}", message)?;
        }
        writeln!(f, "Diff:\n{}", self.sample.diff)?;
        writeln!(f, "Tool Input:\n{:#?}", self.sample.tool_input)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct EvalAssertionOutcome {
    score: usize,
    message: Option<String>,
}

struct StreamingEditToolTest {
    fs: Arc<FakeFs>,
    project: Entity<Project>,
    model: Arc<dyn LanguageModel>,
    judge_model: Arc<dyn LanguageModel>,
    model_thinking_effort: Option<String>,
}

impl StreamingEditToolTest {
    async fn new(cx: &mut TestAppContext) -> Self {
        cx.executor().allow_parking();

        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            SettingsStore::update_global(cx, |store: &mut SettingsStore, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .project
                        .all_languages
                        .defaults
                        .ensure_final_newline_on_save = Some(false);
                    settings.project.all_languages.defaults.format_on_save =
                        Some(FormatOnSave::Off);
                });
            });

            gpui_tokio::init(cx);
            let http_client = Arc::new(ReqwestClient::user_agent("agent tests").unwrap());
            cx.set_http_client(http_client);
            let client = Client::production(cx);
            let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
            language_model::init(cx);
            RefreshLlmTokenListener::register(client.clone(), user_store.clone(), cx);
            language_models::init(user_store, client, cx);
        });

        fs.insert_tree("/root", json!({})).await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let agent_model = SelectedModel::from_str(
            &std::env::var("ZED_AGENT_MODEL")
                .unwrap_or("anthropic/claude-sonnet-4-6-latest".into()),
        )
        .unwrap();
        let judge_model = SelectedModel::from_str(
            &std::env::var("ZED_JUDGE_MODEL")
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
        let (model, judge_model) = cx
            .update(|cx| {
                cx.spawn(async move |cx| {
                    futures::future::join_all(authenticate_provider_tasks).await;
                    let model = Self::load_model(&agent_model, cx).await;
                    let judge_model = Self::load_model(&judge_model, cx).await;
                    (model.unwrap(), judge_model.unwrap())
                })
            })
            .await;

        let model_thinking_effort = model
            .default_effort_level()
            .map(|effort_level| effort_level.value.to_string());

        Self {
            fs,
            project,
            model,
            judge_model,
            model_thinking_effort,
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
                    model.provider_id() == selected_model.provider
                        && model.id() == selected_model.model
                })
                .unwrap_or_else(|| panic!("Model {} not found", selected_model.model.0))
        }))
    }

    /// Build the tool definitions for the model, replacing `edit_file` with the
    /// streaming edit file tool schema. In production the streaming tool is
    /// exposed under the name `"edit_file"` (see `Thread::enabled_tools`), so
    /// the model has never seen the name `"streaming_edit_file"`.
    fn build_tools() -> Vec<LanguageModelRequestTool> {
        let mut tools: Vec<LanguageModelRequestTool> = crate::built_in_tools()
            .filter(|tool| tool.name != EditFileTool::NAME)
            .collect();
        tools.push(LanguageModelRequestTool {
            name: EditFileTool::NAME.to_string(),
            description: StreamingEditFileTool::description().to_string(),
            input_schema: StreamingEditFileTool::input_schema(
                LanguageModelToolSchemaFormat::JsonSchema,
            )
            .to_value(),
            use_input_streaming: StreamingEditFileTool::supports_input_streaming(),
        });
        tools
    }

    async fn eval(
        &self,
        mut eval: EvalInput,
        cx: &mut TestAppContext,
    ) -> Result<StreamingEditEvalOutput> {
        eval.conversation
            .last_mut()
            .context("Conversation must not be empty")?
            .cache = true;

        // Populate the FakeFs so `resolve_path` / `entry_for_path` can find
        // the file in the worktree.
        if let Some(input_content) = eval.input_content.as_deref() {
            let abs_path = Path::new("/root").join(
                eval.input_file_path
                    .strip_prefix("root")
                    .unwrap_or(&eval.input_file_path),
            );
            self.fs.insert_file(&abs_path, input_content.into()).await;

            // Wait for the worktree to pick up the new file.
            cx.run_until_parked();
        }

        let tools = Self::build_tools();

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
            };
            let templates = Templates::new();
            template.render(&templates)?
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

        // The model will call the tool as "edit_file" (the production-visible
        // name), but the schema is from StreamingEditFileTool.
        let tool_input =
            retry_on_rate_limit(async || self.extract_tool_use(request.clone(), cx).await).await?;

        let language_registry = self
            .project
            .read_with(cx, |project, _cx| project.languages().clone());

        let context_server_registry = cx
            .new(|cx| ContextServerRegistry::new(self.project.read(cx).context_server_store(), cx));
        let thread = cx.new(|cx| {
            Thread::new(
                self.project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(self.model.clone()),
                cx,
            )
        });
        let action_log = thread.read_with(cx, |thread, _| thread.action_log().clone());

        let tool = Arc::new(StreamingEditFileTool::new(
            self.project.clone(),
            thread.downgrade(),
            action_log,
            language_registry,
        ));

        let result = cx
            .update(|cx| {
                tool.clone().run(
                    ToolInput::resolved(tool_input.clone()),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        let output = match result {
            Ok(output) => output,
            Err(output) => {
                anyhow::bail!("Tool returned error: {}", output);
            }
        };

        let StreamingEditFileToolOutput::Success { new_text, .. } = &output else {
            anyhow::bail!("Tool returned error output: {}", output);
        };

        let sample = EvalSample {
            tool_input,
            diff: language::unified_diff(
                eval.input_content.as_deref().unwrap_or_default(),
                new_text,
            ),
            text_before: eval.input_content.unwrap_or_default(),
            text_after: new_text.clone(),
        };

        let assertion = eval
            .assertion
            .run(&sample, self.judge_model.clone(), cx)
            .await?;

        Ok(StreamingEditEvalOutput { assertion, sample })
    }

    /// Stream the model completion and extract the first complete tool use
    /// whose name matches `EditFileTool::NAME` (the production-visible name
    /// for the streaming edit tool), parsed as `StreamingEditFileToolInput`.
    async fn extract_tool_use(
        &self,
        request: LanguageModelRequest,
        cx: &mut TestAppContext,
    ) -> Result<StreamingEditFileToolInput> {
        let model = self.model.clone();
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
                    if tool_use.is_input_complete
                        && tool_use.name.as_ref() == EditFileTool::NAME =>
                {
                    let input: StreamingEditFileToolInput = serde_json::from_value(tool_use.input)
                        .context("Failed to parse tool input as StreamingEditFileToolInput")?;
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
                }) if tool_name.as_ref() == EditFileTool::NAME => {
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
            "Stream ended without an edit_file tool use{stop_reason_suffix}{parse_errors_suffix}{streamed_text_suffix}"
        )
    }
}

fn run_eval(eval: EvalInput) -> eval_utils::EvalOutput<()> {
    let dispatcher = gpui::TestDispatcher::new(rand::random());
    let mut cx = TestAppContext::build(dispatcher, None);
    let foreground_executor = cx.foreground_executor().clone();
    let result = foreground_executor.block_test(async {
        let test = StreamingEditToolTest::new(&mut cx).await;
        let result = test.eval(eval, &mut cx).await;
        drop(test);
        cx.run_until_parked();
        result
    });
    cx.quit();
    match result {
        Ok(output) => eval_utils::EvalOutput {
            data: output.to_string(),
            outcome: if output.assertion.score < 80 {
                eval_utils::OutcomeKind::Failed
            } else {
                eval_utils::OutcomeKind::Passed
            },
            metadata: (),
        },
        Err(err) => eval_utils::EvalOutput {
            data: format!("{err:?}"),
            outcome: eval_utils::OutcomeKind::Error,
            metadata: (),
        },
    }
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

fn lines(input: &str, range: std::ops::Range<usize>) -> String {
    input
        .lines()
        .skip(range.start)
        .take(range.len())
        .collect::<Vec<_>>()
        .join("\n")
}

fn tool_use(
    id: impl Into<Arc<str>>,
    name: impl Into<Arc<str>>,
    input: impl Serialize,
) -> MessageContent {
    MessageContent::ToolUse(LanguageModelToolUse {
        id: LanguageModelToolUseId::from(id.into()),
        name: name.into(),
        raw_input: serde_json::to_string_pretty(&input).unwrap(),
        input: serde_json::to_value(input).unwrap(),
        is_input_complete: true,
        thought_signature: None,
    })
}

fn tool_result(
    id: impl Into<Arc<str>>,
    name: impl Into<Arc<str>>,
    result: impl Into<Arc<str>>,
) -> MessageContent {
    MessageContent::ToolResult(LanguageModelToolResult {
        tool_use_id: LanguageModelToolUseId::from(id.into()),
        tool_name: name.into(),
        is_error: false,
        content: LanguageModelToolResultContent::Text(result.into()),
        output: None,
    })
}

fn strip_empty_lines(text: &str) -> String {
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
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

#[test]
#[cfg_attr(not(feature = "unit-eval"), ignore)]
fn eval_delete_function() {
    let input_file_path = "root/blame.rs";
    let input_file_content = include_str!("fixtures/delete_run_git_blame/before.rs");
    let output_file_content = include_str!("fixtures/delete_run_git_blame/after.rs");
    let possible_diffs = vec![
        language::unified_diff(input_file_content, output_file_content),
        language::unified_diff(
            input_file_content,
            &output_file_content
                .replace(
                    "const GIT_BLAME_NO_COMMIT_ERROR: &str = \"fatal: no such ref: HEAD\";\n",
                    "",
                )
                .replace(
                    "const GIT_BLAME_NO_PATH: &str = \"fatal: no such path\";\n",
                    "",
                ),
        ),
    ];

    eval_utils::eval(100, 0.95, eval_utils::NoProcessor, move || {
        run_eval(EvalInput::new(
            vec![
                message(
                    User,
                    [text(indoc::formatdoc! {"
                        Read the `{input_file_path}` file and delete `run_git_blame`. Just that
                        one function, not its usages.
                    "})],
                ),
                message(
                    Assistant,
                    [tool_use(
                        "tool_1",
                        ReadFileTool::NAME,
                        ReadFileToolInput {
                            path: input_file_path.into(),
                            start_line: None,
                            end_line: None,
                        },
                    )],
                ),
                message(
                    User,
                    [tool_result(
                        "tool_1",
                        ReadFileTool::NAME,
                        input_file_content,
                    )],
                ),
            ],
            input_file_path,
            Some(input_file_content.into()),
            EvalAssertion::assert_diff_any(possible_diffs.clone()),
        ))
    });
}

#[test]
#[cfg_attr(not(feature = "unit-eval"), ignore)]
fn eval_extract_handle_command_output() {
    let input_file_path = "root/blame.rs";
    let input_file_content = include_str!("fixtures/extract_handle_command_output/before.rs");
    let possible_diffs = vec![
        include_str!("fixtures/extract_handle_command_output/possible-01.diff"),
        include_str!("fixtures/extract_handle_command_output/possible-02.diff"),
        include_str!("fixtures/extract_handle_command_output/possible-03.diff"),
        include_str!("fixtures/extract_handle_command_output/possible-04.diff"),
        include_str!("fixtures/extract_handle_command_output/possible-05.diff"),
        include_str!("fixtures/extract_handle_command_output/possible-06.diff"),
        include_str!("fixtures/extract_handle_command_output/possible-07.diff"),
        include_str!("fixtures/extract_handle_command_output/possible-08.diff"),
        include_str!("fixtures/extract_handle_command_output/possible-09.diff"),
    ];

    eval_utils::eval(100, 0.95, eval_utils::NoProcessor, move || {
        run_eval(EvalInput::new(
            vec![
                message(
                    User,
                    [text(indoc::formatdoc! {"
                        Read the `{input_file_path}` file and extract a method in
                        the final stanza of `run_git_blame` to deal with command failures,
                        call it `handle_command_output` and take the std::process::Output as the only parameter.
                        Do not document the method and do not add any comments.

                        Add it right next to `run_git_blame` and copy it verbatim from `run_git_blame`.
                    "})],
                ),
                message(
                    Assistant,
                    [tool_use(
                        "tool_1",
                        ReadFileTool::NAME,
                        ReadFileToolInput {
                            path: input_file_path.into(),
                            start_line: None,
                            end_line: None,
                        },
                    )],
                ),
                message(
                    User,
                    [tool_result(
                        "tool_1",
                        ReadFileTool::NAME,
                        input_file_content,
                    )],
                ),
            ],
            input_file_path,
            Some(input_file_content.into()),
            EvalAssertion::assert_diff_any(possible_diffs.clone()),
        ))
    });
}

#[test]
#[cfg_attr(not(feature = "unit-eval"), ignore)]
fn eval_translate_doc_comments() {
    let input_file_path = "root/canvas.rs";
    let input_file_content = include_str!("fixtures/translate_doc_comments/before.rs");

    eval_utils::eval(200, 1., eval_utils::NoProcessor, move || {
        run_eval(EvalInput::new(
            vec![
                message(
                    User,
                    [text(indoc::formatdoc! {"
                        Read the `{input_file_path}` file and edit it (without overwriting it),
                        translating all the doc comments to italian.
                    "})],
                ),
                message(
                    Assistant,
                    [tool_use(
                        "tool_1",
                        ReadFileTool::NAME,
                        ReadFileToolInput {
                            path: input_file_path.into(),
                            start_line: None,
                            end_line: None,
                        },
                    )],
                ),
                message(
                    User,
                    [tool_result(
                        "tool_1",
                        ReadFileTool::NAME,
                        input_file_content,
                    )],
                ),
            ],
            input_file_path,
            Some(input_file_content.into()),
            EvalAssertion::judge_diff("Doc comments were translated to Italian"),
        ))
    });
}

#[test]
#[cfg_attr(not(feature = "unit-eval"), ignore)]
fn eval_use_wasi_sdk_in_compile_parser_to_wasm() {
    let input_file_path = "root/lib.rs";
    let input_file_content =
        include_str!("fixtures/use_wasi_sdk_in_compile_parser_to_wasm/before.rs");

    eval_utils::eval(100, 0.95, eval_utils::NoProcessor, move || {
        run_eval(EvalInput::new(
            vec![
                message(
                    User,
                    [text(indoc::formatdoc! {"
                        Read the `{input_file_path}` file and change `compile_parser_to_wasm` to use `wasi-sdk` instead of emscripten.
                        Use `ureq` to download the SDK for the current platform and architecture.
                        Extract the archive into a sibling of `lib` inside the `tree-sitter` directory in the cache_dir.
                        Compile the parser to wasm using the `bin/clang` executable (or `bin/clang.exe` on windows)
                        that's inside of the archive.
                        Don't re-download the SDK if that executable already exists.

                        Use these clang flags: -fPIC -shared -Os -Wl,--export=tree_sitter_{{language_name}}

                        Here are the available wasi-sdk assets:
                        - wasi-sdk-25.0-x86_64-macos.tar.gz
                        - wasi-sdk-25.0-arm64-macos.tar.gz
                        - wasi-sdk-25.0-x86_64-linux.tar.gz
                        - wasi-sdk-25.0-arm64-linux.tar.gz
                        - wasi-sdk-25.0-x86_64-linux.tar.gz
                        - wasi-sdk-25.0-arm64-linux.tar.gz
                        - wasi-sdk-25.0-x86_64-windows.tar.gz
                    "})],
                ),
                message(
                    Assistant,
                    [tool_use(
                        "tool_1",
                        ReadFileTool::NAME,
                        ReadFileToolInput {
                            path: input_file_path.into(),
                            start_line: Some(971),
                            end_line: Some(1050),
                        },
                    )],
                ),
                message(
                    User,
                    [tool_result(
                        "tool_1",
                        ReadFileTool::NAME,
                        lines(input_file_content, 971..1050),
                    )],
                ),
                message(
                    Assistant,
                    [tool_use(
                        "tool_2",
                        ReadFileTool::NAME,
                        ReadFileToolInput {
                            path: input_file_path.into(),
                            start_line: Some(1050),
                            end_line: Some(1100),
                        },
                    )],
                ),
                message(
                    User,
                    [tool_result(
                        "tool_2",
                        ReadFileTool::NAME,
                        lines(input_file_content, 1050..1100),
                    )],
                ),
                message(
                    Assistant,
                    [tool_use(
                        "tool_3",
                        ReadFileTool::NAME,
                        ReadFileToolInput {
                            path: input_file_path.into(),
                            start_line: Some(1100),
                            end_line: Some(1150),
                        },
                    )],
                ),
                message(
                    User,
                    [tool_result(
                        "tool_3",
                        ReadFileTool::NAME,
                        lines(input_file_content, 1100..1150),
                    )],
                ),
            ],
            input_file_path,
            Some(input_file_content.into()),
            EvalAssertion::judge_diff(indoc::indoc! {"
                    - The compile_parser_to_wasm method has been changed to use wasi-sdk
                    - ureq is used to download the SDK for current platform and architecture
                "}),
        ))
    });
}

#[test]
#[cfg_attr(not(feature = "unit-eval"), ignore)]
fn eval_disable_cursor_blinking() {
    let input_file_path = "root/editor.rs";
    let input_file_content = include_str!("fixtures/disable_cursor_blinking/before.rs");
    let possible_diffs = vec![
        include_str!("fixtures/disable_cursor_blinking/possible-01.diff"),
        include_str!("fixtures/disable_cursor_blinking/possible-02.diff"),
        include_str!("fixtures/disable_cursor_blinking/possible-03.diff"),
        include_str!("fixtures/disable_cursor_blinking/possible-04.diff"),
    ];

    eval_utils::eval(100, 0.51, eval_utils::NoProcessor, move || {
        run_eval(EvalInput::new(
            vec![
                message(User, [text("Let's research how to cursor blinking works.")]),
                message(
                    Assistant,
                    [tool_use(
                        "tool_1",
                        GrepTool::NAME,
                        GrepToolInput {
                            regex: "blink".into(),
                            include_pattern: None,
                            offset: 0,
                            case_sensitive: false,
                        },
                    )],
                ),
                message(
                    User,
                    [tool_result(
                        "tool_1",
                        GrepTool::NAME,
                        [
                            lines(input_file_content, 100..400),
                            lines(input_file_content, 800..1300),
                            lines(input_file_content, 1600..2000),
                            lines(input_file_content, 5000..5500),
                            lines(input_file_content, 8000..9000),
                            lines(input_file_content, 18455..18470),
                            lines(input_file_content, 20000..20500),
                            lines(input_file_content, 21000..21300),
                        ]
                        .join("Match found:\n\n"),
                    )],
                ),
                message(
                    User,
                    [text(indoc::indoc! {"
                            Comment out the lines that interact with the BlinkManager.
                            Keep the outer `update` blocks, but comments everything that's inside (including if statements).
                            Don't add additional comments.
                        "})],
                ),
            ],
            input_file_path,
            Some(input_file_content.into()),
            EvalAssertion::assert_diff_any(possible_diffs.clone()),
        ))
    });
}

#[test]
#[cfg_attr(not(feature = "unit-eval"), ignore)]
fn eval_from_pixels_constructor() {
    let input_file_path = "root/canvas.rs";
    let input_file_content = include_str!("fixtures/from_pixels_constructor/before.rs");

    eval_utils::eval(100, 0.95, eval_utils::NoProcessor, move || {
        run_eval(EvalInput::new(
            vec![
                message(
                    User,
                    [text(indoc::indoc! {"
                            Introduce a new `from_pixels` constructor in Canvas and
                            also add tests for it in the same file.
                        "})],
                ),
                message(
                    Assistant,
                    [tool_use(
                        "tool_1",
                        ReadFileTool::NAME,
                        ReadFileToolInput {
                            path: input_file_path.into(),
                            start_line: None,
                            end_line: None,
                        },
                    )],
                ),
                message(
                    User,
                    [tool_result(
                        "tool_1",
                        ReadFileTool::NAME,
                        input_file_content,
                    )],
                ),
                message(
                    Assistant,
                    [tool_use(
                        "tool_2",
                        GrepTool::NAME,
                        GrepToolInput {
                            regex: "mod\\s+tests".into(),
                            include_pattern: Some("font-kit/src/canvas.rs".into()),
                            offset: 0,
                            case_sensitive: false,
                        },
                    )],
                ),
                message(
                    User,
                    [tool_result("tool_2", GrepTool::NAME, "No matches found")],
                ),
                message(
                    Assistant,
                    [tool_use(
                        "tool_3",
                        GrepTool::NAME,
                        GrepToolInput {
                            regex: "mod\\s+tests".into(),
                            include_pattern: Some("font-kit/src/**/*.rs".into()),
                            offset: 0,
                            case_sensitive: false,
                        },
                    )],
                ),
                message(
                    User,
                    [tool_result("tool_3", GrepTool::NAME, "No matches found")],
                ),
                message(
                    Assistant,
                    [tool_use(
                        "tool_4",
                        GrepTool::NAME,
                        GrepToolInput {
                            regex: "#\\[test\\]".into(),
                            include_pattern: Some("font-kit/src/**/*.rs".into()),
                            offset: 0,
                            case_sensitive: false,
                        },
                    )],
                ),
                message(
                    User,
                    [tool_result(
                        "tool_4",
                        GrepTool::NAME,
                        indoc::indoc! {"
                                Found 6 matches:

                                ## Matches in font-kit/src/loaders/core_text.rs

                                ### mod test › L926-936
                                ```
                                mod test {
                                    use super::Font;
                                    use crate::properties::{Stretch, Weight};

                                    #[cfg(feature = \"source\")]
                                    use crate::source::SystemSource;

                                    static TEST_FONT_POSTSCRIPT_NAME: &'static str = \"ArialMT\";

                                    #[cfg(feature = \"source\")]
                                    #[test]
                                ```

                                55 lines remaining in ancestor node. Read the file to see all.

                                ### mod test › L947-951
                                ```
                                    }

                                    #[test]
                                    fn test_core_text_to_css_font_weight() {
                                        // Exact matches
                                ```

                                ### mod test › L959-963
                                ```
                                    }

                                    #[test]
                                    fn test_core_text_to_css_font_stretch() {
                                        // Exact matches
                                ```

                                ## Matches in font-kit/src/loaders/freetype.rs

                                ### mod test › L1238-1248
                                ```
                                mod test {
                                    use crate::loaders::freetype::Font;

                                    static PCF_FONT_PATH: &str = \"resources/tests/times-roman-pcf/timR12.pcf\";
                                    static PCF_FONT_POSTSCRIPT_NAME: &str = \"Times-Roman\";

                                    #[test]
                                    fn get_pcf_postscript_name() {
                                        let font = Font::from_path(PCF_FONT_PATH, 0).unwrap();
                                        assert_eq!(font.postscript_name().unwrap(), PCF_FONT_POSTSCRIPT_NAME);
                                    }
                                ```

                                1 lines remaining in ancestor node. Read the file to see all.

                                ## Matches in font-kit/src/sources/core_text.rs

                                ### mod test › L265-275
                                ```
                                mod test {
                                    use crate::properties::{Stretch, Weight};

                                    #[test]
                                    fn test_css_to_core_text_font_weight() {
                                        // Exact matches
                                        assert_eq!(super::css_to_core_text_font_weight(Weight(100.0)), -0.7);
                                        assert_eq!(super::css_to_core_text_font_weight(Weight(400.0)), 0.0);
                                        assert_eq!(super::css_to_core_text_font_weight(Weight(700.0)), 0.4);
                                        assert_eq!(super::css_to_core_text_font_weight(Weight(900.0)), 0.8);

                                ```

                                27 lines remaining in ancestor node. Read the file to see all.

                                ### mod test › L278-282
                                ```
                                    }

                                    #[test]
                                    fn test_css_to_core_text_font_stretch() {
                                        // Exact matches
                                ```
                            "},
                    )],
                ),
            ],
            input_file_path,
            Some(input_file_content.into()),
            EvalAssertion::judge_diff(indoc::indoc! {"
                        - The diff contains a new `from_pixels` constructor
                        - The diff contains new tests for the `from_pixels` constructor
                    "}),
        ))
    });
}

#[test]
#[cfg_attr(not(feature = "unit-eval"), ignore)]
fn eval_zode() {
    let input_file_path = "root/zode.py";
    let input_content = None;

    eval_utils::eval(50, 1., eval_utils::NoProcessor, move || {
        run_eval(EvalInput::new(
            vec![
                message(User, [text(include_str!("fixtures/zode/prompt.md"))]),
                message(
                    Assistant,
                    [
                        tool_use(
                            "tool_1",
                            ReadFileTool::NAME,
                            ReadFileToolInput {
                                path: "root/eval/react.py".into(),
                                start_line: None,
                                end_line: None,
                            },
                        ),
                        tool_use(
                            "tool_2",
                            ReadFileTool::NAME,
                            ReadFileToolInput {
                                path: "root/eval/react_test.py".into(),
                                start_line: None,
                                end_line: None,
                            },
                        ),
                    ],
                ),
                message(
                    User,
                    [
                        tool_result(
                            "tool_1",
                            ReadFileTool::NAME,
                            include_str!("fixtures/zode/react.py"),
                        ),
                        tool_result(
                            "tool_2",
                            ReadFileTool::NAME,
                            include_str!("fixtures/zode/react_test.py"),
                        ),
                    ],
                ),
            ],
            input_file_path,
            input_content.clone(),
            EvalAssertion::new(async move |sample, _, _cx| {
                let invalid_starts = [' ', '`', '\n'];
                let mut message = String::new();
                for start in invalid_starts {
                    if sample.text_after.starts_with(start) {
                        message.push_str(&format!("The sample starts with a {:?}\n", start));
                        break;
                    }
                }
                message.pop();

                if message.is_empty() {
                    Ok(EvalAssertionOutcome {
                        score: 100,
                        message: None,
                    })
                } else {
                    Ok(EvalAssertionOutcome {
                        score: 0,
                        message: Some(message),
                    })
                }
            }),
        ))
    });
}

#[test]
#[cfg_attr(not(feature = "unit-eval"), ignore)]
fn eval_add_overwrite_test() {
    let input_file_path = "root/action_log.rs";
    let input_file_content = include_str!("fixtures/add_overwrite_test/before.rs");

    eval_utils::eval(200, 0.5, eval_utils::NoProcessor, move || {
        run_eval(EvalInput::new(
            vec![
                message(
                    User,
                    [text(indoc::indoc! {"
                            Introduce a new test in `action_log.rs` to test overwriting a file.
                            That is, a file already exists, but we call `buffer_created` as if the file were new.
                            Take inspiration from all the other tests in the file.
                        "})],
                ),
                message(
                    Assistant,
                    [tool_use(
                        "tool_1",
                        ReadFileTool::NAME,
                        ReadFileToolInput {
                            path: input_file_path.into(),
                            start_line: None,
                            end_line: None,
                        },
                    )],
                ),
                message(
                    User,
                    [tool_result(
                        "tool_1",
                        ReadFileTool::NAME,
                        indoc::indoc! {"
                                pub struct ActionLog [L13-20]
                                 tracked_buffers [L15]
                                 edited_since_project_diagnostics_check [L17]
                                 project [L19]
                                impl ActionLog [L22-498]
                                 pub fn new [L24-30]
                                 pub fn project [L32-34]
                                 pub fn checked_project_diagnostics [L37-39]
                                 pub fn has_edited_files_since_project_diagnostics_check [L42-44]
                                 fn track_buffer_internal [L46-101]
                                 fn handle_buffer_event [L103-116]
                                 fn handle_buffer_edited [L118-123]
                                 fn handle_buffer_file_changed [L125-158]
                                 async fn maintain_diff [L160-264]
                                 pub fn buffer_read [L267-269]
                                 pub fn buffer_created [L272-276]
                                 pub fn buffer_edited [L279-287]
                                 pub fn will_delete_buffer [L289-304]
                                 pub fn keep_edits_in_range [L306-364]
                                 pub fn reject_edits_in_ranges [L366-459]
                                 pub fn keep_all_edits [L461-473]
                                 pub fn changed_buffers [L476-482]
                                 pub fn stale_buffers [L485-497]
                                fn apply_non_conflicting_edits [L500-561]
                                fn diff_snapshots [L563-585]
                                fn point_to_row_edit [L587-614]
                                enum ChangeAuthor [L617-620]
                                 User [L618]
                                 Agent [L619]
                                enum TrackedBufferStatus [L623-627]
                                 Created [L624]
                                 Modified [L625]
                                 Deleted [L626]
                                struct TrackedBuffer [L629-641]
                                 buffer [L630]
                                 base_text [L631]
                                 unreviewed_changes [L632]
                                 status [L633]
                                 version [L634]
                                 diff [L635]
                                 snapshot [L636]
                                 diff_update [L637]
                                 _open_lsp_handle [L638]
                                 _maintain_diff [L639]
                                 _subscription [L640]
                                impl TrackedBuffer [L643-657]
                                 fn has_changes [L644-650]
                                 fn schedule_diff_update [L652-656]
                                pub struct ChangedBuffer [L659-661]
                                 pub diff [L660]
                                mod tests [L664-1574]
                                 fn init_logger [L678-682]
                                 fn init_test [L684-691]
                                 async fn test_keep_edits [L694-769]
                                 async fn test_deletions [L772-854]
                                 async fn test_overlapping_user_edits [L857-951]
                                 async fn test_creating_files [L954-1010]
                                 async fn test_deleting_files [L1013-1120]
                                 async fn test_reject_edits [L1123-1255]
                                 async fn test_reject_multiple_edits [L1258-1331]
                                 async fn test_reject_deleted_file [L1334-1388]
                                 async fn test_reject_created_file [L1391-1443]
                                 async fn test_random_diffs [L1446-1535]
                                  fn quiesce [L1510-1534]
                                 struct HunkStatus [L1538-1542]
                                  range [L1539]
                                  diff_status [L1540]
                                  old_text [L1541]
                                 fn unreviewed_hunks [L1544-1573]

                                Showing symbols 1-69 (total symbols: 69)

                                Using the line numbers in this outline, you can call this tool again while specifying
                                the start_line and end_line fields to see the implementations of symbols in the outline.
                            "},
                    )],
                ),
                message(
                    Assistant,
                    [
                        text(
                            "Let's look at the existing test cases that are related to file creation to understand the pattern:",
                        ),
                        tool_use(
                            "tool_2",
                            ReadFileTool::NAME,
                            ReadFileToolInput {
                                path: input_file_path.into(),
                                start_line: Some(953),
                                end_line: Some(1010),
                            },
                        ),
                    ],
                ),
                message(
                    User,
                    [tool_result(
                        "tool_2",
                        ReadFileTool::NAME,
                        lines(input_file_content, 953..1010),
                    )],
                ),
                message(
                    Assistant,
                    [
                        text(
                            "Now let's look at another relevant test to understand how they're structured:",
                        ),
                        tool_use(
                            "tool_3",
                            ReadFileTool::NAME,
                            ReadFileToolInput {
                                path: input_file_path.into(),
                                start_line: Some(1012),
                                end_line: Some(1120),
                            },
                        ),
                    ],
                ),
                message(
                    User,
                    [tool_result(
                        "tool_3",
                        ReadFileTool::NAME,
                        lines(input_file_content, 1012..1120),
                    )],
                ),
                message(
                    Assistant,
                    [
                        text("Now let's look at how `buffer_created` is implemented:"),
                        tool_use(
                            "tool_4",
                            ReadFileTool::NAME,
                            ReadFileToolInput {
                                path: input_file_path.into(),
                                start_line: Some(271),
                                end_line: Some(276),
                            },
                        ),
                    ],
                ),
                message(
                    User,
                    [tool_result(
                        "tool_4",
                        ReadFileTool::NAME,
                        lines(input_file_content, 271..276),
                    )],
                ),
            ],
            input_file_path,
            Some(input_file_content.into()),
            EvalAssertion::judge_diff(
                "A new test for overwritten files was created, without changing any previous test",
            ),
        ))
    });
}

#[test]
#[cfg_attr(not(feature = "unit-eval"), ignore)]
fn eval_create_empty_file() {
    let input_file_path = "root/TODO3";
    let input_file_content = None;
    let expected_output_content = String::new();

    eval_utils::eval(100, 0.99, eval_utils::NoProcessor, move || {
        run_eval(EvalInput::new(
            vec![
                message(User, [text("Create a second empty todo file ")]),
                message(
                    Assistant,
                    [
                        text(indoc::formatdoc! {"
                            I'll help you create a second empty todo file.
                            First, let me examine the project structure to see if there's already a todo file, which will help me determine the appropriate name and location for the second one.
                            "}),
                        tool_use(
                            "toolu_01GAF8TtsgpjKxCr8fgQLDgR",
                            ListDirectoryTool::NAME,
                            ListDirectoryToolInput {
                                path: "root".to_string(),
                            },
                        ),
                    ],
                ),
                message(
                    User,
                    [tool_result(
                        "toolu_01GAF8TtsgpjKxCr8fgQLDgR",
                        ListDirectoryTool::NAME,
                        "root/TODO\nroot/TODO2\nroot/new.txt\n",
                    )],
                ),
            ],
            input_file_path,
            input_file_content.clone(),
            EvalAssertion::assert_eq(expected_output_content.clone()),
        ))
    });
}
