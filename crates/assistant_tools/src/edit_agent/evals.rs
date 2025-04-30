use super::*;
use crate::{ReadFileToolInput, streaming_edit_file_tool::StreamingEditFileToolInput};
use Role::*;
use anyhow::{Context, anyhow};
use client::{Client, UserStore};
use collections::HashMap;
use fs::FakeFs;
use gpui::{AppContext, TestAppContext};
use indoc::indoc;
use language_model::{
    LanguageModelRegistry, LanguageModelToolResult, LanguageModelToolUse, LanguageModelToolUseId,
};
use project::Project;
use rand::prelude::*;
use reqwest_client::ReqwestClient;
use serde_json::json;
use std::{cmp::Reverse, io::Write as _, sync::mpsc};
use util::path;

#[test]
fn eval_extract_handle_command_output() {
    let input_file_path = "root/blame.rs";
    let input_file_content = include_str!("evals/fixtures/extract_handle_command_output/before.rs");
    let output_file_content = include_str!("evals/fixtures/extract_handle_command_output/after.rs");
    let edit_description = "Extract `handle_command_output` method from `run_git_blame`.";
    eval(
        100,
        0.95,
        EvalInput {
            conversation: vec![
                message(
                    User,
                    [text(indoc! {"
                        Read the `{input_file_path}` file and extract a method in
                        the final stanza of `run_git_blame` to deal with command failures,
                        call it `handle_command_output` and take the std::process::Output as the only parameter.

                        Add it right next to `run_git_blame` and copy it verbatim from `run_git_blame`.
                    "})],
                ),
                message(
                    Assistant,
                    [tool_use(
                        "tool_1",
                        "read_file",
                        ReadFileToolInput {
                            path: input_file_path.into(),
                            start_line: None,
                            end_line: None,
                        },
                    )],
                ),
                message(
                    User,
                    [tool_result("tool_1", "read_file", input_file_content)],
                ),
                message(
                    Assistant,
                    [tool_use(
                        "tool_2",
                        "edit_file",
                        StreamingEditFileToolInput {
                            display_description: edit_description.into(),
                            path: input_file_path.into(),
                        },
                    )],
                ),
            ],
            input_path: input_file_path.into(),
            input_content: input_file_content.into(),
            edit_description: edit_description.into(),
            expected_output: ExpectedOutput {
                text: output_file_content.into(),
                comparison: ComparisonKind::IgnoreEmptyLines,
            },
        },
    );
}

#[test]
fn eval_delete_run_git_blame() {
    let input_file_path = "root/blame.rs";
    let input_file_content = include_str!("evals/fixtures/delete_run_git_blame/before.rs");
    let output_file_content = include_str!("evals/fixtures/delete_run_git_blame/after.rs");
    let edit_description = "Delete the `run_git_blame` function.";
    eval(
        100,
        0.95,
        EvalInput {
            conversation: vec![
                message(
                    User,
                    [text(indoc! {"
                        Read the `{input_file_path}` file and delete `run_git_blame`. Just that
                        one function, not its usages.
                    "})],
                ),
                message(
                    Assistant,
                    [tool_use(
                        "tool_1",
                        "read_file",
                        ReadFileToolInput {
                            path: input_file_path.into(),
                            start_line: None,
                            end_line: None,
                        },
                    )],
                ),
                message(
                    User,
                    [tool_result("tool_1", "read_file", input_file_content)],
                ),
                message(
                    Assistant,
                    [tool_use(
                        "tool_2",
                        "edit_file",
                        StreamingEditFileToolInput {
                            display_description: edit_description.into(),
                            path: input_file_path.into(),
                        },
                    )],
                ),
            ],
            input_path: input_file_path.into(),
            input_content: input_file_content.into(),
            edit_description: edit_description.into(),
            expected_output: ExpectedOutput {
                text: output_file_content.into(),
                comparison: ComparisonKind::IgnoreEmptyLines,
            },
        },
    );
}

#[test]
fn eval_use_wasi_sdk_in_compile_parser_to_wasm() {
    let input_file_path = "root/lib.rs";
    let input_file_content =
        include_str!("evals/fixtures/use_wasi_sdk_in_compile_parser_to_wasm/before.rs");
    let output_file_content =
        include_str!("evals/fixtures/use_wasi_sdk_in_compile_parser_to_wasm/after.rs");
    let edit_description = "Update compile_parser_to_wasm to use wasi-sdk instead of emscripten";
    eval(
        100,
        0.95,
        EvalInput {
            conversation: vec![
                message(
                    User,
                    [text(indoc! {"
                        Read the `{input_file_path}` file and change `compile_parser_to_wasm` to use `wasi-sdk` instead of emscripten.
                        Use `ureq` to download the SDK for the current platform and architecture.
                        Extract the archive into a sibling of `lib` inside the `tree-sitter` directory in the cache_dir.
                        Compile the parser to wasm using the `bin/clang` executable (or `bin/clang.exe` on windows)
                        that's inside of the archive.
                        Don't re-download the SDK if that executable already exists.

                        Use these clang flags: -fPIC -shared -Os -Wl,--export=tree_sitter_{language_name}

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
                        "read_file",
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
                        "read_file",
                        lines(input_file_content, 971..1050),
                    )],
                ),
                message(
                    Assistant,
                    [tool_use(
                        "tool_2",
                        "read_file",
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
                        "read_file",
                        lines(input_file_content, 1050..1100),
                    )],
                ),
                message(
                    Assistant,
                    [tool_use(
                        "tool_3",
                        "read_file",
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
                        "read_file",
                        lines(input_file_content, 1100..1150),
                    )],
                ),
                message(
                    Assistant,
                    [tool_use(
                        "tool_4",
                        "edit_file",
                        StreamingEditFileToolInput {
                            display_description: edit_description.into(),
                            path: input_file_path.into(),
                        },
                    )],
                ),
            ],
            input_path: input_file_path.into(),
            input_content: input_file_content.into(),
            edit_description: edit_description.into(),
            expected_output: ExpectedOutput {
                text: output_file_content.into(),
                comparison: ComparisonKind::Judge,
            },
        },
    );
}

fn message(
    role: Role,
    contents: impl IntoIterator<Item = MessageContent>,
) -> LanguageModelRequestMessage {
    LanguageModelRequestMessage {
        role,
        content: contents.into_iter().collect(),
        cache: false,
    }
}

fn text(text: impl Into<String>) -> MessageContent {
    MessageContent::Text(text.into())
}

fn lines(input: &str, range: Range<usize>) -> String {
    input
        .lines()
        .skip(range.start)
        .take(range.len())
        .collect::<Vec<_>>()
        .join("\n")
}

#[derive(Clone)]
struct ExpectedOutput {
    text: String,
    comparison: ComparisonKind,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum ComparisonKind {
    IgnoreEmptyLines,
    Judge,
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
        content: result.into(),
    })
}

#[derive(Clone)]
struct EvalInput {
    conversation: Vec<LanguageModelRequestMessage>,
    input_path: PathBuf,
    input_content: String,
    edit_description: String,
    expected_output: ExpectedOutput,
}

fn eval(iterations: usize, expected_pass_ratio: f32, mut eval: EvalInput) {
    let mut evaluated_count = 0;
    report_progress(evaluated_count, iterations);

    let (tx, rx) = mpsc::channel();

    // Cache the last message in the conversation, and run one instance of the eval so that
    // all the next ones are cached.
    eval.conversation.last_mut().unwrap().cache = true;
    run_eval(eval.clone(), tx.clone());

    let executor = gpui::background_executor();
    for _ in 1..iterations {
        let eval = eval.clone();
        let tx = tx.clone();
        executor.spawn(async move { run_eval(eval, tx) }).detach();
    }
    drop(tx);

    let mut failed_count = 0;
    let mut failed_evals = HashMap::default();
    let mut errored_evals = HashMap::default();
    while let Ok(output) = rx.recv() {
        match output {
            Ok(output) => {
                if output.comparison.score < 80 {
                    failed_count += 1;
                    failed_evals
                        .entry(output.buffer_text.clone())
                        .or_insert(Vec::new())
                        .push(output);
                }
            }
            Err(error) => {
                failed_count += 1;
                *errored_evals.entry(format!("{:?}", error)).or_insert(0) += 1;
            }
        }

        evaluated_count += 1;
        report_progress(evaluated_count, iterations);
    }

    let actual_pass_ratio = (iterations - failed_count) as f32 / iterations as f32;
    println!("Actual pass ratio: {}\n", actual_pass_ratio);
    if actual_pass_ratio < expected_pass_ratio {
        let mut errored_evals = errored_evals.into_iter().collect::<Vec<_>>();
        errored_evals.sort_by_key(|(_, count)| Reverse(*count));
        for (error, count) in errored_evals {
            println!("Eval errored {} times. Error: {}", count, error);
        }

        let mut failed_evals = failed_evals.into_iter().collect::<Vec<_>>();
        failed_evals.sort_by_key(|(_, evals)| Reverse(evals.len()));
        for (_buffer_output, evals) in failed_evals {
            println!("Eval failed {} times", evals.len());
            for eval in evals {
                if let Some(judge_output) = &eval.comparison.judge_output {
                    println!("Judge Output:\n{}", judge_output);
                }
                println!("Diff:\n{}", eval.diff);
                println!("Raw Edits:\n{}", eval.raw_edits);
            }
        }

        panic!(
            "Actual pass ratio: {}\nExpected pass ratio: {}",
            actual_pass_ratio, expected_pass_ratio
        );
    }
}

fn run_eval(eval: EvalInput, tx: mpsc::Sender<Result<EvalOutput>>) {
    let dispatcher = gpui::TestDispatcher::new(StdRng::from_entropy());
    let mut cx = TestAppContext::build(dispatcher, None);
    let output = cx.executor().block_test(async {
        let test = EditAgentTest::new(&mut cx).await;
        test.eval(eval, &mut cx).await
    });
    tx.send(output).unwrap();
}

struct EvalOutput {
    comparison: DiffComparison,
    buffer_text: String,
    raw_edits: String,
    diff: String,
}

fn report_progress(evaluated_count: usize, iterations: usize) {
    print!("\r\x1b[KEvaluated {}/{}", evaluated_count, iterations);
    std::io::stdout().flush().unwrap();
}

struct EditAgentTest {
    agent: EditAgent,
    project: Entity<Project>,
    judge_model: Arc<dyn LanguageModel>,
}

impl EditAgentTest {
    async fn new(cx: &mut TestAppContext) -> Self {
        cx.executor().allow_parking();
        cx.update(settings::init);
        cx.update(Project::init_settings);
        cx.update(language::init);
        cx.update(gpui_tokio::init);
        cx.update(client::init_settings);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree("/root", json!({})).await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let (agent_model, judge_model) = cx
            .update(|cx| {
                let http_client = ReqwestClient::user_agent("agent tests").unwrap();
                cx.set_http_client(Arc::new(http_client));

                let client = Client::production(cx);
                let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
                language_model::init(client.clone(), cx);
                language_models::init(user_store.clone(), client.clone(), fs.clone(), cx);

                cx.spawn(async move |cx| {
                    let agent_model =
                        Self::load_model("anthropic", "claude-3-7-sonnet-latest", cx).await;
                    let judge_model =
                        Self::load_model("anthropic", "claude-3-7-sonnet-latest", cx).await;
                    (agent_model.unwrap(), judge_model.unwrap())
                })
            })
            .await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));

        Self {
            agent: EditAgent::new(agent_model, action_log, Templates::new()),
            project,
            judge_model,
        }
    }

    async fn load_model(
        provider: &str,
        id: &str,
        cx: &mut AsyncApp,
    ) -> Result<Arc<dyn LanguageModel>> {
        let (provider, model) = cx.update(|cx| {
            let models = LanguageModelRegistry::read_global(cx);
            let model = models
                .available_models(cx)
                .find(|model| model.provider_id().0 == provider && model.id().0 == id)
                .unwrap();
            let provider = models.provider(&model.provider_id()).unwrap();
            (provider, model)
        })?;
        cx.update(|cx| provider.authenticate(cx))?.await?;
        Ok(model)
    }

    async fn eval(&self, eval: EvalInput, cx: &mut TestAppContext) -> Result<EvalOutput> {
        let path = self
            .project
            .read_with(cx, |project, cx| {
                project.find_project_path(eval.input_path, cx)
            })
            .unwrap();
        let buffer = self
            .project
            .update(cx, |project, cx| project.open_buffer(path, cx))
            .await
            .unwrap();
        buffer.update(cx, |buffer, cx| {
            buffer.set_text(eval.input_content.clone(), cx)
        });
        let raw_output = self
            .agent
            .edit(
                buffer.clone(),
                eval.edit_description,
                eval.conversation,
                &mut cx.to_async(),
            )
            .await?;
        let buffer_text = buffer.read_with(cx, |buffer, _| buffer.text());
        let actual_diff = language::unified_diff(&eval.input_content, &buffer_text);
        let diff_comparison = match eval.expected_output.comparison {
            ComparisonKind::IgnoreEmptyLines => DiffComparison {
                score: if strip_empty_lines(&buffer_text)
                    == strip_empty_lines(&eval.expected_output.text)
                {
                    100
                } else {
                    0
                },
                judge_output: None,
            },
            ComparisonKind::Judge => {
                let expected_diff =
                    language::unified_diff(&eval.input_content, &eval.expected_output.text);
                self.compare_diffs(&actual_diff, &expected_diff, &cx.to_async())
                    .await
                    .context("failed comparing diffs")?
            }
        };

        Ok(EvalOutput {
            comparison: diff_comparison,
            diff: actual_diff,
            buffer_text,
            raw_edits: raw_output,
        })
    }

    async fn compare_diffs(
        &self,
        diff_a: &str,
        diff_b: &str,
        cx: &AsyncApp,
    ) -> Result<DiffComparison> {
        let prompt = DiffJudgeTemplate {
            diff_a: diff_a.to_string(),
            diff_b: diff_b.to_string(),
        }
        .render(&self.agent.templates)
        .unwrap();

        let request = LanguageModelRequest {
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![prompt.into()],
                cache: false,
            }],
            ..Default::default()
        };
        let mut response = self.judge_model.stream_completion_text(request, cx).await?;
        let mut output = String::new();
        while let Some(chunk) = response.stream.next().await {
            let chunk = chunk?;
            output.push_str(&chunk);
        }

        // Parse the score from the response
        let re = regex::Regex::new(r"<score>(\d+)</score>").unwrap();
        if let Some(captures) = re.captures(&output) {
            if let Some(score_match) = captures.get(1) {
                let score = score_match.as_str().parse().unwrap_or(0);
                return Ok(DiffComparison {
                    score,
                    judge_output: Some(output),
                });
            }
        }

        Err(anyhow!(
            "No score found in response. Raw output: {}",
            output
        ))
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct DiffComparison {
    score: usize,
    judge_output: Option<String>,
}

#[derive(Serialize)]
pub struct DiffJudgeTemplate {
    diff_a: String,
    diff_b: String,
}

impl Template for DiffJudgeTemplate {
    const TEMPLATE_NAME: &'static str = "diff_judge.hbs";
}

fn strip_empty_lines(text: &str) -> String {
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}
