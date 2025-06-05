// EVAL 3: Models stripped indentation, breaking everything

#[test]
fn eval_indented_code_editing() {
    eval(100, 0.95, "Edit nested function", || async {
        let buffer = Buffer::local(
            indoc! {"
                fn outer() {
                    fn inner() {
                        todo!()
                    }
                }"
            }
        );

        // Ask to modify the inner function
        agent.edit(
            buffer.clone(),
            "Replace todo with return 42",
            &conversation,
        ).0.await;

        // Models sent:
        // <old_text>
        // fn inner() {
        //     todo!()
        // }
        // </old_text>
        // <new_text>
        // fn inner() {
        //     return 42
        // }
        // </new_text>

        // Code has 8-space indent, model used 4!
    });
}

// This eval failure drove us to build an algorithmic solution:

fn calculate_indent_delta(buffer_text: &str, llm_text: &str) -> IndentDelta {
    let buffer_indent = detect_indent(buffer_text);  // 8 spaces
    let llm_indent = detect_indent(llm_text);        // 0 spaces

    IndentDelta::Spaces(buffer_indent - llm_indent)  // +8
}

// Which we could then test deterministically:

#[gpui::test(iterations = 100)]
async fn test_indentation(cx: &mut TestAppContext, mut rng: StdRng) {
    let agent = init_test(cx).await;
    let buffer = cx.new(|cx| {
        Buffer::local(
            indoc! {"
                lorem
                        ipsum
                        dolor
                        sit
            "},
            cx,
        )
    });
    let (apply, _events) = agent.edit(
        buffer.clone(),
        String::new(),
        &LanguageModelRequest::default(),
        &mut cx.to_async(),
    );

    simulate_llm_output(
        &agent,
        indoc! {"
            <old_text>
                ipsum
                dolor
                sit
            </old_text>
            <new_text>
                ipsum
                dolor
                sit
            amet
            </new_text>
        "},
        &mut rng,
        cx,
    );
    apply.await.unwrap();

    assert_eq!(
        buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
        indoc! {"
            lorem
                    ipsum
                    dolor
                    sit
                amet
        "}
    );
}
