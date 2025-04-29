use super::*;
use crate::{EditFileToolInput, ReadFileToolInput};
use Role::*;
use client::{Client, UserStore};
use collections::HashMap;
use fs::FakeFs;
use gpui::{AppContext, TestAppContext};
use indoc::indoc;
use language_model::{LanguageModelRegistry, LanguageModelToolUse, LanguageModelToolUseId};
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
                        EditFileToolInput {
                            display_description: edit_description.into(),
                            path: input_file_path.into(),
                        },
                    )],
                ),
            ],
            input_path: input_file_path.into(),
            input_content: input_file_content.into(),
            edit_description: edit_description.into(),
            expected_output: output_file_content.into(),
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
                        EditFileToolInput {
                            display_description: edit_description.into(),
                            path: input_file_path.into(),
                        },
                    )],
                ),
            ],
            input_path: input_file_path.into(),
            input_content: input_file_content.into(),
            edit_description: edit_description.into(),
            expected_output: output_file_content.into(),
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
        cache: true,
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
    expected_output: String,
}

fn eval(iterations: usize, expected_pass_ratio: f32, eval: EvalInput) {
    let executor = gpui::background_executor();
    let (tx, rx) = mpsc::channel();
    for _ in 0..iterations {
        let eval = eval.clone();
        let tx = tx.clone();
        executor
            .spawn(async move {
                let dispatcher = gpui::TestDispatcher::new(StdRng::from_entropy());
                let mut cx = TestAppContext::build(dispatcher, None);
                let output = cx
                    .executor()
                    .block_test(async { run_eval(eval, &mut cx).await });
                tx.send(output).unwrap();
            })
            .detach();
    }
    drop(tx);

    let expected_output = strip_empty_lines(&eval.expected_output);
    let mut evaluated_count = 0;
    report_progress(evaluated_count, iterations);

    let mut failed_count = 0;
    let mut failed_evals = HashMap::default();
    while let Ok(output) = rx.recv() {
        if output
            .as_ref()
            .map_or(true, |output| output.buffer_text != expected_output)
        {
            failed_count += 1;
            *failed_evals
                .entry(output.map_err(|error| error.to_string()))
                .or_insert(0) += 1;
        }

        evaluated_count += 1;
        report_progress(evaluated_count, iterations);
    }

    let actual_pass_ratio = (iterations - failed_count) as f32 / iterations as f32;
    println!("Actual pass ratio: {}\n", actual_pass_ratio);
    if actual_pass_ratio < expected_pass_ratio {
        let mut failed_evals = failed_evals.into_iter().collect::<Vec<_>>();
        failed_evals.sort_by_key(|(_, count)| Reverse(*count));
        for (output, count) in failed_evals {
            match output {
                Ok(output) => {
                    println!(
                        "Failed {} times. Raw Output\n{}\nDiff\n{}\n=====",
                        count,
                        output.raw_edits,
                        pretty_assertions::StrComparison::new(
                            &output.buffer_text,
                            &eval.expected_output
                        )
                    );
                }
                Err(error) => {
                    println!("Failed {} times. Error: {}\n=====", count, error);
                }
            }
        }

        panic!(
            "Actual pass ratio: {}\nExpected pass ratio: {}",
            actual_pass_ratio, expected_pass_ratio
        );
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct EvalOutput {
    buffer_text: String,
    raw_edits: String,
}

async fn run_eval(eval: EvalInput, cx: &mut TestAppContext) -> Result<EvalOutput> {
    let test = agent_test(cx).await;
    let path = test
        .project
        .read_with(cx, |project, cx| {
            project.find_project_path(eval.input_path, cx)
        })
        .unwrap();
    let buffer = test
        .project
        .update(cx, |project, cx| project.open_buffer(path, cx))
        .await
        .unwrap();
    buffer.update(cx, |buffer, cx| buffer.set_text(eval.input_content, cx));
    let raw_output = test
        .agent
        .edit(
            buffer.clone(),
            eval.edit_description,
            eval.conversation,
            &mut cx.to_async(),
        )
        .await?;
    let buffer_text = buffer.read_with(cx, |buffer, _| buffer.text());
    Ok(EvalOutput {
        buffer_text: strip_empty_lines(buffer_text),
        raw_edits: raw_output,
    })
}

fn report_progress(evaluated_count: usize, iterations: usize) {
    print!("\r\x1b[KEvaluated {}/{}", evaluated_count, iterations);
    std::io::stdout().flush().unwrap();
}

fn strip_empty_lines(text: impl AsRef<str>) -> String {
    text.as_ref()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

struct EditAgentTest {
    agent: EditAgent,
    project: Entity<Project>,
}

async fn agent_test(cx: &mut TestAppContext) -> EditAgentTest {
    cx.executor().allow_parking();
    cx.update(settings::init);
    cx.update(Project::init_settings);
    cx.update(language::init);
    cx.update(gpui_tokio::init);
    cx.update(client::init_settings);

    let fs = FakeFs::new(cx.executor().clone());
    fs.insert_tree("/root", json!({})).await;
    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let model = cx
        .update(|cx| {
            let http_client = ReqwestClient::user_agent("agent tests").unwrap();
            cx.set_http_client(Arc::new(http_client));

            let client = Client::production(cx);
            let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
            language_model::init(client.clone(), cx);
            language_models::init(user_store.clone(), client.clone(), fs.clone(), cx);

            let models = LanguageModelRegistry::read_global(cx);
            let model = models
                .available_models(cx)
                .find(|model| model.id().0 == "claude-3-7-sonnet-latest")
                .unwrap();

            let provider = models.provider(&model.provider_id()).unwrap();
            let authenticated = provider.authenticate(cx);

            cx.spawn(async move |_| {
                authenticated.await.unwrap();
                model
            })
        })
        .await;
    let action_log = cx.new(|_| ActionLog::new(project.clone()));

    EditAgentTest {
        agent: EditAgent::new(model, action_log, Templates::new()),
        project,
    }
}
