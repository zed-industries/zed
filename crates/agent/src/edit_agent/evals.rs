use super::*;
use crate::{
    EditFileMode, EditFileToolInput, GrepToolInput, ListDirectoryToolInput, ReadFileToolInput,
};
use Role::*;
use client::{Client, UserStore};
use collections::HashMap;
use fs::FakeFs;
use futures::{FutureExt, future::LocalBoxFuture};
use gpui::{AppContext, TestAppContext, Timer};
use http_client::StatusCode;
use indoc::{formatdoc, indoc};
use language_model::{
    LanguageModelRegistry, LanguageModelToolResult, LanguageModelToolResultContent,
    LanguageModelToolUse, LanguageModelToolUseId, SelectedModel,
};
use project::Project;
use prompt_store::{ProjectContext, WorktreeContext};
use rand::prelude::*;
use reqwest_client::ReqwestClient;
use serde_json::json;
use std::{
    cmp::Reverse,
    fmt::{self, Display},
    io::Write as _,
    path::Path,
    str::FromStr,
    sync::mpsc,
    time::Duration,
};
use util::path;

#[test]
#[cfg_attr(not(feature = "edit-agent-eval"), ignore)]
fn eval_extract_handle_command_output() {
    // Test how well agent generates multiple edit hunks.
    //
    // Model                       | Pass rate
    // ----------------------------|----------
    // claude-3.7-sonnet           |  0.99 (2025-06-14)
    // claude-sonnet-4             |  0.97 (2025-06-14)
    // gemini-2.5-pro-06-05        |  0.98 (2025-06-16)
    // gemini-2.5-flash            |  0.11 (2025-05-22)
    // gpt-4.1                     |  1.00 (2025-05-22)

    let input_file_path = "root/blame.rs";
    let input_file_content = include_str!("evals/fixtures/extract_handle_command_output/before.rs");
    let possible_diffs = vec![
        include_str!("evals/fixtures/extract_handle_command_output/possible-01.diff"),
        include_str!("evals/fixtures/extract_handle_command_output/possible-02.diff"),
        include_str!("evals/fixtures/extract_handle_command_output/possible-03.diff"),
        include_str!("evals/fixtures/extract_handle_command_output/possible-04.diff"),
        include_str!("evals/fixtures/extract_handle_command_output/possible-05.diff"),
        include_str!("evals/fixtures/extract_handle_command_output/possible-06.diff"),
        include_str!("evals/fixtures/extract_handle_command_output/possible-07.diff"),
    ];
    let edit_description = "Extract `handle_command_output` method from `run_git_blame`.";
    eval(
        100,
        0.95,
        0.05,
        EvalInput::from_conversation(
            vec![
                message(
                    User,
                    [text(formatdoc! {"
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
                            mode: EditFileMode::Edit,
                        },
                    )],
                ),
            ],
            Some(input_file_content.into()),
            EvalAssertion::assert_diff_any(possible_diffs),
        ),
    );
}

#[test]
#[cfg_attr(not(feature = "edit-agent-eval"), ignore)]
fn eval_delete_run_git_blame() {
    // Model                       | Pass rate
    // ----------------------------|----------
    // claude-3.7-sonnet           | 1.0  (2025-06-14)
    // claude-sonnet-4             | 0.96 (2025-06-14)
    // gemini-2.5-pro-06-05        | 1.0  (2025-06-16)
    // gemini-2.5-flash            |
    // gpt-4.1                     |

    let input_file_path = "root/blame.rs";
    let input_file_content = include_str!("evals/fixtures/delete_run_git_blame/before.rs");
    let output_file_content = include_str!("evals/fixtures/delete_run_git_blame/after.rs");
    let edit_description = "Delete the `run_git_blame` function.";
    eval(
        100,
        0.95,
        0.05,
        EvalInput::from_conversation(
            vec![
                message(
                    User,
                    [text(formatdoc! {"
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
                            mode: EditFileMode::Edit,
                        },
                    )],
                ),
            ],
            Some(input_file_content.into()),
            EvalAssertion::assert_eq(output_file_content),
        ),
    );
}

#[test]
#[cfg_attr(not(feature = "edit-agent-eval"), ignore)]
fn eval_translate_doc_comments() {
    //  Model                          | Pass rate
    // ============================================
    //
    //  claude-3.7-sonnet              |  1.0  (2025-06-14)
    //  claude-sonnet-4                |  1.0  (2025-06-14)
    //  gemini-2.5-pro-preview-03-25   |  1.0  (2025-05-22)
    //  gemini-2.5-flash-preview-04-17 |
    //  gpt-4.1                        |

    let input_file_path = "root/canvas.rs";
    let input_file_content = include_str!("evals/fixtures/translate_doc_comments/before.rs");
    let edit_description = "Translate all doc comments to Italian";
    eval(
        200,
        1.,
        0.05,
        EvalInput::from_conversation(
            vec![
                message(
                    User,
                    [text(formatdoc! {"
                        Read the {input_file_path} file and edit it (without overwriting it),
                        translating all the doc comments to italian.
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
                            mode: EditFileMode::Edit,
                        },
                    )],
                ),
            ],
            Some(input_file_content.into()),
            EvalAssertion::judge_diff("Doc comments were translated to Italian"),
        ),
    );
}

#[test]
#[cfg_attr(not(feature = "edit-agent-eval"), ignore)]
fn eval_use_wasi_sdk_in_compile_parser_to_wasm() {
    //  Model                          | Pass rate
    // ============================================
    //
    //  claude-3.7-sonnet              |  0.96 (2025-06-14)
    //  claude-sonnet-4                |  0.11 (2025-06-14)
    //  gemini-2.5-pro-preview-latest  |  0.99 (2025-06-16)
    //  gemini-2.5-flash-preview-04-17 |
    //  gpt-4.1                        |

    let input_file_path = "root/lib.rs";
    let input_file_content =
        include_str!("evals/fixtures/use_wasi_sdk_in_compile_parser_to_wasm/before.rs");
    let edit_description = "Update compile_parser_to_wasm to use wasi-sdk instead of emscripten";
    eval(
        100,
        0.95,
        0.05,
        EvalInput::from_conversation(
            vec![
                message(
                    User,
                    [text(formatdoc! {"
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
                        EditFileToolInput {
                            display_description: edit_description.into(),
                            path: input_file_path.into(),
                            mode: EditFileMode::Edit,
                        },
                    )],
                ),
            ],
            Some(input_file_content.into()),
            EvalAssertion::judge_diff(indoc! {"
                - The compile_parser_to_wasm method has been changed to use wasi-sdk
                - ureq is used to download the SDK for current platform and architecture
            "}),
        ),
    );
}

#[test]
#[cfg_attr(not(feature = "edit-agent-eval"), ignore)]
fn eval_disable_cursor_blinking() {
    //  Model                          | Pass rate
    // ============================================
    //
    //  claude-3.7-sonnet              |  0.59 (2025-07-14)
    //  claude-sonnet-4                |  0.81 (2025-07-14)
    //  gemini-2.5-pro                 |  0.95 (2025-07-14)
    //  gemini-2.5-flash-preview-04-17 |  0.78 (2025-07-14)
    //  gpt-4.1                        |  0.00 (2025-07-14) (follows edit_description too literally)

    let input_file_path = "root/editor.rs";
    let input_file_content = include_str!("evals/fixtures/disable_cursor_blinking/before.rs");
    let edit_description = "Comment out the call to `BlinkManager::enable`";
    let possible_diffs = vec![
        include_str!("evals/fixtures/disable_cursor_blinking/possible-01.diff"),
        include_str!("evals/fixtures/disable_cursor_blinking/possible-02.diff"),
        include_str!("evals/fixtures/disable_cursor_blinking/possible-03.diff"),
        include_str!("evals/fixtures/disable_cursor_blinking/possible-04.diff"),
    ];
    eval(
        100,
        0.51,
        0.05,
        EvalInput::from_conversation(
            vec![
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
                        EditFileToolInput {
                            display_description: edit_description.into(),
                            path: input_file_path.into(),
                            mode: EditFileMode::Edit,
                        },
                    )],
                ),
            ],
            Some(input_file_content.into()),
            EvalAssertion::assert_diff_any(possible_diffs),
        ),
    );
}

#[test]
#[cfg_attr(not(feature = "edit-agent-eval"), ignore)]
fn eval_from_pixels_constructor() {
    // Results for 2025-06-13
    //
    // The outcome of this evaluation depends heavily on the LINE_HINT_TOLERANCE
    // value. Higher values improve the pass rate but may sometimes cause
    // edits to be misapplied. In the context of this eval, this means
    // the agent might add from_pixels tests in incorrect locations
    // (e.g., at the beginning of the file), yet the evaluation may still
    // rate it highly.
    //
    //  Model                          | Date        | Pass rate
    // =========================================================
    //  claude-4.0-sonnet              | 2025-06-14  | 0.99
    //  claude-3.7-sonnet              | 2025-06-14  | 0.88
    //  gemini-2.5-pro-preview-06-05   | 2025-06-16  | 0.98
    //  gpt-4.1                        |

    let input_file_path = "root/canvas.rs";
    let input_file_content = include_str!("evals/fixtures/from_pixels_constructor/before.rs");
    let edit_description = "Implement from_pixels constructor and add tests.";
    eval(
        100,
        0.95,
        // For whatever reason, this eval produces more mismatched tags.
        // Increasing for now, let's see if we can bring this down.
        0.25,
        EvalInput::from_conversation(
            vec![
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
                        EditFileToolInput {
                            display_description: edit_description.into(),
                            path: input_file_path.into(),
                            mode: EditFileMode::Edit,
                        },
                    )],
                ),
            ],
            Some(input_file_content.into()),
            EvalAssertion::judge_diff(indoc! {"
                    - The diff contains a new `from_pixels` constructor
                    - The diff contains new tests for the `from_pixels` constructor
                "}),
        ),
    );
}

#[test]
#[cfg_attr(not(feature = "edit-agent-eval"), ignore)]
fn eval_zode() {
    //  Model                          | Pass rate
    // ============================================
    //
    //  claude-3.7-sonnet              |  1.0 (2025-06-14)
    //  claude-sonnet-4                |  1.0 (2025-06-14)
    //  gemini-2.5-pro-preview-03-25   |  1.0 (2025-05-22)
    //  gemini-2.5-flash-preview-04-17 |  1.0 (2025-05-22)
    //  gpt-4.1                        |  1.0 (2025-05-22)

    let input_file_path = "root/zode.py";
    let input_content = None;
    let edit_description = "Create the main Zode CLI script";
    eval(
        50,
        1.,
        0.05,
        EvalInput::from_conversation(
            vec![
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
                            EditFileToolInput {
                                display_description: edit_description.into(),
                                path: input_file_path.into(),
                                mode: EditFileMode::Create,
                            },
                        ),
                    ],
                ),
            ],
            input_content,
            EvalAssertion::new(async move |sample, _, _cx| {
                let invalid_starts = [' ', '`', '\n'];
                let mut message = String::new();
                for start in invalid_starts {
                    if sample.text_after.starts_with(start) {
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
        ),
    );
}

#[test]
#[cfg_attr(not(feature = "edit-agent-eval"), ignore)]
fn eval_add_overwrite_test() {
    //  Model                          | Pass rate
    // ============================================
    //
    //  claude-3.7-sonnet              |  0.65 (2025-06-14)
    //  claude-sonnet-4                |  0.07 (2025-06-14)
    //  gemini-2.5-pro-preview-03-25   |  0.35 (2025-05-22)
    //  gemini-2.5-flash-preview-04-17 |
    //  gpt-4.1                        |

    let input_file_path = "root/action_log.rs";
    let input_file_content = include_str!("evals/fixtures/add_overwrite_test/before.rs");
    let edit_description = "Add a new test for overwriting a file in action_log.rs";
    eval(
        200,
        0.5, // TODO: make this eval better
        0.05,
        EvalInput::from_conversation(
            vec![
                message(
                    User,
                    [text(indoc! {"
                        Introduce a new test in `action_log.rs` to test overwriting a file.
                        That is, a file already exists, but we call `buffer_created` as if the file were new.
                        Take inspiration from all the other tests in the file.
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
                    [tool_result(
                        "tool_1",
                        "read_file",
                        indoc! {"
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
                            "read_file",
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
                        "read_file",
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
                            "read_file",
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
                        "read_file",
                        lines(input_file_content, 1012..1120),
                    )],
                ),
                message(
                    Assistant,
                    [
                        text("Now let's look at how `buffer_created` is implemented:"),
                        tool_use(
                            "tool_4",
                            "read_file",
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
                        "read_file",
                        lines(input_file_content, 271..276),
                    )],
                ),
                message(
                    Assistant,
                    [
                        text(indoc! {"
                            Based on my understanding of the existing tests and code, I'll now create a new test for overwriting a file. In this test, we'll:

                            1. Create a file
                            2. Open a buffer for it
                            3. Track it in the action log
                            4. Modify the buffer
                            5. Call `buffer_created` again as if the file were new
                            6. Check that the changes are properly tracked

                            Let's write the test:
                        "}),
                        tool_use(
                            "tool_5",
                            "edit_file",
                            EditFileToolInput {
                                display_description: edit_description.into(),
                                path: input_file_path.into(),
                                mode: EditFileMode::Edit,
                            },
                        ),
                    ],
                ),
            ],
            Some(input_file_content.into()),
            EvalAssertion::judge_diff(
                "A new test for overwritten files was created, without changing any previous test",
            ),
        ),
    );
}

#[test]
#[cfg_attr(not(feature = "edit-agent-eval"), ignore)]
fn eval_create_empty_file() {
    // Check that Edit Agent can create a file without writing its
    // thoughts into it. This issue is not specific to empty files, but
    // it's easier to reproduce with them.
    //
    //  Model                          | Pass rate
    // ============================================
    //
    //  claude-3.7-sonnet              |  1.00 (2025-06-14)
    //  claude-sonnet-4                |  1.00 (2025-06-14)
    //  gemini-2.5-pro-preview-03-25   |  1.00 (2025-05-21)
    //  gemini-2.5-flash-preview-04-17 |  1.00 (2025-05-21)
    //  gpt-4.1                        |  1.00 (2025-05-21)
    //
    //
    // TODO: gpt-4.1-mini errored 38 times:
    // "data did not match any variant of untagged enum ResponseStreamResult"

    let input_file_content = None;
    let expected_output_content = String::new();
    eval(
        100,
        0.99,
        0.05,
        EvalInput::from_conversation(
            vec![
                message(User, [text("Create a second empty todo file ")]),
                message(
                    Assistant,
                    [
                        text(formatdoc! {"
                        I'll help you create a second empty todo file.
                        First, let me examine the project structure to see if there's already a todo file, which will help me determine the appropriate name and location for the second one.
                        "}),
                        tool_use(
                            "toolu_01GAF8TtsgpjKxCr8fgQLDgR",
                            "list_directory",
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
                        "list_directory",
                        "root/TODO\nroot/TODO2\nroot/new.txt\n",
                    )],
                ),
                message(
                    Assistant,
                    [
                        text(formatdoc! {"
                        I can see there's already a `TODO` file in the `root` directory. Let me create a second empty todo file called `TODO3` in the same directory:
                    "}),
                        tool_use(
                            "toolu_01Tb3iQ9griqSYMmVuykQPWU",
                            "edit_file",
                            EditFileToolInput {
                                display_description: "Create empty TODO3 file".to_string(),
                                mode: EditFileMode::Create,
                                path: "root/TODO3".into(),
                            },
                        ),
                    ],
                ),
            ],
            input_file_content,
            // Bad behavior is to write something like
            // "I'll create an empty TODO3 file as requested."
            EvalAssertion::assert_eq(expected_output_content),
        ),
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
        content: LanguageModelToolResultContent::Text(result.into()),
        output: None,
    })
}

#[derive(Clone)]
struct EvalInput {
    conversation: Vec<LanguageModelRequestMessage>,
    edit_file_input: EditFileToolInput,
    input_content: Option<String>,
    assertion: EvalAssertion,
}

impl EvalInput {
    fn from_conversation(
        conversation: Vec<LanguageModelRequestMessage>,
        input_content: Option<String>,
        assertion: EvalAssertion,
    ) -> Self {
        let msg = conversation.last().expect("Conversation must not be empty");
        if msg.role != Role::Assistant {
            panic!("Conversation must end with an assistant message");
        }
        let tool_use = msg
            .content
            .iter()
            .flat_map(|content| match content {
                MessageContent::ToolUse(tool_use) if tool_use.name == "edit_file".into() => {
                    Some(tool_use)
                }
                _ => None,
            })
            .next()
            .expect("Conversation must end with an edit_file tool use")
            .clone();

        let edit_file_input: EditFileToolInput = serde_json::from_value(tool_use.input).unwrap();

        EvalInput {
            conversation,
            edit_file_input,
            input_content,
            assertion,
        }
    }
}

#[derive(Clone)]
struct EvalSample {
    text_before: String,
    text_after: String,
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
                let expected =
                    language::apply_diff_patch(&sample.text_before, possible_diff).unwrap();
                strip_empty_lines(&expected) == strip_empty_lines(&sample.text_after)
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
            .unwrap();

            let request = LanguageModelRequest {
                messages: vec![LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![prompt.into()],
                    cache: false,
                }],
                thinking_allowed: true,
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

            // Parse the score from the response
            let re = regex::Regex::new(r"<score>(\d+)</score>").unwrap();
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

fn eval(
    iterations: usize,
    expected_pass_ratio: f32,
    mismatched_tag_threshold: f32,
    mut eval: EvalInput,
) {
    let mut evaluated_count = 0;
    let mut failed_count = 0;
    report_progress(evaluated_count, failed_count, iterations);

    let (tx, rx) = mpsc::channel();

    // Cache the last message in the conversation, and run one instance of the eval so that
    // all the next ones are cached.
    eval.conversation.last_mut().unwrap().cache = true;
    run_eval(eval.clone(), tx.clone());

    let executor = gpui::background_executor();
    let semaphore = Arc::new(smol::lock::Semaphore::new(32));
    for _ in 1..iterations {
        let eval = eval.clone();
        let tx = tx.clone();
        let semaphore = semaphore.clone();
        executor
            .spawn(async move {
                let _guard = semaphore.acquire().await;
                run_eval(eval, tx)
            })
            .detach();
    }
    drop(tx);

    let mut failed_evals = HashMap::default();
    let mut errored_evals = HashMap::default();
    let mut eval_outputs = Vec::new();
    let mut cumulative_parser_metrics = EditParserMetrics::default();
    while let Ok(output) = rx.recv() {
        match output {
            Ok(output) => {
                cumulative_parser_metrics += output.sample.edit_output.parser_metrics.clone();
                eval_outputs.push(output.clone());
                if output.assertion.score < 80 {
                    failed_count += 1;
                    failed_evals
                        .entry(output.sample.text_after.clone())
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
        report_progress(evaluated_count, failed_count, iterations);
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
    if mismatched_tag_ratio > mismatched_tag_threshold {
        for eval_output in eval_outputs {
            println!("{}", eval_output);
        }
        panic!("Too many mismatched tags: {:?}", cumulative_parser_metrics);
    }
}

fn run_eval(eval: EvalInput, tx: mpsc::Sender<Result<EvalOutput>>) {
    let dispatcher = gpui::TestDispatcher::new(StdRng::from_os_rng());
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
            self.sample.edit_output.parser_metrics
        )?;
        writeln!(f, "Raw Edits:\n{}", self.sample.edit_output.raw_edits)?;
        Ok(())
    }
}

fn report_progress(evaluated_count: usize, failed_count: usize, iterations: usize) {
    let passed_count = evaluated_count - failed_count;
    let passed_ratio = if evaluated_count == 0 {
        0.0
    } else {
        passed_count as f64 / evaluated_count as f64
    };
    print!(
        "\r\x1b[KEvaluated {}/{} ({:.2}% passed)",
        evaluated_count,
        iterations,
        passed_ratio * 100.0
    );
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

        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| {
            settings::init(cx);
            gpui_tokio::init(cx);
            let http_client = Arc::new(ReqwestClient::user_agent("agent tests").unwrap());
            cx.set_http_client(http_client);

            client::init_settings(cx);
            let client = Client::production(cx);
            let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));

            settings::init(cx);
            Project::init_settings(cx);
            language::init(cx);
            language_model::init(client.clone(), cx);
            language_models::init(user_store, client.clone(), cx);
        });

        fs.insert_tree("/root", json!({})).await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let agent_model = SelectedModel::from_str(
            &std::env::var("ZED_AGENT_MODEL").unwrap_or("anthropic/claude-4-sonnet-latest".into()),
        )
        .unwrap();
        let judge_model = SelectedModel::from_str(
            &std::env::var("ZED_JUDGE_MODEL").unwrap_or("anthropic/claude-4-sonnet-latest".into()),
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
        let (agent_model, judge_model) = cx
            .update(|cx| {
                cx.spawn(async move |cx| {
                    futures::future::join_all(authenticate_provider_tasks).await;
                    let agent_model = Self::load_model(&agent_model, cx).await;
                    let judge_model = Self::load_model(&judge_model, cx).await;
                    (agent_model.unwrap(), judge_model.unwrap())
                })
            })
            .await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));

        let edit_format = EditFormat::from_env(agent_model.clone()).unwrap();

        Self {
            agent: EditAgent::new(
                agent_model,
                project.clone(),
                action_log,
                Templates::new(),
                edit_format,
            ),
            project,
            judge_model,
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
        })?
        .await?;
        cx.update(|cx| {
            let models = LanguageModelRegistry::read_global(cx);
            let model = models
                .available_models(cx)
                .find(|model| {
                    model.provider_id() == selected_model.provider
                        && model.id() == selected_model.model
                })
                .expect("Model not found");
            model
        })
    }

    async fn eval(&self, eval: EvalInput, cx: &mut TestAppContext) -> Result<EvalOutput> {
        let path = self
            .project
            .read_with(cx, |project, cx| {
                project.find_project_path(eval.edit_file_input.path, cx)
            })
            .unwrap();
        let buffer = self
            .project
            .update(cx, |project, cx| project.open_buffer(path, cx))
            .await
            .unwrap();

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
            };
            let templates = Templates::new();
            template.render(&templates).unwrap()
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
            }]
            .into_iter()
            .chain(eval.conversation)
            .collect::<Vec<_>>()
        };

        let conversation = LanguageModelRequest {
            messages,
            tools,
            thinking_allowed: true,
            ..Default::default()
        };

        let edit_output = if matches!(eval.edit_file_input.mode, EditFileMode::Edit) {
            if let Some(input_content) = eval.input_content.as_deref() {
                buffer.update(cx, |buffer, cx| buffer.set_text(input_content, cx));
            }
            retry_on_rate_limit(async || {
                self.agent
                    .edit(
                        buffer.clone(),
                        eval.edit_file_input.display_description.clone(),
                        &conversation,
                        &mut cx.to_async(),
                    )
                    .0
                    .await
            })
            .await?
        } else {
            retry_on_rate_limit(async || {
                self.agent
                    .overwrite(
                        buffer.clone(),
                        eval.edit_file_input.display_description.clone(),
                        &conversation,
                        &mut cx.to_async(),
                    )
                    .0
                    .await
            })
            .await?
        };

        let buffer_text = buffer.read_with(cx, |buffer, _| buffer.text());
        let sample = EvalSample {
            edit_output,
            diff: language::unified_diff(
                eval.input_content.as_deref().unwrap_or_default(),
                &buffer_text,
            ),
            text_before: eval.input_content.unwrap_or_default(),
            text_after: buffer_text,
        };
        let assertion = eval
            .assertion
            .run(&sample, self.judge_model.clone(), cx)
            .await?;

        Ok(EvalOutput { assertion, sample })
    }
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
                        // Only retry for specific status codes
                        let should_retry = matches!(
                            *status,
                            StatusCode::TOO_MANY_REQUESTS | StatusCode::SERVICE_UNAVAILABLE
                        ) || status.as_u16() == 529;

                        if should_retry {
                            // Use server-provided retry_after if available, otherwise use default
                            Some(retry_after.unwrap_or(Duration::from_secs(5)))
                        } else {
                            None
                        }
                    }
                    LanguageModelCompletionError::ApiReadResponseError { .. }
                    | LanguageModelCompletionError::ApiInternalServerError { .. }
                    | LanguageModelCompletionError::HttpSend { .. } => {
                        // Exponential backoff for transient I/O and internal server errors
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
            Timer::after(retry_after + jitter).await;
        } else {
            return response;
        }
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
