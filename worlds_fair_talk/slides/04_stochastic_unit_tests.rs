// When AI enters the equation, we need a new approach
// We test AI features by sampling their behavior:

#[test]
fn eval_translate_doc_comments() {
    let input_file_path = "root/canvas.rs";
    let input_file_content = include_str!("evals/fixtures/translate_doc_comments/before.rs");
    let edit_description = "Translate all doc comments to Italian";
    eval(
        200,
        1.,
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
