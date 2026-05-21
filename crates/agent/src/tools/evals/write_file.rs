use crate::{
    AgentTool, ContextServerRegistry, ListDirectoryTool, ListDirectoryToolInput, Template,
    Templates, Thread, ToolCallEventStream, ToolInput, WriteFileTool, WriteFileToolInput,
};
use Role::*;
use anyhow::{Context as _, Result};
use client::{Client, RefreshLlmTokenListener, UserStore};
use fs::FakeFs;
use futures::{FutureExt as _, StreamExt};
use gpui::{AppContext as _, AsyncApp, Entity, TestAppContext, UpdateGlobal as _};
use http_client::StatusCode;
use language::language_settings::FormatOnSave;
use language_model::{
    LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage,
    LanguageModelToolResult, LanguageModelToolResultContent, LanguageModelToolUse,
    LanguageModelToolUseId, MessageContent, Role, SelectedModel,
};
use project::Project;
use prompt_store::{ProjectContext, WorktreeContext};
use rand::prelude::*;
use reqwest_client::ReqwestClient;
use serde::Serialize;
use settings::SettingsStore;
use std::{
    fmt::{self, Display},
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
    time::Duration,
};
use util::path;

#[derive(Clone)]
struct EvalInput {
    conversation: Vec<LanguageModelRequestMessage>,
    input_file_path: PathBuf,
    input_content: Option<String>,
    expected_output_content: String,
}

impl EvalInput {
    fn new(
        conversation: Vec<LanguageModelRequestMessage>,
        input_file_path: impl Into<PathBuf>,
        input_content: Option<String>,
        expected_output_content: String,
    ) -> Self {
        Self {
            conversation,
            input_file_path: input_file_path.into(),
            input_content,
            expected_output_content,
        }
    }
}

struct WriteEvalOutput {
    tool_input: WriteFileToolInput,
    text_after: String,
}

impl Display for WriteEvalOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Tool Input:\n{:#?}", self.tool_input)?;
        writeln!(f, "Text After:\n{}", self.text_after)?;
        Ok(())
    }
}

struct WriteToolTest {
    fs: Arc<FakeFs>,
    project: Entity<Project>,
    model: Arc<dyn LanguageModel>,
    model_thinking_effort: Option<String>,
}

impl WriteToolTest {
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

        fs.insert_tree("/root", serde_json::json!({})).await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
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
                    Self::load_model(&agent_model, cx).await.unwrap()
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

    async fn eval(&self, mut eval: EvalInput, cx: &mut TestAppContext) -> Result<WriteEvalOutput> {
        eval.conversation
            .last_mut()
            .context("Conversation must not be empty")?
            .cache = true;

        if let Some(input_content) = eval.input_content.as_deref() {
            let abs_path = Path::new("/root").join(
                eval.input_file_path
                    .strip_prefix("root")
                    .unwrap_or(&eval.input_file_path),
            );
            self.fs.insert_file(&abs_path, input_content.into()).await;
            cx.run_until_parked();
        }

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
            let templates = Templates::new();
            template.render(&templates)?
        };

        let messages = [LanguageModelRequestMessage {
            role: Role::System,
            content: vec![MessageContent::Text(system_prompt)],
            cache: true,
            reasoning_details: None,
        }]
        .into_iter()
        .chain(eval.conversation)
        .collect::<Vec<_>>();

        let request = LanguageModelRequest {
            messages,
            tools,
            thinking_allowed: true,
            thinking_effort: self.model_thinking_effort.clone(),
            ..Default::default()
        };

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

        let tool = Arc::new(WriteFileTool::new(
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
            Err(output) => anyhow::bail!("Tool returned error: {}", output),
        };

        let crate::EditFileToolOutput::Success { new_text, .. } = &output else {
            anyhow::bail!("Tool returned error output: {}", output);
        };

        if tool_input.path != eval.input_file_path {
            anyhow::bail!(
                "Tool path mismatch. Expected {:?}, got {:?}",
                eval.input_file_path,
                tool_input.path,
            );
        }

        if new_text != &eval.expected_output_content {
            anyhow::bail!(
                "Output content mismatch. Expected {:?}, got {:?}",
                eval.expected_output_content,
                new_text,
            );
        }

        Ok(WriteEvalOutput {
            tool_input,
            text_after: new_text.clone(),
        })
    }

    async fn extract_tool_use(
        &self,
        request: LanguageModelRequest,
        cx: &mut TestAppContext,
    ) -> Result<WriteFileToolInput> {
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
                        && tool_use.name.as_ref() == WriteFileTool::NAME =>
                {
                    let input: WriteFileToolInput = serde_json::from_value(tool_use.input)
                        .context("Failed to parse tool input as WriteFileToolInput")?;
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
                }) if tool_name.as_ref() == WriteFileTool::NAME => {
                    parse_errors.push(format!("{json_parse_error}\nRaw input:\n{raw_input:?}"));
                }
                Err(err) => return Err(anyhow::anyhow!("completion error: {}", err)),
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
            "Stream ended without a write_file tool use{stop_reason_suffix}{parse_errors_suffix}{streamed_text_suffix}"
        )
    }
}

fn run_eval(eval: EvalInput) -> eval_utils::EvalOutput<()> {
    super::run_gpui_eval(
        |cx| {
            async move {
                let test = WriteToolTest::new(cx).await;
                let result = test.eval(eval, cx).await;
                drop(test);
                cx.run_until_parked();
                result
            }
            .boxed_local()
        },
        |_| eval_utils::OutcomeKind::Passed,
    )
}

fn message(
    role: Role,
    content: impl IntoIterator<Item = MessageContent>,
) -> LanguageModelRequestMessage {
    LanguageModelRequestMessage {
        role,
        content: content.into_iter().collect(),
        cache: false,
        reasoning_details: None,
    }
}

fn text(text: impl Into<String>) -> MessageContent {
    MessageContent::Text(text.into())
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
        content: vec![LanguageModelToolResultContent::Text(result.into())],
        output: None,
    })
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
fn eval_create_file() {
    let input_file_path = "root/TODO3";
    let expected_output_content = "todo".to_string();

    eval_utils::eval(100, 1., eval_utils::NoProcessor, move || {
        run_eval(EvalInput::new(
            vec![
                message(
                    User,
                    [text("Create a third todo file. Write 'todo' inside it.")],
                ),
                message(
                    Assistant,
                    [
                        text(indoc::formatdoc! {"
                            I'll help you create a third empty todo file.
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
            None,
            expected_output_content.clone(),
        ))
    });
}

#[test]
#[cfg_attr(not(feature = "unit-eval"), ignore)]
fn eval_overwrite_file() {
    let input_file_path = "root/notes.txt";
    let input_file_content = "old notes\nkeep nothing\n".to_string();
    let expected_output_content = "new notes".to_string();

    eval_utils::eval(100, 1., eval_utils::NoProcessor, move || {
        run_eval(EvalInput::new(
            vec![message(
                User,
                [text(indoc::formatdoc! {"
                    Overwrite `{input_file_path}` so that its complete contents are exactly: 'new notes'
                "})],
            )],
            input_file_path,
            Some(input_file_content.clone()),
            expected_output_content.clone(),
        ))
    });
}
