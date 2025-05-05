use super::*;
use crate::{
    ReadFileToolInput, grep_tool::GrepToolInput,
    streaming_edit_file_tool::StreamingEditFileToolInput,
};
use Role::*;
use anyhow::anyhow;
use client::{Client, UserStore};
use collections::HashMap;
use fs::FakeFs;
use futures::{FutureExt, future::LocalBoxFuture};
use gpui::{AppContext, TestAppContext};
use indoc::indoc;
use language_model::{
    LanguageModelRegistry, LanguageModelToolResult, LanguageModelToolUse, LanguageModelToolUseId,
};
use project::Project;
use rand::prelude::*;
use reqwest_client::ReqwestClient;
use serde_json::json;
use std::{
    cmp::Reverse,
    fmt::{self, Display},
    io::Write as _,
    sync::mpsc,
};
use util::path;

#[test]
#[cfg_attr(not(feature = "eval"), ignore)]
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
                            create_or_overwrite: false,
                        },
                    )],
                ),
            ],
            input_path: input_file_path.into(),
            input_content: Some(input_file_content.into()),
            edit_description: edit_description.into(),
            assertion: EvalAssertion::assert_eq(output_file_content),
        },
    );
}

#[test]
#[cfg_attr(not(feature = "eval"), ignore)]
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
                            create_or_overwrite: false,
                        },
                    )],
                ),
            ],
            input_path: input_file_path.into(),
            input_content: Some(input_file_content.into()),
            edit_description: edit_description.into(),
            assertion: EvalAssertion::assert_eq(output_file_content),
        },
    );
}

#[test]
#[cfg_attr(not(feature = "eval"), ignore)]
fn eval_use_wasi_sdk_in_compile_parser_to_wasm() {
    let input_file_path = "root/lib.rs";
    let input_file_content =
        include_str!("evals/fixtures/use_wasi_sdk_in_compile_parser_to_wasm/before.rs");
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
                            create_or_overwrite: false,
                        },
                    )],
                ),
            ],
            input_path: input_file_path.into(),
            input_content: Some(input_file_content.into()),
            edit_description: edit_description.into(),
            assertion: EvalAssertion::judge_diff(indoc! {"
                - The compile_parser_to_wasm method has been changed to use wasi-sdk
                - ureq is used to download the SDK for current platform and architecture
            "}),
        },
    );
}

#[test]
#[cfg_attr(not(feature = "eval"), ignore)]
fn eval_disable_cursor_blinking() {
    let input_file_path = "root/editor.rs";
    let input_file_content = include_str!("evals/fixtures/disable_cursor_blinking/before.rs");
    let output_file_content = include_str!("evals/fixtures/disable_cursor_blinking/after.rs");
    let edit_description = "Comment out the call to `BlinkManager::enable`";
    eval(
        100,
        0.6, // TODO: make this eval better
        EvalInput {
            conversation: vec![
                message(User, [text("Let's research how to cursor blinking works.")]),
                message(
                    Assistant,
                    [tool_use(
                        "tool_1",
                        "grep",
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
                        "grep",
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
                    [text(indoc! {"
                        Comment out the lines that interact with the BlinkManager.
                        Keep the outer `update` blocks, but comments everything that's inside (including if statements).
                        Don't add additional comments.
                    "})],
                ),
                message(
                    Assistant,
                    [tool_use(
                        "tool_4",
                        "edit_file",
                        StreamingEditFileToolInput {
                            display_description: edit_description.into(),
                            path: input_file_path.into(),
                            create_or_overwrite: false,
                        },
                    )],
                ),
            ],
            input_path: input_file_path.into(),
            input_content: Some(input_file_content.into()),
            edit_description: edit_description.into(),
            assertion: EvalAssertion::assert_eq(output_file_content),
        },
    );
}

#[test]
#[cfg_attr(not(feature = "eval"), ignore)]
fn eval_from_pixels_constructor() {
    let input_file_path = "root/canvas.rs";
    let input_file_content = include_str!("evals/fixtures/from_pixels_constructor/before.rs");
    let edit_description = "Implement from_pixels constructor and add tests.";
    eval(
        100,
        0.95,
        EvalInput {
            conversation: vec![
                message(
                    User,
                    [text(indoc! {"
                        Introduce a new `from_pixels` constructor in Canvas and
                        also add tests for it in the same file.
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
                        "grep",
                        GrepToolInput {
                            regex: "mod\\s+tests".into(),
                            include_pattern: Some("font-kit/src/canvas.rs".into()),
                            offset: 0,
                            case_sensitive: false,
                        },
                    )],
                ),
                message(User, [tool_result("tool_2", "grep", "No matches found")]),
                message(
                    Assistant,
                    [tool_use(
                        "tool_3",
                        "grep",
                        GrepToolInput {
                            regex: "mod\\s+tests".into(),
                            include_pattern: Some("font-kit/src/**/*.rs".into()),
                            offset: 0,
                            case_sensitive: false,
                        },
                    )],
                ),
                message(User, [tool_result("tool_3", "grep", "No matches found")]),
                message(
                    Assistant,
                    [tool_use(
                        "tool_4",
                        "grep",
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
                        "grep",
                        indoc! {"
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
                message(
                    Assistant,
                    [tool_use(
                        "tool_5",
                        "edit_file",
                        StreamingEditFileToolInput {
                            display_description: edit_description.into(),
                            path: input_file_path.into(),
                            create_or_overwrite: false,
                        },
                    )],
                ),
            ],
            input_path: input_file_path.into(),
            input_content: Some(input_file_content.into()),
            edit_description: edit_description.into(),
            assertion: EvalAssertion::judge_diff(indoc! {"
                - The diff contains a new `from_pixels` constructor
                - The diff contains new tests for the `from_pixels` constructor
            "}),
        },
    );
}

#[test]
#[cfg_attr(not(feature = "eval"), ignore)]
fn eval_zode() {
    let input_file_path = "root/zode.py";
    let edit_description = "Create the main Zode CLI script";
    eval(
        200,
        1.,
        EvalInput {
            conversation: vec![
                message(User, [text(include_str!("evals/fixtures/zode/prompt.md"))]),
                message(
                    Assistant,
                    [
                        tool_use(
                            "tool_1",
                            "read_file",
                            ReadFileToolInput {
                                path: "root/eval/react.py".into(),
                                start_line: None,
                                end_line: None,
                            },
                        ),
                        tool_use(
                            "tool_2",
                            "read_file",
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
                            "read_file",
                            include_str!("evals/fixtures/zode/react.py"),
                        ),
                        tool_result(
                            "tool_2",
                            "read_file",
                            include_str!("evals/fixtures/zode/react_test.py"),
                        ),
                    ],
                ),
                message(
                    Assistant,
                    [
                        text(
                            "Now that I understand what we need to build, I'll create the main Python script:",
                        ),
                        tool_use(
                            "tool_3",
                            "edit_file",
                            StreamingEditFileToolInput {
                                display_description: edit_description.into(),
                                path: input_file_path.into(),
                                create_or_overwrite: true,
                            },
                        ),
                    ],
                ),
            ],
            input_path: input_file_path.into(),
            input_content: None,
            edit_description: edit_description.into(),
            assertion: EvalAssertion::new(async move |sample, _, _cx| {
                let invalid_starts = [' ', '`', '\n'];
                let mut message = String::new();
                for start in invalid_starts {
                    if sample.text.starts_with(start) {
                        message.push_str(&format!("The sample starts with a {:?}\n", start));
                        break;
                    }
                }
                // Remove trailing newline.
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
    input_content: Option<String>,
    edit_description: String,
    assertion: EvalAssertion,
}

#[derive(Clone)]
struct EvalSample {
    text: String,
    edit_output: EditAgentOutput,
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
                score: if strip_empty_lines(&sample.text) == strip_empty_lines(&expected) {
                    100
                } else {
                    0
                },
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
            .unwrap();

            let request = LanguageModelRequest {
                messages: vec![LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![prompt.into()],
                    cache: false,
                }],
                ..Default::default()
            };
            let mut response = judge
                .stream_completion_text(request, &cx.to_async())
                .await?;
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
                    return Ok(EvalAssertionOutcome {
                        score,
                        message: Some(output),
                    });
                }
            }

            Err(anyhow!(
                "No score found in response. Raw output: {}",
                output
            ))
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
    let mut eval_outputs = Vec::new();
    let mut cumulative_parser_metrics = EditParserMetrics::default();
    while let Ok(output) = rx.recv() {
        match output {
            Ok(output) => {
                cumulative_parser_metrics += output.sample.edit_output._parser_metrics.clone();
                eval_outputs.push(output.clone());
                if output.assertion.score < 80 {
                    failed_count += 1;
                    failed_evals
                        .entry(output.sample.text.clone())
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
        for (_buffer_output, failed_evals) in failed_evals {
            let eval_output = failed_evals.first().unwrap();
            println!("Eval failed {} times", failed_evals.len());
            println!("{}", eval_output);
        }

        panic!(
            "Actual pass ratio: {}\nExpected pass ratio: {}",
            actual_pass_ratio, expected_pass_ratio
        );
    }

    let mismatched_tag_ratio =
        cumulative_parser_metrics.mismatched_tags as f32 / cumulative_parser_metrics.tags as f32;
    if mismatched_tag_ratio > 0.02 {
        for eval_output in eval_outputs {
            println!("{}", eval_output);
        }
        panic!("Too many mismatched tags: {:?}", cumulative_parser_metrics);
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

#[derive(Clone)]
struct EvalOutput {
    sample: EvalSample,
    assertion: EvalAssertionOutcome,
}

impl Display for EvalOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Score: {:?}", self.assertion.score)?;
        if let Some(message) = self.assertion.message.as_ref() {
            writeln!(f, "Message: {}", message)?;
        }

        writeln!(f, "Diff:\n{}", self.sample.diff)?;

        writeln!(
            f,
            "Parser Metrics:\n{:#?}",
            self.sample.edit_output._parser_metrics
        )?;
        writeln!(f, "Raw Edits:\n{}", self.sample.edit_output._raw_edits)?;
        Ok(())
    }
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
                        Self::load_model("google", "gemini-2.5-pro-preview-03-25", cx).await;
                    let judge_model =
                        Self::load_model("anthropic", "claude-3-7-sonnet-latest", cx).await;
                    (agent_model.unwrap(), judge_model.unwrap())
                })
            })
            .await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));

        Self {
            agent: EditAgent::new(agent_model, project.clone(), action_log, Templates::new()),
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
        let edit_output = if let Some(input_content) = eval.input_content.as_deref() {
            buffer.update(cx, |buffer, cx| buffer.set_text(input_content, cx));
            let (edit_output, _) = self.agent.edit(
                buffer.clone(),
                eval.edit_description,
                eval.conversation,
                &mut cx.to_async(),
            );
            edit_output.await?
        } else {
            let (edit_output, _) = self.agent.overwrite(
                buffer.clone(),
                eval.edit_description,
                eval.conversation,
                &mut cx.to_async(),
            );
            edit_output.await?
        };

        let buffer_text = buffer.read_with(cx, |buffer, _| buffer.text());
        let sample = EvalSample {
            edit_output,
            diff: language::unified_diff(
                eval.input_content.as_deref().unwrap_or_default(),
                &buffer_text,
            ),
            text: buffer_text,
        };
        let assertion = eval
            .assertion
            .run(&sample, self.judge_model.clone(), cx)
            .await?;

        Ok(EvalOutput { assertion, sample })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct EvalAssertionOutcome {
    score: usize,
    message: Option<String>,
}

#[derive(Serialize)]
pub struct DiffJudgeTemplate {
    diff: String,
    assertions: &'static str,
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
