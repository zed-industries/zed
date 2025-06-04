// EVAL 3: Models stripped indentation, breaking everything

#[test]
fn eval_indented_code_editing() {
    eval(100, 0.95, "Edit nested function", || async {
        let buffer = Buffer::local(
            "    fn outer() {\n        fn inner() {\n            todo!()\n        }\n    }"
        );

        // Ask to modify the inner function
        let (task, _) = agent.edit(
            buffer.clone(),
            "Replace todo with return 42",
            &conversation,
        );

        // Result analysis showed:
        // - 85% technically "worked"
        // - But 100% had broken indentation!

        // Models sent:
        // <old_text>fn inner() {\n    todo!()\n}</old_text>
        // <new_text>fn inner() {\n    return 42\n}</new_text>

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
async fn test_reindent_streaming(mut rng: StdRng) {
    // Test the algorithm with random streaming chunks
    let delta = IndentDelta::Spaces(8);
    let new_text = "ipsum\ndolor\nsit\namet";
    let chunks = random_chunks(new_text, &mut rng);

    let reindented = reindent_stream(chunks, delta).collect();
    assert_eq!(reindented,
        "        ipsum\n        dolor\n        sit\n        amet");
}

// But we went further - property-based testing:

#[gpui::test(iterations = 100)]
async fn test_random_indents(mut rng: StdRng) {
    // Generate random text with random indentation
    let len = rng.gen_range(1..=100);
    let text = RandomCharIter::new(&mut rng)
        .take(len)
        .collect::<String>();

    // Random indent delta: -20 to +20 spaces or tabs
    let delta = if rng.gen() {
        IndentDelta::Spaces(rng.gen_range(-20..=20))
    } else {
        IndentDelta::Tabs(rng.gen_range(-20..=20))
    };

    // Split into random chunks
    let chunks = random_chunks(&text, &mut rng);

    // Apply streaming reindentation
    let reindented = reindent_stream(chunks, delta).collect();

    // Property: Every line should have correct indentation adjustment
    // (Even with random text, random deltas, random chunking!)
}

// The eval drove the discovery → algorithmic solution → comprehensive testing
